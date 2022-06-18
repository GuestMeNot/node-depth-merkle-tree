use len_trait::Len;
use rustc_serialize::hex::ToHex;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::fmt::Debug;

use crate::{MerkleTree, MerkleTreeHasher};

const SINGLE_CHAR_VALUES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
const SINGLE_CHAR_VALUES_DUP_LAST_NODE: [&str; 7] = ["a", "b", "c", "d", "e", "f", "f"];

#[cfg(any(test))]
#[doc(hidden)]
pub fn merkle_size_test<T, H>(hash: &dyn Fn(&[&str]) -> Vec<T>)
where
    T: AsRef<[u8]> + Copy + Sized,
    H: Default + MerkleTreeHasher<T>,
{
    let tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    assert_eq!(tree.len(), 12);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn merkle_tree_serde_test<T, H>(hash: &dyn Fn(&[&str]) -> Vec<T>)
where
    T: AsRef<[u8]> + Copy + Debug + DeserializeOwned + PartialEq + Serialize + Sized,
    H: Debug + Default + MerkleTreeHasher<T>,
{
    let tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    let json = serde_json::to_string(&tree).unwrap();
    let tree_de: MerkleTree<T, H> = serde_json::from_str(&json).unwrap();
    assert_eq!(tree, tree_de);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn merkle_tree_serde_failed_test<T, H>(hash: &dyn Fn(&[&str]) -> Vec<T>)
where
    T: AsRef<[u8]> + Copy + Debug + DeserializeOwned + PartialEq + Serialize + Sized,
    H: Clone + Debug + Default + MerkleTreeHasher<T>,
{
    let tree1 = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    let json1 = serde_json::to_string(&tree1).unwrap();
    let tree_de1: MerkleTree<T, H> = serde_json::from_str(&json1).unwrap();

    let tree2 = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES_DUP_LAST_NODE)).unwrap();
    let json2: String = serde_json::to_string(&tree2).unwrap();
    let tree_de2: MerkleTree<T, H> = serde_json::from_str(&json2).unwrap();

    assert_ne!(tree_de1, tree_de2);
}

/// Added to check backwards compatibility of Merkle Root value.
///
/// <b>WARNING</b>: The expected_root value needs to be changed if the code changes!
#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn merkle_root_calculation_test<T, H>(
    hash: &dyn Fn(&[&str]) -> Vec<T>,
    expected_root: &str,
) where
    T: AsRef<[u8]> + Copy + Debug + PartialEq + Sized,
    H: Default + MerkleTreeHasher<T>,
{
    let tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    let root = tree.root();
    let hex_root = root.as_ref().to_hex();
    assert_eq!(expected_root, hex_root);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn merkle_test_repeatable_values<T, H>(hash: &dyn Fn(&[&str]) -> Vec<T>)
where
    T: AsRef<[u8]> + Copy + Debug + PartialEq + Sized,
    H: Debug + Default + MerkleTreeHasher<T>,
{
    let tree1 = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    let tree2 = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    assert_eq!(tree1.root(), tree2.root());
    assert_eq!(tree1, tree2);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn thwart_second_image_attack_using_interior_nodes<T, H>(
    hash: &dyn Fn(&[&str]) -> Vec<T>,
) where
    T: AsRef<[u8]> + Copy + Debug + PartialEq + Sized,
    H: Debug + Default + MerkleTreeHasher<T>,
{
    let valid_tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();

    let mut attack_leaves = Vec::new();
    attack_leaves.push(valid_tree[valid_tree.len() - 3]);
    attack_leaves.push(valid_tree[valid_tree.len() - 2]);
    let attack_tree = MerkleTree::<T, H>::new(&attack_leaves).unwrap();

    assert_ne!(valid_tree.root(), attack_tree.root());
    assert_ne!(valid_tree, attack_tree);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn thwart_second_image_attack_using_root_node<T, H>(hash: &dyn Fn(&[&str]) -> Vec<T>)
where
    T: AsRef<[u8]> + Copy + Debug + PartialEq + Sized,
    H: Debug + Default + MerkleTreeHasher<T>,
{
    let valid_tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();

    let mut attack_leaves = Vec::new();
    attack_leaves.push(valid_tree[valid_tree.len() - 1]);
    let attack_tree: MerkleTree<T, H> = MerkleTree::new(&attack_leaves).unwrap();

    assert_ne!(valid_tree.root(), attack_tree.root());
    assert_ne!(valid_tree, attack_tree);
}

#[cfg(any(test))]
#[doc(hidden)]
pub(crate) fn thwart_second_image_attack_using_duplicate_odd_node<T, H>(
    hash: &dyn Fn(&[&str]) -> Vec<T>,
) where
    T: AsRef<[u8]> + Copy + Debug + PartialEq + Sized,
    H: Debug + Default + MerkleTreeHasher<T>,
{
    let valid_tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES)).unwrap();
    let attack_tree = MerkleTree::<T, H>::new(&hash(&SINGLE_CHAR_VALUES_DUP_LAST_NODE)).unwrap();

    assert_ne!(valid_tree.root(), attack_tree.root());
    assert_ne!(valid_tree, attack_tree);
}
