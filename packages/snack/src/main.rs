fn main() {
    println!("Hello, world!");
    let result = another_function(5);
    let y = {
        let x = 3;
        x + 1
    };
    println!("{} {}", y, result);
    condition_function(y);
}

fn another_function(x: i32) -> i32 {
    x
}

fn condition_function(number: i32) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}