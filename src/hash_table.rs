use std::borrow::Borrow;
use std::hash::BuildHasher;
use std::hash::Hash;
use std::hash::Hasher;
use std::marker::PhantomData;
use std::usize;

pub(crate) const MIN_LOOKUPS: i8 = 64;

pub(crate) trait Log2Ext {
  fn log2(self) -> i8;
}
impl Log2Ext for usize {
  fn log2(self) -> i8 {
    (usize::BITS - self.leading_zeros() - 1) as i8
  }
}

#[derive(Debug)]
pub(crate) struct HashEntry<T> {
  pub(crate) desired_distance: i8,
  pub(crate) value: Option<T>,
}

impl<T> Default for HashEntry<T> {
  fn default() -> Self {
    Self {
      desired_distance: -1,
      value: None,
    }
  }
}

impl<T: Clone> Clone for HashEntry<T> {
  fn clone(&self) -> Self {
    Self {
      desired_distance: self.desired_distance,
      value: self.value.clone(),
    }
  }
}

impl<T> HashEntry<T> {
  pub(crate) const END_VALUE: u8 = 0;

  #[inline]
  pub(crate) fn new(desired_distance: i8) -> Self {
    Self {
      desired_distance,
      value: None,
    }
  }

  #[inline]
  pub(crate) fn empty() -> Self {
    Self::default()
  }

  #[inline]
  pub(crate) fn has_value(&self) -> bool {
    self.desired_distance >= 0 && self.value.is_some()
  }

  #[inline]
  pub(crate) fn is_empty(&self) -> bool {
    self.desired_distance < 0 && self.value.is_none()
  }

  #[inline]
  pub(crate) fn is_at_desired_position(&self) -> bool {
    self.desired_distance <= 0
  }
}

pub trait HashPolicy {
  fn new_capacity(&self, capacity: usize) -> usize;
  fn hash_index(&self, hash: u64, num_slots: usize) -> usize;
  fn commit(&mut self, smth: u64);
  fn reset(&mut self);
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct PowerOf2HashPolicy;

impl PowerOf2HashPolicy {
  #[inline]
  fn next_power_2(n: usize) -> usize {
    if n == 0 {
      1
    } else {
      n.next_power_of_two()
    }
  }
}

impl HashPolicy for PowerOf2HashPolicy {
  #[inline]
  fn new_capacity(&self, capacity: usize) -> usize {
    Self::next_power_2(capacity.max(crate::MIN_LOOKUPS as usize))
  }
  #[inline]
  fn hash_index(&self, hash: u64, num_slots: usize) -> usize {
    hash as usize & num_slots
  }

  #[inline]
  fn commit(&mut self, _smth: u64) {}
  #[inline]
  fn reset(&mut self) {}
}

pub trait SelectHashPolicy {
  type Policy: HashPolicy + Default + Clone;
}

impl<H> SelectHashPolicy for H {
  type Policy = PowerOf2HashPolicy;
}

#[derive(Debug, Clone, Default)]
pub struct BuildHasherWrapper<H, P = <H as SelectHashPolicy>::Policy>
where
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  pub(crate) build_hasher: H,
  pub(crate) policy: P,
  _marker: PhantomData<fn() -> P>,
}

impl<H, P> BuildHasherWrapper<H, P>
where
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  pub fn new(build_hasher: H, policy: P) -> Self {
    Self {
      build_hasher,
      policy,
      _marker: PhantomData,
    }
  }
}

impl<H, P> BuildHasher for BuildHasherWrapper<H, P>
where
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  type Hasher = H::Hasher;

  #[inline]
  fn build_hasher(&self) -> Self::Hasher {
    self.build_hasher.build_hasher()
  }
}

#[derive(Debug)]
pub struct HashTable<
  K,
  V,
  H = std::collections::hash_map::RandomState,
  P = PowerOf2HashPolicy,
> where
  K: Hash + Eq,
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  build_hasher: BuildHasherWrapper<H, P>,

  buckets: Vec<HashEntry<(K, V)>>,
  num_slots: usize,
  num_elements: usize,
  max_lookups: i8,
  max_load_factor: f32,
  _marker: PhantomData<(K, V)>,
}

impl<K, V, H, P> Default for HashTable<K, V, H, P>
where
  K: Hash + Eq,
  H: BuildHasher + Default + Clone,
  P: HashPolicy + Default + Clone,
{
  fn default() -> Self {
    Self::new()
  }
}

impl<K, V, H, P> Clone for HashTable<K, V, H, P>
where
  K: Hash + Eq + Clone,
  V: Clone,
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  fn clone(&self) -> Self {
    let mut new_table = Self::with_capacity_and_hasher_and_policy(
      self.capacity(),
      self.build_hasher.build_hasher.clone(),
      self.build_hasher.policy.clone(),
    );

    for (k, v) in self.iter() {
      new_table.insert(k.clone(), v.clone());
    }

    new_table
  }
}

