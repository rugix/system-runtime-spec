//! String access and construction for generated health node identifiers.

use crate::health::HealthNodeId;

impl HealthNodeId {
    /// Borrow the identifier's string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<String> for HealthNodeId {
    fn from(id: String) -> Self {
        Self(id)
    }
}

impl From<&str> for HealthNodeId {
    fn from(id: &str) -> Self {
        Self(id.into())
    }
}
