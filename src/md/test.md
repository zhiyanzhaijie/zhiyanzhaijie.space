---
title: dioxus site experience
date: 2025-06-11
slug: dioxus_site-exp
tags: ["dioxus"]
---

Hey there.

## How this site work2

Thanks to framework and works followed:
1. Dioxus (a cross-plateform ui framework writed in Rust)
2. TailwindCSS + Iconify (easily use css tool and free icon web site)
3. Markdown.rs (a md comparser writed in rust)
```rust
use dioxus::prelude::*;

#[component]
pub fn TestHear(label: String) -> Element {
    let mut count = use_signal(|| 0);

    rsx! {
        button {
            class: "bg-blue-500 hover:bg-blue-700 text-foreground font-bold py-2 px-4 rounded",
            onclick: move |_| {
                count += 10;
            },
            {format!("{} (点击次数: {})", label, count())}
        }
    }
}
```
and there is the result：
[testhear](interactive:TestHear)


```js
let a = 1;
let b = 3;
for (let i = 0; i < 100; i ++) {
  a += b
}

console.log(a)
```

Happy for using.
