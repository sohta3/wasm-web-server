self.addEventListener('install', (event) => {
    console.log('Service Worker: インストール完了');
});

self.addEventListener('activate', (event) => {
    console.log('Service Worker: アクティブ化完了');
});

self.addEventListener('fetch', async (event) => {
    const url = new URL(event.request.url);

    // localhost:3000へのリクエストをインターセプト
    if (url.hostname === 'localhost' && url.port === '3000') {
        console.log('リクエストをインターセプト:', url.pathname);

        // Wasmモジュールをインポート
        const wasm = await import('./pkg/wasm_web_server.js');
        await wasm.default();

        // Wasmサーバーのインスタンスを作成
        const server = new wasm.WebServer();

        try {
            // リクエストをWasmサーバーに転送
            const response = await server.handle_request(event.request);
            event.respondWith(response);
        } catch (error) {
            console.error('Wasmサーバーエラー:', error);
            event.respondWith(
                new Response('Internal Server Error', {
                    status: 500,
                    headers: { 'Content-Type': 'text/plain' }
                })
            );
        }
    }
});
