#![feature(test)]

extern crate test;

#[cfg(test)]
mod tests {

    use rayon::prelude::*;
    use test::Bencher;

    const LEN: usize = 1000;

    #[bench]
    fn bench_blake3(bencher: &mut Bencher) {
        let values = gen_blake3_values("a", LEN);
        bencher.iter(|| hash_blake3(&values));
    }

    #[bench]
    fn bench_blake3_par(bencher: &mut Bencher) {
        let values = gen_blake3_values("a", LEN);
        bencher.iter(|| hash_blake3_par(&values));
    }

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
