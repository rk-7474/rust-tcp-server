
pub struct Response {
    content: String,
    status: ResponseStatus,
}

pub struct ResponseStatus {
    code: u16,
    message: String,
}

impl Response {
    pub fn to_string(&self) -> String {
        let status_line = format!("HTTP/1.1 {} {}", self.status.code, self.status.message); 
        let length = self.content.len(); 

        format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, length, self.content)
    }

    pub fn new(content: String, code: u16, message: String) -> Response {
        Response {
            content,
            status: ResponseStatus {
                code,
                message
            }
        }
    }
}