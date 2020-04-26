fn main() {
    println!("Hello, world!");
    func01();
    let a: i32 = 11;
    let b: u32 = 22;
    func02(1, 2);
    func02(a, b);

    println!("func3 result is {}", func03(1, 2));
    println!("func3_copy result is {}", func03_copy01(11, 22));
    println!("func3_copy result is {}", func03_copy02(111, 222));

    // 语句是执行一些操作,但是不返回值的指令
    // let y = 1; //语句 不返回值
    // let x = (let y = 1);

    // 表达式会计算一些值
    let y = {
        let x = 1;
        x + 1
    };
    println!("{}", y)
}

fn func01() {
    println!("this is func01");
}

// fn func02(a, b)  error
fn func02(a: i32, b: u32) {
    println!("{},{}", a, b);
}

fn func03(a: i32, b: i32) -> i32 {
    let res = a + b;
    return res;
}

fn func03_copy01(a: i32, b: i32) -> i32 {
    let res = a + b;
    res
}

fn func03_copy02(a: i32, b: i32) -> i32 {
    a + b
}
