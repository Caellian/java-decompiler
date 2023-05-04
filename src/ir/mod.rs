use std::collections::HashMap;

use jvm_class_format::Constant;

pub struct RuntimePool {
    pub constant_pool: HashMap<u16, Constant>,
}
