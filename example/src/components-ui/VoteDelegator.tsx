import React, { useState } from 'react';
import useCardanoApi from '../context/CardanoContext';
import { createTxWithVoteDelegationCert } from '../components-logic/VoteDelegator';

const VoteDelegator: React.FC = () => {
  const { cardanoApi } = useCardanoApi();
  const [isPopupOpen, setIsPopupOpen] = useState<boolean>(false);
  const [loader, setLoader] = useState<boolean>(false);
  const [voteChoise, setVoteChoice] = useState<string>('ABSTAIN');
  const [unsignedTransactionHex, setUnsignedTransactionHex] = useState<string>('');

  const openPopup = () => {
    console.log('[VoteDelegator][openPopup] is called');
    setIsPopupOpen(true);
  };

  const closePopup = () => {
    console.log('[VoteDelegator][closePopup] is called');
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
      const unsigTxHex = await createTxWithVoteDelegationCert(cardanoApi, voteChoise);
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
      <h2>Creating a transaction with the Vote Delegation certificate</h2>

      <button onClick={openPopup}>Create tx with VoteDelegation certificate</button>

      {isPopupOpen && <div id="overlay" onClick={closePopup}></div>}

      {isPopupOpen && (
        <div id="popup">
          <h2>Tx with VoteDelegation certificate</h2>

          {loader ? (
            <div className="loader-container">
              <div className="loader"></div>
              <p>Creating transaction...</p>
            </div>
          ) : (
            <>
              <div>
                <div className="inputWithLabel">
                  <label htmlFor="input-createTx-voteChoice">Vote choice (DRepID, Abstain, No confidence):</label>
                  <input
                    type="text"
                    id="input-createTx-voteChoice"
                    value={voteChoise}
                    onChange={(event) => setVoteChoice(event.target.value)}
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

export default VoteDelegator;
