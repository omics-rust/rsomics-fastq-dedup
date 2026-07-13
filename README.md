# rsomics-fastq-dedup

Deduplicate single-end FASTQ reads by sequence — a fast lossy k-mer hash-bin
(fastp `-D` style) or an exact full-sequence hash (`seqkit rmdup -s` style).

## Install

```
cargo install rsomics-fastq-dedup
```

## Usage

```
# default: fast hash-bin mode, gzip in and out
rsomics-fastq-dedup -i in.fq.gz -o out.fq.gz

# exact full-sequence dedup
rsomics-fastq-dedup -i in.fq -o out.fq --mode full

# larger k-mer, offset fingerprint window from the 3' end
rsomics-fastq-dedup -i in.fq -o out.fq -k 20 --tail-offset 5
```

- `-i, --in1` — input FASTQ (gz/bz2/xz/zst autodetected).
- `-o, --out1` — output FASTQ (`.gz` writes gzip).
- `--mode` — `bin` (fast, lossy; default) or `full` (exact).
- `-k, --kmer-size` — k-mer length for bin mode (default `12`).
- `--tail-offset` — offset from the 3' end for the fingerprint window
  (default `0`).

## Origin

Independent Rust implementation consolidating two dedup operations. Exact
full-sequence dedup (`--mode full`) is checked to keep the same read set as
`seqkit rmdup -s` (black-box plus a committed golden); the k-mer hash-bin mode
(`--mode bin`) follows fastp's `-D` duplication approach. Single-end only.

License: MIT OR Apache-2.0.
Upstream credit: [seqkit](https://github.com/shenwei356/seqkit) (MIT) for
full-sequence dedup; [fastp](https://github.com/OpenGene/fastp) (MIT), Chen et
al. 2018, doi:10.1093/bioinformatics/bty560, for the hash-bin approach.
