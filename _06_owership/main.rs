// Owership
// Rust使用Owership系统的一系列编译时规则来管理内存
fn main(){
    {
        let s1 = String::from("hello");
        let s2 = s1;

        // println!("{s1}, world!");        // Error s1 was moved to s2, s1 is not valid any more
        println!("{s2}, world!");
    }

    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("{s1}, world!");
        println!("{s2}, world!");
    }
}