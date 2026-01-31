import { computed, ref } from "vue";
import { invoke } from "@tauri-apps/api/core";

export function useSerial() {
  const opened = ref(false);
  const busy = ref(false);

  const statusText = computed(() =>
    busy.value ? "处理中..." : opened.value ? "Opened" : "Closed"
  );

  async function toggle() {
    if (busy.value) return;

    busy.value = true;
    const next = !opened.value;

    try {
      if (next) {
        await invoke("serial_open");
      } else {
        await invoke("serial_close");
      }
      opened.value = next;
    } catch (err) {
      // 后端 command 未实现时，让 UI 先跑起来
      console.warn("[serial] backend not ready:", err);
      opened.value = next;
    } finally {
      busy.value = false;
    }
  }

  return { opened, busy, statusText, toggle };
}
