pub(crate) const LEAF_PREFIX: [u8; 1] = [0_u8];
pub(crate) const NON_LEAF_NODE_STARTING_PREFIX: [u8; 1] = [1_u8];

/// A Hasher for generating Merkle Tree leaf and node hashes.
///
/// [MerkleTree](crate::MerkleTree) internally uses a [MerkleTreeHasher] implementation to generate
/// hashes for leaves and nodes.
///
/// [MerkleProof](crate::MerkleProof) internally uses a [MerkleTreeHasher] implementation to
/// verify siblings values.
///
/// [MerkleTreeHasher] implementations are expected to behave well in a multi-threaded environment.
pub trait MerkleTreeHasher<T: Copy> {
    /// The name of the [MerkleTreeHasher] implementation.
    ///
    /// Needed to identify which hasher created a [MerkleTree](crate::MerkleTree)
    /// or [MerkleProof](crate::MerkleProof).
    fn name(&self) -> String;

    /// Hash a Leaf Node. Prefixing it with [`leaf_prefix()`](MerkleTreeHasher.leaf_prefix).
    fn hash_leaf(value: &T) -> T;

    /// Hash a non-Leaf Node. Prefixing it with the specified prefix.
    fn hash_non_leaf_node(prefix: &[u8; 1], lhs: &T, rhs: &T) -> T;

    /// The Leaf Prefix for this [MerkleTreeHasher].
    fn leaf_prefix() -> [u8; 1] {
        LEAF_PREFIX
    }

    /// Starting prefix for non-Leaf Nodes for this [MerkleTreeHasher].
    fn non_leaf_node_starting_prefix() -> [u8; 1] {
        NON_LEAF_NODE_STARTING_PREFIX
    }

    /// Should the non-leaf-node-prefix exceed 255 it will wrap around to this number.
    fn wrap_to_value() -> u8 {
        1
    }
}
