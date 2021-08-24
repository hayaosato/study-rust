fn main() {
    let x = 5;
    println!("The value of x is {}", x);
    let x = x + 1;
    let x = x + 2;
    println!("The value of x is {}", x);
    let hoge = "x";
    println!("{}", hoge);
    let tup: (i32, f64, i8) = (100, 0.24, 3);
    let (x, y, z) = tup;
    println!("{}, {}, {}, {}", z, y, z, x);

    let sum: i32 = another_function(5, 22);
    println!("{}", sum);

    if sum < 20 {
        println!("condition was true");
    } else if sum < 30 {
        println!("sum is between 20 to 30");
    } else {
        println!("condition was false");
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }

    for number in (1..4).rev() {
        println!("{}!", number);
    }
}

fn another_function(x: i32, y: i32) -> i32 {
    x + y
}
