fn main() {
    let message = "Hello, World!";
    println!("{}", message);
    
    let numbers = vec![1, 2, 3, 4, 5];
    for num in numbers {
        if num % 2 == 0 {
            println!("Even: {}", num);
        } else {
            println!("Odd: {}", num);
        }
    }
}
