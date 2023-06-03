mod forms;
mod crypto;
mod stat;

pub use self::{
    forms::*,
    crypto::*,
    stat::*,
};
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    error::InternalError,
    http::StatusCode,
};
use crate::schema;
use serde::{Deserialize, Serialize};
use crate::models::{
    Categories,
    User,
    Cat,
    SmallTag,
    SmallFile,
    Tag,
    TechCategories,
    FeaturedItem,
};
use crate::diesel::{
    Connection,
    PgConnection,
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};

pub static TOKEN: &str = "111";

#[derive(Serialize)]
pub struct ErrorParams {
    pub error: String,
}
#[derive(Serialize)]
pub struct InfoParams {
    pub info: String,
}

#[derive(Deserialize)]
pub struct IsAjaxData {
    pub is_ajax: Option<i16>,
}

#[derive(Deserialize)]
pub struct PageStatData {
    pub view:    i32,
    pub height:  f64, 
    pub seconds: i32,
}

pub fn get_stat_page(types: i16, page: i32) -> PageStatData {
    if page > 1 {
        return PageStatData {
            view:    0,
            height:  0.0, 
            seconds: 0,
        };
    } 
    else {
        return stat_pages
            .filter(schema::stat_pages::types.eq(1))
            .select((
                schema::stat_pages::view,
                schema::stat_pages::height,
                schema::stat_pages::seconds,
            )) 
            .first::<PageStatData>(&_connection)
            .expect("Error.");
    }
}

#[derive(Serialize)]
pub struct UserResp {
    pub username:   String,
    pub image:      String,
    pub perm:       i16,
    pub device:     bool,
    pub categories: (),
}
#[derive(Serialize, Queryable)]
pub struct OwnerResp {
    pub first_name: String,
    pub last_name:  String,
    pub username:   String,
    pub image:      Option<String>,
    pub perm:       i16,
}


pub async fn get_request_user_id(req: &HttpRequest) -> i32 {
    use actix_web_httpauth::headers::authorization::{Authorization, Bearer};

    return match Authorization::<Bearer>::parse(req) {
        Ok(ok) => {
            let token = ok.as_ref().token().to_string();
            return match verify_jwt(token, "05uzefittt").await {
                Ok(ok) => ok.id,
                Err(_) => 0,
            }
        },
        Err(_) => 0,
    };
}
pub async fn get_request_user(req: &HttpRequest, is_ajax: i16) -> UserResp {
    if is_ajax > 0 {
        // не будем получать общие данные, если is_ajax > 0, 
        // так как эти данные уже были получены
        return UserResp {
            id:         0,
            username:   "".to_string(),
            image:      "".to_string(),
            perm:       0,
            device:     true,
            categories: (),
        };
    }
    let user_id = get_request_user_id(&req).await;
    if user_id > 0 {
        use crate::schema::users::dsl::users;

        let user = users
            .filter(schema::users::id.eq(user_id))
            .first::<User>(&_connection)
            .expect("E");
        
        User::create_superuser(user_id);
        return UserResp {
            id:         user.id,
            username:   user.username.clone(),
            image:      user.image.clone(),
            perm:       user.perm,
            device:     is_desctop(&req),
            categories: get_categories_2(is_ajax),
        };
    } 
    return UserResp {
        id:         0,
        username:   "".to_string(),
        image:      "".to_string(),
        perm:       0,
        device:     is_desctop(&req),
        categories: get_categories_2(is_ajax),
    };
}

