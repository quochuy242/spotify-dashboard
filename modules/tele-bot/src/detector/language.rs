/// Language detection based on artist metadata

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Language {
    English,
    Spanish,
    French,
    Japanese,
    Korean,
    Chinese,
    Vietnamese,
    Thai,
    Hindi,
    Unknown,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::English => "English",
            Language::Spanish => "Spanish",
            Language::French => "French",
            Language::Japanese => "Japanese",
            Language::Korean => "Korean",
            Language::Chinese => "Chinese",
            Language::Vietnamese => "Vietnamese",
            Language::Thai => "Thai",
            Language::Hindi => "Hindi",
            Language::Unknown => "Unknown",
        }
    }

    pub fn code(&self) -> &'static str {
        match self {
            Language::English => "en",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Japanese => "ja",
            Language::Korean => "ko",
            Language::Chinese => "zh",
            Language::Vietnamese => "vi",
            Language::Thai => "th",
            Language::Hindi => "hi",
            Language::Unknown => "unknown",
        }
    }
}

/// Detection result with language
#[derive(Debug, Clone)]
pub struct LanguageDetection {
    pub language: Language,
    pub country_code: Option<String>,
}

/// Detect language from artist's country code
///
/// # Arguments
/// * `country_code` - ISO 3166-1 alpha-2 country code (e.g., "US", "GB", "JP")
///
/// # Returns
/// `LanguageDetection` with detected language and original country code
pub fn detect_language_from_country(country_code: Option<&str>) -> LanguageDetection {
    let language = match country_code {
        Some(code) => country_to_language(code),
        None => Language::Unknown,
    };

    LanguageDetection {
        language,
        country_code: country_code.map(|s| s.to_string()),
    }
}

/// Map country code to primary language
fn country_to_language(country_code: &str) -> Language {
    let code_upper = country_code.to_uppercase();

    match code_upper.as_str() {
        // English-speaking countries
        "US" | "GB" | "AU" | "NZ" | "CA" | "IE" | "ZA" => Language::English,

        // Spanish-speaking countries
        "ES" | "MX" | "AR" | "CO" | "CL" | "PE" | "VE" | "CU" => Language::Spanish,

        // French-speaking countries
        "FR" | "BE" | "CH" | "CA" | "SN" | "CG" | "CD" => Language::French,

        // Asian countries
        "JP" => Language::Japanese,
        "KR" => Language::Korean,
        "CN" | "HK" | "TW" | "SG" => Language::Chinese,
        "VN" => Language::Vietnamese,
        "TH" => Language::Thai,
        "IN" => Language::Hindi,

        // Default to Unknown
        _ => Language::Unknown,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_detect_english_countries() {
        assert_eq!(
            detect_language_from_country(Some("US")).language,
            Language::English
        );
        assert_eq!(
            detect_language_from_country(Some("GB")).language,
            Language::English
        );
        assert_eq!(
            detect_language_from_country(Some("AU")).language,
            Language::English
        );
    }

    #[test]
    fn test_detect_spanish_countries() {
        assert_eq!(
            detect_language_from_country(Some("ES")).language,
            Language::Spanish
        );
        assert_eq!(
            detect_language_from_country(Some("MX")).language,
            Language::Spanish
        );
        assert_eq!(
            detect_language_from_country(Some("AR")).language,
            Language::Spanish
        );
    }

    #[test]
    fn test_detect_asian_countries() {
        assert_eq!(
            detect_language_from_country(Some("JP")).language,
            Language::Japanese
        );
        assert_eq!(
            detect_language_from_country(Some("KR")).language,
            Language::Korean
        );
        assert_eq!(
            detect_language_from_country(Some("VN")).language,
            Language::Vietnamese
        );
        assert_eq!(
            detect_language_from_country(Some("CN")).language,
            Language::Chinese
        );
    }

    #[test]
    fn test_detect_european_countries() {
        assert_eq!(
            detect_language_from_country(Some("FR")).language,
            Language::French
        );
    }

    #[test]
    fn test_case_insensitive() {
        assert_eq!(
            detect_language_from_country(Some("us")).language,
            Language::English
        );
        assert_eq!(
            detect_language_from_country(Some("jp")).language,
            Language::Japanese
        );
    }

    #[test]
    fn test_unknown_country() {
        assert_eq!(
            detect_language_from_country(Some("XX")).language,
            Language::Unknown
        );
    }

    #[test]
    fn test_none_country() {
        assert_eq!(
            detect_language_from_country(None).language,
            Language::Unknown
        );
    }

    #[test]
    fn test_language_code() {
        assert_eq!(Language::English.code(), "en");
        assert_eq!(Language::Spanish.code(), "es");
        assert_eq!(Language::Vietnamese.code(), "vi");
        assert_eq!(Language::Japanese.code(), "ja");
    }

    #[test]
    fn test_language_str() {
        assert_eq!(Language::English.as_str(), "English");
        assert_eq!(Language::Vietnamese.as_str(), "Vietnamese");
        assert_eq!(Language::Unknown.as_str(), "Unknown");
    }

    #[test]
    fn test_country_code_preservation() {
        let result = detect_language_from_country(Some("US"));
        assert_eq!(result.country_code, Some("US".to_string()));
        assert_eq!(result.language, Language::English);
    }
}
