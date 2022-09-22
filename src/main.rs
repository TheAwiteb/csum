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

use std::path::Path;

use cli::parser::ALGORITHMS;

mod cli;
mod utils;

fn main() {
    let args = cli::parser::parser();
    let verify_with = args.get_one::<String>("verify");
    let is_plaintext = args.get_flag("plaintext");
    let in_uppercase = args.get_flag("upper");
    let file = args.get_one::<String>("FILE").expect("Is required");
    let algorithms: Vec<&str> = args
        .get_raw("algorithms")
        .map(|raw| {
            raw.map(|alg| alg.to_str().expect("Is has been validated"))
                .collect()
        })
        .unwrap_or_else(|| ALGORITHMS.to_vec());
    if let Some(check) = verify_with {
        utils::checksum(Path::new(file), algorithms, check)
    } else {
        utils::print_hashs(Path::new(file), algorithms, is_plaintext, in_uppercase)
    }
}
