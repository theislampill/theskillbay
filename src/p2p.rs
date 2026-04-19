use libp2p::{
    gossipsub,
    kad::{self, record::Key as RecordKey, store::MemoryStore},
    mdns,
    noise,
    swarm::{NetworkBehaviour, SwarmEvent},
    tcp,
    yamux,
    PeerId,
    Swarm,
};
use futures::prelude::*;
use serde_json;
use tokio::sync::mpsc;
use crate::models::{P2PMessage, ReviewRecord, SignedAnnouncement, ReputationUpdate};

/// P2P discovery using libp2p gossipsub, mdns, and kad DHT

#[derive(NetworkBehaviour)]
pub struct SkillBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
    pub kad: kad::Behaviour<MemoryStore>,
}

pub struct P2PDiscovery {
    swarm: Swarm<SkillBehaviour>,
    announcements: std::sync::Mutex<Vec<SignedAnnouncement>>,
    reviews: std::sync::Mutex<Vec<ReviewRecord>>,
    sender: tokio::sync::mpsc::UnboundedSender<P2PMessage>,
}

impl P2PDiscovery {
    pub async fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let id_keys = libp2p::identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(id_keys.public());

        let tcp_transport = tcp::async_io::Transport::default()
            .upgrade(libp2p::core::upgrade::Version::V1)
            .authenticate(noise::Config::new(&id_keys).unwrap())
            .multiplex(yamux::Config::default())
            .boxed();

        let gossipsub_config = gossipsub::Config::default();
        let mut gossipsub = gossipsub::Behaviour::new(
            gossipsub::MessageAuthenticity::Signed(id_keys.clone()),
            gossipsub_config,
        ).unwrap();
        let topic = gossipsub::IdentTopic::new("theskillbay");
        gossipsub.subscribe(&topic).unwrap();

        let mdns = mdns::tokio::Behaviour::new(mdns::Config::default(), peer_id).unwrap();

        let store = MemoryStore::new(peer_id);
        let kad = kad::Behaviour::new(peer_id, store);

        let behaviour = SkillBehaviour { gossipsub, mdns, kad };

        let swarm = Swarm::new(tcp_transport, behaviour, peer_id, libp2p::swarm::Config::default());

        let (sender, _receiver) = tokio::sync::mpsc::unbounded_channel::<P2PMessage>();

        Ok(Self {
            swarm,
            announcements: std::sync::Mutex::new(vec![]),
            reviews: std::sync::Mutex::new(vec![]),
            sender,
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            tokio::select! {
                event = self.swarm.select_next_some() => match event {
                    SwarmEvent::Behaviour(SkillBehaviourEvent::Gossipsub(
                        gossipsub::Event::Message { message, .. },
                    )) => {
                        if let Ok(msg) = serde_json::from_slice::<P2PMessage>(&message.data) {
                            match msg {
                                P2PMessage::Announcement(ann) => {
                                    self.announcements.lock().unwrap().push(ann);
                                }
                                P2PMessage::Review(review) => {
                                    self.reviews.lock().unwrap().push(review);
                                }
                                P2PMessage::ReputationUpdate(update) => {
                                    // Handle reputation update for consensus
                                    // This would integrate with discovery store
                                }
                            }
                        }
                    }
                    SwarmEvent::Behaviour(SkillBehaviourEvent::Mdns(
                        mdns::Event::Discovered(list),
                    )) => {
                        for (peer_id, addr) in list {
                            self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                            self.swarm.behaviour_mut().kad.add_address(&peer_id, addr);
                        }
                    }
                    SwarmEvent::Behaviour(SkillBehaviourEvent::Kad(
                        kad::Event::RoutingUpdated { peer, .. },
                    )) => {
                        self.swarm.behaviour_mut().kad.bootstrap().ok();
                    }
                    _ => {}
                },
                msg = self.sender.recv() => {
                    if let Some(msg) = msg {
                        // Publish to gossipsub
                        let topic = gossipsub::IdentTopic::new("theskillbay");
                        let data = serde_json::to_vec(&msg)?;
                        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;

                        // Put to DHT
                        let key = match &msg {
                            P2PMessage::Announcement(ann) => kad::RecordKey::new(&ann.skill_id),
                            P2PMessage::Review(review) => {
                                kad::RecordKey::new(&format!("review_{}_{}", review.skill_id, review.timestamp))
                            }
                            P2PMessage::ReputationUpdate(update) => {
                                kad::RecordKey::new(&format!(
                                    "reputation_{}_{}",
                                    update.skill_id, update.timestamp
                                ))
                            }
                        };
                        let record = kad::Record {
                            key,
                            value: serde_json::to_vec(&msg)?,
                            publisher: None,
                            expires: None,
                        };
                        self.swarm.behaviour_mut().kad.put_record(record, kad::Quorum::One).ok();
                    }
                }
            }
        }
    }

    pub fn sender(&self) -> tokio::sync::mpsc::UnboundedSender<P2PMessage> {
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