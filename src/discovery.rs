use crate::models::SignedAnnouncement;
use std::collections::HashMap;
use std::sync::Mutex;
use sled::Tree;
use anyhow::Result;
use bincode;

/// Simple discovery store with persistence
pub struct DiscoveryStore {
    pub announcements: Mutex<HashMap<String, SignedAnnouncement>>,
    tree: Tree,
}

impl DiscoveryStore {
    pub fn new(db: &sled::Db) -> Result<Self> {
        let tree = db.open_tree("announcements")?;
        let mut map = HashMap::new();
        for result in tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let ann: SignedAnnouncement = bincode::deserialize(&value)?;
            map.insert(skill_id, ann);
        }
        Ok(Self {
            announcements: Mutex::new(map),
            tree,
        })
    }

    /// Advertise a skill
    pub fn advertise(&self, announcement: SignedAnnouncement) -> Result<()> {
        let mut map = self.announcements.lock().unwrap();
        map.insert(announcement.skill_id.clone(), announcement.clone());
        self.tree.insert(announcement.skill_id.as_bytes(), bincode::serialize(&announcement)?)?;
        Ok(())
    }

    /// Discover skills by query (simple match)
    pub fn discover(&self, query: &str) -> Vec<SignedAnnouncement> {
        let map = self.announcements.lock().unwrap();
        map.values()
            .filter(|ann| ann.skill_id.contains(query) || ann.metadata.get("name").unwrap_or(&"".to_string()).contains(query))
            .cloned()
            .collect()
    }
}