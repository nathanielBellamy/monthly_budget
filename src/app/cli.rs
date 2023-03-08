use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("data"))]
    pub path: String,
    #[arg(short, long)]
    pub end_yyyy_mm: String,
    #[arg(short, long)]
    pub start_yyyy_mm: String,

    #[arg(short, long, default_value_t = String::from("f"))]
    pub x_test: String, // avoid cli name collision
}

impl Cli {
    pub fn new(path: String, start_yyyy_mm: String, end_yyyy_mm: String, x_test: String) -> Cli {
        Cli {
            path,
            end_yyyy_mm,
            start_yyyy_mm,
            x_test,
        }
    }
}
