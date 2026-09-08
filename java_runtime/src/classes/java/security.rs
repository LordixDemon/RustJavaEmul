use alloc::{format, vec, vec::Vec};

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use java_constants::MethodAccessFlags;
use jvm::{Array, ClassInstanceRef, Jvm, Result, runtime::JavaLangString};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::lang::String};

simple_exception!(
    NoSuchAlgorithmException,
    "java/security/NoSuchAlgorithmException",
    "java/lang/Exception",
    "java.security.NoSuchAlgorithmException"
);

pub struct MessageDigest;

impl MessageDigest {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/security/MessageDigest",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "(Ljava/lang/String;)V", Self::init, Default::default()),
                JavaMethodProto::new(
                    "getInstance",
                    "(Ljava/lang/String;)Ljava/security/MessageDigest;",
                    Self::get_instance,
                    MethodAccessFlags::STATIC,
                ),
                JavaMethodProto::new("update", "([BII)V", Self::update, Default::default()),
                JavaMethodProto::new("digest", "([BII)I", Self::digest_into, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("algorithm", "Ljava/lang/String;", Default::default()),
                JavaFieldProto::new("buffer", "[B", Default::default()),
                JavaFieldProto::new("count", "I", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, algorithm: ClassInstanceRef<String>) -> Result<()> {
        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;
        let buffer = jvm.instantiate_array("B", 64).await?;
        jvm.put_field(&mut this, "algorithm", "Ljava/lang/String;", algorithm).await?;
        jvm.put_field(&mut this, "buffer", "[B", buffer).await?;
        jvm.put_field(&mut this, "count", "I", 0).await
    }

    async fn get_instance(jvm: &Jvm, _: &mut RuntimeContext, algorithm: ClassInstanceRef<String>) -> Result<ClassInstanceRef<Self>> {
        if algorithm.is_null() {
            return Err(jvm.exception("java/security/NoSuchAlgorithmException", "null").await);
        }
        let name = JavaLangString::to_rust_string(jvm, &algorithm).await?;
        let normalized = name.replace('-', "").to_ascii_uppercase();
        if normalized != "MD5" && normalized != "SHA" && normalized != "SHA1" {
            return Err(jvm
                .exception("java/security/NoSuchAlgorithmException", &format!("{name} MessageDigest not available"))
                .await);
        }
        Ok(jvm
            .new_class("java/security/MessageDigest", "(Ljava/lang/String;)V", (algorithm,))
            .await?
            .into())
    }

    async fn update(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        input: ClassInstanceRef<Array<i8>>,
        offset: i32,
        len: i32,
    ) -> Result<()> {
        if input.is_null() || len <= 0 {
            return Ok(());
        }
        let incoming: Vec<i8> = jvm.load_array(&input, offset as usize, len as usize).await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let needed = (count as usize) + incoming.len();
        let mut buffer = jvm.get_field(&this, "buffer", "[B").await?;
        let capacity = jvm.array_length(&buffer).await?;
        if capacity < needed {
            let old: Vec<i8> = jvm.load_array(&buffer, 0, count as usize).await?;
            buffer = jvm.instantiate_array("B", needed.max(capacity * 2)).await?;
            jvm.put_field(&mut this, "buffer", "[B", buffer.clone()).await?;
            jvm.store_array(&mut buffer, 0, old).await?;
        }
        jvm.store_array(&mut buffer, count as usize, incoming).await?;
        jvm.put_field(&mut this, "count", "I", needed as i32).await
    }

    async fn digest_into(
        jvm: &Jvm,
        _: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        mut out: ClassInstanceRef<Array<i8>>,
        offset: i32,
        len: i32,
    ) -> Result<i32> {
        let algorithm: ClassInstanceRef<String> = jvm.get_field(&this, "algorithm", "Ljava/lang/String;").await?;
        let name = JavaLangString::to_rust_string(jvm, &algorithm).await?;
        let count: i32 = jvm.get_field(&this, "count", "I").await?;
        let buffer = jvm.get_field(&this, "buffer", "[B").await?;
        let data: Vec<i8> = if count > 0 {
            jvm.load_array(&buffer, 0, count as usize).await?
        } else {
            Vec::new()
        };
        let bytes: Vec<u8> = data.into_iter().map(|b| b as u8).collect();
        let digest = match name.replace('-', "").to_ascii_uppercase().as_str() {
            "MD5" => md5_digest(&bytes).to_vec(),
            _ => sha1_digest(&bytes).to_vec(),
        };
        let written = digest.len().min(len.max(0) as usize);
        let signed: Vec<i8> = digest.iter().take(written).map(|b| *b as i8).collect();
        if !out.is_null() && written > 0 {
            jvm.store_array(&mut out, offset as usize, signed).await?;
        }
        jvm.put_field(&mut this, "count", "I", 0).await?;
        Ok(written as i32)
    }
}

fn md5_digest(input: &[u8]) -> [u8; 16] {
    let mut s = [0x67452301u32, 0xefcdab89, 0x98badcfe, 0x10325476];
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_le_bytes());
    const K: [u32; 64] = [
        0xd76aa478, 0xe8c7b756, 0x242070db, 0xc1bdceee, 0xf57c0faf, 0x4787c62a, 0xa8304613, 0xfd469501, 0x698098d8, 0x8b44f7af, 0xffff5bb1,
        0x895cd7be, 0x6b901122, 0xfd987193, 0xa679438e, 0x49b40821, 0xf61e2562, 0xc040b340, 0x265e5a51, 0xe9b6c7aa, 0xd62f105d, 0x02441453,
        0xd8a1e681, 0xe7d3fbc8, 0x21e1cde6, 0xc33707d6, 0xf4d50d87, 0x455a14ed, 0xa9e3e905, 0xfcefa3f8, 0x676f02d9, 0x8d2a4c8a, 0xfffa3942,
        0x8771f681, 0x6d9d6122, 0xfde5380c, 0xa4beea44, 0x4bdecfa9, 0xf6bb4b60, 0xbebfbc70, 0x289b7ec6, 0xeaa127fa, 0xd4ef3085, 0x04881d05,
        0xd9d4d039, 0xe6db99e5, 0x1fa27cf8, 0xc4ac5665, 0xf4292244, 0x432aff97, 0xab9423a7, 0xfc93a039, 0x655b59c3, 0x8f0ccc92, 0xffeff47d,
        0x85845dd1, 0x6fa87e4f, 0xfe2ce6e0, 0xa3014314, 0x4e0811a1, 0xf7537e82, 0xbd3af235, 0x2ad7d2bb, 0xeb86d391,
    ];
    const S: [u32; 64] = [
        7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 7, 12, 17, 22, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 5, 9, 14, 20, 4, 11, 16, 23, 4, 11, 16,
        23, 4, 11, 16, 23, 4, 11, 16, 23, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21, 6, 10, 15, 21,
    ];
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 16];
        for (i, word) in w.iter_mut().enumerate() {
            let o = i * 4;
            *word = u32::from_le_bytes([chunk[o], chunk[o + 1], chunk[o + 2], chunk[o + 3]]);
        }
        let mut a = s[0];
        let mut b = s[1];
        let mut c = s[2];
        let mut d = s[3];
        for i in 0..64 {
            let (f, g) = if i < 16 {
                ((b & c) | ((!b) & d), i)
            } else if i < 32 {
                ((d & b) | ((!d) & c), (5 * i + 1) % 16)
            } else if i < 48 {
                (b ^ c ^ d, (3 * i + 5) % 16)
            } else {
                (c ^ (b | (!d)), (7 * i) % 16)
            };
            let f = f.wrapping_add(a).wrapping_add(K[i]).wrapping_add(w[g]);
            a = d;
            d = c;
            c = b;
            b = b.wrapping_add(f.rotate_left(S[i]));
        }
        s[0] = s[0].wrapping_add(a);
        s[1] = s[1].wrapping_add(b);
        s[2] = s[2].wrapping_add(c);
        s[3] = s[3].wrapping_add(d);
    }
    let mut out = [0u8; 16];
    for (i, word) in s.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_le_bytes());
    }
    out
}

