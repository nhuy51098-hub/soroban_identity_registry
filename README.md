# Stellar IDLink

## Project Title
Stellar IDLink (Decentralized Identity & Profile Registry)

## Project Description
Stellar IDLink is a decentralized smart contract platform built on the Stellar blockchain using the Soroban SDK v25. It allows users and organizations to securely register profiles containing their names and phone numbers. Upon registration, the smart contract utilizes cryptographic SHA-256 hashing to generate a unique 32-byte Lookup Code. This tamper-proof code serves as a universal identifier to query and verify identities on-chain without exposing raw data mapping publicly.

## Project Vision
The vision of Stellar IDLink is to bridge the gap between real-world identity and Web3 decentralized ecosystems. By leveraging Stellar’s ultra-low transaction costs and high speed, IDLink provides businesses, logistics units, and dApps with a lightweight, secure, and trustless public identity lookup infrastructure, replacing fragile and privacy-invasive centralized databases.

## Key Features
- **Secure Profile Registration:** Users can anchor their identity (Name + Phone Number) to their Stellar wallet address securely.
- **Cryptographic Lookup Code Generator:** Generates a unique `BytesN<32>` hash code for every entity using Soroban's native SHA-256 engine to prevent identity collisions.
- **On-chain Persistent Storage:** Utilizes Soroban's `persistent()` storage state to guarantee that profiles remain available and verifiable indefinitely.
- **Instant Decentralized Query:** Anyone with the valid Lookup Code can instantly query the associated profile details for transparent verification.
- **Cryptographic Authentication:** Enforces strict wallet ownership validation using the native `require_auth()` mechanism to prevent identity hijacking.

## Usage Instructions
1. **Deploy Contract:** Deploy the compiled Wasm bytecode onto the Stellar Testnet/Mainnet.
2. **Register Profile:** Invoke the `register_profile` function by passing the user's `Address`, `Name` (String), and `Phone` (String).
3. **Save Lookup Code:** The contract will emit a `reg_prof` event and return a unique 32-byte cryptographic lookup key. Save this key (or convert it into a QR code).
4. **Query Identity:** Call the `get_profile` read-only function with the 32-byte Lookup Code to fetch the user's name, phone, and wallet owner address.

## Future Scope
- **Encrypted Privacy Layers:** Integrate asymmetric encryption so only authorized entities (with private keys) can decrypt the phone numbers.
- **QR Code Generator Tooling:** Build a frontend SDK to instantly convert the 32-byte lookup code into shareable QR codes for physical badging.
- **Stellar Asset Integration:** Allow businesses to charge a micro-fee in XLM or USDC for registering premium identity codes.
- **Multi-Field Identity Schemas:** Support advanced metadata additions such as email addresses, tax IDs, or business registration numbers.
- **Expiration & Renewal:** Implement a lease-based model for lookup codes using Soroban's storage expiration ledgers.

## Technology Stack
- **Rust & Soroban SDK (v25.0.0):** For type-safe, optimized smart contract development.
- **Stellar Blockchain:** For decentralized, immutable ledger state and fast execution.
- **SHA-256 Cryptography:** For generating secure, deterministic identity hashes.

## Contribution
Contributions from the blockchain community, cybersecurity experts, and decentralized identity enthusiasts are highly welcome. Please fork the repository and submit a pull request with your suggested improvements.

## License
This project is licensed under the MIT License.

### Contract Detail
ID: CBLD3OZUGLAXZAVYHHDG2KBH6HISAFI2MOX72ZMHWIDY6L5KD6LCP4MD
