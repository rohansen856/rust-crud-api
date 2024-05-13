use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Deserialize, Serialize, Debug, Validate)]
pub struct GetStudentRoutineByDay {
   #[validate(range(min=1, max=5))]
   day: u8,
   institute: String,
   branch: String,
   #[validate(range(min=1, max=10))]
   semester: u8,
}