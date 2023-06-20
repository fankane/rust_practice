use std::{env, process};

use minigrep::InputConfig;

fn main() {
   let f = InputConfig::build(env::args()).unwrap_or_else(|err|{
        println!("启动命令错误: {err}");
        process::exit(1)
   });
   minigrep::read_msg(f)
}

