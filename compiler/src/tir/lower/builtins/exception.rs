//! Exception built-in class implementation

use crate::tir::ids::ClassId;
use crate::tir::types::TirType;

use super::super::symbols::{ClassKey, GlobalSymbols};

impl GlobalSymbols {
    /// Get or create the ClassId for Exception type.
    pub(crate) fn get_or_create_exception_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("Exception");
        let class_id = init_builtin_class!(self, key, "Exception");

        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);
        let exc_type = TirType::Class(class_id);

        register_methods!(self, class_id, "Exception",
            // __init__ takes a string message and returns the exception
            shared "__init__" => (vec![str_type.clone()], exc_type),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
        );

        class_id
    }

    /// Get or create the ClassId for StopIteration exception type.
    /// StopIteration inherits from Exception and is raised by __next__ when iteration is complete.
    pub(crate) fn get_or_create_stop_iteration_class(&mut self) -> ClassId {
        let key = ClassKey::builtin("StopIteration");
        if let Some(&class_id) = self.classes.get(&key) {
            return class_id;
        }

        // Ensure Exception class exists first
        let exception_class_id = self.get_or_create_exception_class();

        let class_id = self.alloc_class();
        self.classes.insert(key, class_id);
        self.class_data[class_id.index()].qualified_name = "__builtin__.StopIteration".to_string();

        // Set Exception as parent
        self.set_parent(class_id, exception_class_id);

        let str_class_id = self.get_or_create_str_class();
        let str_type = TirType::Class(str_class_id);
        let exc_type = TirType::Class(class_id);

        register_methods!(self, class_id, "StopIteration",
            // __init__ takes no args (or optional value, but we'll keep it simple)
            shared "__init__" => (vec![], exc_type),
            shared "__str__" => (vec![], str_type.clone()),
            shared "__repr__" => (vec![], str_type),
        );

        class_id
    }
}
