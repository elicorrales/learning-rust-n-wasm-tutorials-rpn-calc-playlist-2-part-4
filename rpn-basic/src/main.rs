use rpn_calc::*;

fn main() {
    let mut my_calculator = RpnCalculator::new();

    let fake_keybd_inputs =
        vec!['2','3','+','4','1','+','*'];

    for input in fake_keybd_inputs {
        my_calculator.calculate(input);
    }
}
