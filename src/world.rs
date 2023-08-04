use std::collections::HashMap;
use std::any::Any;
use std::any::TypeId;
use std::hash::Hash;

// what i want the interface to look like (approximately)
// world.components.get_XX_component(entity.id())
// world.components.get_mut_XX_component(entity.id())
//
// what it currently is set out to look like
// world.components.get::<Component_type(entity.id())
// world.components.get_mut::<Component_type>(entity.id())

#[derive(Eq, Hash, PartialEq)]
pub struct Id {
    pub id: u64
}

pub trait ComponentTrait: Any {
}

impl<T: Any> ComponentTrait for T {}

pub struct ComponentSet {
    set: HashMap<TypeId, Box<dyn ComponentTrait>>
}

impl ComponentSet {
    pub fn new() -> Self {
        ComponentSet {
            set: HashMap::new()
        }
    }
    pub fn add<C: ComponentTrait>(&mut self, component: Box<dyn ComponentTrait>) -> () {
        self.set.insert(TypeId::of::<C>(), component);
    }
    pub fn remove<C: ComponentTrait>(&mut self) -> () {
        self.set.remove(&TypeId::of::<C>());
    }
    pub fn get<C:ComponentTrait>(&self) -> Option<&Box<dyn ComponentTrait>> {
        return self.set.get(&TypeId::of::<C>());
    }
    pub fn get_mut<C: ComponentTrait>(&mut self) -> Option<&mut Box<dyn ComponentTrait>> {
        return self.set.get_mut(&TypeId::of::<C>());
    }
}

pub trait EntityTrait {
    fn id(&self) -> Id;
}

pub struct World {
    pub components : HashMap<Id, ComponentSet>,
    pub entities : HashMap<Id, Box<dyn EntityTrait>>,
}

impl World {
    pub fn new() -> Self {
        World {
            components: HashMap::new(),
            entities: HashMap::new()
        }
    }

    pub fn add_entity(&mut self, entity_id: Id) -> () {
        //self.entities.insert(entity.id(), entity);
        self.components.insert(entity_id, ComponentSet::new());
    }

    /*pub fn remove_entity(&mut self, entity_id: Id) -> () {
        //self.entities.remove(&entity_id);
    }*/

    pub fn add_component<C: ComponentTrait>(&mut self, entity_id: Id, component: Box<dyn ComponentTrait>) {
        self.components.get_mut(&entity_id).unwrap().add::<C>(component);
    }

    pub fn remove_component<C: ComponentTrait>(&mut self, entity_id: Id) -> () {
        self.components.get_mut(&entity_id).unwrap().remove::<C>();
    }
}
