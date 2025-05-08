# Example how to use the Cardano Serialization Lib

The example includes the dApp with four basic operations:

* Creating a simple transaction
* Registering a wallet staking public key
* Unregistering a wallet staking key
* Vote delegation

## Structure

Everything related to the user interface (UI) is located in the folder `./src/components-ui`.

The logic behind each operation is found in the folder `./src/components-logic` which has the same name.

Commonly used functions are placed in the file `./src/componets-logic/core.ts`.

### Example

Let's take a look at the file `TransactionCreator.tsx`.

It contains only UI and it is located in the folder `./src/components-ui`.

All logic related to this file is placed in the file `TransactionCreator.ts`, which is located in the `./src/components-logic` folder.

## Installation and running

### Instalation

To install the app, go to the `./example` folder. If you're reading this, you're already here.

Run the commands `nvm use` and then `npm install --force`.

That's it! The app is now ready to launch.

### Running

To start the app, simply run the command `npm start`.

## Conclusion

This is a simple and not very attractive app, but it does what it should: show you how to work with the Cardano Serialization Lib.
