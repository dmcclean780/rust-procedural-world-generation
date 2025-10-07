use egui::{ColorImage, TextureHandle, TextureOptions};

use crate::{chunk_list::ChunkList, colors::COLORS_RGBA, tiles::tile_kind::TileKind, math::{div_floor, euclidean_mod}};

pub struct Viewport {
    pub width_pixels: usize,
    pub height_pixels: usize,
    pub width_tiles: usize,
    pub height_tiles: usize,
    pub scale: usize, // pixels per tile
    pub texture: Option<TextureHandle>,
    pub offset_x: isize,      // pixels
    pub offset_y: isize,      // pixels
    pub buffer_chunks: usize, // number of chunks to buffer around the viewport
}

impl Viewport {
    pub fn new(
        width_pixels: usize,
        height_pixels: usize,
        scale: usize,
        buffer_chunks: usize,
    ) -> Self {
        let width_tiles = width_pixels / scale;
        let height_tiles = height_pixels / scale;
        Self {
            width_pixels,
            height_pixels,
            width_tiles,
            height_tiles,
            scale,
            texture: None,
            offset_x: 0,
            offset_y: 0,
            buffer_chunks,
        }
    }

    pub fn init_texture(&mut self, ctx: &egui::Context) {
        if self.texture.is_none() {
            let empty_image = egui::ColorImage::new(
                [self.width_pixels, self.height_pixels],
                egui::Color32::BLACK,
            );
            self.texture =
                Some(ctx.load_texture("viewport_image", empty_image, TextureOptions::default()));
        }
    }

    pub fn stitch_chunks_for_viewport(&self, alive_chunks: &ChunkList) -> Vec<TileKind> {
        let viewport_tiles_x = self.width_tiles + 1;
        let viewport_tiles_y = self.height_tiles + 1;

        let mut tiles = Vec::with_capacity(viewport_tiles_x * viewport_tiles_y);

        let viewport_tile_offset_x = self.offset_x / self.scale as isize;
        let viewport_tile_offset_y = self.offset_y / self.scale as isize;

        // Determine which chunks intersect the viewport
        let first_chunk_x = viewport_tile_offset_x / alive_chunks.chunk_width as isize;
        let first_chunk_y = viewport_tile_offset_y / alive_chunks.chunk_height as isize;
        let last_chunk_x =
            (viewport_tile_offset_x + viewport_tiles_x as isize - 1) / alive_chunks.chunk_width as isize;
        let last_chunk_y =
            (viewport_tile_offset_y + viewport_tiles_y as isize - 1) / alive_chunks.chunk_height as isize;

        for chunk_y in first_chunk_y..=last_chunk_y {
            for row_in_chunk in 0..alive_chunks.chunk_height {
                let world_row = chunk_y * alive_chunks.chunk_height as isize + row_in_chunk as isize;
                if world_row < viewport_tile_offset_y
                    || world_row >= viewport_tile_offset_y + viewport_tiles_y as isize
                {
                    continue;
                }

                for chunk_x in first_chunk_x..=last_chunk_x {
                    let chunk_coord = (chunk_x as i32, chunk_y as i32);
                    let chunk = alive_chunks.get(&chunk_coord);

                    let start_col_in_chunk = if chunk_x == first_chunk_x {
                        viewport_tile_offset_x % alive_chunks.chunk_width as isize
                    } else {
                        0
                    };

                    let end_col_in_chunk = if chunk_x == last_chunk_x {
                        let right =
                            (viewport_tile_offset_x + viewport_tiles_x as isize) % alive_chunks.chunk_width as isize;
                        if right == 0 {
                            alive_chunks.chunk_width as isize
                        } else {
                            right
                        }
                    } else {
                        alive_chunks.chunk_width as isize
                    };

                    if let Some(chunk) = chunk {
                        let start_idx = row_in_chunk * chunk.width + start_col_in_chunk as usize;
                        let end_idx = row_in_chunk * chunk.width + end_col_in_chunk as usize;
                        tiles.extend_from_slice(&chunk.tiles[start_idx..end_idx]);
                    }
                }
            }
        }

        tiles
    }

    pub fn convert_viewport_tiles_to_rgba_buffer(
        &self,
        alive_chunks: &ChunkList,
    ) -> Vec<u8> {
        let mut buffer = vec![0u8; self.width_pixels * self.height_pixels * 4];
        let scale = self.scale as isize;
        let chunk_w = alive_chunks.chunk_width as usize;
        let chunk_h = alive_chunks.chunk_height as usize;

        for pixel_y in 0..self.height_pixels {
            for pixel_x in 0..self.width_pixels {
                // Compute world tile coordinates
                let world_tile_x = (pixel_x as isize + self.offset_x) / scale;
                let world_tile_y = (pixel_y as isize + self.offset_y) / scale;

                // Determine which chunk this tile belongs to
                let chunk_x = div_floor(world_tile_x, chunk_w as isize);
                let chunk_y = div_floor(world_tile_y, chunk_h as isize);

                // Local coordinates inside the chunk
                let local_x = euclidean_mod(world_tile_x, chunk_w) ;
                let local_y = euclidean_mod(world_tile_y, chunk_h);

                let tile = alive_chunks
                    .get(&(chunk_x as i32, chunk_y as i32))
                    .map(|c| c.tiles[local_y as usize * c.width + local_x as usize])
                    .unwrap_or(TileKind::Empty);

                // Convert tile to RGBA
                let mut color = COLORS_RGBA[tile.to_colors().as_u8() as usize];
                color[3] = tile.to_colors().random_alpha(world_tile_x as u32, world_tile_y as u32);

                // Write to pixel buffer
                let dst_idx = (pixel_y * self.width_pixels + pixel_x) * 4;
                buffer[dst_idx..dst_idx + 4].copy_from_slice(&color);
            }
        }

        buffer
    }

    pub fn set_texture_from_chunks(&mut self, ui: &egui::Ui, alive_chunks: &ChunkList) {
        //let visible_tiles = self.stitch_chunks_for_viewport(alive_chunks);

        let viewport_pixels = self.convert_viewport_tiles_to_rgba_buffer(alive_chunks);

        let image = ColorImage::from_rgba_unmultiplied(
            [self.width_pixels, self.height_pixels],
            &viewport_pixels,
        );

        if let Some(texture) = &mut self.texture {
            texture.set(image, egui::TextureOptions::default()); // ✅ update instead of re-allocating
        } else {
            // in case init_texture was never called
            self.texture = Some(ui.ctx().load_texture(
                "viewport_image",
                image,
                TextureOptions::default(),
            ));
        }
    }
}
