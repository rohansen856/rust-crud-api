use actix_web::{get, http::StatusCode, post, web::Query, HttpRequest, HttpResponse, Responder};
use validator::Validate;
use serde::{Deserialize, Serialize};

use crate::schema::routine::RoutineModel;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct GetStudentRoutineByDay {
   #[validate(range(min=1, max=5))]
   day: u8,
   institute: String,
   branch: String,
   #[validate(range(min=1, max=10))]
   semester: u8,
}

#[get("/getStudentRoutineByDay")]
async fn get_student_routine_by_day(req: HttpRequest, query: Query<GetStudentRoutineByDay>) -> impl Responder {
    let request: GetStudentRoutineByDay = query.into_inner();
    let is_valid = request.validate();
    match is_valid {
        Ok(_)=>{
            let _response = RoutineModel {
                id: String::from("id"),
                branch: request.branch,
                class_type: String::from(""),
                course_code: String::from(""),
                course_id: String::from(""),
                course_title: String::from(""),
                from: String::from(""),
                to: String::from(""),
                group: String::from(""),
                institute: request.institute,
                prof: String::from(""),
                room: String::from(""),
                day: 2,
                semester: 2
            };
            HttpResponse::Ok().status(StatusCode::OK).body(format!("{:?}",req))
        },
        Err(_err)=>{
            HttpResponse::Ok().status(StatusCode::BAD_REQUEST).body("Type error")}
    }
}

#[get("/routine?query={query}")]
async fn get_routine() -> impl Responder {
    HttpResponse::Ok().body("hello")
}

#[post("/routine")]
async fn post_routine() -> impl Responder {
    HttpResponse::Ok().body("body")
}