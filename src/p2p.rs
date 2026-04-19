use std::sync::Mutex;
use std::time::Duration;
use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use crate::models::{P2PMessage, ReviewRecord, SignedAnnouncement, ReputationUpdate};

/// Minimal P2P stub for V0. Full libp2p integration is deferred.
pub struct P2PDiscovery {
    announcements: Mutex<Vec<SignedAnnouncement>>,
    reviews: Mutex<Vec<ReviewRecord>>,
    sender: UnboundedSender<P2PMessage>,
}

impl P2PDiscovery {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (sender, _receiver) = unbounded_channel::<P2PMessage>();
        Ok(Self {
            announcements: Mutex::new(vec![]),
            reviews: Mutex::new(vec![]),
            sender,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // Stub: keep the background task alive while the main process handles control flow.
        loop {
            tokio::time::sleep(Duration::from_secs(60)).await;
        }
    }

    pub fn sender(&self) -> UnboundedSender<P2PMessage> {
        self.sender.clone()
    }

    pub async fn advertise(&mut self, announcement: SignedAnnouncement) -> Result<(), Box<dyn std::error::Error>> {
        let msg = P2PMessage::Announcement(announcement);
        let _ = self.sender.send(msg);
        Ok(())
    }

    pub async fn broadcast_review(&mut self, review: ReviewRecord) -> Result<(), Box<dyn std::error::Error>> {
        let msg = P2PMessage::Review(review);
        let _ = self.sender.send(msg);
        Ok(())
    }

    pub async fn broadcast_reputation_update(
        &mut self,
        update: ReputationUpdate,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let msg = P2PMessage::ReputationUpdate(update);
        let _ = self.sender.send(msg);
        Ok(())
    }

    pub fn discover(&self, _query: &str) -> Vec<SignedAnnouncement> {
        self.announcements.lock().unwrap().clone()
    }

    pub fn get_reviews(&self, skill_id: &str) -> Vec<ReviewRecord> {
        self.reviews.lock().unwrap().iter().filter(|r| r.skill_id == skill_id).cloned().collect()
    }
}
