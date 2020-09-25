use clap::Clap;
use rand::Rng;
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
    #[clap(name="LENGTH", short, long, default_value="12")]
    length: usize,

    // #[clap(name="LOWERCASE", short, long)]
    // lower_case: bool,
    
    // #[clap(name="UPPERCASE", short, long)]
    // upper_case: bool,
    
    // #[clap(name="DIGITS", short, long)]
    // digits: bool,
    
    // #[clap(name="SYMBOLS", short, long)]
    // symbols: bool,    
}

fn main() {        
    let opts: Opts = Opts::parse();
    let total_length:usize = opts.length;    
    
}


fn create_pw_string(length: usize) -> String {
    let mut rng = rand::thread_rng();    
    let mut v: Vec<u8> = Vec::new();
    for _ in 0..length {
        let c: u8 = *LCASE.choose(&mut rng).unwrap();
        v.push(c)
    }    
    String::from_utf8(v).unwrap()
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

fn decide_num_of_extructs(total_length:usize, lc: bool, uc: bool, di: bool, sy: bool) -> (usize, usize, usize, usize) {
    let mut rng = rand::thread_rng();
    let num_true: usize = vec![lc, uc, di, sy].iter().filter(|&x| *x).count();
    let base_length:usize = total_length / num_true;
    let mut remaining_length = total_length;

    if lc {
        let num_lc = get_length(1, remaining_length-num_true, base_length, 1.0);
        remaining_length -= num_lc;
    }
     
    if uc {
        let num_uc = get_length(1, remaining_length-num_true+1, base_length, 1.0);
        remaining_length -= num_uc;
    }

    let num_di = get_length(1, remaining_length-2, base_length, 1.0);
    remaining_length -= num_di;
    
    (num_lc, num_uc, num_di, num_sy)
}
