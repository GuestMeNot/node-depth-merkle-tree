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

#[cfg(any(feature = "blake3_hash", test, bench))]
pub use crate::blake3_merkle_tree_hasher::BlakeMerkleTree;
#[cfg(any(feature = "keccak256_hash", test, bench))]
pub use crate::keccak256_merkle_tree_hasher::Keccak256MerkleTree;
#[cfg(any(feature = "md5_hash", test, bench))]
pub use crate::md5_merkle_tree_hasher::Md5MerkleTree;
pub use crate::merkle_tree_hasher::MerkleTreeHasher;
#[cfg(any(feature = "sha256_hash", test, bench))]
pub use crate::sha256_merkle_tree_hasher::Sha256MerkleTree;

pub use crate::merkle_proof::MerkleProof;
pub use crate::merkle_tree::MerkleTree;

use crate::utils::{add_1_if_odd, count_tree_nodes, increment_or_wrap_around, is_odd};

mod blake3_merkle_tree_hasher;
mod keccak256_merkle_tree_hasher;
mod md5_merkle_tree_hasher;
mod merkle_proof;
mod merkle_tree;
mod merkle_tree_hasher;
mod sha256_merkle_tree_hasher;
#[cfg(test)]
mod test_merkle_proof;
#[cfg(test)]
mod test_blake3_merkle_tree;
#[cfg(test)]
mod test_keccak256_merkle_tree;
#[cfg(test)]
mod test_md5_merkle_tree;
#[cfg(test)]
mod test_merkle_tree_generic;
#[cfg(test)]
mod test_sha256_merkle_tree;
mod utils;


