use dioxus::prelude::*;
use log::info;
use rand::prelude::SliceRandom;
use rand::Rng;
use std::collections::{HashMap, HashSet};

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

// 动画配置结构
#[derive(Clone)]
struct AnimationConfig {
    // 网格配置
    grid_rows: usize,
    grid_cols: usize,

    // 时间配置
    initial_delay_ms: u32,
    layer_interval_seconds: f32, // 每层之间的间隔
    block_interval_seconds: f32, // 同层内块之间的间隔
    movement_duration_seconds: f32,
    opacity_transition_seconds: u32,

    // 透明度配置
    opacity_moving: f32,
    opacity_arrived: f32,
    opacity_settled: f32,

    // 视觉配置
    scale_class: &'static str,
    position_classes: &'static str,
    text_classes: &'static str,
}

impl AnimationConfig {
    fn desktop() -> Self {
        Self {
            // 网格配置
            grid_rows: 12,
            grid_cols: 12,

            // 时间配置
            initial_delay_ms: 500,
            layer_interval_seconds: 4.0, // 每层间隔2秒
            block_interval_seconds: 2.0, // 同层内间隔0.3秒
            movement_duration_seconds: 6.0,
            opacity_transition_seconds: 3,

            // 透明度配置
            opacity_moving: 0.2,
            opacity_arrived: 1.0,
            opacity_settled: 0.4,

            // 视觉配置
            scale_class: "scale-[70%]",
            position_classes: "-bottom-[30%] right-[5%]",
            text_classes: "font-mono text-xs text-muted-foreground",
        }
    }

    fn mobile() -> Self {
        Self {
            // 网格配置
            grid_rows: 12,
            grid_cols: 12,

            // 时间配置
            initial_delay_ms: 500,
            layer_interval_seconds: 4.0,
            block_interval_seconds: 2.0,
            movement_duration_seconds: 6.0,
            opacity_transition_seconds: 3,

            // 透明度配置 - 移动端配置
            opacity_moving: 0.2,
            opacity_arrived: 0.5,
            opacity_settled: 0.25,

            // 视觉配置 - 移动端配置
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
    Complete,
}

#[derive(Clone, Debug)]
struct SquareBlock {
    content: String,
    block_id: usize,       // 网格中的固定位置ID，决定最终显示位置
    grid_row: usize,       // 在网格中的行位置
    grid_col: usize,       // 在网格中的列位置
    delay: f32,            // 动画延迟
    from_edge: String,     // 动画起始方向
    priority_layer: usize, // 优先级层级（用于排序）
}

#[derive(Clone, Copy, PartialEq)]
enum OpacityState {
    Moving,
    Arrived,
    Settled,
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    Top,
    Right,
    Bottom,
    Left,
}

impl Direction {
    fn all() -> [Direction; 4] {
        [
            Direction::Top,
            Direction::Right,
            Direction::Bottom,
            Direction::Left,
        ]
    }

    fn to_string(&self) -> &'static str {
        match self {
            Direction::Top => "top",
            Direction::Right => "right",
            Direction::Bottom => "bottom",
            Direction::Left => "left",
        }
    }
}

// 路径计算和冲突检测
struct PathCalculator {
    grid_rows: usize,
    grid_cols: usize,
}

impl PathCalculator {
    fn new(grid_rows: usize, grid_cols: usize) -> Self {
        Self {
            grid_rows,
            grid_cols,
        }
    }

    // 计算曼哈顿距离到中心点
    fn manhattan_distance_to_center(&self, row: usize, col: usize) -> usize {
        let center_row = self.grid_rows / 2;
        let center_col = self.grid_cols / 2;
        ((row as i32 - center_row as i32).abs() + (col as i32 - center_col as i32).abs()) as usize
    }

    // 计算从某个方向进入时经过的路径
    fn calculate_path(&self, row: usize, col: usize, direction: Direction) -> Vec<(usize, usize)> {
        let mut path = Vec::new();

        match direction {
            Direction::Top => {
                // 从上方进入，经过该列的所有上方位置
                for r in 0..row {
                    path.push((r, col));
                }
            }
            Direction::Bottom => {
                // 从下方进入，经过该列的所有下方位置
                for r in (row + 1)..self.grid_rows {
                    path.push((r, col));
                }
            }
            Direction::Left => {
                // 从左方进入，经过该行的所有左方位置
                for c in 0..col {
                    path.push((row, c));
                }
            }
            Direction::Right => {
                // 从右方进入，经过该行的所有右方位置
                for c in (col + 1)..self.grid_cols {
                    path.push((row, c));
                }
            }
        }

        path
    }

    // 检查路径是否与已到达的块冲突
    fn has_path_conflict(
        &self,
        row: usize,
        col: usize,
        direction: Direction,
        arrived_blocks: &HashSet<(usize, usize)>,
    ) -> bool {
        let path = self.calculate_path(row, col, direction);
        path.iter().any(|pos| arrived_blocks.contains(pos))
    }

