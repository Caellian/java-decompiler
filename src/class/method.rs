use num_enum::{IntoPrimitive, TryFromPrimitive};

#[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum MethodHandle {
    GetField = 1,
    GetStatic = 2,
    PutField = 3,
    PutStatic = 4,
    InvokeVirtual = 5,
    InvokeStatic = 6,
    InvokeSpecial = 7,
    NewInvokeSpecial = 8,
    InvokeInterface = 9,
}

impl MethodHandle {
    #[allow(dead_code)]
    fn name(&self) -> String {
        match self {
            MethodHandle::GetField => "GetField",
            MethodHandle::GetStatic => "GetStatic",
            MethodHandle::PutField => "PutField",
            MethodHandle::PutStatic => "PutStatic",
            MethodHandle::InvokeVirtual => "InvokeVirtual",
            MethodHandle::InvokeStatic => "InvokeStatic",
            MethodHandle::InvokeSpecial => "InvokeSpecial",
            MethodHandle::NewInvokeSpecial => "NewInvokeSpecial",
            MethodHandle::InvokeInterface => "InvokeInterface",
        }
        .to_string()
    }
}
