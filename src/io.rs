use std::io::{self, Write};

pub fn get_n_parts(n: usize) -> Vec<String> {
    let s = get_input();
    s.splitn(n, ' ').map(|s| s.to_owned()).collect()
}

pub fn get_input() -> String {
    loop {
        io::stdout().flush().unwrap();
        let mut s = String::new();
        io::stdin()
            .read_line(&mut s)
            .expect("failed to read user input");
        let newline_chars = ['\n', '\r'];
        s = s.chars().filter(|b| !newline_chars.contains(b)).collect();
        if !s.is_empty() {
            return s;
        }
    }
}
