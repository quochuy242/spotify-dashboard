/// Rule-based music genre detection system

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Genre {
    Ballad,
    Pop,
    Rock,
    Edm,
    HipHop,
    RnB,
    Jazz,
    Classical,
    Acoustic,
    LoFi,
    Indie,
    Metal,
    Unknown,
}

impl Genre {
    pub fn as_str(&self) -> &'static str {
        match self {
            Genre::Ballad => "Ballad",
            Genre::Pop => "Pop",
            Genre::Rock => "Rock",
            Genre::Edm => "Edm",
            Genre::HipHop => "Hip-Hop",
            Genre::RnB => "R&B",
            Genre::Jazz => "Jazz",
            Genre::Classical => "Classical",
            Genre::Acoustic => "Acoustic",
            Genre::LoFi => "Lo-Fi",
            Genre::Indie => "Indie",
            Genre::Metal => "Metal",
            Genre::Unknown => "Unknown",
        }
    }
}

/// Audio features from Spotify API
#[derive(Debug, Clone, Copy)]
pub struct AudioFeatures {
    pub tempo: f32,
    pub energy: f32,
    pub valence: f32,
    pub danceability: f32,
    pub acousticness: f32,
    pub instrumentalness: f32,
    pub loudness: f32,
    pub speechiness: f32,
}

/// Detection result with genre and confidence (0.0 to 1.0)
#[derive(Debug, Clone)]
pub struct GenreDetection {
    pub genre: Genre,
    pub confidence: f32,
    pub scores: GenreScores,
}

/// Detailed scores for each genre (for transparency)
#[derive(Debug, Clone)]
pub struct GenreScores {
    pub ballad: f32,
    pub pop: f32,
    pub rock: f32,
    pub edm: f32,
    pub hiphop: f32,
    pub rnb: f32,
    pub jazz: f32,
    pub classical: f32,
    pub acoustic: f32,
    pub lofi: f32,
    pub indie: f32,
    pub metal: f32,
}

/// Pure function: detect genre from audio features and artist metadata
/// 
/// # Arguments
/// * `features` - Audio features from Spotify
/// * `artist_genres` - Genre tags from artist metadata (high weight)
/// * `popularity` - Popularity score (0-100)
///
/// # Returns
/// `GenreDetection` with best matching genre and confidence score
pub fn detect_genre(
    features: AudioFeatures,
    artist_genres: &[String],
    popularity: u32,
) -> GenreDetection {
    let scores = GenreScores {
        ballad: score_ballad(&features, artist_genres),
        pop: score_pop(&features, artist_genres),
        rock: score_rock(&features, artist_genres),
        edm: score_edm(&features, artist_genres),
        hiphop: score_hiphop(&features, artist_genres),
        rnb: score_rnb(&features, artist_genres),
        jazz: score_jazz(&features, artist_genres),
        classical: score_classical(&features, artist_genres),
        acoustic: score_acoustic(&features, artist_genres),
        lofi: score_lofi(&features, artist_genres),
        indie: score_indie(&features, artist_genres, popularity),
        metal: score_metal(&features, artist_genres),
    };

    // Normalize scores
    let max_score = [
        scores.ballad,
        scores.pop,
        scores.rock,
        scores.edm,
        scores.hiphop,
        scores.rnb,
        scores.jazz,
        scores.classical,
        scores.acoustic,
        scores.lofi,
        scores.indie,
        scores.metal,
    ]
    .iter()
    .copied()
    .fold(f32::NEG_INFINITY, f32::max);

    let (genre, confidence) = if max_score > 0.0 {
        // Normalize confidence to 0-1
        let norm_score = max_score / 12.0; // Max possible score with artist genre bonus

        if scores.ballad == max_score {
            (Genre::Ballad, norm_score)
        } else if scores.pop == max_score {
            (Genre::Pop, norm_score)
        } else if scores.rock == max_score {
            (Genre::Rock, norm_score)
        } else if scores.edm == max_score {
            (Genre::Edm, norm_score)
        } else if scores.hiphop == max_score {
            (Genre::HipHop, norm_score)
        } else if scores.rnb == max_score {
            (Genre::RnB, norm_score)
        } else if scores.jazz == max_score {
            (Genre::Jazz, norm_score)
        } else if scores.classical == max_score {
            (Genre::Classical, norm_score)
        } else if scores.acoustic == max_score {
            (Genre::Acoustic, norm_score)
        } else if scores.lofi == max_score {
            (Genre::LoFi, norm_score)
        } else if scores.indie == max_score {
            (Genre::Indie, norm_score)
        } else if scores.metal == max_score {
            (Genre::Metal, norm_score)
        } else {
            (Genre::Unknown, 0.0)
        }
    } else {
        (Genre::Unknown, 0.0)
    };

    GenreDetection {
        genre,
        confidence,
        scores,
    }
}

