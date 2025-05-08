use std::collections::hash_map::RandomState;

use sherwood_table::HashTable;
use sherwood_table::PowerOf2HashPolicy;

fn main() {
  println!("Basic HashTable Example");
  println!("----------------------");

  let mut table: HashTable<String, i32> = HashTable::new();

  table.insert("apple".to_string(), 1);
  table.insert("banana".to_string(), 2);
  table.insert("cherry".to_string(), 3);

  println!("apple: {:?}", table.get("apple"));
  println!("banana: {:?}", table.get("banana"));
  println!("cherry: {:?}", table.get("cherry"));
  println!("dragonfruit: {:?}", table.get("dragonfruit"));

  table.insert("apple".to_string(), 100);
  println!("apple (updated): {:?}", table.get("apple"));

  println!("\nIterating over all entries:");
  for (key, value) in &table {
    println!("{}: {}", key, value);
  }

  let removed = table.remove("banana");
  println!("\nRemoved banana: {:?}", removed);
  println!("banana after removal: {:?}", table.get("banana"));

  println!("Table length: {}", table.len());

  println!("\nAdvanced Example with Custom Hasher");
  println!("----------------------------------");

  let hasher = RandomState::new();
  let policy = PowerOf2HashPolicy;

  let mut advanced_table: HashTable<i32, String, _, _> =
    HashTable::with_capacity_and_hasher_and_policy(10, hasher, policy);

  for i in 0..5 {
    advanced_table.insert(i, format!("Value_{}", i));
  }

  for i in 0..7 {
    println!("Key {}: {:?}", i, advanced_table.get(&i));
  }

  if let Some(value) = advanced_table.get_mut(&2) {
    *value = "Modified Value".to_string();
  }

  println!("\nAfter modification:");
  println!("Key 2: {:?}", advanced_table.get(&2));

  advanced_table = HashTable::new();
  println!("\nNew table is empty: {}", advanced_table.is_empty());
}
