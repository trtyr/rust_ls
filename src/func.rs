use crate::structs::{FileEntry, FileMetadata, FileType};
use chrono::{DateTime, Local};
use std::fs;
use std::fs::DirEntry;
use std::os::windows::ffi::OsStrExt;
use std::path::PathBuf;
use windows::core::PCWSTR;
use windows::Win32::Storage::FileSystem::{
    GetFileAttributesW, FILE_ATTRIBUTE_ARCHIVE, FILE_ATTRIBUTE_COMPRESSED,
    FILE_ATTRIBUTE_DIRECTORY, FILE_ATTRIBUTE_ENCRYPTED, FILE_ATTRIBUTE_HIDDEN,
    FILE_ATTRIBUTE_READONLY, FILE_ATTRIBUTE_REPARSE_POINT, FILE_ATTRIBUTE_SYSTEM,
};

impl FileEntry {
    // 获取文件名称
    fn get_file_name(file: &DirEntry) -> String {
        match file.file_name().into_string() {
            Ok(file_name) => file_name,
            Err(err) => {
                eprintln!("{}", err.display());
                std::process::exit(1);
            }
        }
    }

    // 获取文件 metadata 信息
    fn get_file_metadata(file: &DirEntry) -> FileMetadata {
        let metadata = match file.metadata() {
            Ok(metadata) => metadata,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        };

        FileMetadata {
            size: metadata.len(),
            modified: match metadata.modified() {
                Ok(time) => {
                    let datetime: DateTime<Local> = time.into();
                    datetime.format("%Y-%m-%d %H:%M:%S").to_string()
                }
                Err(err) => {
                    eprintln!("{}", err);
                    std::process::exit(1);
                }
            },
            permissions: format_windows_attributes(&file.path()),

            file_type: match if metadata.is_dir() {
                FileType::Directory
            } else if metadata.is_file() {
                FileType::File
            } else if metadata.is_symlink() {
                FileType::Symlink
            } else {
                FileType::Unknown
            } {
                FileType::Unknown => "未知".to_string(),
                FileType::File => "文件".to_string(),
                FileType::Directory => "目录".to_string(),
                FileType::Symlink => "快捷方式".to_string(),
            },
        }
    }

    // 获取目标目录下的文件信息
    pub(crate) fn get_file(path: &PathBuf) -> Vec<FileEntry> {
        let mut file_data = Vec::default();

        if let Ok(file_iter) = fs::read_dir(path) {
            let _ = file_iter
                .into_iter()
                .filter_map(Result::ok)
                .map(|file| {
                    let file_metadata = FileEntry::get_file_metadata(&file);

                    let file = FileEntry {
                        file_name: FileEntry::get_file_name(&file),
                        file_len: file_metadata.size,
                        permissions: file_metadata.permissions,
                        modified: file_metadata.modified, // 直接使用已经格式化的时间字符串
                        file_type: file_metadata.file_type,
                    };

                    file_data.push(file)
                })
                .collect::<Vec<_>>();
        }
        file_data
    }
}

fn format_windows_attributes(file_path: &PathBuf) -> String {
    // 将路径转换为 Windows API 需要的宽字符格式
    let mut wide_path: Vec<u16> = file_path
        .as_os_str()
        .encode_wide()
        .chain(std::iter::once(0))
        .collect();

    let pcwstr = PCWSTR(wide_path.as_ptr());

    // 获取文件属性
    let attrs = unsafe { GetFileAttributesW(pcwstr) };

    // 如果获取失败，返回错误信息
    if attrs == 0xFFFFFFFF {
        return "无法获取属性".to_string();
    }

    // 定义属性映射表：(Windows 常量, 显示名称)
    let flags = [
        (FILE_ATTRIBUTE_DIRECTORY.0, "目录"),
        (FILE_ATTRIBUTE_REPARSE_POINT.0, "软链接"),
        (FILE_ATTRIBUTE_READONLY.0, "只读"),
        (FILE_ATTRIBUTE_HIDDEN.0, "隐藏"),
        (FILE_ATTRIBUTE_SYSTEM.0, "System"),
        (FILE_ATTRIBUTE_ARCHIVE.0, "Archive"),
        (FILE_ATTRIBUTE_COMPRESSED.0, "Compressed"),
        (FILE_ATTRIBUTE_ENCRYPTED.0, "Encrypted"),
    ];

    let mut parts = Vec::new();
    for (mask, name) in flags {
        if attrs & mask != 0 {
            parts.push(name);
        }
    }

    if parts.is_empty() {
        "Normal".to_string()
    } else {
        parts.join(" | ")
    }
}
