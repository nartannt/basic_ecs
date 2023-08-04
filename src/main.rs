use crate::world::*;

mod world;


fn main() {

    pub struct GraphicComponent {
        eye_nb: u64
    }

    pub struct PhysicsComponent<'a> {
        element: &'a str
    }

    pub struct GameObject<'a> {
        id: u64,
        name: &'a str
    }

    impl EntityTrait for GameObject<'_> {
        fn id(&self) -> Id {
            Id {id: self.id}
        }
    }

    let mut game_state = World::new();

    let player = Box::new(GameObject{id: 10, name: &"peepeepoopoo"});
    let not_player = Box::new(GameObject{id: 11, name: &"skrrskrr"});

    let mut gc = Box::new(GraphicComponent{eye_nb: 2});
    let mut pc = Box::new(PhysicsComponent{element: &"He"});

    game_state.add_entity(player.id());
    game_state.add_entity(not_player.id());

    game_state.add_component::<GraphicComponent>(player.id(), gc);
    game_state.add_component::<PhysicsComponent>(not_player.id(), pc);

    let gc = **game_state.components.get(&player.id()).unwrap().get::<GraphicComponent>().unwrap();
    let pc = *game_state.components.get(&not_player.id()).unwrap().get::<PhysicsComponent>().unwrap();

    assert!(gc.eye_nb == 2);
    assert!(pc.element == "He");



}
