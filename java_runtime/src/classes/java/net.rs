mod exceptions;
mod http_url_connection;
mod jar_url_connection;
mod url;
mod url_class_loader;
mod url_connection;
mod url_encoder;
mod url_stream_handler;

pub use self::{
    exceptions::{MalformedURLException, UnknownServiceException},
    http_url_connection::HttpURLConnection,
    jar_url_connection::JarURLConnection,
    url::URL,
    url_class_loader::URLClassLoader,
    url_connection::URLConnection,
    url_encoder::URLEncoder,
    url_stream_handler::URLStreamHandler,
};

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        HttpURLConnection,
        JarURLConnection,
        MalformedURLException,
        UnknownServiceException,
        URL,
        URLClassLoader,
        URLConnection,
        URLEncoder,
        URLStreamHandler,
    ]
}
