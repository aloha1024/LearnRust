fn main() {
    // bool
    let is_true: bool = true;
    let is_false: bool = false;
    println!("{},{}", is_true, is_false);

    // char 在rust中一个char 是 32位的 所以一个char可以是 "一个汉字"(两个不行)
    let a = 'a';
    println!("{}", a);
    let b = '你';
    println!("{}", b);

    let c: i8 = 110;
    println!("{}", c);

    let d: f32 = 0.01;
    println!("{}", d);

    // 自适应类型 isize,usize
    println!("max:{}", usize::max_value());
    println!("max:{}", isize::max_value());

    // 数组
    // [Type; size] (type也是数组类型的一部分)
    let arr1: [u32; 5] = [1, 2, 3, 4, 5];
    // 在c/c++中 给函数的参数传一个数组时实际上传递的是一个指针,此时的size作为参数是不起作用的
    // 而在rust中 这个size是起作用的
    println!("{}", arr1[0]);
    let arr2: [u32; 3] = [1, 2, 3];
    // show(arr1);
    // error:expected an array with a fixed size of 3 elements, found one with 5 elements
    show(arr2);

    // 元组
    let tup1: (i32, f32, char) = (1, 1.1, 'a');
    println!("{},{},{}", tup1.0, tup1.1, tup1.2);
    let tup2: (i32, f32, char) = (1, 1.1, 'a');
    println!("{},{},{}", tup2.0, tup2.1, tup2.2);

    // 元组的拆解
    let (x, y, z) = tup1;
    println!("{},{},{}", x, y, z)
}

fn show(arr: [u32; 3]) {
    for i in &arr {
        println!("{}", i)
    }
}
