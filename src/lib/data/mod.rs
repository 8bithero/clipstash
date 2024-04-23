use derive_more::{Display, From};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use std::str::FromStr;

#[derive(Clone, Debug, From, Display, Deserialize, Serialize)]
pub struct DdId(Uuid);

impl DdId {
    pub fn new() -> DdId {
        Uuid::new_v4().into()
    }

    pub fn nil() -> DdId {
        Self(Uuid::nil())
    }
}

impl Default for DdId {
    fn default() -> Self {
        Self::new()
    }
}

impl FromStr for DdId {
    type Err = uuid::Error;
    fn from_str(id: &str) -> Result<Self, Self::Err> {
        Ok(DdId(Uuid::parse_str(id)?))
    }
}
