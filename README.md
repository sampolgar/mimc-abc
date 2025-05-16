# MIMC-ABC: Multi-Issuer Multi-Credential Anonymous Credential System

## Overview

This is a research cryptography system benchmarking Anonymous Credentials scenarios. We are benchmarking the feasibility and efficieny of verifying multiple credentials together with Identity Binding - including zkp of an identifier within credentials. We run 4 scenarios for comparison:

- Non-private, non-batch verification
- Non-private batch verification
- Multi-credential batch verification
- Multi-issuer identity binding verification

### Why MIMC-ABC is Important

In real-world scenarios, users often need to present credentials from multiple sources while maintaining their privacy. For example:

- A user may need to prove they hold a government-issued ID, an employer credential, and a training certificate, all tied to the same identity, without revealing their identity.
- In content credentialing, a user may present images signed by different devices to a journal, proving they share an account while selectively disclosing metadata.

## Installation

```sh
# Clone the repository
git clone https://github.com/sampolgar/mimc_abc.git
cd mimc_abc

# Bench
cargo bench
```

## Project Structure

- [`src`](src) - Core library implementation
  - [`credential.rs`](src/credential.rs) - Base credential functionality
  - [`multi_issuer.rs`](src/multi_issuer.rs) - Multi-issuer implementation
  - [`identity_binding.rs`](src/identity_binding.rs) - Identity binding mechanisms
  - [`proof.rs`](src/proof.rs) - Zero-knowledge proof implementations
- [`benches`](benches) - Performance benchmarking suite
  - [`credential_scenarios.rs`](benches/credential_scenarios.rs) - Benchmark scenarios

## How to Run Benchmarks

The project includes a comprehensive benchmarking suite to evaluate the performance of different verification methods. Follow these steps to run the benchmarks:

1. **Install Rust**: Ensure you have Rust installed. You can install it from [rustup.rs](https://rustup.rs/).

2. **Clone the Repository**:

   ```sh
   git clone https://github.com/sampolgar/mimc_abc.git
   cd mimc_abc
   ```

3. **Run Benchmarks**:

   ```sh
   cargo bench
   ```

   This will run the benchmarks defined in credential_scenarios.rs, including:

   - Non-private, non-batch verification
   - Non-private batch verification
   - Multi-credential batch verification
   - Multi-issuer identity binding verification

4. **Analyze Results**: The benchmark results will be displayed in the terminal, showing the performance of each verification method under different configurations (e.g., varying numbers of credentials and attributes).

   For detailed analysis, check the [`benches_analysis`](benches_analysis) directory, which contains Python scripts for data extraction and visualization.

## Technical Background

MIMC-ABC employs position-binding commitments and zero-knowledge proofs to cryptographically bind credentials from distinct issuers to a single, private identifier. Our security model formalizes the identity binding property, ensuring anonymity and unforgeability even against colluding adversaries.

Performance evaluations show that privacy-preserving multi-issuer verification, though approximately 3× slower than non-private baselines (e.g., 18.67ms vs. 6.77ms for 4 credentials), remains efficient for practical use.

## Citation

If you use MIMC-ABC in your research, please cite it as:

```
@software{mimc_abc,
  author = {Polgar, Sam},
  title = {MIMC-ABC: Multi-Issuer Multi-Credential Anonymous Credential System},
  url = {https://github.com/sampolgar/mimc_abc},
  year = {2025}
}
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the terms of the MIT License.
