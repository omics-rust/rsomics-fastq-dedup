use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

fn ours() -> PathBuf {
    PathBuf::from(env!("CARGO_BIN_EXE_rsomics-fastq-dedup"))
}

fn fixture() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/golden/dup.fastq")
}

fn seqkit_available() -> bool {
    Command::new("seqkit")
        .arg("version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
}

#[test]
fn help_works() {
    let out = Command::new(ours())
        .args(["--help"])
        .output()
        .expect("spawn");
    assert!(out.status.success());
}

/// Sorted kept sequences (the dedup result, order-independent).
fn kept_seqs(fastq: &[u8]) -> Vec<String> {
    let text = String::from_utf8_lossy(fastq);
    let lines: Vec<&str> = text.lines().collect();
    let mut seqs: Vec<String> = lines
        .chunks(4)
        .filter(|c| c.len() == 4)
        .map(|c| c[1].to_owned())
        .collect();
    seqs.sort();
    seqs
}

// `--mode full` (exact full-sequence dedup) must keep the same sequence set as
// `seqkit rmdup -s` (compare sorted kept sequences; first-occurrence order is
// the same but we compare sets to be robust).
#[test]
fn full_dedup_matches_seqkit_rmdup() {
    if !seqkit_available() {
        eprintln!("skipping: seqkit not found");
        return;
    }
    let dir = std::env::temp_dir().join("rsomics-fastq-dedup-compat");
    let _ = std::fs::create_dir_all(&dir);
    let ours_out = dir.join("ours.fq");
    let theirs_out = dir.join("seqkit.fq");

    assert!(
        Command::new(ours())
            .args(["-i"])
            .arg(fixture())
            .arg("-o")
            .arg(&ours_out)
            .args(["--mode", "full"])
            .status()
            .unwrap()
            .success()
    );
    assert!(
        Command::new("seqkit")
            .args(["rmdup", "-s"])
            .arg(fixture())
            .arg("-o")
            .arg(&theirs_out)
            .status()
            .unwrap()
            .success()
    );

    let ours_seqs = kept_seqs(&std::fs::read(&ours_out).unwrap());
    let theirs_seqs = kept_seqs(&std::fs::read(&theirs_out).unwrap());
    assert_eq!(ours_seqs, theirs_seqs);
}
