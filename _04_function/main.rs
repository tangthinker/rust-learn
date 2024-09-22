fn main() {
    function_param_1(5);

    function_param_2(9, 'k');

    function_3();

    println!("4. the value of return is {}", function_with_return_4(1, 2));

    println!("5. the value of return is {}", function_auto_return_5(1, 2));


}

fn function_param_1(x: i64) {
    println!("1. the value of x is {x};");
}

fn function_param_2(x: i64, c: char) {
    println!("2. the value of x is {x}, the value of c is {c};");
}

fn function_3() {
    let y = {
        let x = 3;
        x + 1
    };

    println!("3. the value of y is {y}");
}

fn function_with_return_4(a: i64, b: i64) -> i64 {
    return a + b;
}

fn function_auto_return_5(a: i64, b: i64) -> i64 {
    a + b
}


