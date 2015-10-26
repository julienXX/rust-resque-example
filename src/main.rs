extern crate redis;
extern crate rustc_serialize;

use redis::Commands;
use rustc_serialize::Encodable;
use rustc_serialize::json::{self};

#[derive(RustcEncodable, Debug)]
pub struct Job {
    class: String,
    args: Vec<String>
}

fn main() {
    let job: Job = Job { class: "SignupEmail".to_owned(),
                         args: vec!["user@example.com".to_owned()] };

    match enqueue(job) {
        Ok(job) => println!("Enqueued job: {:?}", job),
        Err(_) => { /* connection failed */ }
    }
}

fn enqueue(job: Job) -> redis::RedisResult<Job> {
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let conn = try!(client.get_connection());

    // Encode our Job in JSON
    let json_job = json::encode(&job).unwrap();

    // Add our queue in resque:queues Set
    let _: () = try!(conn.sadd("resque:queues", "rust_test_queue"));
    // Push our job in the resque:queue:rust_test_queue list
    let _: () = try!(conn.rpush("resque:queue:rust_test_queue", json_job));

    Ok(job)
}
