import * as CSL from '@emurgo/cardano-serialization-lib-browser';
import { getTxBuilder, getAddressFromBytes, getAddressFromBech32, getCslUtxos, getTransactionOutput } from './core';
import { CardanoApiType } from '../context/CardanoContext';
import { bytesToHex } from '../utils/helpers';

export type BuildTransactionInputType = {
  amount: string;
  address: string;
  sendAll: boolean;
};

export const createTransaction = async (
  cardanoApi: CardanoApiType | null | undefined,
  buildTransactionInput: BuildTransactionInputType,
) => {
  if (!cardanoApi) {
    throw new Error('A Cardano wallet is not connected');
  }
  const txBuilder = getTxBuilder();

  const changeAddress = await cardanoApi.getChangeAddress();
  const cslChangeAddress = getAddressFromBytes(changeAddress);

  const cslOutputAddress = buildTransactionInput.address
    ? getAddressFromBech32(buildTransactionInput.address)
    : cslChangeAddress;

  const hexUtxos = await cardanoApi.getUtxos();
  const cslUtxos = getCslUtxos(hexUtxos);

  if (buildTransactionInput.sendAll) {
    for (let i = 0; i < cslUtxos.len(); i++) {
      const cslUtxo = cslUtxos.get(i);
      const output = cslUtxo.output();
      txBuilder.add_regular_input(output.address(), cslUtxo.input(), output.amount());
    }

    // Sending everything to the receiver
    txBuilder.add_change_if_needed(cslOutputAddress);
  } else {
    const cslOutput = getTransactionOutput(cslOutputAddress, buildTransactionInput.amount);
    txBuilder.add_output(cslOutput);

    txBuilder.add_inputs_from(cslUtxos, CSL.CoinSelectionStrategyCIP2.LargestFirstMultiAsset);
    txBuilder.add_change_if_needed(cslChangeAddress);
  }

  const cslUnsignedTransaction = txBuilder.build_tx();

  return bytesToHex(cslUnsignedTransaction.to_bytes());
};
