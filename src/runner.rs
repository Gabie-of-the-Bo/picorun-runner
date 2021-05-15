use std::fs;
use std::process::Command;
use wait_timeout::ChildExt;
use std::time::Duration;
use actix_web::{Responder, HttpResponse};

pub enum Language{
    Python
}

fn run_python(timeout: u64) -> Result<String, String>{
    let mut child = Command::new("python3").args(&["environments/python/execution.py"])
                                          .spawn()
                                          .expect("Error executing Python command");

    return match child.wait_timeout(Duration::from_secs(timeout)).unwrap() {
        Some(_) => {
            let output = fs::read_to_string("environments/python/output.out").expect("Error while reading output file");
            Ok(output)
        },

        None => {
            child.kill().unwrap();
            Err(format!("Timeout: the program did not finish in {} seconds", timeout))
        }
    };
}

pub fn run_code(language: Language, timeout: u64) -> Result<String, String>{
    return match language{
        Language::Python => run_python(timeout)
    }
}

pub async fn execute_python() -> impl Responder {
    return match run_code(Language::Python, 10){
        Ok(res) => Ok(HttpResponse::from(res)),
        Err(err) => {
            error!("{}", err);
            Err(HttpResponse::from(err))
        }
    }
}