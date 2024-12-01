use axum::Router;
use crate::calculate_grade_percentage::get_grade_percentage;
pub fn get_routes() -> Router{
    Router::new().merge(get_grade_percentage())
}