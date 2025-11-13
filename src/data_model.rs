use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SourceTypeTotals {
    pub commits: u32,
    pub issues: u32,
    pub merge_requests: u32,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ContributionsResponse {
    pub by_date: HashMap<String, HashMap<String, u32>>,
    pub by_source_total: HashMap<String, SourceTypeTotals>,
}
