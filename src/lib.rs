mod opt;
mod err;
mod core;

/***
 * 重新导出(Re-exporting)
 * 简化外包调用的导出路径，而且也不需要对外暴露模块
 */ 
pub use self::opt::Opt;
pub use self::core::{
    read::{load_csv,write_csv},
    write::replace_column,
};