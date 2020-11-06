extern crate rand;
use rand::Rng;
use crate::args::ComArgs;

const LOWERCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UPPERCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const INTEGERS: &'static [u8] = b"0123456789";
const SYMBOL: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

pub fn generate_pwd(arg: &ComArgs) {
    let mut rng = rand::thread_rng();
    for _ in 0..arg.num {
        let mut pwd: Vec<char> = Vec::new();
        pwd.extend_from_slice(&generate_sub_pwd(arg.length, &LOWERCASE));

        if !arg.upper {
            pwd.extend_from_slice(&generate_sub_pwd(arg.length, &UPPERCASE));
        }

        if !arg.int {
            pwd.extend_from_slice(&generate_sub_pwd(arg.length, &INTEGERS));
        }

        if !arg.symbol {
            pwd.extend_from_slice(&generate_sub_pwd(arg.length, &SYMBOL));
        }

        rng.shuffle(&mut pwd);
        let (left, _right) = pwd.split_at(arg.length);
        let password: String = left.into_iter().collect();
        println!("{}", password);
    }
}

fn generate_sub_pwd(count: usize, set: &[u8]) -> Vec<char> {
    let mut rng = rand::thread_rng();
    (0..count).map(|_| *rng.choose(set).unwrap() as char).collect::<Vec<char>>()
}
