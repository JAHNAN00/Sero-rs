<script setup lang="ts">
import { ref } from "vue";

const props = defineProps<{
  opened: boolean;
}>();

const logs = ref<string[]>(["Serial Monitor Ready.", "（这里将显示串口输出）"]);
const input = ref("");

function append(line: string) {
  const ts = new Date().toLocaleTimeString();
  logs.value.push(`[${ts}] ${line}`);
  if (logs.value.length > 800) logs.value.splice(0, logs.value.length - 800);
}

function clear() {
  logs.value = [];
}

function sendMock() {
  if (!input.value.trim()) return;
  append(`TX: ${input.value.trim()}`);
  input.value = "";
}
</script>

<template>
  <div class="page">
    <div class="row">
      <div>
        <div class="title">串口监视器</div>
        <div class="meta ui-badge">
          <span class="ui-dot" :class="{ ok: props.opened }"></span>
          <span>{{ props.opened ? "Opened" : "Closed" }}</span>
          <span class="sep">•</span>
          <span class="muted">UI 骨架已就绪，接下来接入后端事件流即可</span>
        </div>
      </div>

      <div class="actions">
        <button class="ui-btn" @click="clear">清空</button>
      </div>
    </div>

    <div class="panel ui-card ui-card-strong">
      <div class="panelTitle">输出</div>
      <div class="log ui-mono">
        <div v-if="logs.length === 0" class="empty">暂无内容</div>
        <div v-for="(l, i) in logs" :key="i" class="line">{{ l }}</div>
      </div>

      <div class="send">
        <input
          class="sendInput ui-mono"
          v-model="input"
          :disabled="!props.opened"
          :placeholder="props.opened ? '输入要发送的内容（UI 演示）' : '串口未打开'"
          @keydown.enter="sendMock"
        />
        <button class="ui-btn" :disabled="!props.opened" @click="sendMock">发送</button>
      </div>
    </div>

    <div class="hint">
      后续扩展建议：端口选择 / 波特率 / 自动滚动 / 过滤器 / 关键词高亮 / 协议解析插件
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 14px;
}

.row {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 12px;
}

.title {
  font-size: 18px;
  font-weight: 600;
  letter-spacing: 0.2px;
}

.meta {
  margin-top: 6px;
}

.sep {
  opacity: 0.5;
}

.muted {
  color: var(--muted);
}

.panel {
  padding: 14px;
  border-radius: var(--radius-lg);
}

.panelTitle {
  font-weight: 600;
  margin-bottom: 10px;
  color: var(--muted);
  font-size: 12px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
}

.log {
  height: 420px;
  overflow: auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 10px;
  background: color-mix(in oklab, var(--panel), transparent 15%);
}

.line {
  white-space: pre-wrap;
  padding: 2px 0;
  color: color-mix(in oklab, var(--text), var(--muted) 18%);
}

.empty {
  color: var(--muted);
  padding: 6px 0;
}

.send {
  margin-top: 12px;
  display: flex;
  gap: 10px;
}

.sendInput {
  flex: 1;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: color-mix(in oklab, var(--panel-2), transparent 6%);
  color: var(--text);
  padding: 10px 12px;
  outline: none;
}

.sendInput:focus {
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 55%);
}

.hint {
  font-size: 12px;
  color: var(--muted);
}
</style>
