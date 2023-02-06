import logo from './logo.svg';
import './App.css';
import {BigNum} from "@emurgo/cardano-serialization-lib-asmjs";

function App() {
  const num = BigNum.from_str("33333");
  return (
      <div className="App">
        <header className="App-header">
          <img src={logo} className="App-logo" alt="logo" />
          <p>
            Edit <code>src/App.js</code> and save to reload {num.to_hex()} {num.to_json()}.
          </p>
          <a
              className="App-link"
              href="https://reactjs.org"
              target="_blank"
              rel="noopener noreferrer"
          >
            Learn React
          </a>
        </header>
      </div>
  );
}

export default App;
