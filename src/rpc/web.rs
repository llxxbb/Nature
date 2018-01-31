use biz::*;

struct Web;

impl Teller for Web {
    fn input(_d: WorldConnectionData) -> WorldConnectionResult {
        WorldConnectionResult {
            status: "OK".to_string(),
            err_msg: "".to_string(),
            serial_number: 1,
        }
    }
}
