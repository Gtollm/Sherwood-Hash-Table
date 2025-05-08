extern crate sherwood_table;

use std::collections::HashSet;

use sherwood_table::HashTable;

#[test]
fn test_empty_iterator() {
  let table: HashTable<i32, String> = HashTable::new();
  let mut iter_count = 0;

  for _ in &table {
    iter_count += 1;
  }

  assert_eq!(iter_count, 0);
}

#[test]
fn test_iterator_basic() {
  let mut table: HashTable<i32, String> = HashTable::new();

  table.insert(1, "one".to_string());
  table.insert(2, "two".to_string());
  table.insert(3, "three".to_string());

  let mut keys = HashSet::new();
  let mut values = HashSet::new();

  for (key, value) in &table {
    keys.insert(*key);
    values.insert(value.clone());
  }

  assert_eq!(keys.len(), 3);
  assert!(keys.contains(&1));
  assert!(keys.contains(&2));
  assert!(keys.contains(&3));

  assert_eq!(values.len(), 3);
  assert!(values.contains("one"));
  assert!(values.contains("two"));
  assert!(values.contains("three"));
}

#[test]
fn test_iterator_consistency() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..20 {
    table.insert(i, format!("value_{}", i));
  }

  let mut first_iteration = Vec::new();
  for (key, value) in &table {
    first_iteration.push((*key, value.clone()));
  }

  assert_eq!(first_iteration.len(), 20);

  let mut second_iteration = Vec::new();
  for (key, value) in &table {
    second_iteration.push((*key, value.clone()));
  }

  assert_eq!(second_iteration.len(), 20);

  first_iteration.sort_by_key(|(k, _)| *k);
  second_iteration.sort_by_key(|(k, _)| *k);

  assert_eq!(first_iteration, second_iteration);
}

#[test]
fn test_iterator_after_modifications() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..10 {
    table.insert(i, format!("value_{}", i));
  }

  table.remove(&2);
  table.remove(&5);
  table.remove(&8);

  table.insert(20, "twenty".to_string());
  table.insert(30, "thirty".to_string());

  let mut elements: Vec<(i32, String)> = Vec::new();
  for (key, value) in &table {
    elements.push((*key, value.clone()));
  }

  assert_eq!(elements.len(), 9);

  elements.sort_by_key(|(k, _)| *k);

  let expected_keys = vec![0, 1, 3, 4, 6, 7, 9, 20, 30];
  for (i, (key, _)) in elements.iter().enumerate() {
    assert_eq!(*key, expected_keys[i]);
  }
}

#[test]
fn test_iterator_size_hint() {
  let mut table: HashTable<i32, String> = HashTable::new();

  for i in 0..50 {
    table.insert(i, format!("value_{}", i));
  }

  let mut iter = table.iter();
  assert_eq!(iter.size_hint(), (50, Some(50)));

  for _ in 0..20 {
    iter.next();
  }

  assert_eq!(iter.size_hint(), (30, Some(30)));

  while iter.next().is_some() {}

  assert_eq!(iter.size_hint(), (0, Some(0)));
}

#[test]
fn test_iterator_with_modify_after_clone() {
  let mut original: HashTable<i32, String> = HashTable::new();

  for i in 0..10 {
    original.insert(i, format!("value_{}", i));
  }

  let cloned = original.clone();

  for i in 0..5 {
    original.remove(&i);
  }

  let mut original_items = Vec::new();
  for (key, value) in &original {
    original_items.push((*key, value.clone()));
  }
  assert_eq!(original_items.len(), 5);

  let mut cloned_items = Vec::new();
  for (key, value) in &cloned {
    cloned_items.push((*key, value.clone()));
  }
  assert_eq!(cloned_items.len(), 10);

  original_items.sort_by_key(|(k, _)| *k);
  cloned_items.sort_by_key(|(k, _)| *k);

  for i in 5..10 {
    assert_eq!(original_items[i - 5].0, i as i32);
    assert_eq!(original_items[i - 5].1, format!("value_{}", i));
  }

  for i in 0..10 {
    assert_eq!(cloned_items[i].0, i as i32);
    assert_eq!(cloned_items[i].1, format!("value_{}", i));
  }
}

#[test]
fn test_large_iterator() {
  let mut table: HashTable<i32, i32> = HashTable::new();

  let num_elements = 1000;
  for i in 0..num_elements {
    table.insert(i, i * 2);
  }

  let mut count = 0;
  let mut sum = 0;

  for (key, value) in &table {
    count += 1;
    sum += key + value;
    assert_eq!(*value, *key * 2);
  }

  assert_eq!(count, num_elements);

  assert_eq!(sum, 1498500);
}

