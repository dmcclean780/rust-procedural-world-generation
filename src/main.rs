use eframe::egui;

mod action;
mod bresenham;
mod chunk;
mod chunk_list;
mod colors;
mod tile_map;
mod tiles;
mod viewport;

use bresenham::plot_line;
use chunk_list::ChunkList;
use egui::{ComboBox, Pos2, Vec2};
use std::time::Instant;
use tiles::tile_kind::TileKind;
use viewport::Viewport;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1920.0, 1080.0]), // width, height
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

struct MyApp {
    viewport: Viewport,
    chunks: ChunkList,
    simulation_running: bool,
    simulation_speed: u8,
    current_frame: u64,
    frame_timer: FrameTimer,
    brush_size: usize,
    brush_element: TileKind,
    last_mouse_pos: Option<Pos2>,
}

impl Default for MyApp {
    fn default() -> Self {
        let scale = 4;
        let viewport_width = 1280;
        let viewport_height = 720;

        let chunk_width = 32;
        let chunk_height = 32;
        let chunk_x_num = 32;
        let chunk_y_num = 32;
        let buffer_chunks = 32;

        let starting_speed = 100;
        let initial_run_state = true;

        // Create viewport
        let viewport = Viewport::new(viewport_width, viewport_height, scale, buffer_chunks);

        Self {
            viewport,
            chunks: ChunkList::new(chunk_width, chunk_height, chunk_x_num, chunk_y_num),
            simulation_running: initial_run_state,
            simulation_speed: starting_speed,
            current_frame: 0,
            frame_timer: FrameTimer::new(),
            brush_size: 3,
            last_mouse_pos: None,
            brush_element: TileKind::GameOfLife,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.viewport.init_texture(ctx);

        self.update_if_needed();

        egui::SidePanel::left("side_panel")
            .exact_width(300.0)
            .show(ctx, |ui| {
                self.create_left_control_panel(ui, ctx);
            });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.create_central_panel(ui, ctx);
        });
        egui::SidePanel::right("right_panel")
            .exact_width(200.0)
            .show(ctx, |ui| {
                ui.heading("Brush Size");
                ui.add(egui::Slider::new(&mut self.brush_size, 1..=21).text("Brush Size"));
                ui.horizontal(|ui| {
                    self.tile_kind_selector(ui);
                });
            });

        ctx.request_repaint();

        self.current_frame = self.current_frame.wrapping_add(1);
        self.frame_timer.tick();
    }
}

impl MyApp {
    fn should_update(&self) -> bool {
        let speed = self.simulation_speed.clamp(0, 99);

        // Compute the interval: higher speed = smaller interval
        let interval = 101 - speed; // Maps 1-100 to 100-1
        // Update every `interval` frames
        let should_update = self.simulation_running && (self.current_frame % interval as u64 == 0);
        should_update
    }

    fn tile_kind_selector( &mut self, ui: &mut egui::Ui) {
        ComboBox::from_label("Tile Type")
            .selected_text(format!("{:?}", self.brush_element))
            .show_ui(ui, |ui| {
                ui.selectable_value(&mut self.brush_element, TileKind::Empty, "Empty");
                ui.selectable_value(&mut self.brush_element, TileKind::GameOfLife, "Game of Life");
                ui.selectable_value(&mut self.brush_element, TileKind::Sand, "Sand");
            });
    }

    fn update_if_needed(&mut self) {
        if self.should_update() {
            self.chunks.revive_chunks_near_viewport(
                self.viewport.offset_x as isize / self.viewport.scale as isize,
                self.viewport.offset_y as isize / self.viewport.scale as isize,
                self.viewport.width_tiles,
                self.viewport.height_tiles,
                self.viewport.buffer_chunks, // buffer chunks
            );
            self.chunks.cull_chunks(
                self.viewport.offset_x as isize / self.viewport.scale as isize,
                self.viewport.offset_y as isize / self.viewport.scale as isize,
                self.viewport.width_tiles,
                self.viewport.height_tiles,
                self.viewport.buffer_chunks, // buffer chunks
            );
            self.chunks.update();
        }
    }

    fn create_left_control_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.heading("Controls");

        ui.add(egui::Slider::new(&mut self.simulation_speed, 1..=100).text("Simulation Speed"));
        if ui
            .button(if self.simulation_running {
                "Pause"
            } else {
                "Run"
            })
            .clicked()
        {
            self.simulation_running = !self.simulation_running;
        }

        if ui.button("Reset Viewport").clicked() {
            self.viewport.offset_x = 0;
            self.viewport.offset_y = 0;
        }
        ui.add_enabled_ui(!self.simulation_running, |ui| {
            if ui.button("Step Simulation").clicked() {
                self.chunks.update();
            }
        });

