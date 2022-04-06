use crate::Command::command_manager::*;

pub struct Command<'a> {
    pub cmd:&'static str,
    pub desc:&'static str,
    pub handler:fn(&str,&'a CommandManager)->isize,
}