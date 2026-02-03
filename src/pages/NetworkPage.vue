<template>
  <div class="page">
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
  gap: 12px;
  height: 100%;
  min-height: 0;
}

.card {
  padding: 18px;
  border-radius: var(--radius-lg);
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.cardHeader {
  font-size: 12px;
  letter-spacing: 0.06em;
  text-transform: uppercase;
  color: var(--muted);
  font-weight: 700;
  line-height: 1.3;
}

.contentBox {
  border: 1px solid color-mix(in oklab, var(--border), transparent 15%);
  border-radius: var(--radius-md);
  padding: 16px;
  background: color-mix(in oklab, var(--surface-raised), transparent 8%);
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.lead {
  font-size: 16px;
  font-weight: 600;
  line-height: 1.35;
  color: color-mix(in oklab, var(--text), var(--muted) 8%);
}

.muted {
  color: var(--muted);
  line-height: 1.6;
  max-width: 72ch;
}
</style>
