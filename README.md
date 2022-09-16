# Perihelia

**3D Physics Simulation Game written in Rust with Bevy.**


## Building
To build the project yourself, make sure your environment is setup with Rust and the Rust toolchain. [There are steps here](https://www.rust-lang.org/tools/install) if you don't have Rust installed yet.

First, clone this repository locally:
```bash
  git clone https://github.com/ZakFarmer/Perihelia.git
```

You can then run the following command to build Perihelia:
```bash
  cargo build
```
or if you want to build and run in one go:
```bash
  cargo run
```

**Note that the binary is being built in debug mode and not release - this is recommended by Bevy with optimisation flags defined in Cargo.toml**
## Documentation
The code is heavily commented, but detailed documentation will be coming out soon.

## Roadmap
The roadmap features for this project will likely change quite often so I'll create GitHub milestones for them so I don't lose track.
The main (constant) ones are:

- Ability to save/load worlds/scenes
- Ability to select individual bodies to view their physical properties
- Ability to spawn bodies at will, and be able to choose which type of body to spawn

Then longterm (more ambitious) goals include:
- Seamless web support
- Ability to create account and save your worlds online