<script setup lang="ts">
import { nextTick, onBeforeUnmount, onMounted, ref } from "vue";

type LogKind = "info" | "tx" | "rx";

type LogItem = {
  time: string;
  channel: string;
  text: string;
  kind: LogKind;
};

const props = defineProps<{
  opened: boolean;
}>();

function currentTime() {
  return new Date().toLocaleTimeString();
}

const logs = ref<LogItem[]>([]);
const input = ref("");
const channel = ref("0");
const channels = ["0", "1", "2"] as const;
const channelOpen = ref(false);
const autoScroll = ref(true);
const showTimestamp = ref(true);
const showChannelTag = ref(false);
const showRxTxTag = ref(true);
const appendNull = ref(false);
const appendLf = ref(false);
const appendCr = ref(false);
const chipModel = ref("STM32F103C8T6");
const logRef = ref<HTMLElement | null>(null);
const channelRef = ref<HTMLElement | null>(null);

function scrollToBottom() {
  if (!autoScroll.value) return;
  nextTick(() => {
    const el = logRef.value;
    if (!el) return;
    el.scrollTop = el.scrollHeight;
  });
}

function append(text: string, kind: LogKind) {
  logs.value.push({
    time: currentTime(),
    channel: channel.value,
    text,
    kind,
  });
  if (logs.value.length > 1000) logs.value.splice(0, logs.value.length - 1000);
  scrollToBottom();
}

function clear() {
  logs.value = [];
}

function sendMock() {
  if (!props.opened || !input.value.trim()) return;
  const suffix =
    (appendNull.value ? "\0" : "") +
    (appendCr.value ? "\r" : "") +
    (appendLf.value ? "\n" : "");
  const payload = `${input.value}${suffix}`;
  const displayPayload = payload
    .replace(/\0/g, "\\0")
    .replace(/\n/g, "\\n")
    .replace(/\r/g, "\\r");
  append(displayPayload, "tx");
}

function mockRx() {
  append("target heartbeat ok", "rx");
}

function toggleTimestamp() {
  showTimestamp.value = !showTimestamp.value;
}

function toggleChannelTag() {
  showChannelTag.value = !showChannelTag.value;
}

function toggleRxTxTag() {
  showRxTxTag.value = !showRxTxTag.value;
}

function toggleAutoScroll() {
  autoScroll.value = !autoScroll.value;
}

function toggleChannelMenu() {
  if (!props.opened) return;
  channelOpen.value = !channelOpen.value;
}

function selectChannel(v: string) {
  channel.value = v;
  channelOpen.value = false;
}

function closeChannelMenu() {
  channelOpen.value = false;
}

function onDocPointerDown(e: Event) {
  const target = e.target as Node | null;
  if (!target) return;
  if (!channelRef.value?.contains(target)) closeChannelMenu();
}

onMounted(() => {
  document.addEventListener("pointerdown", onDocPointerDown);
});

onBeforeUnmount(() => {
  document.removeEventListener("pointerdown", onDocPointerDown);
});
</script>

