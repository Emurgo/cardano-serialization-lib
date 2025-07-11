import React, { useEffect } from 'react';
import AddressGenerator from './components-ui/AddressGenerator';
import TransactionCreator from './components-ui/TransactionCreator';
import './styles.css';
import useCardanoApi, { CardanoApiType } from './context/CardanoContext';
import { CONNECTED } from './utils/connectionStates';
import WalletConnector from './components-ui/WalletConnector';
import StakingKeyRegistrar from './components-ui/StakingKeyRegistrar';
import StakingKeyDeregistrar from './components-ui/StakingKeyDeregistrar';
import VoteDelegator from './components-ui/VoteDelegator';

const App: React.FC = () => {
  const { connectionState, selectedWallet, setConnectionState, setConnectionStateFalse } =
    useCardanoApi();
  const isWalletConnected = connectionState === CONNECTED;

  const walletStateWithTimeout = async (walletObject: CardanoApiType, timeout = 2000) => {
    const timeoutPromise = new Promise((_, reject) => {
      setTimeout(() => {
        reject(new Error('Checking connection timeout'));
      }, timeout);
    });
    const walletEnabledPromise = walletObject.isEnabled();
    const response = await Promise.race([walletEnabledPromise, timeoutPromise]);
    return response;
  };

  useEffect(() => {
    const getConnectionState = async () => {
      console.debug(`[dApp][App] Checking connection works`);
      try {
        const walletObject: CardanoApiType = window.cardano[selectedWallet];
        const conState = await walletStateWithTimeout(walletObject, 10000);

        if (conState) {
          setConnectionState(CONNECTED);
        } else {
          setConnectionStateFalse();
        }
      } catch (error) {
        setConnectionStateFalse();
        console.error(error);
      }
    };

    if (isWalletConnected) {
      const connectionTimer = setInterval(getConnectionState, 10000);
      return () => {
        console.debug(`[dApp][App] Checking connection is stopped`);
        clearInterval(connectionTimer);
      };
    }
  }, [isWalletConnected, selectedWallet, setConnectionState, setConnectionStateFalse]);

  return (
    <div className="App">
      <h1>Cardano Serialization Library demonstration</h1>
      <WalletConnector />
      <AddressGenerator />
      <TransactionCreator />
      <StakingKeyRegistrar />
      <StakingKeyDeregistrar />
      <VoteDelegator />
    </div>
  );
};

export default App;
