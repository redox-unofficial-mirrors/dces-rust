#![crate_name = "dces"]
#![crate_type = "lib"]
#![deny(warnings)]

//! # DCES
//!
//! DCES is a library that implements a dense entity component system
//! targeted to be consumed in a native rust ecosystem. In contrast to
//! the Object Oriented Programming (OOP) paradigma, the DCES
//! separates data from behavior. Its entities can have multiple, dynamically changing components.
//!
//! Structure:
//!
//! * entity -> uniquely identify an object, can contain zero or more components.
//! * component -> datatypes that can be added to an entity, stores valid property values.
//! * resource ->
//! * entity_component_manager -> keeps track of an entity store (bound entities and an associated component store).
//! * store -> instance of a given storage provider, that handles multiple entities, componets or resources.
//! * systems -> functions that run for all entities matching a component query.
//! * world -> instance of a given entity storage provider, handled via the entity_component_manager.
//!
//! Features:
//!
//! * Filter/query and sort entities for stores.
//! * Define priorities (run order) for stores.
//!
//! **The library is still WIP. API changes are possible.**
//!
//! [![MIT licensed](https://img.shields.io/badge/license-MIT-blue.svg)](./LICENSE)
//!
//! # Example
//!
//! ```no_run
//! use dces::prelude::*;
//!
//! #[derive(Default)]
//! struct Name {
//!    value: String,
//! }
//!
//! struct PrintSystem;
//!
//! impl System<EntityStore> for PrintSystem {
//!    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>, _res: &mut Resources) {
//!        let (e_store, c_store) = ecm.stores();
//!
//!        for entity in &e_store.inner {
//!            if let Ok(comp) = c_store.get::<Name>("name", *entity) {
//!                println!("{}", comp.value);
//!            }
//!        }
//!    }
//! }
//!
//!
//! let mut world = World::from_entity_store(EntityStore::default());
//!
//! world
//!     .create_entity()
//!     .components(
//!         ComponentBuilder::new()
//!             .with("name", Name {
//!                 value: String::from("DCES"),
//!             })
//!             .build(),
//!     )
//!     .build();
//!
//! world.create_system(PrintSystem).build();
//! world.run();
//!
//!
//! ```
pub mod component;
pub mod entity;
pub mod error;
pub mod prelude;
pub mod resources;
pub mod system;
pub mod world;
