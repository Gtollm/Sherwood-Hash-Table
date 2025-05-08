use std::collections::HashMap;

use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use sherwood_table::HashTable;

fn bench_insertion(c: &mut Criterion) {
  let mut group = c.benchmark_group("insertion");

  for size_usize in [10usize, 100, 1_000, 10_000].iter() {
    let size = *size_usize;
    group.bench_with_input(
      BenchmarkId::new("sherwood_table", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut table: HashTable<i32, i32> = HashTable::with_capacity(s);
          for i_usize in 0..s {
            let i = i_usize as i32;
            table.insert(black_box(i), black_box(i * 2));
          }
          table
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut map: HashMap<i32, i32> = HashMap::with_capacity(s);
          for i_usize in 0..s {
            let i = i_usize as i32;
            map.insert(black_box(i), black_box(i * 2));
          }
          map
        });
      },
    );
  }

  group.finish();
}

fn bench_lookup(c: &mut Criterion) {
  let mut group = c.benchmark_group("lookup");

  for size_usize in [100usize, 1_000, 10_000].iter() {
    let size = *size_usize;
    let mut sherwood_table: HashTable<i32, i32> =
      HashTable::with_capacity(size);
    let mut std_hashmap: HashMap<i32, i32> = HashMap::with_capacity(size);

    for i_usize in 0..size {
      let i = i_usize as i32;
      sherwood_table.insert(i, i * 2);
      std_hashmap.insert(i, i * 2);
    }

    group.bench_with_input(
      BenchmarkId::new("sherwood_table_hits", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut sum = 0;
          for i_usize in 0..s {
            let i = i_usize as i32;
            if let Some(&val) = sherwood_table.get(&black_box(i)) {
              sum += val;
            }
          }
          sum
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap_hits", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut sum = 0;
          for i_usize in 0..s {
            let i = i_usize as i32;
            if let Some(&val) = std_hashmap.get(&black_box(i)) {
              sum += val;
            }
          }
          sum
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("sherwood_table_misses", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut count = 0;
          for i_usize in s..(s * 2) {
            let i = i_usize as i32;
            if sherwood_table.get(&black_box(i)).is_none() {
              count += 1;
            }
          }
          count
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap_misses", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut count = 0;
          for i_usize in s..(s * 2) {
            let i = i_usize as i32;
            if std_hashmap.get(&black_box(i)).is_none() {
              count += 1;
            }
          }
          count
        });
      },
    );
  }

  group.finish();
}

