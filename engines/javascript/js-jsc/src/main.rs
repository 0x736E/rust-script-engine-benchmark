use rusty_jsc::{JSContext};

fn main() {
    let mut context = JSContext::default();

    let sample  = include_str!("../../../../samples/sample.js");

    match context.evaluate_script(sample, 1) {
        Ok(value) => {
            println!("The answer is: {}", value.to_string(&context).unwrap());
        }
        Err(e) => {
            println!("Error evaluating script: {}", e.to_string(&context).unwrap())
        }
    }
}