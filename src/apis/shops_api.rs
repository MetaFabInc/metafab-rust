/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.4.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`create_shop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateShopError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shop_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShopOfferError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shop_offers`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShopOffersError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_shops`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetShopsError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`remove_shop_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RemoveShopOfferError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_shop_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetShopOfferError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`use_shop_offer`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UseShopOfferError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`withdraw_from_shop`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WithdrawFromShopError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}


/// Creates a new game shop and deploys a shop contract on behalf of the authenticating game's primary wallet. The deployed shop contract allows you to create fixed price rates for players to buy specific items from any item collection or ERC1155 contract. Additionally, a shop allows you to create shop offers for some set of item(s) to another set of item(s) or any mix of currency. Shops completely support gasless player transactions.
pub async fn create_shop(configuration: &configuration::Configuration, x_authorization: &str, x_password: &str, create_shop_request: crate::models::CreateShopRequest) -> Result<crate::models::CreateShop200Response, Error<CreateShopError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&create_shop_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateShopError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns a shop offer object for the provided shopOfferId.
pub async fn get_shop_offer(configuration: &configuration::Configuration, shop_id: &str, shop_offer_id: &str) -> Result<crate::models::ShopOffer, Error<GetShopOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/items/{shopOfferId}", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id), shopOfferId=crate::apis::urlencode(shop_offer_id));
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
        let local_var_entity: Option<GetShopOfferError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns all shop offers as an array of shop offer objects.
pub async fn get_shop_offers(configuration: &configuration::Configuration, shop_id: &str) -> Result<Vec<crate::models::ShopOffer>, Error<GetShopOffersError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/offers", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id));
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
        let local_var_entity: Option<GetShopOffersError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of active shops for the game associated with the provided `X-Game-Key`.
pub async fn get_shops(configuration: &configuration::Configuration, x_game_key: &str) -> Result<Vec<crate::models::GetShops200ResponseInner>, Error<GetShopsError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetShopsError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes the provided offer by offerId from the provided shop. Removed offers can no longer be used.
pub async fn remove_shop_offer(configuration: &configuration::Configuration, shop_id: &str, shop_offer_id: &str, x_authorization: &str, x_password: &str) -> Result<crate::models::TransactionModel, Error<RemoveShopOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/offers/{shopOfferId}", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id), shopOfferId=crate::apis::urlencode(shop_offer_id));
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
        let local_var_entity: Option<RemoveShopOfferError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets a new shop offer or updates an existing one for the provided id. Shop offers allow currency to item, item to currency or item to item exchanges.  All request fields besides `id` are optional. Any optional fields omitted will not be used for the offer. This allows you to create many different combinations of offers. For example, you can create an offer that may require 3 unique item ids of specified quantities from a given item collection and gives the user 1 new unique item id in exchange.  Another example, you may want to make a shop offer from one ERC20 token to another. This is also possible - simple set the input and output currency fields and leave the others blank.
pub async fn set_shop_offer(configuration: &configuration::Configuration, shop_id: &str, x_authorization: &str, x_password: &str, set_shop_offer_request: crate::models::SetShopOfferRequest) -> Result<crate::models::TransactionModel, Error<SetShopOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/offers", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&set_shop_offer_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetShopOfferError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Uses a shop offer. The required (input) item(s) and/or currency are removed from the wallet or player wallet using the offer. The given (output) item(s) and/or currency are given to the wallet or player wallet using the offer.
pub async fn use_shop_offer(configuration: &configuration::Configuration, shop_id: &str, shop_offer_id: &str, x_authorization: &str, x_password: &str) -> Result<crate::models::TransactionModel, Error<UseShopOfferError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/offers/{shopOfferId}/uses", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id), shopOfferId=crate::apis::urlencode(shop_offer_id));
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
        let local_var_entity: Option<UseShopOfferError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Withdraws native token, currency or items from a shop. Whenever a shop offer has input requirements, the native tokens, currencies or items for the requirements of that offer are deposited into the shop contract when the offer is used. These can be withdrawn to any other address.
pub async fn withdraw_from_shop(configuration: &configuration::Configuration, shop_id: &str, x_authorization: &str, x_password: &str, withdraw_from_shop_request: crate::models::WithdrawFromShopRequest) -> Result<crate::models::TransactionModel, Error<WithdrawFromShopError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/shops/{shopId}/withdrawals", local_var_configuration.base_path, shopId=crate::apis::urlencode(shop_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&withdraw_from_shop_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<WithdrawFromShopError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

