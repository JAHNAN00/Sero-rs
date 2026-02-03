<script setup lang="ts">
defineProps<{
  title?: string;
}>();
</script>

<template>
  <div class="shell">
    <aside class="left ui-card ui-card-strong">
      <div class="leftTop">
        <slot name="left-top" />
      </div>

      <div class="leftNav">
        <slot name="left-nav" />
      </div>

      <div class="leftFooter">
        <slot name="left-footer" />
      </div>
    </aside>

    <main class="right">
      <div class="rightInner ui-card ui-card-strong">
        <div v-if="title" class="header">
          <div class="headerTitle">{{ title }}</div>
        </div>
        <div class="content">
          <slot />
        </div>
      </div>
    </main>
  </div>
</template>

<style scoped>
.shell {
  height: 100vh;
  width: 100vw;
  min-width: 800px;
  min-height: 500px;

  display: grid;
  grid-template-columns: 160px 1fr;
  grid-template-rows: 1fr;
  gap: 10px;

  padding: 10px;
  overflow: hidden;
}

.left {
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  padding: 8px;
  border-radius: var(--radius-lg);
}

.leftTop {
  flex: 0 0 auto;
  margin-bottom: 6px;
}

.leftNav {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding-right: 2px;
}

.leftFooter {
  margin-top: 6px;
  padding-top: 6px;
  font-size: 11px;
}

.right {
  min-width: 0;
  height: 100%;
  overflow: hidden;
}

.rightInner {
  height: 100%;
  width: 100%;
  border-radius: var(--radius-lg);
  overflow: hidden;
  display: flex;
  flex-direction: column;
}

.header {
  flex: 0 0 auto;
  padding: 12px 14px;
  border-bottom: 1px solid var(--border);
}

.headerTitle {
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0.2px;
}

.content {
  flex: 1 1 auto;
  min-height: 0;
  padding: 12px;
  overflow: hidden;
}

.content > :deep(.page) {
  height: 100%;
  min-height: 0;
}
</style>
