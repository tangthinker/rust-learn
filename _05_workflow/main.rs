fn main() {
    let mut number = 3;

    if number < 5 {
        println!("the number is less than 5");
    } else {
        println!("the number is greater than 5");
    }

    if number % 2 == 0 {
        println!("the number is divisible by 2");
    } else if number % 3 == 0 {
        println!("the number is divisible by 3");
    } else {
        println!("the number is not divisible by 2 or 3");
    }

    let ret = if number % 2 == 0 { true } else { false };
    println!("the number is even? {}", ret);

    loop {
        if number == 0 {
            break;
        }
        number -= 1;
        println!("{number}.repeated!");
    }

    'flag: loop {
        loop {
            number += 1;
            if number == 6 {
                break;
            }
            if number == 10 {
                break 'flag;
            }
            println!("{number}.flag");
        }
    }

    while number >= 0 {
        println!("{number}.while");
        number -= 1;
    }

    let arr = [1, 1, 2, 3, 5, 8];
    print!("[");
    for i in arr {
        print!("{i},")
    }
    print!("]");
}
