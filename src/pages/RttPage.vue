<template>
  <div class="page">
    <div class="title">RTT</div>
    <div class="ui-card ui-card-strong card">
      <div class="muted">预留页面：后续接入 probe-rs / JLink / DAPLink RTT</div>
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 14px;
}
.title {
  font-size: 18px;
  font-weight: 600;
}
.card {
  padding: 14px;
  border-radius: var(--radius-lg);
}
.muted {
  color: var(--muted);
}
</style>
