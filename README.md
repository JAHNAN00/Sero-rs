# Tauri + Vue + TypeScript

This template should help get you started developing with Vue 3 and TypeScript in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

# 记事本

## **1. 安装依赖**

```bash
pnpm install
```

确保前端依赖和 Tauri CLI 都安装好了。

---

## **2. 运行开发环境（带 Tauri 桌面窗口）**

```bash
pnpm tauri dev
```

* 这个命令会执行 `tauri` CLI 的 `dev` 命令
* 会先启动 **Vite 前端开发服务器**
* 然后启动 **Tauri 桌面窗口**，加载前端页面
* 你可以看到默认 Tauri 示例界面（带“Greet”按钮等）

---

## **3. 仅运行前端 Vite 页面（可选）**

如果你只是想测试 Vue 页面而不启动 Tauri 窗口：

```bash
pnpm dev
```

* 打开浏览器访问 Vite 提供的地址（默认 `http://localhost:5173`）

---

## **4. 打包桌面应用**

```bash
pnpm tauri build
```

* 会编译 Rust 后端和前端，生成可执行文件
* 输出在 `src-tauri/target/release` 或 `dist/` 下，视平台而定

---

✅ **验证成功**：

1. `pnpm tauri dev` 后看到桌面窗口
2. 点击默认按钮可以触发提示（例如 greet 命令）

---

