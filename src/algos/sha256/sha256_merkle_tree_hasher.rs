#[cfg(any(feature = "sha256_hash", test, bench))]
use ring::digest::{Context, SHA256};

#[cfg(any(feature = "sha256_hash", test, bench))]
use crate::merkle_tree::MerkleTree;
#[cfg(any(feature = "sha256_hash", test, bench))]
use crate::merkle_tree_hasher::LEAF_PREFIX;

#[cfg(any(feature = "sha256_hash", test, bench))]
use crate::merkle_tree_hasher::MerkleTreeHasher;

#[cfg(any(test))]
use crate::utils::hash_values;

/// Create a [MerkleTree] using [Sha256MerkleTreeHasher]. Enabled using the 'sha256_hash' feature.
#[cfg(any(feature = "sha256_hash", test, bench))]
pub type Sha256MerkleTree = MerkleTree<[u8; 32], Sha256MerkleTreeHasher>;

/// Hasher for a Merkle Tree using SHA-256 Hashing. Enabled using the 'sha256_hash' feature.
///
/// This [MerkleTreeHasher] will behave as expected in a multi-threaded environment.
#[cfg(any(feature = "sha256_hash", test, bench))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Sha256MerkleTreeHasher {}

#[cfg(any(feature = "sha256_hash", test, bench))]
impl MerkleTreeHasher<[u8; 32]> for Sha256MerkleTreeHasher {
    fn name(&self) -> String {
        "SHA-256".to_string()
    }
    fn hash_leaf(leaf: &[u8; 32]) -> [u8; 32] {
        // Creating a new Context each call allows for multi-threading later.
        let mut context = Context::new(&SHA256);
        context.update(&LEAF_PREFIX);
        context.update(leaf);
        let digest = context.finish();
        <[u8; 32]>::try_from(digest.as_ref()).unwrap()
    }
    fn hash_non_leaf_node(prefix: &[u8; 1], lhs: &[u8; 32], rhs: &[u8; 32]) -> [u8; 32] {
        // Creating a new Context each call allows for multi-threading later.
        let mut context = Context::new(&SHA256);
        context.update(prefix);
        context.update(lhs);
        context.update(rhs);
        let digest = context.finish();
        <[u8; 32]>::try_from(digest.as_ref()).unwrap()
    }
}

/// Convenience function used for testing to create SHA-256 hashes from strs.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
pub(crate) fn sha256_hash_leaf_values(values: &[&str]) -> Vec<[u8; 32]> {
    hash_values(values, sha256_hash_into_bytes)
}

/// Convenience function used for testing to create SHA-256 hashes.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
pub(crate) fn sha256_hash_into_bytes(value: &[u8]) -> [u8; 32] {
    let mut context = ring::digest::Context::new(&SHA256);
    context.update(value);
    let digest = context.finish();
    <[u8; 32]>::try_from(digest.as_ref()).unwrap()
}
