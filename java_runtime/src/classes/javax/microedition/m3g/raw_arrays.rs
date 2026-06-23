use alloc::{boxed::Box, vec, vec::Vec};

use jvm::{Array, ClassInstance, ClassInstanceRef, Jvm, Result};

pub(super) async fn raw_u8_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i8>>, count: usize) -> Result<Vec<u8>> {
    let count = count.min(jvm.array_length(array).await?);
    let mut bytes = vec![0; count];
    jvm.array_raw_buffer(array).await?.read(0, &mut bytes)?;
    Ok(bytes)
}

pub(super) async fn raw_i16_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i16>>, count: usize) -> Result<Vec<i16>> {
    let count = count.min(jvm.array_length(array).await?);
    let mut values = vec![0; count];
    jvm.array_raw_buffer(array)
        .await?
        .read(0, bytemuck::cast_slice_mut(values.as_mut_slice()))?;
    #[cfg(target_endian = "big")]
    for value in &mut values {
        *value = i16::from_le(*value);
    }
    Ok(values)
}

pub(super) async fn raw_i32_array(jvm: &Jvm, array: &ClassInstanceRef<Array<i32>>, count: usize) -> Result<Vec<i32>> {
    let count = count.min(jvm.array_length(array).await?);
    let mut values = vec![0; count];
    jvm.array_raw_buffer(array)
        .await?
        .read(0, bytemuck::cast_slice_mut(values.as_mut_slice()))?;
    #[cfg(target_endian = "big")]
    for value in &mut values {
        *value = i32::from_le(*value);
    }
    Ok(values)
}

pub(super) async fn raw_f32_array(jvm: &Jvm, array: &ClassInstanceRef<Array<f32>>, count: usize) -> Result<Vec<f32>> {
    let count = count.min(jvm.array_length(array).await?);
    let mut values = vec![0.0; count];
    jvm.array_raw_buffer(array)
        .await?
        .read(0, bytemuck::cast_slice_mut(values.as_mut_slice()))?;
    #[cfg(target_endian = "big")]
    for value in &mut values {
        *value = f32::from_bits(u32::from_le(value.to_bits()));
    }
    Ok(values)
}

pub(super) async fn store_raw_i32_array(jvm: &Jvm, array: &mut Box<dyn ClassInstance>, values: &[i32]) -> Result<()> {
    #[cfg(target_endian = "little")]
    {
        jvm.array_raw_buffer_mut(array).await?.write(0, bytemuck::cast_slice(values))
    }
    #[cfg(target_endian = "big")]
    {
        let values = values.iter().map(|value| value.to_le()).collect::<Vec<_>>();
        jvm.array_raw_buffer_mut(array).await?.write(0, bytemuck::cast_slice(&values))
    }
}

pub(super) async fn store_raw_f32_array(jvm: &Jvm, array: &mut Box<dyn ClassInstance>, values: &[f32]) -> Result<()> {
    #[cfg(target_endian = "little")]
    {
        jvm.array_raw_buffer_mut(array).await?.write(0, bytemuck::cast_slice(values))
    }
    #[cfg(target_endian = "big")]
    {
        let values = values.iter().map(|value| value.to_bits().to_le()).collect::<Vec<_>>();
        jvm.array_raw_buffer_mut(array).await?.write(0, bytemuck::cast_slice(&values))
    }
}
