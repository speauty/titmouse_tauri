import { Language, Model, TaskEvent, TaskItem } from '@/types/type'
import { Channel, invoke } from '@tauri-apps/api/core'

export const commandLanuages = async (): Promise<Language[]> => await invoke('languages')
export const commandModels = async (): Promise<Model[]> => await invoke('models')
export const commandGraphics = async (): Promise<string[]> => await invoke('graphics')
export const commandCheckVideoPathIsValid = async (pathVideo: string): Promise<string[]> => await invoke('action_check_video_path_is_valid', {pathVideo: pathVideo})

export const commandUploadTask = async (task: TaskItem, onEvent: Channel<TaskEvent>): Promise<string> => await invoke('action_upload_task', {task: task, onEvent})