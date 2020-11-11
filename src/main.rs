use actix_web::http::StatusCode;
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer, Result};
use std::collections::HashMap;
use std::env;

async fn index(req: HttpRequest) -> Result<HttpResponse> {
    let connection_info = req.connection_info().clone();

    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };

    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    let headers = req.headers().to_owned();
    let ua = match headers.get("User-Agent") {
        Some(x) => x.to_str().unwrap().to_owned(),
        None => String::from("Unknown"),
    };
    if ua.starts_with("curl") {
        // response
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(format!("{}\n", ip)))
    } else {
        // response
        Ok(HttpResponse::build(StatusCode::OK)
            .content_type("text/plain; charset=utf-8")
            .body(format!(
                "your ip is: {}\nother routes:\n/ip\n/ua\n/all\n/raw/ip\n/raw/headers\n/raw/useragent\n/raw/all\n/json/ip\n/json/headers\n/json/useragent\n/json/all",
                ip
            )))
    }
}

async fn raw_ip(req: HttpRequest) -> Result<HttpResponse> {
    let connection_info = req.connection_info().clone();

    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };

    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body(format!("{}\n", ip)))
}

async fn raw_user_agent(req: HttpRequest) -> Result<HttpResponse> {
    let headers = req.headers().to_owned();
    let ua = match headers.get("User-Agent") {
        Some(x) => x.to_str().unwrap().to_owned(),
        None => String::from("Unknown"),
    };

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body(format!("{}\n", ua)))
}

async fn raw_headers(req: HttpRequest) -> Result<HttpResponse> {
    let headers = req.headers().to_owned();
    let mut header_str = String::new();

    for header in headers.iter() {
        header_str.push_str(&format!(
            "{}: {}\n",
            header.0.as_str(),
            header.1.to_str().unwrap()
        ));
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body(header_str))
}

async fn raw_all(req: HttpRequest) -> Result<HttpResponse> {
    let headers = req.headers().to_owned();
    let connection_info = req.connection_info().clone();
    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };
    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    let mut response = String::new();
    response.push_str(&format!("ip: {}\n", ip));
    for header in headers.iter() {
        response.push_str(&format!(
            "{}: {}\n",
            header.0.as_str(),
            header.1.to_str().unwrap()
        ));
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("text/plain; charset=utf-8")
        .body(response))
}

async fn json_ip(req: HttpRequest) -> Result<HttpResponse> {
    let connection_info = req.connection_info().clone();

    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };

    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    let mut response = HashMap::new();
    response.insert("ip".to_string(), ip.to_string());
    // response
    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(response))
}

async fn json_user_agent(req: HttpRequest) -> Result<HttpResponse> {
    let headers = req.headers().to_owned();
    let ua = match headers.get("User-Agent") {
        Some(x) => x.to_str().unwrap().to_owned(),
        None => String::from("Unknown"),
    };

    let mut response = HashMap::new();
    response.insert("user-agent".to_string(), ua.to_string());

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(response))
}

async fn json_headers(req: HttpRequest) -> Result<HttpResponse> {
    let header_map = req.headers().to_owned();
    let mut headers = HashMap::new();

    for header in header_map.iter() {
        headers.insert(header.0.to_string(), header.1.to_str().unwrap().to_owned());
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(headers))
}

async fn json_all(req: HttpRequest) -> Result<HttpResponse> {
    let connection_info = req.connection_info().clone();
    let ip_port = match connection_info.realip_remote_addr() {
        Some(ip) => ip.to_owned(),
        None => String::from(""),
    };
    let ip = ip_port
        .split(":")
        .map(|x| x.to_owned())
        .collect::<Vec<String>>()
        .get(0)
        .unwrap()
        .to_owned();

    let mut response = HashMap::new();
    response.insert("ip".to_string(), ip.to_string());

    let header_map = req.headers().to_owned();
    for header in header_map.iter() {
        response.insert(header.0.to_string(), header.1.to_str().unwrap().to_owned());
    }

    Ok(HttpResponse::build(StatusCode::OK)
        .content_type("application/json; charset=utf-8")
        .json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .wrap(middleware::Logger::default())
            .route("/", web::get().to(index))
            .route("/ip", web::get().to(raw_ip))
            .route("/ua", web::get().to(raw_user_agent))
            .route("/all", web::get().to(raw_all))
            .route("/raw/ip", web::get().to(raw_ip))
            .route("/raw/all", web::get().to(raw_all))
            .route("/raw/headers", web::get().to(raw_headers))
            .route("/raw/useragent", web::get().to(raw_user_agent))
            .route("/json/ip", web::get().to(json_ip))
            .route("/json/all", web::get().to(json_all))
            .route("/json/headers", web::get().to(json_headers))
            .route("/json/useragent", web::get().to(json_user_agent))
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
