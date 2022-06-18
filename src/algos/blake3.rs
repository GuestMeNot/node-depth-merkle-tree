#[cfg(any(feature = "blake3_hash", test, bench))]
pub mod blake3_merkle_tree_hasher;
#[cfg(any(feature = "blake3_hash", test, bench))]
mod test_blake3_merkle_tree;

#[cfg(any(feature = "blake3_hash", test, bench))]
pub use blake3_merkle_tree_hasher::{
    blake3_hash_leaf_values, Blake3MerkleTreeHasher, BlakeMerkleTree,
};
