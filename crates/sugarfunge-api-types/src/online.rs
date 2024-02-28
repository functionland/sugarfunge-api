use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct GetAuthoredBlocks {
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GetHeartbeats {
    pub validators: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct HeartbeatTime {
    pub time: Option<u32>,
}
