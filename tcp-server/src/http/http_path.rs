/*
Copyright 2025 CS 462 Personal Data Acquisition Prototype Group
    
Licensed under the Apache License, Version 2.0 (the "License"); you may not use this file except in compliance with the License.
You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
Unless required by applicable law or agreed to in writing, software distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied. See the License for the specific language governing permissions and limitations under the License.
*/
macro_rules! impl_http_path {
    ($enum_name:ident { $(($variant:ident, $const_name:expr)),* }) => {
        impl $enum_name {
            fn with_subpath(&self, subpath: String) -> Self {
                match self {
                    $( $enum_name::$variant(_) => $enum_name::$variant(subpath), )*
                }
            }

            pub fn variants() -> Vec<Self> {
                vec![
                    $( $enum_name::$variant(String::new()), )*
                ]
            }

            pub fn base_path(&self) -> String {
                match self {
                    $( $enum_name::$variant(_) => $const_name.to_string(), )*
                }
            }

            pub fn to_string(&self) -> String {
                match self {
                    $( $enum_name::$variant(subpath) => format!("{}{}", $const_name, subpath), )*
                }
            }
        }
    };
}

impl_http_path!(HttpPath {
    (Index, HttpPath::INDEX_ENDPOINT),
    (NotFound, HttpPath::NOT_FOUND_ENDPOINT),
    (Authentication, HttpPath::AUTHENTICATION_ENDPOINT),
    (User, HttpPath::USER_ENDPOINT),
    (Sensor, HttpPath::SENSOR_ENDPOINT),
    (Session, HttpPath::SESSION_ENDPOINT),
    (SessionSensor, HttpPath::SESSION_SENSOR_ENDPOINT),
    (SessionSensorData, HttpPath::SESSION_SENSOR_DATA_ENDPOINT)
});

#[derive(Debug, Clone)]
pub enum HttpPath {
    Index(String),
    NotFound(String),
    Authentication(String),
    User(String),
    Sensor(String),
    Session(String),
    SessionSensor(String),
    SessionSensorData(String),
}

impl HttpPath {
    const INDEX_ENDPOINT: &str = "/";
    const NOT_FOUND_ENDPOINT: &str = "";
    const AUTHENTICATION_ENDPOINT: &str = "/authentication";
    const USER_ENDPOINT: &str = "/users";
    const SENSOR_ENDPOINT: &str = "/sensors";
    const SESSION_ENDPOINT: &str = "/sessions";
    const SESSION_SENSOR_ENDPOINT: &str = "/sessions-sensors";
    const SESSION_SENSOR_DATA_ENDPOINT: &str = "/sessions-sensors-data";

    pub fn from_string(path: String) -> HttpPath {
        let (base, subpath) = path[1..]
            .split_once('/')
            .map(|(base_part, subpath_part)| (format!("/{base_part}"), format!("/{subpath_part}")))
            .unwrap_or((path.clone(), String::new()));
        match HttpPath::variants()
            .iter()
            .find(|variant| variant.base_path() == base)
            .cloned()
        {
            Some(variant) => variant.with_subpath(subpath),
            None => HttpPath::NotFound(path),
        }
    }

    pub fn subsection(subpath: &str, index: usize) -> Option<&str> {
        if subpath.is_empty() {
            return None;
        }
        subpath.split('/').nth(index + 1)
    }
}
