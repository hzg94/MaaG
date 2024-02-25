import {invoke} from '@tauri-apps/api'

export const AdbDeviceList = async () => {
    if (typeof window.__TAURI_IPC__ == "function") {
        await invoke("list_devices").then(x => {
            console.log(x);

        })
    }
}