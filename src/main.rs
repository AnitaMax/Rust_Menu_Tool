use std::io;

fn main() {
    let mut s=String::new();
    println!("请输入命令:");
    io::stdin().read_line(&mut s).expect("read error");
    let mut it=s.split_whitespace();
    match it.next().unwrap() {
            "help"| "h" => println!("帮助信息"),
            "test" | "t" => println!("测试信息"),
            _ => println!("输入无效"),
    }
}
