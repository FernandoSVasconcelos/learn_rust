fn main() {

    let spaces = "  ";
    let spaces = spaces.len();
    println!("The value of len is: {}", spaces);

    let y = 2.0; // 64 bits float
    let z: f32 = 30.0; // 32 bits float
    println!("The value of y is: {}", y);
    println!("The value of z is : {}", z);

    let z = 'ℤ';
    let heart_eyed_cat = '😻';

    println!("The value of z is: {}", z);
    println!("The value of heart_eyed_cat is: {}", heart_eyed_cat);

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_a_1, _a_2, _a_3) = tup;

    //let five_hundred = tup.0;
    //let six = tup.1;
    //let one = tup.2;

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
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2);

}
