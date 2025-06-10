use dioxus::prelude::*;
use markdown::ParseOptions;

use crate::components::interactive::code_runner::CodeRunnerComponent;
use crate::components::interactive::color_picker::ColorPickerComponent;
use crate::components::interactive::counter::InteractiveCounterButton;
use crate::components::markdown::node::RenderMdastNode;
use crate::components::markdown::registry::ComponentRegistry;
use crate::register_component;

#[component]
pub fn MarkdownRenderer(content: String) -> Element {
    // 为每篇文章创建一个组件注册表
    let registry = ComponentRegistry::new();

    // 注册默认组件
    register_component!(registry, "increment_counter", |params| {
        let label = params
            .get("label")
            .unwrap_or(&"Counter".to_string())
            .to_string();
        rsx! {
            InteractiveCounterButton { label: label }
        }
    });

    // 注册颜色选择器组件
    register_component!(registry, "color_picker", |params| {
        let init_color = params
            .get("color")
            .unwrap_or(&"#ff0000".to_string())
            .to_string();
        rsx! {
            ColorPickerComponent { initial_color: init_color }
        }
    });

    // 注册代码执行器组件
    register_component!(registry, "code_runner", |params| {
        let code = params.get("code").unwrap_or(&"".to_string()).to_string();
        let language = params
            .get("language")
            .unwrap_or(&"rust".to_string())
            .to_string();
        rsx! {
            CodeRunnerComponent {
                code: code,
                language: language
            }
        }
    });

    // 解析Markdown
    let parse_options = ParseOptions::gfm();
    match markdown::to_mdast(&content, &parse_options) {
        Ok(ast_node) => {
            rsx! {
                div { class: "markdown-body",
                    RenderMdastNode {
                        node: ast_node,
                        registry: registry
                    }
                }
            }
        }
        Err(_) => {
            rsx! {
                div { class: "error", "Markdown解析错误" }
            }
        }
    }
}
