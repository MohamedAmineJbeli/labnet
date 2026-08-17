# LabNet

A local security lab generator.

Labnet uses Docker Compose to start isolated, pre-configured security scenarios on your local machine.

## Prequisites

* [Docker](https://docs.docker.com/get-started/get-docker/) (and Docker Compose)

## Installation
Pre-compiled binaries for Linux, macOS, and Windows are available on the [Releases](../../releases) page.

## Building from source

If you prefer to build from source or want to contribute, you need [Rust](https://rust-lang.org/tools/install/) installed.
```bash
git clone https://github.com/MohamedAmineJbeli/labnet.git
cd labnet
cargo build --release
```

The compiled binary will be located at `target/release/labnet`.

## Usage

List available scenarios:
```bash
labnet list
```

Start a scenario:
```bash
labnet up <scenario>
```
This starts the Docker containers in the background and prints the scenario's `MISSION.md` to your terminal.

Stop and remove a scenario:
```bash
labnet down <scenario>
```