import logo from './logo.svg';
import './App.css';
import {BigNum} from "@emurgo/cardano-serialization-lib-browser";
import React, {useState} from 'react';

function App() {
  const [currentStrValue, setStrValue] = new useState("");

  const getBigNumValue = (strValue: string) => {
    return BigNum.from_str(strValue)
  };

  const containsOnlyNumbers = (strValue: string) => {
    return /^\d+$/.test(strValue);
  }

  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>
          <label className="App-warning">
            It is the example project for the react framework with @emurgo/cardano-serialization-lib-browser.
          </label>
        </p>
        <p>
          <div>
            <label>
              Enter value:
            </label>
          </div>
          <div>
            <input className="App-input" type="text" onChange={event => setStrValue(event.target.value)}/>
          </div>
        </p>
        <p>
          <div>
            <label>HEX represantion of the entered value: </label>
            <label>{containsOnlyNumbers(currentStrValue) ? getBigNumValue(currentStrValue).to_hex() : "unexpected value"}</label>
          </div>
        </p>
        <p>
          <div>
            <label>JSON represantion of the entered value: </label>
            <label>{containsOnlyNumbers(currentStrValue) ? getBigNumValue(currentStrValue).to_json() : "unexpected value"}</label>
          </div>
        </p>
      </header>
    </div>
  );
}

export default App;
