export interface TaskItem {
    path_video: string
    lang: number
    graphic: string
    model: string
    num_cores: number
    num_threads: number
    max_len_chars: number
    path_save: string
}

export interface Language {
    code: number
    name_zh: string
}

export interface Model {
    name: string
    path: string
}

export interface TaskEventData  {
    is_success: Boolean
    message: String
    ts_execed: number
    ts: number
}

export type TaskEvent =
{
    event: 'Progress',
    data: TaskEventData
}