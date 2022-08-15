
# {{crate}}

Current version: {{version}}

# This project creates a Merkle Tree and Merkle Proof.

## What is a Merkle Tree?

Imagine you have a magic brush that paints things a color:

  - when you paint a baseball it turns blue.
  - when you paint a baseball glove it turns yellow.
  - if you paint a second baseball it will still turn blue.
  - if you repaint both blue baseballs at the same time they will turn dark blue.
  - if you repaint both dark blue baseballs and the yellow baseball glove at the same time they all turn green.

Side NOTE:

  - if you paint an unpainted baseball and a yellow glove at the same time they will all turn pale yellow.
  - if you paint an unpainted baseball and a blue baseball they will both turn pale blue.
  - The colors never repeat. For example, you can't paint the dark blue items
        and the pale blue items at the same time and get the original blue baseball color.

This example illustrates the basics of a Merkle Tree:

  - when you input a value say A, it always returns B just like painting a baseball turning blue.
  - when you input another value C, it will always return D just like the glove turning yellow.
  - when you input two values like B and D it always returns G just like the blue baseball and the yellow glove both turning green.
  - when you input two values that are identical say B and B it returns a different value DB just like the 2 blue baseballs turn dark blue.
  - when you input a input value A and D it will not return the same value as inputting B and D.
  - when you input a input value A and B it will not return the same value as inputting B and B.

{{readme}}

  **`default`** - Enables the **`sha256_hash`** feature.

  **`blake3_hash`** - Enables MerkleTree and MerkleProof creation using Blake3.

  **`keccak256_hash`** - Enables MerkleTree and MerkleProof creation using Keccak-256.

  **`md5_hash`** - Enables MerkleTree and MerkleProof creation using MD5.

  **`sha256_hash`** - Enables MerkleTree and MerkleProof creation using SHA-256.

  **`parallel_hashing`** - Enables parallel hashing. This is highly experimental!
        `parallel_hashing` looks most promising for a higher number of cores and a large number of leaves.
        It should be tested in your environment. Currently, this only works in parallel for leaf hashes.
        See benches/rayon_bench.rs for more information.

### Examples


 This example assumes the **`sha256_hash`** feature is enabled:
```
 extern crate rustc_serialize;

 use node_depth_merkle_tree::Sha256MerkleTree;
 use rustc_serialize::hex::FromHex;

 // Setup to create a Vec of [u8; 32]
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

 // Build the Merkle Tree using SHA-256 and get the root.
 let tree = Sha256MerkleTree::new(&items).unwrap();
 let _root = tree.root();
 ```

 This example assumes the **`blake3_hash`** feature is enabled:
```
 extern crate rustc_serialize;

 use node_depth_merkle_tree::Blake3MerkleTree;
 use rustc_serialize::hex::FromHex;

 // Setup to create a Vec of [u8; 32]
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

 // Build the Merkle Tree using Blake3 and get the root.
 let tree = Blake3MerkleTree::new(&items).unwrap();
 let _root = tree.root();
 ```

 ### Design Priorities

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
   
 ### Benchmarking:

 Simplified benchmarking showed that while generating hash for each level:

   1. Collecting the intermediate hashes into a Vec and then adding to the tree slowed things down.

   2. Naive usage of Rayon's par_iter() slowed generation of merkle tree hashes:
       <https://docs.rs/rayon/0.6.0/rayon/par_iter/index.html>

   3. Updating Hashers incrementally is faster than concatenating bytes before hashing.
      Likewise, creating a \[\[u8; xx\]\] array and passing the values one \[u8; xx\] at a time is also slow.

 ### Build commands

 Install and Run cargo-readme

    cargo install cargo-readme
    cargo readme > README.md

 Run the tests

     cargo test

 Run the Benchmarks:

    cargo bench --all-features

 Run fmt

     cargo fmt --all -- --check

 Create the rustdocs:

    cargo rustdoc  --all-targets --all-features

 ### Upcoming Goals:

   1. Non-blocking - Play fairly with other activities on the machine.
      Hashing is a relatively expensive operation. Some folks recommend
      releasing the thread after "10 to 100 microseconds between each call to
      `.await`". This could be added as a feature.

      <https://ryhl.io/blog/async-what-is-blocking/>

   2. Concurrency - Code should work in parallel if feasible.
      A natural approach would be to calculate hashes at each tree level concurrently.
      This could be added as a feature to allow existing code to opt out as needed.

      See: `benches\rayon_bench.rs` for more details.

