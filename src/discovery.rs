use crate::models::{SignedAnnouncement, PublishRecord, PinRecord, PatchRecord};
use std::collections::HashMap;
use std::sync::Mutex;
use sled::Tree;
use anyhow::Result;
use bincode;

// STUB: Future replacement with libp2p DHT for decentralized discovery
// For now, using sled-based local persistence

/// Simple discovery store with persistence
pub struct DiscoveryStore {
    pub announcements: Mutex<HashMap<String, SignedAnnouncement>>,
    pub publish_records: Mutex<HashMap<String, PublishRecord>>,
    pub pins: Mutex<HashMap<String, PinRecord>>,
    pub patches: Mutex<HashMap<String, Vec<PatchRecord>>>,
    ann_tree: Tree,
    pub_tree: Tree,
    pin_tree: Tree,
    patch_tree: Tree,
}

impl DiscoveryStore {
    pub fn new(db: &sled::Db) -> Result<Self> {
        let ann_tree = db.open_tree("announcements")?;
        let pub_tree = db.open_tree("publish_records")?;
        let pin_tree = db.open_tree("pins")?;
        let patch_tree = db.open_tree("patches")?;
        let mut ann_map = HashMap::new();
        for result in ann_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let ann: SignedAnnouncement = bincode::deserialize(&value)?;
            ann_map.insert(skill_id, ann);
        }
        let mut pub_map = HashMap::new();
        for result in pub_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let rec: PublishRecord = bincode::deserialize(&value)?;
            pub_map.insert(skill_id, rec);
        }
        let mut pin_map = HashMap::new();
        for result in pin_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let pin: PinRecord = bincode::deserialize(&value)?;
            pin_map.insert(skill_id, pin);
        }
        let mut patch_map = HashMap::new();
        for result in patch_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let patches: Vec<PatchRecord> = bincode::deserialize(&value)?;
            patch_map.insert(skill_id, patches);
        }
        Ok(Self {
            announcements: Mutex::new(ann_map),
            publish_records: Mutex::new(pub_map),
            pins: Mutex::new(pin_map),
            patches: Mutex::new(patch_map),
            ann_tree,
            pub_tree,
            pin_tree,
            patch_tree,
        })
    }

    /// Advertise a skill
    pub fn advertise(&self, announcement: SignedAnnouncement, publish_record: PublishRecord) -> Result<()> {
        let mut ann_map = self.announcements.lock().unwrap();
        ann_map.insert(announcement.skill_id.clone(), announcement.clone());
        self.ann_tree.insert(announcement.skill_id.as_bytes(), bincode::serialize(&announcement)?)?;
        
        let mut pub_map = self.publish_records.lock().unwrap();
        pub_map.insert(publish_record.skill_id.clone(), publish_record.clone());
        self.pub_tree.insert(publish_record.skill_id.as_bytes(), bincode::serialize(&publish_record)?)?;
        Ok(())
    }

    /// Pin a skill
    pub fn pin(&self, pin_record: PinRecord) -> Result<()> {
        let mut pin_map = self.pins.lock().unwrap();
        pin_map.insert(pin_record.skill_id.clone(), pin_record.clone());
        self.pin_tree.insert(pin_record.skill_id.as_bytes(), bincode::serialize(&pin_record)?)?;
        Ok(())
    }

    /// Add a patch
    pub fn add_patch(&self, patch_record: PatchRecord) -> Result<()> {
        let mut patch_map = self.patches.lock().unwrap();
        let patches = patch_map.entry(patch_record.skill_id.clone()).or_insert(vec![]);
        patches.push(patch_record.clone());
        self.patch_tree.insert(patch_record.skill_id.as_bytes(), bincode::serialize(patches)?)?;
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

    /// Get publish record
    pub fn get_publish_record(&self, skill_id: &str) -> Option<PublishRecord> {
        let map = self.publish_records.lock().unwrap();
        map.get(skill_id).cloned()
    }
}