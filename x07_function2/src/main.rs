fn main() {
    let s = String::from("hello");
    let s1 = takes_ownership(s);
    println!("{}", s1);
    takes_ownership(s1);
    let x: i32 = 123;
    makes_copy(x);
    println!("{}", x);
    // println!("{}", s);
    // error : value borrowed here after move
}

fn takes_ownership(some_string: String) -> String {
    println!("some_string = {}", some_string);
    return some_string;
}

fn makes_copy(i: i32) {
    println!("i = {}", i);
}
