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

  /* 关键：用 grid 固定左列 + 右侧自适应 */
  display: grid;
  grid-template-columns: 160px 1fr;
  grid-template-rows: 1fr;
  gap: 14px;

  padding: 14px;
  overflow: hidden;
}

/* 左侧整列 */
.left {
  display: flex;
  flex-direction: column;
  min-width: 0;
  height: 100%;
  padding: 10px;            /* 原来 12px，改小一点 */
  border-radius: var(--radius-lg);
}

/* 顶部按钮容器：不伸缩 */
.leftTop {
  flex: 0 0 auto;
  margin-bottom: 8px;   /* 原来 10px，可再紧一点 */
}



/* 导航容器：占剩余高度 */
/* .leftNav {
  flex: 1 1 auto;
  min-height: 0;
  overflow: auto;
  padding-right: 4px;
} */
.leftNav {
  flex: 1 1 auto;
  min-height: 0;     /* ✅ flex 容器滚动的关键 */
  overflow: auto;    /* ✅ 内容多时滚动 */
  padding-right: 4px;
}


/* 底部说明：不伸缩 */
.leftFooter {
  margin-top: 8px;
  padding-top: 8px;
  font-size: 11px;
}


/* 右侧内容区域：占满剩余空间 */
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
  font-size: 16px;
  font-weight: 600;
  letter-spacing: 0.2px;
}

.content {
  flex: 1 1 auto;
  min-height: 0;
  padding: 16px;
  overflow: auto;
}

</style>
