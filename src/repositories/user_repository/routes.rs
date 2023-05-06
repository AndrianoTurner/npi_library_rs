use actix_web::web::ServiceConfig;
use actix_web::{
    post,
    web, 
    HttpResponse,
    get
};
use crate::models::user::ValidateForm;
use crate::{models::user::{LoginInfo, Register}, State};
use crate::database::models::User;
#[post("/login")]
async fn login(login_info : web::Json<LoginInfo>, state: web::Data<State>) -> actix_web::Result<HttpResponse>{
    let login_info = login_info.into_inner();
    login_info.validate()?;
    match state.database.get_user_by_email(&login_info.email).await{
        Some(user) => {
            if user.check_password(&login_info.password){
                Ok(HttpResponse::Ok().json(user))
            }
            else{
                
                Ok(HttpResponse::Ok().json("Wrong email or password!"))
            }
            
        },
        None => { 
            User::mock_user_password();
            Ok(HttpResponse::Ok().json("Wrong email or password!"))
        },
    }
}

#[post("/register")]

async fn register(register : web::Json<Register>, state : web::Data<State>) -> actix_web::Result<HttpResponse>{
    let reg = register.into_inner();
    reg.validate()?;
    if let Some(_) = state.database.get_user_by_email(&reg.email).await{
        return Ok(HttpResponse::InternalServerError().json("Email is registered!"))
    }
    let user = state.database.create_user(&reg.email,&reg.password).await;
    match user {
        Ok(()) => Ok(HttpResponse::Ok().json("Successfully registered!")),
        Err(e) => Ok(HttpResponse::InternalServerError().json(e.to_string())),
    }
    
}

#[get("/delete/{user_id}")]
async fn delete(user_id : web::Path<i32>, state : web::Data<State>) -> actix_web::Result<HttpResponse>{
    log::debug!("/delete/{}",user_id);
    if let Some(_) = state.database.get_user_by_id(*user_id).await{
        state.database.delete_user_id(*user_id).await;
        Ok(HttpResponse::Ok().json("User deleted!"))
    }
    else{
        Ok(HttpResponse::InternalServerError().json("User not found!"))
    }
}

pub fn build_routes(cfg : &mut ServiceConfig){
    cfg.service(login);
    cfg.service(register);
    cfg.service(delete);
}