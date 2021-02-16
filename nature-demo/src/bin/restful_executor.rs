#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate log;

use std::{env, thread};
use std::str::FromStr;
use std::time::Duration;

use actix_web::{App, HttpResponse, HttpServer, web};
use actix_web::dev::Server;
use actix_web::web::Json;
use dotenv::dotenv;
use reqwest::blocking::Client as BClient;
use reqwest::Client;

use nature::domain::*;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let _ = env_logger::init();
    start_actrix().await
}


lazy_static! {
    pub static ref CLIENT : Client = Client::new();
    pub static ref BCLIENT : BClient = BClient::new();
    pub static ref CALLBACK_ADDRESS: String = "http://localhost:8080/callback".to_string();
    pub static ref GET_BY_META: String = "http://localhost:8080/get_by_key_range".to_string();
}

async fn send_to_warehouse(para: Json<ConverterParameter>) -> HttpResponse {
    thread::spawn(move || send_to_warehouse_thread(para.0));
    // wait 60 seconds to simulate the process of warehouse business.
    HttpResponse::Ok().json(ConverterReturned::Delay { num: 60 })
}

async fn add_score(para: Json<Vec<Instance>>) -> HttpResponse {
    let mut rtn = para.0;
    rtn.iter_mut().for_each(|one| {
        if one.para.contains("subject2") {
            let points = u16::from_str(&one.content).unwrap();
            let content = (points + 4).to_string();
            one.data.content = content;
        }
    });
    HttpResponse::Ok().json(Ok(rtn) as Result<Vec<Instance>>)
}

pub fn start_actrix() -> Server {
    let port = env::var("DEMO_CONVERTER_PORT").unwrap_or_else(|_| "8082".to_string());
    HttpServer::new(
        || App::new()
            .route("/send_to_warehouse", web::post().to(send_to_warehouse))
            .route("/add_score", web::post().to(add_score))
    ).bind("127.0.0.1:".to_owned() + &port).unwrap()
        .run()
}

pub fn send_to_warehouse_thread(para: ConverterParameter) {
    // wait 50ms
    thread::sleep(Duration::new(0, 50000));
    // send result to Nature
    let rtn = DelayedInstances {
        task_id: para.task_id,
        result: ConverterReturned::Instances { ins: vec![para.from] },
    };
    let rtn = BCLIENT.post(&*CALLBACK_ADDRESS).json(&rtn).send();
    let text: String = rtn.unwrap().text().unwrap();
    if text.contains("Err") {
        error!("{}", text);
    } else {
        debug!("warehouse business processed!")
    }
}

pub async fn get_by_meta(cond: &KeyCondition) -> Result<Vec<Instance>> {
    // let rtn = CLIENT.post(&*GET_BY_META).json(cond).send().await?.json::<ConverterReturned>().await?;
    let res = CLIENT.post(&*GET_BY_META).json(cond).send().await?;
    let rtn = res.json::<Result<Vec<Instance>>>().await?;
    // let _ = dbg!(&rtn);
    rtn
}


#[cfg(test)]
mod reqwest_test {
    use reqwest::{Client, Error};
    use tokio::runtime::Runtime;

    use nature::domain::{ConverterParameter, ConverterReturned};

    #[test]
    fn reqwest_test() {
        let _rtn = Runtime::new().unwrap().block_on(http_call());
    }

    async fn http_call() -> Result<(), Error> {
        let para = ConverterParameter {
            from: Default::default(),
            last_state: None,
            task_id: 0,
            master: None,
            cfg: "".to_string(),
        };
        let client = Client::new();
        let rtn = client.post("http://localhost:8082/send_to_warehouse").json(&para).send().await?.json::<ConverterReturned>().await?;
        dbg!(rtn);
        Ok(())
    }
}

