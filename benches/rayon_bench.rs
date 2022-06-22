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
    /// 1. Rayon's `par_iter` does not work well with slices of undefined length. It works well with
    ///    `Vec`s and fixed size arrays.
    ///
    /// 2. Changing `MerkleTree::new` parameter from slice and `Iter` to `&Vec`dramatically
    ///    decreased performance when using Rayon's `par_iter()` to hash the leaves. This
    ///    appears to happen due to allocating the `Vec` on the heap.
    ///
    ///    <https://stackoverflow.com/questions/40006219/why-is-it-discouraged-to-accept-a-reference-to-a-string-string-vec-vec-o>
    ///
    /// 3. Since Rayon's `par_iter` will work with fixed sized arrays, we can change the `MerkleTree::new`
    ///    parameter to a generic `[T; N]` by adding a `const` generic parameter, but this requires
    ///    the caller to know the exact number of leaves at compile time.
    ///
    /// 4. Using Rayon's `par_bridge` reorders the results compared to the original leaf indices.
    ///    This has the advantage of working with exist fn parameters. The disadvantage is that
    ///    `par_bridge` does not keep the original ordering of the iterator. The indices of the
    ///    resulting hashes likely will not match the leaf indices before the call the to `par_bridge`.
    ///
    ///    There are two possibilities to think about:
    ///
    ///      **a.** Calling `enumerate()` before `par_bridge()` (i.e. `iter.enumerate().par_bridge()`).
    ///         This requires bookkeeping code to reorder the results to match the original
    ///         leaf indices.
    ///
    ///      **b.** Requiring sorted leaves. There are additional design questions to consider for
    ///         this case:
    ///
    ///       - How would a naive caller know to sort the leaves prior to calling `MerkleTree::new`?
    ///         This approach seems prone to coding mistakes by callers.
    ///
    ///       - The caller could pass a sorting `Fn` to `MerkleTree::new`. This feels like an
    ///         unreasonable burden to place on the caller. Since the indices do not match those
    ///         passed in it still seems prone to coding mistakes by callers.
    ///
    /// 5. Implement `IntoParallelRefIterator` as outlined below appears to be the best approach
    ///    so far:
    ///
    ///    <https://stackoverflow.com/questions/35863996/cannot-use-rayons-par-iter#35869613>
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
