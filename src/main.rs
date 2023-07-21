use std::io::{self, Write};

fn main() {

    let mut input = String::new();

    println!("Insert initial bid/expected win");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut input).unwrap();
    let input = input.trim();
    let input = input.parse().unwrap();

    println!("For a ${} profit/input, with 3 losses, you would need ${}.", input, doubler(input, 3));
    println!("For a ${} profit/input, with 5 losses, you would need ${}.", input, doubler(input, 5));
    println!("For a ${} profit/input, with 7 losses, you would need ${}.", input, doubler(input, 7));


}

fn doubler(amount: u32, n: u32) -> u32{

    if n == 0 {
        return amount;
    }else if n == 1 {
        return amount * 3;
    }


    let mut prod = amount * 2;
    let mut sum = amount * 3;

    for _ in 2..n + 1{
        // println!("prod {}", prod);
        // println!("sum {}", sum);
        prod = prod * 2;
        sum = sum + prod;

    }
    return sum;
}