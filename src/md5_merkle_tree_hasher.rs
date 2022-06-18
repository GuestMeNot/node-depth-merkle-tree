#[cfg(any(feature = "md5_hash", test, bench))]
use crate::merkle_tree_hasher::LEAF_PREFIX;

#[cfg(any(feature = "md5_hash", test, bench))]
use crate::MerkleTree;

#[cfg(any(feature = "md5_hash", test, bench))]
use crate::MerkleTreeHasher;

#[cfg(any(test))]
use crate::utils::hash_values;

/// Create a [MerkleTree] using [MD5MerkleTreeHasher].
///
/// Enabled using the 'md5_hash' feature.
#[cfg(any(feature = "md5_hash", test, bench))]
pub type Md5MerkleTree = MerkleTree<[u8; 16], Md5MerkleTreeHasher>;

/// Hash using MD5. Enabled using the 'md5_hash' feature.
///
/// This [MerkleTreeHasher] will behave as expected in a multi-threaded environment.
#[cfg(any(feature = "md5_hash", test, bench))]
#[derive(Debug, Default, Copy, Clone)]
pub struct Md5MerkleTreeHasher {}

#[cfg(any(feature = "md5_hash", test, bench))]
impl MerkleTreeHasher<[u8; 16]> for Md5MerkleTreeHasher {
    fn name(&self) -> String {
        "MD5".to_string()
    }
    fn hash_leaf(leaf: &[u8; 16]) -> [u8; 16] {
        let mut context = md5::Context::new();
        context.consume(&LEAF_PREFIX);
        context.consume(leaf);
        let digest = context.compute();
        <[u8; 16]>::try_from(digest.as_ref()).unwrap()
    }
    fn hash_non_leaf_node(prefix: &[u8; 1], lhs: &[u8; 16], rhs: &[u8; 16]) -> [u8; 16] {
        let mut context = md5::Context::new();
        context.consume(prefix);
        context.consume(lhs);
        context.consume(rhs);
        let digest = context.compute();
        <[u8; 16]>::try_from(digest.as_ref()).unwrap()
    }
}

/// Convenience function used for testing to create MD5 hashes from strs.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
pub(crate) fn md5_hash_leaf_values(values: &[&str]) -> Vec<[u8; 16]> {
    hash_values(values, md5_hash_into_bytes)
}

/// Convenience function used for testing to create MD5 hashes.
#[cfg(any(test))]
#[doc(hidden)]
#[inline(always)]
pub(crate) fn md5_hash_into_bytes(value: &[u8]) -> [u8; 16] {
    let mut context = md5::Context::new();
    context.consume(value);
    let digest = context.compute();
    <[u8; 16]>::try_from(digest.as_ref()).unwrap()
}
