
#![allow(non_snake_case,unused)]

use actix_web::{FromRequest, web::Json, error::ErrorUnauthorized,http};
use chrono::{Utc, Duration};
use serde::{Serialize,Deserialize};
use jsonwebtoken::{
    Header, encode, 
    EncodingKey,decode, 
    DecodingKey, Validation,
    Algorithm::HS256, TokenData
};
use std::future::{Ready, ready};

const JWT_SECRET : &[u8] = b"MY_super_SEcret_sicret";
#[derive(Deserialize,Serialize,Debug)]
pub struct Claims{
    pub sub : i32,
    pub exp : usize,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct EncodeResponse{
    pub message : String,
    pub token : String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct Response{
    pub message : String,
}
#[derive(Serialize,Deserialize,Debug)]
pub struct DecodeBody{
    token : String,
}

/// Создает новый JWT Auth Токен
///
/// 
///
/// # Пример
///
/// ```
/// let user_id = 10;
/// let token = encode_token(user_id);
/// ```
pub async fn encode_token(user_id : i32) -> String{
    let exp : usize = Utc::now().checked_add_signed(Duration::minutes(3)).unwrap().timestamp() as usize;
    let claims = Claims{
        sub : user_id,
        exp,
    };

    encode(&Header::default(), &claims, &EncodingKey::from_secret(JWT_SECRET))
        .unwrap()
}

/// Расшифровывает JWT Токен
pub async fn decode_token(body : Json<DecodeBody>) -> Result<i32,String>{
    let decoded_token = decode::<Claims>(
        &body.token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::new(HS256)
    );

    match decoded_token{
        Ok(token) => Ok(token.claims.sub),
        Err(e) => Err(e.to_string())
    }

}

/// Структура, хранящая в себе произвольные данные из Auth Токена
#[derive(Serialize,Deserialize,Debug)]
pub struct AuthenticationToken{
    pub id : i32,
}


impl FromRequest for AuthenticationToken{
    type Error = actix_web::error::Error;
    type Future = Ready<Result<Self,Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        let auth_header = req.headers().get(http::header::AUTHORIZATION);
        if auth_header.is_none() { return ready(Err(ErrorUnauthorized("No authentication token sent!"))); }
        let auth_token : String = auth_header.unwrap().to_str().unwrap_or("").to_string();
        if auth_token.is_empty() { return  ready(Err(ErrorUnauthorized("Invalid auth token!")));}
        let decode = 
            decode::<Claims>(
                &auth_token, 
                &DecodingKey::from_secret(JWT_SECRET), 
                &Validation::new(HS256),
            );

        match decode {
            Ok(token) => ready(Ok(Self{ id : token.claims.sub})),

            Err(_) => ready(Err(ErrorUnauthorized("Unauthorized!")))
            
        }

    }
}