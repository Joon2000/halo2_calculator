mod calculator;

fn main() {
    let result = calculator::run();
    println!("Circuit result: {:?}", result);
}
