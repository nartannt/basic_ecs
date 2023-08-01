use std::collections::HashMap;

// what i want the interface to look like (approximately)
// world.components.get_XX_component(entity.id())
// world.components.get_mut_XX_component(entity.id())
//
// what it currently is set out to look like
// world.components.get(component_type, entity.id())
// world.components.get_mut(component_type, entity.id())

#[derive(Eq, Hash, PartialEq)]
pub struct Id {
    id: u64
}

pub trait ComponentTrait {
    fn parent_id(&self) -> Id; 
}

#[derive(PartialEq, Eq, Clone, Hash)]
pub enum ComponentType {
    EmptyComponent
}


pub struct ComponentSet {
    set: HashMap<ComponentType, Box<dyn ComponentTrait>>
}

impl ComponentSet {
    fn new() -> Self {
        ComponentSet {
            set: HashMap::new()
        }
    }
    fn add(&mut self, component_type: ComponentType, component: Box<dyn ComponentTrait>) -> () {
        self.set.insert(component_type, component);
    }
    fn remove(&mut self, component_type: ComponentType) -> () {
        self.set.remove(&component_type);
    }
    fn get(&self, component_type: ComponentType) -> Option<&Box<dyn ComponentTrait>> {
        return self.set.get(&component_type);
    }
    fn get_mut(&mut self, component_type: ComponentType) -> Option<&mut Box<dyn ComponentTrait>> {
        return self.set.get_mut(&component_type);
    }
}

pub trait EntityTrait {
    fn id(&self) -> Id;
}

pub struct World {
    components : HashMap<Id, ComponentSet>,
    entities : HashMap<Id, Box<dyn EntityTrait>>,
}

impl World {
    pub fn new() -> Self {
        World {
            components: HashMap::new(),
            entities: HashMap::new()
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn EntityTrait>) -> () {
        self.entities.insert(entity.id(), entity);
    }

    pub fn remove_entity(&mut self, entity_id: Id) -> () {
        self.entities.remove(&entity_id);
    }

    pub fn add_component(&mut self, entity_id: Id, component_type: ComponentType, component: Box<dyn ComponentTrait>) {
        self.components.get_mut(&entity_id).unwrap().add(component_type, component);
    }

    pub fn remove_component(&mut self, entity_id: Id, component_type: ComponentType) -> () {
        self.components.get_mut(&entity_id).unwrap().remove(component_type);
    }
}
