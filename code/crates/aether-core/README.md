# Disaronno

Disaronno is designed to be a highly modular blockchain framework designed for the Scattered-Systems ecosystem.
This project was inspired by [Ignite](https://ignite.com) and [Substrate](https://substrate.io),
which respectively serve as the foundation for two of the leading multi-chains, Cosmos and Polkadot.

## Getting Started

### Quickstart

#### Repository

    git clone https://github.com/scattered-systems/disarono.git
    cargo build --workspace
    cargo test --package disaronno

### Workspace

Typically, all repositories will use a standard workspace to reduce the overall complexity of the individual systems.

#### Project Structure

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

## Resources

### External

* [Cardano](https://cardano.org)
* [Ethereum](https://ethereum.org)
* [Milkomeda](https://milkomeda.com)
* [Mina Protocol](https://minaprotocol.com)

### Internal

* [Scattered-Systems](https://scattered-systems.com)
* [Wiki](https://github.com/scattered-systems/aether/wiki)