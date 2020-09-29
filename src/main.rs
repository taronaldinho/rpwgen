use clap::Clap;
use rand::seq::SliceRandom;
use rand_distr::{Normal, Distribution};


const LCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
const UCASE: &'static [u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ";
const DIGITS: &'static [u8] = b"0123456789";
const SYMBOLS: &'static [u8] = b"!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";


#[derive(Clap, Debug)]
#[clap(
    name = "rpwgen",
    version = "0.0.1",
    author = "Kotaro Yamashita",
    about = "Generats Strings for Password."
)]
struct Opts {
    #[clap(name="LENGTH", short="L", long, default_value="12")]
    length: usize,

    #[clap(name="NUM", short="N", long, default_value="10")]
    num: usize,

    #[clap(name="LOWERCASE", short="l", long)]
    lower_case: bool,
    
    #[clap(name="UPPERCASE", short="u", long)]
    upper_case: bool,
    
    #[clap(name="DIGITS", short="d", long)]
    digits: bool,
    
    #[clap(name="SYMBOLS", short="s", long)]
    symbols: bool,    
}



fn main() {      
    let mut rng = rand::thread_rng(); 
    let opts: Opts = Opts::parse();    
    let total_length:usize = opts.length;
    let lc: bool = opts.lower_case;
    let uc: bool = opts.upper_case;
    let di: bool = opts.digits;
    let sy: bool = opts.symbols;
    
    for _ in 0..(opts.num) {
        let (num_lc, num_uc, num_di, num_sy) 
            = decide_num_of_extructs(total_length, lc, uc, di, sy);

        let mut chars_vec: Vec<u8> = Vec::new();

        chars_vec.append(&mut extruct_chars_vec(num_lc, "lc"));
        chars_vec.append(&mut extruct_chars_vec(num_uc, "uc"));
        chars_vec.append(&mut extruct_chars_vec(num_di, "di"));
        chars_vec.append(&mut extruct_chars_vec(num_sy, "sy"));
        chars_vec.shuffle(&mut rng);

        println!("{}", String::from_utf8(chars_vec).unwrap());

    }
}


fn decide_num_of_extructs(total_length:usize, lc: bool, uc: bool, di: bool, sy: bool) 
        -> (usize, usize, usize, usize) {

    // 各文字グループから抽出する文字数をタプルとして返す

    let num_true: usize = vec![lc, uc, di, sy].iter().filter(|&x| *x).count(); // 抽出する文字グループの数
    let base_length: usize = total_length / num_true;

    let mut num_lc: usize = 0;
    let mut num_uc: usize = 0;
    let mut num_di: usize = 0;
    let mut num_sy: usize = 0;
    let mut remaining_groups = num_true;
    let mut remaining_length = total_length;
    
    if lc {
        if remaining_groups > 1 {
            num_lc = get_length(1, remaining_length-remaining_groups, base_length, 2.0);
            remaining_groups -= 1;
            remaining_length -= num_lc;
        } else {
            num_lc = remaining_length;
        }        
    }
    
    if uc {
        if remaining_groups > 1 {
            num_uc = get_length(1, remaining_length-remaining_groups, base_length, 2.0);
            remaining_groups -= 1;
            remaining_length -= num_uc;
        } else {
            num_uc = remaining_length;
        }        
    }
    
    if di {
        if remaining_groups > 1 {
            num_di = get_length(1, remaining_length-remaining_groups, base_length, 2.0);
            remaining_groups -= 1;
            remaining_length -= num_di;
        } else {
            num_di = remaining_length;
        }        
    }
    
    if sy {
        if remaining_groups > 1 {
            num_sy = get_length(1, remaining_length-remaining_groups, base_length, 2.0);
        } else {
            num_sy = remaining_length;
        }        
    }

    (num_lc, num_uc, num_di, num_sy)
}

fn get_length(min: usize, max: usize, mu: usize, sigma2: f64) -> usize {
    let mut rng = rand::thread_rng();
    let normal = Normal::new(mu as f64, (sigma2 as f64).sqrt()).unwrap();
    let num = normal.sample(&mut rng);

    if num < min as f64 {
        return min
    } else if num > max as f64 {
        return max
    } else {
        return num as usize
    }
}


fn extruct_chars_vec(length: usize, char_group: &str) -> Vec<u8> {
    let mut rng = rand::thread_rng();    
    let mut v: Vec<u8> = Vec::new();
    // let mut char_group: &'static [u8];
    let target: &[u8];
    match char_group {
        "lc" => { target = LCASE; },
        "uc" => { target = UCASE; },
        "di" => { target = DIGITS; },
        "sy" => { target = SYMBOLS; },
        _ => panic!(""),
    }

    for _ in 0..length {
        let c: u8 = *target.choose(&mut rng).unwrap();
        v.push(c)
    };
    v
}
