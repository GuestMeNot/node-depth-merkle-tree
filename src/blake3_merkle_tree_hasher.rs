#[cfg(any(feature = "blake3_hash", test, bench))]
use crate::merkle_tree_hasher::LEAF_PREFIX;

#[cfg(any(feature = "blake3_hash", test, bench))]
use crate::MerkleTree;

#[cfg(any(feature = "blake3_hash", test, bench))]
use crate::MerkleTreeHasher;

#[cfg(any(test))]
use crate::utils::hash_values;


/// Create a [MerkleTree] using [Blake3MerkleTreeHasher].
///
/// Enabled using the 'blake3_hash' feature.
#[cfg(any(feature = "blake3_hash", test, bench))]
pub type BlakeMerkleTree = MerkleTree<[u8; 32], Blake3MerkleTreeHasher>;


/// Hash using [`blake3::Hasher`](blake3::Hasher). Enabled using the 'blake3_hash' feature.
///
/// Blake3 was chosen as an example vs. Kangaroo12 to allow fast hashing on commodity hardware.
/// For blockchain miners, this levels the playing field for smaller validators and consequently
/// increases the difficultly of achieving for 51% attacks.
///
/// This [MerkleTreeHasher] will behave as expected in a multi-threaded environment.
#[cfg(any(feature = "blake3_hash", test, bench))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Blake3MerkleTreeHasher {}

#[cfg(any(feature = "blake3_hash", test, bench))]
impl MerkleTreeHasher<[u8; 32]> for Blake3MerkleTreeHasher {
    fn name(&self) -> String {
        "Blake3".to_string()
    }
    fn hash_leaf(leaf: &[u8; 32]) -> [u8; 32] {
        // Creating a new Hasher each call allows for multi-threading later.
        let mut hasher = blake3::Hasher::new();
        hasher.update(&LEAF_PREFIX);
        hasher.update(leaf);
        hasher.finalize().as_bytes().to_owned()
    }
    fn hash_non_leaf_node(prefix: &[u8; 1], lhs: &[u8; 32], rhs: &[u8; 32]) -> [u8; 32] {
        // Creating a new Hasher each call allows for multi-threading later.
        let mut hasher = blake3::Hasher::new();
        hasher.update(prefix);
        hasher.update(lhs);
        hasher.update(rhs);
        hasher.finalize().as_bytes().to_owned()
    }
}

/// Convenience function used for testing to create Blake3 hashes from strs.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
pub(crate) fn blake3_hash_leaf_values(values: &[&str]) -> Vec<[u8; 32]> {
    hash_values(values, blake3_hash_into_bytes)
}

/// Convenience function used for testing to create Blake3 hashes.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
fn blake3_hash_into_bytes(value: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(value);
    hasher.finalize().as_bytes().to_owned()
}
