use chrono::{DateTime, Utc};
use sha2::{Digest, Sha256};

pub struct Block {
    // Headers
    timestamp: DateTime<Utc>,
    prev: String,
    hash: String,
    // Body
    data: Vec<u8>,
}

impl Block {
    pub fn new(timestamp: &DateTime<Utc>, prev: String, hash: String, data: Vec<u8>) -> Self {
        Self {
            timestamp: timestamp.clone(),
            prev,
            hash,
            data,
        }
    }

    pub fn genesis(timestamp: &DateTime<Utc>) -> Self {
        Self {
            timestamp: timestamp.clone(),
            prev: "gen-prev".to_owned(),
            hash: "gen-hash".to_owned(),
            data: [0u8; 32].to_owned().to_vec(),
        }
    }

    pub fn mine(block: &Block, timestamp: &DateTime<Utc>, data: &Vec<u8>) -> Self {
        Self {
            timestamp: timestamp.clone(),
            prev: block.hash.to_owned(),
            hash: "mined hash".to_owned(),
            data: data.to_owned(),
        }
    }
}

#[cfg(test)]
mod block_tests {
    use super::*;
    use hex;

    #[test]
    fn block_identity_test() {
        let ts = Utc::now();
        let block = Block::new(
            &ts,
            "prevhash".to_owned(),
            "hash".to_owned(),
            [0u8; 32].to_owned().to_vec(),
        );
        assert_eq!(block.timestamp, ts);
        assert_eq!(block.prev, "prevhash".to_owned());
        assert_eq!(block.hash, "hash".to_owned());
        assert_eq!(block.data, [0u8; 32].to_owned().to_vec());
    }

    #[test]
    fn genesis_block_test() {
        let ts = Utc::now();
        let block = Block::genesis(&ts);
        assert_eq!(block.timestamp, ts);
        assert_eq!(block.prev, "gen-prev".to_owned());
        assert_eq!(block.hash, "gen-hash".to_owned());
        assert_eq!(block.data, [0u8; 32].to_owned().to_vec());
    }

    #[test]
    fn mine_block_test() {
        let ts = Utc::now();
        let block = Block::genesis(&ts);
        let data = "mined data".to_owned().into_bytes();
        let mined = Block::mine(&block, &ts, &data);
        assert_eq!(mined.data, data);
        assert_eq!(mined.prev, block.hash);
        assert_eq!(mined.timestamp, ts);
    }

    #[test]
    fn hash_test() {
        assert_eq!(
            hex::encode(Sha256::default().chain("foo").finalize()),
            "2C26B46B68FFC68FF99B453C1D30413413422D706483BFA0F98A5E886266E7AE".to_lowercase()
        )
    }
}