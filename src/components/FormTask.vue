<template>
  <div class="w-full">
    <a-form ref="refFormTask" :model="formTask" :rules="taskRules" :colon="false" autocomplete="off">
      <a-form-item label="目标视频" name="path_video">
        <a-input placeholder="请选择目标视频" v-model:value="formTask.path_video" :disabled="isProgressing">
          <template #addonAfter>
            <FolderOpenOutlined @click="fnSelectFileOrDirectory" />
          </template>
        </a-input>
      </a-form-item>
      <a-form-item label="应用模型" name="model">
        <a-select placeholder="请选择应用模型" notFoundContent="暂无数据" v-model:value="formTask.model" :disabled="isProgressing">
          <a-select-option v-for="model in models" :value="model.path">{{ model.name }}</a-select-option>
        </a-select>
      </a-form-item>
      <a-form-item label="视频语种" name="lang">
        <a-select placeholder="请选择视频语种" notFoundContent="暂无数据" v-model:value="formTask.lang" :disabled="isProgressing">
          <a-select-option v-for="lanuage in lanuages" :value="lanuage.code">{{ lanuage.name_zh }}</a-select-option>
        </a-select>
      </a-form-item>
      <a-form-item label="应用显卡" name="graphic">
        <a-select placeholder="请选择应用显卡" notFoundContent="暂无数据" v-model:value="formTask.graphic" :disabled="isProgressing">
          <a-select-option v-for="graphic in graphics" :value="graphic" :title="graphic">{{ graphic }}</a-select-option>
        </a-select>
      </a-form-item>
      <a-row justify="space-between">
        <a-col>
          <a-form-item label="处理器数" name="num_cores">
            <a-input-number placeholder="请输入" class="!w-18" :defaultValue="1" :min="0" :max="16"
              v-model:value="formTask.num_cores" :disabled="isProgressing"/>
          </a-form-item>
        </a-col>
        <a-col>
          <a-form-item label="线程数量" name="num_threads">
            <a-input-number placeholder="请输入" class="!w-18" :defaultValue="1" :min="0" :max="32"
              v-model:value="formTask.num_threads" :disabled="isProgressing" />
          </a-form-item>
        </a-col>
        <a-col>
          <a-form-item label="行最大字符数" name="max_len_chars">
            <a-input-number placeholder="请输入" class="!w-18" :defaultValue="0" :min="0"
              v-model:value="formTask.max_len_chars" :disabled="isProgressing" />
          </a-form-item>
        </a-col>
      </a-row>
      
      <a-form-item label="保存位置" name="path_save">
        <a-input placeholder="默认同视频位置" v-model:value="formTask.path_save" :disabled="isProgressing">
          <template #addonAfter>
            <FolderOpenOutlined @click="fnSelectDirectory" />
          </template>
        </a-input>
      </a-form-item>
      <a-row class="mt-1" justify="center">
        <a-spin :spinning="isProgressing">
          <a-button type="primary" @click="fnSubmitTask">{{isProgressing?'任务处理中':'创建任务'}}</a-button>
        </a-spin>
      </a-row>
      <a-row v-if="cntSuccess" class="my-2" justify="center">
        <a-steps :current="cntSuccess" size="small" :items="steps"></a-steps>
      </a-row>
    </a-form>
  </div>
</template>

<script setup lang="ts">
import { FolderOpenOutlined } from '@ant-design/icons-vue'
import { message } from 'ant-design-vue'
import { open } from '@tauri-apps/plugin-dialog'
import { onMounted, onUnmounted, ref, watch } from 'vue'
import { commandGraphics, commandModels, commandLanuages, commandUploadTask, commandCheckVideoPathIsValid } from '@/libs/commands'
import { Language, Model, TaskItem, TaskEventData, TaskEvent } from '@/types/type'
import { Rule } from 'ant-design-vue/es/form'
import { Channel } from '@tauri-apps/api/core'
import dayjs from 'dayjs'
import { UnlistenFn } from '@tauri-apps/api/event'
import { getCurrentWindow } from '@tauri-apps/api/window'

