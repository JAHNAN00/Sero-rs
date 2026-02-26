# src-tauri/src 架构说明（前端关注版）

本文档面向前端开发者，目标是让你不用通览后端 Rust 代码，也能快速了解：
- 前端可以调用哪些后端能力（Tauri commands）
- 前端可以订阅哪些事件（Tauri events）
- 事件里数据的结构长什么样

如果只看一个目录，请看：`src-tauri/src/api/`。

---

## 1. 目录结构速览

`src-tauri/src/` 是后端 Rust 代码目录，前端开发只需关注它对外提供的 API：
- `src-tauri/src/api/commands.rs`：可调用的命令
- `src-tauri/src/api/events.rs`：事件命名规则
- `src-tauri/src/core/types.rs`：事件数据结构

---

## 2. 前端可调用的后端命令（Tauri Commands）

所有可调用命令都在：`src-tauri/src/api/commands.rs`

### 2.1 命令清单
- `list_sources()` → 获取数据源列表
- `list_parsers()` → 获取可用解析器/阶段
- `start_source(source_id)` → 启动数据源
- `stop_source(source_id)` → 停止数据源
- `attach_pipeline(source_id, pipeline_id)` → 为数据源绑定管线
- `mock_rx(source_id, text)` → 生成测试数据并走完整数据流（示例用）

### 2.2 前端调用示例
```ts
import { invoke } from "@tauri-apps/api/core";

await invoke("list_sources");
await invoke("start_source", { sourceId: "serial" });
await invoke("mock_rx", { sourceId: "serial", text: "temp=23.5 v=3.3" });
```

---

## 3. 前端可订阅的事件（Tauri Events）

事件命名在：`src-tauri/src/api/events.rs`

### 3.1 事件规则
- `data_stream::<source_id>`：原始数据流事件
- `metrics::<pipeline_id>`：解析后的指标事件

例子：
- `data_stream::serial`
- `data_stream::rtt`
- `metrics::serial_demo`
- `metrics::rtt_demo`

### 3.2 前端订阅示例
```ts
import { listen } from "@tauri-apps/api/event";

await listen("data_stream::serial", (event) => {
  console.log("raw packet", event.payload);
});

await listen("metrics::serial_demo", (event) => {
  console.log("metric", event.payload);
});
```

---

## 4. 事件数据结构（后端发给前端的格式）

结构定义在：`src-tauri/src/core/types.rs`

### 4.1 DataPacket（原始数据）
```ts
{
  ts_millis: number,
  source_id: string,
  raw: number[],
  text?: string,
  tags: string[]
}
```

### 4.2 Metric（解析指标）
```ts
{
  ts_millis: number,
  source_id: string,
  name: string,
  value: number
}
```

### 4.3 ParsedEvent（结构化事件）
```ts
{
  ts_millis: number,
  kind: string,
  payload: any
}
```

---

## 5. 实际开发最常用的前端调用流程

1) 订阅数据流事件
```ts
await listen("data_stream::serial", (event) => {
  // UI 日志显示
});
```

2) 点击按钮触发 mock（开发期调试）
```ts
await invoke("mock_rx", { sourceId: "serial", text: "temp=23.5" });
```

3) 订阅 metrics 事件绘图
```ts
await listen("metrics::serial_demo", (event) => {
  // event.payload.value 画折线图
});
```

---

## 6. 如果你只想知道“能做什么”

优先只看下面三个文件：
- `src-tauri/src/api/commands.rs` → 命令
- `src-tauri/src/api/events.rs` → 事件
- `src-tauri/src/core/types.rs` → 数据结构

这三个文件就是“前端的对外 API 文档”。
111