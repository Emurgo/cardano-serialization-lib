import { createContext, ReactNode, useContext, useEffect, useState } from 'react';
import {
  CONNECTED,
  IN_PROGRESS,
  NO_CARDANO,
  NOT_CONNECTED,
} from '../utils/connectionStates';

declare global {
  interface Window {
    cardano: any;
  }
}

type Cip95Type = {
  getPubDRepKey: Function;
  getRegisteredPubStakeKeys: Function;
  getUnregisteredPubStakeKeys: Function;
  signData: Function;
};

export type CardanoApiType = {
  cip95: Cip95Type;
  getBalance: () => Promise<string>;
  getChangeAddress: () => Promise<string>;
  getCollateral: () => Promise<string>;
  getCollateralUtxos: () => Promise<Array<string>>;
  getExtensions: Function;
  getNetworkId: Function;
  getRewardAddresses: Function;
  getUnusedAddresses: Function;
  getUsedAddresses: Function;
  getUtxos: () => Promise<Array<string>>;
  signData: Function;
  signTx: Function;
  submitTx: Function;
  isEnabled: () => Promise<boolean>;
  icon: string;
};

type WalletObjectType = {
  walletObjKey: string;
  walletObjInfo: CardanoApiType;
};

export type CardanoContextType = {
  api: CardanoApiType;
  connect: Function;
  connectionState: number;
  availableWallets: Array<WalletObjectType>;
  setAvailableWallets: Function;
  selectedWallet: string;
  setConnectionState: Function;
  setConnectionStateFalse: Function;
  setSelectedWallet: Function;
};

interface CardanoContextInterface {
  cardanoApi: CardanoApiType | null | undefined;
  connect: Function;
  connectionState: number;
  availableWallets: Array<WalletObjectType>;
  setAvailableWallets: Function;
  selectedWallet: string;
  setConnectionState: Function;
  setConnectionStateFalse: Function;
  setSelectedWallet: Function;
}

const defaultCardanoContextState = {
  cardanoApi: null,
  connect: (
    walletName: string,
    requestId: boolean,
    silent: boolean,
    throwError: boolean = false,
  ) => {},
  connectionState: NOT_CONNECTED,
  availableWallets: [],
  setAvailableWallets: (availableWallets: WalletObjectType[]) => {},
  selectedWallet: '',
  setConnectionState: (connectionState: number) => {},
  setConnectionStateFalse: () => {},
  setSelectedWallet: (selectedWallet: string) => {},
} as CardanoContextInterface;

type CardanoProviderProps = {
  children: ReactNode;
};

const reservedKeys = [
  'enable',
  'isEnabled',
  'getBalance',
  'signData',
  'signTx',
  'submitTx',
  'getUtxos',
  'getCollateral',
  'getUsedAddresses',
  'getUnusedAddresses',
  'getChangeAddress',
  'getRewardAddress',
  'getNetworkId',
  'onAccountChange',
  'onNetworkChange',
  'off',
  '_events',
];

export const CardanoContext = createContext(defaultCardanoContextState);

export function CardanoProvider({ children }: CardanoProviderProps) {
  const [cardanoApi, setCardanoApi] = useState<CardanoApiType | null>();
  const [connectionState, setConnectionState] = useState(NO_CARDANO);
  const [availableWallets, setAvailableWallets] = useState<WalletObjectType[]>([]);
  const [selectedWallet, setSelectedWallet] = useState('');

  const setConnectionStateFalse = () => {
    setConnectionState(NOT_CONNECTED);
    setCardanoApi(null);
  };

  const getAvailableWallets = () => {
    // We need to filter like this because of the Nami wallet.
    // It injects everything into the cardano object not only the object "nami".
    const userWallets = Object.keys(window.cardano).filter(
      (cardanoKey) => !reservedKeys.includes(cardanoKey),
    );
    return userWallets.map((walletName) => {
      return {
        walletObjKey: walletName,
        walletObjInfo: window.cardano[walletName],
      };
    });
  };

  useEffect(() => {
    if (!window.cardano) {
      console.warn('There are no cardano wallets are installed');
      setConnectionState(NO_CARDANO);
      return;
    }

    const tryConnectSilent = async (walletName: string): Promise<void> => {
      let connectResult = null;
      console.debug(`[CardanoContext][tryConnectSilent] is called`);
      try {
        console.debug(`[CardanoContext][tryConnectSilent] trying {false, true}`);
        setConnectionState(IN_PROGRESS);
        connectResult = await connect(walletName, false, true, false);
        if (connectResult != null) {
          console.log('[CardanoContext][tryConnectSilent] RE-CONNECTED!');
          setSelectedWallet(walletName);
          setConnectionState(CONNECTED);
          return;
        }
      } catch (error) {
        setSelectedWallet('');
        setConnectionState(NOT_CONNECTED);
        console.error(error);
      }
    };

    const availableWallets = getAvailableWallets();
    console.log('[CardanoContext] allInfoWallets: ', availableWallets);
    setAvailableWallets(availableWallets);

    if (availableWallets.length === 1) {
      const existingWallet = availableWallets[0].walletObjKey;
      const walletObject = window.cardano[existingWallet];
      walletObject
        .isEnabled()
        .then((response: boolean) => {
          console.debug(`[CardanoContext] Connection is enabled: ${response}`);
          if (response) {
            tryConnectSilent(existingWallet).then();
          } else {
            setConnectionState(NOT_CONNECTED);
          }
        })
        .catch((err: any) => {
          setConnectionState(NOT_CONNECTED);
          console.error(err);
        });
    } else {
      setConnectionState(NOT_CONNECTED);
    }
  }, []);

  const connect = async (
    walletName: string,
    requestId: boolean,
    silent: boolean,
    throwError: boolean = false,
  ): Promise<any> => {
    setConnectionState(IN_PROGRESS);
    setCardanoApi(null);
    console.debug(`[CardanoContext][connect] is called`);

    if (!window.cardano) {
      console.error('There are no cardano wallets are installed');
      setConnectionState(NOT_CONNECTED);
      return;
    }

    console.log(`[CardanoContext][connect] connecting the wallet "${walletName}"`);
    console.debug(
      `[CardanoContext][connect] {requestIdentification: ${requestId}, onlySilent: ${silent}}`,
    );

    try {
      const connectedApi = await window.cardano[walletName].enable({
        requestIdentification: requestId,
        onlySilent: silent,
      });
      console.debug(`[CardanoContext][connect] wallet API object is received`);
      setCardanoApi(connectedApi);
      setSelectedWallet(walletName);
      setConnectionState(CONNECTED);
      return connectedApi;
    } catch (error) {
      console.error(
        `[CardanoContext][connect] The error received while connecting the wallet`,
      );
      setSelectedWallet('');
      setConnectionState(NOT_CONNECTED);
      if (throwError) {
        throw new Error(JSON.stringify(error));
      } else {
        console.error(`[CardanoContext][connect] ${JSON.stringify(error)}`);
      }
    }
  };

  const values = {
    cardanoApi,
    connect,
    connectionState,
    availableWallets,
    setAvailableWallets,
    selectedWallet,
    setConnectionState,
    setConnectionStateFalse,
    setSelectedWallet,
  };

  return <CardanoContext.Provider value={values}>{children}</CardanoContext.Provider>;
}

const useCardanoApi = () => {
  const context = useContext(CardanoContext);

  if (context === undefined) {
    throw new Error('Install any Cardano wallet');
  }

  return context;
};

export default useCardanoApi;
