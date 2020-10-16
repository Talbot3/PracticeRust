fn main() {
    println!("Hello, world!");
    let result = another_function(5);
    let y = {
        let x = 3;
        x + 1
    };
    println!("{} {}", y, result);
}

fn another_function(x: i32) -> i32 {
    x
}