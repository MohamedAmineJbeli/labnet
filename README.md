# LabNet

A local security lab generator.

Labnet uses Docker Compose to start isolated, pre-configured security scenarios on your local machine.

## Prerequisites

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

A scenario is a directory under `labs/` that contains a `docker-compose.yml` file. `labs/template/` is a template for creating new scenarios and is not runnable itself.

LabNet looks for `labs/` next to the `labnet` executable first, then in the current working directory.

List available scenarios:
```bash
labnet list
```
Completed scenarios are prefixed with `[COMPLETED]`.

Start a scenario:
```bash
labnet up <scenario>
```
This starts the Docker containers in the background and prints the scenario's `MISSION.md` to your terminal.

Show the state of every scenario:
```bash
labnet status
```
Each scenario is shown as `[RUNNING]` or `[STOPPED]`, with `[COMPLETED]` appended if it has been marked complete.

Stop and remove a scenario:
```bash
labnet down <scenario>
```

Print a scenario's hints:
```bash
labnet hint <scenario>
```
This prints the scenario's `HINTS.md`.

Print a scenario's solution:
```bash
labnet solution <scenario>
```
This prints the scenario's `SOLUTION.md`.

Mark a scenario as completed:
```bash
labnet complete <scenario>
```
Undo the completion:
```bash
labnet complete <scenario> --undo
```
Completion state is stored in `~/.labnet/state.json`.

## Creating scenarios

See [labs/template/README.md](labs/template/README.md).
