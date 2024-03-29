use crate::error::ErrorType;
use std::convert::TryFrom;

#[derive(Debug)]
#[allow(non_camel_case_types)]
pub enum Instruction {
    aload_0,
    aload_1,
    aload_2,
    aload_3,
    astore {
        index: u8
    },
    astore_0,
    astore_1,
    astore_2,
    astore_3,
    dup,
    bipush {
        byte: u8
    },
    invokedynamic {
        indexbyte1: u8,
        indexbyte2: u8,
        byte3: u8,
        byte4: u8
    },
    invokespecial {
        indexbyte1: u8,
        indexbyte2: u8
    },
    invokestatic {
        indexbyte1: u8,
        indexbyte2: u8
    },
    invokevirtual {
        indexbyte1: u8,
        indexbyte2: u8
    },
    getstatic {
        indexbyte1: u8,
        indexbyte2: u8
    },
    new {
        indexbyte1: u8,
        indexbyte2: u8
    },
    r#eturn,
    ldc {
        index: u8
    },
    nop
}

#[repr(u8)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[allow(non_camel_case_types)]
pub enum Opcode {
    aaload = 0x32,
    aastore = 0x53,
    aconst_null = 0x1,
    aload = 0x19,
    aload_0 = 0x2a,
    aload_1 = 0x2b,
    aload_2 = 0x2c,
    aload_3 = 0x2d,
    anewarray = 0xbd,
    areturn = 0xb0,
    arraylength = 0xbe,
    astore = 0x3a,
    astore_0 = 0x4b,
    astore_1 = 0x4c,
    astore_2 = 0x4d,
    astore_3 = 0x4e,
    athrow = 0xbf,
    baload = 0x33,
    bastore = 0x54,
    bipush = 0x10,
    caload = 0x34,
    castore = 0x55,
    checkcast = 0xc0,
    d2f = 0x90,
    d2i = 0x8e,
    d2l = 0x8f,
    dadd = 0x63,
    daload = 0x31,
    dastore = 0x52,
    dcmpg = 0x98,
    dcmpl = 0x97,
    dconst_0 = 0x0e,
    dconst_1 = 0x0f,
    ddiv = 0x6f,
    dload = 0x18,
    dload_0 = 0x26,
    dload_1 = 0x27,
    dload_2 = 0x28,
    dload_3 = 0x29,
    dmul = 0x6b,
    dneg = 0x77,
    drem = 0x73,
    dreturn = 0xaf,
    dstore = 0x39,
    dstore_0 = 0x47,
    dstore_1 = 0x48,
    dstore_2 = 0x49,
    dstore_3 = 0x4a,
    dsub = 0x67,
    dup = 0x59,
    dup_x1 = 0x5a,
    dup_x2 = 0x5b,
    dup2 = 0x5c,
    dup2_x1 = 0x5d,
    dup2_x2 = 0x5e,
    f2d = 0x8d,
    f2i = 0x8b,
    f2l = 0x8c,
    fadd = 0x62,
    faload = 0x30,
    fastore = 0x51,
    fcmpg = 0x96,
    fcmpl = 0x95,
    fconst_0 = 0xb,
    fconst_1 = 0xc,
    fconst_2 = 0xd,
    fdiv = 0x6e,
    fload = 0x17,
    fload_0 = 0x22,
    fload_1 = 0x23,
    fload_2 = 0x24,
    fload_3 = 0x25,
    fmul = 0x6a,
    fneg = 0x76,
    frem = 0x72,
    freturn = 0xae,
    fstore = 0x38,
    fstore_0 = 0x43,
    fstore_1 = 0x44,
    fstore_2 = 0x45,
    fstore_3 = 0x46,
    fsub = 0x66,
    getfield = 0xb4,
    getstatic = 0xb2,
    goto = 0xa7,
    goto_w = 0xc8,
    i2b = 0x91,
    i2c = 0x92,
    i2d = 0x87,
    i2f = 0x86,
    i2l = 0x85,
    i2s = 0x93,
    iadd = 0x60,
    iaload = 0x2e,
    iand = 0x7e,
    iastore = 0x4f,
    iconst_m1 = 0x2,
    iconst_0 = 0x3,
    iconst_1 = 0x4,
    iconst_2 = 0x5,
    iconst_3 = 0x6,
    iconst_4 = 0x7,
    iconst_5 = 0x8,
    idiv = 0x6c,
    if_acmpeq = 0xa5,
    if_acmpne = 0xa6,
    if_icmpeq = 0x9f,
    if_icmpne = 0xa0,
    if_icmplt = 0xa1,
    if_icmpge = 0xa2,
    if_icmpgt = 0xa3,
    if_icmple = 0xa4,
    ifeq = 0x99,
    ifne = 0x9a,
    iflt = 0x9b,
    ifge = 0x9c,
    ifgt = 0x9d,
    ifle = 0x9e,
    ifnonnull = 0xc7,
    ifnull = 0xc6,
    iinc = 0x84,
    iload = 0x15,
    iload_0 = 0x1a,
    iload_1 = 0x1b,
    iload_2 = 0x1c,
    iload_3 = 0x1d,
    imul = 0x68,
    ineg = 0x74,
    instanceof = 0xc1,
    invokedynamic = 0xba,
    invokeinterface = 0xb9,
    invokespecial = 0xb7,
    invokestatic = 0xb8,
    invokevirtual = 0xb6,
    ior = 0x80,
    irem = 0x70,
    ireturn = 0xac,
    ishl = 0x78,
    ishr = 0x7a,
    istore = 0x36,
    istore_0 = 0x3b,
    istore_1 = 0x3c,
    istore_2 = 0x3d,
    istore_3 = 0x3e,
    isub = 0x64,
    iushr = 0x7c,
    ixor = 0x82,
    jsr = 0xa8,
    jsr_w = 0xc9,
    l2d = 0x8a,
    l2f = 0x89,
    l2i = 0x88,
    ladd = 0x61,
    laload = 0x2f,
    land = 0x7f,
    lastore = 0x50,
    lcmp = 0x94,
    lconst_0 = 0x9,
    lconst_1 = 0xa,
    ldc = 0x12,
    ldc_w = 0x13,
    ldc2_w = 0x14,
    ldiv = 0x6d,
    lload = 0x16,
    lload_0 = 0x1e,
    lload_1 = 0x1f,
    lload_2 = 0x20,
    lload_3 = 0x21,
    lmul = 0x69,
    lneg = 0x75,
    lookupswitch = 0xab,
    lor = 0x81,
    lrem = 0x71,
    lreturn = 0xad,
    lshl = 0x79,
    lshr = 0x7b,
    lstore = 0x37,
    lstore_0 = 0x3f,
    lstore_1 = 0x40,
    lstore_2 = 0x41,
    lstore_3 = 0x42,
    lsub = 0x65,
    lushr = 0x7d,
    lxor = 0x83,
    monitorenter = 0xc2,
    monitorexit = 0xc3,
    multianewarray = 0xc5,
    new = 0xbb,
    newarray = 0xbc,
    nop = 0x0,
    pop = 0x57,
    pop2 = 0x58,
    putfield = 0xb5,
    putstatic = 0xb3,
    ret = 0xa9,
    r#eturn = 0xb1,
    saload = 0x35,
    sastore = 0x56,
    sipush = 0x11,
    swap = 0x5f,
    tableswitch = 0xaa,
    wide = 0xc4
}

