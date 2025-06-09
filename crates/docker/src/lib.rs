pub mod docker;
pub mod requests;

#[cfg(test)]
mod tests {
    use std::io::{Read, Write};
    use std::os::unix::net::UnixStream;

    use reqwest::Client; // This is the reqwest client

    // Replace with your actual Docker socket path
    // const DOCKER_SOCKET_PATH: &str = "/var/run/docker.sock"; // Standard Linux
    #[cfg(target_os = "linux")]
    const DOCKER_SOCKET_PATH: &str = "/var/run/docker.sock"; // Standard Linux
    #[cfg(target_os = "macos")]
    // For testing purposes, do not forget to replace with your actual Colima or docker path
    const DOCKER_SOCKET_PATH: &str = "/Users/homeerr/.colima/default/docker.sock"; // Your Colima path

    #[test]
    fn docker_sock_connection() {
        let docker_sock =
            UnixStream::connect(DOCKER_SOCKET_PATH).unwrap();

        println!("{:?}", docker_sock);
    }

    #[test]
    fn docker_sock_request() {
        let mut stream = match UnixStream::connect(DOCKER_SOCKET_PATH) {
            Ok(sock) => sock,
            Err(e) => {
                eprintln!("Failed to connect: {}", e);
                return;
            }
        };

        // GET /version HTTP/1.1
        // Host: localhost (required for HTTP/1.1)
        // Connection: close (optional, but good for simplicity here)
        // (empty line)
        let request = "GET /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
        let request_containers = "GET /containers/json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";

        match stream.write_all(request.as_bytes()) {
            Ok(_) => println!("Sent request:\n{}", request),
            Err(e) => {
                eprintln!("Failed to write to socket: {}", e);
                return;
            }
        }

        let mut response = String::new();
        match stream.read_to_string(&mut response) {
            Ok(_) => {
                println!("Received response:\n{}", response);

                // --- Basic Parsing ---
                // This just separates headers and body.
                if let Some((headers, body)) = response.split_once("\r\n\r\n") {
                    println!("\n--- Headers ---");
                    // println!("{}", headers);
                    println!("\n--- Body ---");
                    // println!("{}", body);
                } else {
                    println!("\nCould not parse HTTP response.");
                }
            }
            Err(e) => {
                eprintln!("Failed to read from socket: {}", e);
            }
        }
    }

    // #[tokio::test]
    // async fn docker_reqwest_uds_connection() {
    //     // 1. Create a Unix Domain Socket transport
    //     let transport = match Transport::unix(DOCKER_SOCKET_PATH) {
    //         Ok(t) => t,
    //         Err(e) => {
    //             eprintln!("Failed to create Unix transport: {}", e);
    //             // Optionally, check if the socket file exists
    //             if !std::path::Path::new(DOCKER_SOCKET_PATH).exists() {
    //                 eprintln!("Docker socket not found at {}. Please ensure Docker is running and the path is correct.", DOCKER_SOCKET_PATH);
    //             }
    //             panic!("Transport creation failed."); // Fail the test
    //         }
    //     };

    //     // 2. Build a reqwest client with this transport
    //     // Here, we use reqwest::Client::builder() and provide our custom transport.
    //     let client = Client::builder()
    //         .build(transport)
    //         .expect("Failed to build reqwest client with UDS transport");

    //     // 3. Make requests using a dummy HTTP base URL.
    //     // The hostname ("localhost" or "docker" or anything) is ignored by the Unix socket transport;
    //     // it only cares that it's an HTTP request going to the pre-configured socket.
    //     // The path ("/version", "/containers/json", etc.) is what Docker API uses.
    //     let version_url = "http://localhost/version"; // Hostname is a placeholder

    //     println!("Attempting to GET: {}", version_url);

    //     match client.get(version_url).send().await {
    //         Ok(response) => {
    //             println!("Status: {}", response.status());
    //             println!("Headers:\n{:#?}", response.headers());

    //             // Example: Parse the response body as JSON
    //             match response.json::<Value>().await {
    //                 Ok(json_body) => {
    //                     println!("Body (JSON):\n{:#?}", json_body);
    //                     // You can now access parts of the JSON, e.g., json_body["ApiVersion"]
    //                     assert!(
    //                         json_body["ApiVersion"].is_string(),
    //                         "Expected ApiVersion to be a string"
    //                     );
    //                 }
    //                 Err(e) => {
    //                     eprintln!("Failed to parse JSON body: {}", e);
    //                     // Fallback to text if JSON parsing fails
    //                     // match response.text().await {
    //                     //     Ok(text_body) => println!("Body (Text):\n{}", text_body),
    //                     //     Err(e_text) => eprintln!("Failed to read text body: {}", e_text),
    //                     // }
    //                 }
    //             }
    //         }
    //         Err(e) => {
    //             eprintln!("Request to {} failed: {}", version_url, e);
    //             panic!("Request failed."); // Fail the test
    //         }
    //     }

    //     // Example: Listing containers (GET /containers/json)
    //     let containers_url = "http://localhost/containers/json";
    //     println!("\nAttempting to GET: {}", containers_url);
    //     match client.get(containers_url).send().await {
    //         Ok(response) => {
    //             println!("Status for {}: {}", containers_url, response.status());
    //             match response.json::<Vec<Value>>().await {
    //                 // Expecting an array of container objects
    //                 Ok(json_body) => {
    //                     println!("Containers (JSON):\n{:#?}", json_body);
    //                     println!("Found {} containers.", json_body.len());
    //                 }
    //                 Err(e) => eprintln!("Failed to parse containers JSON: {}", e),
    //             }
    //         }
    //         Err(e) => eprintln!("Request to {} failed: {}", containers_url, e),
    //     }
    // }
}
