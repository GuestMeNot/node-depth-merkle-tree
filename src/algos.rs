#[cfg(test)]
mod test_merkle_tree_generic;

#[cfg(any(feature = "blake3_hash", test, bench))]
mod blake3;
#[cfg(any(feature = "keccak256_hash", test, bench))]
mod keccak256;
#[cfg(any(feature = "md5_hash", test, bench))]
mod md5;
#[cfg(any(feature = "sha256_hash", test, bench))]
mod sha256;

#[cfg(any(feature = "blake3_hash", test, bench))]
pub use self::blake3::{blake3_hash_leaf_values, Blake3MerkleTreeHasher, BlakeMerkleTree};
#[cfg(any(feature = "keccak256_hash", test, bench))]
pub use self::keccak256::keccak256_merkle_tree_hasher::{
    Keccak256MerkleTree, Keccak256MerkleTreeHasher,
};
#[cfg(any(feature = "md5_hash", test, bench))]
pub use self::md5::md5_merkle_tree_hasher::{Md5MerkleTree, Md5MerkleTreeHasher};
#[cfg(any(feature = "sha256_hash", test, bench))]
pub use self::sha256::sha256_merkle_tree_hasher::{Sha256MerkleTree, Sha256MerkleTreeHasher};
