import React, { useState } from "react";
import * as CSL from "@emurgo/cardano-serialization-lib-browser";
import { protocolParams } from "../utils/networkConfig";

const TransactionCreator: React.FC = () => {
  const [transactionHex, setTransactionHex] = useState<string>("");

  const strToBigNum = (numberInStr: string) => CSL.BigNum.from_str(numberInStr);

  const createTransaction = () => {
    const privateKey = CSL.PrivateKey.generate_ed25519();
    const publicKey = privateKey.to_public();

    const txBuilder = CSL.TransactionBuilder.new(
      CSL.TransactionBuilderConfigBuilder.new()
        .fee_algo(
          CSL.LinearFee.new(
            strToBigNum(protocolParams.linearFee.minFeeA),
            strToBigNum(protocolParams.linearFee.minFeeB)
          )
        )
        .pool_deposit(strToBigNum(protocolParams.poolDeposit))
        .key_deposit(strToBigNum(protocolParams.keyDeposit))
        .coins_per_utxo_byte(
          strToBigNum(
            Math.floor(
              parseFloat(protocolParams.coinsPerUtxoWord) / 8
            ).toString(10)
          )
        )
        .max_value_size(protocolParams.maxValueSize)
        .max_tx_size(protocolParams.maxTxSize)
        .ex_unit_prices(
          CSL.ExUnitPrices.new(
            CSL.UnitInterval.new(strToBigNum("577"), strToBigNum("10000")),
            CSL.UnitInterval.new(strToBigNum("721"), strToBigNum("10000000"))
          )
        )
        .build()
    );

    const address = CSL.BaseAddress.new(
      0,
      CSL.Credential.from_keyhash(publicKey.hash()),
      CSL.Credential.from_keyhash(publicKey.hash())
    ).to_address();

    const value = CSL.Value.new(strToBigNum("1000000"));
    txBuilder.add_output(CSL.TransactionOutput.new(address, value));
    txBuilder.add_change_if_needed(address);

    const unsignedTx = txBuilder.build_tx();
    const txHash = CSL.TransactionHash.from_hex(unsignedTx.to_hex());
    const witness = CSL.make_vkey_witness(txHash, privateKey);
    const witnessSet = CSL.TransactionWitnessSet.from_hex(witness.to_hex());
    const transaction = CSL.Transaction.new(unsignedTx.body(), witnessSet);

    setTransactionHex(transaction.to_hex());
  };

  return (
    <div>
      <h2>Создание транзакции</h2>
      <button onClick={createTransaction}>Создать транзакцию</button>
      {transactionHex && (
        <div>
          <h3>Транзакция (hex):</h3>
          <p>{transactionHex}</p>
        </div>
      )}
    </div>
  );
};

export default TransactionCreator;
