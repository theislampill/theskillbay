use crate::models::{SignedAnnouncement, PublishRecord, PinRecord, PatchRecord, ReviewRecord, SubmittedPatch, P2PMessage, CredibilityRecord, ReputationUpdate};
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
    pub credibility: Mutex<HashMap<String, CredibilityRecord>>,
    ann_tree: Tree,
    pub_tree: Tree,
    pin_tree: Tree,
    patch_tree: Tree,
    review_tree: Tree,
    submitted_patch_tree: Tree,
    cred_tree: Tree,
    p2p_sender: Option<tokio::sync::mpsc::UnboundedSender<P2PMessage>>,
}

impl DiscoveryStore {
    pub fn new(db: &sled::Db) -> Result<Self> {
        Self::new_with_p2p(db, None)
    }

    pub fn new_with_p2p(db: &sled::Db, p2p_sender: Option<tokio::sync::mpsc::UnboundedSender<P2PMessage>>) -> Result<Self> {
        let ann_tree = db.open_tree("announcements")?;
        let pub_tree = db.open_tree("publish_records")?;
        let pin_tree = db.open_tree("pins")?;
        let patch_tree = db.open_tree("patches")?;
        let review_tree = db.open_tree("reviews")?;
        let submitted_patch_tree = db.open_tree("submitted_patches")?;
        let cred_tree = db.open_tree("credibility")?;
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
        let mut cred_map = HashMap::new();
        for result in cred_tree.iter() {
            let (key, value) = result?;
            let reviewer_id = String::from_utf8(key.to_vec())?;
            let cred: CredibilityRecord = bincode::deserialize(&value)?;
            cred_map.insert(reviewer_id, cred);
        }
        Ok(Self {
            announcements: Mutex::new(ann_map),
            publish_records: Mutex::new(pub_map),
            pins: Mutex::new(pin_map),
            patches: Mutex::new(patch_map),
            reviews: Mutex::new(review_map),
            submitted_patches: Mutex::new(submitted_patch_map),
            credibility: Mutex::new(cred_map),
            ann_tree,
            pub_tree,
            pin_tree,
            patch_tree,
            review_tree,
            submitted_patch_tree,
            cred_tree,
            p2p_sender,
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

        // Broadcast to P2P if available
        if let Some(sender) = &self.p2p_sender {
            let _ = sender.send(P2PMessage::Announcement(announcement));
        }

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
        // Update reputation in announcement with network aggregation
        let mut ann_map = self.announcements.lock().unwrap();
        if let Some(ann) = ann_map.get_mut(&review_record.skill_id) {
            if let Some(network_score) = self.aggregate_network_reputation(&review_record.skill_id) {
                // Use consensus validation
                let local_avg = reviews.iter().map(|r| r.rating as f64).sum::<f64>() / reviews.len() as f64;
                if self.consensus_validate_reputation(&review_record.skill_id, local_avg, 0.5) {
                    ann.reputation.score = network_score;
                } else {
                    // Fallback to local if consensus fails
                    ann.reputation.score = local_avg;
                }
                ann.reputation.reviews = reviews.len() as u32;
                self.ann_tree.insert(review_record.skill_id.as_bytes(), bincode::serialize(ann)?)?;
            }
        }

        // Update credibility
        let mut cred_map = self.credibility.lock().unwrap();
        let cred = cred_map.entry(review_record.reviewer_id.clone()).or_insert(CredibilityRecord {
            reviewer_id: review_record.reviewer_id.clone(),
            score: 0.5, // Initial
            total_reviews: 0,
        });
        cred.total_reviews += 1;
        // Simple: credibility increases with more reviews, up to 1.0
        cred.score = (cred.total_reviews as f64 / 10.0).min(1.0);
        self.cred_tree.insert(review_record.reviewer_id.as_bytes(), bincode::serialize(cred)?)?;

        // Broadcast to P2P if available
        if let Some(sender) = &self.p2p_sender {
            let _ = sender.send(P2PMessage::Review(review_record));
            // Also broadcast reputation update for consensus
            if let Some(network_score) = self.aggregate_network_reputation(&review_record.skill_id) {
                let update = ReputationUpdate {
                    skill_id: review_record.skill_id.clone(),
                    score: network_score,
                    reviews: reviews.len() as u32,
                    timestamp: std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs(),
                };
                let _ = sender.send(P2PMessage::ReputationUpdate(update));
            }
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

    /// Aggregate reputation from network reviews with credibility weighting
    pub fn aggregate_network_reputation(&self, skill_id: &str) -> Option<f64> {
        let reviews = self.reviews.lock().unwrap();
        let skill_reviews: Vec<&ReviewRecord> = reviews.values()
            .flat_map(|v| v.iter())
            .filter(|r| r.skill_id == skill_id)
            .collect();

        if skill_reviews.is_empty() {
            return None;
        }

        let credibility_map = self.credibility.lock().unwrap();
        let total_weight: f64 = skill_reviews.iter()
            .map(|r| credibility_map.get(&r.reviewer_id).map(|c| c.score).unwrap_or(0.5))
            .sum();

        if total_weight == 0.0 {
            return Some(skill_reviews.iter().map(|r| r.rating as f64).sum::<f64>() / skill_reviews.len() as f64);
        }

        let weighted_sum: f64 = skill_reviews.iter()
            .map(|r| {
                let cred = credibility_map.get(&r.reviewer_id).map(|c| c.score).unwrap_or(0.5);
                r.rating as f64 * cred
            })
            .sum();

        Some(weighted_sum / total_weight)
    }

    /// Consensus validation: check if network reputation agrees within threshold
    pub fn consensus_validate_reputation(&self, skill_id: &str, local_score: f64, threshold: f64) -> bool {
        if let Some(network_score) = self.aggregate_network_reputation(skill_id) {
            (network_score - local_score).abs() <= threshold
        } else {
            true // No network data, accept local
        }
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