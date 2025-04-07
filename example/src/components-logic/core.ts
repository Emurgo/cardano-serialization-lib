import * as CSL from '@emurgo/cardano-serialization-lib-browser';
import { protocolParams } from '../utils/networkConfig';
import { hexToBytes } from '../utils/helpers';

export const strToBigNum = (numberInStr: string) => CSL.BigNum.from_str(numberInStr);

export const getRandomPrivAndPubKeys = () => {
  const privateKey = CSL.PrivateKey.generate_ed25519();
  const publicKey = privateKey.to_public();

  return {
    privateKey,
    publicKey,
  };
};

export const getTxBuilder = () =>
  CSL.TransactionBuilder.new(
    CSL.TransactionBuilderConfigBuilder.new()
      .fee_algo(
        CSL.LinearFee.new(strToBigNum(protocolParams.linearFee.minFeeA), strToBigNum(protocolParams.linearFee.minFeeB)),
      )
      .pool_deposit(strToBigNum(protocolParams.poolDeposit))
      .key_deposit(strToBigNum(protocolParams.keyDeposit))
      .coins_per_utxo_byte(strToBigNum(Math.floor(parseFloat(protocolParams.coinsPerUtxoWord) / 8).toString(10)))
      .max_value_size(protocolParams.maxValueSize)
      .max_tx_size(protocolParams.maxTxSize)
      .ex_unit_prices(
        CSL.ExUnitPrices.new(
          CSL.UnitInterval.new(strToBigNum('577'), strToBigNum('10000')),
          CSL.UnitInterval.new(strToBigNum('721'), strToBigNum('10000000')),
        ),
      )
      .build(),
  );

export const getAddress = (paymentKeyHash: CSL.Ed25519KeyHash, stakeKeyHash: CSL.Ed25519KeyHash) =>
  CSL.BaseAddress.new(
    0,
    CSL.Credential.from_keyhash(paymentKeyHash),
    CSL.Credential.from_keyhash(stakeKeyHash),
  ).to_address();

export const getAddressFromBytes = (changeAddressHex: string) => CSL.Address.from_bytes(hexToBytes(changeAddressHex));

export const getAddressFromBech32 = (addressBech32: string) => CSL.Address.from_bech32(addressBech32);

export const getCslUtxos = (utxosHex: Array<string>) => {
  const cslUtxos = CSL.TransactionUnspentOutputs.new();
  for (const utxoHex of utxosHex) {
    const cslUtxo = CSL.TransactionUnspentOutput.from_bytes(hexToBytes(utxoHex));
    cslUtxos.add(cslUtxo);
  }

  return cslUtxos;
};

export const getTransactionOutput = (cslOutputAddress: CSL.Address, txAmountLovelaces: string) => {
  return CSL.TransactionOutput.new(cslOutputAddress, CSL.Value.new(strToBigNum(txAmountLovelaces)));
};

export const getCredential = (keyHash: CSL.Ed25519KeyHash) => CSL.Credential.from_keyhash(keyHash);

export const getPublicKeyFromHex = (publicKeyHex: string) => CSL.PublicKey.from_hex(publicKeyHex)

export const keyHashFromHex = (hexValue: string) => CSL.Ed25519KeyHash.from_hex(hexValue);

export const keyHashFromBech32 = (bech32Value: string) => CSL.Ed25519KeyHash.from_bech32(bech32Value);

export const scriptHashFromBech32 = (bech32Value: string) => CSL.ScriptHash.from_bech32(bech32Value);

export const scriptHashFromHex = (hexValue: string) => CSL.ScriptHash.from_hex(hexValue);

export const getCslCredentialFromHex = (hexValue: string) => {
  const keyHash = keyHashFromHex(hexValue);
  return getCredential(keyHash);
};

export const getCslCredentialFromBech32 = (bech32Value: string) => {
  const keyHash = keyHashFromBech32(bech32Value);
  return getCredential(keyHash);
};

export const getLargestFirstMultiAsset = () => CSL.CoinSelectionStrategyCIP2.LargestFirstMultiAsset;

export const getCertificateBuilder = () => CSL.CertificatesBuilder.new();
