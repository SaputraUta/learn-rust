use std::fmt::{Display, Debug};

pub fn notify<T, U>(item1: &T, item2: &U)
where 
    T: Display + Debug,
    U: Debug,
{
    println!("Item 1: {}", item1);
    println!("Item 2: {:?}", item2);
}

fn main() {
    let x = "Rust 1.72 Released!";
    let y = vec![1, 2, 3];
    notify(&x, &y);
}