mod args;
use args::GenomeArgs;
use std::error::Error;
use clap::Parser;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};

/*
 *Author Gaurav Sablok
 *Universitat Potsdam
 *Date 2024-12-2

 rust-samtools: allows for the filtering of the alignments within the
 given range of the genomic coordinate and also according to the name of the
 reference. This part allows for the filtering of the samtools for a given region
 or a range.

 * */

fn main() {

    let args = GenomeArgs::parse();
    let samtools_start = samtools_start(&args.alignment_arg, args.genome_start.unwrap()).unwrap();
    println!("The selected region has been written: {}", samtools_start);
    let samtools_range = samtools_range(&args.alignment_arg, args.genome_start.unwrap(), args.genome_end.unwrap()).unwrap();
    println!("The selected region has been written: {}", samtools_range);
}

fn samtools_start(pathsam: &str, start:usize) -> Result<String, Box<dyn Error>> {

  #[derive(Debug, Clone, PartialEq, PartialOrd)]
  struct Limit {
    line: String
  }

  let fileopen = File::open(pathsam).expect("file not present");
  let fileread = BufReader::new(fileopen);
  let mut limit_lines = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not found");
    if ! line.starts_with("@") {
      let iden = line;
      limit_lines.push(iden);
    }
  }
  let mut limit:Vec<Limit> = Vec::new();
  for i in limit_lines.iter(){
    let mutable = i.split("\t").filter(|x| *x != "").collect::<Vec<_>>();
    if mutable.len() == 0 {
      continue
    }
    if mutable[3].parse::<usize>().unwrap() == start {
      limit.push(Limit { line: mutable.join(" ").to_string()});
    }
  }
  let mut samtools_start = File::create("sorted_selected-start.sam").expect("file not present");
  for i in limit.iter_mut(){
    write!(samtools_start, "{}\n", i.line).expect("not able to write th line");
  }
    Ok("The files have been written and the summary is given below".to_string())
}

fn samtools_range(pathsam: &str, start:usize, end:usize) -> Result<String, Box< dyn Error>> {


  #[derive(Debug, Clone, PartialEq, PartialOrd)]
  struct UpperLimit {
    line: String
  }


  let fileopen = File::open(pathsam).expect("file not present");
  let fileread = BufReader::new(fileopen);
  let mut upper_lines:Vec<UpperLimit> = Vec::new();

  let mut lines = Vec::new();
  for i in fileread.lines(){
    let line = i.expect("line not found");
    if ! line.starts_with("@") {
      let iden = line;
      lines.push(iden);
    }
  }

  for i in lines.iter(){
    let mutable = i.split("\t").filter(|x| *x != "").collect::<Vec<_>>();
    if mutable.len() == 0 {
      continue
    }
    if mutable[3].parse::<usize>().unwrap() >= start && mutable[3].parse::<usize>().unwrap() <= end {
      upper_lines.push(UpperLimit { line: mutable.join(" ").to_string()});
    }
  }

   let mut commonjoin:Vec<_> = Vec::new();

   for i in upper_lines.iter_mut(){
     commonjoin.push(i.line.clone());
  }

  let mut samtools_range = File::create("samtools-range.sam").expect("file not found");
  for i in commonjoin.iter(){
    write!(samtools_range, "{}\n", i).expect("line not found");
  }

  Ok("The files have been written".to_string())
}
