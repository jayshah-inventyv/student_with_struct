use axum::Json;
use serde::{Serialize, Deserialize};
use std::fs;

use axum::{
    http::StatusCode,
    response::{IntoResponse, Response}
};
use serde_json::Value;


#[derive(Deserialize)]
pub struct RequestData {
    id: i64,
    exam: String
}

#[derive(Serialize)]
struct ResObj<T> {
    status: i32,
    data: T,
}


#[derive(Serialize, Deserialize)]
struct ResponseData {
    student_name: String,
    exam:String,
    percentage: f64,
    email:String,
    phone_number:String,
    grade:String
}

pub async fn calculate_grade(Json(payload): Json<RequestData>) -> Response {
    let data = fs::read_to_string("./src/calculate_grade_percentage/studentData.json".to_string());
    match data {
        Ok(data) => {
            let res: Result<Value, serde_json::Error> = serde_json::from_str(&data);
            match res {
                Ok(studentsdata) => {
                    let students_array = studentsdata
                        .as_array()
                        .expect("Expected studentsdata to be an array");
                    let student = students_array
                        .iter()
                        .find(|x| x.get("id").and_then(|id| id.as_i64()) == Some(payload.id));
                    match student {
                        Some(student) => {
                            let percentage = calculate_percentage(student, &payload.exam);
                            let grade = calculate_grade_from_percentage(percentage);
                            let res_data = ResponseData{
                                student_name: student["name"].as_str().unwrap().to_string(),
                                email: student["email"].as_str().unwrap().to_string(),
                                phone_number: student["phone"].as_str().unwrap().to_string(),
                                percentage,
                                grade,
                                exam: payload.exam
                            };
                            return (
                                StatusCode::OK,
                                Json(ResObj {
                                    status: 2000,
                                    data: res_data,
                                }),
                            ).into_response();
                        }
                        None => {
                            println!("No student found with id: {}", payload.id);
                            return (
                                StatusCode::NOT_FOUND,
                                Json(ResObj {
                                    status: 4000,
                                    data: "No student found with id: ".to_string(),
                                }),
                            )
                                .into_response();
                        }
                    }
                    
                }
                Err(e) => {
                    println!("Error while serializing data: {}", e);
                    return (
                        StatusCode::OK,
                        Json(ResObj {
                            status: 4000,
                            data: "Error while serializing data:".to_string(),
                        }),
                    )
                        .into_response();
                }
            }
        }
        Err(e) => {
            println!("Error reading file: {}", e);
            return (
                StatusCode::OK,
                Json(ResObj {
                    status: 4000,
                    data: "Error reading file:".to_string(),
                }),
            )
                .into_response();
        }
    };
}

fn calculate_percentage(student: &Value, exam: &String) -> f64 {
    let mut percentage: f64 = 0.0;
    let mut total_marks: i64 = 0;
    let mut total_subject: i64 = 0;
    for result in student["marks"][exam]["marksData"].as_array().unwrap() {
        for (_subject, marks) in result.as_object().unwrap().iter() {
            total_marks = total_marks + marks["marks"].as_i64().unwrap();
        }
    }
    total_subject = student["marks"][exam]["marksData"]
        .as_array()
        .unwrap()
        .len() as i64;
    percentage = (total_marks / total_subject) as f64;
    percentage
}

fn calculate_grade_from_percentage(percentage: f64) -> String {
    match percentage {
        p if p >= 90.0 => "A1".to_string(),
        p if p >= 80.0 => "A2".to_string(),
        p if p >= 70.0 => "B1".to_string(),
        p if p >= 60.0 => "B2".to_string(),
        p if p >= 50.0 => "C1".to_string(),
        p if p >= 40.0 => "C2".to_string(),
        p if p >= 33.0 => "D".to_string(),
        _ => "E".to_string()
    }
}