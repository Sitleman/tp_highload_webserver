extern crate core;

use std::{fs, io::{prelude::*, BufReader}, net::{TcpListener, TcpStream}, thread};
use std::arch::x86_64::_rdrand32_step;
use std::thread::sleep;
use std::time::Duration;
use rust::ThreadPool;
use urlencoding::decode;
use std::time::SystemTime;
use chrono;
use url::Url;
use log::{info, warn};
use std::env;

const ROOT_DIR: &str = ".";
const INDEX_FILE: &str = "index.html";
const PORT: &str = "8081";

fn main() {
    let listener = TcpListener::bind(format!("{}{}", "0.0.0.0:", PORT)).unwrap();
    println!("{}{}", "Server start on port: ", PORT);
    println!("{}",std::env::current_dir().unwrap().display());
    let pool = ThreadPool::new(10);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap_or(Result::Ok("".to_string())).unwrap_or_default();

    let content;
    let status;
    let resp_server = "n.gureev";
    let resp_date = format!("{:?}", chrono::offset::Local::now());
    let resp_connection = "n.gureev connection";

    let (method, url, direct_path) = clear_url(request_line.clone());
    // println!("{}", request_line);
    if method != "GET" && method != "HEAD" {
        //println!("{}", method);
        status = "HTTP/1.1 405 METHOD NOT ALLOWED";
        let response = format!("{status}\r\n\
        Server: {resp_server}\r\n\
        Date: {resp_date}\r\n\
        Connection: {resp_connection}\r\n\r\n");
        stream.write_all(response.as_bytes()).unwrap();
        return
    }

    let fullUrl = ROOT_DIR.to_string() + &url;
    println!("{}", fullUrl);
    if fs::metadata(&fullUrl).is_ok() {
        content = fs::read(&fullUrl).unwrap();
        status = "HTTP/1.1 200 OK";
    } else {
        content = fs::read("./static/not_found.html").unwrap();
        if direct_path {
            status = "HTTP/1.1 404 NOT FOUND";
        } else {
            status = "HTTP/1.1 403 FORBIDDEN";
        }
    }
    // println!("{}", fullUrl);
    // println!("{}", status);

    let cont_length = content.len();
    let cont_type = get_mimotype(url);

    let response = format!("{status}\r\n\
    Server: {resp_server}\r\n\
    Date: {resp_date}\r\n\
    Connection: {resp_connection}\r\n\
    Content-Type: {cont_type}\r\n\
    Content-Length: {cont_length}\r\n\r\n");
    stream.write_all(response.as_bytes()).unwrap();
    if method == "GET" {
        stream.write_all(&*content).unwrap();
    }
}

fn clear_url(req :String) -> (String, String, bool) {
    let parts : Vec<&str> = req.split(" ").collect();
    // println!("{:?}", parts);
    if parts.len() != 3 {
        return ("".to_string(), "".to_string(), true);
    }
    let  method =  parts.get(0).unwrap().to_string();
    let  raw_url =  parts.get(1).unwrap();
    let decode_url = decode(raw_url).expect("UTF-8").to_string();
    let url_with_params : Vec<&str>= decode_url.split("?").collect();
    let url = url_with_params[0];
    if url.contains("../") {
        return (method, "/badway.txt".to_string(), true);
    }
    if !url.contains(".") {
        if url.ends_with("/") {
            return (method, url.to_string() + INDEX_FILE, false);
        }
        return (method, url.to_string() + "/" + INDEX_FILE, false);
    }
    return (method, url.to_string(), true);
}

fn get_mimotype(req :String) -> String {
    let parts : Vec<&str> = req.split(".").collect();
    // println!("{}", parts.get(1).unwrap().to_string());
    if parts.len() < 2 {
        return "".to_string();
    }
    let ext= parts[parts.len() - 1];
    return match ext {
        "html" => "text/html",
        "css" => "text/css",
        "js" => "text/javascript",
        "png" => "image/png",
        "jpeg" => "image/jpeg",
        "jpg" => "image/jpeg",
        "gif" => "image/gif",
        "swf" => "application/x-shockwave-flash",
        _ => "",
    }.to_string();
}
