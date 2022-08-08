use serde::{Deserialize, Serialize};

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct GetCpuResponse {
    pub global: GlobalCpuInfo,
    pub cpus: Vec<CpuInfo>,
}

#[derive(Clone, PartialEq, Debug, Serialize, Deserialize)]
pub struct CpuInfo {
    pub name: String,
    #[serde(flatten)]
    pub inner: GlobalCpuInfo,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct GlobalCpuInfo {
    pub usage: f32,
    pub frequency: u64,
}
