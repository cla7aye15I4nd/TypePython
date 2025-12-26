//! ListIterator built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create a ClassId for a list iterator type with the given element type.
    /// Each unique list_iterator[T] gets its own ClassId.
    pub(crate) fn get_or_create_list_iterator_class(&mut self, element_type: &TirType) -> ClassId {
        // Check cache first using ClassKey
        let key = ClassKey::builtin_generic("list_iterator", vec![element_type.clone()]);
        if let Some(&class_id) = self.classes.get(&key) {
            return class_id;
        }

        // Allocate new class for this list iterator type
        let class_id = self.alloc_class();
        self.classes.insert(key, class_id);
        self.class_data[class_id.index()].qualified_name = "__builtin__.list_iterator".to_string();
        self.class_data[class_id.index()].type_params = vec![element_type.clone()];

        let iter_type = TirType::Class(class_id);

        // List iterator methods:
        // - __iter__ returns self (iterator is its own iterator)
        // - __next__ returns the next element or raises StopIteration
        // - __dealloc__ deallocates the iterator (called at end of for-loop)
        register_methods!(self, class_id, "list_iterator",
            unique "__iter__" => (vec![], iter_type),
            unique "__next__" => (vec![], element_type.clone()),
            shared "__dealloc__" => (vec![], TirType::Void),
        );

        class_id
    }
}
