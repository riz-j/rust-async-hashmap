use std::{collections::HashMap, fmt};

// Define a trait MathOperation
trait MathOperation {
    fn operate(&self, a: i32, b: i32) -> i32;
}

// Implement MathOperation for a generic function F
impl<F> MathOperation for F
where
    F: Fn(i32, i32) -> i32, // F is a function that takes two i32 and returns an i32
{
    fn operate(&self, a: i32, b: i32) -> i32 {
        self(a, b)
    }
}

// Define a function that we will use to implement the trait
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

#[derive(Default)]
struct Calculator {
    pub methods: HashMap<String, Box<dyn MathOperation>>,
}

impl Calculator {
    fn register<F>(&mut self, method_name: &str, method: F) -> ()
    where
        F: MathOperation + 'static,
    {
        self.methods
            .insert(method_name.to_string(), Box::new(method));
    }

    fn list_methods(&self) {
        for (name, _) in &self.methods {
            println!("{}", name);
        }
    }
}

fn main() {
    // Use the add function with the MathOperation trait
    let mut calculator = Calculator::default();
    calculator.register("add", add);
    calculator.register("multiply", multiply);

    calculator.list_methods();

    let result = calculator.methods.get("multiply").unwrap().operate(25, 2);
    println!("Result of addition: {}", result); // Output: "Result of addition: 7"
}
