use std::io;

fn main() {
    let mut guess = String::new();
    loop {
        io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

        let guess: u8 = match guess.trim().parse() {
            Ok(num) => num,
            Err(e) => continue,
        };

        println!("{}", converter(guess))
    }
}

fn converter(ascii: u8) -> char {
    return ascii as char;
}
