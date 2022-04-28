use dces::prelude::*;

#[derive(Default)]
struct Name {
    value: String,
}

struct HelloDCES;

impl HelloDCES {
    pub fn say_hello(&self) -> &str {
        return "Hello DECS!";
    }
}

struct PrintSystem;

impl System<EntityStore> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>, res: &mut Resources) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.get::<Name>("name", *entity) {
                println!("{}", comp.value);
            }
        }

        println!("{}", res.get::<HelloDCES>().say_hello());
    }
}

fn main() {
    let mut world = World::from_entity_store(EntityStore::default());

    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(
                    "name",
                    Name {
                        value: String::from("DCES"),
                    },
                )
                .build(),
        )
        .build();

    world.resources_mut().insert(HelloDCES);

    world.create_system(PrintSystem).build();

    world.run();
}
