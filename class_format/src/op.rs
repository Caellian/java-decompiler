use byteorder::ReadBytesExt;
use num_enum::{IntoPrimitive, TryFromPrimitive};
use std::convert::TryFrom;
use std::fmt::{Display, Formatter};
use std::io::{ErrorKind, Read};

macro_rules! impl_ops {
    [$(($op: ident, $code: literal, $argc: literal)),+] => {paste::paste!{
        #[derive(Debug, Copy, Clone, Eq, PartialEq, IntoPrimitive, TryFromPrimitive)]
        #[non_exhaustive]
        #[repr(u8)]
        pub enum Op {
            $([<$op:camel>] = $code),
            +
        }

        const OP_NAMES: [&'static str; 256] = {
            let mut r = [""; 256];
            $(r[Op::[<$op:camel>] as u8 as usize] = stringify!($op);)
            +
            r
        };

        const OP_ARGC: [u8; 256] = {
            let mut r = [0; 256];
            $(r[Op::[<$op:camel>] as u8 as usize] = $argc;)
            +
            r
        };

        impl Op {
            pub const fn name(&self) -> &'static str {
                OP_NAMES[*self as u8 as usize]
            }

            pub const fn argc(&self) -> usize {
                OP_ARGC[*self as u8 as usize] as usize
            }
        }
    }};
}

