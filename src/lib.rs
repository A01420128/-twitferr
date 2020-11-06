extern crate redis;
use redis::Commands;

pub struct Twitferr {
    conn: redis::Connection,
}

impl Twitferr {
    pub fn new(address: &str) -> Twitferr {
        let client = redis::Client::open(address).unwrap();
        let mut conn = client.get_connection().unwrap();

        let _: () = redis::cmd("flushdb").query(&mut conn).unwrap();

        let _: () = conn
            .hset_multiple(
                "users",
                &[("juan", "1234"), ("roberto", "2345"), ("maria", "3456")],
            )
            .unwrap();

        Twitferr { conn }
    }
}
