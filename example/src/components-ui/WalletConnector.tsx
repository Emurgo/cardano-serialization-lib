import React, { useState } from 'react';
import useCardanoApi from '../context/CardanoContext';
import {
  NO_CARDANO,
  NOT_CONNECTED,
  IN_PROGRESS,
  CONNECTED,
} from '../utils/connectionStates';
import './WalletConnector.css';

const WalletConnector: React.FC = () => {
  const {
    connect,
    availableWallets,
    selectedWallet,
    setSelectedWallet,
    connectionState,
  } = useCardanoApi();
  const [selectedUserWallet, setSelectedUserWallet] = useState('');
  const [showWallets, setShowWallets] = useState(false);

  console.log(`[dApp][WalletConnector] is called`);

  const handleSelectionAndClose = () => {
    setSelectedWallet(selectedUserWallet);
    connect(selectedUserWallet, false, false);
    setShowWallets(false);
  };

  const getWalletIcon = () => {
    return window.cardano[selectedWallet].icon;
  };

  const getWalletName = () => {
    const walletName = window.cardano[selectedWallet].name;
    const capitilizedFirstLetter = walletName[0].toUpperCase() + walletName.substring(1);

    return capitilizedFirstLetter;
  };

  if (connectionState === NO_CARDANO) {
    return <div className="center">Connect a Cardano wallet</div>;
  }

  if (connectionState === IN_PROGRESS) {
    return <div className="center">Waiting connection...</div>;
  }

  if (connectionState === CONNECTED) {
    return (
      <div className="center">
        <div className="connected">
          <div className="center">
            <img src={getWalletIcon()} alt="wallet icon" width="72" />
          </div>
          <div>
            <div>
              <div className="center">Connected to {getWalletName()}</div>
            </div>
          </div>
        </div>
      </div>
    );
  }

  return (
    <div className="center">
      {connectionState === NOT_CONNECTED && !showWallets && (
        <button onClick={() => setShowWallets(true)}>Connect wallet</button>
      )}

      {showWallets && (
        <div>
          <div className="center">Select a wallet for connection:</div>
          <div className="available-wallets">
            {availableWallets.map((walletInfo) => (
              <div key={walletInfo.walletObjKey.toLowerCase()}>
                <label>
                  <input
                    type="radio"
                    name="available_wallets"
                    value={walletInfo.walletObjKey.toLowerCase()}
                    onChange={() =>
                      setSelectedUserWallet(walletInfo.walletObjKey.toLowerCase())
                    }
                  />
                  {walletInfo.walletObjKey.toLowerCase()}
                  <div className="center">
                    <img
                      src={walletInfo.walletObjInfo.icon}
                      alt={walletInfo.walletObjKey.toLowerCase()}
                      width="72"
                    />
                  </div>
                </label>
              </div>
            ))}
          </div>
          <div className="center">
            <button onClick={handleSelectionAndClose}>Connect</button>
          </div>
        </div>
      )}
    </div>
  );
};

export default WalletConnector;
