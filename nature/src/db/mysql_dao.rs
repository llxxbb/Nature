use std::env;

use mysql_async::{Conn, from_row, Params, Pool, Row};
use mysql_async::error::{DriverError, Error};
use mysql_async::prelude::*;

pub use instance_dao::*;
pub use meta_dao::*;
pub use relation_dao::*;
pub use task_dao::*;
pub use task_err_dao::*;

use crate::domain::*;

pub mod task_check;
mod instance_dao;
mod meta_dao;
mod relation_dao;
mod task_dao;
mod task_err_dao;

lazy_static! {
   static ref POOL : Pool = get_conn();
}

pub struct MySql;

impl MySql {
    /// count
    pub async fn count<Q, P>(query: Q, params: P) -> Result<u32>
        where
            Q: AsRef<str>,
            P: Into<Params>,
    {
        let conn = MySql::conn().await?;
        match conn.first_exec(query, params).await {
            Ok((_, rtn)) => match rtn {
                Some(row) => {
                    Ok(from_row(row))
                }
                None => Ok(0)
            }
            Err(e) => return Err(MysqlError(e).into())
        }
    }

    /// i(nsert) d(elete) u(pdate)
    pub async fn idu<Q, P>(query: Q, params: P) -> Result<u64>
        where
            Q: AsRef<str>,
            P: Into<Params>,
    {
        let conn = MySql::conn().await?;
        match conn.prep_exec(query, params).await {
            Ok(num) => {
                match num.last_insert_id() {
                    Some(id) => Ok(id),
                    None => Ok(num.affected_rows())
                }
            }
            Err(e) => return Err(MysqlError(e).into())
        }
    }

    pub async fn fetch<Q, P, F, U>(query: Q, params: P, mut fun: F) -> Result<Vec<U>>
        where
            Q: AsRef<str>,
            P: Into<Params>,
            F: FnMut(Row) -> U,
    {
        let conn = MySql::conn().await?;
        match conn.prep_exec(query, params).await {
            Ok(rtn) => {
                match rtn.map_and_drop(|one| fun(one)).await {
                    Ok((_, rtn)) => Ok(rtn),
                    Err(e) => Err(MysqlError(e).into())
                }
            }
            Err(e) => Err(MysqlError(e).into())
        }
    }


    async fn conn() -> Result<Conn> {
        match POOL.get_conn().await {
            Ok(conn) => Ok(conn),
            Err(e) => Err(MysqlError(e).into())
        }
    }
}

fn get_conn() -> Pool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    Pool::new(database_url)
}


pub struct MysqlError(mysql_async::error::Error);

impl Into<NatureError> for MysqlError {
    fn into(self) -> NatureError {
        let msg = format!("database exception: {}", self.0.to_string());
        warn!("{}", msg);
        match self.0 {
            Error::Driver(err) => match err {
                DriverError::ConnectionClosed => NatureError::EnvironmentError(msg),
                DriverError::PoolDisconnected => NatureError::EnvironmentError(msg),
                _ => NatureError::LogicalError(msg)
            },
            Error::Io(_) => NatureError::EnvironmentError(msg),
            Error::Other(_) => NatureError::LogicalError(msg),
            Error::Server(e) => match e.code {
                1062 => NatureError::DaoDuplicated(msg),
                _ => NatureError::EnvironmentError(msg)
            }
            Error::Tls(_) => NatureError::LogicalError(msg),
            Error::Url(_) => NatureError::LogicalError(msg),
        }
    }
}

