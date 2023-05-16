use crate::type_system2::Type;
use crate::utilities::Symbol;
use std::borrow::Cow;

use std::collections::HashMap;

pub(super) struct TypeContext<'src> {
    guarded: bool,
    gamma: HashMap<Symbol<'src>, Type<'src>>,
}

impl<'src> TypeContext<'src> {
    pub fn new() -> Self {
        Self {
            guarded: false,
            gamma: HashMap::new(),
        }
    }
    pub fn lookup(&self, sym: Symbol<'src>) -> Option<Cow<Type<'src>>> {
        let target = self.gamma.get(&sym)?;
        Some(if self.guarded {
            Cow::Owned(Type {
                guarded: true,
                ..target.clone()
            })
        } else {
            Cow::Borrowed(target)
        })
    }
    pub fn guarded<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let backup = self.guarded;
        self.guarded = true;
        let result = f(self);
        self.guarded = backup;
        result
    }
    pub fn with<F, R>(&mut self, sym: Symbol<'src>, r#type: Type<'src>, f: F) -> R
    where
        F: FnOnce(&mut Self) -> R,
    {
        let backup = self.gamma.insert(sym.clone(), r#type);
        let result = f(self);
        if let Some(backup) = backup {
            self.gamma.insert(sym, backup);
        } else {
            self.gamma.remove(&sym);
        }
        result
    }
}
