use super::schemas::APIResponse;
use actix_web::{HttpResponse, Responder, get, web::Path};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
struct FormulaResponse {
    rust: String,
    json: String,
}

#[get("/champions/{name}")]
pub async fn formulas_champions(name: Path<String>) -> impl Responder {
    let writer_content: String =
        match fs::read_to_string(format!("formulas/{}.txt", name.to_lowercase())) {
            Ok(content) => content,
            Err(e) => {
                println!(
                    "An error occured when reading formulas file for to champion: {}, error: {}",
                    name, e
                );
                String::from("Rust formula file not found in the system.")
            }
        };
    let json_content: String = match fs::read_to_string(format!("internal/champions/{}.json", name))
    {
        Ok(content) => content,
        Err(e) => {
            println!(
                "An error occured when reading json file for to champion: {}, error: {}",
                name, e
            );
            String::from("JSON formula file not found in the system.")
        }
    };

    HttpResponse::Ok().json(APIResponse {
        success: true,
        message: (),
        data: FormulaResponse {
            rust: writer_content,
            json: json_content,
        },
    })
}
