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

    {
        let s1 = gives_ownership();
        println!("{s1} was moved from gives ownership");

        let mut s2 = String::from("Hello");
        s2 = takes_and_gives_back(s2);
        println!("{s2}")
    }


}

fn gives_ownership() -> String {

    let str = String::from("你好");

    str

}

fn takes_and_gives_back(mut str: String) -> String {
    str = str + " world!";
    str.push_str(" 你好，世界！");
    str
}