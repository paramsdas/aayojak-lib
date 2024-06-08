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

#[post("/todo/create")]
async fn create_todo(
    todo: web::Query<CreateTodoBody>,
    data: web::Data<AppState>,
) -> impl Responder {
    println!("Creating todo...");
    let mut app_state = data.todo_list.lock().unwrap();
    let new_id = app_state.id_counter;
    let todo_result = Todo::new(&todo.title, new_id);
    match todo_result {
        Ok(todo) => {
            app_state.map.insert(new_id, todo);
            let inserted_todo = app_state.map.get(&new_id).unwrap();
            let todo_from_todo_list = serde_json::to_string(inserted_todo).unwrap();
            app_state.id_counter += 1;
            HttpResponse::Ok().body(todo_from_todo_list)
        }
        Err(e) => HttpResponse::InternalServerError().body(format!("Could not create todo: {e}")),
    }
}

#[get("/todo/get")]
async fn get_all_todos(data: web::Data<AppState>) -> impl Responder {
    println!("Creating todo...");
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
            .service(web::scope("/api").service(create_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
