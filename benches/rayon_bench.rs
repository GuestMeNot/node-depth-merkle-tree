#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use rayon::prelude::*;
    use test::Bencher;

    const LEN: usize = 1000;

    /// This single threaded bench is used for comparison with multi-threaded options.
    #[bench]
    fn bench_blake3(bencher: &mut Bencher) {
        let values = gen_blake3_values("a", LEN);
        bencher.iter(|| hash_blake3(&values));
    }

    /// This bench shows that around 500 to 1000 iterations that Rayon `par_iter()` increases performance.
    ///
    /// **NOTE**: The following points illustrate that using parallel processing to increase
    /// performance of the [MerkleTree](crate::MerkleTree) is tricky:
    ///
    /// 1. Rayon's `par_iter` does not work well with slices of undefined length.
    ///
    /// 2. Passing a `&Vec` to create the `MerkleTree` rather than a slice or `Iter` dramatically
    ///    decreased performance when using Rayon's `par_iter()` to create the [MerkleTree] leaf
    ///    hashes compared to a simple `for` loop
    ///
    /// 2. Creating a generic `[T; N]` using a `const` generic parameter to satisfy Rayon's `par_iter`
    ///    traits requires the caller to know the exact number of leaves at compile time.
    ///
    /// 3. Using Rayon's `par_bridge` leaves reorders the results of the original data.
    ///    There are two possibilities to think about:
    ///
    ///      **a.** Using `iter.enumerate().par_bridge`. This would add extra bookkeeping
    ///         code to reorder the results to match the original leaf indices.
    ///
    ///      **b.** Requiring sorted leaves. There are additional design questions to consider for
    ///         this case:
    ///
    ///       - How does a naive caller know to sort the leaves prior to calling `MerkleTree::new`?
    ///         It could be easy to make a mistake.
    ///
    ///       - The caller could pass a sorting `fn` to `MerkleTree::new`. This feels like an
    ///         unreasonable burden to place on the caller. Additionally, the indices of the
    ///         resulting `MerkleTree` leaves may not match the indices before the call the to
    ///         `MerkleTree::new`. It could then be tricky to use `MerkleProof` correctly.
    #[bench]
    fn bench_blake3_par(bencher: &mut Bencher) {
        let values = gen_blake3_values("a", LEN);
        bencher.iter(|| hash_blake3_par(&values));
    }

    /// This bench shows at around 500 to 1000 iterations
    /// there is no performance improvement gained by using for `rayon::spawn`
    /// vs. using a single thread.
    #[bench]
    fn bench_blake3_spawn(bencher: &mut Bencher) {
        let values = gen_blake3_values("a", LEN);
        bencher.iter(|| hash_blake3_spawn(&values));
    }

    fn hash_blake3(values: &Vec<[u8; 32]>) {
        for value in values {
            hash(value.as_ref());
        }
    }

    fn hash_blake3_spawn(values: &Vec<[u8; 32]>) {
        for value in values {
            let val = value.to_owned();
            rayon::spawn(move || hash_no_ret(val));
        }
    }

    fn hash_no_ret(value: [u8; 32]) {
        hash(value.as_ref());
    }

    fn hash_blake3_par<T: AsRef<[u8]> + Send + Sync>(values: &Vec<T>) {
        let _x: Vec<[u8; 32]> = values
            .par_iter()
            .map(|value| hash(value.as_ref()))
            .collect();
    }

    fn gen_blake3_values(seed: &str, len: usize) -> Vec<[u8; 32]> {
        let mut bytes = seed.as_bytes();
        let mut v = Vec::with_capacity(len);
        for _i in 0..len {
            let val = hash(bytes);
            v.push(val);
            bytes = v.last().unwrap().as_ref();
        }
        v
    }

    fn hash(value: &[u8]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        hasher.update(value);
        hasher.finalize().as_bytes().to_owned()
    }
}
