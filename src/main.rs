use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;

fn file_read() -> Vec<String> {
    let file_handle = File::open("./words_alpha.txt").unwrap();
    let file_buffer = BufReader::new(file_handle);
    file_buffer.lines()
        .map(|l| l.unwrap())
        .filter(|l| l.chars().count() >= 5 && l.chars().count() % 2 != 0) // Not size 1, odd count.
        .filter(|l| l.matches(l.chars().nth(0).unwrap()).count() != l.chars().count()) // Different letters.
        .filter(|l| l == &l.chars().rev().collect::<String>()) // Compare to reverse.
        .collect()
}

fn main() {
    // The base English words, and then the words >= 4 letters.
    let dict_base = file_read();
    let mut file_handle = File::create("./words_out.txt").unwrap();
    for itr_i in & dict_base {
        write!(file_handle, "{}\n", itr_i);
    }
}