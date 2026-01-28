"""
Client library for tele-bot module to communicate with extract-text service
"""

import requests
from typing import Dict, Any, List, Optional
from dataclasses import dataclass


@dataclass
class TrackInfo:
    """Track information"""
    name: Optional[str] = None
    genre: Optional[str] = None
    mood: Optional[str] = None
    year: Optional[int] = None
    era: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary with non-None values"""
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class ArtistInfo:
    """Artist information"""
    name: Optional[str] = None
    country: Optional[str] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary with non-None values"""
        return {k: v for k, v in self.__dict__.items() if v is not None}


@dataclass
class ExtractedMetadata:
    """Extracted music metadata"""
    track: Optional[TrackInfo] = None
    artist: Optional[ArtistInfo] = None
    language: Optional[str] = None
    limit: Optional[int] = None
    
    def to_dict(self) -> Dict[str, Any]:
        """Convert to dictionary with non-None values"""
        result = {}
        if self.track:
            result['track'] = self.track.to_dict()
        if self.artist:
            result['artist'] = self.artist.to_dict()
        if self.language:
            result['language'] = self.language
        if self.limit:
            result['limit'] = self.limit
        return result


class ExtractTextClient:
    """Client for extract-text service"""
    
    def __init__(self, base_url: str = "http://localhost:5001"):
        """
        Initialize client
        
        Args:
            base_url: Base URL of extract-text service
        """
        self.base_url = base_url.rstrip('/')
        self.timeout = 10
    
    def health_check(self) -> bool:
        """Check if service is healthy"""
        try:
            response = requests.get(
                f"{self.base_url}/health",
                timeout=self.timeout
            )
            return response.status_code == 200
        except requests.RequestException:
            return False
    
    def extract(
        self,
        text: str,
        user_id: Optional[str] = None
    ) -> Optional[ExtractedMetadata]:
        """
        Extract metadata from user text
        
        Args:
            text: User input text
            user_id: Optional user identifier
            
        Returns:
            ExtractedMetadata object or None if extraction fails
        """
        try:
            payload = {
                "text": text,
                "user_id": user_id
            }
            
            response = requests.post(
                f"{self.base_url}/extract",
                json=payload,
                timeout=self.timeout
            )
            
            if response.status_code != 200:
                print(f"Extraction failed: {response.json().get('message')}")
                return None
            
            data = response.json().get('data', {})
            
            # Parse nested structures
            track_data = data.get('track')
            artist_data = data.get('artist')
            
            track = TrackInfo(**track_data) if track_data else None
            artist = ArtistInfo(**artist_data) if artist_data else None
            
            return ExtractedMetadata(
                track=track,
                artist=artist,
                language=data.get('language'),
                limit=data.get('limit')
            )
            
        except requests.RequestException as e:
            print(f"Request error: {str(e)}")
            return None
        except Exception as e:
            print(f"Error: {str(e)}")
            return None
    
    def extract_batch(
        self,
        texts: List[Dict[str, str]]
    ) -> List[Dict[str, Any]]:
        """
        Extract metadata from multiple texts
        
        Args:
            texts: List of dicts with 'text' and optional 'user_id'
            
        Returns:
            List of extraction results
        """
        try:
            payload = {"texts": texts}
            
            response = requests.post(
                f"{self.base_url}/extract/batch",
                json=payload,
                timeout=self.timeout
            )
            
            if response.status_code != 200:
                print(f"Batch extraction failed: {response.json().get('message')}")
                return []
            
            return response.json().get('results', [])
            
        except requests.RequestException as e:
            print(f"Request error: {str(e)}")
            return []
        except Exception as e:
            print(f"Error: {str(e)}")
            return []


# Global client instance
_client = None


def get_client(base_url: str = "http://localhost:5001") -> ExtractTextClient:
    """Get or create global client instance"""
    global _client
    if _client is None:
        _client = ExtractTextClient(base_url)
    return _client


def extract_from_user_input(
    text: str,
    user_id: Optional[str] = None
) -> Optional[ExtractedMetadata]:
    """
    Convenience function for tele-bot module
    
    Args:
        text: User input text
        user_id: Optional user identifier
        
    Returns:
        ExtractedMetadata or None
    """
    client = get_client()
    return client.extract(text, user_id)
