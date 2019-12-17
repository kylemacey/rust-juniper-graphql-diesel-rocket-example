
use std::env;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use diesel::pg::PgConnection;
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};

pub mod enums;
pub mod models;
pub mod schema;


pub type PgPool = Pool<ConnectionManager<PgConnection>>;

pub fn connect() -> PgPool {
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL is not configured");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::new(manager).expect("Failed to create pool")
}


pub struct Connection(pub PooledConnection<ConnectionManager<PgConnection>>);

impl<'a, 'r> FromRequest<'a, 'r> for Connection {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PgPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Connection(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

impl Deref for Connection {
    type Target = PgConnection;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}