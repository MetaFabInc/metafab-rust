/*
 * MetaFab API
 *
 *  Complete MetaFab API references and guides can be found at: https://trymetafab.com
 *
 * The version of the OpenAPI document: 1.1.0
 * Contact: metafabproject@gmail.com
 * Generated by: https://openapi-generator.tech
 */


use reqwest;

use crate::apis::ResponseContent;
use super::{Error, configuration};


/// struct for typed errors of method [`batch_transfer_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BatchTransferCurrencyError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`burn_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BurnCurrencyError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateCurrencyError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_currencies`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrenciesError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_currency_balance`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrencyBalanceError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_currency_fees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetCurrencyFeesError {
    Status400(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`mint_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum MintCurrencyError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`set_currency_fees`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SetCurrencyFeesError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`transfer_currency`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TransferCurrencyError {
    Status400(String),
    Status401(String),
    UnknownValue(serde_json::Value),
}


/// Transfers multiple amounts of currency to multiple provided wallet addresses or wallet addresses associated with the provided walletIds. You may also provide a combination of addresses and walletIds in one request, the proper receipients will be automatically determined, with `addresses` getting `amounts` order priority first.  Optional references may be included for the transfer. References are useful for identifying transfers intended to pay for items, trades, services and more.
pub async fn batch_transfer_currency(configuration: &configuration::Configuration, currency_id: &str, x_authorization: &str, x_password: &str, batch_transfer_currency_request: crate::models::BatchTransferCurrencyRequest) -> Result<crate::models::TransactionModel, Error<BatchTransferCurrencyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/batchTransfers", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&batch_transfer_currency_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BatchTransferCurrencyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Removes (burns) the provided amount of currency from the authenticating game or players wallet. The currency amount is permanently removed from the circulating supply of the currency.
pub async fn burn_currency(configuration: &configuration::Configuration, currency_id: &str, x_authorization: &str, x_password: &str, burn_currency_request: crate::models::BurnCurrencyRequest) -> Result<crate::models::TransactionModel, Error<BurnCurrencyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/burns", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&burn_currency_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<BurnCurrencyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates a new game currency and deploys an ERC20 token contract on behalf of the authenticating game's primary wallet. The deployed ERC20 contract is preconfigured to fully support bridging across blockchains, batched transfers and gasless transactions on any supported blockchain as well as full support for gasless transactions from player managed wallets.
pub async fn create_currency(configuration: &configuration::Configuration, x_authorization: &str, x_password: &str, create_currency_request: crate::models::CreateCurrencyRequest) -> Result<crate::models::CreateCurrency200Response, Error<CreateCurrencyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies", local_var_configuration.base_path);
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&create_currency_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<CreateCurrencyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns an array of active currencies for the game associated with the provided `X-Game-Key`.
pub async fn get_currencies(configuration: &configuration::Configuration, x_game_key: &str) -> Result<Vec<crate::models::GetCurrencies200ResponseInner>, Error<GetCurrenciesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies", local_var_configuration.base_path);
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
        let local_var_entity: Option<GetCurrenciesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the current currency balance of the provided wallet address or or the wallet address associated with the provided walletId.
pub async fn get_currency_balance(configuration: &configuration::Configuration, currency_id: &str, address: Option<&str>, wallet_id: Option<&str>) -> Result<f32, Error<GetCurrencyBalanceError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/balances", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

    if let Some(ref local_var_str) = address {
        local_var_req_builder = local_var_req_builder.query(&[("address", &local_var_str.to_string())]);
    }
    if let Some(ref local_var_str) = wallet_id {
        local_var_req_builder = local_var_req_builder.query(&[("walletId", &local_var_str.to_string())]);
    }
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
        let local_var_entity: Option<GetCurrencyBalanceError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Returns the current fee recipient address and fees of the currency for the provided currencyId. Fees are only applicable for gasless transactions performed by default by players.
pub async fn get_currency_fees(configuration: &configuration::Configuration, currency_id: &str) -> Result<crate::models::GetCurrencyFees200Response, Error<GetCurrencyFeesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/fees", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
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
        let local_var_entity: Option<GetCurrencyFeesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Creates (mints) the provided amount of currency to the provided wallet address or wallet address associated with the provided walletId.
pub async fn mint_currency(configuration: &configuration::Configuration, currency_id: &str, x_authorization: &str, x_password: &str, mint_currency_request: crate::models::MintCurrencyRequest) -> Result<crate::models::TransactionModel, Error<MintCurrencyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/mints", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&mint_currency_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<MintCurrencyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Sets the recipient address, basis points, fixed amount and cap amount for a currency's fees.
pub async fn set_currency_fees(configuration: &configuration::Configuration, currency_id: &str, x_authorization: &str, x_password: &str, set_currency_fees_request: crate::models::SetCurrencyFeesRequest) -> Result<crate::models::TransactionModel, Error<SetCurrencyFeesError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/fees", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&set_currency_fees_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<SetCurrencyFeesError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

/// Transfers an amount of currency to the provided wallet address or wallet address associated with the provided walletId. If you want to transfer to multiple wallets with different amounts and optional references in one API request, please see the Batch transfer currency documentation.  An optional reference may be included for the transfer. References are useful for identifying transfers intended to pay for items, trades, services and more.
pub async fn transfer_currency(configuration: &configuration::Configuration, currency_id: &str, x_authorization: &str, x_password: &str, transfer_currency_request: crate::models::TransferCurrencyRequest) -> Result<crate::models::TransactionModel, Error<TransferCurrencyError>> {
    let local_var_configuration = configuration;

    let local_var_client = &local_var_configuration.client;

    let local_var_uri_str = format!("{}/v1/currencies/{currencyId}/transfers", local_var_configuration.base_path, currencyId=crate::apis::urlencode(currency_id));
    let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

    if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
        local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
    }
    local_var_req_builder = local_var_req_builder.header("X-Authorization", x_authorization.to_string());
    local_var_req_builder = local_var_req_builder.header("X-Password", x_password.to_string());
    local_var_req_builder = local_var_req_builder.json(&transfer_currency_request);

    let local_var_req = local_var_req_builder.build()?;
    let local_var_resp = local_var_client.execute(local_var_req).await?;

    let local_var_status = local_var_resp.status();
    let local_var_content = local_var_resp.text().await?;

    if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
        serde_json::from_str(&local_var_content).map_err(Error::from)
    } else {
        let local_var_entity: Option<TransferCurrencyError> = serde_json::from_str(&local_var_content).ok();
        let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
        Err(Error::ResponseError(local_var_error))
    }
}

