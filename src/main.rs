#[allow(non_snake_case)]
use linked_table;
#[allow(non_snake_case)]
mod Command;

use std::io;
// use std::sync::Mutex;
use std::io::Write;
// use std::process::Command; 

// #[macro_use]
// extern crate lazy_static; 


use Command::command_manager::*;

fn main() {

    let mut  command_manager:CommandManager =CommandManager::new();
    command_manager.add_command("test","测试命令参数的输入,会打印输入的参数",|x,_|{
        println!("输出测试:{}",x);
        0
    });

    command_manager.add_command("help","帮助程序,打印所有命令",|_,m|{
        m.print_all();
        0
    });

    command_manager.add_command("quit","退出程序",|_,_|{
        std::process::exit(0);
    });

    //获取用户输入
    let input=io::stdin();
    loop{
        print!(">请输入命令:");
        if io::stdout().flush().is_err() {
            println!("flush err")
        }
        let mut cmd=String::new();
        input.read_line(&mut cmd).unwrap();
        let cmd_vec:Vec<&str>=cmd.trim().split(" ").collect();
        if cmd_vec.len()>0{
            let cmd=cmd_vec[0];
            let mut arg="";
            if cmd_vec.len()>1{
                arg=cmd_vec[1];
            }
            command_manager.run(cmd,arg);
        }
    }
}
