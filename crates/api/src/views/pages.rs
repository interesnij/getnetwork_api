use actix::Addr;
use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
    Result,
};
use crate::schema;
use crate::models::{
    User, Item, Categories, Feedback, CookieUser,
    Tag, StatPage, Cat, SmallTag, CatDetail, Serve,
    Blog, Service, Store, Wiki, Work, ContentBlock,
    ServeCategories, TechCategories, CookieStat,
    SmallFile, File,
};
use crate::utils::{
    establish_connection, get_request_user, is_desctop,
    get_categories_2, get_stat_page, get_is_ajax_page, 
    get_is_ajax, get_page,
    IsAjaxData, IsAjaxPageData, IndexResponse, ErrorParams, 
    TOKEN, UserResp, PageStatData,  OwnerResp,
};
use crate::diesel::{
    RunQueryDsl,
    ExpressionMethods,
    QueryDsl,
};
use actix_web::dev::ConnectionInfo;
use serde_json::to_value;
use serde::{Deserialize, Serialize};
use crate::errors::Error;


pub fn pages_routes(config: &mut web::ServiceConfig) {
    config.route("/", web::get().to(index_page));
    config.route("/info", web::get().to(info_page));
    config.route("/history", web::get().to(history_page));
    config.route("/feedback_list", web::get().to(feedback_list_page));
    config.route("/serve_list", web::get().to(serve_list_page));
    config.route("/cookie_users_list", web::get().to(cookie_users_list_page));

    config.route("/load_tech_category", web::get().to(get_tech_category_page));
    config.route("/load_serve_category", web::get().to(get_serve_category_page));
    config.route("/load_serve", web::get().to(get_serve_page));
    config.route("/load_user_history", web::get().to(get_user_history_page));
    config.route("/load_tech_objects", web::get().to(get_tech_objects_page));
    config.route("/unical_object_form", web::get().to(unical_object_form_page));

    config.route("/create_category", web::get().to(create_category_page));
    config.route("/edit_category", web::get().to(edit_category_page));
    config.route("/create_item", web::get().to(create_item_page));
    config.route("/edit_item", web::get().to(edit_item_page));

    config.route("/edit_file", web::get().to(edit_file_page));
    config.route("/image", web::get().to(image_page));

    config.route("/blog", web::get().to(get_blog_page));
    config.route("/help", web::get().to(get_help_page));
    config.route("/service", web::get().to(get_service_page));
    config.route("/store", web::get().to(get_store_page));
    config.route("/wiki", web::get().to(get_wiki_page));
    config.route("/work", web::get().to(get_work_page));

    config.route("/blogs", web::get().to(blog_category_page));
    config.route("/helps", web::get().to(help_category_page));
    config.route("/services", web::get().to(service_category_page));
    config.route("/stores", web::get().to(store_category_page));
    config.route("/wikis", web::get().to(wiki_category_page));
    config.route("/works", web::get().to(work_category_page));

    config.route("/blog_categories", web::get().to(blog_categories_page));
    config.route("/help_categories", web::get().to(help_categories_page));
    config.route("/service_categories", web::get().to(service_categories_page));
    config.route("/store_categories", web::get().to(store_categories_page));
    config.route("/wiki_categories", web::get().to(wiki_categories_page));
    config.route("/work_categories", web::get().to(work_categories_page));
}


#[derive(Serialize)]
pub struct IndexPageResp {
    pub request_user:  UserResp,
    pub last_works:    Vec<Work>,
    pub last_services: Vec<Service>,
    pub last_wikis:    Vec<Wiki>,
    pub last_blogs:    Vec<Blog>,
    pub last_stores:   Vec<Store>,
    pub view:          i32,
    pub height:        f64, 
    pub seconds:       i32,
}
pub async fn index_page(req: HttpRequest) -> Result<Json<IndexPageResp>, Error> {

    let _stat = get_stat_page(1, 0); 
 
    let _request_user = get_request_user(&req, get_is_ajax(&req)).await;
    let is_superuser = _request_user.perm > 59;

    return Ok(Json(IndexPageResp {
        request_user:  _request_user,
        last_works:    Item::get_works(3, 0, is_superuser),
        last_services: Item::get_services(3, 0, is_superuser),
        last_wikis:    Item::get_wikis(3, 0, is_superuser),
        last_blogs:    Item::get_blogs(3, 0, is_superuser),
        last_stores:   Item::get_stores(3, 0, is_superuser),
        view:          _stat.view,
        height:        _stat.height, 
        seconds:       _stat.seconds,
    }));
}

