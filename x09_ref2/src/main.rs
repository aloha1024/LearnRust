fn main() {
    let ref_s = dangle();
    println!("{}",ref_s);
    println!("Hello, world!");
}

fn dangle() -> String {
    let s = String::from("hello");
    // return &s;
    //error
    return s;
}

//任意给定时间,有了可变引用后就不能再有不可变引用
//引用必须有效