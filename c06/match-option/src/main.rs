fn main() {
    println!("Hello, world!");
    let five = Some(5);
    println!("{:?} unwraps to {:?}",five, five.unwrap());
    let six = plus_one(five);
    println!("{:?}",six);
    let none = plus_one(None);
    println!("{:?}",none);
}

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i+1)
    }
}
