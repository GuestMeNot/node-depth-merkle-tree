#[cfg(test)]
mod tests {
    use crate::algos::test_merkle_tree_generic::{
        merkle_root_calculation_test, merkle_size_test, merkle_test_repeatable_values,
        merkle_tree_serde_failed_test, merkle_tree_serde_test,
        thwart_second_image_attack_using_duplicate_odd_node,
        thwart_second_image_attack_using_interior_nodes,
        thwart_second_image_attack_using_root_node,
    };
    use crate::algos::{blake3_hash_leaf_values, Blake3MerkleTreeHasher};

    type ValueType = [u8; 32];
    type TestMerkleTreeHasher = Blake3MerkleTreeHasher;
    const HASH_VALUES_FN: fn(&[&str]) -> Vec<ValueType> = blake3_hash_leaf_values;

    #[test]
    fn blake3_merkle_size_test() {
        merkle_size_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn blake3_merkle_tree_serde_test() {
        merkle_tree_serde_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn blake3_merkle_tree_serde_failed_test() {
        merkle_tree_serde_failed_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    /// Added to check backwards compatibility of Merkle Root value.
    ///
    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn blake3_merkle_root_calculation_test() {
        merkle_root_calculation_test::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
            "edd5d4b7c614215ae12bbe871e2cf2f5f76747d3789f8809dc545c2cf45e4867",
        );
    }

    #[test]
    fn blake3_merkle_test_repeatable_values() {
        merkle_test_repeatable_values::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn blake3_thwart_second_image_attack_using_interior_nodes() {
        thwart_second_image_attack_using_interior_nodes::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn blake3_thwart_second_image_attack_using_root_node() {
        thwart_second_image_attack_using_root_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn blake3_thwart_second_image_attack_using_duplicate_odd_node() {
        thwart_second_image_attack_using_duplicate_odd_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }
}
