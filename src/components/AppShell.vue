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
  height: 100%;
  width: 100%;

  display: grid;
  grid-template-columns: 210px 1fr;
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
  padding: 10px;
  border-radius: var(--radius-lg);
}

.leftTop {
  flex: 0 0 auto;
  margin-bottom: 10px;
}

.leftNav {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
}

.leftFooter {
  display: flex;
  align-items: center;
  justify-content: center;
  margin-top: 8px;
  padding-top: 10px;
  border-top: 1px solid color-mix(in oklab, var(--border), transparent 18%);
  font-size: 12px;
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
  padding: 14px 16px;
  border-bottom: 1px solid var(--border);
}

.headerTitle {
  font-size: clamp(24px, 2.2vw, 30px);
  font-weight: 700;
  letter-spacing: 0.03em;
  font-kerning: none;
  line-height: 1.12;
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

@media (max-width: 980px) {
  .shell {
    grid-template-columns: 180px 1fr;
    gap: 10px;
    padding: 10px;
  }

  .headerTitle {
    font-size: 22px;
  }
}
</style>
