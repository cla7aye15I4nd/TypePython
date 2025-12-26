//! Str built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create the ClassId for str type.
    pub(crate) fn get_or_create_str_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("str");
        let class_id = init_builtin_class!(self, key, "str");

        let str_type = TirType::Class(class_id);

        register_methods!(self, class_id, "str",
            shared "__len__" => (vec![], TirType::Int),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type.clone()),
            shared "__getitem__" => (vec![TirType::Int], TirType::Int),
        );

        class_id
    }
}
