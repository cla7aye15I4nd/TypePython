//! Bytearray built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create the ClassId for bytearray type.
    pub(crate) fn get_or_create_bytearray_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("bytearray");
        let class_id = init_builtin_class!(self, key, "bytearray");

        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);

        register_methods!(self, class_id, "bytearray",
            shared "append" => (vec![TirType::Int], TirType::Void),
            shared "__len__" => (vec![], TirType::Int),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
            shared "__getitem__" => (vec![TirType::Int], TirType::Int),
            shared "__setitem__" => (vec![TirType::Int, TirType::Int], TirType::Void),
        );

        class_id
    }
}
