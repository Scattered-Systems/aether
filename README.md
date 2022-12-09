# Aether

[![Clippy](https://github.com/scattered-systems/aether/actions/workflows/clippy.yml/badge.svg)](https://github.com/scattered-systems/aether/actions/workflows/clippy.yml)
[![Docker](https://github.com/scattered-systems/aether/actions/workflows/docker.yml/badge.svg)](https://github.com/scattered-systems/aether/actions/workflows/docker.yml)
[![Rust](https://github.com/scattered-systems/aether/actions/workflows/rust.yml/badge.svg)](https://github.com/scattered-systems/aether/actions/workflows/rust.yml)

***

Aether is designed as a natural extension of nodes participating in Disarray which operate proof-of-work consensus mechanisms, empowering users to quickly monetize their excess computational
resources.

## Getting Started

### Docker

Make sure you have docker installed on the target system

#### *Pull the image*

```bash
docker pull scsys/aether:latest
```

#### *Build the image locally (optional)*

```bash
docker buildx build --tag aether:latest .
```

#### *Run the image*

```bash
docker run -p 8080:8080 scsys/aether:latest
```

### Usage

```bash

```

## Contributors

Pull requests are welcome. For major changes, please open an issue first to discuss what you would like to change.

Please make sure to update tests as appropriate.

## License

- [Apache-2.0](https://choosealicense.com/licenses/apache-2.0/)
- [MIT](https://choosealicense.com/licenses/mit/)