<template>
  <div class="page">
    <div class="row">
      <div class="meta ui-badge">
        <span class="ui-dot" :class="{ ok: props.opened }"></span>
        <span>{{ props.opened ? "Opened" : "Closed" }}</span>
      </div>

      <span class="topSep" aria-hidden="true"></span>

      <div class="chipRow">
        <span class="chipLabel">Chip</span>
        <input class="chipInput ui-mono" v-model="chipModel" placeholder="STM32F103C8T6" />
      </div>

      <div class="actions">
        <button
          class="ui-btn compactToggle"
          :class="{ active: showChannelTag }"
          :title="`Channel tag ${showChannelTag ? 'ON' : 'OFF'}`"
          aria-label="toggle channel tag"
          @click="toggleChannelTag"
        >
          Channel
        </button>
        <button
          class="ui-btn compactToggle"
          :class="{ active: showTimestamp }"
          :title="`Timestamp ${showTimestamp ? 'ON' : 'OFF'}`"
          aria-label="toggle timestamp"
          @click="toggleTimestamp"
        >
          Timestamp
        </button>
        <button
          class="ui-btn compactToggle"
          :class="{ active: showRxTxTag }"
          :title="`RX/TX tag ${showRxTxTag ? 'ON' : 'OFF'}`"
          aria-label="toggle rx tx tag"
          @click="toggleRxTxTag"
        >
          RX/TX
        </button>
      </div>
    </div>

    <div class="panel ui-card ui-card-strong">
      <div class="panelTitle">RTT Stream</div>
      <div ref="logRef" class="log ui-mono">
        <div v-if="logs.length === 0" class="empty">No RTT messages.</div>
        <div v-for="(l, i) in logs" :key="i" class="line">
          <span v-if="showTimestamp" class="ts">[{{ l.time }}]</span>
          <span v-if="showChannelTag" class="channelTag">[CH{{ l.channel }}]</span>
          <span
            v-if="showRxTxTag && l.kind !== 'info'"
            class="kindTag"
            :class="`kind-${l.kind}`"
          >
            {{ l.kind.toUpperCase() }}:
          </span>
          <span class="text">{{ l.text }}</span>
        </div>
      </div>

      <div class="send">
        <div ref="channelRef" class="channelPicker" :class="{ open: channelOpen }">
          <button class="channelTrigger ui-mono" :disabled="!props.opened" @click="toggleChannelMenu">
            <span>Channel {{ channel }}</span>
            <span class="triggerChevron">▾</span>
          </button>
          <div v-if="channelOpen" class="channelMenu">
            <button
              v-for="ch in channels"
              :key="ch"
              class="channelItem"
              :class="{ active: ch === channel }"
              @click="selectChannel(ch)"
            >
              Channel {{ ch }}
            </button>
          </div>
        </div>

        <input
          class="sendInput ui-mono"
          v-model="input"
          :disabled="!props.opened"
          :placeholder="props.opened ? 'Type command for RTT channel' : 'Open serial first'"
          @keydown.enter="sendMock"
        />

        <button class="ui-btn" :disabled="!props.opened" @click="sendMock">Send</button>
      </div>

      <div class="controlRow">
        <div class="controlLeft">
          <div class="suffixInline">
            <span class="suffixLabel">Append</span>
            <div class="suffixSegment" role="group" aria-label="append suffix">
              <button
                class="suffixSegBtn"
                :class="{ active: appendNull }"
                :disabled="!props.opened"
                @click="appendNull = !appendNull"
              >
                \0
              </button>
              <button
                class="suffixSegBtn"
                :class="{ active: appendCr }"
                :disabled="!props.opened"
                @click="appendCr = !appendCr"
              >
                \r
              </button>
              <button
                class="suffixSegBtn"
                :class="{ active: appendLf }"
                :disabled="!props.opened"
                @click="appendLf = !appendLf"
              >
                \n
              </button>
            </div>
          </div>

          <div class="autoInline">
            <span class="suffixLabel">Scroll</span>
            <div class="suffixSegment" role="group" aria-label="auto scroll">
              <button
                class="suffixSegBtn autoBtn"
                :class="{ active: autoScroll }"
                :aria-pressed="autoScroll"
                @click="toggleAutoScroll"
              >
                Auto
              </button>
            </div>
          </div>
        </div>

        <div class="controlRight">
          <button class="ui-btn ctrlBtn" @click="mockRx">Mock RX</button>
          <button class="ui-btn clearBtn" @click="clear">Clear</button>
        </div>
      </div>
    </div>

    <div class="hint">
      Status placeholder: runtime debug info will appear here (for example RTT connection failures).
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 14px;
  height: 100%;
  min-height: 0;
}

.row {
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  flex-wrap: nowrap;
  white-space: nowrap;
}

.meta {
  margin-top: 0;
}

.topSep {
  width: 1px;
  height: 18px;
  margin: 0 2px 0 6px;
  background: color-mix(in oklab, var(--border), transparent 14%);
  flex: 0 0 auto;
}

.chipRow {
  display: flex;
  align-items: center;
  gap: 6px;
  flex: 0 1 auto;
}

.chipLabel {
  color: var(--muted);
  font-size: 12px;
}

.chipInput {
  width: 165px;
  min-width: 120px;
  padding: 6px 8px;
}

.actions {
  display: flex;
  gap: 6px;
  margin-left: auto;
  flex: 0 0 auto;
}

.compactToggle {
  min-width: 0;
  padding: 6px 8px;
  font-size: 11px;
  font-weight: 700;
}

.ui-btn.active {
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 20%);
  color: color-mix(in oklab, var(--brand-2), var(--text) 20%);
  box-shadow: 0 0 0 3px color-mix(in oklab, var(--brand-2), transparent 82%);
}

