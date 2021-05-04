use std::io;

fn main() {
    let  x = 5;
    println!("The value of x is: {}", x);
    let x = x + 1;
    println!("The value of x is: {}", x);
    let x = x * 2;
    println!("The value of x is: {}", x);

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of len is: {}", spaces);

    let mut guess = String::new();
    println!("Type guess.");
    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    let guess: u32 = guess.trim().parse().expect("Not a number!");
    println!("The value of guess is: {}", guess);

    let y = 2.0; // 64 bits float
    let z: f32 = 30.0; // 32 bits float

    let sum = 5 + 10;
    let dif = 95.5 - 4.3;
    let prod = 4 * 30;
    let quot = 56.7 / 32.2;
    let remainder = 43 % 5;

    let t = true;
    let f: bool = false;
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a_1, _a_2, _a_3) = tup;

    let tupa: (i32, f64, u8) = (500, 6.4, 1);
    let five_hundred = tupa.0;
    let six = tupa.1;
    let one = tupa.2;

    let ar = [1, 2, 3, 4, 5];
    let mes = ["Janeiro", "Fevereiro", "Março", "Abril", "Maio"];

    let nar: [i32; 5] = [1, 2, 3, 4, 5];
    let nas = [3; 5];

    let first = nas[2];
    let second = nar[1];
    println!("First {}", first);
    println!("Second {}", second);
    println!("Mes {}", mes[2]);
    //println!("Nar {}", nar);
    //println!("Nas {}", nas);

}
