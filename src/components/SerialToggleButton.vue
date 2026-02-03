<script setup lang="ts">
import { computed } from "vue";

const props = defineProps<{
  opened: boolean;
  busy?: boolean;
}>();

const emit = defineEmits<{
  (e: "toggle"): void;
}>();

const title = computed(() => {
  if (props.busy) return "Working...";
  return props.opened ? "Disconnect" : "Connect";
});

const stateText = computed(() => (props.opened ? "Disconnect" : "Connect"));
</script>

<template>
  <div class="toggleCard ui-card">
    <button
      class="toggleBtn"
      :class="{ busy: props.busy }"
      :disabled="props.busy"
      @click="emit('toggle')"
      :title="title"
      aria-label="toggle connection"
    >
      <span class="lampWrap">
        <span class="lamp" :class="{ on: props.opened }">
          <span v-if="props.busy" class="spinner" />
        </span>
      </span>
      <span class="textBlock">
        <span class="state">{{ stateText }}</span>
      </span>
    </button>
  </div>
</template>

<style scoped>
.toggleCard {
  width: 100%;
  padding: 4px;
}

.toggleBtn {
  width: 100%;
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 10px;
  border-radius: 10px;
  border: 1px solid transparent;
  background: transparent;
  color: var(--text);
  cursor: pointer;
  transition: border-color 0.18s ease, background 0.18s ease, transform 0.08s ease;
}

.toggleBtn:hover {
  border-color: color-mix(in oklab, var(--brand), var(--border) 45%);
  background: color-mix(in oklab, var(--surface-raised), var(--brand) 8%);
}

.toggleBtn:active {
  transform: translateY(1px);
}

.toggleBtn:disabled {
  cursor: not-allowed;
  opacity: 0.75;
}

.lampWrap {
  width: 28px;
  height: 28px;
  border-radius: 999px;
  display: grid;
  place-items: center;
  background: color-mix(in oklab, var(--surface-raised), transparent 28%);
  border: 1px solid color-mix(in oklab, var(--border), transparent 20%);
  flex: 0 0 auto;
}

.lamp {
  position: relative;
  width: 11px;
  height: 11px;
  border-radius: 999px;
  background: color-mix(in oklab, var(--warn), white 10%);
  box-shadow: 0 0 0 4px color-mix(in oklab, var(--warn), transparent 82%);
}

.lamp.on {
  background: color-mix(in oklab, var(--ok), white 12%);
  box-shadow: 0 0 0 4px color-mix(in oklab, var(--ok), transparent 80%);
}

.textBlock {
  flex: 1;
  min-width: 0;
  display: flex;
  align-items: center;
  justify-content: center;
  padding-right: 2px;
  min-height: 24px;
}

.state {
  display: inline-block;
  width: 10ch;
  white-space: nowrap;
  font-size: 14px;
  font-weight: 700;
  line-height: 1;
  letter-spacing: 0.01em;
}

.spinner {
  position: absolute;
  inset: -3px;
  border-radius: 999px;
  border: 1px solid color-mix(in oklab, var(--muted), transparent 28%);
  border-top-color: color-mix(in oklab, var(--text), transparent 8%);
  animation: spin 0.6s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
