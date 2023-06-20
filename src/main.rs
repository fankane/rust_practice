use std::{env, process};

mod grep;

fn main() {
   let f = grep::InputConfig::build(env::args()).unwrap_or_else(|err|{
        println!("启动命令错误: {err}");
        process::exit(1)
   });
   grep::read_msg(f)
}

