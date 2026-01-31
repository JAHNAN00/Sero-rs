<script setup lang="ts">
import { openUrl } from "@tauri-apps/plugin-opener";

const props = defineProps<{
  name?: string;
  href: string;
}>();

async function onClick() {
  await openUrl(props.href);
}
</script>

<template>
  <span class="brand" @click="onClick" :title="props.href">
    {{ props.name ?? "Sero" }}
  </span>
</template>

<style scoped>
.brand {
  font-size: 13px;
  font-weight: 700;
  letter-spacing: 0.4px;

  color: var(--muted);
  cursor: pointer;
  user-select: none;

  /* 让它更像“署名”，而不是按钮 */
  opacity: 0.9;
  transition: color 0.16s ease, opacity 0.16s ease;
}

.brand:hover {
  color: var(--text);
  opacity: 1;
  text-decoration: underline;
  text-underline-offset: 3px;
}

.brand:active {
  opacity: 0.7;
}
</style>
