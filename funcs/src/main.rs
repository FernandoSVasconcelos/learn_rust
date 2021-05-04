fn main() 
{
    println!("Hello, world!");
    let x = another_function(5, 6);
    println!("The value of x is: {}", x);
    let a = [10, 20, 30, 40, 50];
    for element in a.iter()
    {
        println!("The value is: {}", element);
    }
    for number in (1..4).rev()
    {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}

fn another_function(x: i32, y: i32) -> i32
{
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
    x + y // Sem ;
}