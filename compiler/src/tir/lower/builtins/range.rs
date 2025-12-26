//! Range built-in iterator class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create the ClassId for range iterator type.
    /// Range is an iterator that yields int values from start to stop with a step.
    pub(crate) fn get_or_create_range_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("range");
        let class_id = init_builtin_class!(self, key, "range");

        let range_type = TirType::Class(class_id);
        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);

        register_methods!(self, class_id, "range",
            // __iter__ returns self (the range object is its own iterator)
            shared "__iter__" => (vec![], range_type),
            // __next__ returns the next int value, or raises StopIteration
            shared "__next__" => (vec![], TirType::Int),
            shared "__len__" => (vec![], TirType::Int),
            // __dealloc__ deallocates the range (called at end of for-loop)
            shared "__dealloc__" => (vec![], TirType::Void),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
        );

        class_id
    }
}
