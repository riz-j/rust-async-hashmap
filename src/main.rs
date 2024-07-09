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

// Define a function that we will use to implement the trait
async fn add(a: i32, b: i32) -> i32 {
    a + b
}

async fn subtract(a: i32, b: i32) -> i32 {
    a - b
}

async fn multiply(a: i32, b: i32) -> i32 {
    a * b
}

async fn divide(a: i32, b: i32) -> i32 {
    a / b
}

#[async_std::main]
async fn main() {
    let mut calc = Calculator::default();
    calc.register("add", add);
    calc.register("subtract", subtract);
    calc.register("multiply", multiply);
    calc.register("divide", divide);

    calc.list_methods();

    let result = calc.methods.get("add").unwrap().operate(24, 2).await;
    assert_eq!(result, 26);

    let result = calc.methods.get("subtract").unwrap().operate(24, 2).await;
    assert_eq!(result, 22);

    let result = calc.methods.get("multiply").unwrap().operate(24, 2).await;
    assert_eq!(result, 48);

    let result = calc.methods.get("divide").unwrap().operate(24, 2).await;
    assert_eq!(result, 12);

    println!("Latest result of calculation: {}", result);
}
