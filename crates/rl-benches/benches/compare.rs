use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_benches::*;

/// Runs the same source through the interpreter and the VM under one group
/// so Criterion plots both curves together (the VM is measured on its
/// pre-compiled chunk - compilation happens outside the timed region).
fn bench_compare(c: &mut Criterion) {
    let mut group = c.benchmark_group("interp-vs-vm");
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(100);

    for (name, src) in BASE_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("interpreter", *name), src, |b, s| {
            b.iter(|| interp_evaluate_only(black_box(s)))
        });

        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("vm", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }

    for (name, src, _expected) in WORKLOAD_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("interpreter", *name), src, |b, s| {
            b.iter(|| interp_evaluate_only(black_box(s)))
        });

        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("vm", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }

    group.finish();
}

criterion_group!(benches_compare, bench_compare);
criterion_main!(benches_compare);
