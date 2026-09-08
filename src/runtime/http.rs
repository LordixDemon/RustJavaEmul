use std::io::Write;

use java_runtime::{IOError, IOResult, RuntimeNet};

use super::RuntimeImpl;

#[async_trait::async_trait]
impl<T> RuntimeNet for RuntimeImpl<T>
where
    T: Sync + Send + Write + 'static,
{
    async fn http_request(
        &self,
        method: &str,
        url: &str,
        headers: &[(String, String)],
        body: &[u8],
    ) -> IOResult<(i32, Vec<(String, String)>, Vec<u8>)> {
        host_http_request(method, url, headers, body)
    }
}

fn host_http_request(method: &str, url: &str, headers: &[(String, String)], body: &[u8]) -> IOResult<(i32, Vec<(String, String)>, Vec<u8>)> {
    #[cfg(target_arch = "wasm32")]
    {
        let _ = (method, url, headers, body);
        Err(IOError::Unsupported)
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        use core::time::Duration;
        use std::io::{Read, Write as _};
        use std::net::TcpStream;

        let without_scheme = url.strip_prefix("http://").or_else(|| url.strip_prefix("https://")).unwrap_or(url);
        let (host_port, path) = match without_scheme.find('/') {
            Some(index) => (&without_scheme[..index], &without_scheme[index..]),
            None => (without_scheme, "/"),
        };
        let (host, port) = match host_port.rsplit_once(':') {
            Some((host, port)) => (host, port.parse().unwrap_or(80)),
            None => (host_port, 80),
        };
        let mut stream = TcpStream::connect((host, port)).map_err(|_| IOError::NotFound)?;
        stream.set_read_timeout(Some(Duration::from_secs(5))).ok();
        stream.set_write_timeout(Some(Duration::from_secs(5))).ok();
        let mut request = format!(
            "{method} {path} HTTP/1.0\r\nHost: {host}\r\nConnection: close\r\nContent-Length: {}\r\n",
            body.len()
        );
        for (key, value) in headers {
            request.push_str(key);
            request.push_str(": ");
            request.push_str(value);
            request.push_str("\r\n");
        }
        request.push_str("\r\n");
        stream.write_all(request.as_bytes()).map_err(|_| IOError::Unsupported)?;
        if !body.is_empty() {
            stream.write_all(body).map_err(|_| IOError::Unsupported)?;
        }
        let mut response = Vec::new();
        stream.read_to_end(&mut response).map_err(|_| IOError::Unsupported)?;
        let header_end = response.windows(4).position(|w| w == b"\r\n\r\n").unwrap_or(response.len());
        let header_bytes = &response[..header_end];
        let body_bytes = if header_end + 4 <= response.len() {
            response[header_end + 4..].to_vec()
        } else {
            Vec::new()
        };
        let header_text = String::from_utf8_lossy(header_bytes);
        let mut lines = header_text.lines();
        let status = lines
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .and_then(|code| code.parse().ok())
            .unwrap_or(200);
        let mut parsed_headers = Vec::new();
        for line in lines {
            if let Some((key, value)) = line.split_once(':') {
                parsed_headers.push((key.trim().to_string(), value.trim().to_string()));
            }
        }
        Ok((status, parsed_headers, body_bytes))
    }
}
