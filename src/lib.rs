pub mod hash_table;

pub use hash_table::*;

#[cfg(test)]
mod tests {

  use std::collections::hash_map::RandomState;

  use crate::BuildHasherWrapper;
  use crate::HashTable;
  use crate::PowerOf2HashPolicy;

  #[test]
  fn table_works() {
    let mut map: HashTable<i32, i32> = HashTable::new();
    map.insert(1, 10);
    assert_eq!(map.get(&1), Some(&10));
    assert_eq!(map.len(), 1);

    let build_hasher = RandomState::new();
    let policy = PowerOf2HashPolicy;
    let wrap_hasher = BuildHasherWrapper::new(build_hasher, policy);

    let mut map2: HashTable<String, usize, _, PowerOf2HashPolicy> =
      HashTable::with_capacity_and_hasher(10, wrap_hasher);

    map2.insert("hello".to_string(), 1);
    assert_eq!(map2.get("hello"), Some(&1));
  }

  #[test]
  fn flat_hash_map_works() {
    let mut map: HashTable<String, i32> = HashTable::new();

    map.insert("a".to_string(), 1);
    assert_eq!(map.get("a"), Some(&1));

    map.insert("a".to_string(), 2);
    assert_eq!(map.get("a"), Some(&2));

    map.insert("b".to_string(), 3);
    assert_eq!(map.len(), 2);

    assert_eq!(map.get("a"), Some(&2));
    assert_eq!(map.get("b"), Some(&3));
  }
}
