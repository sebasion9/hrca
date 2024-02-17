use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name : Option<String>,
    #[arg(long, short = 'H')]
    pub header : Option<Vec<String>>,

    #[arg(long, short = 'M')]
    pub method : Option<String>,

    #[arg(long, short)]
    pub url : String,

    #[command(subcommand)]
    pub command : Option<Commands>

}

#[derive(Subcommand)]
pub enum Commands {
    /// does testing things
    Test {
        /// lists test values
        #[arg(short, long)]
        list : bool
    }
}



