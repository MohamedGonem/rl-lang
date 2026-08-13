use criterion::{BenchmarkId, Criterion, black_box, criterion_group, criterion_main};
use rl_benches::*;

/// The resolve stage only (lex + parse + resolve). Isolation lets a
/// regression be pinned to this stage rather than the whole pipeline.
fn bench_resolve(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/resolve");
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(100);

    for (name, src) in BASE_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("base", *name), src, |b, s| {
            b.iter(|| resolve_only(black_box(s)))
        });
    }
    for (name, src, _) in WORKLOAD_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("workload", *name), src, |b, s| {
            b.iter(|| resolve_only(black_box(s)))
        });
    }

    group.finish();
}

/// VM compile only (resolve once outside the timer, then time just the
/// bytecode compiler).
fn bench_vm_compile(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/vm_compile");
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
    for (name, src, _) in WORKLOAD_PROGRAMS {
        group.bench_with_input(BenchmarkId::new("workload", *name), src, |b, s| {
            b.iter(|| {
                let program = parse_and_resolve(black_box(s));
                let _ = compile_resolved(&program);
            })
        });
    }

    group.finish();
}

/// VM run only (compile once outside the timer, time pure execution on a
/// fresh VM per iteration).
fn bench_vm_run(c: &mut Criterion) {
    let mut group = c.benchmark_group("pipeline/vm_run");
    group.measurement_time(std::time::Duration::from_secs(5));
    group.sample_size(100);

    for (name, src) in BASE_PROGRAMS {
        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("base", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }
    for (name, src, _) in WORKLOAD_PROGRAMS {
        let program = parse_and_resolve(src);
        let chunk = compile_resolved(&program);
        group.bench_with_input(BenchmarkId::new("workload", *name), &chunk, |b, chunk| {
            b.iter(|| run_chunk(black_box(chunk)))
        });
    }

    group.finish();
}

criterion_group!(
    benches_pipeline,
    bench_resolve,
    bench_vm_compile,
    bench_vm_run,
);
criterion_main!(benches_pipeline);
