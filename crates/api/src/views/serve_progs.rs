use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};
use crate::models::User;
use std::borrow::BorrowMut;
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use crate::utils::{
    establish_connection,
    get_request_user, get_stat_page,
    ErrorParams, TOKEN, UserResp,
};
use crate::schema;
use crate::models::{
    ServeCategories,
    NewServeCategories,
    Serve,
    NewServe,
    TechCategories,
    NewTechCategories,
};
use actix_multipart::{Field, Multipart};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::str;


pub fn serve_routes(config: &mut web::ServiceConfig) {
    config.route("/serve", web::get().to(get_serve_page));
    config.route("/serve_categories", web::get().to(serve_categories_page));

    config.service(web::resource("/create_tech_categories")
        .route(web::get().to(create_tech_categories_page))
        .route(web::post().to(create_tech_categories))
    );
    config.route("/load_serve_categories_from_level", web::get().to(load_serve_categories_from_level));
    config.route("/load_form_from_level", web::get().to(load_form_from_level));
    config.service(web::resource("/create_serve_categories")
        .route(web::get().to(create_serve_categories_page))
        .route(web::post().to(create_serve_categories))
    );
    config.service(web::resource("/edit_tech_category")
        .route(web::get().to(edit_tech_category_page))
        .route(web::post().to(edit_tech_category))
    );
    config.service(web::resource("/edit_serve_category")
        .route(web::get().to(edit_serve_category_page))
        .route(web::post().to(edit_serve_category))
    );

    config.service(web::resource("/create_serve")
        .route(web::get().to(create_serve_page))
        .route(web::post().to(create_serve))
    );
    config.service(web::resource("/edit_serve")
        .route(web::get().to(edit_serve_page))
        .route(web::post().to(edit_serve))
    );
    config.route("/delete_serve", web::post().to(delete_serve));
    config.route("/delete_serve_category", web::post().to(delete_serve_category));
    config.route("/delete_tech_category", web::post().to(delete_tech_category));
}


#[derive(Serialize)]
pub struct ServeCategoriesPageResp {
    pub request_user: UserResp,
    pub serve_cats:   Vec<ServeCategories>,
    pub view:         i32,
    pub height:       f64, 
    pub seconds:      i32,
}
pub async fn serve_categories_page(req: HttpRequest) -> Result<Json<ServeCategoriesPageResp>, Error> {
    let _connection = establish_connection();
    let _stat = get_stat_page(111, 0); 

    let _serve_cats = schema::serve_categories::table
        .load::<ServeCategories>(&_connection)
        .expect("E");

    return Ok(Json(ServeCategoriesPageResp {
        request_user:  get_request_user(&req, get_is_ajax(&req)),
        serve_cats:    _serve_cats,
        view:          _stat.view,
        height:        _stat.height, 
        seconds:       _stat.seconds,
    }));
}

#[derive(Deserialize)]
pub struct ServePageData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
#[derive(Serialize)]
pub struct ServePageResp {
    pub request_user: UserResp,
    pub category:     ServeCategories,
    pub object:       Serve,
    pub view:         i32,
    pub height:       f64, 
    pub seconds:      i32,
}
pub async fn get_serve_page(req: HttpRequest) -> Result<Json<ServePageResp>, Error> {
    let params_some = web::Query::<ServePageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() && params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }

    let _connection = establish_connection();
    let _serve = schema::serve::table
        .filter(schema::serve::id.eq(params.id.unwrap()))
        .first::<Serve>(&_connection)
        .expect("E");
    let _s_category = schema::serve_categories::table
        .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    return Ok(Json(ServePageResp {
        request_user: get_request_user(&req, is_ajax),
        category:     _s_category,
        object:       _serve,
        view:         _stat.view, 
        height:       _stat.height, 
        seconds:      _stat.seconds,
    }));
}

