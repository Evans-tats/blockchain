use std::process::exit;
use std::str::Matches;

use bitcoincash_addr::Address;
use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
use crate::transaction::Transaction;
pub struct Cli {
    
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli {})
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust-demo")
            .version("0.1")
            .author("EVANS KIPSANG KIPTENTEN")
            .about("blockchain in rust: a simple blockchain for learning")
            .subcommand(Command::new("printchain")
                .about("print all the chain blocks"))
            .subcommand(Command::new("getbalance")
                .about("get balane in the blockchain")
                .arg(arg!(<ADDRESS>"'The address it get blance for'"))
            )    
            .subcommand(Command::new("create").about("create new blockchain")
                .arg(arg!(<ADDRESS>"'The address to send genesis block reward to'"))
            )

            .subcommand(Command::new("send")
                .about("send in the blockchain")
                .arg(arg!(<FROM>"'Source wallet address'"))
                .arg(arg!(<TO>"'Destination wallet address'"))
                .arg(arg!(<AMOUNT>"'Destination wallet address'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("create") {
            if let Some(address) =  matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                Blockchain::create_blockchain(address.clone())?;
                println!("create_blockchain");
            }
            /*else {
                println!("not printing trsting list ......")
            } */
                
        }
        if let Some(ref matches) = matches.subcommand_matches("getbalance") {
            if let Some(address) = matches.get_one::<String>("ADDRESS") {
                let address = String::from(address);
                let bc = Blockchain::load_blocks()?;
                let utxos = bc.find_UTXO(&address);
                let mut balance  = 0;
                for out in utxos {
                    balance += out.value;
                }
                println!("Baloance of '{}'; '{}'",address,balance)
            }
            /*else {
                println!("Not printing testing list ...........")
            } */
        }

        if let Some(ref matches) = matches.subcommand_matches("send") {
            let from = if let Some(address) = matches.get_one::<String>("FROM") {
                address
            }else{
                println!("from not supply!: usage");
                exit(1)
            };
            let to = if let Some(address) = matches.get_one::<String>("TO") {
                address
            }else{
                println!("from not supply!: usage");
                exit(1)
            };
            let amount = if let Some(amount) = matches.get_one::<String>("AMOUNT") {
                amount.parse()?
            }else{
                println!("amaount not found");
                exit(1)

            };

            let mut bc = Blockchain::load_blocks()?;
            let tx = Transaction::new_UTXO(from, to, amount, &bc)?;
            bc.add_block(vec![tx])?;
            println!("success!");
        }

        if let Some(_) = matches.subcommand_matches("printchain") {
            let bc = Blockchain::load_blocks()?;
            for b in &mut bc.iter() {
                println!("block: {:#?}", b);
            }
        }
        Ok(())
    }
    
}
    
            


