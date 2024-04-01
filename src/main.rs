mod entropy;
mod pool;

use std::io;
use std::io::Write;
use std::io::BufRead;

use zeroize::Zeroize;

fn main() {
    let stdin = io::stdin();
    let input = &mut String::new();

    loop {
        input.zeroize();
        print!("ascii input: ");
        let _ = std::io::stdout().flush().unwrap();
        if stdin.lock().read_line(input).is_ok_and(|nread| nread > 1) {
            let entropy = entropy::entropy(&input.trim());
            println!("{:.2}", entropy);
        } else {
            break;
        }
    }
}
