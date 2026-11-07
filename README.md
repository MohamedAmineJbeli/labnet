# LabNet

A local security lab generator.

Labnet uses Docker Compose to start isolated, pre-configured security scenarios on your local machine.

## Prerequisites

* [Docker](https://docs.docker.com/get-started/get-docker/) (and Docker Compose)

## Installation

Pre-compiled archives for Linux, macOS, and Windows are available on the [Releases](../../releases) page. Each archive contains the `labnet` binary, the `labs/` directory, `README.md`, and the license files.

Extract the archive and run the binary. Keep `labs/` next to the executable; LabNet looks there first.

## Building from source

If you prefer to build from source or want to contribute, you need [Rust](https://rust-lang.org/tools/install/) installed.
```bash
git clone https://github.com/MohamedAmineJbeli/labnet.git
cd labnet
cargo build --release
```

The compiled binary will be located at `target/release/labnet`.

## Bundled scenarios

* `dvwa` — [Damn Vulnerable Web Application](labs/dvwa/README.md), a deliberately vulnerable PHP/MySQL web application.
* `dvwa-command-injection` — [DVWA Command Injection](labs/dvwa-command-injection/README.md), an exercise in OS command injection.
* `dvwa-file-inclusion` — [DVWA File Inclusion](labs/dvwa-file-inclusion/README.md), an exercise in local file inclusion.
* `juice-shop` — [OWASP Juice Shop](labs/juice-shop/README.md), an intentionally insecure web application used for security training.

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

## License

Licensed under either of

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
* MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this project by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

The Docker images referenced by the scenarios are third-party works with their own licenses. Juice Shop is MIT licensed; DVWA is GPL-3.0 licensed.
