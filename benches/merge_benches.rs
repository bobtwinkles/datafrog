#[macro_use]
extern crate criterion;

extern crate rand;

extern crate datafrog;

use criterion::{Criterion, ParameterizedBenchmark};
use datafrog::Relation;

#[derive(Debug,Copy,Clone)]
enum MergeStrategy {
    Individual,
    MultiMerge,
}

fn generate_relations(num_relations: u64, relation_len: u64) -> Vec<Relation<u64>> {
    use rand::random;
    let mut out_relations = Vec::new();

    for _ in 0 .. num_relations {
        out_relations.push(Relation::from(std::iter::empty()));
    }

    for i in 0 .. (num_relations * relation_len) {
        out_relations[random::<usize>() % (num_relations as usize)].elements.push(i);
    }

    out_relations
}

fn do_bench_merge(c: &mut Criterion, num_relations: u64, relation_len: u64) {
    let name = format!("merge_{:03}x{:03}", num_relations, relation_len);
    c.bench("merge", ParameterizedBenchmark::new(
        name.clone(),
        move |b, strategy| {
            match strategy {
                MergeStrategy::Individual => {
                    b.iter_with_setup(
                        move || {
                            generate_relations(num_relations, relation_len)
                        },
                        |data| {
                            let mut iter = data.into_iter();
                            let mut r = iter.next().unwrap();
                            while let Some(rp) = iter.next() {
                                r = r.merge(rp)
                            }
                            r
                        }
                    )
                }
                MergeStrategy::MultiMerge => {
                    b.iter_with_setup(
                        move || {
                            generate_relations(num_relations, relation_len)
                        },
                        |data| {
                            Relation::multi_merge(data)
                        }
                    )
                }
            }
        },
        [MergeStrategy::Individual, MergeStrategy::MultiMerge].iter()
    ));
}

fn bench_merge(c: &mut Criterion) {
    do_bench_merge(c, 3, 100);
}

criterion_group!(benches, bench_merge);
criterion_main!(benches);
