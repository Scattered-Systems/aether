# Aether

Aether was created to support the future of internet based experiences by ensuring the digital security of all
interactions on the network. Globally, Aether will be deployed as an Ethereum native multi-chain with a focus on
security and speed. One of the optimization being developed is the advent of user-centric _microchains_ or a constant
size, zk-SNARK blockchain which operates as their personal cloud computing platform as well as their key to public
spaces.

## Getting Started

### Quickstart

#### Docker

    docker pull jo3mccain/aether:latest

#### From the Source

    git clone https://github.com/scattered-systems/aether.git
    cargo build --workspace
    cargo run --package aether

### Workspace Configuration

Typically, all repositories will use a standard workspace to reduce the overall complexity of the individual systems.

#### Standard Project Structure

    ~/project
        .github
            workflows
        artifacts
            docker
                data
        code
            crates
        config
        deploy
            kube
            docker-compose.yml
        .env
        .gitignore
        Cargo.toml
        LICENSE
        README.md

### Developers

#### Contribute

    git clone https://github.com/scattered-systems/aether.git

    cargo build --package aether
    cargo run --package aether

#### Docker

    docker run jo3mccain/aether

## Resources

### External

* [Cardano](https://cardano.org)
* [Ethereum](https://ethereum.org)
* [Milkomeda](https://milkomeda.com)
* [Mina Protocol](https://minaprotocol.com)

### Internal

* [Scattered-Systems](https://scattered-systems.com)
* [Wiki](https://github.com/scattered-systems/aether/wiki)