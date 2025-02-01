//methods the server allows along with a catchall Error
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Error,
}

impl HttpMethod {
    //const of all possible types to all iteration over all possible values
    pub const ALL_TYPES: [HttpMethod; 5] = [
        Self::Get,
        Self::Post,
        Self::Patch,
        Self::Delete,
        Self::Error,
    ];

    //Returns the String representitive
    pub fn to_string(&self) -> String {
        match self {
            Self::Get => String::from("GET"),
            Self::Post => String::from("POST"),
            Self::Patch => String::from("PATCH"),
            Self::Delete => String::from("DELETE"),
            Self::Error => String::from("ERROR"),
        }
    }

    pub fn from_string(string: String) -> Self {
        for method in Self::ALL_TYPES {
            if string.starts_with(&method.to_string()) {
                return method;
            }
        }
        HttpMethod::Error
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.to_string().into_bytes()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        for method in Self::ALL_TYPES {
            if bytes.starts_with(&method.to_bytes()) {
                return method;
            }
        }
        HttpMethod::Error
    }
}
