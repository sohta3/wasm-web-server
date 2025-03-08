import * as wasm from "./pkg/wasm_web_server.js";

let server = null;

self.addEventListener("install", (event) => {
	console.log("Service Worker: インストール完了");
});

self.addEventListener("activate", (event) => {
	console.log("Service Worker: アクティブ化完了");
	// Service Worker起動時にWebServerを初期化
	event.waitUntil(
		(async () => {
			await wasm.default();
			server = await wasm.WebServer.create();
			console.log("WebServer initialized");
		})(),
	);
});

self.addEventListener("fetch", (event) => {
	const url = new URL(event.request.url);

	// localhost:3000へのリクエストをインターセプト
	if (url.hostname === "localhost" && url.port === "3000") {
		console.log("リクエストをインターセプト:", url.pathname);

		event.respondWith(
			(async () => {
				try {
					if (!server) {
						await wasm.default();
						server = await wasm.WebServer.create();
					}
					return await server.handle_request(event.request);
				} catch (error) {
					console.error("Wasmサーバーエラー:", error);
					return new Response("Internal Server Error", {
						status: 500,
						headers: { "Content-Type": "text/plain" },
					});
				}
			})(),
		);
	}
});
