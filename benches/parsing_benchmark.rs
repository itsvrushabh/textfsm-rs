use criterion::{black_box, criterion_group, criterion_main, Criterion};
use textfsm_rs::TextFSM;

fn benchmark_parsing(c: &mut Criterion) {
    let template_path = "tests/data/cli/cisco_version_template";
    let data_path = "tests/data/cli/cisco_version_example";

    // Bench 1: Initialization + Parsing (Full cycle)
    c.bench_function("parse cisco version (full)", |b| {
        b.iter(|| {
            let mut fsm = TextFSM::from_file(black_box(template_path)).unwrap();
            fsm.parse_file(black_box(data_path), None).unwrap();
        })
    });
}

criterion_group!(benches, benchmark_parsing);
criterion_main!(benches);
