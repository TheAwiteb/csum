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

use super::validators::check_file_existing;
use clap::{arg, builder::StringValueParser, command, ArgMatches};

pub const ALGORITHMS: [&str; 22] = [
    "sha-1",
    "sha2-256",
    "sha2-224",
    "sha2-384",
    "sha2-512",
    "sha3-256",
    "sha3-512",
    "blake",
    "blake2",
    "blake2s",
    "blake3",
    "crc64",
    "crc32c",
    "crc32",
    "crc16",
    "crc8",
    "md5",
    "md6-128",
    "md6-256",
    "md6-512",
    "whirlpool",
    "xor8",
];

pub fn parser() -> ArgMatches {
    command!()
        .arg(
            arg!([FILE] "File to get a sum of it")
                .required(true)
                .value_parser(check_file_existing),
        )
        .arg(
            arg!(-a --algorithms "Checksum algorithms")
                .value_parser(StringValueParser::new())
                .possible_values(ALGORITHMS)
                .ignore_case(true)
                .multiple_values(true),
        )
        .arg(arg!(-v --verify "Verify if your hash match any hash").value_name("HASH"))
        .arg(
            arg!(-p --plaintext "Print a checksums without colors")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("verify"),
        )
        .arg(
            arg!(-u --upper "Return a hash in uppercase")
                .action(clap::ArgAction::SetTrue)
                .conflicts_with("verify"),
        )
        .get_matches()
}
