#![no_main]
use risc0_zkvm::guest::env;

use rustreexo::accumulator::node_hash::NodeHash;
use rustreexo::accumulator::{proof::Proof, stump::Stump};

risc0_zkvm::guest::entry!(main);

pub fn main() {
    let proof: Proof = env::read();
    let acc: Stump = env::read();
    
    // TODO: Pass in the leaf data and build the hash here?
    let leaf_hash: NodeHash = env::read();
    acc.verify(&proof, &[leaf_hash]).unwrap();
       
    // TODO(Davidson): Maybe commit to some hash of the leaf?
    //                 We can't commit to the leaf hash itself, as it would kill the whole
    //                 'zk over the leaf being proved', but maybe, if we pay the cost of oppening
    //                 the commitment hash, we can use private data (like the private key) inside
    //                 the prover, but transaparent to the verifyer
    
    // Commit to the accumulator used for this proof. We need this because you can prove anything
    // if you can pick any acc. We use this in our verification program to check if the acc used 
    // is valid.
    env::commit(&acc);
}
