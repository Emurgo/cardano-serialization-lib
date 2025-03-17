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

export const getStakeKeyDeregCertWithExplicitRefund = (
  stakeCred: CSL.Credential,
  deposit: string,
): CSL.StakeDeregistration => CSL.StakeDeregistration.new_with_explicit_refund(stakeCred, strToBigNum(deposit));

export const getStakeKeyDeregCert = (stakeCred: CSL.Credential): CSL.StakeDeregistration =>
  CSL.StakeDeregistration.new(stakeCred);

export const getCertOfNewStakeDereg = (stakeKeyRegCert: CSL.StakeDeregistration): CSL.Certificate =>
  CSL.Certificate.new_stake_deregistration(stakeKeyRegCert);

const buildDeregStakeKey = (
  stakeKeyHash: string,
  useConway: boolean,
  stakeRefundAmount: string,
): CSL.CertificatesBuilder => {
  const certBuilder = getCertificateBuilder();
  const stakeCred = getCslCredentialFromHex(stakeKeyHash);
  let stakeKeyDeregCert: CSL.StakeDeregistration;
  if (useConway) {
    stakeKeyDeregCert = getStakeKeyDeregCertWithExplicitRefund(stakeCred, stakeRefundAmount);
  } else {
    stakeKeyDeregCert = getStakeKeyDeregCert(stakeCred);
  }
  certBuilder.add(getCertOfNewStakeDereg(stakeKeyDeregCert));

  return certBuilder;
};

export const createTxWithStakeDeregistrationCert = async (
  cardanoApi: CardanoApiType,
  useConway: boolean,
  stakeRefundAmount: string,
) => {
  const txBuilder = getTxBuilder();
  const regPubStakeKeyHash = await cardanoApi.cip95.getRegisteredPubStakeKeys();
  console.log('[StakingKeyDeregistrar] regPubStakeKeyHash:', regPubStakeKeyHash);
  if (regPubStakeKeyHash.length < 1) {
    throw new Error(`Your wallet public stake key is not registered`);
  }
  const unregPubStakeKey = regPubStakeKeyHash[0];
  const stakeKeyHash = getPublicKeyFromHex(unregPubStakeKey).hash().to_hex();
  const certBuilder = buildDeregStakeKey(stakeKeyHash, useConway, stakeRefundAmount);
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
