//! A very simple example used as a self-test of this library against a Bitcoin
//! Core node.
extern crate bitcoin;
extern crate bitcoincore_rpc;

use bitcoincore_rpc::{Client, Error};

fn main() -> Result<(), Error> {
    let mut args = std::env::args();

    let _exe_name = args.next().unwrap();

    let url = args.next().expect("Usage: <rpc_url> [username] [password]");
    let user = args.next();
    let pass = args.next();

    let mut rpc = Client::new(url, user, pass);

    let _blockchain_info = rpc.get_blockchain_info()?;

    let best_block_hash = rpc.get_best_block_hash()?;
    println!("best block hash: {}", best_block_hash);
    let bestblockcount = rpc.get_block_count()?;
    println!("best block height: {}", bestblockcount);
    let best_block_hash_by_height = rpc.get_block_hash(bestblockcount)?;
    println!("best block hash by height: {}", best_block_hash_by_height);
    assert_eq!(best_block_hash_by_height, best_block_hash);

    let bitcoin_block: bitcoin::Block = rpc.get_by_id(&best_block_hash)?;
    println!("best block hash by `get`: {}", bitcoin_block.header.prev_blockhash);
    let bitcoin_tx: bitcoin::Transaction = rpc.get_by_id(&bitcoin_block.txdata[0].txid())?;
    println!("tx by `get`: {}", bitcoin_tx.txid());

    Ok(())
}
