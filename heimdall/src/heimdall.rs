mod decompile;

use clap::{Parser, Subcommand};

use heimdall_common::{
    disassemble::{disassemble, DisassemblerArgs},
};

#[derive(Debug, Parser)]
#[clap(name = "heimdall", author = "Jonathan Becker <jonathan@jbecker.dev>", version)]
       
pub struct Arguments {
    #[clap(subcommand)]
    pub sub: Subcommands,
}

#[derive(Debug, Subcommand)]
#[clap(
    about = "Heimdall is an advanced Ethereum smart contract toolkit for forensic and heuristic analysis.",
    after_help = "For more information, read the wiki: https://jbecker.dev/r/heimdall-rs/wiki"
)]
#[allow(clippy::large_enum_variant)]
pub enum Subcommands {
    #[clap(name = "decompile", about = "Decompile EVM bytecode to Solidity")]
    Decompile(decompile::DecompilerArgs),

    #[clap(name = "disassemble", about = "Disassemble EVM bytecode to assembly")]
    Disassemble(DisassemblerArgs),

    #[clap(name = "config", about = "Display and edit the current configuration")]
    Config(decompile::DecompilerArgs),

    #[clap(name = "cache", about = "Manage cached files for Heimdall.")]
    Cache(decompile::DecompilerArgs),

}

fn main() {
    let args = Arguments::parse();
    
    match args.sub {
        Subcommands::Decompile(cmd) => {
            println!("{:#?}", cmd)
        }

        Subcommands::Disassemble(cmd) => {
            disassemble(cmd);
        }

        Subcommands::Config(cmd) => {
            println!("{:#?}", cmd)
        }

        Subcommands::Cache(cmd) => {
            println!("{:#?}", cmd)
        }
    }
}