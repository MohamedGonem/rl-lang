use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_benches::*;

fn bench_vm_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("vm/compile");
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(100);

    for (name, src) in BASE_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("base", *name), src, |b, s| {
            b.iter(|| {
                let program = parse_and_resolve(black_box(s));
                let _ = compile_resolved(&program);
            })
        });
    }

    for (name, src, _expected) in WORKLOAD_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("workload", *name), src, |b, s| {
            b.iter(|| {
                let program = parse_and_resolve(black_box(s));
                let _ = compile_resolved(&program);
            })
        });
    }

    group.finish();
}

fn bench_vm_run(c: &mut Criterion) {
    let mut group = c.benchmark_group("vm/run");
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(100);

    // Compile once outside the timed region so we measure pure execution.
    for (name, src) in BASE_PROGRAMS {
        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("base", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }

    for (name, src, _expected) in WORKLOAD_PROGRAMS {
        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("workload", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }

    group.finish();
}

criterion_group!(benches_vm, bench_vm_compile, bench_vm_run);
criterion_main!(benches_vm);
