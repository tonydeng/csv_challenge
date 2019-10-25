/**
 * 作用于core/read.rs
 * 将在read.rs中引入的std模块都溢出来，使用了Rust 2018新模块系统支持的use语句内嵌语法
 * */ 
use super::{Error, PathBuf,File,Read,Write};

pub fn load_csv(csv_file: PathBuf) -> Result<String,Error> {
    let file = read(csv_file)?;
    Ok(file)
}

pub fn write_csv(csv_data: &str, filename: &str) -> Result<(),Error> {
    write(csv_data,filename)?;
    Ok(())
}

fn read(path: PathBuf) -> Result<String,Error> {
    let mut buffer = String::new();
    let mut file = open(path)?;
    file.read_to_string(&mut buffer)?;

    if buffer.is_empty() {
        return Err("input file missing")?
    }

    Ok(buffer)
}

fn open(path: PathBuf) -> Result<File, Error> {
    let file = File::open(path)?;
    Ok(file)
}

fn write(data: &str, filename : &str) -> Result<(), Error> {
    let mut buffer = File::create(filename)?;
    buffer.write_all(data.as_bytes())?;
    Ok(())
}