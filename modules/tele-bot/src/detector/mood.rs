/// Rule-based music mood detection system

use super::genre::AudioFeatures;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Mood {
    Happy,
    Sad,
    Energetic,
    Calm,
    Angry,
    Melancholic,
    Peaceful,
    Romantic,
    Unknown,
}

impl Mood {
    pub fn as_str(&self) -> &'static str {
        match self {
            Mood::Happy => "Happy",
            Mood::Sad => "Sad",
            Mood::Energetic => "Energetic",
            Mood::Calm => "Calm",
            Mood::Angry => "Angry",
            Mood::Melancholic => "Melancholic",
            Mood::Peaceful => "Peaceful",
            Mood::Romantic => "Romantic",
            Mood::Unknown => "Unknown",
        }
    }
}

/// Detection result with mood and confidence (0.0 to 1.0)
#[derive(Debug, Clone)]
pub struct MoodDetection {
    pub mood: Mood,
    pub confidence: f32,
    pub scores: MoodScores,
}

/// Detailed scores for each mood (for transparency)
#[derive(Debug, Clone)]
pub struct MoodScores {
    pub happy: f32,
    pub sad: f32,
    pub energetic: f32,
    pub calm: f32,
    pub angry: f32,
    pub melancholic: f32,
    pub peaceful: f32,
    pub romantic: f32,
}

/// Pure function: detect mood from audio features
///
/// # Arguments
/// * `features` - Audio features from Spotify
///
/// # Returns
/// `MoodDetection` with best matching mood and confidence score
pub fn detect_mood(features: AudioFeatures) -> MoodDetection {
    let scores = MoodScores {
        happy: score_happy(&features),
        sad: score_sad(&features),
        energetic: score_energetic(&features),
        calm: score_calm(&features),
        angry: score_angry(&features),
        melancholic: score_melancholic(&features),
        peaceful: score_peaceful(&features),
        romantic: score_romantic(&features),
    };

    // Normalize scores
    let max_score = [
        scores.happy,
        scores.sad,
        scores.energetic,
        scores.calm,
        scores.angry,
        scores.melancholic,
        scores.peaceful,
        scores.romantic,
    ]
    .iter()
    .copied()
    .fold(f32::NEG_INFINITY, f32::max);

    let (mood, confidence) = if max_score > 0.0 {
        // Normalize confidence to 0-1
        let norm_score = max_score / 8.0; // Max possible score

        if scores.happy == max_score {
            (Mood::Happy, norm_score)
        } else if scores.sad == max_score {
            (Mood::Sad, norm_score)
        } else if scores.energetic == max_score {
            (Mood::Energetic, norm_score)
        } else if scores.calm == max_score {
            (Mood::Calm, norm_score)
        } else if scores.angry == max_score {
            (Mood::Angry, norm_score)
        } else if scores.melancholic == max_score {
            (Mood::Melancholic, norm_score)
        } else if scores.peaceful == max_score {
            (Mood::Peaceful, norm_score)
        } else if scores.romantic == max_score {
            (Mood::Romantic, norm_score)
        } else {
            (Mood::Unknown, 0.0)
        }
    } else {
        (Mood::Unknown, 0.0)
    };

    MoodDetection {
        mood,
        confidence,
        scores,
    }
}

// ============================================================================
// MOOD SCORING FUNCTIONS
// ============================================================================

fn score_happy(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // High valence is the primary indicator of happiness
    if features.valence > 0.7 {
        score += 2.0;
    } else if features.valence > 0.5 {
        score += 1.0;
    }

    // Happy songs tend to be energetic and danceable
    if features.energy > 0.6 {
        score += 1.0;
    }
    if features.danceability > 0.6 {
        score += 1.0;
    }

    // Not too loud, moderate acousticness
    if features.loudness > -8.0 && features.loudness < 0.0 {
        score += 1.0;
    }

    score
}

fn score_sad(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Low valence indicates sadness
    if features.valence < 0.4 {
        score += 2.0;
    } else if features.valence < 0.6 {
        score += 1.0;
    }

    // Sad songs are typically slower
    if features.tempo < 90.0 {
        score += 1.0;
    }

    // Lower energy and danceability
    if features.energy < 0.5 {
        score += 1.0;
    }
    if features.danceability < 0.4 {
        score += 1.0;
    }

    // Often more acoustic
    if features.acousticness > 0.5 {
        score += 1.0;
    }

    score
}

fn score_energetic(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // High energy is the primary indicator
    if features.energy > 0.75 {
        score += 2.0;
    } else if features.energy > 0.6 {
        score += 1.0;
    }

    // Energetic songs are usually faster
    if features.tempo > 120.0 {
        score += 1.0;
    }

    // High danceability
    if features.danceability > 0.6 {
        score += 1.0;
    }

    // Louder
    if features.loudness > -6.0 {
        score += 1.0;
    }

    // Less acoustic (more electronic/produced)
    if features.acousticness < 0.3 {
        score += 1.0;
    }

    score
}

fn score_calm(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Low energy is key for calmness
    if features.energy < 0.4 {
        score += 2.0;
    } else if features.energy < 0.6 {
        score += 1.0;
    }

    // Slower tempo
    if features.tempo < 100.0 {
        score += 1.0;
    }

    // Low danceability
    if features.danceability < 0.4 {
        score += 1.0;
    }

    // Moderate to high valence (peaceful, not melancholic)
    if features.valence > 0.5 {
        score += 1.0;
    }

    // Quiet
    if features.loudness < -8.0 {
        score += 1.0;
    }

    score
}

