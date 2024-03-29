use clap::Clap;
use rand::seq::SliceRandom;
use rand_distr::{Distribution, Normal};

const LCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &'static [u8] = b"0123456789";
const SYMBOLS: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";
const SYMBOLS_BMWP: &'static [u8] = b"@#$%^*()_+=&-";

// clapによるコマンドライン引数設定
#[derive(Clap, Debug)]
#[clap(
    name = "rpwgen",
    version = "1.1.0",
    author = "Kotaro Yamashita",
    about = "Generate some password strings."
)]
struct Opts {
    /// Length for each password strings
    #[clap(
        name = "LENGTH",
        short = "L",
        long,
        default_value = "12",
        display_order = 1
    )]
    length: usize,

    /// Number of generating password strings
    #[clap(
        name = "NUM",
        short = "N",
        long,
        default_value = "10",
        display_order = 2
    )]
    num: usize,

    /// No lowercase letters
    #[clap(name = "LOWERCASE", short = "l", long, display_order = 3)]
    lower_case: bool,

    /// No uppercase letters
    #[clap(name = "UPPERCASE", short = "u", long, display_order = 4)]
    upper_case: bool,

    /// No digits
    #[clap(name = "DIGITS", short = "d", long, display_order = 5)]
    digits: bool,

    /// No symbols
    #[clap(name = "SYMBOLS", short = "s", long, display_order = 6)]
    symbols: bool,

    /// Select from a char set for Biz+Mail&Web
    #[clap(name = "BMWP", short = "B", long, display_order = 7)]
    symbols_for_bmwp: bool,
}

fn main() {
    let opts: Opts = Opts::parse();
    let lc: bool = opts.lower_case;
    let uc: bool = opts.upper_case;
    let di: bool = opts.digits;
    let sy: bool = opts.symbols;
    let sb: bool = opts.symbols_for_bmwp;

    if sb {
        println!(
            "[INFORMATION] If '-B' is turned on, other flags ('-l', '-u', '-d', and '-s') are ignored."
        );
    } else if lc && uc && di && sy {
        panic!(
            "
            The flags '-l', '-u', '-d', and '-s' are all turned on.
            At least one of them must be turned off.
            "
        );
    };

    let mut rng = rand::thread_rng();
    let total_length: usize = opts.length;

    println!("{}", "");
    for _ in 0..(opts.num) {
        let num_lc;
        let num_uc;
        let num_di;
        let num_sy;

        if sb {
            (num_lc, num_uc, num_di, num_sy) =
                decide_num_of_extructs(total_length, false, false, false, false);
        } else {
            (num_lc, num_uc, num_di, num_sy) = decide_num_of_extructs(total_length, lc, uc, di, sy);
        }

        let mut chars_vec: Vec<u8> = Vec::new();

        chars_vec.append(&mut extruct_chars_vec(num_lc, "lc"));
        chars_vec.append(&mut extruct_chars_vec(num_uc, "uc"));
        chars_vec.append(&mut extruct_chars_vec(num_di, "di"));
        if sb {
            chars_vec.append(&mut extruct_chars_vec(num_sy, "sb"));
        } else {
            chars_vec.append(&mut extruct_chars_vec(num_sy, "sy"));
        }

        chars_vec.shuffle(&mut rng);

        println!("{}", String::from_utf8(chars_vec).unwrap());
    }
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
        "sb" => {
            target = SYMBOLS_BMWP;
        }
        _ => panic!(""),
    }

    for _ in 0..length {
        let c: u8 = *target.choose(&mut rng).unwrap();
        v.push(c)
    }

    v
}


// 以下、関数の単体テスト
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decide_num_of_extructs() {
        let mut rng = rand::thread_rng();
        let truth_val = vec![true, false];

        for _ in 0..1000 {
            let mut lc = *truth_val.choose(&mut rng).unwrap();
            let mut uc = *truth_val.choose(&mut rng).unwrap();
            let mut di = *truth_val.choose(&mut rng).unwrap();
            let mut sy = *truth_val.choose(&mut rng).unwrap();

            if lc && uc && di && sy {
                let args = vec!["lc", "uc", "di", "sy"];
                match *args.choose(&mut rng).unwrap() {
                    "lc" => {
                        lc = false;
                    }
                    "uc" => {
                        uc = false;
                    }
                    "di" => {
                        di = false;
                    }
                    "sy" => {
                        sy = false;
                    }
                    _ => panic!(""),
                };
            }

            let (num_lc, num_uc, num_di, num_sy) = decide_num_of_extructs(10, lc, uc, di, sy);

            if lc {
                assert_eq!(num_lc, 0)
            } else {
                assert_ne!(num_lc, 0)
            };

            if uc {
                assert_eq!(num_uc, 0)
            } else {
                assert_ne!(num_uc, 0)
            };

            if di {
                assert_eq!(num_di, 0)
            } else {
                assert_ne!(num_di, 0)
            };

            if sy {
                assert_eq!(num_sy, 0)
            } else {
                assert_ne!(num_sy, 0)
            };
        }
    }
}
