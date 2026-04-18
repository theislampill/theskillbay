use sled::{Db, Tree};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct KeyPair {
    pub public_key: String,
    pub private_key: String,
}

pub struct Storage {
    db: Db,
}

impl Storage {
    pub fn new(path: &Path) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn get_keypair(&self) -> Result<KeyPair> {
        let tree = self.db.open_tree("keys")?;
        if let Some(data) = tree.get("keypair")? {
            let kp: KeyPair = bincode::deserialize(&data)?;
            Ok(kp)
        } else {
            // Generate new
            use crate::crypto::*;
            let (pub_key, priv_key) = generate_keypair()?;
            let kp = KeyPair { public_key: pub_key, private_key: priv_key };
            tree.insert("keypair", bincode::serialize(&kp)?)?;
            Ok(kp)
        }
    }

    pub fn save_central_policy(&self, policy: &crate::models::CentralPolicy) -> Result<()> {
        let tree = self.db.open_tree("policies")?;
        tree.insert("central", bincode::serialize(policy)?)?;
        Ok(())
    }

    pub fn load_central_policy(&self) -> Result<Option<crate::models::CentralPolicy>> {
        let tree = self.db.open_tree("policies")?;
        if let Some(data) = tree.get("central")? {
            let policy: crate::models::CentralPolicy = bincode::deserialize(&data)?;
            Ok(Some(policy))
        } else {
            Ok(None)
        }
    }
}