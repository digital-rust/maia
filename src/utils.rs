#[allow(unused_imports)]
use crossterm::{
    execute,
    cursor,
    terminal,
    event,
    style,};

// just a test function
pub fn test_mod_func() -> &'static str {
    let string: &str = "hello";
    return string;
}

// REDEFINE - remove \n & \r from strings
pub fn trim_newline(s: &mut String) -> String {
    if s.ends_with('\n') {
        s.pop();
        if s.ends_with('\r') {
            s.pop();
        }
    }
    return s.to_string();
}

// TERMINAL RELATED
pub fn term_height() -> usize {
    terminal::size().unwrap().1 as usize
}
pub fn term_width() -> usize {
    terminal::size().unwrap().0 as usize
}
pub fn pr_logo() {
    println!(r"
     ___ ___   ____  ____   ____ 
    |   |   | /    ||    | /    |
    | _   _ ||  o  | |  | |  o  |
    |  \_/  ||     | |  | |     |
    |   |   ||  _  | |  | |  _  |
    |   |   ||  |  | |  | |  |  |
    |___|___||__|__||____||__|__|");
    println!("\n");    
}

