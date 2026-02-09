use alloc::string::String;
use alloc::vec::Vec;
use alloc::format;
use crate::error::Error;
use crate::alloc::string::ToString;

#[derive(Debug, Clone)]
pub struct HttpResponse {
    version: String,
    status_code: u32,
    reason: String,
    headers: Vec<Header>,
    body: String,
}

#[derive(Debug, Clone)]
pub struct Header {
    name: String,
    value: String,
}

impl Header {
    pub fn new(name: String, value: String) -> Self {
        Self { name, value }
    }
}

impl HttpResponse {
    pub fn version(&self) -> &str {
        &self.version
    }

    pub fn status_code(&self) -> u32 {
        self.status_code
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }

    pub fn headers(&self) -> &Vec<Header> {
        &self.headers
    }

    pub fn body(&self) -> &str {
        &self.body
    }

    pub fn header_value(&self, name: &str) -> Result<String,String> {
        for h in &self.headers {
            if h.name == name {
                return Ok(h.value.clone());
            }
        }
        Err(format!("failed to find {} in headers", name))
    }

    pub fn new(raw_responce: String) -> Result<Self, Error>{
        let preprocessed_responce = raw_responce.trim_start().replace("\r\n", "\n");

        let (status_line, remaining) = match preprocessed_responce.split_once("\n") {
            Some((s, r)) => (s, r),
            None => {
                return Err(Error::Network(format!(
                    "Invalid http response: {}",
                    preprocessed_responce
                )))
            }
        };

        let (headers,body) = match remaining.split_once("\n\n") {
            Some((h,b)) => {
                let mut headers = Vec::new();
                for header in h.split("\n") {
                    let splitted_header: Vec<&str> = header.splitn(2, ":").collect();
                    headers.push(Header::new(
                        String::from(splitted_header[0].trim()),
                        String::from(splitted_header[1].trim()),
                    ));
                }
                (headers,b)
            },
            None => (Vec::new(), remaining),
        };

        let statues: Vec<&str> = status_line.split(' ').collect();

        Ok(Self {
            version: statues[0].to_string(),
            status_code: statues[1].parse().unwrap_or(404),
            reason: statues[2].to_string(),
            headers,
            body: body.to_string(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_status_line_only() {
        let raw = "HTTP/1.1 200 OK\n\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");
    }

    #[test]
    fn test_one_header() {
        let raw = "HTTP/1.1 200 OK\nDate:xx xx xx\n\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");

        assert_eq!(res.header_value("Date").unwrap(), "xx xx xx".to_string());
    }

    #[test]
    fn test_two_headers_with_white_space(){
        let raw = "HTTP/1.1 200 OK\nDate: xx xx xx\nContent-Length: 42\n\n".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK");

        assert_eq!(res.header_value("Date").unwrap(), "xx xx xx".to_string());
        assert_eq!(res.header_value("Content-Length").unwrap(), "42".to_string());
    }

    #[test]
    fn test_body(){
        let raw = "HTTP/1.1 200 OK\nDate: xx xx xx\n\nbody message".to_string();
        let res = HttpResponse::new(raw).expect("Failed to parse http response");
        assert_eq!(res.version(), "HTTP/1.1");
        assert_eq!(res.status_code(), 200);
        assert_eq!(res.reason(), "OK"); 

        assert_eq!(res.header_value("Date").unwrap(), "xx xx xx".to_string());

        assert_eq!(res.body(), "body message".to_string());
    }

    #[test]
    fn test_invalid() {
        let raw = "HTTP/1.1-200-OK".to_string();
        assert!(HttpResponse::new(raw).is_err());
    }
}