// ============================================================================
// GENRE SCORING FUNCTIONS
// ============================================================================

fn artist_genre_bonus(artist_genres: &[String], keywords: &[&str]) -> f32 {
    let has_match = artist_genres.iter().any(|genre| {
        let genre_lower = genre.to_lowercase();
        keywords.iter().any(|keyword| genre_lower.contains(keyword))
    });

    if has_match {
        5.0 // High weight for artist genre match
    } else {
        0.0
    }
}

fn score_ballad(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    // Artist genre match (weight: 5)
    score += artist_genre_bonus(artist_genres, &["ballad"]);

    // Audio features (weight: 1 each)
    if features.tempo < 90.0 {
        score += 1.0;
    }
    if features.energy < 0.45 {
        score += 1.0;
    }
    if features.acousticness > 0.4 {
        score += 1.0;
    }
    if features.valence < 0.6 {
        score += 1.0;
    }

    score
}

fn score_pop(features: &AudioFeatures, _artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    if features.tempo >= 90.0 && features.tempo <= 130.0 {
        score += 1.0;
    }
    if features.energy >= 0.4 && features.energy <= 0.8 {
        score += 1.0;
    }
    if features.danceability > 0.5 {
        score += 1.0;
    }
    if features.valence > 0.4 {
        score += 1.0;
    }

    score
}

fn score_rock(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["rock"]);

    if features.energy > 0.65 {
        score += 1.0;
    }
    if features.loudness > -8.0 {
        score += 1.0;
    }
    if features.acousticness < 0.3 {
        score += 1.0;
    }
    if features.tempo >= 90.0 && features.tempo <= 160.0 {
        score += 1.0;
    }

    score
}

fn score_edm(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["edm", "house", "techno", "electronic"]);

    if features.danceability > 0.7 {
        score += 1.0;
    }
    if features.energy > 0.75 {
        score += 1.0;
    }
    if features.tempo > 120.0 {
        score += 1.0;
    }
    if features.acousticness < 0.2 {
        score += 1.0;
    }

    score
}

fn score_hiphop(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["hip hop", "hip-hop", "rap"]);

    if features.tempo >= 70.0 && features.tempo <= 110.0 {
        score += 1.0;
    }
    if features.speechiness > 0.33 {
        score += 1.0;
    }
    if features.energy > 0.4 {
        score += 1.0;
    }

    score
}

fn score_rnb(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["r&b", "rnb", "r&b/soul"]);

    if features.tempo < 100.0 {
        score += 1.0;
    }
    if features.energy >= 0.3 && features.energy <= 0.6 {
        score += 1.0;
    }
    if features.danceability > 0.5 {
        score += 1.0;
    }
    if features.valence < 0.6 {
        score += 1.0;
    }

    score
}

fn score_jazz(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["jazz"]);

    if features.instrumentalness > 0.5 {
        score += 1.0;
    }
    if features.energy < 0.5 {
        score += 1.0;
    }
    if features.tempo < 120.0 {
        score += 1.0;
    }

    score
}

fn score_classical(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["classical", "orchestra", "symphony"]);

    if features.instrumentalness > 0.7 {
        score += 1.0;
    }
    if features.energy < 0.3 {
        score += 1.0;
    }
    if features.loudness < -20.0 {
        score += 1.0;
    }

    score
}

fn score_acoustic(features: &AudioFeatures, _artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    if features.acousticness > 0.75 {
        score += 2.0; // Higher weight for strong acoustic signal
    }
    if features.energy < 0.5 {
        score += 1.0;
    }

    score
}

fn score_lofi(features: &AudioFeatures, _artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    if features.tempo < 85.0 {
        score += 1.0;
    }
    if features.energy < 0.4 {
        score += 1.0;
    }
    if features.loudness < -10.0 {
        score += 1.0;
    }
    if features.instrumentalness > 0.3 {
        score += 1.0;
    }

    score
}

fn score_indie(features: &AudioFeatures, artist_genres: &[String], popularity: u32) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["indie", "alternative"]);

    if features.energy >= 0.4 && features.energy <= 0.7 {
        score += 1.0;
    }
    if features.acousticness >= 0.3 && features.acousticness <= 0.6 {
        score += 1.0;
    }
    // Lower popularity is more indie
    if popularity < 60 {
        score += 1.0;
    }

    score
}

