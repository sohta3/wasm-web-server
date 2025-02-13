import init, { WebServer } from './pkg/wasm_web_server.js';

// Service Workerの登録
if ('serviceWorker' in navigator) {
    window.addEventListener('load', async () => {
        try {
            const registration = await navigator.serviceWorker.register('/sw.js');
            console.log('Service Worker 登録成功:', registration.scope);
        } catch (error) {
            console.log('Service Worker 登録失敗:', error);
        }
    });
}
