use std::io;

fn main() {
    let mut s = String::new();
    let mut pig_s = String::new();

    io::stdin()
        .read_line(&mut s)
        .expect("Failed to read line.");
    for word in s.split_whitespace() {
        let mut pig_word = word.to_string();
        match word.chars().next() {
            Some(c) => pig_word.push(pig_char(c)),
            None => continue,
        }
        pig_word.push_str("ay ");
        pig_s.push_str(&pig_word);
    }
    pig_s.pop();
    println!("{pig_s}");
}

fn pig_char(c: char) -> char {
    if !c.is_alphabetic() {
        return 'h';
    }
    for l in ['a', 'e', 'i', 'o', 'u'] {
        if l == c {
            return 'h';
        }
    }
    c.to_lowercase().next().unwrap_or('h')
}
