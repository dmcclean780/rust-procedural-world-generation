use egui::{ColorImage, TextureHandle, TextureOptions};

use crate::{chunk_list::ChunkList, colors::COLORS_RGBA, tiles::tile_kind::TileKind};

pub struct Viewport {
    pub width_pixels: usize,
    pub height_pixels: usize,
    pub width_tiles: usize,
    pub height_tiles: usize,
    pub scale: usize, // pixels per tile
    pub texture: Option<TextureHandle>,
    pub offset_x: usize, // pixels
    pub offset_y: usize, // pixels
    pub buffer_chunks: usize, // number of chunks to buffer around the viewport
}

impl Viewport {
    pub fn new(width_pixels: usize, height_pixels: usize, scale: usize, buffer_chunks: usize) -> Self {
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
            self.texture = Some(ctx.load_texture(
                "viewport_image",
                empty_image,
                TextureOptions::default(),
            ));
        }
    }

    pub fn stitch_chunks_for_viewport(&self, alive_chunks: &ChunkList) -> Vec<TileKind> {
        let viewport_tiles_x = self.width_tiles + 1;
        let viewport_tiles_y = self.height_tiles + 1;

        let mut tiles = Vec::with_capacity(viewport_tiles_x * viewport_tiles_y);

        let viewport_tile_offset_x = self.offset_x / self.scale;
        let viewport_tile_offset_y = self.offset_y / self.scale;

        // Determine which chunks intersect the viewport
        let first_chunk_x = viewport_tile_offset_x / alive_chunks.chunk_width;
        let first_chunk_y = viewport_tile_offset_y / alive_chunks.chunk_height;
        let last_chunk_x  = (viewport_tile_offset_x + viewport_tiles_x - 1) / alive_chunks.chunk_width;
        let last_chunk_y  = (viewport_tile_offset_y + viewport_tiles_y - 1) / alive_chunks.chunk_height;

        for chunk_y in first_chunk_y..=last_chunk_y {
            for row_in_chunk in 0..alive_chunks.chunk_height {
                let world_row = chunk_y * alive_chunks.chunk_height + row_in_chunk;
                if world_row < viewport_tile_offset_y || world_row >= viewport_tile_offset_y + viewport_tiles_y {
                    continue;
                }

                for chunk_x in first_chunk_x..=last_chunk_x {
                    let chunk_coord = (chunk_x as i32, chunk_y as i32);
                    let chunk = alive_chunks.get(&chunk_coord);

                    let start_col_in_chunk = if chunk_x == first_chunk_x {
                        viewport_tile_offset_x % alive_chunks.chunk_width
                    } else {
                        0
                    };

                    let end_col_in_chunk = if chunk_x == last_chunk_x {
                        let right = (viewport_tile_offset_x + viewport_tiles_x) % alive_chunks.chunk_width;
                        if right == 0 { alive_chunks.chunk_width } else { right }
                    } else {
                        alive_chunks.chunk_width
                    };

                    if let Some(chunk) = chunk {
                        let start_idx = row_in_chunk * chunk.width + start_col_in_chunk;
                        let end_idx = row_in_chunk * chunk.width + end_col_in_chunk;
                        tiles.extend_from_slice(&chunk.tiles[start_idx..end_idx]);
                    } 
                }
            }
        }

        tiles
    }

    pub fn convert_viewport_tiles_to_rgba_buffer(&self, visible_tiles: &Vec<TileKind>) -> Vec<u8> {
        let mut viewport_pixels = vec![0u8; self.width_pixels * self.height_pixels * 4];

        let tile_stride = self.width_tiles + 1;
        let sub_x = self.offset_x % self.scale;
        let sub_y = self.offset_y % self.scale;

        for y in 0..self.height_pixels {
            let src_tile_y = (y + sub_y) / self.scale;

            for x in 0..self.width_pixels {
                let src_tile_x = (x + sub_x) / self.scale;

                let tile_idx = src_tile_y * tile_stride + src_tile_x;
                let color = COLORS_RGBA[visible_tiles[tile_idx].to_colors().as_u8() as usize];

                let dst_idx = (y * self.width_pixels + x) * 4;
                viewport_pixels[dst_idx..dst_idx + 4].copy_from_slice(&color);
            }
        }

        viewport_pixels
    }

    pub fn set_texture_from_chunks(&mut self, ui: &egui::Ui, alive_chunks: &ChunkList) {
        let visible_tiles = self.stitch_chunks_for_viewport(alive_chunks);

        let viewport_pixels = self.convert_viewport_tiles_to_rgba_buffer(&visible_tiles);

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
