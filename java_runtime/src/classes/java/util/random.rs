use alloc::vec;

use java_class_proto::{JavaFieldProto, JavaMethodProto};
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext};

// class java.util.Random
pub struct Random;

impl Random {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/Random",
            parent_class: Some("java/lang/Object"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(J)V", Self::init_with_seed, Default::default()),
                JavaMethodProto::new("nextInt", "()I", Self::next_int, Default::default()),
                JavaMethodProto::new("nextInt", "(I)I", Self::next_int_bound, Default::default()),
                JavaMethodProto::new("nextLong", "()J", Self::next_long, Default::default()),
                JavaMethodProto::new("nextBoolean", "()Z", Self::next_boolean, Default::default()),
                JavaMethodProto::new("nextFloat", "()F", Self::next_float, Default::default()),
                JavaMethodProto::new("nextDouble", "()D", Self::next_double, Default::default()),
                JavaMethodProto::new("nextGaussian", "()D", Self::next_gaussian, Default::default()),
                JavaMethodProto::new("next", "(I)I", Self::next, Default::default()),
                JavaMethodProto::new("setSeed", "(J)V", Self::set_seed, Default::default()),
            ],
            fields: vec![
                JavaFieldProto::new("seed", "J", Default::default()),
                JavaFieldProto::new("haveNextNextGaussian", "Z", Default::default()),
                JavaFieldProto::new("nextNextGaussian", "D", Default::default()),
            ],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({:?})", &this);

        let default_seed = 0i64; // TODO
        let _: () = jvm.invoke_special(&this, "java/util/Random", "<init>", "(J)V", (default_seed,)).await?;

        Ok(())
    }

    async fn init_with_seed(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::<init>({:?}, {:?})", &this, seed);

        let _: () = jvm.invoke_special(&this, "java/lang/Object", "<init>", "()V", ()).await?;

        let _: () = jvm.invoke_virtual(&this, "setSeed", "(J)V", (seed,)).await?;

        Ok(())
    }

    async fn next_int(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i32> {
        tracing::debug!("java.util.Random::nextInt({:?})", &this);

        Self::next_bits(jvm, &mut this, 32).await
    }

    async fn next_int_bound(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, bound: i32) -> Result<i32> {
        tracing::debug!("java.util.Random::nextInt({:?}, {:?})", &this, bound);

        if bound <= 0 {
            return Err(jvm.exception("java/lang/IllegalArgumentException", "bound must be positive").await);
        }
        if bound & -bound == bound {
            let bits = Self::next_bits(jvm, &mut this, 31).await? as i64;
            return Ok(((bound as i64 * bits) >> 31) as i32);
        }

        loop {
            let bits = Self::next_bits(jvm, &mut this, 31).await?;
            let value = bits % bound;
            if bits.wrapping_sub(value).wrapping_add(bound - 1) >= 0 {
                return Ok(value);
            }
        }
    }

    async fn next_long(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<i64> {
        tracing::debug!("java.util.Random::nextLong({:?})", &this);

        let high = Self::next_bits(jvm, &mut this, 32).await? as i64;
        let low = Self::next_bits(jvm, &mut this, 32).await? as u32 as i64;
        Ok((high << 32).wrapping_add(low))
    }

    async fn next_boolean(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<bool> {
        tracing::debug!("java.util.Random::nextBoolean({:?})", &this);

        Ok(Self::next_bits(jvm, &mut this, 1).await? != 0)
    }

    async fn next_float(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<f32> {
        tracing::debug!("java.util.Random::nextFloat({:?})", &this);

        Ok(Self::next_bits(jvm, &mut this, 24).await? as f32 / (1u32 << 24) as f32)
    }

    async fn next_double(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<f64> {
        tracing::debug!("java.util.Random::nextDouble({:?})", &this);

        let high = Self::next_bits(jvm, &mut this, 26).await? as i64;
        let low = Self::next_bits(jvm, &mut this, 27).await? as i64;
        Ok(((high << 27) + low) as f64 / (1u64 << 53) as f64)
    }

    async fn next(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, bits: i32) -> Result<i32> {
        Self::next_bits(jvm, &mut this, bits.max(1) as u32).await
    }

    async fn next_gaussian(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<f64> {
        let have: bool = jvm.get_field(&this, "haveNextNextGaussian", "Z").await?;
        if have {
            jvm.put_field(&mut this, "haveNextNextGaussian", "Z", false).await?;
            return jvm.get_field(&this, "nextNextGaussian", "D").await;
        }
        loop {
            let v1 = 2.0 * Self::next_double(jvm, context, this.clone()).await? - 1.0;
            let v2 = 2.0 * Self::next_double(jvm, context, this.clone()).await? - 1.0;
            let s = v1 * v1 + v2 * v2;
            if s >= 1.0 || s == 0.0 {
                continue;
            }
            let multiplier = (-2.0 * s.ln() / s).sqrt();
            jvm.put_field(&mut this, "nextNextGaussian", "D", v2 * multiplier).await?;
            jvm.put_field(&mut this, "haveNextNextGaussian", "Z", true).await?;
            return Ok(v1 * multiplier);
        }
    }

    async fn set_seed(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>, seed: i64) -> Result<()> {
        tracing::debug!("java.util.Random::setSeed({:?}, {:?})", &this, seed);

        let seed = (seed ^ 0x5DEECE66D) & Self::MASK;

        jvm.put_field(&mut this, "seed", "J", seed).await?;
        jvm.put_field(&mut this, "haveNextNextGaussian", "Z", false).await?;

        Ok(())
    }

    async fn next_bits(jvm: &Jvm, this: &mut ClassInstanceRef<Self>, bits: u32) -> Result<i32> {
        let seed: i64 = jvm.get_field(this, "seed", "J").await?;
        let next_seed = seed.wrapping_mul(Self::MULTIPLIER).wrapping_add(Self::ADDEND) & Self::MASK;
        jvm.put_field(this, "seed", "J", next_seed).await?;
        Ok(((next_seed as u64) >> (48 - bits)) as i32)
    }

    const MULTIPLIER: i64 = 0x5DEECE66D;
    const ADDEND: i64 = 0xB;
    const MASK: i64 = (1i64 << 48) - 1;
}
