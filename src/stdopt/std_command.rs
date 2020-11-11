#![allow(unused_macros)]

use std::process::Command;

macro_rules! astring {
    ($name:expr) => {
        String::from($name);
    };
}

macro_rules! aprint {
    ( $( $name:expr),*) => {
       $( print!("{}",$name);)*
    };
}

struct ExcutedCommand {
    name:String,
    args : Vec<String>
}

impl ExcutedCommand {

    fn new(name:String,args : Vec<String>) -> Self{
        ExcutedCommand{
            name,
            args
        }
    }

    fn excute(&self) -> String {
       let cmd = Command::new(&self.name)
        .args(&self.args)
        .output()
        .expect("failed to execute process");
       let result = cmd.stdout;
       String::from_utf8_lossy(&result).to_string()
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_command(){
        use super::*;
        let cmd = astring!("adb");
        let arg = astring!("help");
        let c = ExcutedCommand::new(cmd,vec![arg] );
        aprint!(c.excute());
    }
}   