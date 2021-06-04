use actix_web::{web, post, App, HttpServer, Result, HttpResponse};
use serde::{Deserialize, Serialize};
use actix_web::http::header;
use actix_cors::Cors;



#[derive(Deserialize)]
pub struct RequestBody {
    id  : u16,
    name: String
}

#[derive(Serialize)]
pub struct ResponseBody {
    message:String
}

// APIハンドラ
// 以下の例ではここを書き換えてレスポンスを変化させていきます
async fn index(request_body: web::Json<RequestBody>) -> actix_web::Result<HttpResponse> {

    println!("request_body.id={}", request_body.id);
    println!("request_body.name={}", request_body.name);

    let message = format!("私のIDは{}ですよよよ名前は{}です", 
                            request_body.id,
                            request_body.name);
    Ok(HttpResponse::Ok().json(ResponseBody {
        message
    }))
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");


    HttpServer::new(|| {
        let cors = Cors::default()
            .allowed_origin_fn(|origin, _req_head| {
                true
            })
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
            .allowed_header(header::CONTENT_TYPE)
            .supports_credentials()
            .max_age(3600);

            App::new()
            // ここでハンドラとエンドポイントを対応付け
                .route("/api", web::post().to(index))
    })
    .bind("127.0.0.1:38080")?
    .run()
    .await
}