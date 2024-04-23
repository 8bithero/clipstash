use crate::data::DdId;
use derive_more::Constructor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Constructor, Debug, Deserialize, Serialize)]
pub struct ClipId(DdId);

impl ClipId {
    pub fn into_inner(self) -> DdId {
        self.0
    }
}

impl From<DdId> for ClipId {
    fn from(id: DdId) -> Self {
        Self(id)
    }
}

impl Default for ClipId {
    fn default() -> Self {
        Self(DdId::nil())
    }
}
