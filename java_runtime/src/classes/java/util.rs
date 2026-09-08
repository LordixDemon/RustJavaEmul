pub mod jar;
pub mod zip;

mod abstract_collection;
mod abstract_list;
mod calendar;
mod date;
mod dictionary;
mod enumeration;
mod enumerators;
mod exceptions;
mod gregorian_calendar;
mod hashtable;
mod hashtable_entry;
mod properties;
mod random;
mod simple_time_zone;
mod stack;
mod time_zone;
mod timer;
mod timer_task;
mod timer_thread;
mod vector;

pub use self::{
    abstract_collection::AbstractCollection,
    abstract_list::AbstractList,
    calendar::Calendar,
    date::Date,
    dictionary::Dictionary,
    enumeration::Enumeration,
    enumerators::{HashtableEnumerator, VectorEnumerator},
    exceptions::{EmptyStackException, NoSuchElementException},
    gregorian_calendar::GregorianCalendar,
    hashtable::Hashtable,
    hashtable_entry::HashtableEntry,
    properties::Properties,
    random::Random,
    simple_time_zone::SimpleTimeZone,
    stack::Stack,
    time_zone::TimeZone,
    timer::Timer,
    timer_task::TimerTask,
    timer_thread::TimerThread,
    vector::Vector,
};

pub fn class_protos() -> alloc::vec::Vec<crate::RuntimeClassProtoFactory> {
    let mut factories = alloc::vec::Vec::new();
    factories.extend(proto_factories![
        AbstractCollection,
        AbstractList,
        Calendar,
        Date,
        Dictionary,
        EmptyStackException,
        Enumeration,
        GregorianCalendar,
        Hashtable,
        HashtableEntry,
        HashtableEnumerator,
        NoSuchElementException,
        Properties,
        Random,
        SimpleTimeZone,
        Stack,
        Timer,
        TimerTask,
        TimerThread,
        TimeZone,
        Vector,
        VectorEnumerator,
    ]);
    factories.extend(jar::class_protos());
    factories.extend(zip::class_protos());
    factories
}
