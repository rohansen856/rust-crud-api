use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct RoutineModel {
    pub id: String,
    pub course_id: String,
    pub course_code: String,
    pub course_title: String,
    pub prof: String,
    pub class_type: String,
    pub day: u8,
    pub from: String,
    pub to: String,
    pub group: String,
    pub branch: String,
    pub institute: String,
    pub room: String,
    pub semester: u8,
}