use reqwest::Client;

use nature_common::{CallOutParameter, ConverterReturned, Instance};

use crate::task::ExecutorTrait;

lazy_static! {
    static ref CLIENT : Client = Client::new();
}

pub struct HttpExecutorImpl;

impl ExecutorTrait for HttpExecutorImpl {
    fn execute(&self, address: &str, para: &CallOutParameter) -> ConverterReturned {
        let response = CLIENT.post(address).json(para).send();
        match response {
            Err(e) => {
                warn!("failed get result from : {}, err: {:?}", address, e);
                ConverterReturned::EnvError
            }
            Ok(mut res) => {
                let result = res.json::<Vec<Instance>>();
                match result {
                    Err(e) => ConverterReturned::LogicalError(e.to_string()),
                    Ok(ins) => ConverterReturned::Instances(ins)
                }
            }
        }
    }
}
//
//#[cfg(test)]
//mod test {
//    use std::thread;
//    use std::thread::JoinHandle;
//    use std::time::Duration;
//
//    use actix_web::{App, HttpRequest, HttpServer, Result, web};
//    use actix_web::web::Json;
//
//    use nature_common::{CallOutParameter, ConverterReturned, Instance};
//
//    use crate::task::convert::caller::ExecutorTrait;
//    use crate::task::HttpExecutorImpl;
//
//    fn index(_req: &HttpRequest) -> Result<Json<Vec<Instance>>> {
//        Ok(Json(vec!(Instance::default())))
//    }
//
//    #[test]
//    fn response_ok() {
//        let _ = start_web_server();
//
//        // get result
//        let address = "http://127.0.0.1:8088";
//        let para = CallOutParameter {
//            from: Instance::default(),
//            last_status: None,
//            carrier_id: vec![],
//        };
//        let executor_impl = HttpExecutorImpl {};
//        let converted = executor_impl.execute(address, &para);
//        match converted {
//            ConverterReturned::Instances(ins) => {
//                assert_eq!(1, ins.len())
//            }
//            _ => panic!("err returned")
//        }
//    }
//
//    fn start_web_server() -> JoinHandle<()> {
//        let handler = thread::spawn(|| {
//            HttpServer::new(|| App::new().service(web::resource("/").to(index)))
//                .bind("127.0.0.1:8088")
//                .unwrap()
//                .run();
//        });
//        thread::sleep(Duration::new(1, 0));
//        handler
//    }
//}
