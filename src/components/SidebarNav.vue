<script setup lang="ts">
export type NavItem = {
  key: string;
  label: string;
  subtitle?: string;
  icon?: string;
};

const props = defineProps<{
  modelValue: string;
  items: NavItem[];
  title?: string;
}>();

const emit = defineEmits<{
  (e: "update:modelValue", v: string): void;
}>();

function select(key: string) {
  emit("update:modelValue", key);
}
</script>

<template>
  <div class="nav">
    <div class="navTitle">{{ props.title ?? "通道" }}</div>

    <button
      v-for="it in props.items"
      :key="it.key"
      class="navItem"
      :class="{ active: props.modelValue === it.key }"
      @click="select(it.key)"
      :title="it.subtitle ?? it.label"
    >
      <div class="navLeft">
        <div class="navIcon">{{ it.icon ?? "•" }}</div>
        <div class="navLabel">{{ it.label }}</div>
      </div>
      <div class="chev">›</div>
    </button>
  </div>
</template>


<style scoped>
.nav {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.navTitle {
  font-size: 12px;
  font-weight: 700;
  color: var(--muted);
  padding: 4px 4px 2px 4px;
  letter-spacing: 0.06em;
  line-height: 1.3;
  text-transform: uppercase;
}

.navItem {
  width: 100%;
  max-width: 100%;
  min-width: 0;

  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 12px;
  padding: 12px;
  border-radius: 12px;
  border: 1px solid var(--border);
  background: color-mix(in oklab, var(--surface), var(--surface-2) 30%);

  cursor: pointer;

  color: var(--text);

  overflow: hidden;
  transition: border-color 0.2s ease, transform 0.08s ease, background 0.2s ease;
}

.navItem:hover {
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 45%);
  background: color-mix(in oklab, var(--surface-raised), var(--brand) 12%);
  transform: translateX(2px);
}

.navItem:active {
  transform: translateY(1px);
}

.navItem.active {
  border-color: color-mix(in oklab, var(--brand), var(--border) 28%);
  background: color-mix(in oklab, var(--surface-raised), var(--brand) 18%);
}

.navLeft {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.navIcon {
  width: 24px;
  height: 24px;
  display: grid;
  place-items: center;
  border-radius: 7px;
  background: color-mix(in oklab, var(--surface-2), var(--surface) 52%);
  border: 1px solid var(--border);
  color: color-mix(in oklab, var(--muted), var(--text) 15%);
  flex: 0 0 auto;
}

.navLabel {
  font-weight: 600;
  font-size: 14px;
  line-height: 1.28;
  letter-spacing: 0.03em;
  font-kerning: none;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.chev {
  color: var(--muted);
  font-size: 16px;
  transform: translateY(-1px);
  flex: 0 0 auto;
}
.navItem.active .navIcon {
  border-color: color-mix(in oklab, var(--brand), var(--border) 45%);
  color: color-mix(in oklab, var(--brand), var(--text) 25%);
}
</style>
