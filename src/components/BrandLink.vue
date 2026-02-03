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
  font-weight: 600;
  letter-spacing: 0.05em;
  text-transform: uppercase;

  color: var(--muted);
  cursor: pointer;
  user-select: none;
  opacity: 0.9;
  transition: color 0.16s ease, opacity 0.16s ease, letter-spacing 0.16s ease;
}

.brand:hover {
  color: color-mix(in oklab, var(--brand), var(--text) 48%);
  opacity: 1;
  letter-spacing: 0.08em;
}

.brand:active {
  opacity: 0.7;
}
</style>
