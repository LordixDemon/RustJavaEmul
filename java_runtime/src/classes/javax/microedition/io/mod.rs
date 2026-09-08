pub struct Connector;
pub struct Connection;
pub struct InputConnection;
pub struct OutputConnection;
pub struct StreamConnection;
pub struct ContentConnection;
pub struct StreamConnectionNotifier;
pub struct HttpConnection;
pub struct HttpsConnection;
pub struct SocketConnection;
pub struct ServerSocketConnection;
pub struct UDPDatagramConnection;
pub struct CommConnection;
pub struct SecureConnection;
pub struct DatagramConnection;
pub struct Datagram;
pub struct FileConnection;
pub struct FileSystemRegistry;
pub struct FileSystemListener;
pub struct PushRegistry;
pub struct SecurityInfo;

mod connector;
mod datagram;
mod file;
mod http;
mod interfaces;
mod push;
mod sockets;

pub use self::connector::ConnectionNotFoundException;

pub fn class_protos() -> impl IntoIterator<Item = crate::RuntimeClassProtoFactory> {
    proto_factories![
        CommConnection,
        Connection,
        ConnectionNotFoundException,
        Connector,
        ContentConnection,
        Datagram,
        DatagramConnection,
        FileConnection,
        FileSystemListener,
        FileSystemRegistry,
        HttpConnection,
        HttpsConnection,
        InputConnection,
        OutputConnection,
        PushRegistry,
        SecureConnection,
        SecurityInfo,
        ServerSocketConnection,
        SocketConnection,
        StreamConnection,
        StreamConnectionNotifier,
        UDPDatagramConnection,
    ]
}
