use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short, long, default_value_t = String::from("data/init/"))]
    pub input: String,
    #[arg(short, long, default_value_t = String::from("data/"))]
    pub output: String,
    #[arg(short, long, default_value_t = String::from("example_1"))]
    pub payment_events: String,

    #[arg(short, long)]
    pub endym: String,
    #[arg(short, long)]
    pub startym: String,
}

impl Cli {
    pub fn new(
        input: String,
        output: String,
        payment_events: String,
        startym: String,
        endym: String,
    ) -> Cli {
        Cli {
            input,
            output,
            payment_events,
            endym,
            startym
        }
    }
}
