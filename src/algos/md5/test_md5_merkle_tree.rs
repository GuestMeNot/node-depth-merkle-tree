#[cfg(test)]
mod tests {

    use crate::algos::md5::md5_merkle_tree_hasher::{md5_hash_leaf_values, Md5MerkleTreeHasher};
    use crate::algos::test_merkle_tree_generic::{
        merkle_root_calculation_test, merkle_size_test, merkle_test_repeatable_values,
        merkle_tree_serde_failed_test, merkle_tree_serde_test,
        thwart_second_image_attack_using_duplicate_odd_node,
        thwart_second_image_attack_using_interior_nodes,
        thwart_second_image_attack_using_root_node,
    };

    type ValueType = [u8; 16];
    type TestMerkleTreeHasher = Md5MerkleTreeHasher;
    const HASH_VALUES_FN: fn(&[&str]) -> Vec<ValueType> = md5_hash_leaf_values;

    #[test]
    fn md5_merkle_size_test() {
        merkle_size_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn md5_merkle_tree_serde_test() {
        merkle_tree_serde_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn md5_merkle_tree_serde_failed_test() {
        merkle_tree_serde_failed_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    /// Added to check backwards compatibility of Merkle Root value.
    ///
    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn md5_merkle_root_calculation_test() {
        merkle_root_calculation_test::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
            "c61de42cef4648a991700d6385ffd7b1",
        );
    }

    #[test]
    fn md5_merkle_test_repeatable_values() {
        merkle_test_repeatable_values::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn md5_thwart_second_image_attack_using_interior_nodes() {
        thwart_second_image_attack_using_interior_nodes::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn md5_thwart_second_image_attack_using_root_node() {
        thwart_second_image_attack_using_root_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn md5_thwart_second_image_attack_using_duplicate_odd_node() {
        thwart_second_image_attack_using_duplicate_odd_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }
}
