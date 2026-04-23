use super::{BehaviorContext, MoveResult};

pub fn find_position(ctx: &BehaviorContext) -> MoveResult {
    let x = ctx.x;
    let y = ctx.y;

    if y + 1 >= ctx.height {
        return None;
    }

    if is_empty(x, y + 1, ctx) {
        return Some((x, y + 1));
    }

    None
}

fn is_empty(x: u32, y: u32, ctx: &BehaviorContext) -> bool {
    if x >= ctx.width || y >= ctx.height {
        return false;
    }

    let idx = (y * ctx.width + x) as usize;
    ctx.cells[idx] == 0 && ctx.new_cells[idx] == 0
}
