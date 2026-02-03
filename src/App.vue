<script setup lang="ts">
import { computed, ref } from "vue";
import { useSerial } from "./composables/useSerial";

import AppShell from "./components/AppShell.vue";
import SerialToggleButton from "./components/SerialToggleButton.vue";
import SidebarNav, { type NavItem } from "./components/SidebarNav.vue";
import BrandLink from "./components/BrandLink.vue";

import SerialPage from "./pages/SerialPage.vue";
import RttPage from "./pages/RttPage.vue";
import NetworkPage from "./pages/NetworkPage.vue";

type PageKey = "serial" | "rtt" | "tcp";

const items: NavItem[] = [
  { key: "serial", label: "Serial", subtitle: "Serial Monitor", icon: "⌁" },
  { key: "rtt", label: "RTT", subtitle: "SEGGER RTT", icon: "⇄" },
  { key: "tcp", label: "Network", subtitle: "Socket", icon: "☍" },
];
const active = ref<PageKey>("serial");

const { opened, busy, statusText, toggle } = useSerial();

const pageTitle = computed(() => {
  switch (active.value) {
    case "serial":
      return "Serial Monitor";
    case "rtt":
      return "RTT Monitor";
    case "tcp":
      return "Network";
  }
});

const CurrentPage = computed(() => {
  switch (active.value) {
    case "serial":
      return SerialPage;
    case "rtt":
      return RttPage;
    case "tcp":
      return NetworkPage;
  }
});
</script>

<template>
  <AppShell :title="pageTitle">
    <template #left-top>
      <SerialToggleButton :opened="opened" :busy="busy" :status-text="statusText" @toggle="toggle" />
    </template>

    <template #left-nav>
      <SidebarNav v-model="active" :items="items" title="Channels" />
    </template>

    <template #left-footer>
      <div class="leftFooter">
        <BrandLink name="Sero" href="https://github.com/JAHNAN00/Sero-rs" />
      </div>
    </template>

    <component :is="CurrentPage" :opened="opened" />
  </AppShell>
</template>
