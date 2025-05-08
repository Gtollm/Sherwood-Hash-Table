# Sherwood Table

A high-performance Rust implementation of a hash table using Robin Hood hashing.

## Features

- Fast and memory-efficient hash table implementation
- Uses Robin Hood hashing to reduce probe sequence variance
- Configurable hash policies with `PowerOf2HashPolicy` as default
- Supports custom hashers
- Lazy initialization that allocates memory only when needed
- Full iterator support

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
sherwood_table = "0.1.0"
```

### Basic Examples

```rust
use sherwood_table::HashTable;

// Create a new hash table
let mut table: HashTable<i32, String> = HashTable::new();

// Insert key-value pairs
table.insert(1, "one".to_string());
table.insert(2, "two".to_string());

// Get values
assert_eq!(table.get(&1), Some(&"one".to_string()));

// Update values
table.insert(1, "ONE".to_string());
assert_eq!(table.get(&1), Some(&"ONE".to_string()));

// Iterate over all entries
for (key, value) in &table {
    println!("{}: {}", key, value);
}

// Remove entries
let removed = table.remove(&1);
assert_eq!(removed, Some("ONE".to_string()));
```

### Custom Hashers and Policies

```rust
use sherwood_table::{HashTable, PowerOf2HashPolicy, BuildHasherWrapper};
use std::collections::hash_map::RandomState;

// Create a custom hash state and policy
let hasher = RandomState::new();
let policy = PowerOf2HashPolicy;
let hash_wrapper = BuildHasherWrapper::new(hasher, policy);

// Use the custom hasher and policy
let mut table: HashTable<String, i32, _, _> = 
    HashTable::with_hasher_and_policy(hasher, policy);

// Insert and retrieve as normal
table.insert("hello".to_string(), 42);
assert_eq!(table.get("hello"), Some(&42));
```

## Implementation Details

Sherwood Table uses Robin Hood hashing, a form of open addressing that minimizes variance in probe sequence lengths by systematically shuffling entries based on their "desired distance" from their ideal hash bucket. This approach:

- Reduces worst-case lookup time
- Improves cache locality
- Provides more predictable performance under high load factors

The implementation uses a 1-based indexing scheme for the backing array and handles hash collisions through linear probing with Robin Hood displacement.

## Performance

The hash table is designed for high performance with:
- O(1) average case for insertions, lookups, and deletions
- Optimized memory usage with lazy allocation
- Configurable load factor for space/time tradeoffs
- Efficient resizing strategy

Still not as fast as built-in hash table.

## Acknowledgments

This project was inspired by the `sherwood_v3_table` created by Malte Skarupke. The original work provided valuable insights and ideas.

## License

This project is licensed under the MIT License - see the LICENSE file for details. 