#[derive(Serialize)]
pub struct CreateTechCategoriesResp {
    pub request_user: UserResp,
    pub cats:         Vec<Cat>,
}
pub async fn create_tech_categories_page(req: HttpRequest) -> Result<Json<CreateTechCategoriesResp>, Error> {
    let _request_user = get_request_user(&req, get_is_ajax(&req));
    if _request_user.username.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _categories = tech_categories
        .load::<TechCategories>(&_connection)
        .expect("E");

    return Ok(Json(CreateTechCategoriesResp {
        request_user: _request_user,
        tech_cats:    _categories,
    }));
}

#[derive(Serialize)]
pub struct CreateServeCategoriesResp {
    pub request_user: UserResp,
    pub tech_cats:    Vec<TechCategories>,
}
pub async fn create_serve_categories_page(req: HttpRequest) -> Result<Json<CreateServeCategoriesResp>, Error> {
    let _request_user = get_request_user(&req, get_is_ajax(&req));
    if _request_user.username.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _categories = schema::tech_categories::table
        .load::<TechCategories>(&_connection)
        .expect("E");

    return Ok(Json(CreateServeCategoriesResp {
        request_user: _request_user,
        tech_cats:    _categories,
    }));
}


#[derive(Deserialize)]
pub struct LoadCategoriesData {
    pub level: Option<i16>,
}
#[derive(Serialize)]
pub struct LoadServeCategoriesResp {
    pub serve_cats: Vec<ServeCategories>,
}
pub async fn load_serve_categories_from_level(req: HttpRequest) -> Result<Json<LoadServeCategoriesResp>, Error> {
    let params_some = web::Query::<LoadCategoriesData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.level.is_none() && params.level.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'level' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    return Ok(Json(LoadServeCategoriesResp {
        serve_cats: ServeCategories::get_categories_from_level(params.level.unwrap()),
    }));
}

#[derive(Serialize)]
pub struct LoadFormCategoriesResp {
    pub tech_cats: Vec<TechCategories>,
}
pub async fn load_form_from_level(req: HttpRequest) -> Result<Json<LoadFormCategoriesResp>, Error> {
    let params_some = web::Query::<LoadCategoriesData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.level.is_none() && params.level.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'level' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _tech_categories = schema::tech_categories::table
        .filter(schema::tech_categories::level.eq(params.level.unwrap()))
        .order(schema::tech_categories::position.desc())
        .load::<TechCategories>(&_connection)
        .expect("E");

    return Ok(Json(LoadServeCategoriesResp {
        tech_cats: _tech_categories,
    }));
}

#[derive(Serialize)]
pub struct CreateServeCategoryResp {
    pub request_user: UserResp,
    pub tech_cats:    Vec<TechCategories>,
}
pub async fn create_serve_page(req: HttpRequest) -> Result<Json<CreateServeCategoryResp>, Error> {
    let _request_user = get_request_user(&req, get_is_ajax(&req));
    if _request_user.username.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _categories = schema::tech_categories::table
        .load::<TechCategories>(&_connection)
        .expect("E");

    return Ok(Json(CreateServeCategoryResp {
        request_user: _request_user,
        tech_cats:    _categories,
    }));
}


#[derive(Serialize)]
pub struct EditTechCategoryResp {
    pub request_user: UserResp,
    pub category:     TechCategories,
    pub tech_cats:    Vec<TechCategories>,
}
#[derive(Deserialize)]
pub struct EditItemData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
pub async fn edit_tech_category_page(req: HttpRequest) -> Result<Json<CreateServeCategoryResp>, Error> {
    let params_some = web::Query::<EditItemData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() && params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let _request_user = get_request_user(&req, is_ajax);
    let _connection = establish_connection();

    let _tech_categories = tech_categories
        .load::<TechCategories>(&_connection)
        .expect("E");

    let _category = schema::tech_categories::table
        .filter(schema::tech_categories::id.eq(params.id.unwrap()))
        .first::<TechCategories>(&_connection)
        .expect("E");

    if _request_user.username.is_empty() && _category.user_id != _request_user.id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    return Ok(Json(EditTechCategoryResp {
        request_user: _request_user,
        category:     _category,
        tech_cats:    _tech_categories,
    }));
}

