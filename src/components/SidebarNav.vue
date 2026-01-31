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
  gap: 8px;
}

.navTitle {
  font-size: 12px;
  font-weight: 600;
  color: var(--muted);
  padding: 6px 6px 2px 6px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
}

.navItem {
  width: 100%;
  max-width: 100%;
  min-width: 0;

  display: flex;
  align-items: center;
  justify-content: space-between;
  gap: 10px;

  padding: 10px 12px;

  border-radius: var(--radius-lg);
  border: 1px solid var(--border);
  background: var(--panel);

  cursor: pointer;

  /* 关键：强制按钮文字跟随主题色 */
  color: var(--text);

  overflow: hidden;
  transition: border-color 0.2s ease, transform 0.06s ease, background 0.2s ease;
}

.navItem:hover {
  border-color: color-mix(in oklab, var(--brand-2), var(--border) 55%);
  background: color-mix(in oklab, var(--panel), var(--panel-2) 35%);
}

.navItem:active {
  transform: translateY(1px);
}

.navItem.active {
  border-color: color-mix(in oklab, var(--brand), var(--border) 35%);
  background: color-mix(in oklab, var(--panel-2), var(--brand) 10%);
}

.navLeft {
  display: flex;
  align-items: center;
  gap: 10px;
  min-width: 0;
}

.navIcon {
  width: 22px;
  height: 22px;
  display: grid;
  place-items: center;
  border-radius: 8px;
  background: color-mix(in oklab, var(--panel), var(--panel-2) 50%);
  border: 1px solid var(--border);
  color: var(--muted);
  flex: 0 0 auto;
}

.navLabel {
  font-weight: 600;
  white-space: nowrap;
  overflow: hidden;
  text-overflow: ellipsis;
}

.chev {
  color: var(--muted);
  font-size: 18px;
  transform: translateY(-1px);
  flex: 0 0 auto;
}


</style>
