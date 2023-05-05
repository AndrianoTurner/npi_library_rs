use actix_web::{
    post,
    get, web, HttpResponse,
};
use actix_web::error;
use crate::{models::user::{LoginInfo, Register}, State, database::models::NewUser, schema::user_table::password};
#[get("/login")]
async fn login(user : web::Json<LoginInfo>, state: web::Data<State>) -> HttpResponse{
    return HttpResponse::Ok().body("123");
}

#[get("/register")]

async fn register(register : web::Json<Register>, state : web::Data<State>) -> actix_web::Result<HttpResponse>{
    let email = &register.email;
    let pass = &register.password;
    let user = NewUser::new(email,pass);

    let user = web::block(move || {
        state.database.create_user(user)
    }).await?
    .map_err(error::ErrorInternalServerError)?;

    Ok(HttpResponse::Ok().json(user))
    
}