#[derive(Serialize)]
pub struct EditServeCategoryResp {
    pub request_user: UserResp,
    pub category:     ServeCategories,
    pub tech_cats:    Vec<TechCategories>,
}
pub async fn edit_serve_category_page(req: HttpRequest) -> Result<Json<CreateServeCategoryResp>, Error> {
    let params_some = web::Query::<EditItemData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() && params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let _request_user = get_request_user(&req, is_ajax);
    let _connection = establish_connection();

    let _tech_categories = tech_categories
        .load::<TechCategories>(&_connection)
        .expect("E");

    let _category = schema::serve_categories::table
        .filter(schema::serve_categories::id.eq(params.id.unwrap()))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if _request_user.username.is_empty() && _category.user_id != _request_user.id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    return Ok(Json(EditServeCategoryResp {
        request_user: _request_user,
        category:     _category,
        tech_cats:    _tech_categories,
    }));
}


#[derive(Serialize)]
pub struct EditServeResp {
    pub request_user: UserResp,
    pub object:       Serve,
    pub serve_cats:   Vec<ServeCategories>,
    pub level:        i16,
}
pub async fn edit_serve_page(req: HttpRequest) -> Result<Json<CreateServeCategoryResp>, Error> {
    let params_some = web::Query::<EditItemData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() && params.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let is_ajax: i16;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    let _request_user = get_request_user(&req, is_ajax);
    let _connection = establish_connection();

    let _serve = serve
        .filter(schema::serve::id.eq(params.id.unwrap()))
        .first::<Serve>(&_connection)
        .expect("E");

    let _serve_cat = serve_categories
        .filter(schema::serve_categories::id.eq(&_serve.serve_categories))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    let _level = tech_categories
        .filter(schema::tech_categories::id.eq(_serve_cat.tech_categories))
        .select(schema::tech_categories::level)
        .first::<i16>(&_connection)
        .expect("E.");

    let _serve_cats = ServeCategories::get_categories_from_level(_level);

    if _request_user.username.is_empty() && _serve.user_id != _request_user.id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    return Ok(Json(EditServeResp {
        request_user: _request_user,
        object:       _serve,
        serve_cats:   _serve_cats,
        level:        _level,
    }));
}


pub async fn create_tech_categories(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::category_form;

    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut(), user_id).await;

    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let new_cat = NewTechCategories {
        name:        form.name.clone(),
        description: Some(form.description.clone()),
        position:    form.position,
        count:       0,
        level:       form.level,
        user_id:     user_id,
        view:        0,
        height:      0.0,
        seconds:     0,
    };
    let _new_tech = diesel::insert_into(schema::tech_categories::table)
        .values(&new_cat)
        .execute(&_connection)
        .expect("E.");
    
    return Json(1);
}


pub async fn create_serve_categories(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::category_form;
    
    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let form = category_form(payload.borrow_mut(), user_id).await;

    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let new_cat = NewServeCategories {
        name: form.name.clone(),
        description:     Some(form.description.clone()),
        tech_categories: form.tech_categories,
        position:        form.position,
        count:           0,
        default_price:   0,
        user_id:         user_id,
        view:            0,
        height:          0.0,
        seconds:         0,
    };
    let _new_serve = diesel::insert_into(schema::serve_categories::table)
        .values(&new_cat)
        .execute(&_connection)
        .expect("E.");
    
    return Json(1);
}

pub async fn edit_tech_category(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::category_form;

    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = category_form(payload.borrow_mut(), user_id).await;
    let _connection = establish_connection();
    if form.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _category = tech_categories::tech_categories::table
        .filter(schema::tech_categories::id.eq(form.id))
        .first::<TechCategories>(&_connection)
        .expect("E");
    
    if _category.user_id != user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let edit_cat = crate::models::EditTechCategories {
        name:        form.name.clone(),
        description: Some(form.description.clone()),
        position:    form.position,
        level:       form.level,
    };
    diesel::update(&_category)
        .set(edit_cat)
        .execute(&_connection)
        .expect("E");

    return Json(1);
}

