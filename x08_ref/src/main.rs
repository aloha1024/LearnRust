// fn main() {
//     let s1 = gives_ownership();
//     println!("{}", s1);
//     let s2 = String::from("helloworld");
//     let s3 = takes_and_gives_back(s2);
//     // println!("{}", s2);
//     // value borrowed here after move
//     println!("{}", s3);
//     // println!("Hello, world!");
// }

// fn gives_ownership() -> String {
//     let s = String::from("hello gives_ownership");
//     return s;
// }

// fn takes_and_gives_back(s: String) -> String {
//     return s;
// }
// 引用:用法 &
// 让我们创建一个指向值的引用 但是并不拥有它 因为不拥有这个值 所以当引用离开其值指向的作用域后也不会被丢弃
// 借用:用法 &mut
fn main() {
    let mut s1 = String::from("abcdefg");
    let len1 = calcute_length(&s1);
    let s = &s1;
    let len2 = calcute_length(s);
    println!("{}{}", len1, len2);
    // println!("{}", s1);
    let ms = &mut s1;
    modify_s(ms);
    // println!("{}", s1);
    // println!("{}", s);
    println!("{}", ms);

    let r1 = &s1;
    let r2 = &s1;
    println!("{},{}", r1, r2);
    let r3 = &mut s1;
    r3.push_str("1231231");
}

fn calcute_length(s: &String) -> usize {
    return s.len();
}

fn modify_s(s: &mut String) {
    s.push_str("1234567");
}
