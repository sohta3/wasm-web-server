// Service Workerの登録
if ("serviceWorker" in navigator) {
	window.addEventListener("load", async () => {
		try {
			const registration = await navigator.serviceWorker.register("/sw.js", {
				type: "module",
			});
			console.log("Service Worker 登録成功:", registration.scope);
		} catch (error) {
			console.error("Service Worker 登録失敗:", error);
		}
	});
}
