import React, { useState } from 'react';
import useCardanoApi from '../context/CardanoContext';
import { protocolParams } from '../utils/networkConfig';
import { createTxWithStakeDeregistrationCert } from '../components-logic/StakingKeyDeregistrar';

const StakingKeyDeregistrar: React.FC = () => {
  const { cardanoApi } = useCardanoApi();
  const [stakeRefundAmount, setStakeRefundAmount] = useState<string>(protocolParams.keyDeposit);
  const [useConway, setUseConway] = useState<boolean>(false);
  const [isPopupOpen, setIsPopupOpen] = useState<boolean>(false);
  const [loader, setLoader] = useState<boolean>(false);
  const [unsignedTransactionHex, setUnsignedTransactionHex] = useState<string>('');

  const openPopup = () => {
    console.log('[StakingKeyDeregistrar][openPopup] is called');
    setIsPopupOpen(true);
  };

  const closePopup = () => {
    console.log('[StakingKeyDeregistrar][closePopup] is called');
    setIsPopupOpen(false);
  };

  const createTransactionClick = async () => {
    setLoader(true);
    if (!cardanoApi) {
      setLoader(false);
      closePopup();
      alert('A Cardano wallet is not connected');
      return null;
    }
    try {
      const unsigTxHex = await createTxWithStakeDeregistrationCert(cardanoApi, useConway, stakeRefundAmount);
      setUnsignedTransactionHex(unsigTxHex);
    } catch (error) {
      console.error('Error in building tx:', error);
      alert(error);
    } finally {
      setLoader(false);
      closePopup();
    }
  };

  return (
    <div>
      <h2>Creating a transaction with the Stake Deregistration certificate</h2>

      <button onClick={openPopup}>Create tx with StakeDeregistration certificate</button>

      {isPopupOpen && <div id="overlay" onClick={closePopup}></div>}

      {isPopupOpen && (
        <div id="popup">
          <h2>Tx with StakeDeregistration certificate</h2>

          {loader ? (
            <div className="loader-container">
              <div className="loader"></div>
              <p>Creating transaction...</p>
            </div>
          ) : (
            <>
              <div>
                <div className="checkbox">
                  <label>
                    <input
                      type="checkbox"
                      checked={useConway}
                      onChange={(event) => setUseConway(event.target.checked)}
                    />
                    Use Conway
                  </label>
                </div>
                <div className="inputWithLabel">
                  <label htmlFor="input-createTx-refundAmount">Refund amount (lovelaces):</label>
                  <input
                    type="text"
                    id="input-createTx-refundAmount"
                    value={stakeRefundAmount}
                    onChange={(event) => setStakeRefundAmount(event.target.value)}
                    disabled={!useConway}
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

      {unsignedTransactionHex && (
        <div>
          <h3>Unsigned Transaction (hex):</h3>
          <p>{unsignedTransactionHex}</p>
        </div>
      )}
    </div>
  );
};

export default StakingKeyDeregistrar;
