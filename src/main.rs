use std::collections::HashMap;

// Define a trait MathOperation
trait MathOperation<F> {
    fn operate(&self, a: i32, b: i32) -> i32;
}

// Implement MathOperation for a generic function F
impl<F> MathOperation<F> for F
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

struct Calculator<F> {
    pub methods: HashMap<String, Box<dyn MathOperation<F>>>,
}

fn register<F>(function: F) -> i32
where
    F: MathOperation<F>,
{
    let a = 25;
    let b = 52;

    function.operate(a, b)
}

fn main() {
    // Use the add function with the MathOperation trait
    // calculator.register("add", add);
    // calculator.register("multiply", multiply);
    register(add);
    register(multiply);

    let result = add.operate(3, 4);
    println!("Result of addition: {}", result); // Output: "Result of addition: 7"
}
