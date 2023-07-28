use serde::Deserialize;

pub enum SupportedLanguageCode {
    English,
}

pub fn get_supported_language_string(
    supported_language_code: SupportedLanguageCode,
) -> &'static str {
    match supported_language_code {
        SupportedLanguageCode::English => "en",
    }
}

#[derive(Deserialize, Debug, Hash, Eq, PartialEq)]
pub enum LanguageCode {
    #[serde(rename = "ar")]
    Arabic,

    #[serde(rename = "bn")]
    Bengali,

    #[serde(rename = "bg")]
    Bulgarian,

    #[serde(rename = "my")]
    Burmese,

    #[serde(rename = "ca")]
    Catalan,

    #[serde(rename = "zh")]
    ChineseSimp,

    #[serde(rename = "zh-hk")]
    ChineseTrad,

    #[serde(rename = "cs")]
    Czech,

    #[serde(rename = "da")]
    Danish,

    #[serde(rename = "nl")]
    Dutch,

    #[serde(rename = "en")]
    English,

    #[serde(rename = "tl")]
    Filipino,

    #[serde(rename = "fi")]
    Finnish,

    #[serde(rename = "fr")]
    French,

    #[serde(rename = "de")]
    German,

    #[serde(rename = "el")]
    Greek,

    #[serde(rename = "he")]
    Hebrew,

    #[serde(rename = "hi")]
    Hindi,

    #[serde(rename = "hu")]
    Hungarian,

    #[serde(rename = "id")]
    Indonesian,

    #[serde(rename = "it")]
    Italian,

    #[serde(rename = "ja")]
    Japanese,

    #[serde(rename = "ja-ro")]
    JapaneseRomanized,

    #[serde(rename = "ko")]
    Korean,

    #[serde(rename = "lt")]
    Lithuanian,

    #[serde(rename = "ms")]
    Malay,

    #[serde(rename = "mn")]
    Mongolian,

    #[serde(rename = "no")]
    Norwegian,

    #[serde(rename = "fa")]
    Persian,

    #[serde(rename = "pl")]
    Polish,

    #[serde(rename = "pt-br")]
    PortugueseBr,

    #[serde(rename = "pt")]
    PortuguesePt,

    #[serde(rename = "ro")]
    Romanian,

    #[serde(rename = "ru")]
    Russian,

    #[serde(rename = "sr")]
    SerboCroatian,

    #[serde(rename = "es")]
    SpanishEs,

    #[serde(rename = "es-la")]
    SpanishLATAM,

    #[serde(rename = "sv")]
    Swedish,

    #[serde(rename = "th")]
    Thai,

    #[serde(rename = "tr")]
    Turkish,

    #[serde(rename = "uk")]
    Ukrainian,

    #[serde(rename = "vi")]
    Vietnamese,

    #[serde(rename = "NULL")]
    Null,
}
