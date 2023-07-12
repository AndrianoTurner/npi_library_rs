use crate::auth;
use crate::auth::{AuthenticationToken, Response};
use crate::database::models::User;
use crate::models::request::NormalizeForm;
use crate::{
    models::request::{LoginRequest, RegisterRequest, ValidateForm},
    State,
};
use actix_web::web::ServiceConfig;
use actix_web::{get, post, web, HttpResponse};

#[derive(serde::Serialize, serde::Deserialize)]
struct LoginResponse {
    id: i32,
    email: String,
    token: String,
    errorCode: i32,
    status: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct RegisterResponse {
    status: String,
    errorCode: i32,
}

#[post("/login")]
async fn login(
    login_info: web::Json<LoginRequest>,
    state: web::Data<State>,
) -> actix_web::Result<HttpResponse> {
    let login_info = login_info.into_inner();
    login_info.validate()?;
    match state.database.get_user_by_email(&login_info.email).await {
        Ok(user) => {
            if user.check_password(&login_info.password) {
                let token = auth::encode_token(user.id).await;
                Ok(HttpResponse::Ok().json(LoginResponse {
                    id: user.id,
                    email: user.email,
                    token,
                    status: "Вход успешен!".into(),
                    errorCode: 0,
                }))
            } else {
                Ok(HttpResponse::Ok().json(LoginResponse {
                    id: 0,
                    email: String::new(),
                    token: String::new(),
                    status: "Неверное имя пользователя или пароль!".into(),
                    errorCode: 1,
                }))
            }
        }
        Err(_) => {
            User::mock_user_password();
            Ok(HttpResponse::Ok().json(LoginResponse {
                id: 0,
                email: String::new(),
                token: String::new(),
                status: "Неверное имя пользователя или пароль!".into(),
                errorCode: 1,
            }))
        }
    }
}

#[post("/register")]
async fn register(
    register: web::Json<RegisterRequest>,
    state: web::Data<State>,
) -> HttpResponse {
    let reg = register.into_inner().normalize();
    
    if let Err(_) = reg.validate(){
        return HttpResponse::Ok().json(RegisterResponse{
            errorCode : 4,
            status : "Неверные данные!".to_string(),
        });
    }
        
    if state.database.get_user_by_email(&reg.email).await.is_ok() {
        return HttpResponse::Ok().json(RegisterResponse {
            errorCode: 1,
            status: "Пользователь с такой электронной почтой уже существует!".into(),
        });
    }
    let user = state.database.create_user(&reg.email, &reg.password).await;
    match user {
        Ok(_) => HttpResponse::Ok().json(RegisterResponse {
            status: "Регистрация успешна!".into(),
            errorCode: 0,
        }),
        Err(_) => HttpResponse::Ok().json(RegisterResponse {
            errorCode: 2,
            status: "Внутренняя ошибка!".into(),
        }),
    }
}


#[get("/protected")]
async fn protected(token: AuthenticationToken) -> HttpResponse {
    log::debug!("{:?}", token);
    HttpResponse::Ok().json(Response {
        message: "Gotcha!".to_owned(),
    })
}

pub fn build_routes(cfg: &mut ServiceConfig) {
    cfg.service(login);
    cfg.service(register);
    cfg.service(protected);
}
