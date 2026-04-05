use dioxus::prelude::*;
use dioxus_markdown::CustomComponents;

#[component]
pub fn CodeRunnerComponent(code: String, language: String) -> Element {
    let mut output = use_signal(|| String::new());
    let mut is_running = use_signal(|| false);

    let code_clone = code.clone();
    let language_clone = language.clone();

    let run_code = move |_| {
        is_running.set(true);
        output.set(format!(
            "执行 {} 代码:\n{}\n\n输出: 模拟执行结果",
            language_clone, code_clone
        ));
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

pub fn registe_md_comp(components: &mut CustomComponents) {
    components.register("CodeRunner", |props| {
        let code = props.get("code").unwrap_or_default();
        let language = props.get("language").unwrap_or_else(|| "rust".to_string());
        Ok(rsx! {
            CodeRunnerComponent {
                code,
                language,
            }
        })
    });
}
