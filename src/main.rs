use std::io;

fn main() {
    println!("Fibonacci sequence!\nHow many iterations?");
    
    let iterations = read_number();

    println!("");

    fibonacci_recur((0,1), iterations);

    println!("");

    fibonacci_loop(iterations);
}

fn fibonacci_recur((num1,  num2) : (u64, u64),  num_iter :u32){
    if num_iter == 0{
        return;
    }

    println!("{}\n{}",num1 ,num2);
    if num_iter > 1 {
        fibonacci_recur((num1 + num2, num1 + num2 * 2), num_iter-1);
    }
}

fn fibonacci_loop(mut num_iter: u32){
    let mut num1: u64 = 0;
    let mut num2: u64 = 1;

    while num_iter > 0 {
        println!("{}\n{} ", num1, num2);
        num1 += num2;
        num2 += num1;

        num_iter -= 1;
    }
}

fn read_number() -> u32{
    let input = &mut String::new(); 
    
    io::stdin().read_line(input)
    .expect("Failed to read line");

    let input = match input.trim().parse() {
                Ok(num) => num,
                Err(_) => 0,
                };
    input
}



