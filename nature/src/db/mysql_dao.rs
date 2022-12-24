use std::env;

use mysql_async::{Conn, DriverError, Error, from_row, Params, Pool, Row};
use mysql_async::prelude::{Query, Queryable, StatementLike, WithParams};

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
    pub async fn count(query: &str, params: Params) -> Result<u32>
    {
        let mut conn = MySql::conn().await?;
        match query.with(params).first(&mut conn).await {
            Ok(rtn) => match rtn {
                Some(row) => {
                    Ok(from_row(row))
                }
                None => Ok(0)
            }
            Err(e) => return Err(MysqlError(e).into())
        }
    }

    /// i(nsert) d(elete) u(pdate)
    pub async fn idu<P>(query: &str, params: P) -> Result<u64>
        where P: Into<Params>
    {
        let conn = MySql::conn().await?;
        match query.to_string().with(params.into()).run(conn).await {
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
            Q: AsRef<str> + StatementLike,
            P: Into<Params> + Send,
            F: FnMut(Row) -> U,
    {
        let mut conn = MySql::conn().await?;
        match conn.exec(query, params).await {
            Ok(rtn) => {
                let map = rtn.into_iter().map(|one| fun(one)).collect();
                Ok(map)
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
    Pool::new(database_url.as_str())
}


pub struct MysqlError(Error);

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
            Error::Url(_) => NatureError::LogicalError(msg),
        }
    }
}