        ui.label(format!("FPS: {:.1}", self.frame_timer.get_fps()));
    }

    fn calculate_max_pixel_coord(&self, max_chunk_x: i32, max_chunk_y: i32) -> (i32, i32) {
        let max_pixels_x =
            (max_chunk_x + 1) * self.chunks.chunk_width as i32 * self.viewport.scale as i32
                - self.viewport.width_pixels as i32;
        let max_pixels_y =
            (max_chunk_y + 1) * self.chunks.chunk_height as i32 * self.viewport.scale as i32
                - self.viewport.height_pixels as i32;
        (max_pixels_x, max_pixels_y)
    }

    fn calculate_min_pixel_coord(&self, min_chunk_x: i32, min_chunk_y: i32) -> (i32, i32) {
        let min_pixels_x =
            min_chunk_x * self.chunks.chunk_width as i32 * self.viewport.scale as i32;
        let min_pixels_y =
            min_chunk_y * self.chunks.chunk_height as i32 * self.viewport.scale as i32;
        (min_pixels_x, min_pixels_y)
    }

    fn move_viewport(&mut self, delta: Vec2) {
        let mut min_chunk_x = i32::MAX;
        let mut min_chunk_y = i32::MAX;
        let mut max_chunk_x = i32::MIN;
        let mut max_chunk_y = i32::MIN;

        for (&(chunk_x, chunk_y), _) in self.chunks.iter() {
            min_chunk_x = min_chunk_x.min(chunk_x);
            min_chunk_y = min_chunk_y.min(chunk_y);
            max_chunk_x = max_chunk_x.max(chunk_x);
            max_chunk_y = max_chunk_y.max(chunk_y);
        }

        // Convert chunk coordinates to pixel bounds
        let (min_pixels_x, min_pixels_y) = self.calculate_min_pixel_coord(min_chunk_x, min_chunk_y);
        let (max_pixels_x, max_pixels_y) = self.calculate_max_pixel_coord(max_chunk_x, max_chunk_y);

        // Update offsets based on mouse drag
        self.viewport.offset_x = ((self.viewport.offset_x as isize - delta.x as isize) as i32)
            .clamp(min_pixels_x, max_pixels_x) as usize;

        self.viewport.offset_y = ((self.viewport.offset_y as isize - delta.y as isize) as i32)
            .clamp(min_pixels_y, max_pixels_y) as usize;
    }

    pub fn paint_line(&mut self, start: Pos2, end: Pos2, brush_size: usize) {
        // Convert screen coordinates to world coordinates
        let start_tile_x = (self.viewport.offset_x as f32 + start.x) as usize / self.viewport.scale;
        let start_tile_y = (self.viewport.offset_y as f32 + start.y) as usize / self.viewport.scale;

        let end_tile_x = (self.viewport.offset_x as f32 + end.x) as usize / self.viewport.scale;
        let end_tile_y = (self.viewport.offset_y as f32 + end.y) as usize / self.viewport.scale;

        // Use Bresenham’s algorithm to get all the tiles along the line
        let line_points = plot_line(
            start_tile_x as isize,
            start_tile_y as isize,
            end_tile_x as isize,
            end_tile_y as isize,
        );

        let brush_radius = (brush_size / 2) as isize;

        for (tx, ty) in line_points {
            for dy in -brush_radius..=brush_radius {
                for dx in -brush_radius..=brush_radius {
                    let px = tx + dx;
                    let py = ty + dy;

                    if px < 0 || py < 0 {
                        continue;
                    }

                    let px = px as usize;
                    let py = py as usize;

                    let chunk_width = self.chunks.chunk_width;
                    let chunk_height = self.chunks.chunk_height;

                    let chunk_x = (px / chunk_width) as i32;
                    let chunk_y = (py / chunk_height) as i32;

                    let local_x = px % chunk_width;
                    let local_y = py % chunk_height;

                    if let Some(chunk) = self.chunks.alive_chunks.get_mut(&(chunk_x, chunk_y)) {
                        chunk.tiles[local_y * chunk.width + local_x] = self.brush_element;
                        chunk.mark_dirty();
                    }
                }
            }
        }
    }

    fn create_central_panel(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        self.viewport.set_texture_from_chunks(ui, &self.chunks);

        // Allocate a region for the viewport that can receive drag input
        let (rect, response) = ui.allocate_exact_size(
            egui::vec2(
                self.viewport.width_pixels as f32,
                self.viewport.height_pixels as f32,
            ),
            egui::Sense::click_and_drag(),
        );

        // Handle right-click drag
        if response.dragged_by(egui::PointerButton::Secondary) {
            let delta = response.drag_delta();

            self.move_viewport(delta);
        }

        if response.dragged_by(egui::PointerButton::Primary) {
            if let Some(mouse_pos) = response.hover_pos() {
                let local_pos = mouse_pos - rect.min;
                let mouse_pos = Pos2::new(local_pos.x, local_pos.y);
                if let Some(last) = self.last_mouse_pos {
                    // Draw line between last and current mouse positions
                    self.paint_line(last, mouse_pos, self.brush_size);
                } else {
                    // First paint (single point)
                    //self.paint(mouse_pos, self.brush_size);
                }
                // Update last mouse position
                self.last_mouse_pos = Some(mouse_pos);
            }
        } else {
            // Reset when not dragging
            self.last_mouse_pos = None;
        }

        // Draw the texture inside the allocated rectangle
        if let Some(texture) = &self.viewport.texture {
            ui.painter().image(
                texture.id(),
                rect,
                egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0)),
                egui::Color32::WHITE,
            );
        }
    }
}

struct FrameTimer {
    last_frame: Instant,
    fps: f32,
}

impl FrameTimer {
    fn new() -> Self {
        Self {
            last_frame: Instant::now(),
            fps: 0.0,
        }
    }

    fn tick(&mut self) {
        let now = Instant::now();
        let delta = now.duration_since(self.last_frame);
        self.last_frame = now;

        // Compute instantaneous FPS
        self.fps = 1.0 / delta.as_secs_f32();
    }

    fn get_fps(&self) -> f32 {
        self.fps
    }
}
