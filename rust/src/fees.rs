use super::*;

// constants - are these right?
const ABSTRACT_SIZE_VKEY: usize = 64; // from 
const ABSTRACT_SIZE_SIG: usize = 64;
const HL: usize = 16; // really unsure about this one, found https://github.com/input-output-hk/cardano-base/blob/501d91e426ae84ce0ae056be38bd3db594af9fc2/cardano-crypto-class/src/Cardano/Crypto/Hash/MD5.hs#L16
const HASH_OBJ: usize = 2 + HL;
// CBOR-encoding specific contants I believe
const UINT: usize = 5;
const ARRAY_PREFIX: usize = 2;
const SMALL_ARRAY: usize = 1;
const MAP_PREFIX: usize = 2;
const LABEL_SIZE: usize = 1;
const CBOR_TAG: usize = 2;
const ADDRESS: usize = 2 * HASH_OBJ;
const CREDENTIAL: usize = LABEL_SIZE + HASH_OBJ;
const UNIT_INTERVAL: usize = CBOR_TAG + SMALL_ARRAY + UINT + UINT;
const FEE_SIZE: usize = LABEL_SIZE + UINT;
const TTL_SIZE: usize = LABEL_SIZE + UINT;

fn multisig_node_count(root: &Script) -> usize {
    match &root.0 {
        ScriptEnum::ScriptKeyNode(_) => 1,
        ScriptEnum::ScriptAllOfNode(ScriptAllOfNode { scripts }) => 1usize + scripts.0.iter().map(multisig_node_count).sum::<usize>(),
        _ => 2,
    }
}

fn cert_size(cert: &DelegationCertificate) -> usize {
    match &cert.0 {
        DelegationCertificateEnum::StakeKeyReg(_) |
        DelegationCertificateEnum::StakeScriptKeyReg(_) => SMALL_ARRAY + LABEL_SIZE + HASH_OBJ,
        DelegationCertificateEnum::StakeKeyDereg(_) |
        DelegationCertificateEnum::StakeScriptKeyDereg(_) => SMALL_ARRAY + LABEL_SIZE + HASH_OBJ,
        DelegationCertificateEnum::StakeDeleg(_) |
        DelegationCertificateEnum::StakeScriptDeleg(_) => SMALL_ARRAY + LABEL_SIZE + 2 * HASH_OBJ,
        DelegationCertificateEnum::PoolRegistration(PoolRegistration{ pool_params, .. }) => {
            SMALL_ARRAY + LABEL_SIZE + HASH_OBJ
            // pool owners
            + CBOR_TAG + ARRAY_PREFIX + pool_params.owners.len() * HASH_OBJ
            // cost
            + UINT
            // margin
            + UNIT_INTERVAL
            // pledge
            + UINT
            // operator
            + HASH_OBJ
            // vrf keyhash
            + HASH_OBJ
            // reward acc
            + CREDENTIAL
            // relays (String::len() is in bytes already, no need for utf8 conversion)
            + pool_params.relays.0.iter().map(|url| url.len()).sum::<usize>()
            // metadata (String::len() is in bytes, and metadat should only have 1 element)
            + SMALL_ARRAY + pool_params.metadata.0.first().map_or(0, |pmd| pmd.url.len())
        },
        DelegationCertificateEnum::PoolRetirement(_) => SMALL_ARRAY + LABEL_SIZE + UINT + HASH_OBJ,
        DelegationCertificateEnum::GenesisKeyDeleg(_) => SMALL_ARRAY + 2 * LABEL_SIZE + HASH_OBJ,
        DelegationCertificateEnum::MoveRewardsCert(mir) => {
            SMALL_ARRAY + LABEL_SIZE + MAP_PREFIX + mir.move_instantaneous_reward.len() * (UINT + HASH_OBJ)
        },
    }
}

#[wasm_bindgen]
pub fn txsize(tx: &TransactionBody, witness: &TransactionWitnessSet) -> usize {
    // It seems that in Haskell we have Tx = {Body, vkeySigs, msigScripts, metadata}
    // but in the CDDL we have Body as one then vkeySigs/msigScripts as another
    // as the data is segwit.
    // We might want to make our own version that wraps these then.
    
    // witness size
    let signatures = witness.key_witnesses.as_ref().map_or(0, |keys| keys.len());
    let script_nodes = witness.script_witnesses.as_ref().map_or(0, |scripts| scripts.0.iter().map(multisig_node_count).sum());
    let witness_size = signatures * (ABSTRACT_SIZE_VKEY + ABSTRACT_SIZE_SIG)
                     + HASH_OBJ * script_nodes
                     + SMALL_ARRAY + LABEL_SIZE + MAP_PREFIX
                     + HASH_OBJ * witness.script_witnesses.as_ref().map_or(0, Scripts::len);

    let input_size = LABEL_SIZE + CBOR_TAG + ARRAY_PREFIX
                   + tx.inputs.len() * (SMALL_ARRAY + UINT + HASH_OBJ);
    let output_size = LABEL_SIZE + ARRAY_PREFIX
                    + tx.outputs.len() * (SMALL_ARRAY + UINT + ADDRESS);
    let certs_size = tx.certs.as_ref().map_or(0, |certs| certs.0.iter().map(cert_size).sum::<usize>());
    let withdrawals_size = LABEL_SIZE + MAP_PREFIX
                         + tx.withdrawals.as_ref().map_or(0, |wd| wd.len()) * (UINT + CREDENTIAL);
    // TODO: updates
    let update_size = 0;
    // TODO: metdata/hash
    let metadata_size = ARRAY_PREFIX;
    let metdata_hash_size = ARRAY_PREFIX;
    
    input_size + output_size + certs_size + withdrawals_size + FEE_SIZE + TTL_SIZE + update_size + metdata_hash_size + witness_size + metadata_size
}