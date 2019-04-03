# Bellman WASM demo

This project demonstrates how to generate snark proofs in browser using Bellman code 
on rust compiled to WebAssembly. This project is based on [rust-webpack-template](https://github.com/rustwasm/rust-webpack-template)

The zkSnark in this project verifies that a prover knows a private preimage for Pedersen 
commitment.

# Running

* `npm run start` -- Serve the project locally for development at
  `http://localhost:8080`. It will automatically recompile if you change rust or js code

* `npm run build` -- Bundle the project (in production mode).

# Structure

All the rust code lives under `/crate` dir and compiles the result to `/crate/pkg` as a package.

The main js script for the frontend is `/js/index.js`, the webpage is `/index.html`

The project is managed by webpack, so to adjust the webserver settings edit `/webpack.config.js`
