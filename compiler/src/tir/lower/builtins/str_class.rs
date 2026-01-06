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
            // Core methods
            shared "__len__" => (vec![], TirType::Int),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type.clone()),
            shared "__getitem__" => (vec![TirType::Int], TirType::Int),

            // Case conversion methods (Phase 3)
            shared "lower" => (vec![], str_type.clone()),
            shared "upper" => (vec![], str_type.clone()),

            // Whitespace operations (Phase 3)
            shared "strip" => (vec![], str_type.clone()),

            // String search methods (Phase 3)
            shared "find" => (vec![str_type.clone()], TirType::Int),
            shared "startswith" => (vec![str_type.clone()], TirType::Bool),
            shared "endswith" => (vec![str_type.clone()], TirType::Bool),

            // String modification (Phase 3)
            shared "replace" => (vec![str_type.clone(), str_type.clone()], str_type.clone()),

            // Character classification (Phase 3)
            shared "isalpha" => (vec![], TirType::Bool),
            shared "isdigit" => (vec![], TirType::Bool),
            shared "isspace" => (vec![], TirType::Bool),
        );

        class_id
    }
}
