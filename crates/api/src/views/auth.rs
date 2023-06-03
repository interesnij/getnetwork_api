use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};
use serde::{Deserialize, Serialize};
use crate::utils::{
    establish_connection, is_desctop,
    gen_jwt, get_request_user_id, get_categories_2,
    ErrorParams, TOKEN, UserResp,
};
use bcrypt::{hash, verify};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::schema;
use futures::StreamExt;
use crate::models::{User, NewUser};
use crate::errors::Error;
use actix_multipart::{Field, Multipart};
use std::borrow::BorrowMut;
use crate::AppState;


pub fn auth_routes(config: &mut web::ServiceConfig) {
    config.route("/login", web::post().to(login));
    config.route("/signup", web::post().to(process_signup));
    config.route("/logout", web::get().to(logout));
}

pub async fn logout() -> HttpResponse {
    HttpResponse::Unauthorized().finish()
} 

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginUser2 {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct IncommingUserResp {
    pub username:   String,
    pub image:      String,
    pub perm:       i16,
    pub device:     bool,
    pub categories: (),
}

pub async fn login (
    req: HttpRequest,
    data: Json<LoginUser2>,
    state: web::Data<AppState>
) -> Result<Json<UserResp>, Error> {
    let _user = User::get_user_by_name(&data.username);
    
    if get_request_user_id(&req).await != 0 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if _user.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Пользователь с таким логином не найден".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.username.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'username' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'password' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let _user = _user.expect("E.");

        if verify(data.password.as_str(), _user.password.as_str()).unwrap() {
                let token = gen_jwt(_user.id, state.key.as_ref()).await;
                
                match token {
                    Ok(token_str) => {
                        Ok(Json(IncommingUserResp {
                            token:      token_str,
                            username:   _user.username.clone(),
                            image:      _user.image.clone(),
                            perm:       _user.perm,
                            device:     is_desctop(&req),
                            categories: get_categories_2(0),
                        }))
                    },
                    Err(err) => {
                        let body = serde_json::to_string(&ErrorParams {
                            error: err.to_string(),
                        }).unwrap();
                        Err(Error::BadRequest(body))
                    }
                }
        } else {
            let body = serde_json::to_string(&ErrorParams {
                error: "Пароль неверный!".to_string(),
            }).unwrap();
            Err(Error::BadRequest(body))
        }
    }
}

#[derive(Deserialize)]
pub struct NewUserForm {
    pub token:      Option<String>,
    pub first_name: Option<String>,
    pub last_name:  Option<String>,
    pub username:   Option<String>,
    pub email:      Option<String>,
    pub password:   Option<i16>, 
}

pub async fn process_signup (
    req:   HttpRequest,
    state: web::Data<AppState>,
    data:  Json<NewUserForm>
) -> Result<Json<UserResp>, Error> {
    if data.token.as_deref().unwrap() != TOKEN || get_request_user_id(&req) != 0 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.username.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'username' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.email.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'email' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else if data.password.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Field 'password' is required!".to_string(),
        }).unwrap();
        Err(Error::BadRequest(body))
    }
    else {
        let _connection = establish_connection();
        let form_user = NewUser {
            first_name: data.first_name.clone(),
            last_name:  data.last_name.clone(),
            username:   data.username.clone(),
            email:      data.email.clone(),
            password:   hash(data.password.as_deref().unwrap(), 8).unwrap(),
            bio:        None,
            image:      None,
            perm:       1,
            view:       0,
            height:     0.0,
            seconds:    0,
        };

        let _new_user = diesel::insert_into(schema::users::table)
            .values(&form_user)
            .get_result::<User>(&_connection)
            .expect("Error saving user.");
            
        let token = gen_jwt(_new_user.id, state.key.as_ref()).await;
        Ok(Json(IncommingUserResp {
            token:      token,
            username:   _new_user.username,
            image:      "".to_string(),
            perm:       0,
            device:     is_desctop(&req),
            categories: get_categories_2(0),
        }))
    }
}