fn bench_string_keys(c: &mut Criterion) {
  let mut group = c.benchmark_group("string_keys");

  for size_usize in [10usize, 100, 1_000].iter() {
    let size = *size_usize;
    group.bench_with_input(
      BenchmarkId::new("sherwood_table", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut table: HashTable<String, i32> = HashTable::with_capacity(s);
          for i in 0..s {
            table.insert(format!("key_{}", i), i as i32);
          }

          let mut sum = 0;
          for i in 0..s {
            if let Some(&val) = table.get(&format!("key_{}", i)) {
              sum += val;
            }
          }
          sum
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap", size),
      &size,
      |b, &s| {
        b.iter(|| {
          let mut map: HashMap<String, i32> = HashMap::with_capacity(s);
          for i in 0..s {
            map.insert(format!("key_{}", i), i as i32);
          }

          let mut sum = 0;
          for i in 0..s {
            if let Some(&val) = map.get(&format!("key_{}", i)) {
              sum += val;
            }
          }
          sum
        });
      },
    );
  }

  group.finish();
}

fn bench_removal(c: &mut Criterion) {
  let mut group = c.benchmark_group("removal");

  for size_usize in [100usize, 1_000, 10_000].iter() {
    let size = *size_usize;
    group.bench_with_input(
      BenchmarkId::new("sherwood_table", size),
      &size,
      |b, &s| {
        b.iter_with_setup(
          || {
            let mut table: HashTable<i32, i32> = HashTable::with_capacity(s);
            for i_usize in 0..s {
              let i = i_usize as i32;
              table.insert(i, i * 2);
            }
            table
          },
          |mut table| {
            let mut sum = 0;
            for i_usize in 0..(s / 2) {
              let i = i_usize as i32;
              if let Some(val) = table.remove(&black_box(i)) {
                sum += val;
              }
            }
            sum
          },
        );
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap", size),
      &size,
      |b, &s| {
        b.iter_with_setup(
          || {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(s);
            for i_usize in 0..s {
              let i = i_usize as i32;
              map.insert(i, i * 2);
            }
            map
          },
          |mut map| {
            let mut sum = 0;
            for i_usize in 0..(s / 2) {
              let i = i_usize as i32;
              if let Some(val) = map.remove(&black_box(i)) {
                sum += val;
              }
            }
            sum
          },
        );
      },
    );
  }

  group.finish();
}

fn bench_iteration(c: &mut Criterion) {
  let mut group = c.benchmark_group("iteration");

  for size_usize in [100usize, 1_000, 10_000].iter() {
    let size = *size_usize;
    let mut sherwood_table: HashTable<i32, i32> =
      HashTable::with_capacity(size);
    let mut std_hashmap: HashMap<i32, i32> = HashMap::with_capacity(size);

    for i_usize in 0..size {
      let i = i_usize as i32;
      sherwood_table.insert(i, i * 2);
      std_hashmap.insert(i, i * 2);
    }

    group.bench_with_input(
      BenchmarkId::new("sherwood_table", size),
      &size,
      |b, &_s| {
        b.iter(|| {
          let mut sum = 0;
          for (_, &val) in &sherwood_table {
            sum += black_box(val);
          }
          sum
        });
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap", size),
      &size,
      |b, &_s| {
        b.iter(|| {
          let mut sum = 0;
          for (_, &val) in &std_hashmap {
            sum += black_box(val);
          }
          sum
        });
      },
    );
  }

  group.finish();
}

fn bench_mixed_operations(c: &mut Criterion) {
  let mut group = c.benchmark_group("mixed_operations");

  for size_usize in [100usize, 1_000].iter() {
    let size = *size_usize;
    group.bench_with_input(
      BenchmarkId::new("sherwood_table", size),
      &size,
      |b, &s| {
        b.iter_with_setup(
          || {
            let mut table: HashTable<i32, i32> = HashTable::with_capacity(s);
            for i_usize in 0..s {
              let i = i_usize as i32;
              table.insert(i, i * 2);
            }
            table
          },
          |mut table| {
            let mut sum = 0;

            for i_usize in 0..s {
              let i = i_usize as i32;
              if let Some(&val) = table.get(&i) {
                sum += val;
              }
            }

            for i_usize in 0..(s / 4) {
              let i = i_usize as i32;
              if let Some(val) = table.remove(&i) {
                sum += val;
              }
            }

            for i_usize in s..(s + s / 4) {
              let i = i_usize as i32;
              table.insert(i, i * 3);
            }

            for i_usize in (s / 4)..s {
              let i = i_usize as i32;
              if let Some(&val) = table.get(&i) {
                sum += val;
              }
            }

            sum
          },
        );
      },
    );

    group.bench_with_input(
      BenchmarkId::new("std_hashmap", size),
      &size,
      |b, &s| {
        b.iter_with_setup(
          || {
            let mut map: HashMap<i32, i32> = HashMap::with_capacity(s);
            for i_usize in 0..s {
              let i = i_usize as i32;
              map.insert(i, i * 2);
            }
            map
          },
          |mut map| {
            let mut sum = 0;

            for i_usize in 0..s {
              let i = i_usize as i32;
              if let Some(&val) = map.get(&i) {
                sum += val;
              }
            }

            for i_usize in 0..(s / 4) {
              let i = i_usize as i32;
              if let Some(val) = map.remove(&i) {
                sum += val;
              }
            }

            for i_usize in s..(s + s / 4) {
              let i = i_usize as i32;
              map.insert(i, i * 3);
            }

            for i_usize in (s / 4)..s {
              let i = i_usize as i32;
              if let Some(&val) = map.get(&i) {
                sum += val;
              }
            }

            sum
          },
        );
      },
    );
  }

  group.finish();
}

criterion_group!(
  benches,
  bench_insertion,
  bench_lookup,
  bench_string_keys,
  bench_removal,
  bench_iteration,
  bench_mixed_operations
);
criterion_main!(benches);

