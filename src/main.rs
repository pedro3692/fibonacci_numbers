use std::io;

fn main() {
    println!("Fibonacci sequence!\nHow many numbers?");
    
    let times = read_number();

    println!("");

    fibonacci_recur((0,1), times);
}

fn fibonacci_recur(( n1,  n2) : (u32, u32),  index :u32){
    println!("{}\n{}",n1 ,n2);
    if index !=0 {
        let n3 = n2 + n1;
        fibonacci_recur((n3, n3 + n2), index-1);
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



