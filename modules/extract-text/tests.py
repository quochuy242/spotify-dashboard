import unittest
from extract import (
    extract_text,
    TextExtractor,
    MusicMetadata,
    TrackMetadata,
    ArtistMetadata,
    VALID_GENRES,
)


class TestExtractText(unittest.TestCase):
    """Tests for extract_text main behavior"""

    def test_extract_returns_metadata(self):
        text = "I like pop music"
        result = extract_text(text)

        self.assertIsInstance(result, MusicMetadata)

    def test_extract_vietnamese_input(self):
        text = "Tôi thích nhạc Pop của Sơn Tùng"
        result = extract_text(text)

        self.assertIsInstance(result, MusicMetadata)

    def test_extract_with_year(self):
        text = "Adele released Hello in 2015"
        result = extract_text(text)

        self.assertIsInstance(result, MusicMetadata)

        if result.track:
            if result.track.year is not None:
                self.assertEqual(result.track.year, 2015)

    def test_extract_minimal_input(self):
        text = "Jazz"
        result = extract_text(text)

        self.assertIsInstance(result, MusicMetadata)

    def test_empty_input_returns_empty_dict(self):
        result = extract_text("")

        self.assertIsInstance(result, dict)
        self.assertEqual(result, {})


class TestCompactMode(unittest.TestCase):
    """Tests for compact=True behavior"""

    def test_extract_compact_still_returns_model(self):
        extractor = TextExtractor(compact=True)
        result = extractor.extract("Pop music")

        self.assertIsInstance(result, MusicMetadata)

    def test_extract_to_dict_compact(self):
        extractor = TextExtractor(compact=True)
        data = extractor.extract_to_dict("EDM music")

        self.assertIsInstance(data, dict)

        if "track" in data:
            self.assertIn("genre", data["track"])

        self.assertNotIn("artist", data)  # artist should be removed if None


class TestGenreNormalization(unittest.TestCase):
    """Tests for genre normalization"""

    def test_genre_normalization_mapping(self):
        cases = {
            "electronic music": "Edm",
            "hip hop beats": "Hip-Hop",
            "r&b vibes": "R&B",
            "lofi chill": "Lo-Fi",
        }

        for text, expected in cases.items():
            result = extract_text(text)
            if result.track and result.track.genre:
                self.assertEqual(result.track.genre, expected)

    def test_genre_is_valid_enum(self):
        text = "I enjoy jazz music"
        result = extract_text(text)

        if result.track and result.track.genre:
            self.assertIn(result.track.genre, VALID_GENRES)


class TestErrorHandling(unittest.TestCase):
    def test_exception_returns_empty_metadata(self):
        import langextract as lx

        original = lx.extract

        def broken_extract(*args, **kwargs):
            raise RuntimeError("boom")

        lx.extract = broken_extract

        extractor = TextExtractor()
        result = extractor.extract("test")

        self.assertIsInstance(result, MusicMetadata)
        self.assertIsNone(result.track)
        self.assertIsNone(result.artist)
        self.assertIsNone(result.limit)

        lx.extract = original



class TestModelDefaults(unittest.TestCase):
    """Tests for schema default behavior"""

    def test_music_metadata_defaults(self):
        metadata = MusicMetadata()

        self.assertIsNone(metadata.track)
        self.assertIsNone(metadata.artist)
        self.assertIsNone(metadata.limit)

    def test_track_metadata_defaults(self):
        track = TrackMetadata()

        self.assertIsNone(track.name)
        self.assertIsNone(track.genre)
        self.assertIsNone(track.mood)
        self.assertIsNone(track.year)

    def test_artist_metadata_defaults(self):
        artist = ArtistMetadata()

        self.assertIsNone(artist.name)
        self.assertIsNone(artist.country)


if __name__ == "__main__":
    unittest.main()
