//        Simple Rust CLI to checksum
//     Copyright (C) 2020-2022  TheAwiteb
//      https://github.com/TheAwiteb/inrs
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use checksums::{hash_file, Algorithm};
use colored::Colorize;
use std::{path::Path, str::FromStr};

pub fn print_hashs(file: &Path, algorithms: Vec<&str>, plaintext: bool) {
    let file_name = file
        .file_name()
        .expect("Cannot get file name from file")
        .to_str()
        .expect("Cannot convert file name to string");
    let max_length = algorithms
        .iter()
        .map(|a| a.chars().count())
        .max()
        .expect("Never will get empty");
    for algorithm in algorithms {
        let hash = hash_file(
            file,
            Algorithm::from_str(algorithm).expect("Is has been validated"),
        );

        if plaintext {
            println!("# {algorithm}\n{hash}  {file_name}",)
        } else {
            let length = max_length - algorithm.chars().count();
            println!(
                "{} {} {}{} {}",
                file_name.purple(),
                algorithm.green().bold(),
                " ".repeat(length),
                "➤".yellow(),
                hash.cyan(),
            )
        }
    }
}

pub fn checksum(file: &Path, algorithms: Vec<&str>, check_hash: &str) {
    for algorithm in &algorithms {
        if hash_file(
            file,
            Algorithm::from_str(algorithm).expect("Is has been validated"),
        )
        .eq(check_hash)
        {
            println!(
                "{} Found match with `{}`",
                "✔".green().bold(),
                algorithm.green()
            );
            return;
        }
    }
    println!(
        "{} There is no any match with {} algorithms",
        "✖".red(),
        algorithms.len().to_string().red()
    );
}
