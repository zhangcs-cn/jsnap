pub struct Snapshot {
    id_size: u32,
    version: String,
    timestamp: u64,
}

impl Snapshot {
    pub fn new(id_size: u32, version: String, timestamp: u64) -> Snapshot {
        Snapshot { id_size, version, timestamp }
    }
    pub fn get_id_size(&self) -> u32 {
        self.id_size
    }
    pub fn get_version(&self) -> &String {
        &self.version
    }
    pub fn get_timestamp(&self) -> u64 {
        self.timestamp
    }
}