pub fn get_price_acc_values(price: &i32) -> Option<i32> {
    if price > &3_000_000 {
        let acc = (price * 10) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &2_000_000 && price < &3_000_000 {
        let acc = (price * 7) / 100; // 10% скидка
        return Some(acc);
    }
    else if price > &1_000_000 && price < &2_000_000 {
        let acc = (price * 5) / 100; // 5% скидка
        return Some(acc);
    }
    else {
        return None;
    }
}

//lazy_static! {
    pub fn establish_connection() -> PgConnection {
        use dotenv::dotenv;

        dotenv().ok();
        let database_url = std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");

        PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url))
    }

    pub fn get_categories_2(is_ajax: i16) -> (
        Vec<&Cat>,
        Vec<&Cat>,
        Vec<&Cat>,
        Vec<&Cat>,
        Vec<&Cat>,
        Vec<&Cat>
    ) {
        if is_ajax == 0 {
            let _cats = Categories::get_categories().expect("E.");
            let mut _service_cats = Vec::new();
            let mut _store_cats = Vec::new();
            let mut _blog_cats = Vec::new();
            let mut _wiki_cats = Vec::new();
            let mut _work_cats = Vec::new();
            let mut _help_cats = Vec::new();

            for cat in _cats.iter() {
                match cat.types {
                    1 => _blog_cats.push(cat),
                    2 => _service_cats.push(cat),
                    3 => _store_cats.push(cat),
                    4 => _wiki_cats.push(cat),
                    5 => _work_cats.push(cat),
                    6 => _help_cats.push(cat),
                };
            }

            return (
                _service_cats,
                _store_cats,
                _blog_cats,
                _wiki_cats,
                _work_cats,
                _help_cats
            );
        }
        return (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
    }
//}

fn get_content_type<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("user-agent")?.to_str().ok();
}
pub fn is_desctop(req: &HttpRequest) -> bool {
    if get_content_type(req).unwrap().contains("Mobile") {
        return false;
    };
    return true;
}

fn get_cookie<'a>(req: &'a HttpRequest) -> Option<&'a str> {
    return req.headers().get("cookie")?.to_str().ok();
}

pub async fn get_cookie_user_id(req: &HttpRequest) -> i32 {
    let user_id: i32;
    let cookie_some = get_cookie(req);
    if cookie_some.is_none() {
        return 0;
    }
    let cookie = cookie_some.unwrap();
    let cookie_vec: Vec<&str> = cookie.split(";").collect();
    for c in cookie_vec.iter() {
        let split_c: Vec<&str> = c.split("=").collect();
        if split_c[0] == "user" {
            return split_c[1].parse().unwrap();
        }
    }
    return 0;
}


pub fn get_page(req: &HttpRequest) -> i32 {
    #[derive(Debug, Deserialize)]
    struct Params {
        pub page: Option<i32>,
    }
    let params_some = web::Query::<Params>::from_query(&req.query_string());
    let page: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        page = 1;
    }
    page
}

pub fn get_is_ajax(req: &HttpRequest) -> i16 {
    let params_some = web::Query::<IsAjaxData>::from_query(&req.query_string());
    let is_ajax: i16;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
            is_ajax = params.is_ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
    }
    else {
        is_ajax = 0;
    }
    return is_ajax;
}

#[derive(Deserialize)]
pub struct IsAjaxPageData {
    pub is_ajax: Option<i16>,
    pub page:    Option<i32>,
}
pub fn get_is_ajax_page(req: &HttpRequest) -> (i16, i32) {
    let params_some = web::Query::<IsAjaxPageData>::from_query(&req.query_string());
    let (is_ajax, page) : (i16, i32);
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.is_ajax.is_some() {
            is_ajax = params.is_ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
        if params.page.is_some() {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
    }
    else {
        (is_ajax, page) = (0, 1);
    }
    (is_ajax, page)
}

pub fn get_count_for_ru(count: i16, word1: String, word2: String, word3: String) -> String {
    let a = count % 10;
    let b = count % 100;
    let count_str: String = count.to_string().parse().unwrap();
    if a == 1 && b != 11 {
        return count_str + &word1;
    }
    else if a >= 2 && a <= 4 && (b < 10 || b >= 20) {
        return count_str + &word2;
    }
    else {
        return count_str + &word3;
    }
}
