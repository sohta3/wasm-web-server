use wasm_bindgen::prelude::*;
use web_sys::{Request, Response, ResponseInit, Headers, Cache, console};
use serde::{Serialize, Deserialize};
use wasm_bindgen_futures::JsFuture;
use js_sys::Promise;

#[derive(Serialize, Deserialize, Clone)]
struct Todo {
    id: u32,
    title: String,
    completed: bool,
}

#[derive(Deserialize)]
struct CreateTodoRequest {
    title: String,
    completed: Option<bool>,
}

#[derive(Deserialize)]
struct UpdateTodoRequest {
    title: Option<String>,
    completed: Option<bool>,
}

#[wasm_bindgen]
pub struct WebServer {
    cache: Cache,
}

#[wasm_bindgen]
impl WebServer {
    #[wasm_bindgen]
    pub fn create() -> Promise {
        console::log_1(&"Creating WebServer...".into());
        let promise = js_sys::Promise::new(&mut |resolve, reject| {
            wasm_bindgen_futures::spawn_local(async move {
                match async {
                    console::log_1(&"Getting ServiceWorkerGlobalScope...".into());
                    let global = js_sys::global();
                    console::log_1(&"Got global object".into());

                    let scope = global.dyn_into::<web_sys::ServiceWorkerGlobalScope>()
                        .map_err(|_| JsValue::from_str("Failed to get ServiceWorkerGlobalScope"))?;
                    console::log_1(&"Got ServiceWorkerGlobalScope".into());

                    let cache_storage = scope.caches()?;
                    console::log_1(&"Got CacheStorage".into());

                    let promise = cache_storage.open("todos-cache");
                    let cache = JsFuture::from(promise).await?;
                    console::log_1(&"Opened cache".into());

                    let cache = cache.dyn_into::<Cache>()?;
                    console::log_1(&"Got Cache object".into());

                    let server = WebServer { cache };
                    console::log_1(&"Server initialized".into());
                    Ok::<_, JsValue>(server)
                }.await {
                    Ok(server) => {
                        console::log_1(&"Resolving promise with server".into());
                        resolve.call1(&JsValue::undefined(), &JsValue::from(server)).unwrap();
                    },
                    Err(err) => {
                        console::log_1(&"Rejecting promise with error".into());
                        console::log_1(&err);
                        reject.call1(&JsValue::undefined(), &err).unwrap();
                    }
                }
            });
        });
        promise.unchecked_into()
    }

    fn create_response(body: &str, status: u16, content_type: &str) -> Result<Response, JsValue> {
        let init = ResponseInit::new();
        init.set_status(status);

        let headers = Headers::new()?;
        headers.set("Content-Type", content_type)?;
        init.set_headers(&headers);

        Response::new_with_opt_str_and_init(Some(body), &init)
    }

    #[wasm_bindgen]
    pub async fn handle_request(&mut self, request: Request) -> Result<Response, JsValue> {
        let url = web_sys::Url::new(&request.url())?;
        let path = url.pathname();
        let method = request.method();

        console::log_1(&format!("Received request: {} {}", method, path).into());

        // キャッシュを一時変数に保存
        let cache = &self.cache;

        // TODOリストの取得用ヘルパー関数
        async fn get_todos(cache: &Cache) -> Result<Vec<Todo>, JsValue> {
            let promise = cache.match_with_str("/todos");
            let response = JsFuture::from(promise).await?;

            if response.is_undefined() {
                return Ok(Vec::new());
            }

            let response: Response = response.dyn_into()?;
            let text = JsFuture::from(response.text()?).await?;
            let text = text.as_string().unwrap_or_else(|| "[]".to_string());
            let todos: Vec<Todo> = serde_json::from_str(&text).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(todos)
        }

        // TODOリストの保存用ヘルパー関数
        async fn save_todos(cache: &Cache, todos: &[Todo]) -> Result<(), JsValue> {
            let todos_str = serde_json::to_string(todos).map_err(|e| JsValue::from_str(&e.to_string()))?;
            let response = Response::new_with_opt_str(Some(&todos_str))?;
            let request = Request::new_with_str("/todos")?;
            let promise = cache.put_with_request(&request, &response);
            JsFuture::from(promise).await?;
            Ok(())
        }

        // TODOリスト関連のエンドポイント
        match (method.as_str(), path.as_str()) {
            // TODOリストの取得
            ("GET", "/api/todos") => {
                console::log_1(&"Handling GET /api/todos".into());
                let todos = get_todos(cache).await?;
                let json = serde_json::to_string(&todos).map_err(|e| JsValue::from_str(&e.to_string()))?;
                Self::create_response(&json, 200, "application/json")
            }
            // 新しいTODOの追加
            ("POST", "/api/todos") => {
                console::log_1(&"Handling POST /api/todos".into());
                let mut todos = get_todos(cache).await?;
                let body = JsFuture::from(request.text()?).await?;
                let body_str = body.as_string().ok_or_else(|| JsValue::from_str("Invalid request body"))?;
                console::log_1(&format!("Received body: {}", body_str).into());

                let create_request: CreateTodoRequest = serde_json::from_str(&body_str)
                    .map_err(|e| JsValue::from_str(&format!("Invalid request format: {}", e)))?;

                let new_todo = Todo {
                    id: todos.iter().map(|t| t.id).max().unwrap_or(0) + 1,
                    title: create_request.title,
                    completed: create_request.completed.unwrap_or(false),
                };

                todos.push(new_todo);
                save_todos(cache, &todos).await?;

                Self::create_response(r#"{"status": "created"}"#, 201, "application/json")
            }
            // TODOの更新（完了状態の切り替えなど）
            ("PUT", path) if path.starts_with("/api/todos/") => {
                console::log_1(&format!("Handling PUT {}", path).into());
                let id: u32 = path[11..].parse().map_err(|_| JsValue::from_str("Invalid ID"))?;
                let mut todos = get_todos(cache).await?;
                let body = JsFuture::from(request.text()?).await?;
                let body_str = body.as_string().ok_or_else(|| JsValue::from_str("Invalid request body"))?;
                let update_request: UpdateTodoRequest = serde_json::from_str(&body_str)
                    .map_err(|e| JsValue::from_str(&format!("Invalid request format: {}", e)))?;

                if let Some(todo) = todos.iter_mut().find(|t| t.id == id) {
                    if let Some(title) = update_request.title {
                        todo.title = title;
                    }
                    if let Some(completed) = update_request.completed {
                        todo.completed = completed;
                    }
                    save_todos(cache, &todos).await?;
                    Self::create_response(r#"{"status": "updated"}"#, 200, "application/json")
                } else {
                    Self::create_response(r#"{"error": "Todo not found"}"#, 404, "application/json")
                }
            }
            // TODOの削除
            ("DELETE", path) if path.starts_with("/api/todos/") => {
                console::log_1(&format!("Handling DELETE {}", path).into());
                let id: u32 = path[11..].parse().map_err(|_| JsValue::from_str("Invalid ID"))?;
                let mut todos = get_todos(cache).await?;
                let initial_len = todos.len();
                todos.retain(|t| t.id != id);

                if todos.len() < initial_len {
                    save_todos(cache, &todos).await?;
                    Self::create_response(r#"{"status": "deleted"}"#, 200, "application/json")
                } else {
                    Self::create_response(r#"{"error": "Todo not found"}"#, 404, "application/json")
                }
            }
            // 未対応のエンドポイント
            _ => {
                console::log_1(&format!("No route match for {} {}", method, path).into());
                Self::create_response("Not Found", 404, "text/plain")
            }
        }
    }
}
