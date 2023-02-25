use actix_web::{

 get,
 post,
 put,
 error:ResponseError,
 web::Path,
 web::Json,
 web::Data,
 http::{header::ContentType, StatusCode}
};

use serde::{Serialize, Deserialize};
use derive_more::{Display};

#[get("/task/{task_global_id}")]
pub async fn get_task() -> Json<String> {
   return Json("hello world", to_string());
}