#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use test::Bencher;

    use ring::digest::{Context, SHA256};

    const SINGLE_CHAR_VALUES: [&str; 6] = ["a", "b", "c", "d", "e", "f"];

    #[bench]
    fn bench_sha256(bencher: &mut Bencher) {
        bencher.iter(|| sha256_leaves(&SINGLE_CHAR_VALUES));
    }

    #[bench]
    fn bench_blake3(bencher: &mut Bencher) {
        bencher.iter(|| blake3_leaves(&SINGLE_CHAR_VALUES));
    }

    pub fn blake3_leaves(values: &[&str]) -> Vec<[u8; 32]> {
        hash_values(values, blake3_hash_into_bytes)
    }

    fn sha256_leaves(values: &[&str]) -> Vec<[u8; 32]> {
        hash_values(values, sha256_hash_into_bytes)
    }

    pub fn blake3_hash_into_bytes(values: &[&[u8]]) -> [u8; 32] {
        let mut hasher = blake3::Hasher::new();
        for value in values {
            hasher.update(value);
        }
        hasher.finalize().as_bytes().to_owned()
    }

    pub fn sha256_hash_into_bytes(values: &[&[u8]]) -> [u8; 32] {
        let mut context = Context::new(&SHA256);

        for value in values {
            context.update(value);
        }

        let digest = context.finish();
        <[u8; 32]>::try_from(digest.as_ref()).unwrap()
    }

    fn hash_values<T, H>(values: &[&str], hash_fn: H) -> Vec<T>
        where T: Clone + AsRef<[u8]>,
              H: Fn(&[&[u8]]) -> T {
        let mut hashes: Vec<T> = Vec::with_capacity(values.len());
        for value in values {
            let arr = [value.as_bytes()];
            let hash = hash_fn(arr.as_ref());
            hashes.push(hash);
        }
        hashes
    }
}