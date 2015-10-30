extern crate redis;
extern crate rustc_serialize;

use redis::Commands;
use rustc_serialize::Encodable;
use rustc_serialize::json;


#[derive(RustcEncodable, RustcDecodable, Debug)]
pub struct Job {
    class: String,
    args: Vec<String>
}

fn main() {
    let job: Job = Job { class: "SignupEmail".to_owned(),
                         args: vec!["user@example.com".to_owned()] };

    enqueue(job).unwrap();

    loop {
        reserve().unwrap();
        wait_a_bit();
    }
}

fn enqueue(job: Job) -> redis::RedisResult<Job> {
    // Connect to a local Redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let conn = try!(client.get_connection());

    // Encode our Job in JSON
    let json_job = json::encode(&job).unwrap();

    // Add our queue in resque:queues Set
    try!(conn.sadd("resque:queues", "rust_test_queue"));
    // Push our job in the resque:queue:rust_test_queue list
    try!(conn.rpush("resque:queue:rust_test_queue", json_job));

    println!("Enqueued job: {:?}", job);

    Ok(job)
}

fn wait_a_bit() {
    println!("--: Sleeping for 5.0 seconds");
    std::thread::sleep_ms(5000);
}

fn reserve() -> redis::RedisResult<()> {
    println!("--: Checking rust_test_queue");

    // Connect to a local Redis
    let client = try!(redis::Client::open("redis://127.0.0.1/"));
    let conn = try!(client.get_connection());

    // Check if a job is present in the queue
    let res = conn.lpop("resque:queue:rust_test_queue").unwrap();

    // Perform the job or return
    match res {
        Some(job) => perform(job),
        None => return Ok(()),
    }
}

fn perform(json_job: String) -> redis::RedisResult<()> {
    // Decode JSON
    let job: Job = json::decode(&*json_job).unwrap();
    println!("Found job: {:?}", job);

    // Send our email with something like:
    // send_email(job.args.first());
    // not implemented here.

    Ok(())
}
