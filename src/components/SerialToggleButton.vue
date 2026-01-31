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
</script>

<template>
  <!-- plate：独特底板（点击范围） -->
  <div class="plate ui-card ui-card-strong">
    <button
      class="hit"
      :class="{ busy: props.busy }"
      :disabled="props.busy"
      @click="emit('toggle')"
      :title="title"
      aria-label="toggle connection"
    >
      <span class="lamp" :class="{ on: props.opened }">
        <span v-if="props.busy" class="spinner" />
      </span>
      <span class="text">{{ props.opened ? "ON" : "OFF" }}</span>
    </button>
  </div>
</template>

<style scoped>
/* 独特底板：像一个“槽位”，和导航项完全不一样 */
.plate {
  width: 100%;
  max-width: 100%;
  min-width: 0;

  /* 更紧凑的高度 */
  padding: 8px 10px;
  border-radius: var(--radius-lg);

  border: 1px solid var(--border);
  background: linear-gradient(
      180deg,
      color-mix(in oklab, var(--panel-2), transparent 10%),
      color-mix(in oklab, var(--panel), transparent 25%)
    );
  box-shadow: var(--shadow-soft);

  /* 视觉上更像独立模块 */
  position: relative;
  overflow: hidden;
  
}

/* plate 内部加一点“内凹槽”的感觉 */
.plate::before {
  content: "";
  position: absolute;
  inset: 6px;
  border-radius: calc(var(--radius-lg) - 8px);
  border: 1px solid color-mix(in oklab, var(--border), transparent 25%);
  background: color-mix(in oklab, var(--panel), transparent 25%);
  opacity: 0.7;
  pointer-events: none;
}

/* 真正可点击区域：无白框、无边框、透明 */
.hit {
  position: relative;
  z-index: 1;

  width: 100%;
  max-width: 100%;
  min-width: 0;

  display: flex;
  align-items: center;
  justify-content: center;
  gap: 10px;

  padding: 10px 10px; /* 点击范围靠 padding，不靠边框 */
  border-radius: calc(var(--radius-lg) - 6px);

  border: none;            /* ✅ 去掉按钮自带框 */
  background: transparent; /* ✅ 去掉按钮自带底色 */
  color: var(--text);

  cursor: pointer;

  transition: transform 0.05s ease, background 0.12s ease;
}

/* hover 只改变“槽位内”的高亮，不出现按钮框 */
.hit:hover {
  background: color-mix(in oklab, var(--panel-2), transparent 55%);
}

.hit:active {
  transform: translateY(1px);
}

.hit:disabled {
  cursor: not-allowed;
  opacity: 0.75;
}

/* 灯：OFF 更明显（橙） */
.lamp {
  position: relative;
  width: 16px;
  height: 16px;
  border-radius: 999px;

  border: 1px solid color-mix(in oklab, var(--warn), var(--border) 40%);
  background: radial-gradient(
    circle at 35% 30%,
    color-mix(in oklab, #fff, var(--warn) 20%),
    color-mix(in oklab, var(--warn), black 10%) 55%,
    color-mix(in oklab, black, var(--panel-2) 70%) 100%
  );

  box-shadow:
    0 0 0 5px color-mix(in oklab, var(--warn), transparent 82%),
    0 0 22px color-mix(in oklab, var(--warn), transparent 50%);
  transition: transform 0.05s ease, box-shadow 0.12s ease, background 0.12s ease, border-color 0.12s ease;
}

.lamp.on {
  border-color: color-mix(in oklab, var(--ok), var(--border) 35%);
  background: radial-gradient(
    circle at 35% 30%,
    color-mix(in oklab, #fff, var(--ok) 18%),
    color-mix(in oklab, var(--ok), black 10%) 55%,
    color-mix(in oklab, black, var(--panel-2) 70%) 100%
  );
  box-shadow:
    0 0 0 6px color-mix(in oklab, var(--ok), transparent 82%),
    0 0 24px color-mix(in oklab, var(--ok), transparent 50%);
}

.hit:hover .lamp {
  transform: scale(1.06);
}

/* 文案：小而清晰 */
.text {
  font-size: 12px;
  font-weight: 700;
  letter-spacing: 0.6px;
  color: color-mix(in oklab, var(--text), var(--muted) 18%);
}

/* Busy spinner：小且快 */
.spinner {
  position: absolute;
  inset: -4px;
  border-radius: 999px;
  border: 2px solid color-mix(in oklab, var(--muted), transparent 35%);
  border-top-color: color-mix(in oklab, var(--text), transparent 10%);
  animation: spin 0.65s linear infinite;
}

@keyframes spin {
  to {
    transform: rotate(360deg);
  }
}
</style>
