fn main() {}

fn _dna(seq: &str) -> String {
    let mut counts = (0, 0, 0, 0);

    seq.chars().for_each(|c| match c {
        'A' => counts.0 += 1,
        'C' => counts.1 += 1,
        'G' => counts.2 += 1,
        'T' => counts.3 += 1,
        _ => {}
    });
    format!("{} {} {} {}", counts.0, counts.1, counts.2, counts.3)
}

fn _rna(seq: &str) -> String {
    seq.chars()
        .map(|c| match c {
            'T' => 'U',
            _ => c,
        })
        .collect()
}

fn _revc(seq: &str) -> String {
    seq.chars()
        .rev()
        .map(|c| match c {
            'A' => 'T',
            'C' => 'G',
            'G' => 'C',
            'T' => 'A',
            _ => c,
        })
        .collect()
}

fn _fib(n: i64, k: i64) -> i64 {
    let mut a = 1;
    let mut b = 1;
    let mut s = 0;

    for _i in 1..n - 1 {
        s = a * k + b;
        a = b;
        b = s;
    }
    s
}

fn _gc(fasta_file: &str) -> String {
    struct Fasta {
        name: String,
        seq: String,
    }

    fn quickparse_fasta(fasta_file: &str) -> Vec<Fasta> {
        let mut fastas = Vec::new();

        let mut name = String::new();
        let mut seq = String::new();

        for line in fasta_file.lines() {
            if line.starts_with(">") {
                if !name.is_empty() {
                    fastas.push(Fasta {
                        name: name.clone(),
                        seq: seq.clone(),
                    });
                }
                name = line[1..].to_string();
                seq = String::new();
            } else {
                seq.push_str(line);
            }
        }

        if !name.is_empty() {
            fastas.push(Fasta {
                name: name.clone(),
                seq: seq.clone(),
            });
        }

        fastas
    }

    let file_contents = std::fs::read_to_string(fasta_file).unwrap();
    let fastas = quickparse_fasta(&file_contents);

    let mut gc_content = 0.0;
    let mut name = String::new();

    for fasta in &fastas {
        let gc_count = fasta.seq.chars().filter(|c| c == &'G' || c == &'C').count() as f64;
        let gc_ratio = gc_count / fasta.seq.len() as f64;
        if gc_ratio > gc_content {
            gc_content = gc_ratio;
            name = fasta.name.clone();
        }
    }

    format!("{}\n{}", name, gc_content * 100.0)
}

fn _hamm(seq_a: &str, seq_b: &str) -> i32 {
    let mut counter = 0;
    seq_a.chars().zip(seq_b.chars()).for_each(|(a, b)| {
        if a == b {
            counter += 1;
        }
    });
    counter
}

fn _iprb(k: i32, m: i32, n: i32) -> f64 {
    let k = k as f64;
    let m = m as f64;
    let n = n as f64;
    let t = k + m + n;
    let mut p = k / t;
    p += (n / t) * (k / (t - 1.0));
    p += (n / t) * (m / (t - 1.0)) * 0.5;
    p += (m / t) * (k / (t - 1.0));
    p += (m / t) * (m - 1.0) / (t - 1.0) * 0.75;
    p += (m / t) * (n / (t - 1.0)) * 0.5;
    p
}

fn _prot(rna_seq: &str) -> String {
    let rna_seq = rna_seq.to_uppercase().trim().to_string();
    let mut codons = std::collections::HashMap::new();
    {
        codons.insert("UUU", "F");
        codons.insert("CUU", "L");
        codons.insert("AUU", "I");
        codons.insert("GUU", "V");
        codons.insert("UUC", "F");
        codons.insert("CUC", "L");
        codons.insert("AUC", "I");
        codons.insert("GUC", "V");
        codons.insert("UUA", "L");
        codons.insert("CUA", "L");
        codons.insert("AUA", "I");
        codons.insert("GUA", "V");
        codons.insert("UUG", "L");
        codons.insert("CUG", "L");
        codons.insert("AUG", "M");
        codons.insert("GUG", "V");
        codons.insert("UCU", "S");
        codons.insert("CCU", "P");
        codons.insert("ACU", "T");
        codons.insert("GCU", "A");
        codons.insert("UCC", "S");
        codons.insert("CCC", "P");
        codons.insert("ACC", "T");
        codons.insert("GCC", "A");
        codons.insert("UCA", "S");
        codons.insert("CCA", "P");
        codons.insert("ACA", "T");
        codons.insert("GCA", "A");
        codons.insert("UCG", "S");
        codons.insert("CCG", "P");
        codons.insert("ACG", "T");
        codons.insert("GCG", "A");
        codons.insert("UAU", "Y");
        codons.insert("CAU", "H");
        codons.insert("AAU", "N");
        codons.insert("GAU", "D");
        codons.insert("UAC", "Y");
        codons.insert("CAC", "H");
        codons.insert("AAC", "N");
        codons.insert("GAC", "D");
        codons.insert("UAA", "Stop");
        codons.insert("CAA", "Q");
        codons.insert("AAA", "K");
        codons.insert("GAA", "E");
        codons.insert("UAG", "Stop");
        codons.insert("CAG", "Q");
        codons.insert("AAG", "K");
        codons.insert("GAG", "E");
        codons.insert("UGU", "C");
        codons.insert("CGU", "R");
        codons.insert("AGU", "S");
        codons.insert("GGU", "G");
        codons.insert("UGC", "C");
        codons.insert("CGC", "R");
        codons.insert("AGC", "S");
        codons.insert("GGC", "G");
        codons.insert("UGA", "Stop");
        codons.insert("CGA", "R");
        codons.insert("AGA", "R");
        codons.insert("GGA", "G");
        codons.insert("UGG", "W");
        codons.insert("CGG", "R");
        codons.insert("AGG", "R");
        codons.insert("GGG", "G");
    }

    let mut prot_seq = String::new();

    for i in (0..rna_seq.len()).step_by(3) {
        let codon = &rna_seq[i..i + 3];
        if let Some(amino_acid) = codons.get(codon) {
            if *amino_acid == "Stop" {
                break;
            }
            prot_seq.push_str(amino_acid);
        }
    }

    prot_seq
}
