use std::cell::RefCell;
use std::cell::RefMut;

pub struct Wrapper<Id, Content: ?Sized> {
    pub id: Id,
    pub content: RefCell<Content>
}

#[macro_export]
macro_rules! components {
    // we could want a dependency between the id type and the component TODO when needed
    ($id_type: ty, $($component_name:ident : $component_type: ty),*) => {

        pub struct Components {
            $(
                pub $component_name: Vec<Wrapper<$id_type, $component_type>>,
            )*
        }

        // for now we will keep a single unique component of each type per collection
        pub struct ComponentCollection {
            $(
                pub $component_name: Option<$id_type>,
            )*
        }

        impl Components {
            $(
            pub fn $component_name(&mut self, id_opt: Option<$id_type>) -> Option<RefMut<$component_type>> {
                let res = match id_opt {
                    None => None,
                    Some(id) => {
                        let res = self.$component_name.iter().
                            find(|w| w.id == id).
                                and_then(|w| Some(w.content.borrow_mut()));
                        return res;
                    }
                };
                return res;
            }
            )*
        }

    }
}


components!(u64, foo: i32, bar: i64, boo: Box<str>);

pub fn test() {
    let test_val = Wrapper {id: 0, content: RefCell::new(1515)};
    let mut all_components = Components {
        foo: Vec::new(),
        bar: vec![test_val],
        boo: Vec::new()
    };
    let mut get_int = all_components.bar(Some(0)).unwrap();
    
    *get_int = 1444;

    // what the interface is currently projected to look like
    //components.graphic_component(go.components.graphic_component).unwrap().load_model(&display);
    print!("{}", *get_int);
}