#[derive(Serialize)]
pub struct AboutPageResp {
    pub request_user: UserResp,
    pub help_cats:    Vec<Cat>,
    pub view:         i32,
    pub height:       f64, 
    pub seconds:      i32,
} 
pub async fn info_page(req: HttpRequest) -> Result<Json<AboutPageResp>, Error> {
    use crate::utils::get_is_ajax;

    let _connection = establish_connection();
    let _stat = get_stat_page(2, 0);

    let _help_cats: Vec<Cat>;
    let cats_res = block(move || Categories::get_categories_for_types(6)).await?;
    let _help_cats = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };
    return Ok(Json(AboutPageResp {
        request_user: get_request_user(&req, get_is_ajax(&req)).await,
        help_cats:    _help_cats,
        view:         _stat.view,
        height:       _stat.height, 
        seconds:      _stat.seconds,
    }));
}


#[derive(Serialize)]
pub struct HistoryPageResp {
    pub request_user:     UserResp,
    pub object_list:      Vec<CookieStat>,
    pub next_page_number: i16,
}
pub async fn history_page(req: HttpRequest) -> Result<Json<HistoryPageResp>, Error> {
    use schema::cookie_users::dsl::cookie_users;
    use crate::models::{CookieUser, CookieStat};
    use crate::utils::{get_is_ajax_page, get_or_create_cookie_user_id};

    let _connection = establish_connection();
    let user_id = get_or_create_cookie_user_id(_connection, &req).await;
        
    let _cookie_user = cookie_users
        .filter(schema::cookie_users::id.eq(&user_id))
        .first::<CookieUser>(&_connection)
        .expect("Error");

    let object_list: Vec<CookieStat>;
    let next_page_number: i32;
    let (is_ajax, page) = get_is_ajax_page(&req);
    
    let _res = block(move || CookieStat::get_stat_list(user_id, page, 20)).await?;
    let _dict = match _res {
        Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
        Err(_error) => {object_list = Vec::new(); next_page_number = 0},
    };

    return Ok(Json(HistoryPageResp {
        request_user:     get_request_user(&req, is_ajax).await,
        object_list:      object_list,
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct FeedbackListResp {
    pub request_user:     UserResp,
    pub feedback_list:    Vec<Feedback>,
    pub next_page_number: i16,
}
pub async fn feedback_list_page(req: HttpRequest) -> Result<Json<FeedbackListResp>, Error> {
    let (is_ajax, page) = get_is_ajax_page(&req);
    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm > 59 {
        use crate::utils::get_is_ajax_page;

        let object_list: Vec<Feedback>;
        let next_page_number: i32;
        let _res = block(move || Feedback::get_list(page, 20)).await?;
        let _dict = match _res {
            Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
            Err(_error) => {object_list = Vec::new(); next_page_number = 0},
        };

        return Ok(Json(FeedbackListResp {
            request_user:     _request_user,
            feedback_list:    object_list,
            next_page_number: next_page_number,
        }));
    }
    else {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
}

#[derive(Serialize)]
pub struct ServeListResp {
    pub request_user: UserResp,
    pub tech_cats:    Vec<TechCategories>,
}
pub async fn serve_list_page(req: HttpRequest) -> Result<Json<ServeListResp>, Error> {
    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;
    use crate::utils::get_is_ajax;

    let _connection = establish_connection();
    let tech_cats = tech_categories
        .order(schema::tech_categories::level.asc())
        .load::<TechCategories>(&_connection)
        .expect("E.");

    return Ok(Json(ServeListResp {
        request_user: get_request_user(&req, get_is_ajax(&req)).await,
        tech_cats:    tech_cats,
    }));
}

#[derive(Serialize)]
pub struct TechCategoryResp {
    pub request_user: UserResp,
    pub object:       TechCategories,
}
#[derive(Deserialize)]
pub struct TechCategoryData {
    pub is_ajax: Option<i16>,
    pub id:      Option<i32>,
}
pub async fn get_tech_category_page(req: HttpRequest) -> Result<Json<TechCategoryResp>, Error> {
    let params_some = web::Query::<TechCategoryData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let _connection = establish_connection();
    let tech_category = tech_categories
        .filter(schema::tech_categories::id.eq(params.id.unwrap()))
        .first::<TechCategories>(&_connection)
        .expect("E."); 

    return Ok(Json(TechCategoryResp {
        request_user: get_request_user(&req, 2).await,
        object:       tech_category,
    }));
}

#[derive(Serialize)]
pub struct ServeCategoryResp {
    pub request_user: UserResp,
    pub object:       ServeCategories,
}
#[derive(Deserialize)]
pub struct ServeCategoryData {
    pub is_ajax: Option<i16>,
    pub id:      Option<i32>,
}
pub async fn get_serve_category_page(req: HttpRequest) -> Result<Json<ServeCategoryResp>, Error> {
    let params_some = web::Query::<ServeCategoryData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::models::ServeCategories;
    use crate::schema::serve_categories::dsl::serve_categories;

    let _connection = establish_connection();
    let serve_category = serve_categories
        .filter(schema::serve_categories::id.eq(params.id.unwrap()))
        .first::<ServeCategories>(&_connection)
        .expect("E."); 

    return Ok(Json(ServeCategoryResp {
        request_user: get_request_user(&req, 2).await,
        object:       serve_category,
    }));
}

#[derive(Serialize)]
pub struct ServeResp {
    pub request_user: UserResp,
    pub object:       Serve,
}
#[derive(Deserialize)]
pub struct ServeData {
    pub is_ajax: Option<i16>,
    pub id:      Option<i32>,
}
pub async fn get_serve_page(req: HttpRequest) -> Result<Json<ServeResp>, Error> {
    let params_some = web::Query::<ServeData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::models::Serve;
    use crate::schema::serve::dsl::serve;

    let _connection = establish_connection();
    let serve = serve
        .filter(schema::serve::id.eq(params.id.unwrap()))
        .first::<Serve>(&_connection)
        .expect("E."); 

    return Ok(Json(ServeResp {
        request_user: get_request_user(&req, 2).await,
        object:       serve,
    }));
}

#[derive(Serialize)]
pub struct CookieUsersResp {
    pub request_user:     UserResp,
    pub object_list:      Vec<CookieUser>,
    pub next_page_number: i16,
}
pub async fn cookie_users_list_page(req: HttpRequest) -> Result<Json<CookieUsersResp>, Error> {
    use crate::utils::get_is_ajax_page;
    use crate::models::CookieUser;

    let (is_ajax, page) = get_is_ajax_page(&req);
    let _connection = establish_connection();

    let (object_list, next_page_number) = CookieUser::get_users_list(page, 20);
    
    return Ok(Json(CookieUsersResp {
        request_user:     get_request_user(&req, is_ajax).await,
        object_list:      object_list,
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct UserHistoryResp {
    pub request_user:     UserResp,
    pub object_list:      Vec<CookieUser>,
    pub next_page_number: i16,
}
#[derive(Deserialize)]
pub struct UserHistoryData {
    pub user_id: Option<i32>,
    pub page:    Option<i32>,
}
pub async fn get_user_history_page(req: HttpRequest) -> Result<Json<UserHistoryResp>, Error> {
    let page = get_page(&req);
    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permisson Denied!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params_some = web::Query::<UserHistoryData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.user_id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'user_id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::models::CookieStat;

    let object_list: Vec<CookieStat>;
    let next_page_number: i32;
    let _res = block(move || CookieStat::get_stat_list(params.user_id.unwrap(), page, 20)).await?;
    let _dict = match _res {
        Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
        Err(_error) => {object_list = Vec::new(); next_page_number = 0},
    };

    return Ok(Json(UserHistoryResp {
        request_user:     _request_user,
        object_list:      object_list,
        next_page_number: next_page_number,
    }));
}

#[derive(Serialize)]
pub struct TechObjectsResp {
    pub request_user: UserResp,
    pub object:       TechCategories,
    pub is_admin:     bool,
}
#[derive(Deserialize)]
pub struct TechObjectsData {
    pub id: Option<i32>,
}
pub async fn get_tech_objects_page(req: HttpRequest) -> Result<Json<TechObjectsResp>, Error> {
    let params_some = web::Query::<TechObjectsData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.id.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'id' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    use crate::models::TechCategories;
    use crate::schema::tech_categories::dsl::tech_categories;

    let is_admin = get_request_user(&req, 2).await.perm > 59;
    let _connection = establish_connection();
    let _cat = tech_categories
        .filter(schema::tech_categories::id.eq(params.id.unwrap()))
        .first::<TechCategories>(&_connection)
        .expect("E.");
    
    return Ok(Json(TechObjectsResp {
        request_user: _request_user,
        object:       _cat,
        is_admin:     _request_user.perm > 59,
    }));
}

#[derive(Serialize)]
pub struct UnicalObjectFormResp {
    pub request_user: UserResp,
    pub cats:         Vec<Cat>,
    pub biznes_mode:  bool,
}
#[derive(Deserialize)]
pub struct UnicalObjectFormData {
    pub types: Option<i16>,
}
pub async fn unical_object_form_page(req: HttpRequest) -> Result<Json<UnicalObjectFormResp>, Error> {
    let _request_user = get_request_user(&req, 2).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    let params_some = web::Query::<UnicalObjectFormData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.types.is_none() && params.types.unwrap() < 1 {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'types' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    let _connection = establish_connection();
    let types = params.types.unwrap();
    let _cats: Vec<Cat>;
    let cats_res = block(move || Categories::get_categories_for_types(types)).await?;
    let _cats = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };

    return Ok(Json(UnicalObjectFormResp {
        request_user: _request_user,
        cats:         _cats,
        biznes_mode:  vec![2,3,5].iter().any(|i| i==&types),
    }));
}

#[derive(Serialize)]
pub struct CreateCategoryResp {
    pub request_user: UserResp,
    pub cats:         Vec<Categories>,
}
pub async fn create_category_page(req: HttpRequest) -> Result<Json<CreateCategoryResp>, Error> {
    let _request_user = get_request_user(&req, get_is_ajax(&req)).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _cats = schema::categories::table
        .load::<Categories>(&_connection)
        .expect("Error");
    
    return Ok(Json(CreateCategoryResp {
        request_user: _request_user,
        cats:         _cats,
    }));
}


#[derive(Serialize)]
pub struct EditCategoryResp {
    pub request_user: UserResp,
    pub cat:          Categories,
    pub cats:         Vec<Categories>,
}
#[derive(Deserialize)]
pub struct EditCategoryData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
pub async fn edit_category_page(req: HttpRequest) -> Result<Json<EditCategoryResp>, Error> {
    let params_some = web::Query::<EditCategoryData>::from_query(&req.query_string());
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
    
    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _cats = schema::categories::table
        .load::<Categories>(&_connection)
        .expect("E");
    let _cat = schema::categories::table
        .filter(schema::categories::id.eq(params.id.unwrap()))
        .load::<Categories>(&_connection)
        .expect("E");

    return Ok(Json(EditCategoryResp {
        request_user: _request_user,
        cat:          _cat,
        cats:         _cats,
    }));
}

#[derive(Serialize)]
pub struct CreateItemResp {
    pub request_user: UserResp,
    pub all_tags:     Vec<Tag>,
}
pub async fn create_item_page(req: HttpRequest) -> Result<Json<CreateItemResp>, Error> {
    let is_ajax = get_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let all_tags = schema::tags::table
        .load::<Tag>(&_connection)
        .expect("Error.");
    
    return Ok(Json(CreateItemResp {
        request_user: _request_user,
        all_tags:     all_tags,
    }));
}

#[derive(Serialize)]
pub struct EditItemResp {
    pub request_user: UserResp,
    pub object:       Item,
    pub cats:         Vec<Categories>,
}
#[derive(Deserialize)]
pub struct EditItemData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
pub async fn edit_item_page(req: HttpRequest) -> Result<Json<EditItemResp>, Error> {
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

    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();
    let _cats = schema::categories::table
        .load::<Categories>(&_connection)
        .expect("E");
    let object = schema::items::table
        .filter(schema::items::id.eq(params.id.unwrap()))
        .load::<Item>(&_connection)
        .expect("E");

    return Ok(Json(EditItemResp {
        request_user: _request_user,
        object:       object,
        cats:         _cats,
    }));
}

#[derive(Serialize)]
pub struct EditFileResp {
    pub request_user: UserResp,
    pub file:         File,
}
#[derive(Deserialize)]
pub struct EditFileData {
    pub id:      Option<i32>,
    pub is_ajax: Option<i16>,
}
pub async fn edit_file_page(req: HttpRequest) -> Result<Json<EditFileResp>, Error> {
    let params_some = web::Query::<EditFileData>::from_query(&req.query_string());
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

    let _request_user = get_request_user(&req, is_ajax).await;
    if _request_user.perm < 60 {
        let body = serde_json::to_string(&ErrorParams {
            error: "Permission Denied".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    use crate::schema::files::dsl::files;
    use crate::models::File;

    let _connection = establish_connection();
    let _file = files
        .filter(schema::files::id.eq(params.id.unwrap()))
        .first::<File>(&_connection)
        .expect("E");

    return Ok(Json(EditFileResp {
        request_user: _request_user,
        file:         _file,
    }));
}


#[derive(Serialize)]
pub struct ImageResp {
    pub item: Item,
    pub prev: Option<File>,
    pub next: Option<File>,
}
#[derive(Deserialize)]
pub struct ImageData {
    pub id: Option<i32>,
}
pub async fn image_page(req: HttpRequest) -> Result<Json<ImageResp>, Error> {
    use crate::schema::{
        files::dsl::files,
        items::dsl::items,
    };
    use crate::models::File;

    let params_some = web::Query::<ImageData>::from_query(&req.query_string());
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

    let _id = params.id.unwrap();
    let _connection = establish_connection();
    let _file = files
        .filter(schema::files::id.eq(_id))
        .first::<File>(&_connection)
        .expect("E.");

    let _item = items
        .filter(schema::items::id.eq(_file.item_id))
        .filter(schema::items::types.eq(_file.item_types))
        .first::<Item>(&_connection)
        .expect("E.");

    let _images = _item.get_images_ids();
    let _images_len = _images.len();
    let mut prev: Option<File> = None;
    let mut next: Option<File> = None;

    for (i, obj) in _images.iter().enumerate().rev() {
        if obj == &_id {
            if (i + 1) != _images_len {
                let _next = Some(&_images[i + 1]);
                next = Some(files
                    .filter(schema::files::id.eq(_next.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            if i != 0 {
                let _prev = Some(&_images[i - 1]);
                prev = Some(files
                    .filter(schema::files::id.eq(_prev.unwrap()))
                    .filter(schema::files::types.eq(_item.types))
                    .first::<File>(&_connection)
                    .expect("E"));
            };
            break;
        }
    };

    return Ok(Json(ImageResp {
        item: _item,
        prev: prev,
        next: next,
    }));
}


#[derive(Serialize)]
pub struct ItemFilesResp {
    pub photos: Vec<SmallFile>,
    pub videos: Vec<SmallFile>,
    pub audios: Vec<SmallFile>,
    pub docs:   Vec<SmallFile>,
}
#[derive(Serialize)]
pub struct ItemServeResp {
    pub open_tech_cats:  Vec<TechCategories>,
    pub close_tech_cats: Vec<TechCategories>,
    pub serve_ids:       Vec<i32>,
}

#[derive(Serialize)]
pub struct ObjectPageResp {
    pub request_user: UserResp,
    pub object:       ItemDetailResp,
    pub category:     Cat,
    pub cats:         Vec<Cat>,
    pub all_tags:     Vec<SmallTag>,
    pub prev:         Option<FeaturedItem>,
    pub next:         Option<FeaturedItem>,
}

#[derive(Serialize)]
pub struct CategoryPageResp {
    pub request_user:     UserResp,
    pub category:         CatDetail,
    pub cats:             Vec<Cat>,
    pub all_tags:         Vec<SmallTag>,
    pub object_list:      Vec<Blog>,
    pub next_page_number: i32,
}

#[derive(Serialize)]
pub struct ItemResp {
    pub id:          i32,
    pub slug:        String,
    pub title:       String,
    pub description: Option<String>,
    pub created:     String,
    pub price:       i32,
    pub price_acc:   Option<i32>,
    pub image:       String,
}
#[derive(Serialize)]
pub struct CatDataResp {
    pub category:    &Cat,
    pub object_list: Vec<ItemResp>,
} 

#[derive(Serialize)]
pub struct CategoriesPageResp {
    pub request_user: UserResp,
    pub categories:   Vec<CatDataResp>,
    pub cats:         Vec<Cat>,
    pub all_tags:     Vec<SmallTag>,
    pub view:         i32,
    pub height:       f64, 
    pub seconds:      i32,
}

#[derive(Serialize)]
pub struct ItemDetailResp {
    pub id:         i32,
    pub slug:       String,
    pub title:      String,
    pub link:       Option<String>,
    pub item_types: i16,
    pub created:    String,
    pub price:      i32,
    pub price_acc:  Option<i32>,
    pub image:      String,
    pub view:       i32,
    pub height:     f64, 
    pub seconds:    i32,
    pub tags:       Vec<SmallTag>,
    pub owner:      OwnerResp,
    pub contents:   Vec<ContentBlock>,
}
fn get_item_data(object: Item, perm: i16) -> ItemDetailResp {
    // получаем детали универсального объекта
    let types = object.types;
    if object.item_types < 10 && perm < 10 {
        return ItemDetailResp {
            id:          object.id,
            slug:        object.slug.clone(),
            title:       object.title.clone(),
            link:        None,
            item_types:  0,
            created:     "".to_string(),
            price:       0,
            price_acc:   None,
            image:       "".to_string(),
            view:        0,
            height:      0.0, 
            seconds:     0,
            tags:        Vec::new(),
            owner:       Item::get_owner(object.user_id),
            contents:    Vec::new(),
        } 
    }
    else {
        return ItemDetailResp {
            id:          object.id,
            slug:        object.slug.clone(),
            title:       object.title.clone(),
            link:        object.link.clone(),
            item_types:  object.item_types,
            created:     object.created.format("%d-%m-%Y в %H:%M").to_string(),
            price:       object.price,
            price_acc:   object.price_acc,
            image:       object.get_image(),
            view:        object.view,
            height:      object.height, 
            seconds:     object.seconds,
            tags:        object.get_tags().expect("E"),
            owner:       Item::get_owner(object.user_id),
            contents:    object.get_contents(),
        } 
    }
}

#[derive(Deserialize)]
pub struct ItemDetailData {
    pub cat_slug: Option<String>,
    pub slug:     Option<String>,
    pub is_ajax:  Option<i16>,
}
async fn get_item_page (
    req:   HttpRequest,
    types: i16 
) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы универсального объекта
    use schema::{
        items::dsl::items,
        categories::dsl::categories,
    };

    let params_some = web::Query::<ItemDetailData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if params.cat_slug.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'cat_slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    
    let slug = params.slug.as_deref().unwrap().to_string();
    let cat_slug = params.cat_slug.as_deref().unwrap().to_string();
    if slug.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' is empty!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }
    else if cat_slug.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'cat_slug' is empty!".to_string(),
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

    let _item = items
        .filter(schema::items::slug.eq(slug))
        //.filter(schema::items::types.eq(types))
        .first::<Item>(&_connection)
        .expect("E");
    let _category = categories
        .filter(schema::categories::slug.eq(cat_slug))
        .filter(schema::categories::types.eq(_item.types))
        .first::<Categories>(&_connection)
        .expect("E");

    let _cats: Vec<Cat>;
    let _tags: Vec<SmallTag>;

    let cats_res = block(move || Categories::get_categories_for_types(types)).await?;
    _cats = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };
    let tags_res = block(move || Categories::get_tags(types)).await?;
    _tags = match tags_res {
        Ok(_list) => _list,
        Err(_error) => Vec::new(),
    };

    let (prev, next) = _category.get_featured_items(_item.types, _item.id);


    let cat_data = Cat {
        name:  _category.name.clone(),
        slug:  _category.slug.clone(),
        count: _category.count,
        id:    _category.id,
        image: Some(_category.get_image()),
        types: _category.types,
    };

    return Ok(Json(ObjectPageResp {
        request_user: get_request_user(&req, is_ajax).await,
        object:       get_item_data(_item, _request_user.perm),
        category:     cat_data,
        cats:         _cats,
        all_tags:     _tags,
        prev:         prev,
        next:         next,
    }));
}

pub async fn get_blog_page(req: HttpRequest) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы статьи блога
    return get_item_page(req, 1).await;
}
pub async fn get_help_page(req: HttpRequest) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы помощи
    return get_item_page(req, 6).await;
}
pub async fn get_service_page(req: HttpRequest) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы услуги сервиса
    return get_item_page(req, 2).await;
}
pub async fn get_store_page(req: HttpRequest,) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы товара
    return get_item_page(req, 3).await;
}
pub async fn get_wiki_page(req: HttpRequest,) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы обучающей статьи
    return get_item_page(req, 4).await;
}
pub async fn get_work_page(req: HttpRequest) -> Result<Json<ObjectPageResp>, Error> {
    // получаем детали страницы работы
    return get_item_page(req, 5).await;
}

#[derive(Deserialize)]
pub struct CategoryDetailData {
    pub slug:    Option<String>,
    pub is_ajax: Option<i16>,
    pub page:    Option<i32>,
}
async fn item_category_page (
    req: HttpRequest, 
    types: i16
) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы универсальной категории
    use crate::schema::categories::dsl::categories;
    use crate::utils::{is_desctop, get_page};

    let params_some = web::Query::<CategoryDetailData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.slug.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let slug = params.slug.as_deref().unwrap().to_string();
    if slug.is_empty() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'slug' is empty!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let _connection = establish_connection();

    let _category = categories
        .filter(schema::categories::slug.eq(slug))
        .filter(schema::categories::types.eq(types))
        .select((
            schema::categories::name,
            schema::categories::slug,
            schema::categories::count,
            schema::categories::id,
            schema::categories::image,
            schema::categories::view,
            schema::categories::height,
            schema::categories::seconds,
        ))
        .first::<CatDetail>(&_connection)
        .expect("E");

    let is_ajax: i16;
    let page:    i32;
    if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
        is_ajax = params.is_ajax.unwrap();
    }
    else {
        is_ajax = 0;
    }
    if params.page.is_some() && params.page.unwrap() > 1 {
        page = params.page.unwrap();
    }
    else {
        page = 1;
    }

    let _cats: Vec<Cat>;
    let _tags: Vec<SmallTag>;
    let object_list: Vec<Blog>;
    let next_page_number: i32;
        
    if is_ajax < 3 {
        let cats_res = block(move || Categories::get_categories_for_types(types)).await?;
        _cats = match cats_res {
            Ok(_ok) => _ok,
            Err(_error) => Vec::new(),
        };
        let tags_res = block(move || Categories::get_tags(types)).await?;
        _tags = match tags_res {
            Ok(_list) => _list,
            Err(_error) => Vec::new(),
        };
    }
    else {
        _cats = Vec::new();
        _tags = Vec::new();
    }

    let _request_user = get_request_user(&req, is_ajax).await;
    let _res = block(move || Categories::get_blogs_list(_category.id, page, 20, _request_user.perm == 60)).await?;
    let _dict = match _res {
        Ok(_ok) => {object_list = _ok.0; next_page_number = _ok.1},
        Err(_error) => {object_list = Vec::new(); next_page_number = 0},
    };
    return Ok(Json(CategoryPageResp {
        request_user:     _request_user,
        category:         _category,
        cats:             _cats,
        all_tags:         _tags,
        object_list:      object_list,
        next_page_number: next_page_number,
    }));
}

pub async fn blog_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории блога
    return item_category_page(req, 1).await;
}
pub async fn help_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории помощи
    return item_category_page(req, 6).await;
}
pub async fn service_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории услуг сервиса
    return item_category_page(req, 2).await;
}
pub async fn store_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории товаров
    return item_category_page(req, 3).await;
}
pub async fn wiki_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории обучающих статей
    return item_category_page(req, 4).await;
}
pub async fn work_category_page(req: HttpRequest) -> Result<Json<CategoryPageResp>, Error> {
    // получаем детали страницы категории товаров
    return item_category_page(req, 5).await;
}

async fn item_categories_page (
    req: HttpRequest,
    types: i16,
) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий по типу
    use crate::utils::{is_desctop, get_stat_page};
    use crate::schema::stat_pages::dsl::stat_pages;
    use crate::models::StatPage;

    let _connection = establish_connection();
    let _stat = stat_pages
        .filter(schema::stat_pages::types.eq(Categories::get_stat_type(types)))
        .first::<StatPage>(&_connection)
        .expect("E");

    let _cats: Vec<Cat>;
    let _tags: Vec<SmallTag>;
    let _request_user = get_request_user(&req, get_is_ajax(&req)).await;
    let is_superuser = _request_user.perm == 60;

    let cats_res = block(move || Categories::get_categories_for_types(types)).await?;
    _cats = match cats_res {
        Ok(_ok) => _ok,
        Err(_error) => Vec::new(),
    };

    let tags_res = block(move || Categories::get_tags(types)).await?;
    _tags = match tags_res {
        Ok(_list) => _list,
        Err(_error) => Vec::new(),
    };

    let mut categories: Vec<CatDataResp> = Vec::new();
    for cat in &_cats.iter() {
        let mut stack = Vec::new();
        for i in cat.get_items_list(6, types, is_superuser).iter() {
            stack.push( ItemResp {
                id:          i.id,
                slug:        i.slug.clone(),
                title:       i.title.clone(),
                description: i.description.clone(),
                created:     i.created.format("%d-%m-%Y в %H:%M").to_string(),
                price:       i.price,
                price_acc:   i.price_acc,
                image:       i.get_image(),
            });
            categories.push( CatDataResp {
                category:    cat,
                object_list: stack,

            });
        }
    }
    
    return Ok(Json( CategoriesPageResp {
        request_user: _request_user,
        categories:   categories,
        cats:         _cats,
        all_tags:     _tags,
        view:         _stat.view,
        height:       _stat.height, 
        seconds:      _stat.seconds,
    }));
}

pub async fn blog_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий блога
    return item_categories_page(req, 41).await;
} 
pub async fn help_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий помощи
    return item_categories_page(req, 101).await;
}
pub async fn service_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий сервиса
    return item_categories_page(req, 61).await;
}
pub async fn store_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий товаров
    return item_categories_page(req, 71).await;
}
pub async fn wiki_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий википедии
    return item_categories_page(req, 81).await;
}
pub async fn work_categories_page(req: HttpRequest) -> Result<Json<CategoriesPageResp>, Error> {
    // получаем детали страницы категорий работ
    return item_categories_page(req, 91).await;
} 