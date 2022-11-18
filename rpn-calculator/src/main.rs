use std::io::Write;
use rpn_calc::*;


fn main() {
    println!("Hello, world, I am your friendly calculator!");

    let mut my_calculator = RpnCalculator::new();


    let mut quit = false;
    while !quit {

        let mut input_string = String::new();
        std::io::stdin().read_line(&mut input_string);

        for mychar in input_string.chars() {

            if mychar == 'q' {
                println!("quitting....");
                quit = true;
            }

            my_calculator.calculate(mychar);

        }

    }

}
/* println!("Hello, world, I am your friendly calculator!");

    let mut my_calculator = RpnCalculator::new();

    let myterm = Term::stdout();

    let mut quit = false;
    while !quit {
        let mychar = myterm.read_char().unwrap();

        if mychar == 'q' {
            println!("quitting....");
            quit = true;
        }

        my_calculator.calculate(mychar);

    }
*/