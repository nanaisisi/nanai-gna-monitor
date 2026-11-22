#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rect {
    x: f64,
    y: f64,
    width: f64,
    height: f64,
}

impl Rect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn x(&self) -> f64 {
        self.x
    }

    pub fn y(&self) -> f64 {
        self.y
    }

    pub fn width(&self) -> f64 {
        self.width
    }

    pub fn height(&self) -> f64 {
        self.height
    }
}

#[derive(Debug, Clone)]
pub struct TreemapItem<T> {
    value: f64,
    data: T,
}

impl<T> TreemapItem<T> {
    pub fn new(value: f64, data: T) -> Self {
        Self { value, data }
    }

    #[allow(dead_code)]
    pub fn value(&self) -> f64 {
        self.value
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &T {
        &self.data
    }
}

#[derive(Debug, Clone)]
pub struct TreemapNode<T> {
    rect: Rect,
    data: T,
}

impl<T> TreemapNode<T> {
    #[allow(dead_code)]
    pub fn rect(&self) -> Rect {
        self.rect
    }

    #[allow(dead_code)]
    pub fn data(&self) -> &T {
        &self.data
    }

    pub fn into_parts(self) -> (Rect, T) {
        (self.rect, self.data)
    }
}

/// Squarified Treemap layout algorithm (similar to WizTree / SpaceMonger / WinDirStat)
pub fn layout_treemap<T: Clone>(items: &[TreemapItem<T>], bounds: Rect) -> Vec<TreemapNode<T>> {
    if items.is_empty() || bounds.width <= 0.0 || bounds.height <= 0.0 {
        return Vec::new();
    }

    let total_value: f64 = items.iter().map(|item| item.value.max(0.0)).sum();
    if total_value <= 0.0 {
        return Vec::new();
    }

    // Sort descending by value
    let mut sorted_items = items.to_vec();
    sorted_items.sort_by(|a, b| {
        b.value
            .partial_cmp(&a.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let total_area = bounds.width * bounds.height;
    let mut normalized_items: Vec<(f64, T)> = sorted_items
        .into_iter()
        .map(|item| ((item.value.max(0.0) / total_value) * total_area, item.data))
        .collect();

    let mut result = Vec::new();
    let mut row = Vec::new();
    squarify(SquarifyState {
        children: &mut normalized_items,
        row: &mut row,
        bounds,
        result: &mut result,
    });
    result
}

struct SquarifyState<'a, T> {
    children: &'a mut Vec<(f64, T)>,
    row: &'a mut Vec<(f64, T)>,
    bounds: Rect,
    result: &'a mut Vec<TreemapNode<T>>,
}

fn squarify<T: Clone>(mut state: SquarifyState<'_, T>) {
    if state.children.is_empty() {
        if !state.row.is_empty() {
            layout_row(state.row, state.bounds, state.result);
        }
        return;
    }

    if state.bounds.width <= 1.0 || state.bounds.height <= 1.0 {
        return;
    }

    let shortest_side = state.bounds.width.min(state.bounds.height);
    let next_item = &state.children[0];

    let mut candidate_row = state.row.clone();
    candidate_row.push(next_item.clone());

    if state.row.is_empty()
        || worst_aspect_ratio(state.row, shortest_side)
            >= worst_aspect_ratio(&candidate_row, shortest_side)
    {
        let item = state.children.remove(0);
        state.row.push(item);
        squarify(state);
    } else {
        let used_bounds = layout_row(state.row, state.bounds, state.result);
        state.row.clear();
        state.bounds = remaining_bounds(state.bounds, used_bounds);
        squarify(state);
    }
}

fn worst_aspect_ratio<T>(row: &[(f64, T)], side: f64) -> f64 {
    if row.is_empty() || side <= 0.0 {
        return f64::MAX;
    }
    let sum: f64 = row.iter().map(|(area, _)| *area).sum();
    if sum <= 0.0 {
        return f64::MAX;
    }
    let side_sq = side * side;
    let sum_sq = sum * sum;

    let mut max_ratio: f64 = 0.0;
    for &(area, _) in row {
        if area <= 0.0 {
            continue;
        }
        let r1 = (side_sq * area) / sum_sq;
        let r2 = sum_sq / (side_sq * area);
        let ratio = r1.max(r2);
        if ratio > max_ratio {
            max_ratio = ratio;
        }
    }
    max_ratio
}

fn layout_row<T: Clone>(row: &[(f64, T)], bounds: Rect, result: &mut Vec<TreemapNode<T>>) -> Rect {
    let row_area: f64 = row.iter().map(|(area, _)| *area).sum();
    if row_area <= 0.0 {
        return bounds;
    }

    let is_horizontal = bounds.width >= bounds.height;
    if is_horizontal {
        let row_width = (row_area / bounds.height).min(bounds.width);
        let mut current_y = bounds.y;

        for (area, data) in row {
            let item_height = if row_area > 0.0 {
                (area / row_width).min(bounds.y + bounds.height - current_y)
            } else {
                0.0
            };

            result.push(TreemapNode {
                rect: Rect {
                    x: bounds.x,
                    y: current_y,
                    width: row_width.max(1.0),
                    height: item_height.max(1.0),
                },
                data: data.clone(),
            });
            current_y += item_height;
        }

        Rect {
            x: bounds.x,
            y: bounds.y,
            width: row_width,
            height: bounds.height,
        }
    } else {
        let row_height = (row_area / bounds.width).min(bounds.height);
        let mut current_x = bounds.x;

        for (area, data) in row {
            let item_width = if row_area > 0.0 {
                (area / row_height).min(bounds.x + bounds.width - current_x)
            } else {
                0.0
            };

            result.push(TreemapNode {
                rect: Rect {
                    x: current_x,
                    y: bounds.y,
                    width: item_width.max(1.0),
                    height: row_height.max(1.0),
                },
                data: data.clone(),
            });
            current_x += item_width;
        }

        Rect {
            x: bounds.x,
            y: bounds.y,
            width: bounds.width,
            height: row_height,
        }
    }
}

fn remaining_bounds(total: Rect, used: Rect) -> Rect {
    if total.width >= total.height {
        Rect {
            x: total.x + used.width,
            y: total.y,
            width: (total.width - used.width).max(0.0),
            height: total.height,
        }
    } else {
        Rect {
            x: total.x,
            y: total.y + used.height,
            width: total.width,
            height: (total.height - used.height).max(0.0),
        }
    }
}
