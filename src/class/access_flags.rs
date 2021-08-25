use crate::error::AccessFlagError;
use byteorder::{ReadBytesExt, BE};
use std::io::Read;
bitflags! {
    pub struct AccessFlags: u16 {
        /// Declared public; may be accessed from outside its package.
        const PUBLIC = 0x0001;

        /// Declared private; accessible only within the
        /// defining class and other classes belonging to the same nest.
        const PRIVATE = 0x0002;

        /// Declared protected; may be accessed within subclasses.
        const PROTECTED = 0x0004;

        /// Declared static.
        const STATIC = 0x0008;

        /// Declared final; never directly assigned to after object construction.
        const FINAL = 0x0010;

        /// Treat superclass methods specially when invoked by
        /// the invokespecial instruction.
        const SUPER = 0x0020;

        /// Declared final; no subclasses allowed.
        const VOLATILE = 0x0040;

        /// Declared transient; not written or read by a
        /// persistent object manager.
        const TRANSIENT = 0x0080;

        /// Declared native; implemented in a language other
        /// than the Java programming language.
        const NATIVE = 0x0100;

        /// Is an interface, not a class.
        const INTERFACE = 0x0200;

        /// Declared abstract; must not be instantiated.
        const ABSTRACT = 0x0400;

        /// Declared strictfp; floating-point mode is FP-strict.
        const STRICT = 0x0800;

        /// Declared synthetic; not present in the source code.
        const SYNTHETIC = 0x1000;

        /// Declared as an annotation interface.
        const ANNOTATION = 0x2000;

        /// Declared as an enum class.
        const ENUM = 0x4000;

        /// Is a module, not a class or interface.
        const MODULE = 0x8000;
    }
}

impl AccessFlags {
    pub fn read_from<R: Read>(r: &mut R) -> Result<AccessFlags, AccessFlagError> {
        let found = r.read_u16::<BE>()?;
        AccessFlags::from_bits(found).ok_or(AccessFlagError::InvalidValue { found })
    }
}
