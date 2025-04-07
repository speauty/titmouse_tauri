import { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

export const listenForMin = async (): Promise<UnlistenFn> => {
  return await getCurrentWindow().onResized(async ({ payload: size }) => {
    if (size.type == 'Physical' && size.width == 0) {
      if (await getCurrentWindow().isVisible()) await getCurrentWindow().hide()
    }
  })
}