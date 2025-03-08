import * as wasm from "./pkg/wasm_web_server.js";

self.addEventListener("install", (event) => {
	console.log("Service Worker: インストール完了");
});

self.addEventListener("activate", (event) => {
	console.log("Service Worker: アクティブ化完了");
});

self.addEventListener("fetch", async (event) => {
	const url = new URL(event.request.url);

	// localhost:3000へのリクエストをインターセプト
	if (url.hostname === "localhost" && url.port === "3000") {
		console.log("リクエストをインターセプト!:", url.pathname);

		event.respondWith(
			(async () => {
				try {
					await wasm.default();
					// Wasmサーバーのインスタンスを作成
					const server = new wasm.WebServer();
					// リクエストをWasmサーバーに転送
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
