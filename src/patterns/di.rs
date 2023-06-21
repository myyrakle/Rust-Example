use std::{
    any::{Any, TypeId},
    collections::HashMap,
};

pub struct DependencyManager {
    map: HashMap<TypeId, Box<dyn Any>>,
}

impl DependencyManager {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }

    pub fn register<T: 'static>(&mut self, value: T) {
        self.map.insert(TypeId::of::<T>(), Box::new(value));
    }

    pub fn get<T: 'static>(&self) -> Option<&T> {
        self.map
            .get(&TypeId::of::<T>())
            .map(|any| any.downcast_ref::<T>())
            .flatten()
    }
}

trait IService {
    fn get_name(&self) -> String;
}

struct ServiceA {
    name: String,
}

impl ServiceA {
    fn new() -> Self {
        Self {
            name: "ServiceA".to_string(),
        }
    }
}

impl IService for ServiceA {
    fn get_name(&self) -> String {
        self.name.clone()
    }
}

#[tokio::main]
async fn main() {
    let mut di = DependencyManager::new();
    di.register(Box::new(ServiceA::new()) as Box<dyn IService>);

    let service: &Box<dyn IService> = di.get().unwrap();
    println!("{}", service.get_name());
}
