use sorting_algorithm::lizi_sort;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::seq::SliceRandom;
use rand::thread_rng;

fn bubble_sort(mut list: Vec<u32>) -> Vec<u32> {
    let n = list.len();
    for i in 0..n {
        for j in 0..n - 1 - i {
            if list[j] > list[j + 1] {
                list.swap(j, j + 1);
            }
        }
    }
    list
}

fn benchmark_sort(c: &mut Criterion) {
    c.bench_function("my sort", |b| {
        b.iter(|| {
            let mut list: Vec<u32> = (1..=20000).collect();
            list.shuffle(&mut thread_rng());
            black_box(lizi_sort(list));
        });
    });

    c.bench_function("std sort", |b| {
        b.iter(|| {
            let mut list: Vec<u32> = (1..=20000).collect();
            list.shuffle(&mut thread_rng());
            black_box(bubble_sort(list));
        });
    });
}

criterion_group!(benches, benchmark_sort);
criterion_main!(benches);