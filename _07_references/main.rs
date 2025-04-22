
fn main() {
    let s1 = String::from("Hello world!");

    let len = str_len(&s1); // 使用引用来进行传递

    println!("{s1} len is {len}");

    let s2 = String::from("你好，世界！");

    let len = str_len(&s2); // 一个中文字符占用3个字节

    println!("{s2} len is {len}");

    let mut s3 = String::from("Hello mutable reference!");
    str_change(&mut s3);    // 使用可变引用进行传递

    println!("{s3}");

    {
        let mut s1 = String::from("Hello world!");

        let r1 = &mut s1;
        // let r2 = &mut s1;  // 报错，同一时刻中只允许一个可变引用

        println!("{}", r1);

        let r2 = &mut s1;
        println!("{}", r2);
    }

    {
        let mut s1 = String::from("Hello world!");

        let r1 = &s1;
        let r2 = &s1;
        // let r3 = &mut s1; // 报错，同一时刻不能同时有可变和不可变引用
        println!("{} {}", r1, r2);

        let r3 = &mut s1;
        println!("{r3}");
    }

}

fn str_len(str: &String) -> usize {
    str.len()
}

fn str_change(mut_str: &mut String) {
    mut_str.push_str(" was changed!")
}

// fn dangle_pointer() -> &String {
//     let s = String::from("hello");

//     &s
// }   // s is dropped now, s pointer is a dangle pointer

// right 
fn no_dangle() -> String {
    let s = String::from("hello");
    s
}