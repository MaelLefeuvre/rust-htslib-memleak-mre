
use rust_htslib::faidx;
use rand::{thread_rng, Rng};

fn main() {
    let fasta     = std::env::args().nth(1).expect("No reference provided");
    let reference = faidx::Reader::from_path(&fasta).unwrap();

    let mut rng = thread_rng();
    loop {
        // ---- Fetch a random chromosome, position and read length within a reference genome 
        let chr   = rng.gen_range(1..=22u32).to_string();
        let len   = rng.gen_range(10..50usize);
        let start = rng.gen_range(0..100usize - len);
        let refseq = reference.fetch_seq(chr, start, start + len).unwrap();
        println!("{}", unsafe{ std::str::from_utf8_unchecked(refseq)});
    }
}