pub async fn edit_serve_category(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::serve_category_form;

    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = serve_category_form(payload.borrow_mut(), user_id).await;
    let _connection = establish_connection();
    if form.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let s_category = serve_categories::serve_categories::table
        .filter(schema::serve_categories::id.eq(form.id))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if s_category.user_id != user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let new_cat = crate::models::EditServeCategories {
        name:        form.name.clone(),
        description: Some(form.description.clone()),
        position:    form.position,
        
    };
    diesel::update(&s_category)
        .set(new_cat)
        .execute(&_connection)
        .expect("E");

    return Json(1);
}

#[derive(Deserialize, Serialize, Debug)]
pub struct ServeForm {
    pub token:            String,
    pub id:               i32,
    pub name:             String,
    pub description:      String,
    pub position:         i16,
    pub serve_categories: i32,
    pub price:            i32,
    pub man_hours:        i16,
    pub is_default:       bool,
    pub serve_id:         Option<i32>,
}

pub async fn serve_split_payload(payload: &mut Multipart) -> ServeForm {
    let mut form: ServeForm = ServeForm {
        token:            "".to_string(),
        id:               0,
        name:             "".to_string(),
        description:      "".to_string(),
        position:         0,
        serve_categories: 0,
        price:            0,
        man_hours:        0,
        is_default:       true,
        serve_id:         None,
    };

    while let Some(item) = payload.next().await {
        let mut field: Field = item.expect("split_payload err");
        let name = field.name();

        if name == "id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.id = _int;
                }
            }
        }
        else if name == "position" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.position = _int;
                }
            }
        }
        else if name == "serve_categories" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_categories = _int;
                }
            }
        }
        else if name == "serve_id" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.serve_id = Some(_int);
                }
            }
        }
        else if name == "price" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i32 = s.parse().unwrap();
                    form.price = _int;
                }
            }
        }
        else if name == "man_hours" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let _int: i16 = s.parse().unwrap();
                    form.man_hours = _int;
                }
            }
        }
        else if name == "is_default" {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    form.is_default = s.to_string() == "on";
                }
            }
        }
        else {
            while let Some(chunk) = field.next().await {
                let data = chunk.expect("split_payload err chunk");
                if let Ok(s) = str::from_utf8(&data) {
                    let data_string = s.to_string();
                    if field.name() == "name" {
                        form.name = data_string;
                    } else if field.name() == "description" {
                        form.description = data_string;
                    } else if field.name() == "token" {
                        form.token = data_string;
                    }
                }
            }
        }
    }
    form
}

pub async fn create_serve(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    use crate::utils::serve_category_form;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = serve_split_payload(payload.borrow_mut(), user_id).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _cat_id = form.serve_categories;
    let _category = serve_categories
        .filter(schema::serve_categories::id.eq(_cat_id))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if _category.user_id != user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _new_serve = NewServe {
        name:             form.name.clone(),
        description:      Some(form.description.clone()),
        position:         form.position,
        serve_categories: _cat_id,
        price:            form.price,
        man_hours:        form.man_hours,
        is_default:       form.is_default,
        user_id:          _request_user.id,
        tech_cat_id:      _category.tech_categories,
        height:           0.0,
        seconds:          0,
        serve_id:         form.serve_id,
        view:             0,
    };

    let _serve = diesel::insert_into(schema::serve::table)
        .values(&_new_serve)
        .get_result::<Serve>(&_connection)
        .expect("E.");

    if form.is_default {
        diesel::update(&_category)
            .set(schema::serve_categories::default_price.eq(_category.default_price + _serve.price))
            .execute(&_connection)
            .expect("E.");
    }
    diesel::update(&_category)
        .set(schema::serve_categories::count.eq(_category.count + 1))
        .execute(&_connection)
        .expect("E.");
        
    return Json(1);
}

