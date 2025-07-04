use std::{
    io::{Read, Write},
    os::unix::net::UnixStream,
};

pub enum DockerApi {
    Version,
    Info,
    Containers,
    ContainerDetails(String),
    Volumes,
    Images,
    Networks,
    Ping,
    VolumeDetails(String),
}

impl DockerApi {
    pub fn endpoint(&self) -> String {
        match self {
            DockerApi::Version => "version".to_string(),
            DockerApi::Info => "info".to_string(),
            DockerApi::Containers => "containers/json".to_string(),
            DockerApi::ContainerDetails(container_id) => {
                format!("containers/{}/json", container_id)
            }
            DockerApi::Volumes => "volumes".to_string(),
            DockerApi::Images => "images".to_string(),
            DockerApi::Networks => "networks".to_string(),
            DockerApi::Ping => "_ping".to_string(),
            DockerApi::VolumeDetails(volume_name) => format!("volumes/{}", volume_name),
        }
    }
}

pub enum DockerRequestMethod {
    Get,
    Post,
    Put,
    Delete,
}

impl DockerRequestMethod {
    pub fn method(&self) -> &str {
        match self {
            DockerRequestMethod::Get => "GET",
            DockerRequestMethod::Post => "POST",
            DockerRequestMethod::Put => "PUT",
            DockerRequestMethod::Delete => "DELETE",
        }
    }
}

// TODO: Create a HashMap with DockerApi and DockerRequestMethod
// DockerApiConfig<DockerApi, DockerRequestMethod>
// Maybe in the future, use a Complex configuration struct if needed.

#[derive(Debug)]
pub struct TeusRequestBuilder {
    pub socket: String,
    pub host: String,
    socket_stream: UnixStream,
}

impl TeusRequestBuilder {
    // "GET /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    pub fn new(socket: String, host: String) -> Result<Self, Box<dyn std::error::Error>> {
        let stream = match UnixStream::connect(&socket) {
            Ok(sock) => sock,
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
                return Err(Box::new(e));
            }
        };

        Ok(TeusRequestBuilder {
            socket,
            host,
            socket_stream: stream,
        })
    }

    #[inline]
    fn format_url_request(
        &self,
        method: DockerRequestMethod,
        api: DockerApi,
        query: Option<String>,
    ) -> String {
        let query_str = query.map(|q| format!("?{}", q)).unwrap_or_default();
        println!("QUERY STR: {}", query_str);
        format!(
            "{} /{}{} HTTP/1.1\r\nHost: {}\r\nConnection: close\r\n\r\n",
            method.method(),
            api.endpoint(),
            query_str,
            self.host,
        )
    }

    /// Helper method to parse the response buffer into a string.
    fn parse_buffer_to_string(&self, response_buffer: String) -> String {
        println!("RESPONSE BUFFER:\n{}", response_buffer);

        // Keep the headers and body parts separately.
        let (headers_str, body_part) = match response_buffer.split_once("\r\n\r\n") {
            Some((headers, body)) => (headers, body),
            None => {
                eprintln!("Could not parse HTTP response: No header/body separator found.");
                return "Error: Invalid HTTP response".to_string();
            }
        };

        // Check the headers to see if the response is chunked.
        let is_chunked = headers_str
            .lines()
            .any(|line| line.to_lowercase().contains("transfer-encoding: chunked"));

        if is_chunked {
            println!("--> Detected chunked response. Parsing chunks...");
            let mut lines = body_part.trim().lines();
            lines.next(); // Skip the hex size
            lines.next_back(); // Skip the trailing "0"
            lines.collect::<String>()
        } else {
            println!("--> Detected Content-Length response. Body is raw JSON.");
            // If not chunked, the body is already the complete JSON. No processing needed.
            body_part.to_string()
        }
    }

    // TODO: Fill the method by matching the Docker API
    // Ex: Version => GET
    // StartService => POST
    // DeleteContainer => DELETE
    // etc..
    // TODO: Implement query params in the request builder
    pub fn make_request(
        &mut self,
        method: DockerRequestMethod,
        api: DockerApi,
        query: Option<String>,
    ) -> String {
        let request = self.format_url_request(method, api, query);
        println!("REQUEST:\n{}", request);
        if let Err(e) = self.socket_stream.write_all(request.as_bytes()) {
            eprintln!("Failed to write to socket: {}", e);
            return String::new(); // Return early on error
        }

        // Flush the write buffer to ensure the request is sent
        if let Err(e) = self.socket_stream.flush() {
            eprintln!("Failed to flush socket: {}", e);
            return String::new();
        }

        let mut response_buffer = String::new();
        if let Err(e) = self.socket_stream.read_to_string(&mut response_buffer) {
            eprintln!("Failed to read from socket: {}", e);
            return "Error: Failed to read response".to_string();
        }

        let response = self.parse_buffer_to_string(response_buffer);
        response
    }
}

mod tests {
    use super::*;

    // Helper function to avoid repeating setup code
    fn _setup_builder() -> TeusRequestBuilder {
        let socket = "/Users/homeerr/.colima/default/docker.sock".to_string();
        let host = "localhost".to_string();
        TeusRequestBuilder::new(socket, host).unwrap()
    }

    #[test]
    fn builds_get_request_correctly() {
        let builder = _setup_builder();
        let get = builder.format_url_request(DockerRequestMethod::Get, DockerApi::Version, None);
        assert_eq!(
            get,
            "GET /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        );
    }

    #[test]
    fn builds_put_request_correctly() {
        let builder = _setup_builder();
        let put = builder.format_url_request(DockerRequestMethod::Put, DockerApi::Version, None);
        assert_eq!(
            put,
            "PUT /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        );
    }

    #[test]
    fn builds_post_request_correctly() {
        let builder = _setup_builder();
        let post = builder.format_url_request(DockerRequestMethod::Post, DockerApi::Version, None);
        assert_eq!(
            post,
            "POST /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        );
    }

    #[test]
    fn builds_delete_request_correctly() {
        let builder = _setup_builder();
        let delete =
            builder.format_url_request(DockerRequestMethod::Delete, DockerApi::Version, None);
        assert_eq!(
            delete,
            "DELETE /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n"
        );
    }
}
