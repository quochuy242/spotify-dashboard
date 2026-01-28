import sys
import pytest
from datetime import datetime
from unittest.mock import Mock

sys.path.append("../")

from extractor import TextExtractor, MusicMetadata


# ------------------
# Fixtures
# ------------------

@pytest.fixture
def mock_jsonformer_output():
    return {
        "track": {
            "name": "Hello",
            "genre": "hip hop",
            "mood": "happy",
            "year": 21,
        },
        "artist": {
            "name": "Adele",
            "language": "english",
        },
        "limit": 5,
    }


@pytest.fixture
def extractor():
    fake_jsonformer = Mock()
    return TextExtractor(
        compact=False,
        jsonformer=fake_jsonformer,
    )


@pytest.fixture
def compact_extractor():
    fake_jsonformer = Mock()
    return TextExtractor(
        compact=True,
        jsonformer=fake_jsonformer,
    )


# ------------------
# Tests
# ------------------

def test_extract_basic_normalization(extractor, mock_jsonformer_output):
    extractor.jsonformer.return_value = mock_jsonformer_output

    result = extractor.extract("some input text")

    assert isinstance(result, MusicMetadata)

    assert result.track.name == "Hello"
    assert result.track.genre == "Hip-Hop"
    assert result.track.mood == "Happy"
    assert result.track.year == 2021

    assert result.artist.name == "Adele"
    assert result.artist.language == "English"

    assert result.limit == 5


def test_compact_mode_removes_none_fields(compact_extractor, mock_jsonformer_output):
    mock_jsonformer_output["artist"] = None
    compact_extractor.jsonformer.return_value = mock_jsonformer_output

    result = compact_extractor.extract_to_dict("some input")

    assert "artist" not in result
    assert "track" in result
    assert result["track"]["genre"] == "Hip-Hop"


def test_limit_out_of_range(extractor):
    extractor.jsonformer.return_value = {"limit": 99}

    result = extractor.extract("test")

    assert result.limit == 10


def test_year_out_of_range(extractor):
    current_year = datetime.now().year
    extractor.jsonformer.return_value = {"track": {"year": 1800}}

    result = extractor.extract("test")

    assert result.track.year == current_year


def test_unknown_mood_and_language(extractor):
    extractor.jsonformer.return_value = {
        "track": {"mood": "weird"},
        "artist": {"language": "klingon"},
    }

    result = extractor.extract("test")

    assert result.track.mood == "Unknown"
    assert result.artist.language == "Unknown"


def test_empty_input_returns_empty_dict():
    from extractor import extract_text

    assert extract_text("", compact=True) == {}
    assert extract_text("   ", compact=True) == {}


def test_jsonformer_exception_is_handled(extractor):
    extractor.jsonformer.side_effect = Exception("boom")

    result = extractor.extract("test")

    assert isinstance(result, MusicMetadata)

@pytest.mark.integration
def test_with_real_jsonformer():
    extractor = TextExtractor(compact=True)

    result = extractor.extract("A happy hip hop song by Adele")

    assert result.track.genre == "Hip-Hop"
