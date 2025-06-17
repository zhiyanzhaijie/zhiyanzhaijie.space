use dioxus::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, PartialEq)]
enum AnimationState {
    Initial,
    Animating,
    Complete,
}

#[derive(Clone, Debug)]
struct SquareBlock {
    content: String,
    block_id: usize,
    delay: f32,
    from_edge: String,
    queue_id: usize, // 0 或 1，表示属于哪个队列
}

#[component]
pub fn AnimatedBird() -> Element {
    let mut animation_state = use_signal(|| AnimationState::Initial);
    let mut square_blocks = use_signal(|| Vec::<SquareBlock>::new());
    let flashing_blocks = use_signal(|| std::collections::HashSet::<usize>::new());
    let completed_blocks = use_signal(|| std::collections::HashSet::<usize>::new());
    let bird_text = include_str!("../bird.txt");

    // 初始化正方形块
    use_effect(move || {
        let lines: Vec<&str> = bird_text.lines().collect();
        let mut rng = rand::thread_rng();

        // 使用5x5网格分割
        let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
        let grid_rows = 5;
        let grid_cols = 5;
        let lines_per_block = (lines.len() + grid_rows - 1) / grid_rows; // 向上取整
        let chars_per_block = (max_line_length + grid_cols - 1) / grid_cols; // 向上取整

        let mut blocks = Vec::new();

        // 生成5x5网格的25个区域
        for grid_y in 0..grid_rows {
            for grid_x in 0..grid_cols {
                let start_line = grid_y * lines_per_block;
                let end_line = if grid_y == grid_rows - 1 {
                    lines.len() // 最后一行包含所有剩余行
                } else {
                    ((grid_y + 1) * lines_per_block).min(lines.len())
                };

                let start_char = grid_x * chars_per_block;
                let end_char = if grid_x == grid_cols - 1 {
                    max_line_length // 最后一列包含所有剩余字符
                } else {
                    (grid_x + 1) * chars_per_block
                };

                let block_id = grid_y * grid_cols + grid_x;
                // 提取该区域的内容
                let mut content_lines = Vec::new();

                for line_idx in start_line..end_line {
                    let line_content = if line_idx < lines.len() {
                        let line = lines[line_idx];
                        let line_chars: Vec<char> = line.chars().collect();

                        // 严格提取指定字符范围
                        let mut region_line = String::new();
                        for char_idx in start_char..end_char {
                            if char_idx < line_chars.len() {
                                region_line.push(line_chars[char_idx]);
                            } else {
                                region_line.push(' ');
                            }
                        }
                        region_line
                    } else {
                        " ".repeat(end_char - start_char)
                    };

                    content_lines.push(line_content);
                }

                let block_content = content_lines.join("\n");

                // 只有包含非空字符的块才添加
                if block_content.trim().len() > 0 {
                    // 随机选择起始边缘
                    let edges = ["top", "right", "bottom", "left"];
                    let from_edge = edges[rng.gen_range(0..4)].to_string();
                    // 双队列系统：13个块为队列0，12个块为队列1
                    let queue_id = if blocks.len() < 13 { 0 } else { 1 };
                    let queue_index = if queue_id == 0 {
                        blocks.len()
                    } else {
                        blocks.len() - 13
                    };

                    // 队列0从0秒开始，队列1从1秒开始，队列内每个块间隔6秒
                    let queue_offset = if queue_id == 0 { 0.0 } else { 1.0 };
                    let delay = queue_offset + queue_index as f32 * 6.0;

                    // 调试信息
                    let non_space_chars = block_content
                        .chars()
                        .filter(|&c| !c.is_whitespace())
                        .count();

                    blocks.push(SquareBlock {
                        content: block_content,
                        block_id,
                        delay,
                        from_edge,
                        queue_id,
                    });
                }
            }
        }

        // 完整性验证
        let total_chars: usize = blocks
            .iter()
            .map(|b| b.content.chars().filter(|&c| !c.is_whitespace()).count())
            .sum();
        let original_chars = bird_text.chars().filter(|&c| !c.is_whitespace()).count();

        // 计算双队列的总动画时长
        let queue0_count = blocks.iter().filter(|b| b.queue_id == 0).count();
        let queue1_count = blocks.iter().filter(|b| b.queue_id == 1).count();
        let queue0_total = if queue0_count > 0 {
            (queue0_count - 1) as f32 * 6.0 + 10.0
        } else {
            0.0
        };
        let queue1_total = if queue1_count > 0 {
            1.0 + (queue1_count - 1) as f32 * 6.0 + 10.0
        } else {
            0.0
        };
        let total_animation_time = queue0_total.max(queue1_total);
        square_blocks.set(blocks);
    });

    // 自动开始动画
    use_effect(move || {
        let blocks = square_blocks.read().clone();
        let flashing_signal = flashing_blocks.clone();

        spawn(async move {
            // 延迟500ms开始动画
            gloo_timers::future::TimeoutFuture::new(500).await;
            animation_state.set(AnimationState::Animating);

            // 为每个块安排透明度变化效果
            let completed_signal = completed_blocks.clone();
            for block in &blocks {
                let block_id = block.block_id;
                let arrival_time = ((block.delay + 10.0) * 1000.0) as u32;
                let mut flashing_signal_clone = flashing_signal.clone();
                let mut completed_signal_clone = completed_signal.clone();

                spawn(async move {
                    // 等待块到达最终位置
                    gloo_timers::future::TimeoutFuture::new(arrival_time).await;

                    // 阶段1：突然跳转到0.9透明度
                    flashing_signal_clone.with_mut(|set| {
                        set.insert(block_id);
                    });

                    // 阶段2：1秒后开始衰退到0.5
                    gloo_timers::future::TimeoutFuture::new(1000).await;
                    flashing_signal_clone.with_mut(|set| {
                        set.remove(&block_id);
                    });
                    completed_signal_clone.with_mut(|set| {
                        set.insert(block_id);
                    });
                });
            }

            // 计算双队列的总动画时长
            let queue0_count = blocks.iter().filter(|b| b.queue_id == 0).count();
            let queue1_count = blocks.iter().filter(|b| b.queue_id == 1).count();
            let queue0_total = if queue0_count > 0 {
                (queue0_count - 1) as f32 * 6.0 + 10.0
            } else {
                0.0
            };
            let queue1_total = if queue1_count > 0 {
                1.0 + (queue1_count - 1) as f32 * 6.0 + 10.0
            } else {
                0.0
            };
            let total_time = (queue0_total.max(queue1_total) * 1000.0) as u32;

            gloo_timers::future::TimeoutFuture::new(total_time).await;
            animation_state.set(AnimationState::Complete);
        });
    });

    let current_state = *animation_state.read();
    let blocks = square_blocks.read();
    let flashing_set = flashing_blocks.read();
    let completed_set = completed_blocks.read();

    rsx! {
        div {
            class: "fixed inset-0 z-0 pointer-events-none overflow-hidden",

            // 2x2 Grid 容器
            div {
                class: "absolute scale-[70%] -bottom-[30%] right-[5%]",

                div {
                    class: "grid grid-cols-5 grid-rows-5 font-mono text-xs text-muted-foreground",

                    // 按照网格顺序渲染块
                    for block in blocks.iter() {
                        div {
                            key: "{block.block_id}",
                            class: "whitespace-pre select-none will-change-transform",
                            style: {
                                let is_flashing = flashing_set.contains(&block.block_id);
                                let is_completed = completed_set.contains(&block.block_id);

                                let base_opacity = if is_flashing {
                                    0.9  // 阶段1：瞬间跳转到0.9
                                } else if is_completed {
                                    0.5  // 阶段3：永远保持0.5
                                } else {
                                    0.3  // 动画过程中保持0.3
                                };

                                let transition_props = if is_flashing && !is_completed {
                                    // 从0.9衰退到0.5时添加1秒过渡
                                    "transition: transform 10s ease-out, opacity 1s ease-out;"
                                } else {
                                    "transition: transform 10s ease-out;"
                                };

                                match current_state {
                                    AnimationState::Initial => {
                                        let transform = match block.from_edge.as_str() {
                                            "top" => "translateY(-150vh)",
                                            "right" => "translateX(150vw)",
                                            "bottom" => "translateY(150vh)",
                                            _ => "translateX(-150vw)", // left
                                        };
                                        format!(
                                            "transform: {}; opacity: {};",
                                            transform, base_opacity
                                        )
                                    },
                                    AnimationState::Animating => format!(
                                        "transform: translateX(0) translateY(0); opacity: {}; {}; transition-delay: {}s;",
                                        base_opacity, transition_props, block.delay
                                    ),
                                    AnimationState::Complete => format!(
                                        "transform: translateX(0) translateY(0); opacity: {}; {};",
                                        base_opacity, transition_props
                                    ),
                                }
                            },
                            "{block.content}"
                        }
                    }
                }
            }
        }
    }
}
