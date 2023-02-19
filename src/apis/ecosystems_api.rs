/*
 * MetaFab API
 *
 * Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.5.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`approve_ecosystem_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ApproveEcosystemGameError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_ecosystem`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthEcosystemError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_profile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthProfileError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`auth_profile_player`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AuthProfilePlayerError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_ecosystem`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateEcosystemError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_profile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateProfileError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_profile_player`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateProfilePlayerError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ecosystem`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEcosystemError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ecosystem_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEcosystemGameError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_ecosystem_games`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetEcosystemGamesError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_profile_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProfileGameError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_profile_games`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetProfileGamesError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`unapprove_ecosystem_game`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UnapproveEcosystemGameError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_ecosystem`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateEcosystemError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_profile`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateProfileError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_profile_player`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateProfilePlayerError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}


/// Approves a game for an ecosystem. By approving a game, it allows that game to integrate the ability for profile accounts from an ecosystem to login directly to the approved game and play. This also allows games to request access to assets held at the profile level for the game to frictionlessly interact with on behalf of the profile.
pub async fn approve_ecosystem_game(configuration: &configuration::Configuration, ecosystem_id: &str, x_authorization: &str, approve_ecosystem_game_request: crate::models::ApproveEcosystemGameRequest) -> Result<(), Error<ApproveEcosystemGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}/games", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&approve_ecosystem_game_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<ApproveEcosystemGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an existing ecosystem object containing authorization keys when provided a valid email (in place of username) and password login using Basic Auth.
pub async fn auth_ecosystem(configuration: &configuration::Configuration, ) -> Result<crate::models::EcosystemModel, Error<AuthEcosystemError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/auth", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthEcosystemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an existing profile object containing access token, wallet, and other details when provided a valid profile username and password login using Basic Auth.
pub async fn auth_profile(configuration: &configuration::Configuration, x_ecosystem_key: &str) -> Result<crate::models::AuthProfile200Response, Error<AuthProfileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/auth", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Ecosystem-Key", x_ecosystem_key.to_string());
    if let Some(ref local_var_auth_conf) = local_var_configuration.basic_auth {
        local_var_req_builder = local_var_req_builder.basic_auth(local_var_auth_conf.0.to_owned(), local_var_auth_conf.1.to_owned());
    };

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthProfileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an existing player object containing access token, wallet, wallet decrypt key, profile authorization and other details for a game when provided profile authentication and the player's username.
pub async fn auth_profile_player(configuration: &configuration::Configuration, profile_id: &str, game_id: &str, x_authorization: &str, x_wallet_decrypt_key: &str, x_username: &str) -> Result<crate::models::AuthPlayer200Response, Error<AuthProfilePlayerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}/games/{gameId}/players/auth", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id), gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Wallet-Decrypt-Key", x_wallet_decrypt_key.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Username", x_username.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<AuthProfilePlayerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new ecosystem. An ecosystem is a parent entity that many profiles live under for a given ecosystem of games. Ecosystems allow your players to create one profile within your ecosystem that allows a single account and wallet to be used across all of the approved games in your ecosystem that they play.
pub async fn create_ecosystem(configuration: &configuration::Configuration, create_ecosystem_request: crate::models::CreateEcosystemRequest) -> Result<crate::models::EcosystemModel, Error<CreateEcosystemError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.json(&create_ecosystem_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateEcosystemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Create a new profile. Profiles are automatically associated with an internally managed wallet. Profiles can be thought of as a umbrella account that can be used to sign into and create player accounts across many games and have a singular asset store wallet at the profile level that can be used across all connected player accounts for games those player accounts are a part of.  Profiles are associated to a parent ecosystem of games. This allows an ecosystem to approve a permissioned set of games that can request authorized wallet permissions from profiles of players for their game.
pub async fn create_profile(configuration: &configuration::Configuration, x_ecosystem_key: &str, create_profile_request: crate::models::CreateProfileRequest) -> Result<crate::models::AuthProfile200Response, Error<CreateProfileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Ecosystem-Key", x_ecosystem_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&create_profile_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateProfileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new player account for the provided game id linked to the authenticating profile. The created player account will default to using the parent profile's wallet for any transactions, wallet content balance checks and verifications, and more.
pub async fn create_profile_player(configuration: &configuration::Configuration, profile_id: &str, game_id: &str, x_authorization: &str, x_wallet_decrypt_key: &str, create_profile_player_request: crate::models::CreateProfilePlayerRequest) -> Result<crate::models::AuthPlayer200Response, Error<CreateProfilePlayerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}/games/{gameId}/players", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id), gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Wallet-Decrypt-Key", x_wallet_decrypt_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&create_profile_player_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateProfilePlayerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a ecosystem object for the provided ecosystem id.
pub async fn get_ecosystem(configuration: &configuration::Configuration, ecosystem_id: &str) -> Result<crate::models::PublicEcosystem, Error<GetEcosystemError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEcosystemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a game object for the provided game id that the ecosystem has approved.
pub async fn get_ecosystem_game(configuration: &configuration::Configuration, ecosystem_id: &str, game_id: &str) -> Result<crate::models::PublicGame, Error<GetEcosystemGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}/games/{gameId}", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id), gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEcosystemGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of games the ecosystem has approved.
pub async fn get_ecosystem_games(configuration: &configuration::Configuration, ecosystem_id: &str) -> Result<Vec<crate::models::PublicGame>, Error<GetEcosystemGamesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}/games", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetEcosystemGamesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a game this profile has connected player accounts for.
pub async fn get_profile_game(configuration: &configuration::Configuration, profile_id: &str, game_id: &str, x_authorization: &str) -> Result<crate::models::GetProfileGames200ResponseInner, Error<GetProfileGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}/games/{gameId}", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id), gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProfileGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of games the authorized profile has connected player accounts for.
pub async fn get_profile_games(configuration: &configuration::Configuration, profile_id: &str, x_authorization: &str) -> Result<Vec<crate::models::GetProfileGames200ResponseInner>, Error<GetProfileGamesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}/games", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetProfileGamesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Unapproves a game for an ecosystem. The game will no longer be able to allow profiles from the ecosystem to login. All profile permissions approved for the game will also be revoked.
pub async fn unapprove_ecosystem_game(configuration: &configuration::Configuration, ecosystem_id: &str, game_id: &str, x_authorization: &str) -> Result<(), Error<UnapproveEcosystemGameError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}/games/{gameId}", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id), gameId=crate::apis::urlencode(game_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        Ok(())
    } else {
        let local_var_entity: Option<UnapproveEcosystemGameError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update various fields specific to an ecosystem. Such as changing its password, resetting its published or secret key, or updating its approved games.
pub async fn update_ecosystem(configuration: &configuration::Configuration, ecosystem_id: &str, x_authorization: &str, update_ecosystem_request: crate::models::UpdateEcosystemRequest) -> Result<crate::models::EcosystemModel, Error<UpdateEcosystemError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/ecosystems/{ecosystemId}", local_var_configuration.base_path, ecosystemId=crate::apis::urlencode(ecosystem_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&update_ecosystem_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateEcosystemError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update various fields specific to a profile. Such as changing its password and resetting its access token.
pub async fn update_profile(configuration: &configuration::Configuration, profile_id: &str, x_authorization: &str, update_profile_request: crate::models::UpdateProfileRequest) -> Result<crate::models::ProfileModel, Error<UpdateProfileError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.json(&update_profile_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateProfileError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Update various fields specific to a player. Such as changing its permissions.
pub async fn update_profile_player(configuration: &configuration::Configuration, profile_id: &str, game_id: &str, player_id: &str, x_authorization: &str, x_wallet_decrypt_key: &str, update_profile_player_request: crate::models::UpdateProfilePlayerRequest) -> Result<crate::models::UpdateProfilePlayer200Response, Error<UpdateProfilePlayerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/profiles/{profileId}/games/{gameId}/players/{playerId}", local_var_configuration.base_path, profileId=crate::apis::urlencode(profile_id), gameId=crate::apis::urlencode(game_id), playerId=crate::apis::urlencode(player_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::PATCH, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Wallet-Decrypt-Key", x_wallet_decrypt_key.to_string());
    local_var_req_builder = local_var_req_builder.json(&update_profile_player_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<UpdateProfilePlayerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

