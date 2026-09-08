//! Minimal `EngineBenchHot` classfile with a tight interpreter loop.

pub const CLASS_NAME: &str = "EngineBenchHot";

pub fn hot_class_bytes() -> Vec<u8> {
    let mut out = Vec::with_capacity(256);
    out.extend_from_slice(&0xCAFE_BABEu32.to_be_bytes());
    out.extend_from_slice(&0u16.to_be_bytes()); // minor
    out.extend_from_slice(&49u16.to_be_bytes()); // major Java 5

    // Constant pool count is n+1.
    let utf8s = [
        "EngineBenchHot",   // 1
        "java/lang/Object", // 3
        "<init>",           // 5
        "()V",              // 6
        "loop",             // 9
        "(I)I",             // 10
        "Code",             // 11
        "bits",             // 12
    ];
    // Layout:
    // 1 Utf8 EngineBenchHot
    // 2 Class #1
    // 3 Utf8 java/lang/Object
    // 4 Class #3
    // 5 Utf8 <init>
    // 6 Utf8 ()V
    // 7 NameAndType #5 #6
    // 8 Methodref #4 #7
    // 9 Utf8 loop
    // 10 Utf8 (I)I
    // 11 Utf8 Code
    // 12 Utf8 bits
    out.extend_from_slice(&13u16.to_be_bytes());
    write_utf8(&mut out, utf8s[0]);
    write_class(&mut out, 1);
    write_utf8(&mut out, utf8s[1]);
    write_class(&mut out, 3);
    write_utf8(&mut out, utf8s[2]);
    write_utf8(&mut out, utf8s[3]);
    write_name_and_type(&mut out, 5, 6);
    write_methodref(&mut out, 4, 7);
    write_utf8(&mut out, utf8s[4]);
    write_utf8(&mut out, utf8s[5]);
    write_utf8(&mut out, utf8s[6]);
    write_utf8(&mut out, utf8s[7]);

    out.extend_from_slice(&0x0021u16.to_be_bytes()); // public super
    out.extend_from_slice(&2u16.to_be_bytes()); // this
    out.extend_from_slice(&4u16.to_be_bytes()); // super
    out.extend_from_slice(&0u16.to_be_bytes()); // interfaces
    out.extend_from_slice(&0u16.to_be_bytes()); // fields
    out.extend_from_slice(&3u16.to_be_bytes()); // methods: <init>, loop, bits

    write_code_method(&mut out, 0x0001, 5, 6, 1, 1, &[0x2a, 0xb7, 0x00, 0x08, 0xb1]);
    write_code_method(&mut out, 0x0009, 9, 10, 3, 3, &LOOP_CODE);
    write_code_method(&mut out, 0x0009, 12, 10, 2, 1, &BITS_CODE);

    out.extend_from_slice(&0u16.to_be_bytes()); // class attributes
    out
}

const LOOP_CODE: [u8; 25] = [
    0x03, // iconst_0
    0x3c, // istore_1
    0x03, // iconst_0
    0x3d, // istore_2
    0xa7, 0x00, 0x0e, // goto +14 -> 18
    0x1b, // iload_1
    0x1c, // iload_2
    0x60, // iadd
    0x1c, // iload_2
    0x04, // iconst_1
    0x60, // iadd
    0x68, // imul
    0x3c, // istore_1
    0x84, 0x02, 0x01, // iinc 2, 1
    0x1c, // iload_2
    0x1a, // iload_0
    0xa1, 0xff, 0xf3, // if_icmplt -13 -> 7
    0x1b, // iload_1
    0xac, // ireturn
];

const BITS_CODE: [u8; 11] = [
    0x1a, // iload_0
    0x04, // iconst_1
    0x78, // ishl
    0x1a, // iload_0
    0x82, // ixor
    0x1a, // iload_0
    0x7e, // iand
    0x1a, // iload_0
    0x80, // ior
    0x74, // ineg
    0xac, // ireturn
];

fn write_utf8(out: &mut Vec<u8>, value: &str) {
    out.push(1);
    let bytes = value.as_bytes();
    out.extend_from_slice(&(bytes.len() as u16).to_be_bytes());
    out.extend_from_slice(bytes);
}

fn write_class(out: &mut Vec<u8>, name_index: u16) {
    out.push(7);
    out.extend_from_slice(&name_index.to_be_bytes());
}

fn write_name_and_type(out: &mut Vec<u8>, name: u16, descriptor: u16) {
    out.push(12);
    out.extend_from_slice(&name.to_be_bytes());
    out.extend_from_slice(&descriptor.to_be_bytes());
}

fn write_methodref(out: &mut Vec<u8>, class: u16, name_and_type: u16) {
    out.push(10);
    out.extend_from_slice(&class.to_be_bytes());
    out.extend_from_slice(&name_and_type.to_be_bytes());
}

fn write_code_method(out: &mut Vec<u8>, access: u16, name: u16, descriptor: u16, max_stack: u16, max_locals: u16, code: &[u8]) {
    out.extend_from_slice(&access.to_be_bytes());
    out.extend_from_slice(&name.to_be_bytes());
    out.extend_from_slice(&descriptor.to_be_bytes());
    out.extend_from_slice(&1u16.to_be_bytes()); // one attribute
    out.extend_from_slice(&11u16.to_be_bytes()); // Code
    let attr_len = 12 + code.len() as u32;
    out.extend_from_slice(&attr_len.to_be_bytes());
    out.extend_from_slice(&max_stack.to_be_bytes());
    out.extend_from_slice(&max_locals.to_be_bytes());
    out.extend_from_slice(&(code.len() as u32).to_be_bytes());
    out.extend_from_slice(code);
    out.extend_from_slice(&0u16.to_be_bytes()); // exceptions
    out.extend_from_slice(&0u16.to_be_bytes()); // code attributes
}