.panel {
  padding: 14px;
  border-radius: var(--radius-lg);
  flex: 1 1 auto;
  min-height: 0;
  display: flex;
  flex-direction: column;
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
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  padding: 10px;
  background: color-mix(in oklab, var(--panel), transparent 15%);
}

.line {
  display: flex;
  align-items: baseline;
  flex-wrap: wrap;
  gap: 2px;
  white-space: pre-wrap;
  padding: 2px 0;
  color: color-mix(in oklab, var(--text), var(--muted) 18%);
}

.ts {
  color: #bda7ff;
  margin-right: 0;
}

.channelTag {
  color: #8fd3ff;
  margin-right: 0;
}

.kindTag {
  margin-right: 0;
  font-weight: 700;
}

.kind-tx {
  color: #f7b955;
}

.kind-rx {
  color: #63e6a9;
}

.text {
  color: color-mix(in oklab, var(--text), var(--muted) 12%);
}

.empty {
  color: var(--muted);
  padding: 6px 0;
}

.send {
  margin-top: 12px;
  display: flex;
  flex-wrap: wrap;
  gap: 10px;
}

.channelPicker {
  position: relative;
  width: 112px;
}

.channelTrigger {
  width: 100%;
  min-height: 42px;
  display: inline-flex;
  align-items: center;
  justify-content: space-between;
  gap: 8px;
  padding: 10px 12px;
  border: 1px solid var(--border);
  border-radius: var(--radius-md);
  background: color-mix(in oklab, var(--panel-2), black 18%);
  color: var(--text);
  cursor: pointer;
}

.channelTrigger:disabled {
  opacity: 0.75;
  cursor: not-allowed;
}

.triggerChevron {
  color: var(--muted);
  transition: transform 0.16s ease;
}

.channelPicker.open .triggerChevron {
  transform: rotate(180deg);
}

.channelTrigger:focus {
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 55%);
}

.channelMenu {
  position: absolute;
  bottom: calc(100% + 6px);
  left: 0;
  right: 0;
  z-index: 10;
  padding: 6px;
  border-radius: var(--radius-md);
  background: #1b2233;
  border: 1px solid color-mix(in oklab, var(--border), transparent 8%);
  box-shadow: var(--shadow-soft);
}

.channelItem {
  width: 100%;
  border: 1px solid transparent;
  border-radius: calc(var(--radius-md) - 4px);
  background: transparent;
  color: var(--text);
  text-align: left;
  padding: 8px 10px;
  cursor: pointer;
}

.channelItem:hover {
  background: color-mix(in oklab, var(--panel-2), transparent 12%);
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 72%);
}

.channelItem.active {
  background: color-mix(in oklab, var(--panel-2), var(--brand) 16%);
  border-color: color-mix(in oklab, var(--brand), var(--border) 52%);
}

@media (prefers-color-scheme: light) {
  .ts {
    color: #775fc7;
  }

  .channelTrigger {
    background: color-mix(in oklab, var(--panel-2), white 8%);
  }

  .channelMenu {
    background: #f2f5fb;
  }
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

.suffixInline {
  display: flex;
  align-items: center;
  gap: 8px;
}

.controlRow {
  margin-top: 8px;
  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 16px;
  flex-wrap: wrap;
}

.controlLeft,
.controlRight {
  display: flex;
  align-items: center;
  gap: 12px;
}

.autoInline {
  display: flex;
  align-items: center;
  gap: 8px;
}

.suffixLabel {
  color: var(--muted);
  font-size: 12px;
}

.suffixSegment {
  display: inline-flex;
  align-items: stretch;
  border: 1px solid color-mix(in oklab, var(--border), transparent 8%);
  border-radius: 999px;
  overflow: hidden;
  background: color-mix(in oklab, var(--panel-2), transparent 6%);
}

.suffixSegBtn {
  appearance: none;
  border: 0;
  border-left: 1px solid color-mix(in oklab, var(--border), transparent 14%);
  background: transparent;
  color: var(--muted);
  padding: 6px 12px;
  min-width: 44px;
  cursor: pointer;
}

.suffixSegBtn:first-child {
  border-left: 0;
}

.suffixSegBtn.active {
  color: color-mix(in oklab, var(--brand-2), var(--text) 20%);
  background: color-mix(in oklab, var(--brand-2), transparent 82%);
}

.suffixSegBtn:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.autoBtn {
  min-width: 62px;
}

.ctrlBtn {
  min-width: 90px;
}

.clearBtn {
  min-width: 84px;
}

.hint {
  font-size: 12px;
  color: var(--muted);
}
</style>
