use dioxus::prelude::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;

// 组件渲染函数类型
pub type ComponentRenderer = Arc<dyn Fn(&HashMap<String, String>) -> Element + 'static>;

// 组件注册表
#[derive(Clone)]
pub struct ComponentRegistry {
    components: Rc<RefCell<HashMap<String, ComponentRenderer>>>,
}

impl ComponentRegistry {
    pub fn new() -> Self {
        Self {
            components: Rc::new(RefCell::new(HashMap::new())),
        }
    }

    pub fn register<F>(&self, name: &str, renderer: F)
    where
        F: Fn(&HashMap<String, String>) -> Element + 'static,
    {
        self.components
            .borrow_mut()
            .insert(name.to_string(), Arc::new(renderer));
    }

    pub fn get_component(&self, name: &str, params: &HashMap<String, String>) -> Option<Element> {
        if let Some(renderer) = self.components.borrow().get(name) {
            Some(renderer(params))
        } else {
            None
        }
    }
}

impl PartialEq for ComponentRegistry {
    fn eq(&self, other: &Self) -> bool {
        // 简单比较内存地址，因为每个注册表都是唯一的
        Rc::ptr_eq(&self.components, &other.components)
    }
}

#[macro_export]
macro_rules! register_component {
    ($registry:expr, $name:expr, $renderer:expr) => {
        $registry.register($name, $renderer)
    };
}
