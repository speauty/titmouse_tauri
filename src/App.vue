<template>
  <a-config-provider>
    <a-watermark :content="[textWatermark, dayjs().format('YYYY-MM-DDTHH:mm:ss')]" :gap="[30, 20]"
      :font="{ fontSize: 8, fontWeight: 'light', fontStyle: 'italic' }">
      <main class="w-screen h-screen p-1 py-2 select-none">
        <div class="w-full h-full flex flex-col justify-between items-center p-1">
            <ComponentFormTask />
            <div class="w-full mt-2">
              <a-typography-title :level="5">常见问题</a-typography-title>
              <a-typography-paragraph>1. 模型下载和安装：点击<a-typography-link href="https://huggingface.co/ggerganov/whisper.cpp/tree/main" target="_blank">此处</a-typography-link>下载模型，把下载好的模型放到软件目录中的models目录，在软件中，右键刷新即可；</a-typography-paragraph>
              <a-typography-paragraph class="!mb-0">2. 问题反馈：点击<a-typography-link href="https://github.com/speauty/titmouse/issues/new" target="_blank">此处</a-typography-link>进行反馈，也可发送邮件到<a-typography-paragraph class="inline" type="success" copyable>speauty@163.com</a-typography-paragraph>，注意问题描述和截图；</a-typography-paragraph>
            </div>
        </div>
      </main>
    </a-watermark>
  </a-config-provider>
</template>

<script setup lang="ts">
import dayjs from 'dayjs'
import ComponentFormTask from '@/components/FormTask.vue'
import { onMounted, onUnmounted } from 'vue'
import { listenForMin } from './libs/listeners'
import { UnlistenFn } from '@tauri-apps/api/event'
const textWatermark: string = import.meta.env.VITE_WATERMARK ?? 'TS'
let fnUnlistentForMin: UnlistenFn|null

onMounted(async () => {
  if (fnUnlistentForMin) fnUnlistentForMin()
  fnUnlistentForMin = await listenForMin()
})

onUnmounted(() => {
  if (fnUnlistentForMin) fnUnlistentForMin()
})

</script>