mod connection;
use connection::PostgresClient;
use std::net::TcpListener;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::collections::HashMap;
use serde_json;

fn parse_mpvh_body(req: &str) -> (&str, &str) {
    let req_v: Vec<&str> = req.split("\r\n\r\n").collect();
    (req_v[0], req_v[1])
}

fn parse_mpv_headers(mpvh: &str) -> (&str, Vec<&str>) {
    let mut mpvh_v: Vec<&str> = mpvh.split("\r\n").collect();
    let headers = mpvh_v.split_off(1);
    let mpv = mpvh_v[0];
    (mpv, headers)
}

fn parse_mpv(mpv: &str) -> (&str, &str, &str) {
    let line_v: Vec<&str> = mpv.split(" ").collect();
    (line_v[0], line_v[1], line_v[2])
}

fn handle_client(mut stream:TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let peer_addr = &format!("{:?}", stream.peer_addr().unwrap());
    let mut buffer = [1;1024];
    stream.read(&mut buffer).unwrap();
    let req = String::from_utf8_lossy(&buffer);
    let (mpvh, body) = parse_mpvh_body(&req);
    let (mpv, headers) = parse_mpv_headers(mpvh);
    let (method, path, http_version) = parse_mpv(mpv);
    let mut headers_map = HashMap::new();
    for item in headers {
        let items: Vec<&str> = item.split(":").collect();
        headers_map.insert(items[0], items[1].trim());
    }
    let url = &format!("http://{}{}", headers_map["Host"], path);
    let headers_json = &(serde_json::to_string(&headers_map)?);
    let body_trim = body.trim_matches(char::from(1));

    println!("http_version:{}", http_version);
    println!("method:{}",       method);
    println!("url:{}",          url);
    println!("headers:{}",      headers_json);
    println!("body:{}",         body_trim);
    println!("peer_addr:{}",    peer_addr);
    println!("***********************************");

    let host = "db";
    let dbname = "postgres";
    let username = "web";
    let password = "123456";
    let mut psql = PostgresClient::new(host, dbname, username, password)?;
    psql.create_table();
    psql.insert_data(http_version, method, url, headers_json, body_trim, peer_addr);

    let status_line = if buffer.starts_with(b"GET / HTTP/1.1\r\n"){
        "HTTP/1.1 200\r\n\r\n200"
    }else{
        "HTTP/1.1 404 Not Found\r\n\r\n"
    };
    let response = format!("{}",status_line);
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("0.0.0.0:80")?;
    println!("running on 80");
    for stream in listener.incoming(){
        handle_client(stream?)?;
    }
    Ok(())
}