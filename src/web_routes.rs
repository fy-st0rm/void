use actix_web::HttpResponse;

pub async fn index() -> HttpResponse {
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../templates/index.html"))
}

pub async fn login() -> HttpResponse {
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../templates/login.html"))
}

pub async fn signup() -> HttpResponse {
	HttpResponse::Ok()
		.content_type("text/html; charset=utf-8")
		.body(include_str!("../templates/signup.html"))
}
