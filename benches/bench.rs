use criterion::{Criterion, criterion_group, criterion_main};
use std::hint::black_box;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;

fn bench_fastq_dedup(c: &mut Criterion) {
    let bin = env!("CARGO_BIN_EXE_rsomics-fastq-dedup");
    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fq = manifest.join("tests/golden/dup.fastq");
    c.bench_function("rsomics-fastq-dedup golden", |b| {
        b.iter(|| {
            let out_file = NamedTempFile::new().unwrap();
            let out = Command::new(black_box(bin))
                .arg("-i")
                .arg(fq.to_str().unwrap())
                .arg("-o")
                .arg(out_file.path())
                .output()
                .unwrap();
            assert!(out.status.success());
        });
    });
}

criterion_group!(benches, bench_fastq_dedup);
criterion_main!(benches);
