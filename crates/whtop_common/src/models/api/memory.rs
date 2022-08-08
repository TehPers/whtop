use serde::{Serialize, Deserialize};

/// Response from getting the memory usage metrics.
#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct GetMemoryResponse {
    /// Total memory in kilobytes.
    pub total: u64,
    /// Used memory in kilobytes.
    pub used: u64,
    /// Free (unallocated) memory in kilobytes.
    pub free: u64,
    /// Available (reusable) memory in kilobytes.
    pub available: u64,
}
