use crate::ext::ReadByteVecExt;
use byteorder::ReadBytesExt;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io::{ErrorKind, Read};

macro_rules! impl_ops {
    [$(($op: ident, $value: literal, $name: literal, $argc: literal)),+] => {
        #[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
        #[non_exhaustive]
        #[repr(u8)]
        pub enum Op {
            $($op = $value),
            +
        }

        impl Op {
            pub fn name(&self) -> &'static str {
                match self {
                    $(Op::$op => $name),
                    +
                }
            }

            pub fn argc(&self) -> usize {
                match self {
                    $(Op::$op => $argc),
                    +
                }
            }
        }
    };
}

impl_ops![
    (Aaload, 0x32, "aaload", 0),
    (Aastore, 0x53, "aastore", 0),
    (AconstNull, 0x01, "aconst_null", 0),
    (Aload, 0x19, "aload", 1),
    (Aload0, 0x2a, "aload_0", 0),
    (Aload1, 0x2b, "aload_1", 0),
    (Aload2, 0x2c, "aload_2", 0),
    (Aload3, 0x2d, "aload_3", 0),
    (Anewarray, 0xbd, "anewarray", 2),
    (Areturn, 0xb0, "areturn", 0),
    (Arraylength, 0xbe, "arraylength", 0),
    (Astore, 0x3a, "astore", 1),
    (Astore0, 0x4b, "astore_0", 0),
    (Astore1, 0x4c, "astore_1", 0),
    (Astore2, 0x4d, "astore_2", 0),
    (Astore3, 0x4e, "astore_3", 0),
    (Athrow, 0xbf, "athrow", 0),
    (Baload, 0x33, "baload", 0),
    (Bastore, 0x54, "bastore", 0),
    (Bipush, 0x10, "bipush", 1),
    (Breakpoint, 0xca, "breakpoint", 0),
    (Caload, 0x34, "caload", 0),
    (Castore, 0x55, "castore", 0),
    (Checkcast, 0xc0, "checkcast", 2),
    (D2f, 0x90, "d_2f", 0),
    (D2i, 0x8e, "d_2i", 0),
    (D2l, 0x8f, "d_2l", 0),
    (Dadd, 0x63, "dadd", 0),
    (Daload, 0x31, "daload", 0),
    (Dastore, 0x52, "dastore", 0),
    (Dcmpg, 0x98, "dcmpg", 0),
    (Dcmpl, 0x97, "dcmpl", 0),
    (Dconst0, 0x0e, "dconst_0", 0),
    (Dconst1, 0x0f, "dconst_1", 0),
    (Ddiv, 0x6f, "ddiv", 0),
    (Dload, 0x18, "dload", 1),
    (Dload0, 0x26, "dload_0", 0),
    (Dload1, 0x27, "dload_1", 0),
    (Dload2, 0x28, "dload_2", 0),
    (Dload3, 0x29, "dload_3", 0),
    (Dmul, 0x6b, "dmul", 0),
    (Dneg, 0x77, "dneg", 0),
    (Drem, 0x73, "drem", 0),
    (Dreturn, 0xaf, "dreturn", 0),
    (Dstore, 0x39, "dstore", 1),
    (Dstore0, 0x47, "dstore_0", 0),
    (Dstore1, 0x48, "dstore_1", 0),
    (Dstore2, 0x49, "dstore_2", 0),
    (Dstore3, 0x4a, "dstore_3", 0),
    (Dsub, 0x67, "dsub", 0),
    (Dup, 0x59, "dup", 0),
    (DupX1, 0x5a, "dupX1", 0),
    (DupX2, 0x5b, "dupX2", 0),
    (Dup2, 0x5c, "dup_2", 0),
    (Dup2X1, 0x5d, "dup2X1", 0),
    (Dup2X2, 0x5e, "dup2X2", 0),
    (F2d, 0x8d, "f_2d", 0),
    (F2i, 0x8b, "f_2i", 0),
    (F2l, 0x8c, "f_2l", 0),
    (Fadd, 0x62, "fadd", 0),
    (Faload, 0x30, "faload", 0),
    (Fastore, 0x51, "fastore", 0),
    (Fcmpg, 0x96, "fcmpg", 0),
    (Fcmpl, 0x95, "fcmpl", 0),
    (Fconst0, 0x0b, "fconst_0", 0),
    (Fconst1, 0x0c, "fconst_1", 0),
    (Fconst2, 0x0d, "fconst_2", 0),
    (Fdiv, 0x6e, "fdiv", 0),
    (Fload, 0x17, "fload", 1),
    (Fload0, 0x22, "fload_0", 0),
    (Fload1, 0x23, "fload_1", 0),
    (Fload2, 0x24, "fload_2", 0),
    (Fload3, 0x25, "fload_3", 0),
    (Fmul, 0x6a, "fmul", 0),
    (Fneg, 0x76, "fneg", 0),
    (Frem, 0x72, "frem", 0),
    (Freturn, 0xae, "freturn", 0),
    (Fstore, 0x38, "fstore", 1),
    (Fstore0, 0x43, "fstore_0", 0),
    (Fstore1, 0x44, "fstore_1", 0),
    (Fstore2, 0x45, "fstore_2", 0),
    (Fstore3, 0x46, "fstore_3", 0),
    (Fsub, 0x66, "fsub", 0),
    (Getfield, 0xb4, "getfield", 2),
    (Getstatic, 0xb2, "getstatic", 2),
    (Goto, 0xa7, "goto", 2),
    (GotoW, 0xc8, "goto_w", 4),
    (I2b, 0x91, "i_2b", 0),
    (I2c, 0x92, "i_2c", 0),
    (I2d, 0x87, "i_2d", 0),
    (I2f, 0x86, "i_2f", 0),
    (I2l, 0x85, "i_2l", 0),
    (I2s, 0x93, "i_2s", 0),
    (Iadd, 0x60, "iadd", 0),
    (Iaload, 0x2e, "iaload", 0),
    (Iand, 0x7e, "iand", 0),
    (Iastore, 0x4f, "iastore", 0),
    (IconstM1, 0x02, "iconstM1", 0),
    (Iconst0, 0x03, "iconst_0", 0),
    (Iconst1, 0x04, "iconst_1", 0),
    (Iconst2, 0x05, "iconst_2", 0),
    (Iconst3, 0x06, "iconst_3", 0),
    (Iconst4, 0x07, "iconst_4", 0),
    (Iconst5, 0x08, "iconst_5", 0),
    (Idiv, 0x6c, "idiv", 0),
    (IfAcmpeq, 0xa5, "if_acmpeq", 2),
    (IfAcmpne, 0xa6, "if_acmpne", 2),
    (IfIcmpeq, 0x9f, "if_icmpeq", 2),
    (IfIcmpge, 0xa2, "if_icmpge", 2),
    (IfIcmpgt, 0xa3, "if_icmpgt", 2),
    (IfIcmple, 0xa4, "if_icmple", 2),
    (IfIcmplt, 0xa1, "if_icmplt", 2),
    (IfIcmpne, 0xa0, "if_icmpne", 2),
    (Ifeq, 0x99, "ifeq", 2),
    (Ifge, 0x9c, "ifge", 2),
    (Ifgt, 0x9d, "ifgt", 2),
    (Ifle, 0x9e, "ifle", 2),
    (Iflt, 0x9b, "iflt", 2),
    (Ifne, 0x9a, "ifne", 2),
    (Ifnonnull, 0xc7, "ifnonnull", 2),
    (Ifnull, 0xc6, "ifnull", 2),
    (Iinc, 0x84, "iinc", 2),
    (Iload, 0x15, "iload", 1),
    (Iload0, 0x1a, "iload_0", 0),
    (Iload1, 0x1b, "iload_1", 0),
    (Iload2, 0x1c, "iload_2", 0),
    (Iload3, 0x1d, "iload_3", 0),
    (Impdep1, 0xfe, "impdep_1", 0),
    (Impdep2, 0xff, "impdep_2", 0),
    (Imul, 0x68, "imul", 0),
    (Ineg, 0x74, "ineg", 0),
    (Instanceof, 0xc1, "instanceof", 2),
    (Invokedynamic, 0xba, "invokedynamic", 4),
    (Invokeinterface, 0xb9, "invokeinterface", 4),
    (Invokespecial, 0xb7, "invokespecial", 2),
    (Invokestatic, 0xb8, "invokestatic", 2),
    (Invokevirtual, 0xb6, "invokevirtual", 2),
    (Ior, 0x80, "ior", 0),
    (Irem, 0x70, "irem", 0),
    (Ireturn, 0xac, "ireturn", 0),
    (Ishl, 0x78, "ishl", 0),
    (Ishr, 0x7a, "ishr", 0),
    (Istore, 0x36, "istore", 1),
    (Istore0, 0x3b, "istore_0", 0),
    (Istore1, 0x3c, "istore_1", 0),
    (Istore2, 0x3d, "istore_2", 0),
    (Istore3, 0x3e, "istore_3", 0),
    (Isub, 0x64, "isub", 0),
    (Iushr, 0x7c, "iushr", 0),
    (Ixor, 0x82, "ixor", 0),
    (L2d, 0x8a, "l_2d", 0),
    (L2f, 0x89, "l_2f", 0),
    (L2i, 0x88, "l_2i", 0),
    (Ladd, 0x61, "ladd", 0),
    (Laload, 0x2f, "laload", 0),
    (Land, 0x7f, "land", 0),
    (Lastore, 0x50, "lastore", 0),
    (Lcmp, 0x94, "lcmp", 0),
    (Lconst0, 0x09, "lconst_0", 0),
    (Lconst1, 0x0a, "lconst_1", 0),
    (Ldc, 0x12, "ldc", 1),
    (LdcW, 0x13, "ldc_w", 2),
    (Ldc2W, 0x14, "ldc2W", 2),
    (Ldiv, 0x6d, "ldiv", 0),
    (Lload, 0x16, "lload", 1),
    (Lload0, 0x1e, "lload_0", 0),
    (Lload1, 0x1f, "lload_1", 0),
    (Lload2, 0x20, "lload_2", 0),
    (Lload3, 0x21, "lload_3", 0),
    (Lmul, 0x69, "lmul", 0),
    (Lneg, 0x75, "lneg", 0),
    (Lookupswitch, 0xab, "lookupswitch", 0),
    (Lor, 0x81, "lor", 0),
    (Lrem, 0x71, "lrem", 0),
    (Lreturn, 0xad, "lreturn", 0),
    (Lshl, 0x79, "lshl", 0),
    (Lshr, 0x7b, "lshr", 0),
    (Lstore, 0x37, "lstore", 1),
    (Lstore0, 0x3f, "lstore_0", 0),
    (Lstore1, 0x40, "lstore_1", 0),
    (Lstore2, 0x41, "lstore_2", 0),
    (Lstore3, 0x42, "lstore_3", 0),
    (Lsub, 0x65, "lsub", 0),
    (Lushr, 0x7d, "lushr", 0),
    (Lxor, 0x83, "lxor", 0),
    (Monitorenter, 0xc2, "monitorenter", 0),
    (Monitorexit, 0xc3, "monitorexit", 0),
    (Multianewarray, 0xc5, "multianewarray", 3),
    (New, 0xbb, "new", 2),
    (Newarray, 0xbc, "newarray", 1),
    (Nop, 0x00, "nop", 0),
    (Pop, 0x57, "pop", 0),
    (Pop2, 0x58, "pop_2", 0),
    (Putfield, 0xb5, "putfield", 2),
    (Putstatic, 0xb3, "putstatic", 2),
    (Return, 0xb1, "return", 0),
    (Saload, 0x35, "saload", 0),
    (Sastore, 0x56, "sastore", 0),
    (Sipush, 0x11, "sipush", 2),
    (Swap, 0x5f, "swap", 0)
];

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Instruction {
    pub op: Op,
    pub args: Vec<u8>,
}

impl Instruction {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Instruction, std::io::Error> {
        let op = Op::try_from(r.read_u8()?)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

        let args = r.read_byte_vec(op.argc())?;

        Ok(Instruction { op, args })
    }

    pub fn from_bytecode(code: impl AsRef<[u8]>) -> Vec<Instruction> {
        let code = code.as_ref();
        let mut instructions = Vec::with_capacity(code.len());

        let mut pos = 0;
        while pos < code.len() {
            let op = Op::try_from(code[pos]).unwrap();
            let mut instruction = Instruction {
                op,
                args: Vec::with_capacity(op.argc()),
            };
            let argc = op.argc();
            for offset in 0..argc {
                instruction.args.push(code[pos + offset]);
            }
            instructions.push(instruction);
            pos += 1 + argc;
        }

        instructions
    }
}