fn sha1_digest(input: &[u8]) -> [u8; 20] {
    let mut h = [0x67452301u32, 0xEFCDAB89, 0x98BADCFE, 0x10325476, 0xC3D2E1F0];
    let mut msg = input.to_vec();
    let bit_len = (input.len() as u64) * 8;
    msg.push(0x80);
    while (msg.len() % 64) != 56 {
        msg.push(0);
    }
    msg.extend_from_slice(&bit_len.to_be_bytes());
    for chunk in msg.chunks_exact(64) {
        let mut w = [0u32; 80];
        for i in 0..16 {
            let o = i * 4;
            w[i] = u32::from_be_bytes([chunk[o], chunk[o + 1], chunk[o + 2], chunk[o + 3]]);
        }
        for i in 16..80 {
            w[i] = (w[i - 3] ^ w[i - 8] ^ w[i - 14] ^ w[i - 16]).rotate_left(1);
        }
        let mut a = h[0];
        let mut b = h[1];
        let mut c = h[2];
        let mut d = h[3];
        let mut e = h[4];
        for (i, &word) in w.iter().enumerate() {
            let (f, k) = if i < 20 {
                ((b & c) | ((!b) & d), 0x5A827999)
            } else if i < 40 {
                (b ^ c ^ d, 0x6ED9EBA1)
            } else if i < 60 {
                ((b & c) | (b & d) | (c & d), 0x8F1BBCDC)
            } else {
                (b ^ c ^ d, 0xCA62C1D6)
            };
            let temp = a.rotate_left(5).wrapping_add(f).wrapping_add(e).wrapping_add(k).wrapping_add(word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        h[0] = h[0].wrapping_add(a);
        h[1] = h[1].wrapping_add(b);
        h[2] = h[2].wrapping_add(c);
        h[3] = h[3].wrapping_add(d);
        h[4] = h[4].wrapping_add(e);
    }
    let mut out = [0u8; 20];
    for (i, word) in h.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![MessageDigest, NoSuchAlgorithmException]
}
