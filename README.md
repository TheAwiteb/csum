<div align="center">

# Csum
Simple Rust CLI to checksum

<a href="https://www.gnu.org/licenses/">
  <img src="https://img.shields.io/badge/license-GPLv3-orange.svg" alt="License">
</a>
<a href="https://rust-lang.org/">
  <img src="https://img.shields.io/badge/Made%20with-Rust-orange.svg" alt="Rust">
</a>
<br>
<a href="https://github.com/theawiteb/csum">
  <img src="https://badge.fury.io/gh/theawiteb%2Fcsum.svg" alt="version">
<br>
<a href="https://github.com/TheAwiteb/csum/actions/workflows/ci.yml">
  <img src="https://github.com/TheAwiteb/csum/actions/workflows/ci.yml/badge.svg" alt="Continuous Integration">
</a>
<br>
<a href="https://github.com/TheAwiteb/csum/actions/workflows/release.yml">
  <img src="https://github.com/TheAwiteb/csum/actions/workflows/release.yml/badge.svg" alt="Release">
</a>

</div>


## Requirements
 * [Rust](https://www.rust-lang.org/)

## Install
### With Cargo
```bash
cargo install csum
csum --version
```
### From source
```bash
# Clone the repo
git clone https://github.com/theawiteb/csum.git
# Change directory to it
cd csum
# Build it with cargo
cargo build --release
# Move the binary to `/usr/bin` (Unix like system) (need permission to move in `/usr/bin`)
# You can change binary directory to `~/.cargo/bin` if its exists and its in `$PATH`
sudo mv ./target/release/csum /usr/bin/csum
# Print the version
csum --version
```
### Binary
You can install binary file from [releases page](https://github.com/theawiteb/csum/releases/latest)

## Usage
```
USAGE:
    csum [OPTIONS] <FILE>

ARGS:
    <FILE>    File to get a sum of it

OPTIONS:
    -a, --algorithms <algorithms>...    Checksum algorithms [possible values: sha-1, sha2-256,
                                        sha2-224, sha2-384, sha2-512, sha3-256, sha3-512, blake,
                                        blake2, blake2s, blake3, crc64, crc32c, crc32, crc16, crc8,
                                        md5, md6-128, md6-256, md6-512, whirlpool, xor8]
    -h, --help                          Print help information
    -p, --plaintext                     Print a checksums without colors
    -u, --upper                         Return a hash in uppercase
    -v, --verify <HASH>                 Verify if your hash match any hash
    -V, --version                       Print version information
```

## Images

|Left|Right|
|:-:|:-:|
|![](https://i.suar.me/xWr7M/l)|![](https://i.suar.me/wXyxM/l)|
|![](https://i.suar.me/72Emm/l)|![](https://i.suar.me/VXGYr/l)|


## License
GNU General Public License version 3 of the license for more see <https://www.gnu.org/licenses/>
