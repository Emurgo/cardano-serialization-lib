import React, { useState } from "react";
import * as CSL from "@emurgo/cardano-serialization-lib-browser";

const AddressGenerator: React.FC = () => {
  const [address, setAddress] = useState<string>("");

  const generateAddress = () => {
    const privateKey = CSL.PrivateKey.generate_ed25519();
    const publicKey = privateKey.to_public();

    const address = CSL.BaseAddress.new(
      0, // NetworkId (0 - testnet, 1 - mainnet)
      CSL.Credential.from_keyhash(publicKey.hash()),
      CSL.Credential.from_keyhash(publicKey.hash())
    ).to_address();

    setAddress(address.to_bech32());
  };

  return (
    <div>
      <h2>Cardano address generator</h2>
      <button onClick={generateAddress}>Generate address</button>
      {address && (
        <div>
          <h3>Your address:</h3>
          <p>{address}</p>
        </div>
      )}
    </div>
  );
};

export default AddressGenerator;
