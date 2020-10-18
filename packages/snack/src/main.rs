struct Point<T> {
    x: T,
    y: T,
}

struct Point2<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point2<T, U> {
    fn mixup<V, W>(self, other: Point2<V, W>) -> Point2<T, W> {
        Point2 {
            x: self.x,
            y: other.y,
        }
    }
}

struct Point3<T> {
    x: T,
    y: T,
}

impl<T> Point3<T> {
    fn x(&self) -> &T {
        &self.x
    }

    fn y(&self) -> &T {
        &self.y
    }
}



fn main() {
    println!("Hello, world!");
    let result = another_function(5);
    let y: i32 = {
        let x = 3;
        x + 1
    };
    println!("{} {}", y, result);
    condition_function(y);
    while_function(y);

    let integer = Point {x: 5, y: 10};
    let float = Point {x: 1.0, y: 4.0};
    println!("{} {}", integer.x, float.y);

    let integer = Point2 {x: 5, y: 10};
    let float = Point2 {x: 1.0, y: 4.0};
    let p1 = Point2 { x: 5, y: 10.4 };
    let p2 = Point2 { x: "Hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);

    println!("{} {}", integer.x, float.y);
    let p = Point3 { x: 5, y: 10 };
    println!("p.x = {}, p.y = {}", p.x(), p.y());
}

fn another_function(x: i32) -> i32 {
    x
}

fn condition_function(number:  i32 ) {
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
}

fn while_function(_number: i32) {
    let mut number = _number;
    while number != 0 {
        println!("{}!", number);
        number = number - 1;
    }
    println!("LIFTOFF!!!");
}