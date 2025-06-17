import * as CSL from '@emurgo/cardano-serialization-lib-browser';
import { getRandomPrivAndPubKeys } from './core';

export const generateAddress = (setAddress: Function) => {
  const { publicKey } = getRandomPrivAndPubKeys();

  const address = CSL.BaseAddress.new(
    0, // NetworkId (0 - testnet, 1 - mainnet)
    CSL.Credential.from_keyhash(publicKey.hash()),
    CSL.Credential.from_keyhash(publicKey.hash()),
  ).to_address();

  setAddress(address.to_bech32());
};
