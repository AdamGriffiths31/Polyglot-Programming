use clap::Parser;

fn main() {
    let opts = rust::opts::Opts::parse();
    println!("{:?}", opts);
}
