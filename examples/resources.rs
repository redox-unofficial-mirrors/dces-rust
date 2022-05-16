use dces::prelude::*;

#[derive(Default)]
struct Name {
    value: String,
}

#[derive(Default)]
struct MyResource  {
   count: i32,
   name: String,
}

impl MyResource {
    pub fn create() -> Self {
        MyResource {
            count: 0,
            name: "My_DCES_Resource".to_string(),
        }
    }
    pub fn change_name(&mut self, name: &str) {
        self.name = name.to_string();
    }


    pub fn increment_count(&mut self, count: i32) -> i32 {
        self.count += count;
        self.count
    }

    pub fn print(&mut self) {
        println!("MyResource property count: '{}'", self.count);
        println!("MyResource property name: '{}'", self.name);
    }

    pub fn say_hello(&self) -> &str {
        "Hey, I'm a resource."
    }
}

struct PrintSystem;

impl System<EntityStore> for PrintSystem {
    fn run(&self, ecm: &mut EntityComponentManager<EntityStore>, res: &mut Resources) {
        let (e_store, c_store) = ecm.stores();

        for entity in &e_store.inner {
            if let Ok(comp) = c_store.get::<Name>("name", *entity) {
                println!("Component: porperty 'Name', value '{}'", comp.value);
            }
        }

        if res.contains::<MyResource>() {
            // initial state
            println!("Resource: say_hello() gives '{}'", res.get::<MyResource>().say_hello());
            println!("Resource: {:?}", res.get_mut::<MyResource>().print());

            // lets update the elements
            res.get_mut::<MyResource>().change_name("My name is 'My_DCES_Resource'");
            println!("Resource: {:?}", res.get_mut::<MyResource>().print());

            res.get_mut::<MyResource>().increment_count(5);
            println!("Resource: {:?}", res.get_mut::<MyResource>().print());
        }
    }
}

fn main() {
    // create a new `world` that is instatiated with its entities inside a vector
    let mut world = World::from_entity_store(EntityStore::default());

    // create a name property
    world
        .create_entity()
        .components(
            ComponentBuilder::new()
                .with(
                    "name",
                    Name {
                        value: String::from("DCES property"),
                    },
                )
                .build(),
        )
        .build();

    // crate a new resource with suitable defaults
    world.resources_mut().insert(MyResource::create());

    // create a new print system
    world.create_system(PrintSystem).build();

    // start the loop
    world.run();
}
