# Distributed On-Device Computation (DOC)

## Overview
DOC is a disruptive alternative to the traditional SaaS (Software as a Service) model, rebuilt from first principles in physics. By decentralizing computation to user devices and P2P networks, DOC minimizes energy waste, eliminates latency, and enhances resilience. Inspired by concepts like Landauer's principle (minimal energy per bit operation), information theory, and entropy management, DOC treats software as self-sustaining "particles" in a quantum-like field—propagating efficiently without central servers.

The goal is to shift from subscription-based clouds to one-time payments or token models, potentially reducing global data center energy use by 80-90% while empowering users with ownership.

This repository contains the proof-of-concept (POC) prototype, starting with a basic P2P node using Rust, WebAssembly, and libp2p.

## Features
- **On-Device Execution**: Run software locally to bypass speed-of-light delays.
- **P2P Syncing**: Decentralized updates via peer swarms for low-entropy propagation.
- **Energy Optimization**: Designed to minimize bit flips and transmissions.
- **Niche Start**: Initial focus on high-latency apps (e.g., note-taking with delta syncing).

## Installation
1. **Prerequisites**:
   - Rust (1.82+): Install via [rustup.rs](https://rustup.rs).
   - WebAssembly tools: `cargo install wasm-pack`.
   - libp2p: Included as dependency.

2. **Clone the Repo**:
   ```
   git clone https://github.com/<your-username>/doc-prototype.git
   cd doc-prototype
   ```

3. **Build**:
   ```
   cargo build
   ```

## Usage
Run the basic P2P node POC:
```
cargo run
```
- Output: Generates a peer ID and listens on a local address (e.g., `/ip4/127.0.0.1/tcp/53488`).
- Test: In a second terminal, run `cargo run -- /ip4/127.0.0.1/tcp/<port>` to dial and observe ping events.

For full DOC ecosystem, expand to WebAssembly modules for browser/on-device apps.

## Roadmap
- Phase 1: POC (current: P2P basics).
- Phase 2: Delta syncing, security layers.
- Phase 3: Token rewards for node hosting, marketplace integration.

See the project plan for details on physics validations (e.g., energy metrics).

## Contributing
Contributions welcome! Fork the repo, create a branch, and submit a PR. Focus on efficiency (e.g., reduce CPU cycles). See CONTRIBUTING.md (coming soon).

## License
MIT License. See [LICENSE](LICENSE) for details.
