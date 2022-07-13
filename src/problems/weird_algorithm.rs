#[allow(unused_imports)]
use std::io::{BufWriter, stdin, stdout, Write};

#[derive(Default)]
struct Scanner {
    buffer: Vec<String>
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(token) = self.buffer.pop() {
                return token.parse().ok().expect("Failed parse");
            }
            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed read");
            self.buffer = input.split_whitespace().rev().map(String::from).collect();
        }
    }
}


pub fn main() {
    let mut scan = Scanner::default();
    let out = &mut BufWriter::new(stdout());
    let mut n = scan.next::<i128>();
    let mut seq: Vec<String> = Vec::new();
    seq.push(n.to_string());
    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (n * 3) + 1;
        }
        seq.push(n.to_string())
    }
    out.write(seq.join(" ").as_bytes()).unwrap();
}