//   Instruction      Code  ArgC
#[rustfmt::skip]
impl_ops![
    (aaload,          0x32, 0),
    (aastore,         0x53, 0),
    (aconst_null,     0x01, 0),
    (aload,           0x19, 1),
    (aload_0,         0x2a, 0),
    (aload_1,         0x2b, 0),
    (aload_2,         0x2c, 0),
    (aload_3,         0x2d, 0),
    (anewarray,       0xbd, 2),
    (areturn,         0xb0, 0),
    (arraylength,     0xbe, 0),
    (astore,          0x3a, 1),
    (astore_0,        0x4b, 0),
    (astore_1,        0x4c, 0),
    (astore_2,        0x4d, 0),
    (astore_3,        0x4e, 0),
    (athrow,          0xbf, 0),
    (baload,          0x33, 0),
    (bastore,         0x54, 0),
    (bipush,          0x10, 1),
    (breakpoint,      0xca, 0),
    (caload,          0x34, 0),
    (castore,         0x55, 0),
    (checkcast,       0xc0, 2),
    (d_2f,            0x90, 0),
    (d_2i,            0x8e, 0),
    (d_2l,            0x8f, 0),
    (dadd,            0x63, 0),
    (daload,          0x31, 0),
    (dastore,         0x52, 0),
    (dcmpg,           0x98, 0),
    (dcmpl,           0x97, 0),
    (dconst_0,        0x0e, 0),
    (dconst_1,        0x0f, 0),
    (ddiv,            0x6f, 0),
    (dload,           0x18, 1),
    (dload_0,         0x26, 0),
    (dload_1,         0x27, 0),
    (dload_2,         0x28, 0),
    (dload_3,         0x29, 0),
    (dmul,            0x6b, 0),
    (dneg,            0x77, 0),
    (drem,            0x73, 0),
    (dreturn,         0xaf, 0),
    (dstore,          0x39, 1),
    (dstore_0,        0x47, 0),
    (dstore_1,        0x48, 0),
    (dstore_2,        0x49, 0),
    (dstore_3,        0x4a, 0),
    (dsub,            0x67, 0),
    (dup,             0x59, 0),
    (dupX1,           0x5a, 0),
    (dupX2,           0x5b, 0),
    (dup_2,           0x5c, 0),
    (dup2X1,          0x5d, 0),
    (dup2X2,          0x5e, 0),
    (f_2d,            0x8d, 0),
    (f_2i,            0x8b, 0),
    (f_2l,            0x8c, 0),
    (fadd,            0x62, 0),
    (faload,          0x30, 0),
    (fastore,         0x51, 0),
    (fcmpg,           0x96, 0),
    (fcmpl,           0x95, 0),
    (fconst_0,        0x0b, 0),
    (fconst_1,        0x0c, 0),
    (fconst_2,        0x0d, 0),
    (fdiv,            0x6e, 0),
    (fload,           0x17, 1),
    (fload_0,         0x22, 0),
    (fload_1,         0x23, 0),
    (fload_2,         0x24, 0),
    (fload_3,         0x25, 0),
    (fmul,            0x6a, 0),
    (fneg,            0x76, 0),
    (frem,            0x72, 0),
    (freturn,         0xae, 0),
    (fstore,          0x38, 1),
    (fstore_0,        0x43, 0),
    (fstore_1,        0x44, 0),
    (fstore_2,        0x45, 0),
    (fstore_3,        0x46, 0),
    (fsub,            0x66, 0),
    (getfield,        0xb4, 2),
    (getstatic,       0xb2, 2),
    (goto,            0xa7, 2),
    (goto_w,          0xc8, 4),
    (i_2b,            0x91, 0),
    (i_2c,            0x92, 0),
    (i_2d,            0x87, 0),
    (i_2f,            0x86, 0),
    (i_2l,            0x85, 0),
    (i_2s,            0x93, 0),
    (iadd,            0x60, 0),
    (iaload,          0x2e, 0),
    (iand,            0x7e, 0),
    (iastore,         0x4f, 0),
    (iconstM1,        0x02, 0),
    (iconst_0,        0x03, 0),
    (iconst_1,        0x04, 0),
    (iconst_2,        0x05, 0),
    (iconst_3,        0x06, 0),
    (iconst_4,        0x07, 0),
    (iconst_5,        0x08, 0),
    (idiv,            0x6c, 0),
    (if_acmpeq,       0xa5, 2),
    (if_acmpne,       0xa6, 2),
    (if_icmpeq,       0x9f, 2),
    (if_icmpge,       0xa2, 2),
    (if_icmpgt,       0xa3, 2),
    (if_icmple,       0xa4, 2),
    (if_icmplt,       0xa1, 2),
    (if_icmpne,       0xa0, 2),
    (ifeq,            0x99, 2),
    (ifge,            0x9c, 2),
    (ifgt,            0x9d, 2),
    (ifle,            0x9e, 2),
    (iflt,            0x9b, 2),
    (ifne,            0x9a, 2),
    (ifnonnull,       0xc7, 2),
    (ifnull,          0xc6, 2),
    (iinc,            0x84, 2),
    (iload,           0x15, 1),
    (iload_0,         0x1a, 0),
    (iload_1,         0x1b, 0),
    (iload_2,         0x1c, 0),
    (iload_3,         0x1d, 0),
    (impdep_1,        0xfe, 0),
    (impdep_2,        0xff, 0),
    (imul,            0x68, 0),
    (ineg,            0x74, 0),
    (instanceof,      0xc1, 2),
    (invokedynamic,   0xba, 4),
    (invokeinterface, 0xb9, 4),
    (invokespecial,   0xb7, 2),
    (invokestatic,    0xb8, 2),
    (invokevirtual,   0xb6, 2),
    (ior,             0x80, 0),
    (irem,            0x70, 0),
    (ireturn,         0xac, 0),
    (ishl,            0x78, 0),
    (ishr,            0x7a, 0),
    (istore,          0x36, 1),
    (istore_0,        0x3b, 0),
    (istore_1,        0x3c, 0),
    (istore_2,        0x3d, 0),
    (istore_3,        0x3e, 0),
    (isub,            0x64, 0),
    (iushr,           0x7c, 0),
    (ixor,            0x82, 0),
    (l_2d,            0x8a, 0),
    (l_2f,            0x89, 0),
    (l_2i,            0x88, 0),
    (ladd,            0x61, 0),
    (laload,          0x2f, 0),
    (land,            0x7f, 0),
    (lastore,         0x50, 0),
    (lcmp,            0x94, 0),
    (lconst_0,        0x09, 0),
    (lconst_1,        0x0a, 0),
    (ldc,             0x12, 1),
    (ldc_w,           0x13, 2),
    (ldc2W,           0x14, 2),
    (ldiv,            0x6d, 0),
    (lload,           0x16, 1),
    (lload_0,         0x1e, 0),
    (lload_1,         0x1f, 0),
    (lload_2,         0x20, 0),
    (lload_3,         0x21, 0),
    (lmul,            0x69, 0),
    (lneg,            0x75, 0),
    (lookupswitch,    0xab, 0),
    (lor,             0x81, 0),
    (lrem,            0x71, 0),
    (lreturn,         0xad, 0),
    (lshl,            0x79, 0),
    (lshr,            0x7b, 0),
    (lstore,          0x37, 1),
    (lstore_0,        0x3f, 0),
    (lstore_1,        0x40, 0),
    (lstore_2,        0x41, 0),
    (lstore_3,        0x42, 0),
    (lsub,            0x65, 0),
    (lushr,           0x7d, 0),
    (lxor,            0x83, 0),
    (monitorenter,    0xc2, 0),
    (monitorexit,     0xc3, 0),
    (multianewarray,  0xc5, 3),
    (new,             0xbb, 2),
    (newarray,        0xbc, 1),
    (nop,             0x00, 0),
    (pop,             0x57, 0),
    (pop_2,           0x58, 0),
    (putfield,        0xb5, 2),
    (putstatic,       0xb3, 2),
    (return,          0xb1, 0),
    (saload,          0x35, 0),
    (sastore,         0x56, 0),
    (sipush,          0x11, 2),
    (swap,            0x5f, 0)
];

impl Display for Op {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.name())
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Instruction {
    pub op: Op,
    pub args: [u8; 4],
}

impl Instruction {
    pub fn read_from<R: Read>(r: &mut R) -> Result<Instruction, std::io::Error> {
        let op = Op::try_from(r.read_u8()?)
            .map_err(|e| std::io::Error::new(ErrorKind::InvalidData, e))?;

        let mut args = [0; 4];
        r.read_exact(&mut args[..op.argc()])?;

        Ok(Instruction { op, args })
    }

    pub fn from_bytecode(code: impl AsRef<[u8]>) -> Vec<Instruction> {
        let code = code.as_ref();
        let mut instructions = Vec::with_capacity(code.len());

        let mut pos = 0;
        while pos < code.len() {
            let op = Op::try_from(code[pos]).unwrap();
            pos += 1;
            let mut instruction = Instruction { op, args: [0; 4] };

            let argc = op.argc();
            for offset in 0..argc {
                instruction.args[offset] = code[pos + offset];
            }
            instructions.push(instruction);
            pos += argc;
        }

        instructions
    }
}
