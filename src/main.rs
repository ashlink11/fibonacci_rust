/*
Convert temperatures between Fahrenheit and Celsius.
Generate the nth Fibonacci number.
Print the lyrics to the Christmas carol “The Twelve Days of Christmas,” 
taking advantage of the repetition in the song.
*/
fn main() {
    println!("Hello, world!");
    println!("Fib 1 is {}", fibonacci(1));
    println!("Fib 2 is {}", fibonacci(2));
    println!("Fib 3 is {}", fibonacci(3));
    println!("Fib 4 is {}", fibonacci(4));    
    println!("Fib 5 is {}", fibonacci(5));
    println!("Fib 6 is {}", fibonacci(6));
    println!("Fib 7 is {}", fibonacci(7));    
    println!("37 celsius is {} F", c_to_f(37.0));    
    println!("98.6 F is {} celsius", f_to_c(98.6));    
}

fn fibonacci(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }
    return fibonacci(n-1) + fibonacci(n-2);
}

fn f_to_c (f: f32) -> f32 {
    ((f - 32.0) * 5.0) / 9.0
}

fn c_to_f (c: f32) -> f32 {
    ((c / 5.0) * 9.0) + 32.0
}