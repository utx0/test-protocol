mod utils;

use anchor_client::Cluster;
use structopt::StructOpt;
use crate::utils::load_keypair;

#[derive(Debug,StructOpt)]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: SubCommand,
}

#[derive(Debug, StructOpt)]
pub enum  SubCommand {
    Farming {
        #[structopt(subcommand)]
        scmd: FarmingSubCommand,
    },
    Multisig {
        #[structopt(subcommand)]
        scmd: MultiSigSubCommand,
    },
    Pooling {
        #[structopt(subcommand)]
        scmd: PoolingSubCommand,
    },
    Staking {
        #[structopt(subcommand)]
        scmd: StakingSubCommand,
    }
}

#[derive(Debug,StructOpt)]
pub enum FarmingSubCommand {
    DepositLPTokens,
    WithDrawLPTokens
}

#[derive(Debug,StructOpt)]
pub enum MultiSigSubCommand {
    CreateMultiSig,
    CreateTx,
}

#[derive(Debug,StructOpt)]
pub enum PoolingSubCommand {
    DepositTokens,
    WithdrawTokens,
    SwapTokens,
}

#[derive(Debug,StructOpt)]
pub enum StakingSubCommand {
    DepositTokens,
    WithDrawTokens,
}

const DEFAULT_KEYPAIR: &str = "~/.config/solana/id.json";

fn main() -> anyhow::Result<()> {
    let exe = Opt::from_args();
    println!("exe: {:?}", exe);

    let keypair = load_keypair(DEFAULT_KEYPAIR);
    let cluster = Cluster::default();

    let program_id = match exe.cmd {
        SubCommand::Pooling { .. } => pooling::ID,
        SubCommand::Farming { .. } => farming::ID,
        SubCommand::Multisig { .. } => multisig::ID,
        SubCommand::Staking { .. } => staking::ID,
    };



    Ok(())
}
