#[cfg(test)]
mod tests {

    use crate::keccak256_merkle_tree_hasher::{
        keccak256_hash_leaf_values, Keccak256MerkleTreeHasher,
    };
    use crate::test_merkle_tree_generic::{
        merkle_root_calculation_test, merkle_size_test, merkle_test_repeatable_values,
        merkle_tree_serde_failed_test, merkle_tree_serde_test,
        thwart_second_image_attack_using_duplicate_odd_node,
        thwart_second_image_attack_using_interior_nodes,
        thwart_second_image_attack_using_root_node,
    };

    type ValueType = [u8; 32];
    type TestMerkleTreeHasher = Keccak256MerkleTreeHasher;
    const HASH_VALUES_FN: fn(&[&str]) -> Vec<ValueType> = keccak256_hash_leaf_values;

    #[test]
    fn keccak256_merkle_size_test() {
        merkle_size_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn keccak256_merkle_tree_serde_test() {
        merkle_tree_serde_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn keccak256_merkle_tree_serde_failed_test() {
        merkle_tree_serde_failed_test::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    /// Added to check backwards compatibility of Merkle Root value.
    ///
    /// <b>WARNING</b>: The hash value needs to be changed if the code changes!
    #[test]
    fn keccak256_merkle_root_calculation_test() {
        merkle_root_calculation_test::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
            "52a195c0c851e24cf9e99cf7f61552bd24d25a1fb784a5657b3b9d4634aec633",
        );
    }

    #[test]
    fn keccak256_merkle_test_repeatable_values() {
        merkle_test_repeatable_values::<ValueType, TestMerkleTreeHasher>(&HASH_VALUES_FN);
    }

    #[test]
    fn keccak256_thwart_second_image_attack_using_interior_nodes() {
        thwart_second_image_attack_using_interior_nodes::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn keccak256_thwart_second_image_attack_using_root_node() {
        thwart_second_image_attack_using_root_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }

    #[test]
    fn keccak256_thwart_second_image_attack_using_duplicate_odd_node() {
        thwart_second_image_attack_using_duplicate_odd_node::<ValueType, TestMerkleTreeHasher>(
            &HASH_VALUES_FN,
        );
    }
}
