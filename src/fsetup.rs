use std::fs::File;
use std::io::prelude::*;
use actix_web::{web, Responder, HttpResponse};

use crate::preproc::*;
use crate::runner::Language;

fn write_file(filename: &str, content: &str) -> Result<(), String>{
    return match File::create(filename){
        Ok(mut file) => {
            match file.write_all(content.as_bytes()){
                Ok(()) => Ok(()),
                Err(_) => Err(format!("Error while writing to '{}'", filename))
            }
        },

        Err(_) => {
            Err(format!("Error while creating '{}'", filename))
        }
    }
}

fn write_preparation(language: Language, content: &str) -> Result<(), String>{
    return match language{
        Language::Python => write_file("environments/python/preparation.py", content)
    };
}

fn write_execution(language: Language, content: &str) -> Result<(), String>{
    return match language{
        Language::Python => write_file("environments/python/execution.py", preproc_python_execution(content).as_str())
    };
}

fn write_code(language: Language, content: &str) -> Result<(), String>{
    return match language{
        Language::Python => write_file("environments/python/code.py", preproc_python_code(content).as_str())
    };
}

pub async fn write_preparation_python(content: web::Bytes) -> impl Responder {
    return match write_preparation(Language::Python, String::from_utf8(content.to_vec()).unwrap().as_str()){
        Ok(()) => HttpResponse::Ok(),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
        }
    }
}

pub async fn write_execution_python(content: web::Bytes) -> impl Responder {
    return match write_execution(Language::Python, String::from_utf8(content.to_vec()).unwrap().as_str()){
        Ok(()) => HttpResponse::Ok(),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
        }
    }
}

pub async fn write_code_python(content: web::Bytes) -> impl Responder {
    return match write_code(Language::Python, String::from_utf8(content.to_vec()).unwrap().as_str()){
        Ok(()) => HttpResponse::Ok(),
        Err(err) => {
            error!("{}", err);
            HttpResponse::InternalServerError()
        }
    }
}