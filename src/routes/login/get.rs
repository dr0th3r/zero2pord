use actix_web::{web, HttpResponse};
use actix_web::http::header::ContentType;

pub async fn login_form() -> HttpResponse {
    let error_html: String = todo!();
    HttpResponse::Ok()
        .content_type(ContentType::html())
        .body(format!(
            r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta http-equiv="content-type" content="text/html; charset=utf-8">
    <title>Login</title>
</head>
<body>
    {error_html}
   <form action="/login" method="post">
        <label>Username
            <input
                type="text"
                placeholder="Enter Username"
                name="username"
            >
        </label>
        <label>Password
            <input 
                type="text"
                placeholder="Enter Password"
                name="Password"
            >
        </label>
        <button type="submit">Login</button>
   </form> 
</body>
</html>"#,
        ))
}