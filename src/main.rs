use clap::Clap;
// use rand::Rng;
use rand::seq::SliceRandom;

const LCASE: &'static [u8] = b"abcdefghijklmnopqrstuvwxyz";
// const UCASE: &str  = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
// const DIGITS: &str = "0123456789";
// const SYMBOLS: &str = "!\"#$%&'()-=^~\\|@`[]{};:+*,./_<>?";

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

    #[clap(name="LOWERCASE", short, long)]
    lower_case: bool,
    
    #[clap(name="UPPERCASE", short, long)]
    upper_case: bool,
    
    #[clap(name="DIGITS", short, long)]
    digits: bool,
    
    #[clap(name="SYMBOLS", short, long)]
    symbols: bool,    
}

fn main() {    
    let mut rng = rand::thread_rng();
    let c = *LCASE.choose(&mut rng).unwrap();

    // let mut v =Vec::new();
    // for c in LC.chars() {
    //     v.push(c);
    // }
    // let opts: Opts = Opts::parse();
    // let index = opts.length;
    // let c = &v[index];
    println!("{}", c as char);    
}