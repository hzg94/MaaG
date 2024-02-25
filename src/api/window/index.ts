import {invoke} from '@tauri-apps/api'

export const CloseLoadPage = async () => {
    if (typeof window.__TAURI_IPC__ == "function") {
        await invoke("close_splashscreen")
    }
}

