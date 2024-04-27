use clap::{Parser, Subcommand};
use lib::Blockchain;
#[derive(Parser)]
#[command(name = "blockchain-rust", version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a block with some data
    Addblock {
        /// the data stored in the block
        #[arg(short, long, value_name = "DATA")]
        data: String,
    },
    /// Print data of blockchain
    Printchain,
}
impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Addblock { data } => {
                cmd_add_block(data);
            }
            Commands::Printchain => {
                cmd_print_chain();
            }
        }
    }
}

fn cmd_add_block(data: &str) {
    let mut blockchain = Blockchain::new().unwrap();
    let block = blockchain.add_block(data).unwrap();
    println!("Block added with hash: {}", block.hash);
}

fn cmd_print_chain() {
    let blockchain = Blockchain::new().unwrap();
    for block in blockchain.iter() {
        println!(
            "Block Hash:{}\nBlock Data:{}\nPrev Block Hash:{}\n\n",
            block.hash, block.data, block.prev_block_hash
        );
    }
}
