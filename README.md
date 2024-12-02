# rust-samtools-filter
 - rust-samtools: allows for the filtering of the alignments within the given range of the genomic coordinate and also according to   the name of the reference. This part allows for the filtering of the samtools for a given region  or a range.
 - some options were missing in samtools and no need to write any other scripts. A single compiled binary. 
 - general note: Incase of Golang and RUST, please see the last commit message and if it says compiled binary then it is completed or else still in development version.
 
 ```
 cargo build
 ``
 
 ```
 λ gauravsablok rust-samtools-filter → λ git main* → ./target/debug/rust-samtools-filter -h
Usage: rust-samtools-filter <ALIGNMENT_ARG> [GENOME_START] [GENOME_END]

Arguments:
  <ALIGNMENT_ARG>  please provide the path to the alignment file
  [GENOME_START]   please provide the genome capture region start
  [GENOME_END]     please provide the genome capture region end

Options:
  -h, --help     Print help
  -V, --version  Print version
 ```
 - for a specific coordinate 
 ```
 ./target/debug/rust-samtools-filter ./sample-files/alignreads-metagenomics.sam 21001 
 ```
 - for a range of coordinate
 ```
 ./target/debug/rust-samtools-filter ./sample-files/alignreads-metagenomics.sam 21001 613960
 ```

Gaurav Sablok

- How to cite
```
  @software{
  author = {Gaurav Sablok},
  title = {{Genome-view:Developing samtools components in RUST for Genome Analysis}},
  url = {https://github.com/applicativesystem/rust-samtools-filter},
  version = {0.1.0},
  year = {2024}
}
```
