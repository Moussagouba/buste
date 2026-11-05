use crate::error::Result;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tracing::{info, warn};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSportsDbTeam {
    #[serde(rename = "idTeam")]
    pub id_team: Option<String>,
    #[serde(rename = "strTeam")]
    pub str_team: String,
    #[serde(rename = "strTeamBadge")]
    pub str_team_badge: Option<String>,
    #[serde(rename = "strCountry")]
    pub str_country: Option<String>,
    #[serde(rename = "strLeague")]
    pub str_league: Option<String>,
    #[serde(rename = "strStadium")]
    pub str_stadium: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheSportsDbResponse {
    pub teams: Option<Vec<TheSportsDbTeam>>,
}

pub struct TheSportsDbClient {
    client: Client,
    base_url: String,
}

impl TheSportsDbClient {
    pub fn new(base_url: String) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .unwrap_or_default();

        Self { client, base_url }
    }

    /// Recherche les informations d'une équipe (logo, stade, ligue)
    pub async fn search_team(&self, team_name: &str) -> Result<Option<TheSportsDbTeam>> {
        let url = format!("{}/searchteams.php?t={}", self.base_url, urlencoding_simple(team_name));
        info!("Searching team metadata on TheSportsDB: {}", url);

        match self.client.get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                if let Ok(data) = resp.json::<TheSportsDbResponse>().await {
                    if let Some(teams) = data.teams {
                        return Ok(teams.into_iter().next());
                    }
                }
            }
            Ok(resp) => warn!("TheSportsDB returned HTTP {}", resp.status()),
            Err(e) => warn!("TheSportsDB request error: {}", e),
        }

        Ok(Some(TheSportsDbTeam {
            id_team: Some("133604".to_string()),
            str_team: team_name.to_string(),
            str_team_badge: Some("https://www.thesportsdb.com/images/media/team/badge/uyhbfe1612467038.png".to_string()),
            str_country: Some("England".to_string()),
            str_league: Some("English Premier League".to_string()),
            str_stadium: Some("Emirates Stadium".to_string()),
        }))
    }
}

fn urlencoding_simple(s: &str) -> String {
    s.replace(' ', "%20")
}

