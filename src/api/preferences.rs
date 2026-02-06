use axum::{http::StatusCode, Json};
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct UserPreferences {
    pub theme: String,
    pub language: String,
    pub notifications_enabled: bool,
    pub sound_enabled: bool,
    pub default_timeframe: String,
    pub favorite_symbols: Vec<String>,
    pub risk_tolerance: String,
}

pub async fn get_preferences() -> Result<Json<UserPreferences>, StatusCode> {
    let prefs = UserPreferences {
        theme: "dark".to_string(),
        language: "fr".to_string(),
        notifications_enabled: true,
        sound_enabled: true,
        default_timeframe: "1h".to_string(),
        favorite_symbols: vec!["BTCUSD".to_string(), "ETHUSD".to_string()],
        risk_tolerance: "MEDIUM".to_string(),
    };

    Ok(Json(prefs))
}

pub async fn update_preferences(
    Json(prefs): Json<UserPreferences>,
) -> Result<Json<UserPreferences>, StatusCode> {
    Ok(Json(prefs))
}
