extern crate sherwood_table;

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;

use sherwood_table::HashPolicy;
use sherwood_table::HashTable;
use sherwood_table::PowerOf2HashPolicy;

#[derive(Clone, Debug, PartialEq, Eq)]
struct CollisionKey(i32);

impl Hash for CollisionKey {
  fn hash<H: Hasher>(&self, _state: &mut H) {}
}

#[test]
fn test_collision_handling() {
  let mut table: HashTable<CollisionKey, String> = HashTable::new();

  for i in 0..20 {
    table.insert(CollisionKey(i), format!("value_{}", i));
  }

  assert_eq!(table.len(), 20);

  for i in 0..20 {
    assert_eq!(table.get(&CollisionKey(i)), Some(&format!("value_{}", i)));
  }

  table.remove(&CollisionKey(5));
  table.remove(&CollisionKey(10));
  table.remove(&CollisionKey(15));

  assert_eq!(table.len(), 17);

  for i in 0..20 {
    if i == 5 || i == 10 || i == 15 {
      assert_eq!(table.get(&CollisionKey(i)), None);
    } else {
      assert_eq!(table.get(&CollisionKey(i)), Some(&format!("value_{}", i)));
    }
  }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct LargeKey {
  data: [u8; 1024],
  id: usize,
}

impl LargeKey {
  fn new(id: usize) -> Self {
    let mut data = [0u8; 1024];
    for i in 0..1024 {
      data[i] = ((id + i) % 256) as u8;
    }
    Self { data, id }
  }
}

#[test]
fn test_large_keys() {
  let mut table: HashTable<LargeKey, String> = HashTable::new();

  for i in 0..10 {
    table.insert(LargeKey::new(i), format!("value_{}", i));
  }

  assert_eq!(table.len(), 10);

  for i in 0..10 {
    assert_eq!(table.get(&LargeKey::new(i)), Some(&format!("value_{}", i)));
  }

  let mut different_key = LargeKey::new(5);
  different_key.data[500] = 42;
  assert_eq!(table.get(&different_key), None);
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct HighLoadFactorPolicy;

impl HashPolicy for HighLoadFactorPolicy {
  fn new_capacity(&self, capacity: usize) -> usize {
    let min_capacity = capacity.max(4);
    (min_capacity as f64 * 0.75).ceil() as usize
  }

  fn hash_index(&self, hash: u64, num_slots: usize) -> usize {
    hash as usize % (num_slots + 1)
  }

  fn commit(&mut self, _smth: u64) {}

  fn reset(&mut self) {}
}

#[test]
fn test_high_load_factor() {
  let mut table: HashTable<i32, String, _, HighLoadFactorPolicy> =
    HashTable::with_hasher_and_policy(RandomState::new(), HighLoadFactorPolicy);

  for i in 0..100 {
    table.insert(i, format!("value_{}", i));
  }

  assert_eq!(table.len(), 100);

  for i in 0..100 {
    assert_eq!(table.get(&i), Some(&format!("value_{}", i)));
  }

  for i in 0..50 {
    if i % 2 == 0 {
      table.remove(&i);
    }
  }

  assert_eq!(table.len(), 75);

  for i in 0..20 {
    if i % 2 == 0 {
      table.insert(i, format!("new_value_{}", i));
    }
  }

  for i in 0..100 {
    if i < 20 && i % 2 == 0 {
      match table.get(&i) {
        Some(value) => {
          assert!(
            value == &format!("new_value_{}", i)
              || value == &format!("value_{}", i),
            "Expected new_value_{} or value_{}, got {:?}",
            i,
            i,
            value
          );
        }
        None => panic!("Expected a value for key {}, but got None", i),
      }
    } else if i >= 50 || i % 2 == 1 {
      assert_eq!(
        table.get(&i),
        Some(&format!("value_{}", i)),
        "Expected value_{} for key {}, but got {:?}",
        i,
        i,
        table.get(&i)
      );
    } else if i >= 20 && i < 50 && i % 2 == 0 {
      assert_eq!(
        table.get(&i),
        None,
        "Expected None for key {}, but got {:?}",
        i,
        table.get(&i)
      );
    }
  }
}

#[derive(Clone)]
struct HighCollisionHasher;
impl Hasher for HighCollisionHasher {
  fn finish(&self) -> u64 {
    42
  }

  fn write(&mut self, bytes: &[u8]) {
    let _ = bytes.first();
  }
}

#[derive(Clone)]
struct HighCollisionHashBuilder;
impl BuildHasher for HighCollisionHashBuilder {
  type Hasher = HighCollisionHasher;

  fn build_hasher(&self) -> Self::Hasher {
    HighCollisionHasher
  }
}

#[test]
fn test_high_collision_hasher() {
  let mut table: HashTable<String, i32, HighCollisionHashBuilder> =
    HashTable::with_hasher(HighCollisionHashBuilder);

  let num_items = 50;

  for i in 0..num_items {
    table.insert(format!("key_{}", i), i);
  }

  assert_eq!(table.len(), num_items as usize);

  for i in 0..num_items {
    assert_eq!(table.get(&format!("key_{}", i)), Some(&i));
  }

  for i in 0..num_items {
    if i % 3 == 0 {
      table.remove(&format!("key_{}", i));
    }
  }

  for i in 0..num_items {
    if i % 3 == 0 {
      assert_eq!(table.get(&format!("key_{}", i)), None);
    } else {
      assert_eq!(table.get(&format!("key_{}", i)), Some(&i));
    }
  }
}

#[test]
fn test_resize_stress() {
  let mut table: HashTable<i32, i32> = HashTable::with_capacity(16);

  for i in 0..1000 {
    table.insert(i, i * 2);
  }

  assert_eq!(table.len(), 1000);

  for i in 1000..5000 {
    table.insert(i, i * 2);
  }

  assert_eq!(table.len(), 5000);

  for i in 0..5000 {
    assert_eq!(table.get(&i), Some(&(i * 2)));
  }

  for i in 0..5000 {
    if i % 2 == 0 {
      table.remove(&i);
    }
  }

  assert_eq!(table.len(), 2500);

  for i in 0..5000 {
    if i % 2 == 0 {
      assert_eq!(table.get(&i), None);
    } else {
      assert_eq!(table.get(&i), Some(&(i * 2)));
    }
  }
}

#[test]
fn test_unusual_key_types() {
  let mut table1: HashTable<(), i32> = HashTable::new();
  table1.insert((), 42);
  assert_eq!(table1.get(&()), Some(&42));

  #[derive(Debug, Clone, PartialEq, Eq, Hash)]
  struct UnitStruct;

  let mut table2: HashTable<UnitStruct, i32> = HashTable::new();
  table2.insert(UnitStruct, 42);
  assert_eq!(table2.get(&UnitStruct), Some(&42));

  let mut table3: HashTable<Option<i32>, String> = HashTable::new();
  table3.insert(None, "none".to_string());
  table3.insert(Some(1), "some_1".to_string());
  table3.insert(Some(2), "some_2".to_string());

  assert_eq!(table3.get(&None), Some(&"none".to_string()));
  assert_eq!(table3.get(&Some(1)), Some(&"some_1".to_string()));
  assert_eq!(table3.get(&Some(2)), Some(&"some_2".to_string()));
  assert_eq!(table3.get(&Some(3)), None);
}

