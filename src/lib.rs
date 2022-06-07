//! Creates a Merkle Tree and Merkle Proof.
//!
//! ## Why another Merkle Tree?
//!
//! 1. Merkle Tree hash calculations are CPU intensive and ideally shouldn't block to other CPU activities.
//! 2. merkle_light is single-threaded.
//! 3. rs_merkle is slower in example benchmarks cases than merkle_lite and this implementation.
//!
//!
//! Crate Features
#![doc = document_features::document_features!()]

#[cfg(any(feature = "blake3_hash", test, bench))]
pub use crate::blake3_merkle_tree_hasher::Blake3MerkleTreeHasher;
pub use crate::merkle_tree_hasher::MerkleTreeHasher;
#[cfg(any(feature = "sha256_hash", test, bench))]
pub use crate::sha256_merkle_tree_hasher::Sha256MerkleTreeHasher;

pub use crate::merkle_proof::MerkleProof;
pub use crate::merkle_tree::MerkleTree;

use crate::utils::{add_1_if_odd, count_tree_nodes, increment_or_wrap_around, is_odd};

mod blake3_merkle_tree_hasher;


mod merkle_proof;
mod merkle_tree;
mod merkle_tree_hasher;
mod sha256_merkle_tree_hasher;
#[cfg(test)]
mod test_merkle_proof;
#[cfg(test)]
mod test_merkle_tree;
mod utils;

/// Create a [MerkleTree] using [Blake3MerkleTreeHasher]. Enabled using the 'blake3_hash' feature.
#[cfg(any(feature = "blake3_hash", test, bench))]
pub type BlakeMerkleTree = MerkleTree<[u8; 32], Blake3MerkleTreeHasher>;

/// Create a [MerkleTree] using [Sha256MerkleTreeHasher]. Enabled using the 'sha256_hash' feature.
#[cfg(any(feature = "sha256_hash", test, bench))]
pub type Sha256MerkleTree = MerkleTree<[u8; 32], Sha256MerkleTreeHasher>;
