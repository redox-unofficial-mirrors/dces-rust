# DCES

DCES is a library that provides a variant of the Entity Component
System: https://en.wikipedia.org/wiki/Entity–component–system.

The goal of DCES is a lightweight ECS library with zero dependencies.
This eases creation of UI frameworks and game engines. It is being developed as part
of OrbTk, a (G)UI framework written in Rust. All widgets and
properties of OrbTk are handled by DCES.

[![Build status](https://gitlab.redox-os.org/redox-os/dces-rust/badges/develop/pipeline.svg)](https://gitlab.redox-os.org/redox-os/dces-rust/pipelines)
[![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
[![crates.io](https://img.shields.io/badge/crates.io-v0.3-orange.svg)](https://crates.io/crates/dces)
[![docs.rs](https://docs.rs/dces/badge.svg)](https://docs.rs/dces)

## Features:

* Register entities with components
* Share components between entities
* Register systems and read / write components of entities
* Order systems execution by priority
* Register container for entity organization (Vec, FxHashMap, Custom Container, ...)
* Register init and cleanup system

## Usage

To include DCES in your project, you need to include a dependency line
to your `Cargo.toml` file:

```text
dces = "~0.4"
```

The bleading edge code of DCES is managed inside the `development`
branch. Just add a dependency line to your `Cargo.toml` file that
referenced this branch:

```text
dces = { git = https://gitlab.redox-os.org/redox-os/dces-rust.git#development }
```

## Examples

The project will assemble maintained examples in the `examples/` directory.

Use the following lines as a quick starter:

```rust
use dces::prelude::*;

#[derive(Default)]
struct Name {
	value: String,
}

struct PrintSystem;

impl System<EntityStore, ComponentStore> for PrintSystem {
	fn run(&self, ecm: &mut EntityComponentManager<EntityStore, ComponentStore>) {
		let (e_store, c_store) = ecm.stores();

		for entity in &e_store.inner {
			if let Ok(comp) = c_store.get::<Name>(*entity) {
				println!("{}", comp.value);
			}
		}
	}
}

fn main() {
	let mut world = World::<EntityStore, ComponentStore>::new();

	world
		.create_entity()
		.components(
			ComponentBuilder::new()
				.with(Name {
					value: String::from("DCES"),
				})
				.build(),
		)
		.build();

	world.create_system(PrintSystem).build();
	world.run();
}
```

You can start the `basic` example by executing the following command:

```text
cargo run --example basic
```

## Library documentation

You can build and run the latest library documentation by executing the following command:

```text
cargo doc --no-deps --open
```

## Online guide

If you like to dig into the concept root of DCES, you may enjoy to
read the [[`guide`][dces-guide]. The sources are provided inside a sub-module. It
discusses the very concept on which `DCES` is built.  A bird`s eye
view of essential building blocks is presented. Their discussion is
co-centered on the details relevant to the implementation.

You may download the project including this sub-module while cloning
the repository like this:

```text
git clone --recurse-submodules https://gitlab.redox-os.org/redox-os/dces-rust.git#development
```

[dces_guide]:`https://gitlab.redox-os.org/rzerres/guide/dces-rust/en/html/index.html`

## Future features

* Concurrency of systems with same priority
* Advanced example
* Book

## Inspirations

* [Specs - Parallel ECS](https://github.com/slide-rs/specs)
* [Rustic Entity-Component System](https://github.com/AndyBarron/rustic-ecs)

## FAQ

### Why not Specs
Because DCES is developed to fulfill the requirements of OrbTk. To reduce the dependency tree of OrbTk
DCES depends on zero crates.

## License

Licensed under MIT license ([LICENSE](LICENSE)).
