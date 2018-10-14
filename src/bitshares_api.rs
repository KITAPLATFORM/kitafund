use serde::{Serialize, Serializer};
use serde_derive::{Deserialize, Serialize};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
pub struct IdGenerator {
    next_id: u64,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        Self { next_id: 1 }
    }

    pub fn next(&mut self) -> u64 {
        let id = self.next_id;

        self.next_id += 1;

        id
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct JsonRpcRequest<T> {
    pub method: String,
    pub params: T,
    pub id: u64,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct LoginApiRequest {
    pub api_id: u64,
    pub method: LoginApiRequestMethods,
}

impl Serialize for LoginApiRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.method {
            LoginApiRequestMethods::Login(..) => {
                (self.api_id, "login", &self.method).serialize(serializer)
            },
            LoginApiRequestMethods::Database => {
                (self.api_id, "database", &self.method).serialize(serializer)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct DatabaseApiRequest {
    pub api_id: u64,
    pub method: DatabaseApiRequestMethods,
}

impl Serialize for DatabaseApiRequest {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self.method {
            DatabaseApiRequestMethods::GetAccountBalances(..) => {
                (self.api_id, "get_account_balances", &self.method).serialize(serializer)
            }
            DatabaseApiRequestMethods::GetChainId => {
                (self.api_id, "get_chain_id", &self.method).serialize(serializer)
            },
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum LoginApiRequestMethods {
    Login(String, String),
    Database,
}

impl Serialize for LoginApiRequestMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            LoginApiRequestMethods::Login(user, password) => {
                (user, password).serialize(serializer)
            },
            LoginApiRequestMethods::Database => ().serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum DatabaseApiRequestMethods {
    GetAccountBalances(String, Vec<String>),
    GetChainId,
}

impl Serialize for DatabaseApiRequestMethods {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            DatabaseApiRequestMethods::GetAccountBalances(account_name_or_id, assets) => {
                (account_name_or_id, assets).serialize(serializer)
            },
            DatabaseApiRequestMethods::GetChainId => ().serialize(serializer),
        }
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct JsonRpcResponse<T> {
    pub id: u64,
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub result: T,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct JsonRpcResponseError<T> {
    pub id: u64,
    #[serde(rename = "jsonrpc")]
    pub version: String,
    pub error: JsonRpcError<T>,
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct JsonRpcError<T> {
    code: i64,
    message: String,
    data: T,
}

pub type LoginApiLoginResponsMethod = bool;

pub type LoginApiDatabaseResponseMethod = u64;