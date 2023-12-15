import {invoke} from '@tauri-apps/api'

export const CloseLoadPage = async () => {
    await invoke("close_splashscreen")
}

