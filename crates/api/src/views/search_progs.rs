use actix_web::{
    HttpRequest,
    HttpResponse,
    web,
    web::{block, Data, Json},
};

use crate::models::{Item, Blog, Service, Store, Wiki, Work, Help};

use crate::utils::{
    establish_connection,
    get_request_user, get_is_ajax,
    ErrorParams, TOKEN, UserResp,
};


pub fn search_routes(config: &mut web::ServiceConfig) {
    config.route("/search", web::get().to(empty_search_page));
    config.route("/search", web::get().to(search_page));
    config.route("/search_blogs", web::get().to(search_blogs_page));
    config.route("/search_services", web::get().to(search_services_page));
    config.route("/search_stores", web::get().to(search_stores_page));
    config.route("/search_wikis", web::get().to(search_wikis_page));
    config.route("/search_works", web::get().to(search_works_page));
    config.route("/search_help", web::get().to(search_help_page));
}


#[derive(Serialize)]
pub struct EmptySearchResp {
    pub request_user: UserResp,
}
pub async fn empty_search_page(req: HttpRequest) -> Result<Json<EmptySearchResp>, Error> {
    return Ok(Json(EmptySearchResp {
        request_user: get_request_user(&req, get_is_ajax(&req)),
    }));
}


#[derive(Deserialize)]
pub struct SearchPageData {
    pub q:       String,
    pub is_ajax: i16,
}
#[derive(Serialize)]
pub struct SearchPageResp {
    pub request_user:   UserResp,
    pub works_list:     Vec<Work>,
    pub services_list:  Vec<Service>,
    pub wikis_list:     Vec<Wiki>,
    pub blogs_list:     Vec<Blog>,
    pub stores_list:    Vec<Store>,
    pub helps_list:     Vec<Help>,
    pub works_count:    usize,
    pub services_count: usize,
    pub wikis_count:    usize,
    pub blogs_count:    usize,
    pub stores_count:   usize,
    pub helps_count:    usize,
    pub is_ajax:        i32,
    pub q:              String,
}
pub async fn search_page(req: HttpRequest) -> Result<Json<SearchPageResp>, Error> {
    let params_some = web::Query::<SearchPageData>::from_query(&req.query_string());
    if params_some.is_err() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametrs not found!".to_string(),
        }).unwrap();
        return Err(Error::BadRequest(body));
    }

    let params = params_some.unwrap();
    if params.q.is_none() {
        let body = serde_json::to_string(&ErrorParams {
            error: "parametr 'q' not found!".to_string(),
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
    let _q = params.q.as_deref().unwrap().to_string();
    let _q_standalone = "%".to_owned() + &_q + "%";
    let is_admin = _request_user.perm > 59; 

    let (works_list, works_count) = Item::search_works(&_q_standalone, 3, 0, is_admin);
    let (services_list, services_count) = Item::search_services(&_q_standalone, 3, 0, is_admin);
    let (wikis_list, wikis_count) = Item::search_wikis(&_q_standalone, 3, 0, is_admin);
    let (blogs_list, blogs_count) = Item::search_blogs(&_q_standalone, 3, 0, is_admin);
    let (stores_list, stores_count) = Item::search_stores(&_q_standalone, 3, 0, is_admin);
    let (helps_list, helps_count) = Item::search_helps(&_q_standalone, 3, 0, is_admin);

    return Ok(Json(SearchPageResp {
        request_user:  _request_user,
        works_list:     works_list,
        services_list:  services_list,
        wikis_list:     wikis_list,
        blogs_list:     blogs_list,
        stores_list:    stores_list,
        helps_list:     helps_list,
        works_count:    works_count,
        services_count: services_count,
        wikis_count:    wikis_count,
        blogs_count:    blogs_count,
        stores_count:   stores_count,
        helps_count:    helps_count,
        q:              _q,
    }));
}

#[derive(Deserialize)]
struct SearchItemsPageData {
    pub q:       String,
    pub page:    i16,
    pub is_ajax: i16,
}
fn get_q_page_is_ajax(req: &HttpRequest) -> (String, i16, i16) {
    let params_some = web::Query::<SearchItemsPageData>::from_query(&req.query_string());
    let q: String;
    let page: i32;
    let is_ajax: i32;
    if params_some.is_ok() {
        let params = params_some.unwrap();
        if params.page.is_some() && params.page.unwrap() > 1 {
            page = params.page.unwrap();
        }
        else {
            page = 1;
        }
        if params.is_ajax.is_some() && params.is_ajax.unwrap() > 0 {
            is_ajax = params.is_ajax.unwrap();
        }
        else {
            is_ajax = 0;
        }
        if params.q.is_none() {
            q = String::new();
        }
        else {
            q = params.q.as_deref().unwrap().to_string();
        }
    }
    else {
        page = 1;
        is_ajax = 0;
        q = String::new();
    }

    return (q, page, is_ajax);
}

#[derive(Serialize)]
pub struct SearchBlogsResp {
    pub request_user:     UserResp,
    pub blogs_list:       Vec<Blog>,
    pub blogs_count:      usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_blogs_page(req: HttpRequest) -> Result<Json<SearchBlogsResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchBlogsResp {
            request_user:     _request_user,
            blogs_list:       Vec::new(),
            blogs_count:      0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Item::search_blogs(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchBlogsResp {
        request_user:     _request_user,
        blogs_list:       items_list,
        blogs_count:      items_count,
        q:                q,
        next_page_number: 0,
    }));
}

#[derive(Serialize)]
pub struct SearchServicesResp {
    pub request_user:     UserResp,
    pub service_list:     Vec<Service>,
    pub service_count:    usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_services_page(req: HttpRequest) -> Result<Json<SearchServicesResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchServicesResp {
            request_user:     _request_user,
            services_list:    Vec::new(),
            services_count:   0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Service::search_services(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchServicesResp {
        request_user:     _request_user,
        services_list:    items_list,
        services_count:   items_count,
        q:                q,
        next_page_number: 0,
    }));
}

#[derive(Serialize)]
pub struct SearchStoresResp {
    pub request_user:     UserResp,
    pub stores_list:      Vec<Store>,
    pub stores_count:     usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_stores_page(req: HttpRequest) -> Result<Json<SearchStoresResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchStoresResp {
            request_user:     _request_user,
            stores_list:      Vec::new(),
            stores_count:     0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Store::search_stores(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchStoresResp {
        request_user:     _request_user,
        stores_list:      items_list,
        stores_count:     items_count,
        q:                q,
        next_page_number: 0,
    }));
}

#[derive(Serialize)]
pub struct SearchWikisResp {
    pub request_user:     UserResp,
    pub wikis_list:       Vec<Wiki>,
    pub wikis_count:      usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_wikis_page(req: HttpRequest) -> Result<Json<SearchWikisResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchWikisResp {
            request_user:     _request_user,
            wikis_list:       Vec::new(),
            wikis_count:      0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Wiki::search_wikis(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchWikisResp {
        request_user:     _request_user,
        wikis_list:       items_list,
        wikis_count:      items_count,
        q:                q,
        next_page_number: 0,
    }));
}

#[derive(Serialize)]
pub struct SearchWorksResp {
    pub request_user:     UserResp,
    pub works_list:       Vec<Work>,
    pub works_count:      usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_works_page(req: HttpRequest) -> Result<Json<SearchWorksResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchWorksResp {
            request_user:     _request_user,
            works_list:       Vec::new(),
            works_count:      0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Work::search_works(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchWorksResp {
        request_user:     _request_user,
        works_list:       items_list,
        works_count:      items_count,
        q:                q,
        next_page_number: 0,
    }));
}

#[derive(Serialize)]
pub struct SearchHelpsResp {
    pub request_user:     UserResp,
    pub helps_list:       Vec<Help>,
    pub helps_count:      usize,
    pub q:                String,
    pub next_page_number: i16,
}
pub async fn search_helps_page(req: HttpRequest) -> Result<Json<SearchHelpsResp>, Error> {
    let (q, page, is_ajax) = get_q_page_is_ajax(&req);
    let _request_user = get_request_user(&req, is_ajax);
    if q.is_empty() {
        return Ok(Json(SearchHelpsResp {
            request_user:     _request_user,
            helps_list:       Vec::new(),
            helps_count:      0,
            q:                q,
            next_page_number: 0,
        }));
    }

    let _q_standalone = "%".to_owned() + &q + "%";
    let mut next_page_number = 0;
    let offset: i32;
    let next_item: i32;
    
    if page > 1 {
        offset = (page - 1) * 20;
        next_item = page * 20 + 1;
    }
    else {
        offset = 0;
        next_item = 21;
    }

    let (items_list, items_count) = Help::search_helps(&_q_standalone, 20, offset.into(), _request_user.perm > 59);
    if items_count >= next_item {
        next_page_number = page + 1;
    }

    return Ok(Json(SearchHelpsResp {
        request_user:     _request_user,
        helps_list:       items_list,
        helps_count:      items_count,
        q:                q,
        next_page_number: 0,
    }));
}