pub async fn edit_serve(req: HttpRequest, mut payload: Multipart) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let form = serve_split_payload(payload.borrow_mut()).await;
    if form.token != TOKEN.to_string() {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    if form.id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _serve = schema::serve::table
        .filter(schema::serve::id.eq(form.id))
        .first::<Serve>(&_connection)
        .expect("E");
    let _category = serve_categories::serve_categories::table
        .filter(schema::serve_categories::id.eq(_serve.serve_categories))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if _serve.user_id != user_id && _request_user.perm != 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let is_default = form.is_default;
    if _serve.is_default {
        // если опция дефолтная
        if !is_default {
            // если в форме галочка снята
            diesel::update(&_category)
                .set(schema::serve_categories::default_price
                    .eq(_category.default_price - _serve.price)
                )
                .execute(&_connection)
                .expect("E.");
            }
        }
    else {
        // если опция не дефолтная
        if is_default {
            // если в форме галочка поставлена
            diesel::update(&_category)
                .set(schema::serve_categories::default_price
                        .eq(_category.default_price + _serve.price)
                )
                .execute(&_connection)
                .expect("E.");
        }
    }

    let __serve = crate::models::EditServe {
        name:        form.name.clone(),
        description: Some(form.description.clone()),
        position:    form.position,
        price:       form.price,
        man_hours:   form.man_hours,
        is_default:  is_default,
        serve_id:    form.serve_id,
    };

    diesel::update(&_serve)
        .set(__serve)
        .execute(&_connection)
        .expect("E");

    return Json(1);
}

#[derive(Deserialize)]
pub struct DeleteServeData {
    pub token: String,
    pub id:    Option<i32>,
}
pub async fn delete_serve(req: HttpRequest, data: Json<DeleteServeData>) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "request user not authenticate!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _serve = schema::serve::table
        .filter(schema::serve::id.eq(data.id.unwrap()))
        .first::<Serve>(&_connection)
        .expect("E");
    if _request_user.perm != 60 && _serve.user_id != user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _category = schema::serve_categories::table
        .filter(schema::serve_categories::id.eq(_serve.serve_categories))
        .first::<ServeCategories>(&_connection)
        .expect("E");
    if _serve.is_default {
        diesel::update(&_category)
            .set((
                schema::serve_categories::count.eq(&_category.count - 1),
                schema::serve_categories::default_price.eq(&_category.default_price - _serve.price),
            ))
            .execute(&_connection)
            .expect("Error.");
    }
    else {
        diesel::update(&_category)
            .set(schema::serve_categories::count.eq(&_category.count - 1))
            .execute(&_connection)
            .expect("Error.");
    }

    diesel::delete(&_serve).execute(&_connection).expect("E");
    return Ok(Json(1));
}

pub async fn delete_tech_category(req: HttpRequest, data: Json<DeleteServeData>) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "request user not authenticate!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    let _connection = establish_connection();
    let _category = schema::tech_categories::table
        .filter(schema::tech_categories::id.eq(data.id.unwrap()))
        .first::<TechCategories>(&_connection)
        .expect("E");

    if _request_user.perm != 60 && _category.user_id != user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    diesel::delete(&_category).execute(&_connection).expect("E");
    return Ok(Json(1));
}

pub async fn delete_serve_category(req: HttpRequest, data: Json<DeleteServeData>) -> Result<Json<i16>, Error> {
    let _request_user = get_request_user(&req, 3);
    let user_id = _request_user.id;
    if user_id < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "request user not authenticate!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    if data.token.is_none() || data.token.as_deref().unwrap() != TOKEN {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    if data.id.is_none() || data.id.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    let _connection = establish_connection();

    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let s_category = schema::serve_categories::table
        .filter(schema::serve_categories::id.eq(data.id.unwrap()))
        .first::<ServeCategories>(&_connection)
        .expect("E");

    if _request_user.perm != 60 && s_category.user_id != user_id {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _category = tech_categories
        .filter(schema::tech_categories::id.eq(s_category.tech_categories))
        .first::<TechCategories>(&_connection)
        .expect("E");

    diesel::delete(&s_category).execute(&_connection).expect("E");
    diesel::update(&_category)
        .set(schema::tech_categories::count.eq(&_category.count - 1))
        .execute(&_connection)
        .expect("E");

    return Ok(Json(1));
}
