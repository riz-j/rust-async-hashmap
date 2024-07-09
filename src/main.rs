use std::{collections::HashMap, future::Future, pin::Pin};

// Define a trait MathOperation
trait MathOperation {
    fn operate(&self, a: i32, b: i32) -> Pin<Box<dyn Future<Output = i32> + Send>>;
}

// Implement MathOperation for a generic function F
impl<F, Fut> MathOperation for F
where
    F: Fn(i32, i32) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = i32> + Send + 'static,
{
    fn operate(&self, a: i32, b: i32) -> Pin<Box<dyn Future<Output = i32> + Send>> {
        Box::pin((self)(a, b))
    }
}

// Define a function that we will use to implement the trait
async fn add(a: i32, b: i32) -> i32 {
    a + b
}

async fn multiply(a: i32, b: i32) -> i32 {
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

#[async_std::main]
async fn main() {
    let mut calculator = Calculator::default();
    calculator.register("add", add);
    calculator.register("multiply", multiply);

    calculator.list_methods();

    let result = calculator
        .methods
        .get("multiply")
        .unwrap()
        .operate(25, 2)
        .await;

    println!("Result of calculation: {}", result);
}
