use crate::chunk::Chunk;

pub fn below_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
) -> Option<(usize, bool, (i32, i32))> {
    vertical_tile(x, y, chunk, neighbors, true)
}

pub fn below_right_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
) -> Option<(usize, bool, (i32, i32))> {
    diagonal_tile(x, y, chunk, neighbors, true, true)
}

pub fn below_left_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
) -> Option<(usize, bool, (i32, i32))> {
    diagonal_tile(x, y, chunk, neighbors, true, false)
}

fn vertical_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
    down: bool,
) -> Option<(usize, bool, (i32, i32))> {

    let offset_y: i32 = if down { 1 } else { -1 };
    let offset_x: i32 = 0;

    get_tile(x, y, chunk, neighbors, offset_x, offset_y)
}

fn diagonal_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
    down: bool,
    right: bool,
) -> Option<(usize, bool, (i32, i32))> {
    let offset_x: i32 = if right { 1 } else { -1 };
    let offset_y: i32 = if down { 1 } else { -1 };

    get_tile(x, y, chunk, neighbors, offset_x, offset_y)
}

fn get_tile(
    x: usize,
    y: usize,
    chunk: &Chunk,
    neighbors: &[&Chunk],
    offset_x: i32,
    offset_y: i32,
) -> Option<(usize, bool, (i32, i32))> {
    let mut next_y = y as i32 + offset_y;
    let mut next_x = x as i32 + offset_x;
    let mut chunk_x = chunk.x as i32;
    let mut chunk_y = chunk.y as i32;

    let crossed_y = next_y >= chunk.height as i32 || next_y < 0;
    let crossed_x = next_x >= chunk.width as i32 || next_x < 0;

    if crossed_y {
        if offset_y > 0 {
            next_y = 0; // moved down → top row of neighbor
        } else {
            next_y = chunk.height as i32 - 1; // moved up → bottom row of neighbor
        }
        chunk_y += offset_y;
    }

    if crossed_x {
        if offset_x > 0 {
            next_x = 0; // moved right, wrap to left edge of neighbor
        } else {
            next_x = chunk.width as i32 - 1; // moved left, wrap to right edge of neighbor
        }
        chunk_x += offset_x;
    }

    let cross_chunk = crossed_y || crossed_x;

    if !cross_chunk {
        let next_idx = (next_y * chunk.width as i32 + next_x) as usize;
        return Some((next_idx, false, (chunk.x as i32, chunk.y as i32)));
    }

    // Find neighbor chunk
    let neighbor_coord = (chunk_x, chunk_y);
    if let Some(&neighbor_chunk) = neighbors.iter().find(|c| (c.x, c.y) == neighbor_coord) {
        if next_y < neighbor_chunk.height as i32
            && next_x < neighbor_chunk.width as i32
            && next_y >= 0
            && next_x >= 0
        {
            let below_idx = (next_y * neighbor_chunk.width as i32 + next_x) as usize;
            Some((below_idx, true, neighbor_coord))
        } else {
            None
        }
    } else {
        None
    }
}
