extern crate sherwood_table;

use std::collections::hash_map::RandomState;
use std::hash::BuildHasher;

use sherwood_table::BuildHasherWrapper;
use sherwood_table::HashPolicy;
use sherwood_table::HashTable;
use sherwood_table::PowerOf2HashPolicy;

#[test]
fn test_power_of_2_policy() {
  let policy = PowerOf2HashPolicy;

  assert_eq!(policy.hash_index(5, 7), 5 & 7);
  assert_eq!(policy.hash_index(10, 7), 10 & 7);
  assert_eq!(policy.hash_index(15, 7), 15 & 7);
  assert_eq!(policy.hash_index(16, 15), 16 & 15);
  assert_eq!(policy.hash_index(31, 15), 31 & 15);
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct ModuloHashPolicy;

impl HashPolicy for ModuloHashPolicy {
  fn new_capacity(&self, capacity: usize) -> usize {
    let primes = [5, 11, 17, 23, 29, 37, 47, 59, 71, 89, 107, 131];
    for &prime in &primes {
      if prime >= capacity {
        return prime;
      }
    }
    capacity
  }

  fn hash_index(&self, hash: u64, num_slots: usize) -> usize {
    (hash as usize) % (num_slots + 1)
  }

  fn commit(&mut self, _smth: u64) {}

  fn reset(&mut self) {}
}

#[test]
fn test_custom_policy() {
  let policy = ModuloHashPolicy;

  assert_eq!(policy.new_capacity(1), 5);
  assert_eq!(policy.new_capacity(5), 5);
  assert_eq!(policy.new_capacity(6), 11);
  assert_eq!(policy.new_capacity(11), 11);
  assert_eq!(policy.new_capacity(12), 17);

  assert_eq!(policy.hash_index(5, 4), 5 % 5);
  assert_eq!(policy.hash_index(10, 4), 10 % 5);
  assert_eq!(policy.hash_index(15, 4), 15 % 5);
  assert_eq!(policy.hash_index(16, 10), 16 % 11);
  assert_eq!(policy.hash_index(31, 10), 31 % 11);
}

#[test]
fn test_hash_table_with_custom_policy() {
  let build_hasher = RandomState::new();
  let policy = ModuloHashPolicy;

  let mut table: HashTable<i32, String, _, ModuloHashPolicy> =
    HashTable::with_capacity_and_hasher_and_policy(10, build_hasher, policy);

  for i in 0..20 {
    table.insert(i, format!("value_{}", i));
  }

  for i in 0..20 {
    assert_eq!(table.get(&i), Some(&format!("value_{}", i)));
  }

  assert!(table.capacity() >= 4);

  let retrieved_policy = table.policy();

  assert_eq!(retrieved_policy.new_capacity(6), 11);
  assert_eq!(retrieved_policy.new_capacity(12), 17);
}

#[test]
fn test_policy_with_edge_cases() {
  let mut table: HashTable<i32, i32, _, ModuloHashPolicy> =
    HashTable::with_capacity_and_hasher_and_policy(
      1000,
      RandomState::new(),
      ModuloHashPolicy,
    );

  for i in 0..200 {
    table.insert(i, i * 2);
  }

  for i in 0..200 {
    assert_eq!(table.get(&i), Some(&(i * 2)));
  }

  let mut zero_table: HashTable<i32, i32, _, ModuloHashPolicy> =
    HashTable::with_capacity_and_hasher_and_policy(
      0,
      RandomState::new(),
      ModuloHashPolicy,
    );

  for i in 0..10 {
    zero_table.insert(i, i * 3);
  }

  for i in 0..10 {
    assert_eq!(zero_table.get(&i), Some(&(i * 3)));
  }
}

#[test]
fn test_policy_hash_distribution() {
  let mut power2_table: HashTable<i32, i32> = HashTable::new();
  let mut modulo_table: HashTable<i32, i32, _, ModuloHashPolicy> =
    HashTable::with_hasher_and_policy(RandomState::new(), ModuloHashPolicy);

  for i in 0..100 {
    power2_table.insert(i, i);
    modulo_table.insert(i, i);
  }

  for i in 0..100 {
    assert_eq!(power2_table.get(&i), Some(&i));
    assert_eq!(modulo_table.get(&i), Some(&i));
  }
}

#[test]
fn test_build_hasher_wrapper() {
  let build_hasher = RandomState::new();
  let policy = ModuloHashPolicy;

  let wrapper = BuildHasherWrapper::new(build_hasher.clone(), policy.clone());

  let _hasher = wrapper.build_hasher();

  let mut table: HashTable<String, i32, _, ModuloHashPolicy> =
    HashTable::with_hasher_and_policy(build_hasher, policy);

  table.insert("test".to_string(), 42);
  assert_eq!(table.get("test"), Some(&42));
}
