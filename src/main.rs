use regex::Regex;
use std::collections::HashMap;
use std::io;

fn main() {
    loop {
        println!("Please provide a kmer (q to quit):");

        let mut kmer = String::new();

        io::stdin()
            .read_line(&mut kmer)
            .expect("Failed to read line");
        let kmer = kmer.trim().to_uppercase();

        // Check if the user wants to quit
        if kmer == "Q" {
            break;
        }

        // Check if the kmer has an even number of nucleotides
        // Important for determining the lexigraphical order
        // (no discrepancies possible)
        if kmer.len() % 2 == 0 {
            println!("The kmer should have an odd number of nucleotides");
        }

        // Check if the kmer only contains standard nucleotides
        let re = Regex::new(r"^(?i)[ACGT]+$").unwrap();
        if !re.is_match(&kmer) {
            println!("The kmer should only contain A, C, G, T");
            continue;
        }

        let complement = complement_dna(&kmer);
        let reverse_complement = complement_dna(&kmer).chars().rev().collect::<String>();

        let canonical = if kmer < reverse_complement {
            &kmer
        } else {
            &reverse_complement
        };

        println!("K-mer:      >{}", kmer);
        println!("Complement:  {}<", complement);
        println!("Rev.comp:   >{}", reverse_complement);
        println!("Canonical:  >{}", canonical);
    }
}

// Returns the complementary sequence of a kmer
fn complement_dna(kmer: &str) -> String {
    let mut c_table: HashMap<char, char> = HashMap::new();
    c_table.insert('A', 'T');
    c_table.insert('C', 'G');
    c_table.insert('G', 'C');
    c_table.insert('T', 'A');

    let mut complement = String::with_capacity(kmer.len());
    for c in kmer.chars() {
        complement.push(c_table[&c]);
    }
    complement
}
