extern crate sherwood_table;

use sherwood_table::HashTable;

#[test]
fn test_empty_table() {
  let table: HashTable<i32, String> = HashTable::new();
  assert_eq!(table.len(), 0);
  assert!(table.is_empty());
  assert_eq!(table.capacity(), 0);
  assert_eq!(table.get(&1), None);
}

#[test]
fn test_insert_and_get() {
  let mut table: HashTable<i32, String> = HashTable::new();

  assert_eq!(table.insert(1, "one".to_string()), None);
  assert_eq!(table.insert(2, "two".to_string()), None);
  assert_eq!(table.insert(3, "three".to_string()), None);

  assert_eq!(table.len(), 3);
  assert!(!table.is_empty());

  assert_eq!(table.get(&1), Some(&"one".to_string()));
  assert_eq!(table.get(&2), Some(&"two".to_string()));
  assert_eq!(table.get(&3), Some(&"three".to_string()));
  assert_eq!(table.get(&4), None);
}

#[test]
fn test_insert_overwrite() {
  let mut table: HashTable<i32, String> = HashTable::new();

  table.insert(1, "one".to_string());
  assert_eq!(table.get(&1), Some(&"one".to_string()));

  let old_value = table.insert(1, "ONE".to_string());
  assert_eq!(old_value, Some("one".to_string()));
  assert_eq!(table.get(&1), Some(&"ONE".to_string()));
  assert_eq!(table.len(), 1);
}

#[test]
fn test_get_mut() {
  let mut table: HashTable<i32, String> = HashTable::new();

  table.insert(1, "one".to_string());
  table.insert(2, "two".to_string());

  if let Some(value) = table.get_mut(&1) {
    *value = "ONE".to_string();
  }

  assert_eq!(table.get(&1), Some(&"ONE".to_string()));
  assert_eq!(table.get(&2), Some(&"two".to_string()));
}

#[test]
fn test_with_capacity() {
  let mut table: HashTable<i32, String> = HashTable::with_capacity(16);

  for i in 0..10 {
    table.insert(i, format!("value_{}", i));
  }

  assert_eq!(table.len(), 10);
  assert!(table.capacity() >= 10);

  for i in 0..10 {
    assert_eq!(table.get(&i), Some(&format!("value_{}", i)));
  }
}

#[test]
fn test_clone() {
  let mut original: HashTable<i32, String> = HashTable::new();

  for i in 0..5 {
    original.insert(i, format!("value_{}", i));
  }

  let cloned = original.clone();

  assert_eq!(original.len(), cloned.len());

  for i in 0..5 {
    assert_eq!(original.get(&i), cloned.get(&i));
  }

  original.insert(5, "new_value".to_string());
  original.remove(&0);

  assert_eq!(original.len(), 5);
  assert_eq!(cloned.len(), 5);
  assert_eq!(cloned.get(&0), Some(&"value_0".to_string()));
  assert_eq!(cloned.get(&5), None);
}

#[test]
fn test_different_key_types() {
  let mut string_table: HashTable<String, i32> = HashTable::new();
  string_table.insert("apple".to_string(), 1);
  string_table.insert("banana".to_string(), 2);

  assert_eq!(string_table.get("apple"), Some(&1));
  assert_eq!(string_table.get("banana"), Some(&2));
  assert_eq!(string_table.get("cherry"), None);

  let mut tuple_table: HashTable<(i32, i32), String> = HashTable::new();
  tuple_table.insert((1, 2), "pair_1_2".to_string());
  tuple_table.insert((3, 4), "pair_3_4".to_string());

  assert_eq!(tuple_table.get(&(1, 2)), Some(&"pair_1_2".to_string()));
  assert_eq!(tuple_table.get(&(3, 4)), Some(&"pair_3_4".to_string()));
  assert_eq!(tuple_table.get(&(5, 6)), None);
}

#[test]
fn test_capacity_growth() {
  let mut table: HashTable<i32, i32> = HashTable::new();

  for i in 0..100 {
    table.insert(i, i * 2);
  }

  assert_eq!(table.len(), 100);
  assert!(table.capacity() >= 100);

  for i in 0..100 {
    assert_eq!(table.get(&i), Some(&(i * 2)));
  }
}

#[test]
fn test_clear_by_removing_all() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..10 {
    table.insert(i, format!("value_{}", i));
  }

  assert_eq!(table.len(), 10);

  for i in 0..10 {
    table.remove(&i);
  }

  assert_eq!(table.len(), 0);
  assert!(table.is_empty());

  for i in 0..5 {
    table.insert(i, format!("new_value_{}", i));
  }

  assert_eq!(table.len(), 5);
  for i in 0..5 {
    assert_eq!(table.get(&i), Some(&format!("new_value_{}", i)));
  }
}

