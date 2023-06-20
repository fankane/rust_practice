use std::fs;

#[derive(Debug)]
pub struct InputConfig {
    query: String,
    file_path: String,
}

impl InputConfig {
    // pub fn new(args: &[String]) -> Result<InputConfig, &str> {
    //     if args.len() < 3 {
    //         return Err("必要参数不够");
    //     }
    //     Ok(InputConfig { query: args[1].clone(), file_path:  args[2].clone() }) 
    // }
    // pub fn build(args: &[String]) -> Result<InputConfig, &str> {
    //     if args.len() < 3 {
    //         return Err("必要参数不够");
    //     }
    //     Ok(InputConfig { query: args[1].clone(), file_path:  args[2].clone() }) 
    // }

    pub fn build(mut args: impl Iterator<Item = String>) -> Result<InputConfig, &'static str> {
        args.next(); //第一个参数是执行文件，跳过
        let query_input = match args.next() {
            Some(a) => a,
            None => return Err("查询项缺乏"), 
        };

        let file_path_input = match args.next() {
            Some(a) => a,
            None => return Err("文件缺乏"), 
        };

        Ok(InputConfig { query: query_input, file_path:  file_path_input }) 
    }
}

pub fn read_msg(conf: InputConfig) {

   // 读取文件内容
   let content = fs::read_to_string(conf.file_path).expect("read file failed");

    for res in search(&conf.query, &content) {
        println!("{res}");
    }
//    println!("{}", content)
}

pub fn search<'a>(query:&'a str, content: &'a str) -> Vec<&'a str> {
    // let mut resList = Vec::new();
    // for tmp in content.lines() {
    //     if tmp.contains(query) {
    //         resList.push(tmp);
    //     }
    // }
    // resList
    content.
    lines().
    filter(|line| line.contains(query)).
    collect()
}

mod testsHF {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}