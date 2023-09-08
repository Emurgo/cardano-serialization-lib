use crate::fakes::{fake_anchor_data_hash, fake_key_hash, fake_script_hash, fake_tx_hash};
use crate::*;

#[test]
fn anchor_ser_round_trip() {
    let anchor = Anchor::new(
        &URL::new("https://iohk.io".to_string()).unwrap(),
        &fake_anchor_data_hash(1),
    );

    let cbor = anchor.to_bytes();
    let cbor_hex = anchor.to_hex();
    let json = anchor.to_json().unwrap();

    assert_eq!(anchor, Anchor::from_bytes(cbor).unwrap());
    assert_eq!(anchor, Anchor::from_hex(&cbor_hex).unwrap());
    assert_eq!(anchor, Anchor::from_json(&json).unwrap());
}

#[test]
fn drep_key_hash_ser_round_trip() {
    let drep = DRep::new_key_hash(&fake_key_hash(1));

    let cbor = drep.to_bytes();
    let cbor_hex = drep.to_hex();
    let json = drep.to_json().unwrap();

    assert_eq!(drep, DRep::from_bytes(cbor).unwrap());
    assert_eq!(drep, DRep::from_hex(&cbor_hex).unwrap());
    assert_eq!(drep, DRep::from_json(&json).unwrap());
    assert_eq!(drep.kind(), DRepKind::KeyHash);
}

#[test]
fn drep_script_hash_ser_round_trip() {
    let drep = DRep::new_script_hash(&fake_script_hash(1));

    let cbor = drep.to_bytes();
    let cbor_hex = drep.to_hex();
    let json = drep.to_json().unwrap();

    assert_eq!(drep, DRep::from_bytes(cbor).unwrap());
    assert_eq!(drep, DRep::from_hex(&cbor_hex).unwrap());
    assert_eq!(drep, DRep::from_json(&json).unwrap());
    assert_eq!(drep.kind(), DRepKind::ScriptHash);
}

#[test]
fn drep_always_abstain_ser_round_trip() {
    let drep = DRep::new_always_abstain();

    let cbor = drep.to_bytes();
    let cbor_hex = drep.to_hex();
    let json = drep.to_json().unwrap();

    assert_eq!(drep, DRep::from_bytes(cbor).unwrap());
    assert_eq!(drep, DRep::from_hex(&cbor_hex).unwrap());
    assert_eq!(drep, DRep::from_json(&json).unwrap());
    assert_eq!(drep.kind(), DRepKind::AlwaysAbstain);
}

#[test]
fn drep_always_no_confidence_ser_round_trip() {
    let drep = DRep::new_always_no_confidence();

    let cbor = drep.to_bytes();
    let cbor_hex = drep.to_hex();
    let json = drep.to_json().unwrap();

    assert_eq!(drep, DRep::from_bytes(cbor).unwrap());
    assert_eq!(drep, DRep::from_hex(&cbor_hex).unwrap());
    assert_eq!(drep, DRep::from_json(&json).unwrap());
    assert_eq!(drep.kind(), DRepKind::AlwaysNoConfidence);
}

#[test]
fn governance_action_id_ser_round_trip() {
    let gov_action_id =
        GovernanceActionId::new(&fake_tx_hash(1), GovernanceActionIndex::from(42u32));

    let cbor = gov_action_id.to_bytes();
    let cbor_hex = gov_action_id.to_hex();
    let json = gov_action_id.to_json().unwrap();

    assert_eq!(gov_action_id, GovernanceActionId::from_bytes(cbor).unwrap());
    assert_eq!(
        gov_action_id,
        GovernanceActionId::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(gov_action_id, GovernanceActionId::from_json(&json).unwrap());
}

#[test]
fn voter_constitutional_committee_hot_key_hash_ser_round_trip() {
    let voter =
        Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&fake_key_hash(1)));

    let cbor = voter.to_bytes();
    let cbor_hex = voter.to_hex();
    let json = voter.to_json().unwrap();

    assert_eq!(voter, Voter::from_bytes(cbor).unwrap());
    assert_eq!(voter, Voter::from_hex(&cbor_hex).unwrap());
    assert_eq!(voter, Voter::from_json(&json).unwrap());
    assert_eq!(voter.kind(), VoterKind::ConstitutionalCommitteeHotKeyHash);
}

#[test]
fn voter_constitutional_committee_hot_script_hash_ser_round_trip() {
    let voter = Voter::new_constitutional_committee_hot_key(&Credential::from_scripthash(
        &fake_script_hash(1),
    ));

    let cbor = voter.to_bytes();
    let cbor_hex = voter.to_hex();
    let json = voter.to_json().unwrap();

    assert_eq!(voter, Voter::from_bytes(cbor).unwrap());
    assert_eq!(voter, Voter::from_hex(&cbor_hex).unwrap());
    assert_eq!(voter, Voter::from_json(&json).unwrap());
    assert_eq!(
        voter.kind(),
        VoterKind::ConstitutionalCommitteeHotScriptHash
    );
}

#[test]
fn voter_drep_key_hash_ser_round_trip() {
    let voter = Voter::new_drep(&Credential::from_keyhash(&fake_key_hash(1)));

    let cbor = voter.to_bytes();
    let cbor_hex = voter.to_hex();
    let json = voter.to_json().unwrap();

    assert_eq!(voter, Voter::from_bytes(cbor).unwrap());
    assert_eq!(voter, Voter::from_hex(&cbor_hex).unwrap());
    assert_eq!(voter, Voter::from_json(&json).unwrap());
    assert_eq!(voter.kind(), VoterKind::DRepKeyHash);
}

