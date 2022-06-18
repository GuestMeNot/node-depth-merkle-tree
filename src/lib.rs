#![feature(in_band_lifetimes)]
//! Creates a Merkle Tree and Merkle Proof.
//!
//! ## Why another Merkle Tree?
//!
//! 1. Merkle Tree hash calculations are CPU intensive and ideally shouldn't block to other CPU activities.
//! 2. merkle_light is single-threaded.
//! 3. rs_merkle is slower in example benchmarks cases than merkle_lite and this implementation.
//!
//!
//! ## Crate Features
#![doc = document_features::document_features!()]

#[cfg(any(feature = "keccak256_hash", test, bench))]
pub use crate::algos::Keccak256MerkleTree;
#[cfg(any(feature = "md5_hash", test, bench))]
pub use crate::algos::Md5MerkleTree;
#[cfg(any(feature = "sha256_hash", test, bench))]
pub use crate::algos::Sha256MerkleTree;
#[cfg(any(feature = "blake3_hash", test, bench))]
pub use crate::algos::{blake3_hash_leaf_values, BlakeMerkleTree};

pub use merkle_proof::MerkleProof;
pub use merkle_tree::MerkleTree;
pub use merkle_tree_hasher::MerkleTreeHasher;

use crate::utils::{add_1_if_odd, count_tree_nodes, increment_or_wrap_around, is_odd};

mod algos;
mod merkle_proof;
mod merkle_tree;
mod merkle_tree_hasher;
#[cfg(test)]
mod test_merkle_proof;
mod utils;
