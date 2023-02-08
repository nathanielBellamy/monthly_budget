use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("data/init/"))]
    pub input: String,
    #[arg(short, long, default_value_t = String::from("data/"))]
    pub output: String,
    #[arg(short, long, default_value_t = String::from("example_1/"))]
    pub transactions: String,

    #[arg(short, long)]
    pub endym: String,
    #[arg(short, long)]
    pub startym: String,
}