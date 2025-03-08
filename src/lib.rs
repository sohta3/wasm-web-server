use wasm_bindgen::prelude::*;
use web_sys::{Request, Response, ResponseInit, Headers};
use js_sys::Promise;

#[wasm_bindgen]
pub struct WebServer {
    routes: Vec<(String, String)>, // (path, content)
}

#[wasm_bindgen]
impl WebServer {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        let mut server = WebServer {
            routes: Vec::new(),
        };
        server.add_default_routes();
        server
    }

    fn create_response(body: &str, status: u16, content_type: &str) -> Result<Response, JsValue> {
        let mut init = ResponseInit::new();
        init.set_status(status);

        let headers = Headers::new()?;
        headers.set("Content-Type", content_type)?;
        init.set_headers(&headers);

        Response::new_with_opt_str_and_init(Some(body), &init)
    }

    fn add_default_routes(&mut self) {
        self.routes.push(("/".to_string(), "Welcome to Wasm Web Server!".to_string()));
        self.routes.push(("/hello".to_string(), "Hello from Wasm Server!".to_string()));
        self.routes.push(("/api/data".to_string(), r#"{"message": "APIからのデータです"}"#.to_string()));
    }

    #[wasm_bindgen]
    pub fn handle_request(&self, request: Request) -> Result<Response, JsValue> {
        let url = web_sys::Url::new(&request.url())?;
        let path = url.pathname();

        for (route_path, content) in &self.routes {
            if &path == route_path {
                if route_path.contains("api") {
                    return Self::create_response(&content, 200, "application/json");
                } else {
                    return Self::create_response(&content, 200, "text/plain");
                }
            }
        }

        // 404 Not Found
        Self::create_response("Not Found", 404, "text/plain")
    }
}
