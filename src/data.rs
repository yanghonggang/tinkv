//! Data file implementation.
use crate::error::Result;
use crate::util::{checksum, current_timestamp};
use serde::{Deserialize, Serialize};
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::path::{Path, PathBuf};

/// Data entry definition.
#[derive(Serialize, Deserialize, Debug)]
pub struct Entry {
    key: Vec<u8>,
    value: Vec<u8>,
    // timestamp in seconds
    timestamp: u32,
    // crc32 checksum
    checksum: u32,
}

impl Entry {
    fn new(key: Vec<u8>, value: Vec<u8>) -> Self {
        let mut ent = Entry {
            key: key,
            value: value,
            timestamp: current_timestamp(),
            checksum: 0,
        };
        ent.checksum = ent.fresh_checksum();
        ent
    }

    fn fresh_checksum(&self) -> u32 {
        // TODO: optimize it to avoid cloning.
        checksum(&[self.key.clone(), self.value.clone()].concat())
    }

    fn is_valid(&self) -> bool {
        self.checksum == self.fresh_checksum()
    }
}

#[derive(Debug)]
pub(crate) struct File {
    id: u64,
    path: PathBuf,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_entry() {
        let ent = Entry::new(b"key".to_vec(), b"value".to_vec());
        assert_eq!(ent.timestamp <= current_timestamp(), true);
        assert_eq!(ent.checksum, 3327521766);
    }

    #[test]
    fn test_checksum_valid() {
        let ent = Entry::new(b"key".to_vec(), b"value".to_vec());
        assert_eq!(ent.is_valid(), true);
    }

    #[test]
    fn test_checksum_invalid() {
        let mut ent = Entry::new(b"key".to_vec(), b"value".to_vec());
        ent.value = b"value_changed".to_vec();
        assert_eq!(ent.is_valid(), false);
    }
}