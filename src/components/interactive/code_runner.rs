use dioxus::prelude::*;

#[component]
pub fn CodeRunnerComponent(code: String, language: String) -> Element {
    let mut output = use_signal(|| String::new());
    let mut is_running = use_signal(|| false);

    // 克隆用于闭包的变量
    let code_clone = code.clone();
    let language_clone = language.clone();

    let run_code = move |_| {
        // 实际应用中，这里可能需要调用外部API或WebAssembly执行代码
        // 这里只是一个模拟实现
        is_running.set(true);
        output.set(format!(
            "执行 {} 代码:\n{}\n\n输出: 模拟执行结果",
            language_clone, code_clone
        ));
        // 在真实应用中应该用异步操作
        is_running.set(false);
    };

    rsx! {
        div {
            class: "code-runner",
            pre {
                code {
                    class: format!("language-{}", language),
                    "{code}"
                }
            }
            button {
                disabled: is_running(),
                onclick: run_code,
                {if is_running() { "执行中..." } else { "运行代码" }}
            }

            div {
                class: "output",
                if !output().is_empty() {
                    pre {
                        {output()}
                    }
                }
            }
        }
    }
}