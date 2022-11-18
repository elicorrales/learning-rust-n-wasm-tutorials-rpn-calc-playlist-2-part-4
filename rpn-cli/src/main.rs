use std::io::Write;
use console::Term;

fn main() {
    println!("Hello, world, I am your friendly calculator!");

    let myterm = Term::stdout();

    let mut quit = false;
    while !quit {
        let mychar = myterm.read_char().unwrap();
        print!("{mychar}");
        std::io::stdout().flush().unwrap();

        if mychar == 'q' {
            println!("quitting....");
            quit = true;
        }
    }
}

/*
    loop {
        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string);
        println!("{}", input_string.trim());
    }
*/