impl TryFrom<u8> for Opcode {
    type Error = ErrorType;

    fn try_from(v: u8) -> Result<Opcode, Self::Error> {
        match v {
            x if x == Opcode::aaload as u8 => Ok(Opcode::aaload),
            x if x == Opcode::aastore as u8 => Ok(Opcode::aastore),
            x if x == Opcode::aconst_null as u8 => Ok(Opcode::aconst_null),
            x if x == Opcode::aload as u8 => Ok(Opcode::aload),
            x if x == Opcode::aload_0 as u8 => Ok(Opcode::aload_0),
            x if x == Opcode::aload_1 as u8 => Ok(Opcode::aload_1),
            x if x == Opcode::aload_2 as u8 => Ok(Opcode::aload_2),
            x if x == Opcode::aload_3 as u8 => Ok(Opcode::aload_3),
            x if x == Opcode::anewarray as u8 => Ok(Opcode::anewarray),
            x if x == Opcode::areturn as u8 => Ok(Opcode::areturn),
            x if x == Opcode::arraylength as u8 => Ok(Opcode::arraylength),
            x if x == Opcode::astore as u8 => Ok(Opcode::astore),
            x if x == Opcode::astore_0 as u8 => Ok(Opcode::astore_0),
            x if x == Opcode::astore_1 as u8 => Ok(Opcode::astore_1),
            x if x == Opcode::astore_2 as u8 => Ok(Opcode::astore_2),
            x if x == Opcode::astore_3 as u8 => Ok(Opcode::astore_3),
            x if x == Opcode::athrow as u8 => Ok(Opcode::athrow),
            x if x == Opcode::baload as u8 => Ok(Opcode::baload),
            x if x == Opcode::bastore as u8 => Ok(Opcode::bastore),
            x if x == Opcode::bipush as u8 => Ok(Opcode::bipush),
            x if x == Opcode::caload as u8 => Ok(Opcode::caload),
            x if x == Opcode::castore as u8 => Ok(Opcode::castore),
            x if x == Opcode::checkcast as u8 => Ok(Opcode::checkcast),
            x if x == Opcode::d2f as u8 => Ok(Opcode::d2f),
            x if x == Opcode::d2i as u8 => Ok(Opcode::d2i),
            x if x == Opcode::d2l as u8 => Ok(Opcode::d2l),
            x if x == Opcode::dadd as u8 => Ok(Opcode::dadd),
            x if x == Opcode::daload as u8 => Ok(Opcode::daload),
            x if x == Opcode::dastore as u8 => Ok(Opcode::dastore),
            x if x == Opcode::dcmpg as u8 => Ok(Opcode::dcmpg),
            x if x == Opcode::dcmpl as u8 => Ok(Opcode::dcmpl),
            x if x == Opcode::dconst_0 as u8 => Ok(Opcode::dconst_0),
            x if x == Opcode::dconst_1 as u8 => Ok(Opcode::dconst_1),
            x if x == Opcode::ddiv as u8 => Ok(Opcode::ddiv),
            x if x == Opcode::dload as u8 => Ok(Opcode::dload),
            x if x == Opcode::dload_0 as u8 => Ok(Opcode::dload_0),
            x if x == Opcode::dload_1 as u8 => Ok(Opcode::dload_1),
            x if x == Opcode::dload_2 as u8 => Ok(Opcode::dload_2),
            x if x == Opcode::dload_3 as u8 => Ok(Opcode::dload_3),
            x if x == Opcode::dmul as u8 => Ok(Opcode::dmul),
            x if x == Opcode::dneg as u8 => Ok(Opcode::dneg),
            x if x == Opcode::drem as u8 => Ok(Opcode::drem),
            x if x == Opcode::dreturn as u8 => Ok(Opcode::dreturn),
            x if x == Opcode::dstore as u8 => Ok(Opcode::dstore),
            x if x == Opcode::dstore_0 as u8 => Ok(Opcode::dstore_0),
            x if x == Opcode::dstore_1 as u8 => Ok(Opcode::dstore_1),
            x if x == Opcode::dstore_2 as u8 => Ok(Opcode::dstore_2),
            x if x == Opcode::dstore_3 as u8 => Ok(Opcode::dstore_3),
            x if x == Opcode::dsub as u8 => Ok(Opcode::dsub),
            x if x == Opcode::dup as u8 => Ok(Opcode::dup),
            x if x == Opcode::dup_x1 as u8 => Ok(Opcode::dup_x1),
            x if x == Opcode::dup_x2 as u8 => Ok(Opcode::dup_x2),
            x if x == Opcode::dup2 as u8 => Ok(Opcode::dup2),
            x if x == Opcode::dup2_x1 as u8 => Ok(Opcode::dup2_x1),
            x if x == Opcode::dup2_x2 as u8 => Ok(Opcode::dup2_x2),
            x if x == Opcode::f2d as u8 => Ok(Opcode::f2d),
            x if x == Opcode::f2i as u8 => Ok(Opcode::f2i),
            x if x == Opcode::f2l as u8 => Ok(Opcode::f2l),
            x if x == Opcode::fadd as u8 => Ok(Opcode::fadd),
            x if x == Opcode::faload as u8 => Ok(Opcode::faload),
            x if x == Opcode::fastore as u8 => Ok(Opcode::fastore),
            x if x == Opcode::fcmpg as u8 => Ok(Opcode::fcmpg),
            x if x == Opcode::fcmpl as u8 => Ok(Opcode::fcmpl),
            x if x == Opcode::fconst_0 as u8 => Ok(Opcode::fconst_0),
            x if x == Opcode::fconst_1 as u8 => Ok(Opcode::fconst_1),
            x if x == Opcode::fconst_2 as u8 => Ok(Opcode::fconst_2),
            x if x == Opcode::fdiv as u8 => Ok(Opcode::fdiv),
            x if x == Opcode::fload as u8 => Ok(Opcode::fload),
            x if x == Opcode::fload_0 as u8 => Ok(Opcode::fload_0),
            x if x == Opcode::fload_1 as u8 => Ok(Opcode::fload_1),
            x if x == Opcode::fload_2 as u8 => Ok(Opcode::fload_2),
            x if x == Opcode::fload_3 as u8 => Ok(Opcode::fload_3),
            x if x == Opcode::fmul as u8 => Ok(Opcode::fmul),
            x if x == Opcode::fneg as u8 => Ok(Opcode::fneg),
            x if x == Opcode::frem as u8 => Ok(Opcode::frem),
            x if x == Opcode::freturn as u8 => Ok(Opcode::freturn),
            x if x == Opcode::fstore as u8 => Ok(Opcode::fstore),
            x if x == Opcode::fstore_0 as u8 => Ok(Opcode::fstore_0),
            x if x == Opcode::fstore_1 as u8 => Ok(Opcode::fstore_1),
            x if x == Opcode::fstore_2 as u8 => Ok(Opcode::fstore_2),
            x if x == Opcode::fstore_3 as u8 => Ok(Opcode::fstore_3),
            x if x == Opcode::fsub as u8 => Ok(Opcode::fsub),
            x if x == Opcode::getfield as u8 => Ok(Opcode::getfield),
            x if x == Opcode::getstatic as u8 => Ok(Opcode::getstatic),
            x if x == Opcode::goto as u8 => Ok(Opcode::goto),
            x if x == Opcode::goto_w as u8 => Ok(Opcode::goto_w),
            x if x == Opcode::i2b as u8 => Ok(Opcode::i2b),
            x if x == Opcode::i2c as u8 => Ok(Opcode::i2c),
            x if x == Opcode::i2d as u8 => Ok(Opcode::i2d),
            x if x == Opcode::i2f as u8 => Ok(Opcode::i2f),
            x if x == Opcode::i2l as u8 => Ok(Opcode::i2l),
            x if x == Opcode::i2s as u8 => Ok(Opcode::i2s),
            x if x == Opcode::iadd as u8 => Ok(Opcode::iadd),
            x if x == Opcode::iaload as u8 => Ok(Opcode::iaload),
            x if x == Opcode::iand as u8 => Ok(Opcode::iand),
            x if x == Opcode::iastore as u8 => Ok(Opcode::iastore),
            x if x == Opcode::iconst_m1 as u8 => Ok(Opcode::iconst_m1),
            x if x == Opcode::iconst_0 as u8 => Ok(Opcode::iconst_0),
            x if x == Opcode::iconst_1 as u8 => Ok(Opcode::iconst_1),
            x if x == Opcode::iconst_2 as u8 => Ok(Opcode::iconst_2),
            x if x == Opcode::iconst_3 as u8 => Ok(Opcode::iconst_3),
            x if x == Opcode::iconst_4 as u8 => Ok(Opcode::iconst_4),
            x if x == Opcode::iconst_5 as u8 => Ok(Opcode::iconst_5),
            x if x == Opcode::idiv as u8 => Ok(Opcode::idiv),
            x if x == Opcode::if_acmpeq as u8 => Ok(Opcode::if_acmpeq),
            x if x == Opcode::if_acmpne as u8 => Ok(Opcode::if_acmpne),
            x if x == Opcode::if_icmpeq as u8 => Ok(Opcode::if_icmpeq),
            x if x == Opcode::if_icmpne as u8 => Ok(Opcode::if_icmpne),
            x if x == Opcode::if_icmplt as u8 => Ok(Opcode::if_icmplt),
            x if x == Opcode::if_icmpge as u8 => Ok(Opcode::if_icmpge),
            x if x == Opcode::if_icmpgt as u8 => Ok(Opcode::if_icmpgt),
            x if x == Opcode::if_icmple as u8 => Ok(Opcode::if_icmple),
            x if x == Opcode::ifeq as u8 => Ok(Opcode::ifeq),
            x if x == Opcode::ifne as u8 => Ok(Opcode::ifne),
            x if x == Opcode::iflt as u8 => Ok(Opcode::iflt),
            x if x == Opcode::ifge as u8 => Ok(Opcode::ifge),
            x if x == Opcode::ifgt as u8 => Ok(Opcode::ifgt),
            x if x == Opcode::ifle as u8 => Ok(Opcode::ifle),
            x if x == Opcode::ifnonnull as u8 => Ok(Opcode::ifnonnull),
            x if x == Opcode::ifnull as u8 => Ok(Opcode::ifnull),
            x if x == Opcode::iinc as u8 => Ok(Opcode::iinc),
            x if x == Opcode::iload as u8 => Ok(Opcode::iload),
            x if x == Opcode::iload_0 as u8 => Ok(Opcode::iload_0),
            x if x == Opcode::iload_1 as u8 => Ok(Opcode::iload_1),
            x if x == Opcode::iload_2 as u8 => Ok(Opcode::iload_2),
            x if x == Opcode::iload_3 as u8 => Ok(Opcode::iload_3),
            x if x == Opcode::imul as u8 => Ok(Opcode::imul),
            x if x == Opcode::ineg as u8 => Ok(Opcode::ineg),
            x if x == Opcode::instanceof as u8 => Ok(Opcode::instanceof),
            x if x == Opcode::invokedynamic as u8 => Ok(Opcode::invokedynamic),
            x if x == Opcode::invokeinterface as u8 => Ok(Opcode::invokeinterface),
            x if x == Opcode::invokespecial as u8 => Ok(Opcode::invokespecial),
            x if x == Opcode::invokestatic as u8 => Ok(Opcode::invokestatic),
            x if x == Opcode::invokevirtual as u8 => Ok(Opcode::invokevirtual),
            x if x == Opcode::ior as u8 => Ok(Opcode::ior),
            x if x == Opcode::irem as u8 => Ok(Opcode::irem),
            x if x == Opcode::ireturn as u8 => Ok(Opcode::ireturn),
            x if x == Opcode::ishl as u8 => Ok(Opcode::ishl),
            x if x == Opcode::ishr as u8 => Ok(Opcode::ishr),
            x if x == Opcode::istore as u8 => Ok(Opcode::istore),
            x if x == Opcode::istore_0 as u8 => Ok(Opcode::istore_0),
            x if x == Opcode::istore_1 as u8 => Ok(Opcode::istore_1),
            x if x == Opcode::istore_2 as u8 => Ok(Opcode::istore_2),
            x if x == Opcode::istore_3 as u8 => Ok(Opcode::istore_3),
            x if x == Opcode::isub as u8 => Ok(Opcode::isub),
            x if x == Opcode::iushr as u8 => Ok(Opcode::iushr),
            x if x == Opcode::ixor as u8 => Ok(Opcode::ixor),
            x if x == Opcode::jsr as u8 => Ok(Opcode::jsr),
            x if x == Opcode::jsr_w as u8 => Ok(Opcode::jsr_w),
            x if x == Opcode::l2d as u8 => Ok(Opcode::l2d),
            x if x == Opcode::l2f as u8 => Ok(Opcode::l2f),
            x if x == Opcode::l2i as u8 => Ok(Opcode::l2i),
            x if x == Opcode::ladd as u8 => Ok(Opcode::ladd),
            x if x == Opcode::laload as u8 => Ok(Opcode::laload),
            x if x == Opcode::land as u8 => Ok(Opcode::land),
            x if x == Opcode::lastore as u8 => Ok(Opcode::lastore),
            x if x == Opcode::lcmp as u8 => Ok(Opcode::lcmp),
            x if x == Opcode::lconst_0 as u8 => Ok(Opcode::lconst_0),
            x if x == Opcode::lconst_1 as u8 => Ok(Opcode::lconst_1),
            x if x == Opcode::ldc as u8 => Ok(Opcode::ldc),
            x if x == Opcode::ldc_w as u8 => Ok(Opcode::ldc_w),
            x if x == Opcode::ldc2_w as u8 => Ok(Opcode::ldc2_w),
            x if x == Opcode::ldiv as u8 => Ok(Opcode::ldiv),
            x if x == Opcode::lload as u8 => Ok(Opcode::lload),
            x if x == Opcode::lload_0 as u8 => Ok(Opcode::lload_0),
            x if x == Opcode::lload_1 as u8 => Ok(Opcode::lload_1),
            x if x == Opcode::lload_2 as u8 => Ok(Opcode::lload_2),
            x if x == Opcode::lload_3 as u8 => Ok(Opcode::lload_3),
            x if x == Opcode::lmul as u8 => Ok(Opcode::lmul),
            x if x == Opcode::lneg as u8 => Ok(Opcode::lneg),
            x if x == Opcode::lookupswitch as u8 => Ok(Opcode::lookupswitch),
            x if x == Opcode::lor as u8 => Ok(Opcode::lor),
            x if x == Opcode::lrem as u8 => Ok(Opcode::lrem),
            x if x == Opcode::lreturn as u8 => Ok(Opcode::lreturn),
            x if x == Opcode::lshl as u8 => Ok(Opcode::lshl),
            x if x == Opcode::lshr as u8 => Ok(Opcode::lshr),
            x if x == Opcode::lstore as u8 => Ok(Opcode::lstore),
            x if x == Opcode::lstore_0 as u8 => Ok(Opcode::lstore_0),
            x if x == Opcode::lstore_1 as u8 => Ok(Opcode::lstore_1),
            x if x == Opcode::lstore_2 as u8 => Ok(Opcode::lstore_2),
            x if x == Opcode::lstore_3 as u8 => Ok(Opcode::lstore_3),
            x if x == Opcode::lsub as u8 => Ok(Opcode::lsub),
            x if x == Opcode::lushr as u8 => Ok(Opcode::lushr),
            x if x == Opcode::lxor as u8 => Ok(Opcode::lxor),
            x if x == Opcode::monitorenter as u8 => Ok(Opcode::monitorenter),
            x if x == Opcode::monitorexit as u8 => Ok(Opcode::monitorexit),
            x if x == Opcode::multianewarray as u8 => Ok(Opcode::multianewarray),
            x if x == Opcode::new as u8 => Ok(Opcode::new),
            x if x == Opcode::newarray as u8 => Ok(Opcode::newarray),
            x if x == Opcode::nop as u8 => Ok(Opcode::nop),
            x if x == Opcode::pop as u8 => Ok(Opcode::pop),
            x if x == Opcode::pop2 as u8 => Ok(Opcode::pop2),
            x if x == Opcode::putfield as u8 => Ok(Opcode::putfield),
            x if x == Opcode::putstatic as u8 => Ok(Opcode::putstatic),
            x if x == Opcode::ret as u8 => Ok(Opcode::ret),
            x if x == Opcode::eturn as u8 => Ok(Opcode::eturn),
            x if x == Opcode::saload as u8 => Ok(Opcode::saload),
            x if x == Opcode::sastore as u8 => Ok(Opcode::sastore),
            x if x == Opcode::sipush as u8 => Ok(Opcode::sipush),
            x if x == Opcode::swap as u8 => Ok(Opcode::swap),
            x if x == Opcode::tableswitch as u8 => Ok(Opcode::tableswitch),
            x if x == Opcode::wide as u8 => Ok(Opcode::wide),
            x if x == Opcode::r#eturn as u8 => Ok(Opcode::r#eturn),
            _ => Err(ErrorType::IntegerConversion)
        }
    }
}
