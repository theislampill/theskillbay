use crate::models::{SignedAnnouncement, PublishRecord, PinRecord, PatchRecord, ReviewRecord, SubmittedPatch};
use crate::dedupe;
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
    pub reviews: Mutex<HashMap<String, Vec<ReviewRecord>>>,
    pub submitted_patches: Mutex<HashMap<String, Vec<SubmittedPatch>>>,
    ann_tree: Tree,
    pub_tree: Tree,
    pin_tree: Tree,
    patch_tree: Tree,
    review_tree: Tree,
    submitted_patch_tree: Tree,
}

impl DiscoveryStore {
    pub fn new(db: &sled::Db) -> Result<Self> {
        let ann_tree = db.open_tree("announcements")?;
        let pub_tree = db.open_tree("publish_records")?;
        let pin_tree = db.open_tree("pins")?;
        let patch_tree = db.open_tree("patches")?;
        let review_tree = db.open_tree("reviews")?;
        let submitted_patch_tree = db.open_tree("submitted_patches")?;
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
        let mut review_map = HashMap::new();
        for result in review_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let reviews: Vec<ReviewRecord> = bincode::deserialize(&value)?;
            review_map.insert(skill_id, reviews);
        }
        let mut submitted_patch_map = HashMap::new();
        for result in submitted_patch_tree.iter() {
            let (key, value) = result?;
            let skill_id = String::from_utf8(key.to_vec())?;
            let patches: Vec<SubmittedPatch> = bincode::deserialize(&value)?;
            submitted_patch_map.insert(skill_id, patches);
        }
        Ok(Self {
            announcements: Mutex::new(ann_map),
            publish_records: Mutex::new(pub_map),
            pins: Mutex::new(pin_map),
            patches: Mutex::new(patch_map),
            reviews: Mutex::new(review_map),
            submitted_patches: Mutex::new(submitted_patch_map),
            ann_tree,
            pub_tree,
            pin_tree,
            patch_tree,
            review_tree,
            submitted_patch_tree,
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

    /// Add a submitted patch
    pub fn add_submitted_patch(&self, patch: SubmittedPatch) -> Result<()> {
        let mut patch_map = self.submitted_patches.lock().unwrap();
        let patches = patch_map.entry(patch.skill_id.clone()).or_insert(vec![]);
        patches.push(patch.clone());
        self.submitted_patch_tree.insert(patch.skill_id.as_bytes(), bincode::serialize(patches)?)?;
        Ok(())
    }

    /// Add a review
    pub fn add_review(&self, review_record: ReviewRecord) -> Result<()> {
        let mut review_map = self.reviews.lock().unwrap();
        let reviews = review_map.entry(review_record.skill_id.clone()).or_insert(vec![]);
        reviews.push(review_record.clone());
        self.review_tree.insert(review_record.skill_id.as_bytes(), bincode::serialize(reviews)?)?;
        // Update reputation in announcement
        let mut ann_map = self.announcements.lock().unwrap();
        if let Some(ann) = ann_map.get_mut(&review_record.skill_id) {
            let avg_rating = reviews.iter().map(|r| r.rating as f64).sum::<f64>() / reviews.len() as f64;
            ann.reputation.score = avg_rating;
            ann.reputation.reviews = reviews.len() as u32;
            self.ann_tree.insert(review_record.skill_id.as_bytes(), bincode::serialize(ann)?)?;
        }
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

    /// Find similar skills to avoid duplicates
    pub fn find_similar(&self, target: &SignedAnnouncement, threshold: f64) -> Vec<(SignedAnnouncement, f64)> {
        let map = self.announcements.lock().unwrap();
        map.values()
            .filter(|ann| ann.skill_id != target.skill_id)
            .map(|ann| {
                let score = dedupe::similarity_score(target, ann);
                (ann.clone(), score)
            })
            .filter(|(_, score)| *score > threshold)
            .collect()
    }

    /// Get publish record
    pub fn get_publish_record(&self, skill_id: &str) -> Option<PublishRecord> {
        let map = self.publish_records.lock().unwrap();
        map.get(skill_id).cloned()
    }
}