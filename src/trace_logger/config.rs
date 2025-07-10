use serde::{Deserialize, Serialize};

pub const B: usize = 1;
pub const KB: usize = 1024 * B;
pub const MB: usize = 1024 * KB;
pub const GB: usize = 1024 * MB;

// log to the file path
const FILE: Option<String> = None;
fn file() -> Option<String> {
    FILE
}

// log will rotate to the given backup path
const BACKUP: Option<String> = None;
fn backup() -> Option<String> {
    BACKUP
}

// max log size before rotate in bytes
const MAX_SIZE: u64 = GB as u64;
fn max_size() -> u64 {
    MAX_SIZE
}

// logger queue depth
const QUEUE_DEPTH: usize = 4096;
fn queue_depth() -> usize {
    QUEUE_DEPTH
}

// single message buffer size in bytes
const SINGLE_MESSAGE_SIZE: usize = KB;
fn single_message_size() -> usize {
    SINGLE_MESSAGE_SIZE
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TraceLog {
    #[serde(default = "backup")]
    backup: Option<String>,
    #[serde(default = "file")]
    file: Option<String>,
    #[serde(default = "max_size")]
    max_size: u64,
    #[serde(default = "queue_depth")]
    queue_depth: usize,
    #[serde(default = "single_message_size")]
    single_message_size: usize,
}

impl TraceLog {
    pub fn file(&self) -> Option<String> {
        self.file.clone()
    }

    pub fn backup(&self) -> Option<String> {
        match &self.backup {
            Some(path) => Some(path.clone()),
            None => self.file.as_ref().map(|path| format!("{path}.old")),
        }
    }

    pub fn max_size(&self) -> u64 {
        self.max_size
    }

    pub fn queue_depth(&self) -> usize {
        self.queue_depth
    }

    pub fn single_message_size(&self) -> usize {
        self.single_message_size
    }
}

impl Default for TraceLog {
    fn default() -> Self {
        Self {
            file: file(),
            backup: backup(),
            max_size: max_size(),
            queue_depth: queue_depth(),
            single_message_size: single_message_size(),
        }
    }
}

pub trait TraceConfig {
    fn trace(&self) -> &TraceLog;
}
