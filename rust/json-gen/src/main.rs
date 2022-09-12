use std::fs;
use std::path::Path;

use cardano_serialization_lib::*;
use cardano_serialization_lib::address::*;
use cardano_serialization_lib::crypto::*;
use cardano_serialization_lib::metadata::*;
use cardano_serialization_lib::plutus::*;
use cardano_serialization_lib::utils::*;

//#[macro_export]
macro_rules! gen_json_schema {
    ($name:ident) => {
        //let out_dir = std::env::var_os("OUT_DIR").expect("no env");
        let dest_path = Path::new(&"schemas").join(&format!("{}.json", stringify!($name)));
        fs::write(&dest_path, serde_json::to_string_pretty(&schemars::schema_for!($name)).unwrap()).unwrap();
    }
}

fn main() {
    let schema_path = Path::new(&"schemas");
    if !schema_path.exists() {
        fs::create_dir(schema_path).unwrap();
    }
    // lib.rs
    gen_json_schema!(UnitInterval);
    gen_json_schema!(Transaction);
    gen_json_schema!(TransactionInputs);
    gen_json_schema!(TransactionOutputs);
    gen_json_schema!(Certificates);
    gen_json_schema!(TransactionBody);
    gen_json_schema!(TransactionInput);
    gen_json_schema!(TransactionOutput);
    gen_json_schema!(StakeRegistration);
    gen_json_schema!(StakeDeregistration);
    gen_json_schema!(StakeDelegation);
    gen_json_schema!(Ed25519KeyHashes);
    gen_json_schema!(Relays);
    gen_json_schema!(PoolParams);
    gen_json_schema!(PoolRegistration);
    gen_json_schema!(PoolRetirement);
    gen_json_schema!(GenesisKeyDelegation);
    gen_json_schema!(MoveInstantaneousRewardsCert);
    gen_json_schema!(Certificate);
    //gen_json_schema!(CertificateEnum);
    gen_json_schema!(MIRPot);
    gen_json_schema!(MIRToStakeCredentials);
    gen_json_schema!(MoveInstantaneousReward);
    gen_json_schema!(MIREnum);
    gen_json_schema!(Ipv4);
    gen_json_schema!(Ipv6);
    gen_json_schema!(URL);
    gen_json_schema!(DNSRecordAorAAAA);
    gen_json_schema!(DNSRecordSRV);
    gen_json_schema!(SingleHostAddr);
    gen_json_schema!(SingleHostName);
    gen_json_schema!(MultiHostName);
    gen_json_schema!(Relay);
    //gen_json_schema!(RelayEnum);
    gen_json_schema!(PoolMetadata);
    gen_json_schema!(StakeCredentials);
    gen_json_schema!(RewardAddresses);
    gen_json_schema!(Withdrawals);
    gen_json_schema!(TransactionWitnessSet);
    gen_json_schema!(ScriptPubkey);
    gen_json_schema!(ScriptAll);
    gen_json_schema!(ScriptAny);
    gen_json_schema!(ScriptNOfK);
    gen_json_schema!(TimelockStart);
    gen_json_schema!(TimelockExpiry);
    gen_json_schema!(NativeScript);
    //gen_json_schema!(NativeScriptEnum);
    gen_json_schema!(NativeScripts);
    gen_json_schema!(Update);
    gen_json_schema!(GenesisHashes);
    gen_json_schema!(ScriptHashes);
    gen_json_schema!(ProposedProtocolParameterUpdates);
    gen_json_schema!(ProtocolVersion);
    gen_json_schema!(ProtocolParamUpdate);
    gen_json_schema!(TransactionBodies);
    gen_json_schema!(TransactionWitnessSets);
    gen_json_schema!(AuxiliaryDataSet);
    gen_json_schema!(Block);
    gen_json_schema!(Header);
    gen_json_schema!(OperationalCert);
    gen_json_schema!(HeaderBody);
    gen_json_schema!(AssetName);
    gen_json_schema!(AssetNames);
    gen_json_schema!(Assets);
    gen_json_schema!(MultiAsset);
    gen_json_schema!(MintAssets);
    gen_json_schema!(Mint);
    gen_json_schema!(NetworkId);
    gen_json_schema!(NetworkIdKind);
    gen_json_schema!(ScriptRef);
    gen_json_schema!(DataOption);
    gen_json_schema!(HeaderLeaderCertEnum);

    // crypto.rs
    gen_json_schema!(PublicKey);
    gen_json_schema!(Vkey);
    //gen_json_schema!(Vkeys);
    gen_json_schema!(Vkeywitness);
    gen_json_schema!(Vkeywitnesses);
    gen_json_schema!(BootstrapWitness);
    gen_json_schema!(BootstrapWitnesses);
    gen_json_schema!(Ed25519Signature);
    gen_json_schema!(Ed25519KeyHash);
    gen_json_schema!(ScriptHash);
    gen_json_schema!(TransactionHash);
    gen_json_schema!(GenesisDelegateHash);
    gen_json_schema!(GenesisHash);
    gen_json_schema!(AuxiliaryDataHash);
    gen_json_schema!(PoolMetadataHash);
    gen_json_schema!(VRFKeyHash);
    gen_json_schema!(BlockHash);
    gen_json_schema!(DataHash);
    gen_json_schema!(ScriptDataHash);
    gen_json_schema!(VRFVKey);
    gen_json_schema!(KESVKey);
    gen_json_schema!(Nonce);
    gen_json_schema!(VRFCert);
    // address.rs
    gen_json_schema!(StakeCredential);
    gen_json_schema!(StakeCredType);
    gen_json_schema!(Address);
    gen_json_schema!(RewardAddress);
    // plutus.rs
    gen_json_schema!(PlutusScript);
    gen_json_schema!(PlutusScripts);
    gen_json_schema!(ConstrPlutusData);
    gen_json_schema!(CostModel);
    gen_json_schema!(Costmdls);
    gen_json_schema!(ExUnitPrices);
    gen_json_schema!(ExUnits);
    gen_json_schema!(Language);
    gen_json_schema!(LanguageKind);
    gen_json_schema!(Languages);
    gen_json_schema!(PlutusMap);
    gen_json_schema!(PlutusData);
    gen_json_schema!(PlutusList);
    gen_json_schema!(PlutusData);
    //gen_json_schema!(PlutusDataEnum);
    gen_json_schema!(Redeemer);
    gen_json_schema!(RedeemerTag);
    gen_json_schema!(RedeemerTagKind);
    gen_json_schema!(Redeemers);
    //gen_json_schema!(Strings);
    // metadata.rs
    gen_json_schema!(TransactionMetadatum);
    gen_json_schema!(GeneralTransactionMetadata);
    gen_json_schema!(AuxiliaryData);
    // utils.rs
    gen_json_schema!(BigNum);
    gen_json_schema!(BigInt);
    gen_json_schema!(Int);
    gen_json_schema!(Value);
    gen_json_schema!(TransactionUnspentOutput);
}