impl<K, V, H, P> HashTable<K, V, H, P>
where
  K: Hash + Eq,
  H: BuildHasher + Default + Clone,
  P: HashPolicy + Default + Clone,
{
  pub fn new() -> Self {
    Self::with_hasher(H::default())
  }

  pub fn with_capacity(capacity: usize) -> Self {
    Self::with_capacity_and_hasher(capacity, H::default())
  }
}

impl<K, V, H, P> HashTable<K, V, H, P>
where
  K: Hash + Eq,
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  #[inline]
  pub fn with_hasher(build_hasher: H) -> Self {
    Self::with_hasher_and_policy(build_hasher, P::default())
  }

  #[inline]
  pub fn with_hasher_and_policy(build_hasher: H, policy: P) -> Self {
    Self::with_capacity_and_hasher_and_policy(0usize, build_hasher, policy)
  }

  #[inline]
  pub fn with_capacity_and_hasher(capacity: usize, build_hasher: H) -> Self {
    Self::with_capacity_and_hasher_and_policy(
      capacity,
      build_hasher,
      P::default(),
    )
  }

  #[inline]
  pub fn with_capacity_and_hasher_and_policy(
    capacity: usize,
    build_hasher: H,
    policy: P,
  ) -> Self {
    Self {
      build_hasher: BuildHasherWrapper::new(build_hasher, policy.clone()),
      buckets: Vec::with_capacity(capacity),
      num_slots: 0,
      num_elements: 0,
      max_lookups: MIN_LOOKUPS - 1,
      max_load_factor: 0.5f32,
      _marker: PhantomData,
    }
  }

  #[inline]
  pub fn len(&self) -> usize {
    self.num_elements
  }

  #[inline]
  pub fn is_empty(&self) -> bool {
    self.num_elements == 0
  }

  #[inline]
  pub fn capacity(&self) -> usize {
    if self.num_slots == 0 {
      0
    } else {
      self.num_slots + 1
    }
  }

  #[inline]
  fn calculate_required_vec_len(capacity: usize, max_lookups: i8) -> usize {
    if capacity == 0 {
      return 0;
    }
    capacity + (max_lookups as usize)
  }

  #[inline]
  fn compute_max_lookups(num_buckets: usize) -> i8 {
    if num_buckets == 0 {
      MIN_LOOKUPS - 1
    } else {
      let log2_val = usize::BITS - num_buckets.leading_zeros() - 1;
      (log2_val as i8).max(MIN_LOOKUPS)
    }
  }

  #[inline]
  fn hash_key<Q: ?Sized>(&self, key: &Q) -> u64
  where
    K: Borrow<Q>,
    Q: Hash,
  {
    let mut hasher = self.build_hasher.build_hasher();
    key.hash(&mut hasher);
    hasher.finish()
  }

  #[inline]
  fn desired_index<Q: ?Sized>(&self, key: &Q) -> usize
  where
    K: Borrow<Q>,
    Q: Hash,
  {
    let hash = self.hash_key(key);
    if self.capacity() == 0 {
      return 0;
    }
    self.build_hasher.policy.hash_index(hash, self.num_slots)
  }

  #[inline]
  fn keys_equal<Q: ?Sized>(&self, query_key: &Q, entry_key: &K) -> bool
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    entry_key.borrow() == query_key
  }

  #[inline]
  fn reserve(&mut self, additional: usize) {
    let new_num_elements = self.num_elements.checked_add(additional).unwrap();
    let new_num_buckets = (new_num_elements as f64
      / (self.max_load_factor as f64).min(0.99))
    .ceil() as usize;

    if new_num_buckets > self.capacity() {
      let new_capacity_hint = new_num_buckets.max(MIN_LOOKUPS as usize);
      self.resize(new_capacity_hint);
    }
  }

  #[inline]
  pub fn resize(&mut self, capacity_hint: usize) {
    let new_capacity = self.build_hasher.policy.new_capacity(capacity_hint);
    if new_capacity == self.capacity() && !self.buckets.is_empty() {
      return;
    }

    let new_max_lookups = Self::compute_max_lookups(new_capacity);
    let required_vec_size =
      Self::calculate_required_vec_len(new_capacity, new_max_lookups);

    let new_buckets = if required_vec_size == 0 {
      Vec::new()
    } else {
      let mut vec = Vec::with_capacity(required_vec_size);
      vec.resize_with(required_vec_size, HashEntry::empty);
      vec
    };

    let old_buckets = std::mem::replace(&mut self.buckets, new_buckets);
    let _old_num_slots =
      std::mem::replace(&mut self.num_slots, new_capacity.saturating_sub(1));
    let _old_max_loockups =
      std::mem::replace(&mut self.max_lookups, new_max_lookups);
    let old_num_elements = std::mem::replace(&mut self.num_elements, 0);

    if old_num_elements > 0 {
      for mut entry in old_buckets {
        if entry.has_value() {
          let (key, value) = entry
            .value
            .take()
            .expect("Value existed, but Option was None during resize");
          self.insert_during_resize(key, value);
        }
      }
    }
  }

  #[inline]
  fn insert_during_resize(&mut self, key: K, value: V) {
    let hash = self.hash_key(&key);
    let desired_index =
      self.build_hasher.policy.hash_index(hash, self.num_slots);

    let mut current_index = desired_index;
    let mut distance = 0i8;
    let mut item_to_insert = Some((key, value));

    if self.buckets.is_empty() {
      panic!("empty bucket vector during insertion");
    }

    loop {
      if distance > self.max_lookups {
        panic!("distance > max_loockups");
      }

      if current_index >= self.buckets.len() {
        current_index = 0;
      }

      let entry = &mut self.buckets[current_index];

      if entry.is_empty() {
        entry.value = item_to_insert.take();
        entry.desired_distance = distance;
        self.num_elements += 1;
        return;
      }

      if entry.desired_distance < distance {
        std::mem::swap(&mut item_to_insert, &mut entry.value);
        std::mem::swap(&mut distance, &mut entry.desired_distance);
      }

      distance += 1;
      current_index += 1;
      if current_index == self.buckets.len() {
        current_index = 0;
      }
    }
  }

  #[inline]
  pub fn insert(&mut self, key: K, value: V) -> Option<V> {
    self.reserve(1);

    let mut item_to_insert = Some((key, value));

    'insert_loop: loop {
      if self.buckets.is_empty() {
        self.resize(MIN_LOOKUPS as usize);
        if self.buckets.is_empty() {
          panic!("resize failed");
        }
        continue 'insert_loop;
      }

      let current_key_ref = match &item_to_insert {
        Some((k, _)) => k,
        None => panic!("None unexpectidly in insert"),
      };

      let hash = self.hash_key(current_key_ref);
      let desired_index =
        self.build_hasher.policy.hash_index(hash, self.num_slots);

      let mut current_index = desired_index;
      let mut distance = 0i8;

      loop {
        if distance > self.max_lookups {
          let (k_to_reinsert, v_to_reinsert) = item_to_insert
            .take()
            .expect("item cannot be None for resize");

          self.resize(self.num_slots + 1);

          item_to_insert = Some((k_to_reinsert, v_to_reinsert));
          continue 'insert_loop;
        }

        if current_index >= self.buckets.len() {
          current_index = 0;
        }

        debug_assert!(
          current_index < self.buckets.len(),
          "current_index out of bounds in insert ({}/{})",
          current_index,
          self.buckets.len()
        );

        let entry = &mut self.buckets[current_index];

        if let Some((entry_key, entry_value)) = entry.value.as_mut() {
          if let Some((key_to_compare, _)) = &item_to_insert {
            if key_to_compare == entry_key {
              let (_, new_value) = item_to_insert.take().unwrap();
              let old_val = std::mem::replace(entry_value, new_value);
              return Some(old_val);
            }
          }
        }
        if entry.is_empty() {
          entry.value = item_to_insert.take();
          entry.desired_distance = distance;
          self.num_elements += 1;
          return None;
        }

        if entry.desired_distance < distance {
          std::mem::swap(&mut item_to_insert, &mut entry.value);
          std::mem::swap(&mut distance, &mut entry.desired_distance);
        }

        distance += 1;
        current_index += 1;
        if current_index == self.buckets.len() {
          current_index = 0;
        }
      }
    }
  }

  #[inline]
  pub fn get<Q: ?Sized>(&self, key: &Q) -> Option<&V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    if self.is_empty() || self.buckets.is_empty() {
      return None;
    }

    let desired_index = self.desired_index(key);
    let mut current_index = desired_index;
    let mut distance = 0i8;

    loop {
      if current_index >= self.buckets.len() {
        current_index = 0;
      }

      let entry = &self.buckets[current_index];

      if entry.has_value() && entry.desired_distance < distance {
        return None;
      }

      if let Some((entry_key, entry_value)) = entry.value.as_ref() {
        if self.keys_equal(key, entry_key) {
          return Some(entry_value);
        }
      }

      if entry.is_empty() {
        return None;
      }
      if distance >= self.max_lookups {
        return None;
      }

      distance += 1;
      current_index += 1;
      if current_index == self.buckets.len() {
        current_index = 0;
      }
    }
  }

  #[inline]
  pub fn get_mut<Q: ?Sized>(&mut self, key: &Q) -> Option<&mut V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    if self.is_empty() || self.buckets.is_empty() {
      return None;
    }

    let desired_index = self.desired_index(key);
    let mut current_index = desired_index;
    let mut distance = 0i8;

    loop {
      if current_index >= self.buckets.len() {
        current_index = 0;
      }

      let entry = &self.buckets[current_index];

      if entry.has_value() && entry.desired_distance < distance {
        return None;
      }

      if let Some((entry_key, _)) = &entry.value {
        if self.keys_equal(key, entry_key) {
          return self.buckets[current_index].value.as_mut().map(|(_, v)| v);
        }
      }

      if entry.is_empty() {
        return None;
      }

      if distance >= self.max_lookups {
        return None;
      }

      distance += 1;
      current_index += 1;
      if current_index == self.buckets.len() {
        current_index = 0;
      }
    }
  }

  #[inline]
  pub fn hasher(&self) -> &H {
    &self.build_hasher.build_hasher
  }

  #[inline]
  pub fn policy(&self) -> &P {
    &self.build_hasher.policy
  }

  pub fn iter(&self) -> Iter<'_, K, V> {
    Iter {
      buckets: &self.buckets,
      index: 0,
      items_remaining: self.num_elements,
    }
  }

  #[inline]
  pub fn remove<Q: ?Sized>(&mut self, key: &Q) -> Option<V>
  where
    K: Borrow<Q>,
    Q: Hash + Eq,
  {
    if self.is_empty() || self.buckets.is_empty() {
      return None;
    }

    let desired_idx_of_key_to_remove = self.desired_index(key);
    let mut current_probe_idx = desired_idx_of_key_to_remove;
    let mut distance = 0i8;

    loop {
      if current_probe_idx >= self.buckets.len() {
        current_probe_idx = 0;
      }

      let entry = &self.buckets[current_probe_idx];

      if entry.is_empty() || entry.desired_distance < distance {
        return None;
      }

      if let Some((entry_key, _)) = entry.value.as_ref() {
        if self.keys_equal(key, entry_key) {
          break;
        }
      }

      distance += 1;
      if distance > self.max_lookups {
        return None;
      }
      current_probe_idx += 1;
      if current_probe_idx == self.buckets.len() {
        current_probe_idx = 0;
      }
    }

    let mut hole_idx = current_probe_idx;

    let removed_value = self.buckets[hole_idx].value.take().unwrap().1;
    self.buckets[hole_idx].desired_distance = -1;
    self.num_elements -= 1;

    loop {
      let mut candidate_to_shift_idx = hole_idx + 1;
      if candidate_to_shift_idx == self.buckets.len() {
        candidate_to_shift_idx = 0;
      }

      if self.buckets[candidate_to_shift_idx].is_empty()
        || self.buckets[candidate_to_shift_idx].desired_distance == 0
      {
        break;
      }

      let value_to_move = self.buckets[candidate_to_shift_idx].value.take();
      let dd_of_moved_item =
        self.buckets[candidate_to_shift_idx].desired_distance;

      self.buckets[hole_idx].value = value_to_move;
      self.buckets[hole_idx].desired_distance = dd_of_moved_item - 1;

      self.buckets[candidate_to_shift_idx].value = None;
      self.buckets[candidate_to_shift_idx].desired_distance = -1;

      hole_idx = candidate_to_shift_idx;
    }

    Some(removed_value)
  }
}

pub struct Iter<'a, K, V> {
  buckets: &'a [HashEntry<(K, V)>],
  index: usize,
  items_remaining: usize,
}

impl<'a, K, V> Iterator for Iter<'a, K, V> {
  type Item = (&'a K, &'a V);

  fn next(&mut self) -> Option<Self::Item> {
    if self.items_remaining == 0 {
      return None;
    }

    while self.index < self.buckets.len() {
      let entry = &self.buckets[self.index];
      self.index += 1;

      if entry.has_value() {
        if let Some((key, value)) = &entry.value {
          self.items_remaining -= 1;
          return Some((key, value));
        }
      }
    }
    None
  }

  fn size_hint(&self) -> (usize, Option<usize>) {
    (self.items_remaining, Some(self.items_remaining))
  }
}

impl<'a, K, V, H, P> IntoIterator for &'a HashTable<K, V, H, P>
where
  K: Hash + Eq,
  H: BuildHasher + Clone,
  P: HashPolicy + Default + Clone,
{
  type Item = (&'a K, &'a V);
  type IntoIter = Iter<'a, K, V>;

  fn into_iter(self) -> Self::IntoIter {
    self.iter()
  }
}
