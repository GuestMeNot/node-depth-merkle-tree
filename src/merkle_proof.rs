use std::marker::PhantomData;
use rustc_serialize::hex::ToHex;

use serde::{Deserialize, Serialize};

use crate::{increment_or_wrap_around, is_odd, MerkleTreeHasher};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct MerkleProof<T: Copy + Sized, H: MerkleTreeHasher<T> + Default> {
    pub(crate) leaf_index: usize,
    pub(crate) num_leaves: usize,
    pub(crate) leaf_hash: T,
    pub(crate) result_hashes: Vec<T>,
    pub(crate) sibling_hashes: Vec<T>,
    pub(crate) hash_name: String,
    pub(crate) _dummy: PhantomData<H>,
}

impl<T: AsRef<[u8]> + Copy + PartialEq, H: MerkleTreeHasher<T> + Default> MerkleProof<T, H> {
    pub fn validate_proof(&self, leaf: &T) -> bool {

        let mut hash = <H as MerkleTreeHasher<T>>::hash_leaf(leaf);
        if self.leaf_hash != hash {
            return false;
        }

        let mut num_leaves_for_level = self.num_leaves;
        let mut current_idx = self.leaf_index;
        let wrap_to_value = <H as MerkleTreeHasher<T>>::wrap_to_value();
        let mut interior_node_level_prefix = <H as MerkleTreeHasher<T>>::non_leaf_node_starting_prefix();
        let siblings_wo_leaf = &self.sibling_hashes[..&self.sibling_hashes.len() - 1];

        for idx in 0..siblings_wo_leaf.len() {

            let sibling_hash = siblings_wo_leaf[idx];
            let mut incremented = false;

            if is_odd(current_idx) {
                hash = <H as MerkleTreeHasher<T>>::hash_non_leaf_node(&interior_node_level_prefix, &sibling_hash, &hash);
            } else {
                if sibling_hash == hash {
                    interior_node_level_prefix[0] = increment_or_wrap_around(interior_node_level_prefix[0], wrap_to_value);
                    incremented = true;
                }
                hash = <H as MerkleTreeHasher<T>>::hash_non_leaf_node(&interior_node_level_prefix, &hash, &sibling_hash);
            }

            if !incremented && is_odd(num_leaves_for_level) {
                interior_node_level_prefix[0] = increment_or_wrap_around(interior_node_level_prefix[0], wrap_to_value);
            }

            let result = self.result_hashes.as_slice()[idx];

            if result != hash {
                return false;
            }

            interior_node_level_prefix[0] = increment_or_wrap_around(interior_node_level_prefix[0], wrap_to_value);

            current_idx /= 2;

            if is_odd(num_leaves_for_level) {
                num_leaves_for_level += 1;
            }
            num_leaves_for_level /= 2;

        }

        let root = self.sibling_hashes[self.sibling_hashes.len() - 1];
        hash == root
    }
}

impl<T: AsRef<[u8]> + Copy + PartialEq, H: MerkleTreeHasher<T> + Default> PartialEq for MerkleProof<T, H> {
    fn eq(&self, other: &Self) -> bool {
        self.num_leaves == other.num_leaves &&
            self.leaf_index == other.leaf_index &&
            self.hash_name == other.hash_name &&
            self.sibling_hashes.eq(&other.sibling_hashes)
    }
}
