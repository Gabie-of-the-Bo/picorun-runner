use std::fs;
use std::io::Read;
use std::process::{Command, Stdio};
use wait_timeout::ChildExt;
use std::time::Duration;
use actix_web::{Responder, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct ProgramOutput {
    output: String,
    logs: String
}

pub enum Language{
    Python
}

fn run_python(timeout: u64) -> Result<ProgramOutput, ProgramOutput>{
    let mut child = Command::new("python3").args(&["environments/python/execution.py"])
                                          .stdout(Stdio::piped())
                                          .stderr(Stdio::piped()) 
                                          .spawn()
                                          .expect("Error executing Python command");

    return match child.wait_timeout(Duration::from_secs(timeout)).unwrap() {
        Some(exit_code) => {
            let mut logs = String::new();
            child.stdout.unwrap().read_to_string(&mut logs).expect("Error while reading stdout");

            if exit_code.success(){
                let output = fs::read_to_string("environments/python/output.out").expect("Error while reading output file");
                return Ok(ProgramOutput{
                    output: output,
                    logs: logs
                });

            } else{
                let mut output = String::new();
                child.stderr.unwrap().read_to_string(&mut output).expect("Error while reading stderr");
                return Err(ProgramOutput{
                    output: output,
                    logs: logs
                });
            }
        },

        None => {
            child.kill().unwrap();

            Err(ProgramOutput{
                output: format!("Timeout: the program did not finish in {} seconds", timeout),
                logs: String::new()
            })
        }
    };
}

pub fn run_code(language: Language, timeout: u64) -> Result<ProgramOutput, ProgramOutput>{
    return match language{
        Language::Python => run_python(timeout)
    }
}

pub async fn execute_python() -> impl Responder {
    return match run_code(Language::Python, 10){
        Ok(res) => Ok(HttpResponse::from(serde_json::to_string(&res))),
        Err(err) => {
            error!("{}", err.output);
            Err(HttpResponse::from(serde_json::to_string(&err)))
        }
    }
}