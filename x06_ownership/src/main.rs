/*
rust通过所有权管理内存,编译器在编译时就会根据所有权规则对内存使用进行检查
堆 栈
编译时,数据类型大小固定,则是分配在栈上
编译时,数据类型大小不固定,则是分配在堆上

作用域 :{}
string 内存回收
移动
clone
栈上数据拷贝
函数作用域
*/

fn main() {
    let x: i32 = 1;
    {
        let y: i32 = 2;
        println!("{}", x);
        println!("{}", y);
    }
    // println!("{}", y);

    {
        // let mut s1 = String::from("hello");
        // s1.push_str("world");
        let s1 = String::from("hello");
        println!("{}", s1);
        let s2 = s1;
        println!("{}", s2);
        // println!("{}",s1); move to s2, s1 invalid

        // clone
        let s3 = s2.clone();
        println!("{}", s3);
        println!("{}", s2);
    }

    // copy trait
    let a = 1;
    let b = a;
    println!("a={},b={}", a, b);
    // 常用的具有copy trait的有
    //  int float bool char tup
}
