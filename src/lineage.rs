use serde::{Serialize, Deserialize};

use crate::hash::{Hash};

#[derive(Clone, Debug, Serialize, Deserialize, Hash)]
pub struct Lineage {
    source_hashes: Vec<Hash>,
    identity_hash: Hash,
}

impl Lineage {
    pub fn new() -> Self {
        Lineage {
            source_hashes: Vec::new(),
            identity_hash: Hash::new(),
        }
    }

    pub fn from_hashes(source_hashes: Vec<Hash>) -> Self {
        let identity_hash = derive_identity(source_hashes.clone());

        Lineage {
            source_hashes,
            identity_hash,
        }
    }

    pub fn with_hash(&self, hash: Hash) -> Self {
        let mut source_hashes: Vec<Hash> = self.source_hashes.clone();
        source_hashes.push(hash);

        let identity_hash = derive_identity(source_hashes.clone());

        Lineage {
            source_hashes,
            identity_hash,
        }
    }

    pub fn to_string(&self) -> String {
        self.identity_hash.to_string().clone().unwrap()
    }
}

impl PartialEq for Lineage {
    fn eq(&self, other: &Self) -> bool {
        self.identity_hash == other.identity_hash
    }
}

impl Eq for Lineage {}

fn derive_identity(source_hashes: Vec<Hash>) -> Hash {

    let mut hashes = source_hashes.clone();

    // We must ensure hashes are finalized
    for hash in hashes.iter_mut() {
        if hash.is_unfinalized() {
            hash.finalize();
        }
    }

    let mut identity_hash = Hash::from_items(hashes);
    identity_hash.finalize();

    identity_hash
}
