use std::io::{stdin, stdout, Read, Write};

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();
}

fn main() {

    println!(r"
         ___ ___   ____  ____   ____ 
        |   |   | /    ||    | /    |
        | _   _ ||  o  | |  | |  o  |
        |  \_/  ||     | |  | |     |
        |   |   ||  _  | |  | |  _  |
        |   |   ||  |  | |  | |  |  |
        |___|___||__|__||____||__|__|");
    println!("\n");

    pause();
}
