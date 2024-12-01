use axum::{routing::post, Router};
pub mod student_model;
use self::student_model::{calculate_grade};


pub fn get_grade_percentage() -> Router {
    Router::new()
    .route("/student/getGrade", post(calculate_grade))
}