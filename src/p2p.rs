use libp2p::{
    gossipsub, mdns, noise, swarm::NetworkBehaviour, swarm::SwarmEvent, tcp, yamux, PeerId, Swarm,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use futures::prelude::*;
use tokio::sync::mpsc;
use crate::models::SignedAnnouncement;

/// P2P discovery using libp2p gossipsub and mdns

#[derive(NetworkBehaviour)]
pub struct SkillBehaviour {
    pub gossipsub: gossipsub::Behaviour,
    pub mdns: mdns::tokio::Behaviour,
}

pub struct P2PDiscovery {
    swarm: Swarm<SkillBehaviour>,
    announcements: std::sync::Mutex<Vec<SignedAnnouncement>>,
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

        let behaviour = SkillBehaviour { gossipsub, mdns };

        let swarm = Swarm::new(tcp_transport, behaviour, peer_id, libp2p::swarm::Config::default());

        Ok(Self {
            swarm,
            announcements: std::sync::Mutex::new(vec![]),
        })
    }

    pub async fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        loop {
            match self.swarm.select_next_some().await {
                SwarmEvent::Behaviour(SkillBehaviourEvent::Gossipsub(gossipsub::Event::Message { message, .. })) => {
                    if let Ok(ann) = serde_json::from_slice::<SignedAnnouncement>(&message.data) {
                        self.announcements.lock().unwrap().push(ann);
                    }
                }
                SwarmEvent::Behaviour(SkillBehaviourEvent::Mdns(mdns::Event::Discovered(list))) => {
                    for (peer_id, _addr) in list {
                        self.swarm.behaviour_mut().gossipsub.add_explicit_peer(&peer_id);
                    }
                }
                _ => {}
            }
        }
    }

    pub async fn advertise(&mut self, announcement: SignedAnnouncement) -> Result<(), Box<dyn std::error::Error>> {
        let topic = gossipsub::IdentTopic::new("theskillbay");
        let data = serde_json::to_vec(&announcement)?;
        self.swarm.behaviour_mut().gossipsub.publish(topic, data)?;
        Ok(())
    }

    pub fn discover(&self, _query: &str) -> Vec<SignedAnnouncement> {
        // Simple: return all known announcements
        // TODO: Implement proper querying
        self.announcements.lock().unwrap().clone()
    }
}