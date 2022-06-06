#[cfg(test)]
mod tests {

    use len_trait::Len;
    use rustc_serialize::hex::ToHex;

    use crate::{Blake3MerkleTreeHasher, BlakeMerkleTree, MerkleTree, MerkleTreeHasher, Sha256MerkleTree};
    use crate::blake3_merkle_tree_hasher::blake3_hash_leaf_values;
    use crate::merkle_tree_hasher::LEAF_PREFIX;
    use crate::sha256_merkle_tree_hasher::sha256_hash_leaf_values;
    use crate::utils::hash_values;

    const SINGLE_CHAR_VALUES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];
    const SINGLE_CHAR_VALUES_DUP_LAST_NODE: [&str; 7] = ["a", "b", "c", "d", "e", "f", "f"];

    #[test]
    fn blake3_merkle_size_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        assert_eq!(tree.len(), 12);
    }


    #[test]
    fn blake3_merkle_tree_serde_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let json = serde_json::to_string(&tree).unwrap();
        let tree_de: MerkleTree<[u8; 32], Blake3MerkleTreeHasher> = serde_json::from_str(&json).unwrap();
        assert_eq!(tree, tree_de);
    }

    #[test]
    fn blake3_merkle_tree_serde_failed_test() {
        let tree1 = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let json1 = serde_json::to_string(&tree1).unwrap();
        let tree_de1: MerkleTree<[u8; 32], Blake3MerkleTreeHasher> = serde_json::from_str(&json1).unwrap();

        let tree2 = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES_DUP_LAST_NODE)).unwrap();
        let json2 = serde_json::to_string(&tree2).unwrap();
        let tree_de2: MerkleTree<[u8; 32], Blake3MerkleTreeHasher> = serde_json::from_str(&json2).unwrap();

        assert_ne!(tree_de1, tree_de2);
    }

    /// Added to check backwards compatibility of Merkle Root value.
    ///
    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn blake3_merkle_root_calculation_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let root = tree.root().to_hex();
        assert_eq!(root, "edd5d4b7c614215ae12bbe871e2cf2f5f76747d3789f8809dc545c2cf45e4867");
    }

    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn md5_16_bit_hash_test() {
        let tree = Md5MerkleTree::build(&md5_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let root = tree.root().to_hex();
        assert_eq!("c61de42cef4648a991700d6385ffd7b1", root);
    }

    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn sha256_hash_test() {
        let tree = Sha256MerkleTree::build(&sha256_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let root = tree.root().to_hex();
        assert_eq!("52a195c0c851e24cf9e99cf7f61552bd24d25a1fb784a5657b3b9d4634aec633", root);
    }

    #[test]
    fn blake3_merkle_test_repeatable_values() {
        let tree1 = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let tree2 = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        assert_eq!(tree1, tree2);
    }

    #[test]
    fn thwart_second_image_attack_using_interior_nodes() {
        let valid_tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();

        let mut attack_leaves = Vec::new();
        attack_leaves.push(valid_tree[valid_tree.len() - 3]);
        attack_leaves.push(valid_tree[valid_tree.len() - 2]);
        let attack_tree = BlakeMerkleTree::build(&attack_leaves).unwrap();

        assert_ne!(valid_tree.root(), attack_tree.root());
    }

    #[test]
    fn thwart_second_image_attack_using_root_node() {
        let valid_tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();

        let mut attack_leaves = Vec::new();
        attack_leaves.push(valid_tree[valid_tree.len() - 1]);
        let attack_tree = BlakeMerkleTree::build(&attack_leaves).unwrap();

        assert_ne!(valid_tree.root(), attack_tree.root());
    }

    #[test]
    fn thwart_second_image_attack_using_duplicate_odd_node() {
        let valid_tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        let attack_tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES_DUP_LAST_NODE)).unwrap();

        assert_ne!(valid_tree.root(), attack_tree.root());
    }

    pub type Md5MerkleTree = MerkleTree<[u8; 16], Md5HashManager>;

    #[derive(Debug, Default, Copy, Clone)]
    pub struct Md5HashManager {}

    impl MerkleTreeHasher<[u8; 16]> for Md5HashManager {
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

    fn md5_hash_leaf_values(values: &[&str]) -> Vec<[u8; 16]> {
        hash_values(values, md5_hash_into_bytes)
    }

    #[inline]
    fn md5_hash_into_bytes(value: &[u8]) -> [u8; 16] {
        let mut context = md5::Context::new();
        context.consume(value);
        let digest = context.compute();
        <[u8; 16]>::try_from(digest.as_ref()).unwrap()
    }
}