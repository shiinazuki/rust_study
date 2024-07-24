use std::{env, error::Error, fs};

pub fn run(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    reult(args)
}

fn reult(args: impl Iterator<Item = String>) -> Result<(), Box<dyn Error>> {
    let args = Args::build(args)?;

    let contents = fs::read_to_string(format!("./chapt12_io_item/{}", args.file_path))?;

    let result = if args.ignore_case {
        search_case_insensitive(&args.query, &contents)
    } else {
        search(&args.query, &contents)
    };

    for line in result {
        println!("{}", line);
    }

    Ok(())
}

pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    // let mut result = Vec::new();
    // for line in contents.lines() {
    //     if line.to_lowercase().contains(&query) {
    //         result.push(line);
    //     }
    // }

    // 使用迭代器语法优化
    contents
        .lines()
        .filter(|line| line.contains(query))
        .collect()
}

// 大小写不敏感
pub fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    contents
        .lines()
        .filter(|line| line.to_lowercase().contains(&query.to_lowercase()))
        .collect()
}

struct Args {
    query: String,
    file_path: String,
    ignore_case: bool,
}

impl Args {
    fn build(mut args: impl Iterator<Item = String>) -> Result<Args, &'static str> {
        args.next();

        let query = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a query string"),
        };
        let file_path = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't get a file path"),
        };

        let ignore_case = env::var("IGNORE_CASE").is_ok();

        Ok(Args {
            query,
            file_path,
            ignore_case,
        })
    }
}
