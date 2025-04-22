// slice是一种引用类型

fn main() {
    let s = String::from("hello world!");

    let hello = &s[0..5];
    let world = &s[6..11];
    println!("{hello} {world}");

    let len = s.len();

    let all = &s[0..len];
    println!("{all}");

    let word = first_world(&s);
    println!("{word}");

    let int_slice = [1, 2, 3, 4];

    for (i, &num) in int_slice.iter().enumerate() {
        println!("{i} positon is {num}");
    }

}

fn first_world(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// &str 字符串切片类型