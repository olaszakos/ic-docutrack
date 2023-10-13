# ic-docutrack

ic-docutrack is a proof-of-concept dapp for sharing documents in encrypted form with other people and track access control rights as well as confirmations to have seen them. It is being developed by DFINITY and Julius Baer.

## Documentation

The [doc](doc/) directory will contain a description of the high-level design of the dapp as well as user instructions.

## Implementation

To run the dapp, run the following in one terminal window:

```
dfx start --clean
```

And in another terminal"

```
# Install needed frontend dependencies.
npm install -g pnpm
pnpm install

# Deploy the canisters.
dfx deploy
dfx deps deploy
```

In your browser you can now go to <canister_id>.localhost:8000 to access the frontend.

See our [CONTRIBUTING](.github/CONTRIBUTING.md) document to get started.
