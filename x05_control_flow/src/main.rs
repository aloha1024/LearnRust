fn main() {
    println!("Hello, world!");

    // if
    let y = 1;
    if y == 1 {
        println!("y=1");
    }

    // if-else
    let y = 2;
    if y == 1 {
        println!("y=1");
    } else {
        println!("y!=1");
    }

    // if else if else
    let y = 5;
    if y == 1 {
        println!("y=1");
    } else if y == 2 {
        println!("y=2");
    } else if y == 3 {
        println!("y=3");
    } else {
        println!("ok");
    }

    // let 中使用if
    let condition = false;
    let x = if condition {
        5
    } else {
        6
        //six //error
    };
    println!("{}", x);

    let mut counter = 0;
    // loop
    loop {
        println!("this is a loop");
        if counter == 10 {
            break;
        }
        counter = counter + 1;
    }

    let result = loop {
        counter += 1;
        if counter == 20 {
            break counter * 2;
        }
    };
    println!("{}", result);

    // while
    let mut i = 0;
    while i != 10 {
        i = i + 1;
        println!("i = {}", i);
    }

    let arr: [u32; 5] = [1, 2, 3, 4, 5];
    for element1 in arr.iter() {
        println!("ele1 = {}", element1);
    }
    for element2 in &arr {
        println!("ele2 = {}", element2);
    }
}
