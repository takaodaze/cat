use clap::Parser;
#[derive(Parser, Debug)]
#[clap(author, version, about)]
struct Args {
    file_name: String,
}
fn main() {
    let args = Args::parse();
    println!("{:?}", args);
}
