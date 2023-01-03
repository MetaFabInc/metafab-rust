/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.1
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_lootbox_manager`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateLootboxManagerError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lootbox_manager_lootbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLootboxManagerLootboxError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lootbox_manager_lootboxes`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLootboxManagerLootboxesError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_lootbox_managers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetLootboxManagersError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`open_lootbox_manager_lootbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenLootboxManagerLootboxError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_lootbox_manager_lootbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveLootboxManagerLootboxError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_lootbox_manager_lootbox`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetLootboxManagerLootboxError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}


/// Creates a new game lootbox manager and deploys a lootbox manager contract on behalf of the authenticating game's primary wallet. The deployed lootbox manager contract allows you to create lootbox behavior for existing items. For example, you can define item id(s) from one of your item collections as the requirement(s) to open a \"lootbox\". The required item(s) would be burned from the interacting player's wallet and the player would receive item(s) from a weighted randomized set of possible items the lootbox can contain.
pub async fn create_lootbox_manager(configuration: &configuration::Configuration, x_authorization: &str, x_password: &str, create_lootbox_manager_request: crate::models::CreateLootboxManagerRequest) -> Result<crate::models::CreateLootboxManager200Response, Error<CreateLootboxManagerError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&create_lootbox_manager_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateLootboxManagerError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a lootbox manager lootbox object for the provided lootboxManagerLootboxId.
pub async fn get_lootbox_manager_lootbox(configuration: &configuration::Configuration, lootbox_manager_id: &str, lootbox_manager_lootbox_id: &str) -> Result<crate::models::LootboxManagerLootbox, Error<GetLootboxManagerLootboxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId}", local_var_configuration.base_path, lootboxManagerId=crate::apis::urlencode(lootbox_manager_id), lootboxManagerLootboxId=crate::apis::urlencode(lootbox_manager_lootbox_id));
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
        let local_var_entity: Option<GetLootboxManagerLootboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all lootbox manager lootboxes as an array of lootbox objects.
pub async fn get_lootbox_manager_lootboxes(configuration: &configuration::Configuration, lootbox_manager_id: &str) -> Result<Vec<crate::models::LootboxManagerLootbox>, Error<GetLootboxManagerLootboxesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers/{lootboxManagerId}/lootboxes", local_var_configuration.base_path, lootboxManagerId=crate::apis::urlencode(lootbox_manager_id));
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
        let local_var_entity: Option<GetLootboxManagerLootboxesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of active lootbox managers for the game associated with the provided `X-Game-Key`.
pub async fn get_lootbox_managers(configuration: &configuration::Configuration, x_game_key: &str) -> Result<Vec<crate::models::GetLootboxManagers200ResponseInner>, Error<GetLootboxManagersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Game-Key", x_game_key.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<GetLootboxManagersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Opens a lootbox manager lootbox. The required input item(s) are burned from the wallet or player wallet opening the lootbox. The given output item(s) are given to the wallet or player wallet opening the lootbox.
pub async fn open_lootbox_manager_lootbox(configuration: &configuration::Configuration, lootbox_manager_id: &str, lootbox_manager_lootbox_id: &str, x_authorization: &str, x_password: &str) -> Result<Vec<crate::models::TransactionModel>, Error<OpenLootboxManagerLootboxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId}/opens", local_var_configuration.base_path, lootboxManagerId=crate::apis::urlencode(lootbox_manager_id), lootboxManagerLootboxId=crate::apis::urlencode(lootbox_manager_lootbox_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<OpenLootboxManagerLootboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes the provided lootbox by lootboxId from the provided lootbox manager. Removed lootboxes can no longer be used.
pub async fn remove_lootbox_manager_lootbox(configuration: &configuration::Configuration, lootbox_manager_id: &str, lootbox_manager_lootbox_id: &str, x_authorization: &str, x_password: &str) -> Result<crate::models::TransactionModel, Error<RemoveLootboxManagerLootboxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers/{lootboxManagerId}/lootboxes/{lootboxManagerLootboxId}", local_var_configuration.base_path, lootboxManagerId=crate::apis::urlencode(lootbox_manager_id), lootboxManagerLootboxId=crate::apis::urlencode(lootbox_manager_lootbox_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<RemoveLootboxManagerLootboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a new lootbox manager lootbox or updates an existing one for the provided id. Lootboxes allow item(s) to be burned to receive a random set of possible item(s) based on probability weight.  Lootboxes can require any number of unique types of items and quantities to open a created lootbox type within the system. A common pattern with lootboxes is to create a lootbox item type within an item collection, and require it as the input item type.
pub async fn set_lootbox_manager_lootbox(configuration: &configuration::Configuration, lootbox_manager_id: &str, x_authorization: &str, x_password: &str, set_lootbox_manager_lootbox_request: crate::models::SetLootboxManagerLootboxRequest) -> Result<crate::models::TransactionModel, Error<SetLootboxManagerLootboxError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/lootboxManagers/{lootboxManagerId}/lootboxes", local_var_configuration.base_path, lootboxManagerId=crate::apis::urlencode(lootbox_manager_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&set_lootbox_manager_lootbox_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetLootboxManagerLootboxError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

