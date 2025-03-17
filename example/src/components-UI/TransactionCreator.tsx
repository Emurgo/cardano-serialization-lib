import React, { useState } from 'react';
import { BuildTransactionInputType, createTransaction } from '../components-logic/TransactionCreator';
import useCardanoApi from '../context/CardanoContext';

const TransactionCreator: React.FC = () => {
  const { cardanoApi } = useCardanoApi();
  const [transactionHex, setTransactionHex] = useState<string>('');
  const [buildTransactionInput, setBuildTransactionInput] = useState<BuildTransactionInputType>({
    amount: '2000000',
    address: '',
    sendAll: false,
  });
  const [isPopupOpen, setIsPopupOpen] = useState<boolean>(false);
  const [loader, setLoader] = useState<boolean>(false);

  const openPopup = () => {
    console.log('[TransactionCreator][openPopup] is called');
    setIsPopupOpen(true);
  };

  const closePopup = () => {
    console.log('[TransactionCreator][closePopup] is called');
    setIsPopupOpen(false);
  };

  const createTransactionClick = async () => {
    if (buildTransactionInput.sendAll && !buildTransactionInput.address) {
      alert('Receiver address is required');
      throw new Error('Receiver address is required');
    } else {
      setLoader(true);
      try {
        const unsignedTxInHex = await createTransaction(cardanoApi, buildTransactionInput);
        setTransactionHex(unsignedTxInHex);
      } catch (error) {
        alert('Error received while builing the transaction. Please check the logs');
        console.error('Error received while builing the transaction.', error);
      } finally {
        setLoader(false);
        closePopup();
      }
    }
  };

  return (
    <div>
      <h2>Creating a simple transaction</h2>

      <button onClick={openPopup}>Create a transaction</button>

      {isPopupOpen && <div id="overlay" onClick={closePopup}></div>}

      {isPopupOpen && (
        <div id="popup">
          <h2>Simple transaction data</h2>

          {loader ? (
            <div className="loader-container">
              <div className="loader"></div>
              <p>Creating transaction...</p>
            </div>
          ) : (
            <>
              <div>
                <div className="inputWithLabel">
                  <label htmlFor="input-createTx-receiverAddress">Receiver address (Bech32):</label>
                  <input
                    type="text"
                    id="input-createTx-receiverAddress"
                    value={buildTransactionInput.address}
                    onChange={(event) =>
                      setBuildTransactionInput({ ...buildTransactionInput, address: event.target.value })
                    }
                  />
                </div>
                <div className="checkbox">
                  <label>
                    <input
                      type="checkbox"
                      checked={buildTransactionInput.sendAll}
                      onChange={(event) =>
                        setBuildTransactionInput({ ...buildTransactionInput, sendAll: event.target.checked })
                      }
                    />
                    Send all
                  </label>
                </div>
                <div className="inputWithLabel">
                  <label htmlFor="input-createTx-amount">Amount (lovelaces):</label>
                  <input
                    type="number"
                    id="input-createTx-amount"
                    value={buildTransactionInput.amount}
                    onChange={(event) =>
                      setBuildTransactionInput({ ...buildTransactionInput, amount: event.target.value })
                    }
                    disabled={buildTransactionInput.sendAll}
                  />
                </div>
              </div>
              <div className="center">
                <button onClick={createTransactionClick}>Create transaction</button>
              </div>
            </>
          )}
        </div>
      )}

      {transactionHex && (
        <div>
          <h3>Unsigned Transaction (hex):</h3>
          <p>{transactionHex}</p>
        </div>
      )}
    </div>
  );
};

export default TransactionCreator;
