# Rust-Counterexamples

## Summary

This repository contains the artifact for the paper *Counterexamples in Safe Rust*. It provides examples of Rust code that, while written in Safe Rust, can still lead to memory safety issues under specific circumstances. These examples are provided for educational purposes and demonstrate the limits of Safe Rust's guarantees.

## Security Disclaimer

**Important Notice:** The examples in this repository demonstrate how Safe Rust can be subverted to cause undefined behavior. We strongly recommend running this code in a sandboxed environment to avoid any unintended side effects. The code examples provided may only be safe in specific OS environments or versions of Rust. Please refer to individual code files for details.

## Dependencies

To run the examples in this repository, you will need the following dependencies:

- **Rust and Cargo**: Follow the instructions [here]([https://rustup.rs/](https://www.rust-lang.org/tools/install)) to install Rust and Cargo.
- **Additional dependencies**: GDB, rust-nightly

## Installation

1. Clone this repository:

    ```sh
    git clone https://github.com/DavisPL/rust-counterexamples.git
    cd rust-counterexamples
    ```

## Repository Structure

- **src/bin/**: Contains the source code for all the examples.
- **Cargo.toml** and **Cargo.lock**: Rust package files documenting required dependencies.
- **README.md**: This file.

## Running the code examples 

To run `proc_self_mem_1` you should run the following command on your terminal 

    ```sh
    cargo run --bin proc_self_mem_1
    ```

For security reasons, all of these files have been disabled be default to prevent accidental execution. You will need to manually update the `RUN_FLAG` to `true` in the concerned file, to be able to run it. 
**Note:** `cargo_wrapper.rs` can modify the compiler on your system. You can find instructions in the same file to revert the changes. 

### Tables from the Paper

Below are key tables from the paper *Counterexamples in Safe Rust*. These tables show the evaluation of various tools and patterns within the scope of Rust's safety:

#### Table 1: Comprehensive Evaluation of Tools on Different Attack Patterns

| **Code Example**             | **CWE**  | **Filesystem Access** | **Command Execution** | **Compiler Unsoundness** | **Build-time Effects** | **Environment Variables** | **Miri** | **Verus** | **Prusti-Dev** | **Flux** | **Rudra** |
|------------------------------|----------|-----------------------|-----------------------|--------------------------|------------------------|---------------------------|----------|-----------|----------------|----------|-----------|
| /proc/self/mem-1             | CWE-123  | ✓                     |                       |                          |                        |                           | ⚠️       | ⚠️        | ⚠️              | ✗        | ✗         |
| /proc/self/mem-2             | CWE-125,787,119,124 | ✓            |                       |                          |                        |                           | ⚠️       | ✗         | ⚠️              | ⚠️       | ✗         |
| GDB sudo                     | CWE-123  |                       | ✓                     |                          |                        |                           | ✗        | —         | ⚠️              | —        | ✗         |
| Static Lifetime              | CWE-416,825 |                      |                       | ✓                        |                        |                           | ✓        | ⚠️        | ⚠️              | ✓        | ✓         |
| Cargo Wrapper                | CWE-426  | ✓                     | ✓                     |                          | ✓                      |                           | ✗        | ✗         | ✗              | ✗        | ✗         |
| Path `ls`                    | CWE-426  | ✓                     |                       |                          |                        | ✓                         | ⚠️       | ✗         | ✗              | ✗        | ✗         |
| Dangling Nightly Lifetime    | CWE-416,825 |                      |                       | ✓                        |                        |                           | ✓        | ⚠️        | ✓              | ⚠️       | ✓         |
| Trait Upcasting              | CWE-704,476,843 |                  |                       | ✓                        |                        |                           | ✓        | ⚠️        | ⚠️              | ⚠️       | ✗         |
| Large Array Initialization   | CWE-665  |                       |                       | ✓                        |                        |                           | ✗        | ✗         | ⚠️              | ⚠️       | ✗         |


*Description: This table provides a comprehensive evaluation of tools on different attack patterns described in the paper and two additional examples. It includes the corresponding CWEs, related attack patterns, and the output from existing verification and analysis tools.*

#### Table 2: Frequency of Attack Patterns

| **Attack Pattern**       | **Top 500 Crates** | **Random 500 Crates** | **RustSec** | **rustdecimal** |
|--------------------------|--------------------|-----------------------|-------------|-----------------|
| Filesystem Access        | 3                  | 4                     | 1           | ✓               |
| Command Execution        | 16                 | 13                    | 2           | ✓               |
| Compiler Unsoundness     | 0                  | 0                     | 1*          | ✗               |
| Build-time Effects       | 0                  | 0                     | 1           | ✗               |
| Environment Variables    | 8                  | 3                     | 0           | ✓               |


*Description: This table shows the frequency of the attack patterns considered in examples identified within the top 500 crates, a random set of 500 crates, vulnerabilities listed in RustSec, and the `rustdecimal` supply-chain attack.*


## Information and Authors

This repository is maintained by:

- **Author Name 1**: [mhassnain@ucdavis.edu]
- **Author Name 2**: [cdstanford@ucdavis.edu]

For the paper *Counterexamples in Safe Rust*: [LINK_TO_PAPER].

## Contributing

We welcome contributions to this repository. Please follow the standard [Rust community guidelines](https://www.rust-lang.org/community) and adhere to the project's [code of conduct](CODE_OF_CONDUCT.md).

### How to Contribute

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Make your changes.
4. Submit a pull request with a detailed description of your changes.

## Citing this Repository

If you use this repository in your research or work, please cite it as follows:

```plaintext
@article{yourpaper2023,
  title={Counterexamples in Safe Rust},
  author={Your Name and Co-author's Name},
  journal={Conference Name},
  year={2023},
  publisher={Publisher Name}
}
