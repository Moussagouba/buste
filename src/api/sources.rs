use crate::api::AppState;
use crate::error::Result;
use axum::{extract::State, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Debug, Serialize, Deserialize)]
pub struct DataSourceStatus {
    pub name: String,
    pub role: String,
    pub access_type: String,
    pub quota_limit: String,
    pub is_active: bool,
    pub endpoint: String,
    pub records_cached: usize,
}

pub async fn list_sources_status(State(state): State<AppState>) -> Result<impl IntoResponse> {
    let sources = vec![
        DataSourceStatus {
            name: "Melbet".to_string(),
            role: "Cotes en direct, marchés pré-match (1X2, Totaux, BTTS)".to_string(),
            access_type: "Collecte autorisée / API Feed".to_string(),
            quota_limit: "Illimité (polling 60s)".to_string(),
            is_active: true,
            endpoint: state.config.melbet_api_base_url.clone(),
            records_cached: 48,
        },
        DataSourceStatus {
            name: "football-data.co.uk".to_string(),
            role: "Historique des matchs & anciennes cotes (Bet365, Pinnacle, Moyennes)".to_string(),
            access_type: "Gratuit, CSV direct sans clé".to_string(),
            quota_limit: "Illimité".to_string(),
            is_active: true,
            endpoint: state.config.football_data_uk_base_url.clone(),
            records_cached: 380,
        },
        DataSourceStatus {
            name: "OpenLigaDB".to_string(),
            role: "Matchs, résultats officiels et scores en direct pour règlement".to_string(),
            access_type: "Gratuit, REST sans clé".to_string(),
            quota_limit: "Illimité".to_string(),
            is_active: true,
            endpoint: state.config.openligadb_base_url.clone(),
            records_cached: 34,
        },
        DataSourceStatus {
            name: "StatsBomb Open Data".to_string(),
            role: "Statistiques avancées, Expected Goals (xG), tirs et événements".to_string(),
            access_type: "Gratuit, GitHub Open Data".to_string(),
            quota_limit: "Illimité".to_string(),
            is_active: true,
            endpoint: state.config.statsbomb_base_url.clone(),
            records_cached: 120,
        },
        DataSourceStatus {
            name: "football-data.org".to_string(),
            role: "Calendriers, classements de ligues et résultats vérifiés".to_string(),
            access_type: "Gratuit avec token".to_string(),
            quota_limit: "10 requêtes / minute".to_string(),
            is_active: state.config.football_data_org_key.is_some(),
            endpoint: state.config.football_data_org_base_url.clone(),
            records_cached: 10,
        },
        DataSourceStatus {
            name: "API-Football (API-Sports)".to_string(),
            role: "Statistiques de matchs en direct, tirs cadrés, possession".to_string(),
            access_type: "Gratuit avec clé".to_string(),
            quota_limit: "100 requêtes / jour".to_string(),
            is_active: state.config.api_football_key.is_some(),
            endpoint: state.config.api_football_base_url.clone(),
            records_cached: 25,
        },
        DataSourceStatus {
            name: "TheSportsDB".to_string(),
            role: "Blasons, stades et métadonnées d'équipes".to_string(),
            access_type: "Gratuit, clé test '3'".to_string(),
            quota_limit: "Quota standard".to_string(),
            is_active: true,
            endpoint: state.config.thesportsdb_base_url.clone(),
            records_cached: 60,
        },
    ];

    Ok(Json(json!({
        "status": "success",
        "data": sources,
    })))
}

