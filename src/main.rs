mod func;
mod structs;

use crate::structs::{Cli, FileEntry};
use clap::Parser;
use owo_colors::{OwoColorize};
use std::fs;
use tabled::{Table, settings::Style};
use tabled::settings::Color;
use tabled::settings::object::{Columns, Rows};
use tabled::tables::TableValue::Column;

fn main() {
    let cli = Cli::parse();

    let path = match cli.path {
        Some(path) => path,
        None => match std::env::current_dir() {
            Ok(path) => path,
            Err(err) => {
                eprintln!("{}", err);
                std::process::exit(1);
            }
        },
    };

    if let Ok(does_exists) = fs::exists(&path) {
        if does_exists {
            let tip = format!("目标路径: {}\n", path.display().green());
            println!("{}", tip.green());

            let get_file = FileEntry::get_file(&path);
            let mut table = Table::new(&get_file);
            table.with(Style::rounded());
            table.modify(Rows::first(), Color::FG_BRIGHT_CYAN);
            println!("{}", table);

        } else {
            let result = format!("目标路径不存在: {}", path.display());
            println!("{}", result.red());
        }
    } else {
        println!("{}", "读取目标路径发生错误".red());
    }

    // println!("{}", path.display());
}
