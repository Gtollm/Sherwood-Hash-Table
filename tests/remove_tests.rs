extern crate sherwood_table;

use std::hash::BuildHasher;
use std::hash::Hasher;

use sherwood_table::HashTable;

#[test]
fn test_remove_single_item() {
  let mut table: HashTable<i32, String> = HashTable::new();

  table.insert(1, "one".to_string());
  assert_eq!(table.len(), 1);

  let removed = table.remove(&1);
  assert_eq!(removed, Some("one".to_string()));
  assert_eq!(table.len(), 0);
  assert!(table.is_empty());
  assert_eq!(table.get(&1), None);
}

#[test]
fn test_remove_nonexistent_item() {
  let mut table: HashTable<i32, String> = HashTable::new();

  let removed = table.remove(&1);
  assert_eq!(removed, None);

  table.insert(1, "one".to_string());
  table.insert(2, "two".to_string());

  let removed = table.remove(&3);
  assert_eq!(removed, None);
  assert_eq!(table.len(), 2);
}

#[test]
fn test_remove_and_reinsert() {
  let mut table: HashTable<i32, String> = HashTable::new();

  table.insert(1, "one".to_string());
  table.remove(&1);
  table.insert(1, "ONE".to_string());

  assert_eq!(table.len(), 1);
  assert_eq!(table.get(&1), Some(&"ONE".to_string()));
}

#[test]
fn test_remove_with_collisions() {
  #[derive(Clone)]
  struct FixedHasher;
  impl Hasher for FixedHasher {
    fn finish(&self) -> u64 {
      0
    }
    fn write(&mut self, _bytes: &[u8]) {}
  }

  #[derive(Clone)]
  struct FixedHashBuilder;
  impl BuildHasher for FixedHashBuilder {
    type Hasher = FixedHasher;
    fn build_hasher(&self) -> Self::Hasher {
      FixedHasher
    }
  }

  let mut table: HashTable<i32, String, FixedHashBuilder> =
    HashTable::with_hasher(FixedHashBuilder);

  table.insert(1, "one".to_string());
  table.insert(2, "two".to_string());
  table.insert(3, "three".to_string());

  assert_eq!(table.len(), 3);

  let removed = table.remove(&2);
  assert_eq!(removed, Some("two".to_string()));
  assert_eq!(table.len(), 2);

  assert_eq!(table.get(&1), Some(&"one".to_string()));
  assert_eq!(table.get(&2), None);
  assert_eq!(table.get(&3), Some(&"three".to_string()));

  let removed = table.remove(&1);
  assert_eq!(removed, Some("one".to_string()));
  assert_eq!(table.len(), 1);

  assert_eq!(table.get(&1), None);
  assert_eq!(table.get(&3), Some(&"three".to_string()));
}

#[test]
fn test_remove_all_items() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..100 {
    table.insert(i, format!("value_{}", i));
  }
  assert_eq!(table.len(), 100);

  for i in 0..100 {
    let removed = table.remove(&i);
    assert_eq!(removed, Some(format!("value_{}", i)));
  }

  assert_eq!(table.len(), 0);
  assert!(table.is_empty());

  for i in 0..100 {
    assert_eq!(table.get(&i), None);
  }
}

#[test]
fn test_remove_and_insert_mixed() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..50 {
    table.insert(i, format!("initial_{}", i));
  }

  for i in 0..30 {
    table.remove(&i);
    table.insert(i + 100, format!("new_{}", i + 100));
  }

  assert_eq!(table.len(), 50);

  for i in 0..30 {
    assert_eq!(table.get(&i), None);
  }

  for i in 30..50 {
    assert_eq!(table.get(&i), Some(&format!("initial_{}", i)));
  }

  for i in 100..130 {
    assert_eq!(table.get(&i), Some(&format!("new_{}", i)));
  }
}

#[test]
fn test_remove_with_string_keys() {
  let mut table: HashTable<String, i32> = HashTable::new();

  table.insert("apple".to_string(), 1);
  table.insert("banana".to_string(), 2);
  table.insert("cherry".to_string(), 3);

  let removed = table.remove("banana");
  assert_eq!(removed, Some(2));
  assert_eq!(table.len(), 2);

  assert_eq!(table.get("apple"), Some(&1));
  assert_eq!(table.get("banana"), None);
  assert_eq!(table.get("cherry"), Some(&3));
}

#[test]
fn test_clone_after_remove() {
  let mut original: HashTable<i32, String> = HashTable::new();
  original.insert(1, "one".to_string());
  original.insert(2, "two".to_string());
  original.insert(3, "three".to_string());

  original.remove(&2);

  let cloned = original.clone();

  assert_eq!(cloned.len(), 2);
  assert_eq!(cloned.get(&1), Some(&"one".to_string()));
  assert_eq!(cloned.get(&2), None);
  assert_eq!(cloned.get(&3), Some(&"three".to_string()));
}

#[test]
fn test_iterator_after_remove() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..10 {
    table.insert(i, format!("value_{}", i));
  }

  table.remove(&3);
  table.remove(&5);
  table.remove(&7);

  assert_eq!(table.len(), 7);

  let mut collected = Vec::new();
  for (key, value) in &table {
    collected.push((*key, value.clone()));
  }

  assert_eq!(collected.len(), 7);

  assert!(!collected.iter().any(|(k, _)| *k == 3));
  assert!(!collected.iter().any(|(k, _)| *k == 5));
  assert!(!collected.iter().any(|(k, _)| *k == 7));
}
