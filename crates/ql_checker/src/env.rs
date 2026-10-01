//! Environment symbol table management and builtin intrinsic registry.

use crate::types::ResolvedType;
use std::collections::HashMap;

/// Tracks variable scope bindings and builtin function signatures.
pub struct SymbolTable {
    bindings: HashMap<String, ResolvedType>,
}

impl SymbolTable {
    pub fn new() -> Self {
        let mut bindings = HashMap::new();

        // Seed runtime intrinsic signatures
        bindings.insert("relu".to_string(), ResolvedType::Void);
        bindings.insert("sigmoid".to_string(), ResolvedType::Void);
        bindings.insert("print".to_string(), ResolvedType::Void);
        bindings.insert("transpose".to_string(), ResolvedType::Void);
        bindings.insert("zeros".to_string(), ResolvedType::Void);
        bindings.insert("random".to_string(), ResolvedType::Void);
        bindings.insert("mse_loss".to_string(), ResolvedType::Void);
        bindings.insert("read_csv".to_string(), ResolvedType::Void);

        SymbolTable { bindings }
    }

    pub fn insert(&mut self, name: String, ty: ResolvedType) {
        self.bindings.insert(name, ty);
    }

    pub fn get(&self, name: &str) -> Option<&ResolvedType> {
        self.bindings.get(name)
    }

    pub fn contains_key(&self, name: &str) -> bool {
        self.bindings.contains_key(name)
    }
}