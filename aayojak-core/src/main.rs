use std::{collections::HashMap, sync::Mutex};

use aayojak::structures::todo::Todo;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

// Structures used only for server-side processing of data
struct TodoList {
    map: HashMap<i32, Todo>,
    id_counter: i32,
}

struct AppState {
    todo_list: Mutex<TodoList>,
}

#[derive(Deserialize)]
struct CreateTodoBody {
    title: String,
}

// Endpoints
#[get("/")]
async fn welcome() -> impl Responder {
    HttpResponse::Ok().body("Hello world, welcome to aayojak!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
    HttpResponse::Ok().body(req_body)
}

#[get("/")]
async fn api_version() -> impl Responder {
    HttpResponse::Ok().body("v0.0.1")
}

#[post("/todo/create")]
async fn create_todo(
    todo_request_body: web::Json<CreateTodoBody>,
    data: web::Data<AppState>,
) -> impl Responder {
    let app_state_result = data.todo_list.lock();
    if let Err(err) = app_state_result {
        return HttpResponse::InternalServerError()
            .body(format!("Error acquiring mutex-guard: {err}"));
    }

    let mut app_state = app_state_result.unwrap();

    let new_id = app_state.id_counter;
    // create todo
    println!("Creating todo...");
    let todo = Todo::new(&todo_request_body.title, new_id);
    app_state.map.insert(new_id, todo);
    let inserted_todo = app_state.map.get(&new_id).unwrap();

    //parse response
    match serde_json::to_string(inserted_todo) {
        Ok(response_body_todo) => {
            app_state.id_counter += 1;
            return HttpResponse::Ok().body(response_body_todo);
        }
        Err(err) => {
            app_state.id_counter += 1;
            return HttpResponse::InternalServerError()
                .body(format!("Error parsing response body: {err}"));
        }
    }
}

#[get("/todo/get")]
async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    let app_state = data.todo_list.lock().unwrap();
    HttpResponse::Ok().body(serde_json::to_string(&app_state.map).unwrap())
}

// MAIN SERVER
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let todo_list = web::Data::new(AppState {
        todo_list: Mutex::new(TodoList {
            map: HashMap::new(),
            id_counter: 0,
        }),
    });

    HttpServer::new(move || {
        App::new()
            .service(welcome)
            .service(echo)
            .app_data(todo_list.clone())
            .service(
                web::scope("/api")
                    .service(create_todo)
                    .service(api_version)
                    .service(get_all_todos),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
