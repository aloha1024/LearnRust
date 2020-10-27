const MAX_POINT: u32 = 10000;
fn main() {
    // 1.变量类型
    // 变量类型可以自动推导
    let a = 1;
    // let b: u32 = 1;
    // 变量没有用mut,则不可变
    let mut b: u32 = 1;
    println!("a={},b={}",a,b);

    // b=2;
    // println!("b={}",b);
    // error[E0384]: cannot assign twice to immutable variable `b`
    // 不能对变量赋值两次
    b=2;
    println!("b={}",b);

    // 2.隐藏性
    let b: f32 = 1.1;
    // 将前面的变量隐藏
    println!("b={}",b);

    // 3.常量
    println!("max={}",MAX_POINT)

}
