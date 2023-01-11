use std::sync::Arc;

use sysinfo::System;
use tokio::sync::RwLock;

#[derive(Clone)]
pub struct SystemState {
    pub system: Arc<RwLock<System>>,
}
