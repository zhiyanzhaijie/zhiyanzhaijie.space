use crate::components::markdown::registry::ComponentRegistry;
use crate::utils::url::percent_decode;
use dioxus::prelude::*;
use markdown::mdast;
use std::collections::HashMap;

use once_cell::sync::Lazy;
use syntect::html::{ClassStyle, ClassedHTMLGenerator};
use syntect::parsing::SyntaxSet;
use syntect::util::LinesWithEndings;

// 静态初始化 SyntaxSet 和 Theme
static SYNTAX_SET: Lazy<SyntaxSet> = Lazy::new(|| SyntaxSet::load_defaults_newlines());

/// 解析交互式链接的参数
pub fn parse_interactive_params(url: &str, link_text: String) -> (String, HashMap<String, String>) {
    let mut params = HashMap::new();

    // 添加链接文本作为默认标签
    params.insert("label".to_string(), link_text);

    // 移除 "interactive:" 前缀并分割组件名和参数
    let content = url.strip_prefix("interactive:").unwrap_or(url);
    let parts: Vec<&str> = content.splitn(2, '?').collect();

    let component_name = parts[0].to_string();

    // 解析参数（如果有）
    if parts.len() > 1 {
        for param_pair in parts[1].split('&') {
            let pair: Vec<&str> = param_pair.splitn(2, '=').collect();
            if pair.len() == 2 {
                // URL解码参数值
                let value = percent_decode(pair[1]);
                params.insert(pair[0].to_string(), value);
            }
        }
    }

    (component_name, params)
}

/// 渲染Markdown AST节点的组件
#[component]
pub fn RenderMdastNode(node: mdast::Node, registry: ComponentRegistry) -> Element {
    match node {
        mdast::Node::Root(root) => rsx! {
            for child in root.children.clone() {
                RenderMdastNode { node: child.clone(), registry: registry.clone() }
            }
        },
        mdast::Node::Paragraph(paragraph) => rsx! {
            p {
                for child in paragraph.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            }
        },
        mdast::Node::Heading(heading) => {
            let children_rsx = rsx! {
                for child in heading.children {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            };
            match heading.depth {
                1 => rsx! { h1 { {children_rsx} } },
                2 => rsx! { h2 { {children_rsx} } },
                3 => rsx! { h3 { {children_rsx} } },
                4 => rsx! { h4 { {children_rsx} } },
                5 => rsx! { h5 { {children_rsx} } },
                6 => rsx! { h6 { {children_rsx} } },
                _ => rsx! { div { {children_rsx} } },
            }
        }
        mdast::Node::Text(text) => rsx! { "{text.value}" },
        mdast::Node::Link(link) => {
            if link.url.starts_with("interactive:") {
                // 提取链接文本
                let link_text = link
                    .children
                    .iter()
                    .filter_map(|child| {
                        if let mdast::Node::Text(t) = child {
                            Some(t.value.clone())
                        } else {
                            None
                        }
                    })
                    .collect::<String>();

                // 解析参数
                let (component_name, params) =
                    parse_interactive_params(&link.url, link_text.clone());

                // 从注册表获取组件
                if let Some(element) = registry.get_component(&component_name, &params) {
                    return element;
                } else {
                    // 未注册组件的回退处理
                    rsx! {
                        a {
                            href: link.url,
                            title: link.title.as_deref(),
                            {format!("未知交互组件: {}", component_name)}
                        }
                    }
                }
            } else {
                rsx! {
                    a {
                        href: link.url,
                        title: link.title.as_deref(),
                        for child in link.children.clone() {
                            RenderMdastNode { node: child.clone(), registry: registry.clone() }
                        }
                    }
                }
            }
        }
        mdast::Node::List(list) => {
            let children_rsx = rsx! {
                for child in list.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            };
            if list.ordered {
                rsx! {
                    ol {
                        start: list.start.map(|s| s.to_string()),
                        {children_rsx}
                    }
                }
            } else {
                rsx! { ul { {children_rsx} } }
            }
        }
        mdast::Node::ListItem(list_item) => rsx! {
            li {
                if let Some(checked) = list_item.checked {
                    input {
                        r#type: "checkbox",
                        checked: checked,
                        disabled: true,
                    }
                }
                for child in list_item.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            }
        },
        mdast::Node::Code(code_block) => {
            let lang = code_block.lang.as_deref().unwrap_or("text");
            // 实现按static lang
            let static_lang = match code_block.lang.as_deref() {
                Some("rust") => "rust",
                Some("python") => "python",
                _ => "text",
            };
            // 查找语法定义
            let syntax = SYNTAX_SET
                .find_syntax_by_token(lang)
                .unwrap_or_else(|| SYNTAX_SET.find_syntax_plain_text());

            // 使用 ClassedHTMLGenerator 生成带 CSS 类的高亮 HTML
            let mut html_generator = ClassedHTMLGenerator::new_with_class_style(
                syntax,
                &SYNTAX_SET,
                // ClassStyle::SpacedPrefixed {
                //     prefix: (static_lang),
                // },
                ClassStyle::Spaced,
            );

            // 逐行处理代码
            for line in LinesWithEndings::from(&code_block.value) {
                let _ = html_generator.parse_html_for_line_which_includes_newline(line);
            }

            // 获取最终的高亮 HTML
            let highlighted_code = html_generator.finalize();

            // 渲染高亮代码
            rsx! {
              figure {
                class: "relative border border-border rounded my-5",
                figcaption {
                  class: "absolute inline-block text-red-400 -top-3 left-3 bg-background px-2 py-1 text-sm rounded-xl border border-border leading-none",
                  "{lang}"
                }
                pre {
                    code {
                        class: format!("language-{}", lang),
                        dangerous_inner_html: "{highlighted_code}"
                    }
                }
              }
            }
        }
        mdast::Node::InlineCode(inline_code) => rsx! {
            code { "{inline_code.value}" }
        },
        mdast::Node::Strong(strong) => rsx! {
            strong {
                for child in strong.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            }
        },
        mdast::Node::Emphasis(emphasis) => rsx! {
            em {
                for child in emphasis.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            }
        },
        mdast::Node::Break(_) => rsx! { br {} },
        mdast::Node::ThematicBreak(_) => rsx! { hr {} },
        mdast::Node::Blockquote(blockquote) => rsx! {
            blockquote {
                for child in blockquote.children.clone() {
                    RenderMdastNode { node: child.clone(), registry: registry.clone() }
                }
            }
        },
        _ => {
            rsx! {}
        }
    }
}
