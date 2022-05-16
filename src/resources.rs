use std::any::{type_name, Any, TypeId};
use fxhash::FxHashMap;

use crate::component::Component;

/// `Resources` provides a resources store.
///
/// This store is designed to save global elements, like services.
/// You can manage resources consuming the implemented functions.
///
/// # Examples
///
/// ```rust
/// use dces::resources::Resources;
///
/// struct MyResource<'r> {
///   count: i32,
///   name:  &'r str,
/// }
///
/// impl <'r> MyResource<'r> {
///     pub fn create() -> Self {
///         MyResource {
///             count: 0,
///             name: "My_DCES_Resource",
///         }
///     }
///
///     pub fn change_name(&mut self, name: &'r str) -> &str {
///         self.name = name;
///         self.name
///     }
///
///     pub fn increment_count(&mut self, count: i32) -> i32 {
///         self.count += count;
///         self.count
///     }
///
///     pub fn say_hello(&self) -> &str {
///         "Hey, I'm a resource."
///     }
/// }
///
/// fn main() {
///     let mut resource_store = Resources::new();
///     resource_store.insert(MyResource::create());
///
///     assert_eq!("Hey, I'm a resource.", resource_store.get::<MyResource>().say_hello());
///     assert_eq!(3, resource_store.get_mut::<MyResource>().increment_count(3));
///     assert_eq!("My name is 'My_DCES_Resource'", resource_store.get_mut::<MyResource>().change_name("My name is 'My_DCES_Resource'"));
/// }
/// ```
#[derive(Default)]
pub struct Resources {
    resources: FxHashMap<TypeId, Box<dyn Any>>,
}

impl Resources {
    /// Returns a boolean, whether the given resource store contains a resource matching the given type.
    /// Returns `true` if the resources contains a given type overwise `false` .
    pub fn contains<C: Component>(&self) -> bool {
        self.resources.contains_key(&TypeId::of::<C>())
    }

    /// Gets an element from the resources.
    ///
    /// # Panics
    ///
    /// Panics if the there is no element matching the given type.
    pub fn get<C: Component>(&self) -> &C {
        self.resources
            .get(&TypeId::of::<C>())
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): can't find type {}.",
                    type_name::<C>()
                )
            })
            .downcast_ref()
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }

    /// Gets a mutable reference to the requested element.
    ///
    /// # Panics
    ///
    /// Panics if the there is no service for the given type.
    pub fn get_mut<C: Component>(&mut self) -> &mut C {
        self.resources
            .get_mut(&TypeId::of::<C>())
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): type {} could not be found.",
                    type_name::<C>()
                )
            })
            .downcast_mut()
            .unwrap_or_else(|| {
                panic!(
                    "Resources.get(): cannot convert to type: {}",
                    type_name::<C>()
                )
            })
    }

    /// Inserts a new resource into the Resources map.
    pub fn insert<C: Component>(&mut self, resource: C) {
        self.resources.insert(TypeId::of::<C>(), Box::new(resource));
    }

    /// Returns true if the resources contains no elements.
    pub fn is_empty(&self) -> bool {
        self.resources.is_empty()
    }

    /// Returns the number of elements in the resources.
    pub fn len(&self) -> usize {
        self.resources.len()
    }

    /// Creates a service resources with an empty Resources map.
    pub fn new() -> Self {
        Resources::default()
    }

    /// Try to get an element from the resources.
    pub fn try_get<C: Component>(&self) -> Option<&C> {
        if let Some(e) = self.resources.get(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_ref() {
                return Some(r);
            }
        }

        None
    }

    /// Try to get an element from the resources.
    pub fn try_get_mut<C: Component>(&mut self) -> Option<&mut C> {
        if let Some(e) = self.resources.get_mut(&TypeId::of::<C>()) {
            if let Some(r) = e.downcast_mut() {
                return Some(r);
            }
        }

        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct ServiceOne;
    struct ServiceTwo;

    #[test]
    fn insert() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get::<ServiceOne>().is_some());
        assert!(resources.try_get::<ServiceTwo>().is_some());
    }

    #[test]
    fn try_get_mut() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);
        resources.insert(ServiceTwo);

        assert!(resources.try_get_mut::<ServiceOne>().is_some());
        assert!(resources.try_get_mut::<ServiceTwo>().is_some());
    }

    #[test]
    fn contains() {
        let mut resources = Resources::new();
        resources.insert(ServiceOne);

        assert!(resources.contains::<ServiceOne>());
        assert!(!resources.contains::<ServiceTwo>());
    }

    #[test]
    fn len() {
        let mut resources = Resources::new();
        assert_eq!(resources.len(), 0);

        resources.insert(ServiceOne);
        assert_eq!(resources.len(), 1);

        resources.insert(ServiceTwo);
        assert_eq!(resources.len(), 2);
    }

    #[test]
    fn is_empty() {
        let mut resources = Resources::new();
        assert!(resources.is_empty());

        resources.insert(ServiceOne);
        assert!(!resources.is_empty());
    }
}
