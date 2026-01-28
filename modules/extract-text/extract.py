import langextract as lx
import textwrap
from typing import Optional, Dict, Any
from pydantic import BaseModel, Field
from logging import getLogger

logger = getLogger(__name__)
logger.setLevel("INFO")

# Valid genres from tele-bot module
VALID_GENRES = {
    "Ballad",
    "Pop",
    "Rock",
    "Edm",
    "Hip-Hop",
    "R&B",
    "Jazz",
    "Classical",
    "Acoustic",
    "Lo-Fi",
    "Indie",
    "Metal",
}


# Define nested schema classes
class TrackMetadata(BaseModel):
    """Track information"""

    name: Optional[str] = Field(None, description="Song/track name")
    genre: Optional[str] = Field(None, description="Music genre")
    mood: Optional[str] = Field(None, description="Mood or emotion of the music")
    year: Optional[int] = Field(None, description="Specific year mentioned")


class ArtistMetadata(BaseModel):
    """Artist information"""

    name: Optional[str] = Field(None, description="Artist/musician name")
    country: Optional[str] = Field(None, description="Artist country/origin")


class MusicMetadata(BaseModel):
    """Extracted music metadata from user input"""

    track: Optional[TrackMetadata] = Field(None, description="Track information")
    artist: Optional[ArtistMetadata] = Field(None, description="Artist information")
    limit: Optional[int] = Field(None, description="Limit or quantity mentioned")


PROMPT = textwrap.dedent(
    """
    Extract music-related information from the user input text.
    Only include fields that are explicitly mentioned or can be clearly inferred.
    Return the extracted data in the specified format.
    
    Text to analyze: {text}
    
    Extract the following if present:
    
    Track information:
    - name: Song or track name
    - genre: Music genre (valid: Ballad, Pop, Rock, Edm, Hip-Hop, R&B, Jazz, Classical, Acoustic, Lo-Fi, Indie, Metal)
    - mood: Mood or emotion (valid: Happy, Sad, Energetic, Calm, Angry, Melancholic, Peaceful, Romantic)
    - year: Specific year mentioned (as integer)
    
    Artist information:
    - name: Artist's name
    - country: Artist's country
    
    Other information:
    - limit: Limit or quantity if mentioned (as integer)
    """
)

EXAMPLES: Dict[str, Any] = [
    {
        "input": "Mình đang tìm một bài pop buồn của Taylor Swift, phát hành khoảng năm 2020.",
        "output": {"track": {"genre": "Pop", "mood": "Sad", "year": 2020}, "artist": {"name": "Taylor Swift"}},
    },
    {
        "input": "Gợi ý cho tôi 5 bài nhạc jazz nhẹ nhàng để nghe buổi tối.",
        "output": {"track": {"genre": "Jazz", "mood": "Calm"}, "limit": 5},
    },
    {
        "input": "Tôi muốn nghe bài “Yellow” của Coldplay, ban nhạc đến từ Anh.",
        "output": {"track": {"name": "Yellow"}, "artist": {"name": "Coldplay", "country": "UK"}},
    },
    {
        "input": "Có bài nhạc EDM nào nghe sôi động để tập gym không?",
        "output": {"track": {"genre": "Edm", "mood": "Energetic"}},
    },
    {
        "input": "Cho mình khoảng 3 bài nhạc phát hành năm 2018, nghe thư giãn là được.",
        "output": {"track": {"mood": "Calm", "year": 2018}, "other": {"limit": 3}},
    },
]


class TextExtractor:
    """Service to extract music metadata from user text"""

    def __init__(self, compact: bool = False):
        self.compact = compact

    def extract(self, user_input: str) -> MusicMetadata:
        """
        Extract music metadata from user input text

        Args:
            user_input: Raw text from user
            compact: Whether to return a compact version (only non-None fields) of the metadata (default: False)

        Returns:
            MusicMetadata object with extracted fields
        """
        try:
            # Extract using langextract
            result = lx.extract(
                text_or_document=user_input,
                prompt=PROMPT,
                examples=EXAMPLES,
                model_id="gemini-2.5-flash",
            )

            # Parse result to MusicMetadata
            if isinstance(result, dict):
                # Normalize track genre if present
                if "track" in result and isinstance(result["track"], dict):
                    if "genre" in result["track"] and result["track"]["genre"]:
                        result["track"]["genre"] = self._normalize_genre(result["track"]["genre"])
                # Filter out None values
                if self.compact:
                    # from track
                    if "track" in result and isinstance(result["track"], dict):
                        result["track"] = {k: v for k, v in result["track"].items() if v is not None}
                        if not result["track"]:
                            result["track"] = None
                        
                    # from artist
                    if "artist" in result and isinstance(result["artist"], dict):
                        result["artist"] = {k: v for k, v in result["artist"].items() if v is not None}
                        if not result["artist"]:
                            result["artist"] = None

                    # from other
                    filtered_result = {k: v for k, v in result.items() if v is not None}
                    return MusicMetadata(**filtered_result)
                
                # Ensure output always conforms to full-schema
                return MusicMetadata(**result)
            else:
                logger.warning(f"Unexpected result type: {type(result)}")
                return result

        except Exception as e:
            logger.error(f"Error during extraction: {str(e)}, return empty metadata")
            return MusicMetadata()
    
    def _normalize_genre(self, genre: str) -> str:
        """
        Normalize extracted genre to match valid genres from tele-bot

        Args:
            genre: Genre string to normalize

        Returns:
            Normalized genre or original if no match found
        """
        if not genre:
            return genre

        genre_lower = genre.lower().strip()

        # Direct matches
        for valid in VALID_GENRES:
            if genre_lower == valid.lower():
                return valid

        # Partial/fuzzy matches
        mapping = {
            "edm": "Edm",
            "electronic": "Edm",
            "dance": "Edm",
            "hiphop": "Hip-Hop",
            "hip hop": "Hip-Hop",
            "rap": "Hip-Hop",
            "rnb": "R&B",
            "r&b": "R&B",
            "soul": "R&B",
            "lofi": "Lo-Fi",
            "lo-fi": "Lo-Fi",
            "indie": "Indie",
            "ballad": "Ballad",
            "pop": "Pop",
            "rock": "Rock",
            "jazz": "Jazz",
            "classical": "Classical",
            "acoustic": "Acoustic",
            "metal": "Metal",
            "heavy metal": "Metal",
        }

        if genre_lower in mapping:
            return mapping[genre_lower]

        # Check if genre is contained in any valid genre
        for valid in VALID_GENRES:
            if genre_lower in valid.lower() or valid.lower() in genre_lower:
                return valid

        # Return original if no match
        logger.warning(f"Could not normalize genre: {genre}")
        return genre

    def extract_to_dict(self, user_input: str) -> Dict[str, Any]:
        """
        Extract and return as dictionary (only non-None fields)

        Args:
            user_input: Raw text from user

        Returns:
            Dictionary with only populated fields
        """
        metadata = self.extract(user_input)
        metadata_dict = metadata.model_dump()
        return {k: v for k, v in metadata_dict.items() if v is not None} if self.compact else metadata_dict



def extract_text(text: str, compact: bool = False) -> dict:
    if not text or not text.strip():
        return {}

    try:
        extractor = TextExtractor(compact=compact)
        return extractor.extract_to_dict(text) if compact else extractor.extract(text)
    except Exception:
        return {}

