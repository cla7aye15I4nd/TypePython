//! List built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create a ClassId for a list type with the given element type.
    /// Each unique list[T] gets its own ClassId.
    pub(crate) fn get_or_create_list_class(&mut self, element_type: &TirType) -> ClassId {
        // Check cache first using ClassKey
        let key = ClassKey::builtin_generic("list", vec![element_type.clone()]);
        if let Some(&class_id) = self.classes.get(&key) {
            return class_id;
        }

        // Allocate new class for this list type
        let class_id = self.alloc_class();
        self.classes.insert(key, class_id);
        self.class_data[class_id.index()].qualified_name = "__builtin__.list".to_string();
        self.class_data[class_id.index()].type_params = vec![element_type.clone()];

        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);

        // Get the list iterator type for __iter__ return value
        let list_iter_class_id = self.get_or_create_list_iterator_class(element_type);
        let list_iter_type = TirType::Class(list_iter_class_id);

        // For generic list[T]:
        // - unique: methods with type-dependent signatures (need separate FuncId per T)
        // - shared: methods with fixed signatures (reuse same FuncId across all list types)
        register_methods!(self, class_id, "list",
            unique "append" => (vec![element_type.clone()], TirType::Void),
            shared "__len__" => (vec![], TirType::Int),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
            unique "__getitem__" => (vec![TirType::Int], element_type.clone()),
            unique "__setitem__" => (vec![TirType::Int, element_type.clone()], TirType::Void),
            unique "__iter__" => (vec![], list_iter_type),
        );

        class_id
    }
}
