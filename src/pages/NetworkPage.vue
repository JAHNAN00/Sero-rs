<template>
  <div class="page">
    <div class="title">Network</div>

    <div class="ui-card ui-card-strong card">
      <div class="cardHeader">Network Console</div>

      <div class="contentBox">
        <div class="lead">Reserved for the next network monitor redesign.</div>
        <div class="muted">
          We will add endpoint management, socket stream logs, and send/receive controls here.
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.page {
  display: flex;
  flex-direction: column;
  gap: 14px;
  height: 100%;
  min-height: 0;
}

.title {
  font-size: 18px;
  font-weight: 600;
}

.card {
  padding: 18px;
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.cardHeader {
  font-size: 13px;
  letter-spacing: 0.4px;
  text-transform: uppercase;
  color: var(--muted);
  font-weight: 700;
}

.contentBox {
  border: 1px solid color-mix(in oklab, var(--border), transparent 15%);
  border-radius: var(--radius-md);
  padding: 14px;
  background: color-mix(in oklab, var(--panel-2), transparent 8%);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.lead {
  font-size: 14px;
  font-weight: 600;
  color: color-mix(in oklab, var(--text), var(--muted) 12%);
}

.muted {
  color: var(--muted);
  line-height: 1.55;
  max-width: 72ch;
}
</style>