    // 为块选择最佳方向
    fn choose_best_direction(
        &self,
        row: usize,
        col: usize,
        arrived_blocks: &HashSet<(usize, usize)>,
        rng: &mut rand::prelude::ThreadRng,
    ) -> Direction {
        let directions = Direction::all();
        let mut available_directions = Vec::new();

        // 找到所有无冲突的方向
        for direction in &directions {
            if !self.has_path_conflict(row, col, *direction, arrived_blocks) {
                available_directions.push(*direction);
            }
        }

        if !available_directions.is_empty() {
            available_directions[rng.gen_range(0..available_directions.len())]
        } else {
            let mut min_conflicts = usize::MAX;
            let mut best_direction = Direction::Top;

            for direction in &directions {
                let path = self.calculate_path(row, col, *direction);
                let conflicts = path
                    .iter()
                    .filter(|pos| arrived_blocks.contains(pos))
                    .count();
                if conflicts < min_conflicts {
                    min_conflicts = conflicts;
                    best_direction = *direction;
                }
            }

            best_direction
        }
    }
}

// Helper function to generate grid CSS classes
fn get_grid_classes(rows: usize, cols: usize) -> String {
    match (rows, cols) {
        (1, 1) => "grid-cols-1 grid-rows-1",
        (2, 2) => "grid-cols-2 grid-rows-2",
        (3, 3) => "grid-cols-3 grid-rows-3",
        (4, 4) => "grid-cols-4 grid-rows-4",
        (5, 5) => "grid-cols-5 grid-rows-5",
        (6, 6) => "grid-cols-6 grid-rows-6",
        (7, 7) => "grid-cols-7 grid-rows-7",
        (8, 8) => "grid-cols-8 grid-rows-8",
        (9, 9) => "grid-cols-9 grid-rows-9",
        (10, 10) => "grid-cols-10 grid-rows-10",
        (11, 11) => "grid-cols-11 grid-rows-11",
        (12, 12) => "grid-cols-12 grid-rows-12",
        (12, cols) => match cols {
            1 => "grid-cols-1 grid-rows-12",
            2 => "grid-cols-2 grid-rows-12",
            3 => "grid-cols-3 grid-rows-12",
            4 => "grid-cols-4 grid-rows-12",
            5 => "grid-cols-5 grid-rows-12",
            6 => "grid-cols-6 grid-rows-12",
            7 => "grid-cols-7 grid-rows-12",
            8 => "grid-cols-8 grid-rows-12",
            9 => "grid-cols-9 grid-rows-12",
            10 => "grid-cols-10 grid-rows-12",
            11 => "grid-cols-11 grid-rows-12",
            _ => "grid-cols-12 grid-rows-12",
        },
        _ => "grid-cols-12 grid-rows-12", // Default fallback
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
        9 => "row-start-9",
        10 => "row-start-10",
        11 => "row-start-11",
        12 => "row-start-12",
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
        9 => "col-start-9",
        10 => "col-start-10",
        11 => "col-start-11",
        12 => "col-start-12",
        _ => "col-start-1",
    };

    format!("{} {}", row_class, col_class)
}

#[component]
pub fn AnimatedBird() -> Element {
    let mut config = use_signal(|| AnimationConfig::default());
    let mut animation_state = use_signal(|| AnimationState::Initial);
    let mut square_blocks = use_signal(|| Vec::<SquareBlock>::new());
    let opacity_states = use_signal(|| HashMap::<usize, OpacityState>::new());
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
            let mut rng = rand::thread_rng();

            // 使用配置的网格尺寸
            let max_line_length = lines.iter().map(|line| line.len()).max().unwrap_or(0);
            let lines_per_block = (lines.len() + config.grid_rows - 1) / config.grid_rows;
            let chars_per_block = (max_line_length + config.grid_cols - 1) / config.grid_cols;

            let mut blocks = Vec::new();
            let path_calculator = PathCalculator::new(config.grid_rows, config.grid_cols);

            // 生成网格区域，保持原有的位置对应关系
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

                    // 保持原有的block_id计算方式，确保位置固定
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

                    // 只过滤完全空的块，保留包含空格的块（ASCII艺术中空格也是重要的）
                    if !block_content.chars().all(|c| c == ' ' || c == '\n') {
                        // 计算优先级层级（到中心的距离）
                        let priority_layer =
                            path_calculator.manhattan_distance_to_center(grid_y, grid_x);

                        blocks.push(SquareBlock {
                            content: block_content,
                            block_id,
                            grid_row: grid_y,
                            grid_col: grid_x,
                            delay: 0.0,               // 稍后计算
                            from_edge: String::new(), // 稍后确定
                            priority_layer,
                        });
                    }
                }
            }

            // 按优先级层级分组，但保持块的原始位置信息
            let mut layers: HashMap<usize, Vec<SquareBlock>> = HashMap::new();
            for block in blocks {
                layers
                    .entry(block.priority_layer)
                    .or_insert_with(Vec::new)
                    .push(block);
            }

            // 为每层随机排序（只影响动画播放顺序，不影响最终位置）
            for (_, layer_blocks) in layers.iter_mut() {
                layer_blocks.shuffle(&mut rng);
            }

            // 智能分配方向和延迟
            let mut final_blocks = Vec::new();
            let mut arrived_blocks = HashSet::new();

            // 按层级顺序处理
            let mut sorted_layers: Vec<_> = layers.into_iter().collect();
            sorted_layers.sort_by_key(|(layer, _)| *layer);

            for (layer_index, (_, layer_blocks)) in sorted_layers.into_iter().enumerate() {
                // 计算这一层的基础延迟
                let layer_base_delay = layer_index as f32 * config.layer_interval_seconds;

                for (block_index, mut block) in layer_blocks.into_iter().enumerate() {
                    // 选择最佳方向（基于网格位置，不是动画顺序）
                    let direction = path_calculator.choose_best_direction(
                        block.grid_row,
                        block.grid_col,
                        &arrived_blocks,
                        &mut rng,
                    );
                    block.from_edge = direction.to_string().to_string();

                    // 计算延迟（这里影响动画播放顺序）
                    block.delay =
                        layer_base_delay + block_index as f32 * config.block_interval_seconds;

                    // 标记为将要到达（基于网格位置）
                    arrived_blocks.insert((block.grid_row, block.grid_col));
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
            let blocks = square_blocks.read().clone();
            let opacity_signal = opacity_states.clone();
            let config = config.clone();

            spawn(async move {
                // 使用配置的初始延迟
                gloo_timers::future::TimeoutFuture::new(config().initial_delay_ms).await;
                animation_state.set(AnimationState::Animating);

                // 为每个块安排透明度状态变化
                for block in &blocks {
                    let block_id = block.block_id;
                    let arrival_time =
                        ((block.delay + config().movement_duration_seconds) * 1000.0) as u32;
                    let mut opacity_signal_clone = opacity_signal.clone();

                    spawn(async move {
                        // 等待块到达最终位置
                        gloo_timers::future::TimeoutFuture::new(arrival_time).await;

                        // 瞬间跳转到arrived状态
                        opacity_signal_clone.with_mut(|map| {
                            map.insert(block_id, OpacityState::Arrived);
                        });

                        // 立即开始衰退过渡
                        opacity_signal_clone.with_mut(|map| {
                            map.insert(block_id, OpacityState::Settled);
                        });
                    });
                }

                // 计算总动画时长
                let max_delay = blocks
                    .iter()
                    .map(|b| b.delay + config().movement_duration_seconds)
                    .fold(0.0, f32::max);
                let total_time = (max_delay * 1000.0) as u32;

                gloo_timers::future::TimeoutFuture::new(total_time).await;
                animation_state.set(AnimationState::Complete);
            });
        }
    });

    let current_state = *animation_state.read();
    let blocks = square_blocks.read();
    let opacity_map = opacity_states.read();

    // 克隆配置以避免借用问题
    let config_for_render = config();

    rsx! {
        div {
            class: "fixed inset-0 z-0 pointer-events-none overflow-hidden",

            div {
                class: "absolute {config_for_render.scale_class} {config_for_render.position_classes}",

                div {
                    class: "grid {get_grid_classes(config_for_render.grid_rows, config_for_render.grid_cols)} {config_for_render.text_classes}",

                    for block in blocks.iter() {
                        div {
                            key: "{block.block_id}",
                            class: "whitespace-pre select-none will-change-transform {get_grid_position_classes(block.grid_row, block.grid_col)}",
                            style: {
                                let opacity_state = opacity_map.get(&block.block_id).unwrap_or(&OpacityState::Moving);

                                let (base_opacity, transition_opacity) = match opacity_state {
                                    OpacityState::Moving => (config_for_render.opacity_moving, String::new()),
                                    OpacityState::Arrived => (config_for_render.opacity_arrived, String::new()),
                                    OpacityState::Settled => (config_for_render.opacity_settled, format!("transition: opacity {}s ease-out;", config_for_render.opacity_transition_seconds)),
                                };

                                match current_state {
                                    AnimationState::Initial => {
                                        let transform = match block.from_edge.as_str() {
                                            "top" => "translateY(-150vh)",
                                            "right" => "translateX(150vw)",
                                            "bottom" => "translateY(150vh)",
                                            _ => "translateX(-150vw)",
                                        };
                                        format!(
                                            "transform: {}; opacity: {};",
                                            transform, config_for_render.opacity_settled
                                        )
                                    },
                                    AnimationState::Animating => format!(
                                        "transform: translateX(0) translateY(0); opacity: {}; transition: transform {}s ease-out; transition-delay: {}s; {}",
                                        base_opacity, config_for_render.movement_duration_seconds, block.delay, transition_opacity
                                    ),
                                    AnimationState::Complete => format!(
                                        "transform: translateX(0) translateY(0); opacity: {}; {}",
                                        base_opacity, transition_opacity
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
