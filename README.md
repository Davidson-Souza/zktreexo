## RISC-0 utreexo proof verification

> Do not use this in serious stuff, I don't know what I'm doing.

This is a minimal zk implementation for Utreexo proofs. It let's you prove that a UTXO exists in the 
UTXO set, without reveling which UTXO is being proved. It **does not** prove possesion over the UTXO,
therefore, anyone can prove any UTXO.

## Running

To run this, you'll need:
 - The accumulator for a given block
 - A proof for a given Utxo

With that in hands, you can generate the proofs by running:

```bash
cargo run --release -- prove <proof_json> <acc_json> <leaf_hash> <output>
```

where: 

<proof_json> is the json representation of a proof, looks like this:

```json
{
    "targets": [Number],    // The targets we're proving, should be only one for now
    "hashes": [String]      // The hashes needed to prove this utxo
}
```

<acc_json> is the json representation of a stump. This should look like this:

```json
{
    "roots": [String],  // Current acc roots
    "leaves": Number    // how many leaves in the current acc
}
```

<leaf_hash> is the hash of the leaf we're proving. This is define in the utreexo protocol and should
be the sha256 of the serialized leaf_data as follows:

> UTREEXO_TAG_V1 | UTREEXO_TAG_V1 | block_hash | txid | vout | header_code | amount | spk

Where:

    - UTREEXO_TAG_V1 it's just the sha512 hash of the string `UtreexoV1` 
      represented as a vector of [u8] ([85 116 114 101 101 120 111 86 49])
    - block_hash is the block that included this tx
    - txid is the id of the transaction that created that utxo
    - vout is the index of this utxo inside the tx
    - header code is the height of the block containing this tx, left-shifted by one an OR-ed
      by whether the utxo is a coinbase or not
    - amount is a u64 for the amount of satoshis in this output
    - spk is the consensus-encoded scriptPubkey for this output

To find more about this leaf, check out [this](https://github.com/Davidson-Souza/Floresta/blob/124440d581c1ccb1293b178b2fc6b6fc109255bb/crates/floresta-chain/src/pruned_utreexo/consensus.rs#L70) code.

## Example

Here's an example proving in a test accumulator. This case comes from rustreexo's testcases:

```bash
RUST_LOG="info" cargo run --release -- prove '{"targets": [3], "hashes":["dbc1b4c900ffe48d575b5da5c638040125f65db0fe3e24494b76ea986457d986", "02242b37d8e851f1e86f46790298c7097df06893d6226b7c1453c213e91717de"]}' '{"roots": ["df46b17be5f66f0750a4b3efa26d4679db170a72d41eb56c3e4ff75a58c65386", "9eec588c41d87b16b0ee226cb38da3864f9537632321d8be855a73d5616dcc73"], "leaves": 6}' 084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5 out.txt
```

This should output a 32-bytes id that you should store somewhere. It also writes a giant json inside 
out.txt. This json is your proof!

You can verify the proof with:

```bash
RUST_LOG="info" cargo run --release -- verify out.txt <id the prover gave you>
```

If you want to test verifying, I'm leaving the proof for the above example in proof.json.sample, the id 
is `03476bc80aa398cbdf5ded471db60e6730f096ac4b51ed540bb8e1331af9cbe3`. Verify with:

```bash
RUST_LOG="info" cargo run --release -- verify proof.json.sample 03476bc80aa398cbdf5ded471db60e6730f096ac4b51ed540bb8e1331af9cbe3 
```