fn score_angry(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Very high energy
    if features.energy > 0.8 {
        score += 2.0;
    } else if features.energy > 0.65 {
        score += 1.0;
    }

    // Very low valence (negative emotion)
    if features.valence < 0.3 {
        score += 2.0;
    } else if features.valence < 0.5 {
        score += 1.0;
    }

    // Faster tempo
    if features.tempo > 120.0 {
        score += 1.0;
    }

    // Loud
    if features.loudness > -5.0 {
        score += 1.0;
    }

    score
}

fn score_melancholic(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Low valence combined with moderate energy
    if features.valence < 0.5 {
        score += 1.0;
    }

    // Moderate to low energy (contemplative)
    if features.energy < 0.6 && features.energy > 0.3 {
        score += 1.0;
    }

    // Slower tempo
    if features.tempo < 110.0 {
        score += 1.0;
    }

    // Higher acousticness
    if features.acousticness > 0.4 {
        score += 1.0;
    }

    // Lower danceability
    if features.danceability < 0.5 {
        score += 1.0;
    }

    // Some instrumentalness (often reflective)
    if features.instrumentalness > 0.2 {
        score += 1.0;
    }

    score
}

fn score_peaceful(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Low energy
    if features.energy < 0.45 {
        score += 2.0;
    } else if features.energy < 0.6 {
        score += 1.0;
    }

    // Slow tempo
    if features.tempo < 95.0 {
        score += 1.0;
    }

    // High valence (positive, uplifting)
    if features.valence > 0.6 {
        score += 1.0;
    }

    // Very quiet
    if features.loudness < -10.0 {
        score += 1.0;
    }

    // Often acoustic or instrumental
    if features.acousticness > 0.5 || features.instrumentalness > 0.4 {
        score += 1.0;
    }

    score
}

fn score_romantic(features: &AudioFeatures) -> f32 {
    let mut score = 0.0;

    // Moderate to high valence (positive emotion)
    if features.valence > 0.6 {
        score += 1.0;
    }

    // Slower to moderate tempo
    if features.tempo >= 80.0 && features.tempo <= 110.0 {
        score += 1.0;
    }

    // Lower to moderate energy (smooth, not aggressive)
    if features.energy < 0.65 {
        score += 1.0;
    }

    // Moderate danceability
    if features.danceability > 0.4 && features.danceability < 0.7 {
        score += 1.0;
    }

    // Often more acoustic
    if features.acousticness > 0.3 {
        score += 1.0;
    }

    // Not too loud (intimate feel)
    if features.loudness < -4.0 {
        score += 1.0;
    }

    // Low speechiness (vocal/instrumental focus)
    if features.speechiness < 0.2 {
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
    fn test_happy_detection() {
        let features = AudioFeatures {
            tempo: 130.0,
            energy: 0.8,
            valence: 0.85,
            danceability: 0.75,
            acousticness: 0.2,
            instrumentalness: 0.1,
            loudness: -5.0,
            speechiness: 0.05,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Happy);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_sad_detection() {
        let features = AudioFeatures {
            tempo: 70.0,
            energy: 0.3,
            valence: 0.2,
            danceability: 0.2,
            acousticness: 0.7,
            instrumentalness: 0.1,
            loudness: -12.0,
            speechiness: 0.05,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Sad);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_energetic_detection() {
        let features = AudioFeatures {
            tempo: 140.0,
            energy: 0.9,
            valence: 0.7,
            danceability: 0.8,
            acousticness: 0.1,
            instrumentalness: 0.2,
            loudness: -4.0,
            speechiness: 0.1,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Energetic);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_calm_detection() {
        let features = AudioFeatures {
            tempo: 80.0,
            energy: 0.3,
            valence: 0.65,
            danceability: 0.25,
            acousticness: 0.6,
            instrumentalness: 0.3,
            loudness: -12.0,
            speechiness: 0.05,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Calm);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_angry_detection() {
        let features = AudioFeatures {
            tempo: 135.0,
            energy: 0.88,
            valence: 0.2,
            danceability: 0.5,
            acousticness: 0.1,
            instrumentalness: 0.1,
            loudness: -3.0,
            speechiness: 0.2,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Angry);
        assert!(result.confidence > 0.3);
    }

    #[test]
    fn test_romantic_detection() {
        let features = AudioFeatures {
            tempo: 95.0,
            energy: 0.45,
            valence: 0.7,
            danceability: 0.55,
            acousticness: 0.5,
            instrumentalness: 0.2,
            loudness: -7.0,
            speechiness: 0.08,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Romantic);
        assert!(result.confidence > 0.35);
    }

    #[test]
    fn test_peaceful_detection() {
        let features = AudioFeatures {
            tempo: 60.0,
            energy: 0.2,
            valence: 0.75,
            danceability: 0.15,
            acousticness: 0.8,
            instrumentalness: 0.6,
            loudness: -15.0,
            speechiness: 0.02,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Peaceful);
        assert!(result.confidence > 0.4);
    }

    #[test]
    fn test_melancholic_detection() {
        let features = AudioFeatures {
            tempo: 85.0,
            energy: 0.45,
            valence: 0.35,
            danceability: 0.35,
            acousticness: 0.65,
            instrumentalness: 0.4,
            loudness: -10.0,
            speechiness: 0.1,
        };

        let result = detect_mood(features);
        assert_eq!(result.mood, Mood::Melancholic);
        assert!(result.confidence > 0.35);
    }

    #[test]
    fn test_score_transparency() {
        let features = sample_features();
        let result = detect_mood(features);

        // All scores should be accessible for transparency
        assert!(result.scores.happy >= 0.0);
        assert!(result.scores.sad >= 0.0);
        assert!(result.scores.energetic >= 0.0);
        assert!(result.scores.calm >= 0.0);
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
            speechiness: 0.5,
        };

        let result = detect_mood(features);
        assert!(result.confidence < 0.4 || result.mood == Mood::Happy || result.mood == Mood::Calm);
    }
}
