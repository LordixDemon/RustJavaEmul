use alloc::{vec, vec::Vec};

use chrono::{DateTime, Datelike, FixedOffset, NaiveDate, NaiveTime, TimeZone as ChronoTimeZone, Timelike};

use java_class_proto::JavaMethodProto;
use jvm::{ClassInstanceRef, Jvm, Result};

use crate::{RuntimeClassProto, RuntimeContext, classes::java::util::TimeZone};

// class java.util.GregorianCalendar
pub struct GregorianCalendar;

impl GregorianCalendar {
    pub fn as_proto() -> RuntimeClassProto {
        RuntimeClassProto {
            name: "java/util/GregorianCalendar",
            parent_class: Some("java/util/Calendar"),
            interfaces: vec![],
            methods: vec![
                JavaMethodProto::new("<init>", "()V", Self::init, Default::default()),
                JavaMethodProto::new("<init>", "(Ljava/util/TimeZone;)V", Self::init_with_time_zone, Default::default()),
                JavaMethodProto::new("computeTime", "()V", Self::compute_time, Default::default()),
                JavaMethodProto::new("computeFields", "()V", Self::compute_fields, Default::default()),
            ],
            fields: vec![],
            access_flags: Default::default(),
        }
    }

    async fn init(jvm: &Jvm, context: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::<init>({this:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Calendar", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "time", "J", context.now() as i64).await?;
        jvm.invoke_virtual(&this, "computeFields", "()V", ()).await
    }

    async fn init_with_time_zone(
        jvm: &Jvm,
        context: &mut RuntimeContext,
        mut this: ClassInstanceRef<Self>,
        time_zone: ClassInstanceRef<TimeZone>,
    ) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::<init>({this:?}, {time_zone:?})");

        let _: () = jvm.invoke_special(&this, "java/util/Calendar", "<init>", "()V", ()).await?;
        jvm.put_field(&mut this, "zone", "Ljava/util/TimeZone;", time_zone).await?;
        jvm.put_field(&mut this, "time", "J", context.now() as i64).await?;
        jvm.invoke_virtual(&this, "computeFields", "()V", ()).await
    }

    async fn compute_time(jvm: &Jvm, _: &mut RuntimeContext, mut this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeTime({:?})", &this);

        let fields = jvm.get_field(&this, "fields", "[I").await?;
        let fields: Vec<i32> = jvm.load_array(&fields, 0, 17).await?;
        let time = lenient_fields_to_millis(&fields);
        jvm.put_field(&mut this, "time", "J", time).await
    }

    async fn compute_fields(jvm: &Jvm, _: &mut RuntimeContext, this: ClassInstanceRef<Self>) -> Result<()> {
        tracing::debug!("java.util.GregorianCalendar::computeFields({:?})", &this);

        let time: i64 = jvm.get_field(&this, "time", "J").await?;
        let date_time = DateTime::from_timestamp_millis(time).unwrap_or_else(|| DateTime::from_timestamp(0, 0).unwrap());

        let calculated_fields = vec![
            1, // CE
            date_time.year(),
            date_time.month() as i32 - 1,
            date_time.iso_week().week() as _,
            (date_time.day() / 7) as _,
            date_time.day() as _,
            date_time.ordinal() as _,
            date_time.weekday().number_from_monday() as _,
            (date_time.day() % 7) as _,
            (date_time.hour() / 12) as _,
            (date_time.hour() % 12) as _,
            date_time.hour() as _,
            date_time.minute() as _,
            date_time.second() as _,
            (date_time.nanosecond() / 1_000_000) as _,
            0,
            0,
        ];

        let mut fields = jvm.get_field(&this, "fields", "[I").await?;
        jvm.store_array(&mut fields, 0, calculated_fields).await?;

        Ok(())
    }
}

fn lenient_fields_to_millis(fields: &[i32]) -> i64 {
    let year = fields[1].clamp(1, 9999);
    let month = (fields[2].clamp(0, 11) + 1) as u32;
    let mut day = fields[5].clamp(1, 31) as u32;
    let hour = fields[11].clamp(0, 23) as u32;
    let minute = fields[12].clamp(0, 59) as u32;
    let second = fields[13].clamp(0, 59) as u32;
    let millis = fields[14].clamp(0, 999) as u32;
    let offset_secs = (i64::from(fields[15]) / 1000).clamp(-18 * 3600, 18 * 3600) as i32;
    let tz = FixedOffset::east_opt(offset_secs).unwrap_or_else(|| FixedOffset::east_opt(0).unwrap());

    let naive = loop {
        if let Some(date) = NaiveDate::from_ymd_opt(year, month, day) {
            if let Some(time) = NaiveTime::from_hms_milli_opt(hour, minute, second, millis) {
                break date.and_time(time);
            }
        }
        if day > 1 {
            day -= 1;
            continue;
        }
        break NaiveDate::from_ymd_opt(1970, 1, 1).and_then(|date| date.and_hms_opt(0, 0, 0)).unwrap();
    };

    tz.from_local_datetime(&naive)
        .single()
        .unwrap_or_else(|| tz.from_utc_datetime(&naive))
        .timestamp_millis()
}
