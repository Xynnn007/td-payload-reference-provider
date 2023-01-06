//! A simple tool to calculate td-payload's reference value due to given kernel

use clap::{arg, command};
use sha2::Digest;
use std::path::Path;

pub const KERNEL_SIZE: &str = "33554432";

fn main() {
    let matches = command!()
        .arg(
            arg!(-k --kernel "vmlinuz kernel")
                .required(true)
                .takes_value(true)
                .allow_invalid_utf8(false),
        )
        .arg(
            arg!(-s --"kernel-size" "KERNEL_SIZE of the target td-shim")
                .required(false)
                .default_value(KERNEL_SIZE),
        )
        .get_matches();
    let path = matches.value_of("kernel").unwrap();
    let path = Path::new(path).to_path_buf();
    let siz: u64 = matches.value_of("kernel-size").unwrap().parse().unwrap();
    let file_size = std::fs::metadata(&path).unwrap().len();
    if file_size > siz {
        panic!("File size should be less than `kernel-size`");
    }

    let diff = siz - file_size;
    let mut buf = std::fs::read(path).unwrap();
    buf.extend_from_slice(&vec![0; diff as usize]);
    let mut hasher = sha2::Sha384::new();
    hasher.update(&buf);
    let res = hasher.finalize();
    let hex_res = hex::encode(res);
    println!("{hex_res}");
}
