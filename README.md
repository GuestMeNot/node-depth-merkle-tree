Creates a Merkle Tree.

 ## Why another Merkle Tree?

   1. Merkle Tree hash calculations are CPU intensive and ideally shouldn't block to other CPU activities.
   2. merkle_light is single-threaded.
   3. rs_merkle is slower in example benchmarks cases than merkle_lite and this implementation.

 ## Example

  This example assumes we have enabled the 'sha256_hash' feature:
```
  use node_depth_merkle_tree::Sha256MerkleTree;
  use rustc_serialize::hex::FromHex;
  
  let item1: [u8; 32] = *<&[u8; 32]>::try_from(
      "52a195c0c851e24cf9e99cf7f61552bd24d25a1fb784a5657b3b9d4634aec633"
          .from_hex()
          .unwrap()
          .as_slice(),
  )
  .unwrap();

  let item2: [u8; 32] = *<&[u8; 32]>::try_from(
      "edd5d4b7c614215ae12bbe871e2cf2f5f76747d3789f8809dc545c2cf45e4867"
          .from_hex()
          .unwrap()
          .as_slice(),
  )
  .unwrap();
  
  let items = vec![item1, item2];
  let tree = Sha256MerkleTree::build(&items).unwrap();
  let _root = tree.root();
```

 ## Build commands

 Install and Run Cargo Readme

    cargo install cargo-readme
    cargo readme > README.md

 Create the rustdocs:

    cargo rustdoc  --all-targets --all-features

 Run the Benchmarks:

    cargo bench --all-features

  Run the tests

     cargo test

  Run fmt

     cargo fmt --all -- --check


 ## Design Priorities

   1. Security - Performance is sacrificed to avoid Merkle Tree Second Preimage attacks.
      Duplication of the last leaf is not possible for this merkle tree implementation.
      This implementation uses node levels instead of 0 and 1 as prefixes.
      Unit tests are added to check various Second Preimage attacks.

      See: <https://djsec.wordpress.com/2018/02/21/attacking-merkle-trees-with-a-second-preimage-attack/>
   

   2. Performance - Ease of use and Readability are somewhat sacrificed in favor of
      performance. Merkle trees can be CPU intensive and should be calculated quickly.
      Floating point arithmetic and complex calculations are avoided.
      Use of simple usize addition and any arithmetic which the compiler will convert to quick operations.


      For example: x % 2 is optimized to x & 1.

   3. Maximizing performance is more important that saving space unless the space is dramatically
      increased for a small bump in performance.


   4. Usage Simplicity

         a. Rolling back changes, and other fancy features are not supported.
            Adding async makes the API trickier. This could be handled internally if needed.
   
         b. The Api should be straightforward and minimal. It should be idiomatic Rust.

         c. By looking at code samples, usage should be straightforward.


   5. Flexibility - There should be a way to modify certain expected behaviors without obfuscating the code.

         a. We want to be able the change the hash function as needed and be able to use any
         hashing implementation.

         b. We want to be able to change the hash length as needed.

         c. Allow future the implementation to perform concurrent hash calculation.


   6. Code Readability - Where performance, security and flexibility are not sacrificed,
         the code should be straightforward. Fancy features other than generics and
         passing hash functions should be avoided. Bit-twiddling is generally avoided.


   7. Minimize requirements on the Caller.

         a. If leaf uniqueness is required, the caller must ensure that is the case beforehand.

         b. No non-standard traits the caller needs to implement other than 
             [MerkleTreeHasher](src/merkle_tree_hasher.rs).

         c. The choice of how to hash should be a calling code decision.
   
         d. Supply a few common [MerkleTreeHasher](src/merkle_tree_hasher.rs)s implementations.

         e. Allow callers to [Serialize](https://docs.serde.rs/serde/ser/trait.Serialize.html)
             and [Deserialize](https://docs.serde.rs/serde/de/trait.Deserialize.html) a 
             [MerkleTree](src/merkle_tree.rs) and [MerkleProof](src/merkle_proof.rs) using serde.
   
 ## Benchmarking:

 Simplified benchmarking showed that while generating hash for each level:

   1. Functional programming significantly slowed Merkle Tree generation.
       Collecting the intermediate hashes into a Vec and then adding to the tree slowed things down.

   2. Rayon's par_iter() slowed generation of merkle tree hashes:
       <https://docs.rs/rayon/0.6.0/rayon/par_iter/index.html>

   3. Updating Hashers incrementally, is generally faster than concatenating bytes before hashing.
      Likewise creating a \[\[u8; xx\]\] array and passing the values one \[u8; xx\] at a time is also slow.
      Either way, this is a calling code decision.
   
 ## Upcoming Goals:

   1. Non-blocking - Play fairly with other activities on the machine.
      Hashing is a relatively expensive operation. Some folks recommend
      releasing the thread after ~10ns. This could be added as a feature to
      allow existing code to opt out as needed.

   2. Concurrency - Code should work in parallel if feasible.
      A natural approach would be to calculate hashes at each tree level concurrently.
      This could be added as a feature to allow existing code to opt out as needed.
