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

use std::{fs::File, path::Path};

pub fn check_file_existing(path: &str) -> Result<String, String> {
    let file_path = Path::new(path);
    if !file_path.exists() {
        Err(format!("'{path}' is not exists"))
    } else if !file_path.is_file() {
        Err(format!("'{path}' is not file"))
    } else if File::open(file_path).is_err() {
        Err(format!("Cannot read `{path}`"))
    } else {
        Ok(path.to_string())
    }
}
