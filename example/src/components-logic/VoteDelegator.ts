import * as CSL from '@emurgo/cardano-serialization-lib-browser';
import { CardanoApiType } from '../context/CardanoContext';
import {
  getAddressFromBytes,
  getCertificateBuilder,
  getCslCredentialFromHex,
  getCslUtxos,
  getLargestFirstMultiAsset,
  getPublicKeyFromHex,
  getTransactionOutput,
  getTxBuilder,
  keyHashFromBech32,
  keyHashFromHex,
  scriptHashFromBech32,
  scriptHashFromHex,
} from './core';
import { bytesToHex } from '../utils/helpers';

const getDRepAbstain = () => CSL.DRep.new_always_abstain();

const getDRepNoConfidence = () => CSL.DRep.new_always_no_confidence();

const getDRepFromBech32 = (dRepIdBech32: string) => CSL.DRep.from_bech32(dRepIdBech32);

const getDRepNewKeyHash = (keyHash: CSL.Ed25519KeyHash) => CSL.DRep.new_key_hash(keyHash);

const getDRepNewScriptKeysHash = (scriptHash: CSL.ScriptHash) => CSL.DRep.new_script_hash(scriptHash);

const getVoteDelegCert = (stakeCred: CSL.Credential, dRepKeyHash: CSL.DRep) =>
  CSL.VoteDelegation.new(stakeCred, dRepKeyHash);

const dRepToCredentialHex = (dRepId: string): CSL.DRep => {
  const isPotentiallyValidHex = /^(22|23)[0-9a-fA-F]{56}$/.test(dRepId);
  try {
    if (dRepId.startsWith('drep1')) {
      if (dRepId.length === 58) {
        // CIP129 drep1 encoding is extended value with internal prefix
        return getDRepFromBech32(dRepId);
      }
      // Pre CIP129 drep1 encoding means same as drep_vkh1 now
      return getDRepNewKeyHash(keyHashFromBech32(dRepId));
    }
    if (dRepId.startsWith('drep_vkh1')) {
      return getDRepNewKeyHash(keyHashFromBech32(dRepId));
    }
    if (dRepId.startsWith('drep_script1')) {
      return getDRepNewScriptKeysHash(scriptHashFromBech32(dRepId));
    }
    if (isPotentiallyValidHex && dRepId.startsWith('22')) {
      return getDRepNewKeyHash(keyHashFromHex(dRepId.substring(2)));
    }
    if (isPotentiallyValidHex && dRepId.startsWith('23')) {
      return getDRepNewScriptKeysHash(scriptHashFromHex(dRepId.substring(2)));
    }

    try {
      return getDRepNewKeyHash(keyHashFromHex(dRepId));
    } catch (error1) {
      return getDRepNewScriptKeysHash(scriptHashFromHex(dRepId));
    }
  } catch (error) {
    console.error(`Error in parsing credential: ${error}`);
    throw new Error(`Error in parsing credential: ${error}`);
  }
};

const getDRepFromInput = (input: string): CSL.DRep => {
  const cleanedInput = input.trim();
  if (cleanedInput.toUpperCase() === 'ABSTAIN') {
    return getDRepAbstain();
  } else if (cleanedInput.toUpperCase() === 'NO CONFIDENCE') {
    return getDRepNoConfidence();
  } else {
    return dRepToCredentialHex(cleanedInput);
  }
};

const getCertOfNewVoteDelegation = (voteCert: CSL.VoteDelegation) => CSL.Certificate.new_vote_delegation(voteCert);

export const createTxWithVoteDelegationCert = async (cardanoApi: CardanoApiType, voteChoice: string) => {
  const dRepObject = getDRepFromInput(voteChoice);
  const regStakePubKeysHash = await cardanoApi.cip95.getRegisteredPubStakeKeys();
  if (regStakePubKeysHash.length < 1) {
    console.error('The wallet staking key should be registered before voting');
    throw new Error('The wallet staking key should be registered before voting');
  }
  const regPubStakeKey = regStakePubKeysHash[0];
  const stakeKeyHash = getPublicKeyFromHex(regPubStakeKey).hash().to_hex();
  const stakeCred = getCslCredentialFromHex(stakeKeyHash);
  // build Vote delegation certificate
  const voteDelegation = getVoteDelegCert(stakeCred, dRepObject);
  const newVoteDelegationCertificate = getCertOfNewVoteDelegation(voteDelegation);

  // cert builder
  const certBuilder = getCertificateBuilder();
  certBuilder.add(newVoteDelegationCertificate);

  const txBuilder = getTxBuilder();
  txBuilder.set_certs_builder(certBuilder);
  const changeAddress = await cardanoApi.getChangeAddress();
  const cslChangeAddress = getAddressFromBytes(changeAddress);

  let addressBytes;
  const usedAddresses = await cardanoApi.getUsedAddresses();
  if (usedAddresses.length === 0) {
    const unusedAddresses = await cardanoApi.getUnusedAddresses();
    if (unusedAddresses.length === 0) {
      throw new Error('There are no used or unused addresses in the connected wallet');
    }
    addressBytes = unusedAddresses[0];
  }
  addressBytes = usedAddresses[0];
  const cslOutputAddress = getAddressFromBytes(addressBytes);

  const hexUtxos = await cardanoApi.getUtxos();

  txBuilder.add_output(getTransactionOutput(cslOutputAddress, '1000000'));
  const cslUtxos = getCslUtxos(hexUtxos);
  txBuilder.add_inputs_from(cslUtxos, getLargestFirstMultiAsset());
  txBuilder.add_change_if_needed(cslChangeAddress);
  const cslUnsignedTransaction = txBuilder.build_tx();
  return bytesToHex(cslUnsignedTransaction.to_bytes());
};
