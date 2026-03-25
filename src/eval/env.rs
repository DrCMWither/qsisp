use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::error::EvalError;
use super::value::Value;
use crate::locale::Locale;

#[derive(Clone)]
pub struct Env {
    pub frame: Rc<RefCell<EnvFrame>>,
    pub locale: Locale,
}

#[derive(Debug)]
pub(crate) struct EnvFrame {
    pub(crate) vars: HashMap<String, Value>,
    pub(crate) parent: Option<Env>,
}

impl PartialEq for Env {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.frame, &other.frame)
    }
}

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let frame = self.frame.borrow();
        let keys: Vec<&String> = frame.vars.keys().collect();

        f.debug_struct("Env")
            .field("bindings", &keys)
            .field("has_parent", &frame.parent.is_some())
            .field("locale", &self.locale)
            .finish()
    }
}

impl Env {
    pub fn new(locale: Locale) -> Self {
        Self {
            frame: Rc::new(RefCell::new(EnvFrame {
                vars: HashMap::new(),
                parent: None,
            })),
            locale,
        }
    }

    pub fn child(parent: Env) -> Self {
        Self {
            locale: parent.locale.clone(),
            frame: Rc::new(RefCell::new(EnvFrame {
                vars: HashMap::new(),
                parent: Some(parent),
            })),
        }
    }

    pub fn get(&self, name: &str) -> Option<Value> {
        let frame = self.frame.borrow();

        if let Some(v) = frame.vars.get(name) {
            return Some(v.clone());
        }

        let parent = frame.parent.clone();
        drop(frame);

        parent.and_then(|p| p.get(name))
    }

    pub fn set(&self, name: String, value: Value) {
        self.frame.borrow_mut().vars.insert(name, value);
    }

    pub fn assign(&self, name: &str, value: Value) -> Result<(), EvalError> {
        {
            let mut frame = self.frame.borrow_mut();
            if frame.vars.contains_key(name) {
                frame.vars.insert(name.to_string(), value);
                return Ok(());
            }
        }

        let parent = self.frame.borrow().parent.clone();
        if let Some(parent) = parent {
            parent.assign(name, value)
        } else {
            Err(EvalError::UndefinedSymbol(name.to_string()))
        }
    }
}