use std::fmt::format;


pub struct RpnCalculator {
    stack: Vec<u32>,
}

impl RpnCalculator {
    pub fn new() -> RpnCalculator {
        RpnCalculator{stack:Vec::new()}
    }

    pub fn calculate(&mut self, input:char) {

        if input.is_ascii_digit() {
            self.stack.push(input.to_digit(10).unwrap());
        } else {
            match input {
                '+' => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a+b);
                },
                '*' => {
                    let b = self.stack.pop().unwrap();
                    let a = self.stack.pop().unwrap();
                    self.stack.push(a*b);
                },
                _   => {}
            }
        } 

        let msg = format!("{input}=>{:?}",self.stack);
        println!("{msg}");
    }
}