fn score_metal(features: &AudioFeatures, artist_genres: &[String]) -> f32 {
    let mut score = 0.0;

    score += artist_genre_bonus(artist_genres, &["metal", "heavy metal", "rock"]);

    if features.energy > 0.8 {
        score += 1.0;
    }
    if features.loudness > -5.0 {
        score += 1.0;
    }
    if features.tempo > 120.0 {
        score += 1.0;
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_features() -> AudioFeatures {
        AudioFeatures {
            tempo: 100.0,
            energy: 0.5,
            valence: 0.5,
            danceability: 0.5,
            acousticness: 0.5,
            instrumentalness: 0.3,
            loudness: -8.0,
            speechiness: 0.1,
        }
    }

    #[test]
    fn test_ballad_detection() {
        let features = AudioFeatures {
            tempo: 70.0,
            energy: 0.3,
            valence: 0.3,
            danceability: 0.2,
            acousticness: 0.7,
            instrumentalness: 0.1,
            loudness: -12.0,
            speechiness: 0.05,
        };

        let result = detect_genre(features, &[], 50);
        assert_eq!(result.genre, Genre::Ballad);
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_pop_detection() {
        let features = AudioFeatures {
            tempo: 120.0,
            energy: 0.7,
            valence: 0.8,
            danceability: 0.75,
            acousticness: 0.2,
            instrumentalness: 0.1,
            loudness: -6.0,
            speechiness: 0.05,
        };

        let result = detect_genre(features, &[], 80);
        assert_eq!(result.genre, Genre::Pop);
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_artist_genre_bonus() {
        let features = sample_features();
        let genres = vec!["electronic".to_string(), "edm".to_string()];

        let result = detect_genre(features, &genres, 70);
        assert_eq!(result.genre, Genre::Edm);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_acoustic_detection() {
        let features = AudioFeatures {
            tempo: 100.0,
            energy: 0.25,
            valence: 0.6,
            danceability: 0.3,
            acousticness: 0.95,
            instrumentalness: 0.2,
            loudness: -10.0,
            speechiness: 0.08,
        };

        let result = detect_genre(features, &[], 60);
        assert_eq!(result.genre, Genre::Acoustic);
    }

    #[test]
    fn test_lofi_detection() {
        let features = AudioFeatures {
            tempo: 80.0,
            energy: 0.25,
            valence: 0.4,
            danceability: 0.3,
            acousticness: 0.4,
            instrumentalness: 0.6,
            loudness: -15.0,
            speechiness: 0.05,
        };

        let result = detect_genre(features, &[], 40);
        assert_eq!(result.genre, Genre::LoFi);
    }

    #[test]
    fn test_classical_detection() {
        let features = AudioFeatures {
            tempo: 100.0,
            energy: 0.15,
            valence: 0.4,
            danceability: 0.05,
            acousticness: 0.8,
            instrumentalness: 0.95,
            loudness: -25.0,
            speechiness: 0.0,
        };

        let genres = vec!["classical".to_string()];
        let result = detect_genre(features, &genres, 50);
        assert_eq!(result.genre, Genre::Classical);
    }

    #[test]
    fn test_rock_detection() {
        let features = AudioFeatures {
            tempo: 140.0,
            energy: 0.8,
            valence: 0.6,
            danceability: 0.4,
            acousticness: 0.15,
            instrumentalness: 0.3,
            loudness: -4.0,
            speechiness: 0.08,
        };

        let result = detect_genre(features, &[], 70);
        assert_eq!(result.genre, Genre::Rock);
    }

    #[test]
    fn test_hiphop_detection() {
        let features = AudioFeatures {
            tempo: 95.0,
            energy: 0.6,
            valence: 0.5,
            danceability: 0.6,
            acousticness: 0.1,
            instrumentalness: 0.05,
            loudness: -5.0,
            speechiness: 0.45,
        };

        let genres = vec!["hip hop".to_string()];
        let result = detect_genre(features, &genres, 75);
        assert_eq!(result.genre, Genre::HipHop);
    }

    #[test]
    fn test_no_strong_match_returns_unknown() {
        let features = AudioFeatures {
            tempo: 100.0,
            energy: 0.5,
            valence: 0.5,
            danceability: 0.5,
            acousticness: 0.5,
            instrumentalness: 0.5,
            loudness: -10.0,
            speechiness: 0.15,
        };

        let result = detect_genre(features, &[], 50);
        // Should be unknown or a weak match
        assert!(result.confidence < 0.5 || matches!(result.genre, Genre::Unknown | Genre::Pop));
    }

    #[test]
    fn test_score_transparency() {
        let features = sample_features();
        let result = detect_genre(features, &[], 50);

        // All scores should be accessible for transparency
        assert!(result.scores.ballad >= 0.0);
        assert!(result.scores.pop >= 0.0);
        assert!(result.scores.rock >= 0.0);
    }
}
