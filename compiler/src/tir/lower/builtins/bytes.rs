//! Bytes built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create the ClassId for bytes type.
    pub(crate) fn get_or_create_bytes_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("bytes");
        let class_id = init_builtin_class!(self, key, "bytes");

        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);

        register_methods!(self, class_id, "bytes",
            shared "__len__" => (vec![], TirType::Int),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
            shared "__getitem__" => (vec![TirType::Int], TirType::Int),
        );

        class_id
    }

    /// Check if a class ID corresponds to the bytes class.
    pub(crate) fn is_bytes_class(&self, class_id: ClassId) -> bool {
        self.class_data
            .get(class_id.index())
            .map(|c| c.qualified_name == "__builtin__.bytes")
            .unwrap_or(false)
    }
}
