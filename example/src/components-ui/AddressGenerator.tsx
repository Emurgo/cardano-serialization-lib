import React, { useState } from 'react';
import { generateAddress } from '../components-logic/AddressGenerator';

const AddressGenerator: React.FC = () => {
  const [address, setAddress] = useState<string>('');

  return (
    <div>
      <h2>Cardano address generator</h2>
      <button onClick={() => generateAddress(setAddress)}>Generate address</button>
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
