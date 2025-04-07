import * as CSL from '@emurgo/cardano-serialization-lib-browser';
import {
  getAddressFromBytes,
  getCertificateBuilder,
  getCslCredentialFromHex,
  getCslUtxos,
  getLargestFirstMultiAsset,
  getPublicKeyFromHex,
  getTransactionOutput,
  getTxBuilder,
  strToBigNum,
} from './core';
import { CardanoApiType } from '../context/CardanoContext';
import { bytesToHex } from '../utils/helpers';

export const getStakeKeyRegCertWithExplicitDeposit = (
  stakeCred: CSL.Credential,
  deposit: string,
): CSL.StakeRegistration => CSL.StakeRegistration.new_with_explicit_deposit(stakeCred, strToBigNum(deposit));

export const getStakeKeyRegCert = (stakeCred: CSL.Credential): CSL.StakeRegistration =>
  CSL.StakeRegistration.new(stakeCred);

export const getCertOfNewStakeReg = (stakeKeyRegCert: CSL.StakeRegistration): CSL.Certificate =>
  CSL.Certificate.new_stake_registration(stakeKeyRegCert);

const buildRegStakeKey = (
  stakeKeyHash: string,
  useConway: boolean,
  stakeDepositAmount: string,
): CSL.CertificatesBuilder => {
  const certBuilder = getCertificateBuilder();
  const stakeCred = getCslCredentialFromHex(stakeKeyHash);
  let stakeKeyRegCert: CSL.StakeRegistration;
  if (useConway) {
    stakeKeyRegCert = getStakeKeyRegCertWithExplicitDeposit(stakeCred, stakeDepositAmount);
  } else {
    stakeKeyRegCert = getStakeKeyRegCert(stakeCred);
  }
  certBuilder.add(getCertOfNewStakeReg(stakeKeyRegCert));

  return certBuilder;
};

export const createTxWithStakeRegistrationCert = async (
  cardanoApi: CardanoApiType,
  useConway: boolean,
  stakeDepositAmount: string,
) => {
  const txBuilder = getTxBuilder();
  const unregPubStakeKeyHash = await cardanoApi.cip95.getUnregisteredPubStakeKeys();
  console.log('[StakingKeyRegistrar] unregPubStakeKeyHash:', unregPubStakeKeyHash);
  if (unregPubStakeKeyHash.length < 1) {
    throw new Error(`Your wallet public stake key is already registered`);
  }
  const unregPubStakeKey = unregPubStakeKeyHash[0];
  const stakeKeyHash = getPublicKeyFromHex(unregPubStakeKey).hash().to_hex();
  const certBuilder = buildRegStakeKey(stakeKeyHash, useConway, stakeDepositAmount);
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
