fn main() {
    println!("Allo, Ola!");
    let name = "Ola";
    println!("Hola, {}!", name);
    let mut age = 42;
    age += 1;
    let address: &str = "123, Smartville MyCounty Region UK";
    println!("Current age: {}", age);
    println!("Address: {}", address);

    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug)]
    struct Location {
        x: i32,
        y: i32,
        z: i32,
    }

    let point = Point { x: 24, y: 42 };
    println!("({}, {})", point.x, point.y);

    println!("Another Approach");
    println!("{:#?}", point);

    println!("Location Struct\n");
    let location = Location {
        x: 23,
        y: 35,
        z: 108,
    };

    println!("{:#?}", location);

    fn max_value(a: i32, b: i32) -> i32 {
        if a > b {
            a
        } else {
            b
        }
    }

    let max_res = max_value(3, 5);
    println!("Max Value: {}", max_res);
}
