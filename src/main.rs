mod func;
mod structs;

use crate::structs::{Cli, FileEntry};
use clap::Parser;
use owo_colors::OwoColorize;
use std::fs;
use tabled::settings::Color;
use tabled::settings::object::{Columns, Rows};
use tabled::tables::TableValue::Column;
use tabled::{Table, settings::Style};

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

    match fs::canonicalize(&path) {
        Ok(absolute_path) => {
            let tip = format!("目标路径: {}\n", absolute_path.display().green());
            println!("{}", tip.green());

            let get_file = FileEntry::get_file(&absolute_path);
            let mut table = Table::new(&get_file);
            table.with(Style::rounded());
            table.modify(Rows::first(), Color::FG_BRIGHT_CYAN);
            println!("{}", table);
        }
        Err(_) => {
            let result = format!("目标路径不存在: {}", path.display());
            println!("{}", result.red());
        }
    }

    // println!("{}", path.display());
}
