use clap::{arg, Command};

use crate::blockchain::Blockchain;
use crate::errors::Result;
pub struct Cli {
    bc: Blockchain,
}

impl Cli {
    pub fn new() -> Result<Cli> {
        Ok(Cli{
            bc: Blockchain::new()?,
        })
    }
    pub fn run(&mut self) -> Result<()> {
        let matches = Command::new("blockchain-rust")
            .version("1.0")
            .author("EVANS KIPTENTEN")
            .subcommand(Command::new("printchain").about("print all the chain block"))
            .subcommand(
                Command::new("addblock")
                    .about("add a block to blockchain")
                    .arg(arg!(<Data>"'blockchain data'")),
            )
            .get_matches();
        if let Some(ref matches) = matches.subcommand_matches("addblock") {
            if let Some(c) = matches.get_one::<String>("DATA") {
                self.addblock(String::from(c))?;
            }else {
                println!("Not printing testing list .......");
            }
        }
        if let Some(_) = matches.subcommand_matches("printchain")  {
            self.print_chain();

        }
            
        Ok(())
           
    }
    fn addblock(&mut self, data: String) -> Result<()> {
        self.bc.add_block(data)
    }
    fn print_chain(&mut self) {
        for b in &mut self.bc.iter() {
            println!("block{:#?}", b);
        }
    }
            

            
}

