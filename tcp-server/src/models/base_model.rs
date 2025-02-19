use serde::de::DeserializeOwned;

use crate::{
    data::Database,
    http::{HttpHeader, HttpResponse, HttpStatus},
};

type Result<T> = crate::Result<T>;

pub trait BaseModel {
    const TYPE_NAME: &'static str;
    const REQUIRED_VALUES: &'static str;
    #[allow(unused)]
    fn is_valid(&self) -> bool;
    fn public_json(&self) -> String;

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized;

    fn create_error_msg() -> String {
        format!("Error creating {}.", Self::TYPE_NAME)
    }

    fn to_ok_response(&self) -> HttpResponse {
        HttpResponse::new(
            HttpStatus::OK,
            HttpHeader::default_json(),
            self.public_json(),
        )
    }

    fn to_created_response(&self) -> HttpResponse {
        HttpResponse::new(
            HttpStatus::Created,
            HttpHeader::default_json(),
            self.public_json(),
        )
    }

    fn try_insert_model(database: &dyn Database, body: Option<serde_json::Value>) -> HttpResponse
    where
        Self: DeserializeOwned,
    {
        match body {
            Some(json) => match Self::from_json(json) {
                Ok(model) => match (Self::insert_interface())(database, model) {
                    Ok(new_model) => new_model.to_created_response(),
                    Err(_) => HttpResponse::bad_request(&Self::create_error_msg()),
                },
                Err(msg) => HttpResponse::invalid_body(Some(&msg)),
            },
            None => HttpResponse::missing_body(Some(&Self::REQUIRED_VALUES)),
        }
    }

    //fn try_update_model(database: &dyn Database, body: Option<serde_json::Value>) -> HttpResponse;

    //fn try_delete_model(database: &dyn Database, body: Option<serde_json::Value>) -> HttpResponse;

    fn from_json(json: serde_json::Value) -> Result<Self>
    where
        Self: DeserializeOwned + Sized,
    {
        match serde_json::from_value::<Self>(json) {
            Ok(model) => Ok(model),
            Err(_) => Err(format!(
                "failed to parse to {}.{}",
                Self::TYPE_NAME,
                Self::create_error_msg()
            )),
        }
    }
}
