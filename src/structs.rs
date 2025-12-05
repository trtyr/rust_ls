use clap::Parser;
use std::path::PathBuf;
use tabled::Tabled;

#[derive(Debug, Parser)]
#[command(version = "1.0.0", about = "定制版 ls")]
pub struct Cli {
    pub(crate) path: Option<PathBuf>,
}

#[derive(Debug)]
pub enum FileType {
    File,
    Directory,
    Symlink,
    Unknown,
}

#[derive(Debug)]
pub enum Permissions {}

#[derive(Debug, Tabled)]
pub struct FileEntry {
    #[tabled(rename = "文件属性")]
    pub(crate) permissions: String,
    #[tabled(rename = "类型")]
    pub(crate) file_type: String,
    #[tabled(rename = "最后修改时间")]
    pub(crate) modified: String,
    #[tabled(rename = "字节大小")]
    pub(crate) file_len: u64,

    #[tabled(rename = "名称")]
    pub(crate) file_name: String,
}

pub struct FileMetadata {
    pub(crate) size: u64,
    pub(crate) modified: String,
    pub(crate) permissions: String,
    pub(crate) file_type: String,
}
