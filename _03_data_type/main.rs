fn main() {
    // scalar types
    {
        // signed int                           /   unsigned int
        // i8/i16/i32/i64/i128/isize(with arch)     u8/u16...
        // default i32 int32
        // integer overflow will cause panic in debug mode
        //                       cause wrapping in release mode, e.g. u8 256 -> 0 257 -> 1

        // floating point
        // f32 f64
        // default f64

        // boolean
        // bool : true false
        // one bit size

        // character
        // char
        // single char
        // 4 bytes in size, Unicode Scalar Value
    }

    // compound types
    {
        // tuple
        // can group different type in one tuple
        // fixed length
        let t = (400, 3.14, 'k');
        let second = t.1;
        println!("the tup is {second}");

        // array
        // every element in array must be the same type
        // fixed length
        let arr = [1, 2, 34, 4];
        let second = arr[1];
        println!("the arr is {second}");
    }
}
