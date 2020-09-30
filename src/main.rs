use clap::Clap;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

const LCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &'static [u8] = b"0123456789";
const SYMBOLS: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

#[derive(Clap, Debug)]
#[clap(
    name = "rpwgen",
    version = "1.0.0",
    author = "Kotaro Yamashita",
    about = "Generats Strings for Password."
)]
struct Opts {
    /// Length for each password strings
    #[clap(name = "LENGTH", short = "L", long, default_value = "12")]
    length: usize,

    /// Number of generating password strings
    #[clap(name = "NUM", short = "N", long, default_value = "10")]
    num: usize,

    /// No lowercase letters
    #[clap(name = "LOWERCASE", short = "l", long)]
    lower_case: bool,

    /// No uppercase letters
    #[clap(name = "UPPERCASE", short = "u", long)]
    upper_case: bool,

    /// No digits
    #[clap(name = "DIGITS", short = "d", long)]
    digits: bool,

    /// No symbols
    #[clap(name = "SYMBOLS", short = "s", long)]
    symbols: bool,
}

fn main() {
    let mut rng = rand::thread_rng();
    let opts: Opts = Opts::parse();
    let total_length: usize = opts.length;
    let lc: bool = opts.lower_case;
    let uc: bool = opts.upper_case;
    let di: bool = opts.digits;
    let sy: bool = opts.symbols;

    println!("{}", "");

    for _ in 0..(opts.num) {
        let (num_lc, num_uc, num_di, num_sy) = decide_num_of_extructs(total_length, lc, uc, di, sy);

        let mut chars_vec: Vec<u8> = Vec::new();

        chars_vec.append(&mut extruct_chars_vec(num_lc, "lc"));
        chars_vec.append(&mut extruct_chars_vec(num_uc, "uc"));
        chars_vec.append(&mut extruct_chars_vec(num_di, "di"));
        chars_vec.append(&mut extruct_chars_vec(num_sy, "sy"));
        chars_vec.shuffle(&mut rng);

        println!("{}", String::from_utf8(chars_vec).unwrap());
    }

    println!("{}", "");
}

// 各文字グループから抽出する文字数をタプルとして返す
fn decide_num_of_extructs(
    total_length: usize,
    lc: bool,
    uc: bool,
    di: bool,
    sy: bool,
) -> (usize, usize, usize, usize) {
    let num_false: usize = vec![lc, uc, di, sy].iter().filter(|&x| !*x).count(); // 抽出する文字グループの数
    let base_length: usize = total_length / num_false; // 抽出数の平均

    let mut num_lc: usize = 0;
    let mut num_uc: usize = 0;
    let mut num_di: usize = 0;
    let mut num_sy: usize = 0;
    let mut remaining_groups = num_false;
    let mut remaining_length = total_length;

    const SIGMA2: f64 = 2.0; // 抽出数の分散

    if !lc {
        if remaining_groups > 1 {
            num_lc = get_length(1, remaining_length - remaining_groups, base_length, SIGMA2);
            remaining_groups -= 1;
            remaining_length -= num_lc;
        } else {
            num_lc = remaining_length;
        }
    }

    if !uc {
        if remaining_groups > 1 {
            num_uc = get_length(1, remaining_length - remaining_groups, base_length, SIGMA2);
            remaining_groups -= 1;
            remaining_length -= num_uc;
        } else {
            num_uc = remaining_length;
        }
    }

    if !di {
        if remaining_groups > 1 {
            num_di = get_length(1, remaining_length - remaining_groups, base_length, SIGMA2);
            remaining_groups -= 1;
            remaining_length -= num_di;
        } else {
            num_di = remaining_length;
        }
    }

    if !sy {
        if remaining_groups > 1 {
            num_sy = get_length(1, remaining_length - remaining_groups, base_length, SIGMA2);
        } else {
            num_sy = remaining_length;
        }
    }

    (num_lc, num_uc, num_di, num_sy)
}

// 正規分布から抽出数を得る
fn get_length(min: usize, max: usize, mu: usize, sigma2: f64) -> usize {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mu as f64, (sigma2 as f64).sqrt()).unwrap();
    let num = normal.sample(&mut rng);

    if num < min as f64 {
        return min;
    } else if num > max as f64 {
        return max;
    } else {
        return num as usize;
    }
}

// 指定した抽出数、文字グループから抽出する
fn extruct_chars_vec(length: usize, char_group: &str) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut v: Vec<u8> = Vec::new();
    // let mut char_group: &'static [u8];
    let target: &[u8];
    match char_group {
        "lc" => {
            target = LCASE;
        }
        "uc" => {
            target = UCASE;
        }
        "di" => {
            target = DIGITS;
        }
        "sy" => {
            target = SYMBOLS;
        }
        _ => panic!(""),
    }

    for _ in 0..length {
        let c: u8 = *target.choose(&mut rng).unwrap();
        v.push(c)
    }

    v
}
