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
    fn fill_from(&mut self, other: &Self);

    fn insert_interface() -> impl FnOnce(&dyn Database, Self) -> Result<Self>
    where
        Self: Sized;

    fn update_interface() -> impl FnOnce(&dyn Database, &str, Self) -> Result<Self>
    where
        Self: Sized;

    fn delete_interface() -> impl FnOnce(&dyn Database, &str) -> Result<()>
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

    fn insert_model(database: &dyn Database, body: Option<serde_json::Value>) -> HttpResponse
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

    fn update_model(
        database: &dyn Database,
        subpath: &str,
        body: Option<serde_json::Value>,
    ) -> HttpResponse
    where
        Self: DeserializeOwned,
    {
        match body {
            Some(json) => match Self::from_json(json) {
                Ok(update_model) => {
                    match (Self::update_interface())(database, subpath, update_model) {
                        Ok(updated_model) => updated_model.to_ok_response(),
                        Err(_) => HttpResponse::json_404(&Self::TYPE_NAME),
                    }
                }
                Err(_) => HttpResponse::bad_request(&Self::create_error_msg()),
            },
            None => HttpResponse::missing_body(Some(&Self::REQUIRED_VALUES)),
        }
    }

    fn delete_model(database: &dyn Database, subpath: &str) -> HttpResponse
    where
        Self: DeserializeOwned,
    {
        match (Self::delete_interface())(database, subpath) {
            Ok(_) => HttpResponse::no_content(),
            Err(_) => HttpResponse::json_404(&Self::TYPE_NAME),
        }
    }

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
