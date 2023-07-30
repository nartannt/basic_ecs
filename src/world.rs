
// what i want the interface to look like (approximately)
// world.components.get_XX_component(entity.id())
// world.components.get_mut_XX_component(entity.id())

pub struct Id {
    id: u64
}

pub trait ComponentTrait {
    fn id(&self) -> Id; 
}

pub enum ComponentType {
    EmptyComponent
}

pub trait ComponentSet {
    fn new() -> Self;
    fn add(&mut self, component: dyn ComponentTrait) -> ();
    fn remove(&mut self, component_type: ComponentType) -> ();
    fn get(&self, component_type: ComponentType) -> & dyn ComponentTrait;
    fn get_mut(&mut self, component_type: ComponentType) -> &mut dyn ComponentTrait;
}

pub trait EntityTrait {
    fn foo(&self) -> ();
}

pub struct World {
    components : Vec<Option<Box<dyn ComponentTrait>>>,
    entities : Vec<Option<Box<dyn EntityTrait>>>,
}
