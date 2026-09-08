mod array_index_out_of_bounds_exception;
mod boolean;
mod byte;
mod character;
mod class;
mod class_loader;
mod cloneable;
mod comparable;
mod double;
mod exception;
mod exception_in_initializer_error;
mod exceptions;
mod float;
mod integer;
mod long;
mod math;
mod number;
mod object;
mod runnable;
mod runtime;
mod runtime_exception;
mod short;
mod string;
mod string_buffer;
mod system;
mod thread;
mod throwable;

pub mod reflect;

pub use self::{
    array_index_out_of_bounds_exception::ArrayIndexOutOfBoundsException,
    boolean::Boolean,
    byte::Byte,
    character::Character,
    class::Class,
    class_loader::ClassLoader,
    cloneable::Cloneable,
    comparable::Comparable,
    double::Double,
    exception::Exception,
    exception_in_initializer_error::ExceptionInInitializerError,
    exceptions::{
        AbstractMethodError, ArithmeticException, ArrayStoreException, ClassCastException, ClassFormatError, ClassNotFoundException,
        CloneNotSupportedException, Error, IllegalAccessException, IllegalArgumentException, IllegalMonitorStateException, IllegalStateException,
        IncompatibleClassChangeError, IndexOutOfBoundsException, InstantiationError, InstantiationException, InternalError, InterruptedException,
        LinkageError, NegativeArraySizeException, NoClassDefFoundError, NoSuchFieldError, NoSuchMethodError, NullPointerException,
        NumberFormatException, OutOfMemoryError, SecurityException, StringIndexOutOfBoundsException, UnknownError, UnsatisfiedLinkError,
        UnsupportedOperationException, VerifyError, VirtualMachineError,
    },
    float::Float,
    integer::Integer,
    long::Long,
    math::Math,
    number::Number,
    object::Object,
    runnable::Runnable,
    runtime::Runtime,
    runtime_exception::RuntimeException,
    short::Short,
    string::String,
    string_buffer::StringBuffer,
    system::System,
    thread::Thread,
    throwable::Throwable,
};

pub use self::reflect::Array as ReflectArray;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        AbstractMethodError,
        ArithmeticException,
        ArrayIndexOutOfBoundsException,
        ArrayStoreException,
        Boolean,
        Byte,
        Character,
        Class,
        ClassCastException,
        ClassFormatError,
        ClassLoader,
        ClassNotFoundException,
        Cloneable,
        CloneNotSupportedException,
        Comparable,
        Double,
        Error,
        Exception,
        ExceptionInInitializerError,
        Float,
        IllegalAccessException,
        IllegalArgumentException,
        IllegalMonitorStateException,
        IllegalStateException,
        InstantiationError,
        InstantiationException,
        IncompatibleClassChangeError,
        IndexOutOfBoundsException,
        Integer,
        InternalError,
        InterruptedException,
        LinkageError,
        Long,
        Math,
        NegativeArraySizeException,
        NoClassDefFoundError,
        NoSuchFieldError,
        NoSuchMethodError,
        NullPointerException,
        Number,
        NumberFormatException,
        Object,
        OutOfMemoryError,
        ReflectArray,
        Runnable,
        Runtime,
        RuntimeException,
        SecurityException,
        Short,
        String,
        StringBuffer,
        StringIndexOutOfBoundsException,
        System,
        Thread,
        Throwable,
        UnknownError,
        UnsatisfiedLinkError,
        UnsupportedOperationException,
        VerifyError,
        VirtualMachineError,
    ]
}