const lanuages = ref<Language[]>([])
const models = ref<Model[]>([])
const graphics = ref<string[]>([])
const steps = [{title: "提交任务"}, {title: "任务开始"}, {title: "命令合成"}, {title: "字幕解析"}, {title: "写入成功"}, {title: "任务完成"},]

const formTask = ref<TaskItem>({ num_cores: 4, num_threads: 8, max_len_chars: 130 } as TaskItem)
const latestVideoPathValid = ref<string>("")
const refFormTask = ref()
const taskRules: Record<string, Rule[]> = {
  path_video: [{ required: true, message: '', trigger: 'change' }],
  model: [{ required: true, message: '', trigger: 'change' }],
}
const cntSuccess = ref<number>(0)
const isProgressing = ref<boolean>(false)

let fnUnlistentForDrop: UnlistenFn|null

const fnSubmitTask = () => {
  cntSuccess.value = 0
  refFormTask.value.validate().then(async () => {
    cntSuccess.value = 1
    isProgressing.value = true
    const onTaskEvent = new Channel<TaskEvent>()
    onTaskEvent.onmessage = (msg: TaskEvent) => {
      const data = msg.data as TaskEventData
      const tsFormatted = dayjs(data.ts).format("YYYY-MM-DD HH:mm:ss")
      if (data.is_success) {
        cntSuccess.value++
      } else {
        message.error(`${data.message}, 已运行${data.ts_execed}s, 执行时间: ${tsFormatted}`, 3)
        isProgressing.value = false
      }
      if (cntSuccess.value == 6) {
        message.success(`转换成功, 总共耗时: ${data.ts_execed}s`, 5)
        formTask.value.path_video = ""
        isProgressing.value = false
        setTimeout(() => { if (!isProgressing.value) cntSuccess.value = 0 }, 5e3)
      }
    }
    commandUploadTask(formTask.value, onTaskEvent)
  })
}

const fnSelectFileOrDirectory = async () => {
  const file = await open({
    multiple: false,
    directory: false,
  })
  if (!file) {
    if (!formTask.value.path_video) message.error('您暂未选择目标视频', 2)
    return
  }
  formTask.value.path_video = file as string
}

const fnSelectDirectory = async () => {
  const directory = await open({
    multiple: false, directory: true,
  })
  if (!directory) {
    if (!formTask.value.path_video) message.error('您暂未选择保存位置', 2)
    return
  }
  formTask.value.path_save = directory as string
}

const fnLoadOptions = async () => {
  lanuages.value = await commandLanuages()
  models.value = await commandModels()
  graphics.value = await commandGraphics()

  if (lanuages.value.length) formTask.value.lang = lanuages.value[0].code
  if (models.value.length) formTask.value.model = models.value[0].path
  if (graphics.value.length) formTask.value.graphic = graphics.value[0]
}

watch((): string => formTask.value.path_video, async (pathNew: string, pathOld: string) => {
  if (!pathNew || pathNew == pathOld || pathNew == latestVideoPathValid.value) return
  const err = await commandCheckVideoPathIsValid(pathNew)
  if (!err) {
    latestVideoPathValid.value = pathNew
    return
  }
  message.error(err, 2)
  formTask.value.path_video = latestVideoPathValid.value
})

const fnDropEvent = async () => {
  if (fnUnlistentForDrop) fnUnlistentForDrop()
  fnUnlistentForDrop = await getCurrentWindow().onDragDropEvent(async (event) => {
    if (event.payload.type === 'drop' && event.payload.paths.length == 1) {
      formTask.value.path_video = event.payload.paths[0]
    }
  })
}

onMounted(async () => {
  fnLoadOptions()
  await fnDropEvent()
})

onUnmounted(() => {
  if (fnUnlistentForDrop) fnUnlistentForDrop()
})
</script>