#[test]
fn voter_drep_script_hash_ser_round_trip() {
    let voter = Voter::new_drep(&Credential::from_scripthash(&fake_script_hash(1)));

    let cbor = voter.to_bytes();
    let cbor_hex = voter.to_hex();
    let json = voter.to_json().unwrap();

    assert_eq!(voter, Voter::from_bytes(cbor).unwrap());
    assert_eq!(voter, Voter::from_hex(&cbor_hex).unwrap());
    assert_eq!(voter, Voter::from_json(&json).unwrap());
    assert_eq!(voter.kind(), VoterKind::DRepScriptHash);
}

#[test]
fn voter_staking_pool_ser_round_trip() {
    let voter = Voter::new_staking_pool(&fake_key_hash(1));

    let cbor = voter.to_bytes();
    let cbor_hex = voter.to_hex();
    let json = voter.to_json().unwrap();

    assert_eq!(voter, Voter::from_bytes(cbor).unwrap());
    assert_eq!(voter, Voter::from_hex(&cbor_hex).unwrap());
    assert_eq!(voter, Voter::from_json(&json).unwrap());
    assert_eq!(voter.kind(), VoterKind::StakingPoolKeyHash);
}

#[test]
fn voting_procedure_no_ser_round_trip() {
    let voting_procedure = VotingProcedure::new(VoteKind::No);

    let cbor = voting_procedure.to_bytes();
    let cbor_hex = voting_procedure.to_hex();
    let json = voting_procedure.to_json().unwrap();

    assert_eq!(voting_procedure, VotingProcedure::from_bytes(cbor).unwrap());
    assert_eq!(
        voting_procedure,
        VotingProcedure::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(voting_procedure, VotingProcedure::from_json(&json).unwrap());
    assert_eq!(voting_procedure.vote_kind(), VoteKind::No);
}

#[test]
fn voting_procedure_yes_ser_round_trip() {
    let voting_procedure = VotingProcedure::new(VoteKind::Yes);

    let cbor = voting_procedure.to_bytes();
    let cbor_hex = voting_procedure.to_hex();
    let json = voting_procedure.to_json().unwrap();

    assert_eq!(voting_procedure, VotingProcedure::from_bytes(cbor).unwrap());
    assert_eq!(
        voting_procedure,
        VotingProcedure::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(voting_procedure, VotingProcedure::from_json(&json).unwrap());
    assert_eq!(voting_procedure.vote_kind(), VoteKind::Yes);
}

#[test]
fn voting_procedure_abstain_ser_round_trip() {
    let voting_procedure = VotingProcedure::new(VoteKind::Abstain);

    let cbor = voting_procedure.to_bytes();
    let cbor_hex = voting_procedure.to_hex();
    let json = voting_procedure.to_json().unwrap();

    assert_eq!(voting_procedure, VotingProcedure::from_bytes(cbor).unwrap());
    assert_eq!(
        voting_procedure,
        VotingProcedure::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(voting_procedure, VotingProcedure::from_json(&json).unwrap());
    assert_eq!(voting_procedure.vote_kind(), VoteKind::Abstain);
}

#[test]
fn voting_procedures_single_item_ser_round_trip() {
    let mut voting_procedures = VotingProcedures::new();

    voting_procedures.insert(
        &Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&fake_key_hash(1))),
        &GovernanceActionId::new(&fake_tx_hash(1), GovernanceActionIndex::from(42u32)),
        &VotingProcedure::new(VoteKind::Yes),
    );

    let cbor = voting_procedures.to_bytes();
    let cbor_hex = voting_procedures.to_hex();
    let json = voting_procedures.to_json().unwrap();

    assert_eq!(
        voting_procedures,
        VotingProcedures::from_bytes(cbor).unwrap()
    );
    assert_eq!(
        voting_procedures,
        VotingProcedures::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(
        voting_procedures,
        VotingProcedures::from_json(&json).unwrap()
    );
}

#[test]
fn voting_procedures_muiltiple_items_ser_round_trip() {
    let mut voting_procedures = VotingProcedures::new();

    voting_procedures.insert(
        &Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&fake_key_hash(1))),
        &GovernanceActionId::new(&fake_tx_hash(1), GovernanceActionIndex::from(42u32)),
        &VotingProcedure::new(VoteKind::Yes),
    );

    voting_procedures.insert(
        &Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&fake_key_hash(2))),
        &GovernanceActionId::new(&fake_tx_hash(2), GovernanceActionIndex::from(43u32)),
        &VotingProcedure::new(VoteKind::No),
    );

    voting_procedures.insert(
        &Voter::new_constitutional_committee_hot_key(&Credential::from_keyhash(&fake_key_hash(3))),
        &GovernanceActionId::new(&fake_tx_hash(3), GovernanceActionIndex::from(44u32)),
        &VotingProcedure::new(VoteKind::Abstain),
    );

    let cbor = voting_procedures.to_bytes();
    let cbor_hex = voting_procedures.to_hex();
    let json = voting_procedures.to_json().unwrap();

    assert_eq!(
        voting_procedures,
        VotingProcedures::from_bytes(cbor).unwrap()
    );
    assert_eq!(
        voting_procedures,
        VotingProcedures::from_hex(&cbor_hex).unwrap()
    );
    assert_eq!(
        voting_procedures,
        VotingProcedures::from_json(&json).unwrap()
    );
}
