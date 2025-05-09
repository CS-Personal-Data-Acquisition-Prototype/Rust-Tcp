//methods the server allows along with a catchall Error
pub enum HttpMethod {
    Get,
    Post,
    Patch,
    Delete,
    Options,
    Error,
}

impl HttpMethod {
    //const of all possible types to all iteration over all possible values
    pub const ALL_TYPES: [HttpMethod; 6] = [
        Self::Get,
        Self::Post,
        Self::Patch,
        Self::Delete,
        Self::Options,
        Self::Error,
    ];

    //Returns the String representitive
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Get => "GET",
            Self::Post => "POST",
            Self::Patch => "PATCH",
            Self::Delete => "DELETE",
            Self::Options => "OPTIONS",
            Self::Error => "ERROR",
        }
    }

    #[allow(unused)]
    pub fn from_string(string: String) -> Self {
        for method in Self::ALL_TYPES {
            if string.starts_with(method.as_str()) {
                return method;
            }
        }
        HttpMethod::Error
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        self.as_str().to_string().into_bytes()
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
