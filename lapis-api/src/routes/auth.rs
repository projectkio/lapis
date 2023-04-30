use crate::{
    jwt,
    model::{LoginUserSchema, RegisterUserSchema, TokenClaims, User},
    responses::user::FilteredUser,
    state::AppState,
};
use actix_web::{
    cookie::{time::Duration as ActixWebDuration, Cookie},
    get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder,
};
use argon2::{
    password_has::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{prelude::*, Duration};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use sqlx::Row;

#[post("/auth/register")]
async fn user_register(
    body: web::Json<RegisterUserSchema>,
    data: web::Data<AppState>,
) -> impl Responder {
    let exists: bool = sqlx::query("SELECT EXISTS(SELECT 1 FROM users WHERE email = $1)")
        .bind(body.email.to_owned())
        .fetch_one(&data.db)
        .await
        .unwrap()
        .get(0);

    if exists {
        return HttpResponse::Conflicts().json(serde_json::json!({
            "status": "fail",
            "message": "User with that email already exists",
        }));
    }

    let salt = SaltString::generate(&mut OsRng);
    let hashed_password = Argon2::default()
        .hash_password(body.password.as_bytes(), &salt)
        .expect("Error while hashing password")
        .to_string();

    let query_result = sqlx::query_as!(
        User,
        "INSERT INTO users (name, email, password) VALUES ($1, $2, $3) RETURNING *",
        body.name.to_string(),
        body.email.to_string().to_lowercase(),
        hashed_password
    )
    .fetch_one(&data.db)
    .await;

    match query_result {
        Ok(user) => {
            let user_response = serde_json::json!({
                "status": "success",
                "data": serde_json::json!({
                    "user": FilteredUser::from(user)
                })
            });

            return HttpResponse::Ok().json(user_response);
        }
        Err(e) => {
            return HttpResponse::InternalServerError().json(serde_json::json!({
                "status": "error",
                "message": format!("{:?}", e)
            }));
        }
    }
}

#[post("/auth/login")]
async fn user_login(body: web::Json<LoginUserSchema>, data: web::Data<AppState>) -> impl Responder {
    let query_result = sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", body.email)
        .fetch_optional(&data.db)
        .await
        .unwrap();

    let is_valid = query_result.to_owned().map_or(false, |user| {
        let parsed_hash = PasswordHash::new(&user.password).unwrap();
        Argon2::default()
            .verify_password(body.password.as_bytes(), &parsed_hash)
            .map_or(false, |_| true)
    });

    if !is_valid {
        return HttpResponse::BadRequest().json(json!({
            "status": "fail",
            "message": "Invalid email or password",
        }));
    }

    let user = query_result.unwrap();

    let now = Utc::now();
    let iat = now.timestamp() as usize;
    let exp = (now + Duration::minutes(60)).timestamp() as usize;
    let claims: TokenClaims = TokenClaims {
        sub: user.id.to_string(),
        exp,
        iat,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(data.env.jwt_secret.as_ref()),
    )
    .unwrap();

    let cookie = Cookie::build("token", token.to_owned())
        .path("/")
        .max_age(ActixWebDuration::new(60 * 60, 0))
        .http_only(true)
        .finish();

    HttpResponse::ok().cookie(cookie).json(json!({
        "status": "success",
        "token": token
    }))
}

#[get("/auth/logout")]
async fn user_logout(_: jwt::JwtMiddleware) -> impl Responder {
    let cookie = Cookie::build("token", "")
        .path("/")
        .max_age(ActixWebDuration::new(-1, 0))
        .http_only(true)
        .finish();

    HttpResponse::Ok().cookie(cookie).json(json!({
        "status": "success"
    }))
}
