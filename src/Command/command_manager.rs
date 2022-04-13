use linked_table::*;
use crate::Command::command::Command;
pub struct  CommandManager<'a>{
    commands:LinkedTable<Command<'a>>,
}

impl<'a> CommandManager<'a>{
   pub fn new()->CommandManager<'a>{
       CommandManager{
           commands:LinkedTable::new(),
       }
   }
   pub fn add_command(& mut self,cmd:&'static str,desc:&'static str,handler:fn(&str,&'a CommandManager)->isize){
       let new_cmd=Command{
           cmd:cmd,
           desc:desc,
           handler:handler,
       };
       self.commands.push(new_cmd);
   }
   pub fn run(&'a self,cmd:&str,arg:&str){
       let command_node=self.commands.find(|x| x.cmd==cmd);
       if command_node.is_none(){
           println!("无效命令");
       }else{
           let command=command_node.unwrap().get_data();
           let f=command.handler;
           let ret=f(arg,self);
           if ret!=0{
               print!("X");
           }
       }
   }

   pub fn print_all(&self){
       println!("命令列表:");
         self.commands.for_all(|x|{
              println!("{}:{}",x.cmd,x.desc);
         });
    //    let mut cur=self.commands.head.as_ref();
    //    while !cur.is_none(){
    //        let command=cur.unwrap().get_data();
    //        println!("{}:{}",command.cmd,command.desc);
    //        cur=cur.unwrap().next.as_ref();
    //    }
   }
}
