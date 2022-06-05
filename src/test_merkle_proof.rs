#[cfg(test)]
mod tests {

    use crate::BlakeMerkleTree;
    use crate::blake3_merkle_tree_hasher::blake3_hash_leaf_values;

    const SINGLE_CHAR_VALUES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

    #[test]
    fn blake3_merkle_proof_test() {
        let leaves = blake3_hash_leaf_values(&SINGLE_CHAR_VALUES);
        let tree = BlakeMerkleTree::build(&leaves).unwrap();
        for idx in 0..leaves.len() {
            let proof = tree.build_proof(idx).unwrap();
            let valid = proof.validate_proof(&leaves.as_slice()[idx]);
            assert!(valid);
        }
    }

    #[test]
    fn blake3_merkle_proof_repeatable_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        for idx in 0..SINGLE_CHAR_VALUES.len() {
            let proof1 = tree.build_proof(idx).unwrap();
            let proof2 = tree.build_proof(idx).unwrap();
            assert_eq!(proof1, proof2);
        }
    }

    #[test]
    fn blake3_merkle_proof_failure_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        for idx in 0..SINGLE_CHAR_VALUES.len() {
            let proof = tree.build_proof(idx).unwrap();
            assert!(!proof.validate_proof(&tree.root()));
        }
    }

    #[test]
    fn blake3_merkle_proof_serde_test() {
        let tree = BlakeMerkleTree::build(&blake3_hash_leaf_values(&SINGLE_CHAR_VALUES)).unwrap();
        for idx in 0..SINGLE_CHAR_VALUES.len() {
            let proof = tree.build_proof(idx).unwrap();
            let json = serde_json::to_string(&proof).unwrap();
            let proof_de = serde_json::from_str(&json).unwrap();
            assert_eq!(proof, proof_de);
        }
    }
}