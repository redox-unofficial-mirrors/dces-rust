use core::any::{Any, TypeId};

#[cfg(feature = "no_std")]
use alloc::collections::{BTreeMap, FxHashMap};

use crate::entity::*;

pub use self::component_store::*;

mod component_store;

/// The entity builder is used to create an entity with components.
pub struct EntityBuilder<'a, E>
where
    E: EntityStore,
{
    /// The created entity.
    pub entity: Entity,

    /// Reference to the component store.
    pub component_store: &'a mut ComponentStore,

    /// Reference to the entity store.
    pub entity_store: &'a mut E,
}

impl<'a, E> EntityBuilder<'a, E>
where
    E: EntityStore,
{
    pub fn components(self, components: (BuildComponents, BuildSharedComponents)) -> Self {
        self.component_store.append(self.entity, components);
        self
    }
    /// Finishing the creation of the entity.
    pub fn build(self) -> Entity {
        self.entity_store.register_entity(self.entity);
        // self.component_store.register_entity(self.entity);
        self.entity
    }
}

/// This trait provied associated functions to handle all supported
/// components types.
///
/// This trait is implicitly implemented for all other types.
pub trait Component: Any {}
impl<E: Any> Component for E {}

/// Used to store a `component` identified via its type
/// id. Dynamic component adding will make use of this struct.
pub struct ComponentBox {
    component: Box<dyn Any>,
    type_id: TypeId,
}

/// Used to store a `shared component` identified via its type
/// id. Dynamic component adding will make use of this struct.
pub struct SharedComponentBox {
    source: Entity,
    type_id: TypeId,
}

impl SharedComponentBox {
    /// Creates the shared component box.
    pub fn new(type_id: TypeId, source: impl Into<Entity>) -> Self {
        SharedComponentBox {
            source: source.into(),
            type_id,
        }
    }

    /// Consumes the component box and returns its type id beside the source (the entity).
    pub fn consume(self) -> (TypeId, Entity) {
        (self.type_id, self.source)
    }
}

impl ComponentBox {
    /// Creates the component box.
    pub fn new<C: Component>(component: C) -> Self {
        ComponentBox {
            component: Box::new(component),
            type_id: TypeId::of::<C>(),
        }
    }

    /// Consumes the component box and returns the type id and the component.
    pub fn consume(self) -> (TypeId, Box<dyn Any>) {
        (self.type_id, self.component)
    }
}

/// The EntityComponentManager represents the structure of an entity store.
///
/// It will keep track of the bound entities and the associated component store.
#[derive(Default)]
pub struct EntityComponentManager<E>
where
    E: EntityStore,
{
    component_store: ComponentStore,
    entity_store: E,
    entity_counter: u32,
}

impl<E> EntityComponentManager<E>
where
    E: EntityStore,
{
    /// Return a reference to the component store.
    pub fn component_store(&self) -> &ComponentStore {
        &self.component_store
    }

    /// Return a mutable reference to the component store.
    pub fn component_store_mut(&mut self) -> &mut ComponentStore {
        &mut self.component_store
    }

    /// Creates a new entity and returns an `EntityBuilder`.
    pub fn create_entity(&mut self) -> EntityBuilder<'_, E> {
        let entity: Entity = self.entity_counter.into();
        self.entity_counter += 1;

        EntityBuilder {
            entity,
            component_store: &mut self.component_store,
            entity_store: &mut self.entity_store,
        }
    }

    /// Return a reference to the entity store.
    pub fn entity_store(&mut self) -> &mut E {
        &mut self.entity_store
    }

    /// Return a mutable reference to the entity store.
    pub fn entity_store_mut(&mut self) -> &mut E {
        &mut self.entity_store
    }

    /// Create a new entity component manager.
    pub fn new(entity_store: E) -> Self {
        EntityComponentManager {
            entity_counter: 0,
            component_store: ComponentStore::default(),
            entity_store,
        }
    }

    /// Register a new `entity`.
    pub fn register_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.entity_store.register_entity(entity);
        // self.component_store.register_entity(entity);
    }

    /// Removes an `entity` from the manager.
    pub fn remove_entity(&mut self, entity: impl Into<Entity>) {
        let entity = entity.into();
        self.component_store.remove_entity(entity);
        self.entity_store.remove_entity(entity);
    }

    /// Returns references to the entity and component store.
    pub fn stores(&self) -> (&E, &ComponentStore) {
        (&self.entity_store, &self.component_store)
    }

    /// Returns mutable references to the entity and component store.
    pub fn stores_mut(&mut self) -> (&mut E, &mut ComponentStore) {
        (&mut self.entity_store, &mut self.component_store)
    }
}
