import textwrap
import time
from logging import getLogger
from typing import Any, Dict, List, Optional

from jsonformer import Jsonformer, highlight_values
from pydantic import BaseModel, Field
from transformers import AutoModelForCausalLM, AutoTokenizer

logger = getLogger(__name__)
logger.setLevel("INFO")


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
    language: Optional[str] = Field(None, description="Artist language/origin")


class MusicMetadata(BaseModel):
    """Extracted music metadata from user input"""

    track: Optional[TrackMetadata] = Field(None, description="Track information")
    artist: Optional[List[ArtistMetadata]] = Field(None, description="Artist information")
    limit: Optional[int] = Field(None, description="Limit or quantity mentioned")


PROMPT = textwrap.dedent(
    """
    Extract music-related information from the user input text.
    Only include fields that are explicitly mentioned or can be clearly inferred.
    Return the extracted data in the specified format.
    
    Text to analyze: {text}
    
    Extract the following if present:
    
    Schema: {schema}
    
    If {compact}, return the extracted data with all fields, including None values.
    """
)

SCHEMA = {
    "type": "object",
    "properties": {
        "track": {
            "type": "object",
            "properties": {
                "name": {"type": "string"},
                "genre": {
                    "type": "string",
                    "enum": [
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
                    ],
                },
                "mood": {
                    "type": "string",
                    "enum": [
                        "Happy",
                        "Sad",
                        "Energetic",
                        "Calm",
                        "Angry",
                        "Melancholic",
                        "Peaceful",
                        "Romantic",
                        "Unknown",
                    ],
                },
                "year": {"type": "integer"},
            },
        },
        "artist": {
            "type": "array",
            "items": {
                "type": "object",
                "properties": {
                    "name": {"type": "string"},
                    "language": {
                        "type": "string",
                        "enum": [
                            "English",
                            "Spanish",
                            "French",
                            "Japanese",
                            "Korean",
                            "Chinese",
                            "Vietnamese",
                            "Thai",
                            "Hindi",
                            "Unknown",
                        ],
                    },
                },
            },
        },
        "limit": {"type": "integer"},
    },
}


class TextExtractor:
    """Service to extract music metadata from user text"""

    def __init__(self, model_name: str, schema: Dict[str, Any] = SCHEMA, prompt: str = PROMPT, compact: bool = False, **kwargs):
        """
        Initialization module

        Args:
            model_name (str): HuggingFace model name, e.g. "moonshotai/Kimi-K2.5"
            schema (Dict[str, Any], optional): JSON Schema to guide extraction. Defaults to SCHEMA.
            prompt (str, optional): Prompt to guide extraction. Defaults to PROMPT.
            compact (bool, optional): Whether to return metadata as dictionary. Defaults to False.
        """
        self.trust_remote_code = kwargs.get("trust_remote_code", True)
        self.tokenizer = AutoTokenizer.from_pretrained(model_name, trust_remote_code=self.trust_remote_code)
        self.model = AutoModelForCausalLM.from_pretrained(model_name, trust_remote_code=self.trust_remote_code)

        self.compact = compact
        self.schema = schema
        self.prompt = prompt

    def extract(self, user_input: str, to_dict: bool = False) -> MusicMetadata:
        """
        Extract music metadata from user input text

        Args:
            user_input: Raw text from user
            to_dict: Whether to return metadata as dictionary

        Returns:
            MusicMetadata object with extracted fields
        """
        try:
            prompt = self.prompt.format(text=user_input, schema=self.schema, compact=self.compact)
            builder = Jsonformer(
                tokenizer=self.tokenizer,
                model=self.model,
                prompt=prompt,
                json_schema=self.schema,
            )
            start_time = time.time()
            output = builder()
            logger.debug(f"Extracted metadata: {highlight_values(output)}")
            logger.debug(f"Extraction time: {time.time() - start_time:.2f} seconds")
            if to_dict:
                return output
            return MusicMetadata(**output)
        except Exception as e:
            logger.error(f"Error during extraction: {str(e)}, return empty metadata")
            return MusicMetadata()


# For manual testing
def main():
    extractor = TextExtractor(model_name="Qwen/Qwen3-4B", compact=True)
    examples = [
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
    
    for example in examples:
        result = extractor.extract(example["input"], to_dict=True)
        assert result == example["output"], f"Failed for input: {example['input']}, target: {example['output']}, predict: {result}"

if __name__ == "__main__":
    main()