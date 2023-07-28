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
