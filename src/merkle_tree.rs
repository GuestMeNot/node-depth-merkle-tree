use std::marker::PhantomData;
use std::slice::Iter;

use anyhow::Result;
use len_trait::{Empty, Len};
use serde::{Deserialize, Serialize};

#[cfg(any(feature = "parallel_hashing"))]
use rayon::prelude::*;

use crate::{
    add_1_if_odd, count_tree_nodes, increment_or_wrap_around, is_odd, MerkleProof, MerkleTreeHasher,
};

#[cfg(any(test))]
use std::ops::Index;

/// A Merkle Tree implementation which uses levels for non-leaf nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MerkleTree<T: Copy + Sized, H: MerkleTreeHasher<T>> {
    num_leaves: usize,
    pub(crate) tree: Vec<T>,
    hash_name: String,
    _dummy: PhantomData<H>,
}

impl<'a, T: 'a + AsRef<[u8]> + Copy + Send + Sync, H: Default + MerkleTreeHasher<T>>
    MerkleTree<T, H>
{
    /// Builds a MerkleTree from leaves of type T and a [MerkleTreeHasher] of type H.
    pub fn new(leaves: &[T]) -> Result<MerkleTree<T, H>> {
        let itr = leaves.iter();
        <MerkleTree<T, H>>::new_from_itr(itr)
    }

    /// Builds a MerkleTree from a leaf [Iter] of type T and a [MerkleTreeHasher] of type H.
    pub fn new_from_itr(leaves: Iter<T>) -> Result<MerkleTree<T, H>> {
        let num_leaves = leaves.len();

        // Creating the MerkleTree near the end of this function reduces performance.
        // This appears to be due to copying the tree.
        let mut merkle_tree = MerkleTree {
            num_leaves,
            tree: Vec::with_capacity(count_tree_nodes(num_leaves)),
            hash_name: H::default().name(),
            _dummy: Default::default(),
        };

        MerkleTree::<T, H>::add_leaves(&mut merkle_tree, leaves);

        let wrap_to_value = <H as MerkleTreeHasher<T>>::wrap_to_value();

        // Prefixes are added to thwart Merkle Tree Second Preimage Attacks
        let mut interior_node_level_prefix: [u8; 1] =
            <H as MerkleTreeHasher<T>>::non_leaf_node_starting_prefix();

        let mut actual_level_count = num_leaves;
        let mut actual_level_count_idx = actual_level_count - 1;

        // if actual_level_count is odd we will use the last hash twice to get an even number of hashes.
        // This is done at each level of the tree other than the last level.
        let mut logical_level_count = add_1_if_odd(actual_level_count);

        // We need to keep track of the index offset for each level.
        let mut non_leaf_nodes_calculated = 0;

        while actual_level_count > 1 {
            let mut count = 0;

            while count < actual_level_count {
                let lhs_idx = count + non_leaf_nodes_calculated;

                // If this is the last index for a tree level then use the last index for the level.
                let rhs_idx = if count == actual_level_count_idx {
                    lhs_idx
                } else {
                    lhs_idx + 1
                };

                // if the number of leaves is odd, thwart an attacker inserting an extra leaf with the same hash as the last leaf.
                if lhs_idx == rhs_idx {
                    interior_node_level_prefix[0] =
                        increment_or_wrap_around(interior_node_level_prefix[0], wrap_to_value);
                }

                let lhs = &merkle_tree.tree[lhs_idx];
                let rhs = &merkle_tree.tree[rhs_idx];

                let hash_val = <H as MerkleTreeHasher<T>>::hash_non_leaf_node(
                    &interior_node_level_prefix,
                    lhs,
                    rhs,
                );

                // Collecting the hash values in a Vec before adding them the tree slows things considerably.
                // This observation rules out functional programming for now...
                merkle_tree.tree.push(hash_val);

                // advance index to the next pair of nodes to hash.
                count += 2;
            }

            // Merkle Tree Second Preimage attacks are a little harder when hashing with a tree level.
            interior_node_level_prefix[0] =
                increment_or_wrap_around(interior_node_level_prefix[0], 1);

            non_leaf_nodes_calculated += actual_level_count;

            // The next level will have half the number of items.
            // The compiler should automatically optimize this by right shifting bits.
            actual_level_count = logical_level_count / 2;

            // So we don't add one inside the loop
            actual_level_count_idx = actual_level_count - 1;

            // use a even number if odd. For example:
            //   if there are three nodes on the previous level we will have two results: 3/2 + 1 = 2
            logical_level_count = add_1_if_odd(actual_level_count);
        }

        Ok(merkle_tree)
    }

    #[cfg(not(any(feature = "parallel_hashing")))]
    fn add_leaves(merkle_tree: &mut MerkleTree<T, H>, leaves: Iter<T>) {
        for leaf in leaves {
            merkle_tree
                .tree
                .push(<H as MerkleTreeHasher<T>>::hash_leaf(leaf));
        }
    }

    // This is not enabled for testing by default.
    #[cfg(any(feature = "parallel_hashing"))]
    fn add_leaves(merkle_tree: &mut MerkleTree<T, H>, leaves: Iter<T>) {
        leaves
            .map(|leaf| leaf.clone())
            .collect::<Vec<T>>()
            .par_iter()
            .map(|leaf| <H as MerkleTreeHasher<T>>::hash_leaf(leaf))
            .collect_into_vec(&mut merkle_tree.tree);
    }

    pub fn build_proof(&self, leaf_index: usize) -> Result<MerkleProof<T, H>> {
        let mut actual_level_count = self.num_leaves;

        let mut current_level_idx = leaf_index;

        let mut sibling_hashes = Vec::new();
        let mut result_hashes = Vec::new();

        let mut non_leaf_nodes_calculated_idx = 0;
        let mut logical_level_count = add_1_if_odd(actual_level_count);

        let leaf_hash = self.tree[leaf_index];

        while actual_level_count > 1 {
            let sibling_level_idx = if !is_odd(current_level_idx) {
                if current_level_idx >= actual_level_count - 1 {
                    current_level_idx
                } else {
                    current_level_idx + 1
                }
            } else {
                current_level_idx - 1
            };

            let sibling_hash = self.tree[non_leaf_nodes_calculated_idx + sibling_level_idx];
            sibling_hashes.push(sibling_hash);

            current_level_idx /= 2;
            non_leaf_nodes_calculated_idx += actual_level_count;

            result_hashes.push(self.tree[non_leaf_nodes_calculated_idx + current_level_idx]);

            // The next level will have half the number of items.
            // The compiler should automatically optimize this by right shifting bits.
            actual_level_count = logical_level_count / 2;

            // use a even number if odd. For example:
            //   if there are three nodes on the previous level we will have two results: 3/2 + 1 = 2
            logical_level_count = add_1_if_odd(actual_level_count);
        }

        sibling_hashes.push(self.root());

        let num_leaves = self.num_leaves;

        Ok(MerkleProof {
            leaf_index,
            num_leaves,
            leaf_hash,
            sibling_hashes,
            result_hashes,
            hash_name: self.hash_name.clone(),
            _dummy: Default::default(),
        })
    }

    /// Returns the number of leaves used to create this Merkle Tree.
    pub fn num_leaves(&self) -> usize {
        self.num_leaves
    }

    /// Returns the Merkle Tree root.
    pub fn root(&self) -> T {
        self.tree[self.tree.len() - 1]
    }
}

impl<T: Copy, H: Default + MerkleTreeHasher<T>> Empty for MerkleTree<T, H> {
    fn is_empty(&self) -> bool {
        self.tree.is_empty()
    }
}

/// Only implemented in 'test' configuration.
#[cfg(any(test))]
impl<T: Copy + Send + ?Sized + Sync, H: Default + MerkleTreeHasher<T>> Index<usize>
    for MerkleTree<T, H>
{
    type Output = T;
    fn index(&self, index: usize) -> &T {
        &self.tree[index]
    }
}

impl<T: Copy, H: Default + MerkleTreeHasher<T>> Len for MerkleTree<T, H> {
    /// Returns the total number of leaves and nodes in this [MerkleTree]
    fn len(&self) -> usize {
        self.tree.len()
    }
}

impl<T: Copy + PartialEq + Sized, H: Default + MerkleTreeHasher<T>> PartialEq for MerkleTree<T, H> {
    fn eq(&self, other: &Self) -> bool {
        self.num_leaves == other.num_leaves
            && self.hash_name == other.hash_name
            && self.tree.eq(&other.tree)
    }
}
