pub mod api_football;
pub mod football_data_org;
pub mod football_data_uk;
pub mod openligadb;
pub mod statsbomb;
pub mod thesportsdb;

pub use api_football::ApiFootballClient;
pub use football_data_org::FootballDataOrgClient;
pub use football_data_uk::FootballDataUkClient;
pub use openligadb::OpenLigaDbClient;
pub use statsbomb::StatsBombClient;
pub use thesportsdb::TheSportsDbClient;

