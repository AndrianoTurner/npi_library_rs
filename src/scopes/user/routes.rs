use actix_web::web::ServiceConfig;
use actix_web::{
    post,
    web, 
    HttpResponse,
    get
};
use crate::auth::{AuthenticationToken, Response};
use crate::models::request::NormalizeForm;
use crate::{models::request::{LoginRequest,ValidateForm,RegisterRequest}, State};
use crate::database::models::User;
use crate::auth;
#[post("/login")]
async fn login(login_info : web::Json<LoginRequest>, state: web::Data<State>) -> actix_web::Result<HttpResponse>{
    #[derive(serde::Serialize,serde::Deserialize)]
    struct Response{
        id : i32,
        email : String,
        token : String,
    }

    let login_info = login_info.into_inner();
    login_info.validate()?;
    match state.database.get_user_by_email(&login_info.email).await{
        
        Ok(user) => {
            if user.check_password(&login_info.password){
                let token = auth::encode_token(user.id).await;
                Ok(HttpResponse::Ok().json(Response {id : user.id, email : user.email,token}))
            }
            else{
                
                Ok(HttpResponse::Ok().json("Wrong email or password!"))
            }
            
        },
        Err(_) => { 
            User::mock_user_password();
            Ok(HttpResponse::Ok().json("Wrong email or password!"))
        },
    }
}

#[post("/register")]
async fn register(register : web::Json<RegisterRequest>, state : web::Data<State>) -> actix_web::Result<HttpResponse>{
    let reg = register.into_inner().normalize();
    reg.validate()?;
    match state.database.get_user_by_email(&reg.email).await{
        Ok(_) => return Ok(HttpResponse::BadRequest().json("Email is registered")),
        _ => ()
    };
    let user = state.database.create_user(&reg.email,&reg.password).await;
    match user {
        Ok(()) => Ok(HttpResponse::Ok().json("Successfully registered!")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(format!("Registration Error {}",e))),
    }
    
}

#[get("/delete/{user_id}")]
async fn delete(user_id : web::Path<i32>, state : web::Data<State>) -> actix_web::Result<HttpResponse>{
    log::debug!("/delete/{}",user_id);
    if state.database.get_user_by_id(*user_id).await.is_some(){
        state.database.delete_user_id(*user_id).await.unwrap();
        Ok(HttpResponse::Ok().json("User deleted!"))
    }
    else{
        Ok(HttpResponse::InternalServerError().json("User not found!"))
    }
}

#[get("/protected")]
async fn protected(token : AuthenticationToken) -> HttpResponse{
    log::debug!("{:?}",token);
    HttpResponse::Ok().json(Response{message : "Gotcha!".to_owned()})
}

pub fn build_routes(cfg : &mut ServiceConfig){
    cfg.service(login);
    cfg.service(register);
    cfg.service(delete);
    cfg.service(protected);
}