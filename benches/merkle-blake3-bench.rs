#![feature(test)]

extern crate merkle_light;
extern crate test;

/// To run these benchmarks the command line is: cargo bench --features="blake3_hash"
#[cfg(test)]
mod tests {
    use test::Bencher;

    use merkle_light::hash::Algorithm;
    use merkle_light::merkle::MerkleTree;
    use rs_merkle::Hasher;
    use rustc_serialize::hex::ToHex;

    use another_merkle_tree::{Blake3MerkleTreeHasher, BlakeMerkleTree, MerkleTreeHasher};

    #[derive(Clone)]
    pub struct Blake3RsMerkleAlgorithm {}

    /// Blake3 Hasher for rs_merkle
    impl Hasher for Blake3RsMerkleAlgorithm {
        type Hash = [u8; 32];

        fn hash(data: &[u8]) -> [u8; 32] {
            *blake3::hash(data).as_bytes()
        }
    }

    pub struct Blake3MerkleLiteAlgorithm(blake3::Hasher);

    impl Blake3MerkleLiteAlgorithm {
        pub fn new() -> Blake3MerkleLiteAlgorithm {
            Blake3MerkleLiteAlgorithm(blake3::Hasher::new())
        }
    }

    impl Default for Blake3MerkleLiteAlgorithm {
        fn default() -> Blake3MerkleLiteAlgorithm {
            Blake3MerkleLiteAlgorithm::new()
        }
    }

    impl std::hash::Hasher for Blake3MerkleLiteAlgorithm {
        #[inline]
        fn finish(&self) -> u64 {
            // All merkle_lite examples have this line since it isn't called internally.
            unimplemented!()
        }

        #[inline]
        fn write(&mut self, msg: &[u8]) {
            self.0.reset();
            self.0.update(msg);
        }
    }

    impl Algorithm<[u8; 32]> for Blake3MerkleLiteAlgorithm {
        #[inline]
        fn hash(&mut self) -> [u8; 32] {
            *self.0.finalize().as_bytes()
        }

        #[inline]
        fn reset(&mut self) {
            self.0.reset();
        }
    }

    impl Blake3MerkleLiteAlgorithm {}

    const SINGLE_CHAR_VALUES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

    #[bench]
    fn bench_blake3_another_merkle_tree(bencher: &mut Bencher) {
        // hash the leaves first.
        let leaves = Blake3MerkleTreeHasher::hash_leaf_values(&SINGLE_CHAR_VALUES);
        bencher.iter(|| BlakeMerkleTree::build(&leaves));
    }

    #[bench]
    fn bench_blake3_rs_merkle(bencher: &mut Bencher) {
        // hash the leaves first.
        let leaves = Blake3MerkleTreeHasher::hash_leaf_values(&SINGLE_CHAR_VALUES);
        bencher.iter(|| blake3_rs_merkle(&leaves));
    }

    #[bench]
    fn bench_blake3_merkle_light(bencher: &mut Bencher) {
        // hash the leaves first.
        let leaves = blake3_hash_leaf_values(&SINGLE_CHAR_VALUES);
        bencher.iter(|| blake3_merkle_light(&leaves));
    }

    fn blake3_rs_merkle(leaves: &Vec<[u8; 32]>) -> Option<String> {
        rs_merkle::MerkleTree::<Blake3RsMerkleAlgorithm>::from_leaves(leaves).root_hex()
    }

    fn blake3_merkle_light(leaves: &Vec<[u8; 32]>) -> String {
        MerkleTree::<[u8; 32], Blake3MerkleLiteAlgorithm>::
        from_iter(leaves.to_owned()).root().to_hex()
    }
}