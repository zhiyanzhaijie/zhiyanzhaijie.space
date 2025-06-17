---
title: 深入探索 Dioxus
date: 2024-02-20
slug: exploring-dioxus
tags: ["dioxus"]
---

在上一篇文章中，我简单介绍了 Dioxus。现在，让我们更深入地了解它的一些核心概念和交互能力。

## 组件和 Props ma1

就像 React 一样，Dioxus 的 UI 由可重用的组件构建。你可以传递数据（props）给子组件。

### 交互式组件示例

这是一个通过 Markdown 渲染的交互式按钮示例：[点我计数！](interactive:increment_counter)

试试这个颜色选择器：[选择颜色](interactive:color_picker?color=%23ff5500)

```rust
// 计数器组件示例
#[component]
fn InteractiveCounterButton(label: String) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            onclick: move |_| {
                count += 1;
            },
            "{label} (点击次数: {count})"
        }
    }
}
```

```rust
use crate::{AppTheme, ACTIVE_THEME};
use dioxus::prelude::*;

#[component]
pub fn ThemeSwitcher() -> Element {
    let current_theme = ACTIVE_THEME.read();

    rsx! {
        div {
            class: "theme-switcher p-4",
            span { class: "mr-2", "Select Theme:" }
            select {
                class: "p-2 rounded border bg-gray-100 dark:bg-gray-700 dark:text-white focus:ring-2",
                oninput: move |event| {
                    let new_theme_str = event.value();
                    let new_theme = match new_theme_str.as_str() {
                        "light" => AppTheme::Light,
                        "dark" => AppTheme::Dark,
                        _ => AppTheme::default(), // 安全回退
                    };

                    *ACTIVE_THEME.write() = new_theme; // 更新全局主题状态
                    log::info!("Theme changed to: {}", new_theme_str);
                    // (可选) 将选择的主题保存到 localStorage
                    #[cfg(target_arch = "wasm32")]
                    {
                        let window = web_sys::window().expect("no global `window` exists");
                        let storage = window.local_storage().expect("no local storage").expect("local storage is not available");
                        if let Err(e) = storage.set_item("app_theme", &new_theme_str) {
                        }
                    }
                },
                option { value: "light", selected: *current_theme == AppTheme::Light, "Light" }
                option { value: "dark", selected: *current_theme == AppTheme::Dark, "Dark" }
            }
        }
    }
}

```


## 代码运行演示

下面是一个简单的Rust代码示例，你可以点击按钮运行它：

[运行代码](interactive:code_runner?language=rust&code=fn%20main()%20%7B%0A%20%20println!(%22Hello%2C%20Dioxus!%22)%3B%0A%20%20%0A%20%20let%20numbers%20%3D%20vec![1%2C%202%2C%203%2C%204%2C%205]%3B%0A%20%20let%20sum%3A%20i32%20%3D%20numbers.iter().sum()%3B%0A%20%20println!(%22Sum%3A%20%7B%7D%22%2C%20sum)%3B%0A%7D)

## 状态管理

Dioxus 提供了多种状态管理方案，从简单的 `use_signal` 到更复杂的全局状态管理，使得构建交互式应用变得简单。

```rust
// 基础的状态管理
let mut count = use_signal(|| 0);  // 创建一个可变信号

// 更新状态
count += 10;

// 读取状态
let current = *count.get();
```

## 路由

`dioxus-router` 使得在 Dioxus 应用中添加客户端或服务器端路由变得轻而易举。路由定义非常直观：

```rust
#[derive(Clone, Routable)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/blog/:slug")]
    BlogPost { slug: String },
    #[route("/about")]
    About {},
}
```

总的来说，Dioxus 的设计哲学和强大的功能集给我留下了深刻的印象。通过将交互组件嵌入Markdown内容，我们可以创建更加生动和有教育意义的技术博客。
