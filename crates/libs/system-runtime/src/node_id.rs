//! String access and construction for generated health node and value identifiers.

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

impl crate::value::ValueId {
    /// Borrow the identifier's string representation.
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl From<&str> for crate::value::ValueId {
    fn from(id: &str) -> Self {
        Self(id.into())
    }
}

impl From<String> for crate::value::ValueId {
    fn from(id: String) -> Self {
        Self(id)
    }
}
