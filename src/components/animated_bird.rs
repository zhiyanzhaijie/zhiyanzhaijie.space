use dioxus::prelude::*;
#[cfg(target_arch = "wasm32")]
use gloo_timers::future::TimeoutFuture;
use log::info;
use std::collections::HashMap;

#[cfg(target_arch = "wasm32")]
fn is_mobile() -> bool {
    if let Some(window) = web_sys::window() {
        if let Ok(Some(media_query)) = window.match_media("(max-width: 768px)") {
            let is_mobile = media_query.matches();
            info!(
                "检测到设备类型: {}",
                if is_mobile { "移动端" } else { "桌面端" }
            );
            is_mobile
        } else {
            info!("媒体查询失败，默认为桌面端");
            false
        }
    } else {
        info!("无法获取窗口对象，默认为桌面端");
        false
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn is_mobile() -> bool {
    false
}

// 动画配置结构
#[derive(Clone)]
struct AnimationConfig {
    grid_rows: usize,
    grid_cols: usize,
    initial_delay_ms: u32,
    layer_interval_seconds: f32,
    block_interval_seconds: f32,
    opacity_duration_seconds: f32,
    scale_class: &'static str,
    position_classes: &'static str,
    text_classes: &'static str,
}

impl AnimationConfig {
    fn desktop() -> Self {
        Self {
            grid_rows: 8,
            grid_cols: 8,
            initial_delay_ms: 900,
            layer_interval_seconds: 0.65,
            block_interval_seconds: 0.15,
            opacity_duration_seconds: 2.0,
            scale_class: "scale-[70%]",
            position_classes: "-bottom-[30%] right-[5%]",
            text_classes: "font-mono text-xs text-muted-foreground",
        }
    }

    fn mobile() -> Self {
        Self {
            grid_rows: 8,
            grid_cols: 8,
            initial_delay_ms: 900,
            layer_interval_seconds: 0.65,
            block_interval_seconds: 0.15,
            opacity_duration_seconds: 2.0,
            scale_class: "scale-[55%] w-[130%]",
            position_classes: "-right-[18%] -bottom-[45%]",
            text_classes: "font-mono text-xs text-muted-foreground",
        }
    }
}

impl Default for AnimationConfig {
    fn default() -> Self {
        if is_mobile() {
            info!("初始化移动端配置");
            Self::mobile()
        } else {
            info!("初始化桌面端配置");
            Self::desktop()
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum AnimationState {
    Initial,
    Animating,
}

#[derive(Clone, Debug)]
struct SquareBlock {
    content: String,
    block_id: usize,
    grid_row: usize,
    grid_col: usize,
    delay: f32,
    priority_layer: usize,
}

fn manhattan_distance_to_center(grid_rows: usize, grid_cols: usize, row: usize, col: usize) -> usize {
    let center_row = grid_rows / 2;
    let center_col = grid_cols / 2;
    ((row as i32 - center_row as i32).abs() + (col as i32 - center_col as i32).abs()) as usize
}

// Helper function to generate grid CSS classes
fn get_grid_classes(rows: usize, cols: usize) -> String {
    match (rows, cols) {
        (8, 8) => "grid-cols-8 grid-rows-8",
        _ => "grid-cols-8 grid-rows-8",
    }
    .to_string()
}

// Helper function to get grid position CSS classes
fn get_grid_position_classes(row: usize, col: usize) -> String {
    let row_class = match row + 1 {
        1 => "row-start-1",
        2 => "row-start-2",
        3 => "row-start-3",
        4 => "row-start-4",
        5 => "row-start-5",
        6 => "row-start-6",
        7 => "row-start-7",
        8 => "row-start-8",
        _ => "row-start-1",
    };

    let col_class = match col + 1 {
        1 => "col-start-1",
        2 => "col-start-2",
        3 => "col-start-3",
        4 => "col-start-4",
        5 => "col-start-5",
        6 => "col-start-6",
        7 => "col-start-7",
        8 => "col-start-8",
        _ => "col-start-1",
    };

    format!("{} {}", row_class, col_class)
}

#[component]
pub fn AnimatedBird() -> Element {
    let mut config = use_signal(|| AnimationConfig::default());
    let mut animation_state = use_signal(|| AnimationState::Initial);
    let mut square_blocks = use_signal(|| Vec::<SquareBlock>::new());
    let bird_text = include_str!("../bird.txt");

    // 监听窗口大小变化，更新配置
    use_effect(move || {
        let new_config = if is_mobile() {
            info!("响应式更新: 切换到移动端配置");
            AnimationConfig::mobile()
        } else {
            info!("响应式更新: 切换到桌面端配置");
            AnimationConfig::desktop()
        };
        config.set(new_config);
    });

    // 初始化正方形块
    use_effect({
        let config = config();
        move || {
            let lines: Vec<&str> = bird_text.lines().collect();
            let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
            let lines_per_block = (lines.len() + config.grid_rows - 1) / config.grid_rows;
            let chars_per_block = (max_line_length + config.grid_cols - 1) / config.grid_cols;

            let mut blocks = Vec::new();

            for grid_y in 0..config.grid_rows {
                for grid_x in 0..config.grid_cols {
                    let start_line = grid_y * lines_per_block;
                    let end_line = if grid_y == config.grid_rows - 1 {
                        lines.len()
                    } else {
                        ((grid_y + 1) * lines_per_block).min(lines.len())
                    };

                    let start_char = grid_x * chars_per_block;
                    let end_char = if grid_x == config.grid_cols - 1 {
                        max_line_length
                    } else {
                        (grid_x + 1) * chars_per_block
                    };

                    let block_id = grid_y * config.grid_cols + grid_x;
                    let mut content_lines = Vec::new();

                    for line_idx in start_line..end_line {
                        let line_content = if line_idx < lines.len() {
                            let line = lines[line_idx];
                            let line_chars: Vec<char> = line.chars().collect();

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

                    if !block_content.chars().all(|c| c == ' ' || c == '\n') {

                        blocks.push(SquareBlock {
                            content: block_content,
                            block_id,
                            grid_row: grid_y,
                            grid_col: grid_x,
                            delay: 0.0,
                            priority_layer: manhattan_distance_to_center(
                                config.grid_rows,
                                config.grid_cols,
                                grid_y,
                                grid_x,
                            ),
                        });
                    }
                }
            }

            let mut layers: HashMap<usize, Vec<SquareBlock>> = HashMap::new();
            for block in blocks {
                layers
                    .entry(block.priority_layer)
                    .or_insert_with(Vec::new)
                    .push(block);
            }
            let mut final_blocks = Vec::new();

            let mut sorted_layers: Vec<_> = layers.into_iter().collect();
            sorted_layers.sort_by_key(|(layer, _)| *layer);

            for (layer_index, (_, layer_blocks)) in sorted_layers.into_iter().enumerate() {
                let layer_base_delay = layer_index as f32 * config.layer_interval_seconds;
                let mut sorted_blocks = layer_blocks;
                sorted_blocks.sort_by_key(|block| block.block_id);

                for (block_index, mut block) in sorted_blocks.into_iter().enumerate() {
                    block.delay = layer_base_delay + block_index as f32 * config.block_interval_seconds;
                    final_blocks.push(block);
                }
            }

            // 按block_id排序，确保渲染时的顺序正确（这保证了网格位置的正确性）
            final_blocks.sort_by_key(|block| block.block_id);
            square_blocks.set(final_blocks);
        }
    });

    // 自动开始动画
    use_effect({
        let config = config.clone();
        move || {
            let config_for_effect = config.clone();

            #[cfg(target_arch = "wasm32")]
            spawn(async move {
                TimeoutFuture::new(config_for_effect().initial_delay_ms).await;
                animation_state.set(AnimationState::Animating);
            });

            #[cfg(not(target_arch = "wasm32"))]
            {
                let _ = config_for_effect().initial_delay_ms;
                animation_state.set(AnimationState::Animating);
            }
        }
    });

    let current_state = *animation_state.read();
    let blocks = square_blocks.read();
    let config_for_render = config();

    rsx! {
        div {
            class: "fixed inset-0 z-0 pointer-events-none overflow-hidden",

            div {
                class: "absolute",
                class: "{config_for_render.scale_class}",
                class: "{config_for_render.position_classes}",

                div {
                    class: "grid",
                    class: "{get_grid_classes(config_for_render.grid_rows, config_for_render.grid_cols)}",
                    class: "{config_for_render.text_classes}",

                    for block in blocks.iter() {
                        div {
                            key: "{block.block_id}",
                            class: "whitespace-pre select-none will-change-[opacity]",
                            class: "{get_grid_position_classes(block.grid_row, block.grid_col)}",
                            style: {

                                match current_state {
                                    AnimationState::Initial => "opacity: 0;".to_string(),
                                    AnimationState::Animating => format!(
                                        "opacity: 1; transition: opacity {}s ease-out; transition-delay: {}s;",
                                        config_for_render.opacity_duration_seconds, block.delay
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
