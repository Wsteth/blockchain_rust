use clap::{Parser, Subcommand};
use lib::{transaction::Transaction, Blockchain};
#[derive(Parser)]
#[command(name = "blockchain-rust", version, about)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Print data of blockchain
    Printchain,
    /// Create a blockchain
    Createblockchain {
        /// address
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
    /// Get balance
    Getbalance {
        /// address
        #[arg(short, long, value_name = "ADDRESS")]
        address: String,
    },
    /// Send coins
    Send {
        /// from
        #[arg(short, long, value_name = "ADDRESS")]
        from: String,
        /// to
        #[arg(short, long, value_name = "ADDRESS")]
        to: String,
        /// amount
        #[arg(short, long, value_name = "AMOUNT")]
        amount: u32,
    },
}
impl Cli {
    pub fn run() {
        let cli = Cli::parse();

        match &cli.command {
            Commands::Printchain => {
                cmd_print_chain();
            }
            Commands::Createblockchain { address } => {
                cmd_create_blockchain(address);
            }
            Commands::Getbalance { address } => {
                cmd_get_balance(address);
            }
            Commands::Send { from, to, amount } => {
                cmd_send(from, to, *amount);
            }
        }
    }
}

fn cmd_print_chain() {
    let blockchain = Blockchain::new().unwrap();
    for block in blockchain.iter() {
        println!(
            "Block Hash:{}\nTransactions:{:#?}\nPrev Block Hash:{}\n\n",
            block.hash, block.transactions, block.prev_block_hash
        );
    }
}

fn cmd_create_blockchain(address: &str) {
    Blockchain::create_blockchain(address).unwrap();
}

fn cmd_get_balance(address: &str) {
    let blockchain = Blockchain::new().unwrap();
    let utxos = blockchain.find_utxo(address);

    let mut balance = 0;
    for out in utxos {
        balance += out.value;
    }
    println!("Balance of {}: {}", address, balance);
}

fn cmd_send(from: &str, to: &str, amount: u32) {
    let mut blockchain = Blockchain::new().unwrap();

    let tx = Transaction::new_utxo_transaction(from, to, amount as i64, &blockchain);
    match blockchain.mine_block(vec![tx]) {
        Ok(_) => println!("Success!"),
        Err(_) => println!("Fail!"),
    };
}
