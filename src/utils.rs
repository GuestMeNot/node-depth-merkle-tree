#![warn(missing_docs)]

/// Internal use convenience function to hash strings into a [Vec] of type T.
#[doc(hidden)]
pub(crate) fn hash_values<T, H>(values: &[&str], hash_fn: H) -> Vec<T>
    where T: Clone + AsRef<[u8]>,
          H: Fn(&[u8]) -> T {
    let mut hashes: Vec<T> = Vec::with_capacity(values.len());
    for value in values {
        let hash = hash_fn(value.as_bytes());
        hashes.push(hash);
    }
    hashes
}

/// Given the number of leaves count the number nodes needed for the Merkle Tree.
#[inline]
#[doc(hidden)]
pub(crate) fn count_tree_nodes(num_leaves: usize) -> usize {
    let mut count = num_leaves;
    let mut level_count = num_leaves;
    while level_count > 1 {
        let prev_level_count = level_count;
        level_count /= 2;
        if prev_level_count != 2 {
            level_count = add_1_if_odd(level_count);
        }
        count += level_count;
    }
    count
}

/// if there are more than u8::MAX levels we wrap around to the wrap_to_value parameter value.
#[inline]
#[doc(hidden)]
pub(crate) fn increment_or_wrap_around(prefix: u8, wrap_to_value: u8) -> u8 {
    if prefix == u8::MAX {
        wrap_to_value
    } else {
        prefix + 1
    }
}

/// If the parameter is odd increment by one.
#[inline]
#[doc(hidden)]
pub(crate) fn add_1_if_odd(value: usize) -> usize {
    if is_odd(value) {
        value + 1
    } else {
        value
    }
}

/// True if value is odd.
#[inline]
#[doc(hidden)]
pub(crate) fn is_odd(value: usize) -> bool {
    true as usize == value % 2
}
