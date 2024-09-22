fn main() {
    // mutable
    {
        let mut x = 5;
        println!("the value is {x}");
        x = 6;
        println!("the value is {x}");
    }

    // constants
    {
        const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
        println!("three hours is {THREE_HOURS_IN_SECONDS} seconds!");
    }

    // shadowing
    {
        let y = 5;
        println!("the y is {y}");

        // must use let to overshandow the first variable
        let y = "hello world!";
        println!("the y is {y}");
    }
}
