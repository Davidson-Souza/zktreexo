// Written in 2024 by Davidson Souza.
// SPDX-License-Identifier: MIT

/// A simple program that can prove that a UTXO is in the UTXO set, without revealing the actual
/// UTXO. This is a toy project, and **should not be used for real purposes**.
///
/// To acchieve this, we use two distinct tools:
///
/// 1 - To prove that a UTXO exists, without requiring access to the whole UTXO set, we use utreexo. In
///     this implementation we use rustreexo (https://github.com/mit-dci/rustreexo).
/// 2 - The actual ZK proof is generated using RISC-0 (https://risczero.com), a tool for generating
///     STARKS proofs for the exectution of RISC-V programs. The program that will be executed
///     inside the zk vm is inside `methods/guest/src/main`, and it just verify a utreexo proof
use std::{path::PathBuf, str::FromStr};

use clap::Parser;
use hex::FromHex;
use log::info;
use risc0_zkvm::{
    default_prover,
    guest::sha::Impl,
    sha::{Digest, Sha256},
    ExecutorEnv, Receipt,
};
use rustreexo::accumulator::{node_hash::NodeHash, proof::Proof, stump::Stump};
use serde::Deserialize;
use serde::Serialize;

use methods::{GUEST_CODE_FOR_ZK_PROOF_ELF, GUEST_CODE_FOR_ZK_PROOF_ID};

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cmd = Cli::parse();

    match cmd {
        Cli::Prove {
            utreexo_proof,
            leaf_hash,
            utreexo_acc,
            output,
        } => {
            let acc: CliStump = serde_json::from_str(&utreexo_acc)?;
            let acc = Stump {
                leaves: acc.leaves,
                roots: acc
                    .roots
                    .into_iter()
                    .map(|root| NodeHash::from_str(&root).expect("invalid hash"))
                    .collect(),
            };

            let proof: CliProof = serde_json::from_str(&utreexo_proof)?;
            let proof = Proof {
                targets: proof.targets,
                hashes: proof
                    .hashes
                    .into_iter()
                    .map(|root| NodeHash::from_str(&root).expect("invalid hash"))
                    .collect(),
            };

            info!(
                "Your id is {}, you'll need this to verify the proof",
                Digest::from(GUEST_CODE_FOR_ZK_PROOF_ID)
            );
            let leaf_hash = NodeHash::from_str(&leaf_hash)?;
            let env = ExecutorEnv::builder()
                .write(&proof)?
                .write(&acc)?
                .write(&leaf_hash)?
                .build()
                .unwrap();

            // Obtain the default prover.
            let prover = default_prover();

            // Produce a receipt by proving the specified ELF binary.
            let receipt = prover.prove_elf(env, GUEST_CODE_FOR_ZK_PROOF_ELF)?;
            std::fs::write(output.clone(), serde_json::to_string(&receipt)?)?;
            info!(
                "Proof writen to {}",
                output.to_str().expect("should be a valid dir")
            );
        }

        Cli::Verify {
            witness,
            session_id,
        } => {
            let witness = std::fs::read_to_string(witness).unwrap();
            let receipt: Receipt = serde_json::from_str(&witness).unwrap();

            receipt.verify(Digest::from_hex(session_id)?)?;
            println!("Everything looks great ⚡")
        }
    }

    Ok(())
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
enum Cli {
    Prove {
        utreexo_proof: String,
        utreexo_acc: String,
        leaf_hash: String,
        output: PathBuf,
    },
    Verify {
        witness: PathBuf,
        session_id: String,
    },
}

#[derive(Deserialize, Serialize)]
struct CliProof {
    pub targets: Vec<u64>,
    pub hashes: Vec<String>,
}

#[derive(Deserialize, Serialize)]
struct CliStump {
    pub roots: Vec<String>,
    pub leaves: u64,
}
