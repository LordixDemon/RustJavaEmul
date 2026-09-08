mod buffered_input_stream;
mod buffered_reader;
mod buffered_writer;
mod byte_array_input_stream;
mod byte_array_output_stream;
mod data_input;
mod data_input_stream;
mod data_output;
mod data_output_stream;
mod exceptions;
mod file;
mod file_descriptor;
mod file_input_stream;
mod file_output_stream;
mod filter_input_stream;
mod filter_output_stream;
mod input_stream;
mod input_stream_reader;
mod output_stream;
mod output_stream_writer;
mod print_stream;
mod print_writer;
mod random_access_file;
mod reader;
mod serializable;
mod string_writer;
mod writer;

pub use self::{
    buffered_input_stream::BufferedInputStream,
    buffered_reader::BufferedReader,
    buffered_writer::BufferedWriter,
    byte_array_input_stream::ByteArrayInputStream,
    byte_array_output_stream::ByteArrayOutputStream,
    data_input::DataInput,
    data_input_stream::DataInputStream,
    data_output::DataOutput,
    data_output_stream::DataOutputStream,
    exceptions::{EOFException, FileNotFoundException, IOException, InterruptedIOException, UTFDataFormatException, UnsupportedEncodingException},
    file::File,
    file_descriptor::FileDescriptor,
    file_input_stream::FileInputStream,
    file_output_stream::FileOutputStream,
    filter_input_stream::FilterInputStream,
    filter_output_stream::FilterOutputStream,
    input_stream::InputStream,
    input_stream_reader::InputStreamReader,
    output_stream::OutputStream,
    output_stream_writer::OutputStreamWriter,
    print_stream::PrintStream,
    print_writer::PrintWriter,
    random_access_file::RandomAccessFile,
    reader::Reader,
    serializable::Serializable,
    string_writer::StringWriter,
    writer::Writer,
};

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        BufferedInputStream,
        BufferedReader,
        BufferedWriter,
        ByteArrayInputStream,
        ByteArrayOutputStream,
        DataInput,
        DataInputStream,
        DataOutput,
        DataOutputStream,
        EOFException,
        File,
        FileDescriptor,
        FileInputStream,
        FileNotFoundException,
        FileOutputStream,
        FilterInputStream,
        FilterOutputStream,
        InputStream,
        InputStreamReader,
        InterruptedIOException,
        IOException,
        UnsupportedEncodingException,
        UTFDataFormatException,
        OutputStream,
        OutputStreamWriter,
        PrintStream,
        PrintWriter,
        RandomAccessFile,
        Reader,
        Serializable,
        StringWriter,
        Writer,
    ]
}
