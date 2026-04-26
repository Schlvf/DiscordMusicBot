use crate::errors::BotError;

pub fn load_string<T: AsRef<str>>(key: T) -> Result<String, BotError> {
    dotenvy::var(key.as_ref()).map_err(|e| {
        BotError::Environment(format!(
            "Failed to load {} from environment: {}",
            key.as_ref(),
            e
        ))
    })
}
