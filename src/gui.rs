use std::fs::File;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;
use imgui::{Condition, Context, TreeNodeId, Ui};
use imgui_dx11_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use winit::event::Event;
use serde_derive::{Serialize, Deserialize};
use crate::raytracing::raytracer::{Raytracer, RaytracerState};
use crate::window::Window;

pub const SETTINGS_PATH: &'static str = "./settings.json";

#[derive(Serialize, Deserialize)]
pub struct GUIState {
    pub raytracer_state: RaytracerState,
    pub image_path: String,
    pub selected: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub mode_tree: GUIModeTree
}

impl GUIState {
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Self {
        let file = File::open(path);

        let file = match file {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error loading settings file, using defaults!");
                return Self::default();
            }
        };

        let settings = serde_json::from_reader::<File, GUIState>(file);

        settings.unwrap_or_else(|_| {
            eprintln!("Error deserializing settings file, using defaults!");
            Self::default()
        })
    }

    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) {
        let file = File::create(path);

        let file = match file {
            Ok(file) => file,
            Err(_) => {
                eprintln!("Error creating settings file, settings will not be saved!");
                return;
            }
        };

        let result = serde_json::to_writer(file, self);

        match result {
            Ok(_) => {}
            Err(_) => {
                eprintln!("Error serializing settings file, settings will not be saved!");
            }
        }
    }
}

impl Default for GUIState {
    fn default() -> Self {
        let initial_mode_tree = GUIModeTree {
            selection: 0,
            settings: GUIModeSettings::PixelRandom,
            sub_tree: None
        };

        let imgui_state = GUIState {
            raytracer_state: RaytracerState::Created,
            image_path: "".to_string(),
            selected: 0,
            samples_per_pixel: 100,
            max_bounces: 10,
            mode_tree: initial_mode_tree
        };

        imgui_state
    }
}

pub struct GUIResult {
    pub render_start_button_clicked: bool,
    pub render_stop_button_clicked: bool,
    pub export_button_clicked: bool,
}

pub struct GUI {
    window: Arc<winit::window::Window>,
	state: GUIState,
    context: Context,
    renderer: Renderer,
    platform: WinitPlatform
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GUIModeTree {
    selection: usize,
    pub settings: GUIModeSettings,
    pub sub_tree: Option<Box<GUIModeTree>>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum GUIModeSettings {
    PixelRandom,
    Line {
        reverse_order_horizontal: bool,
        random_order_vertical: bool,
        transparent: bool,
    },
    Stripe {
        random_order_horizontal: bool,
        reverse_order_vertical: bool,
        transparent: bool,
    },
    Tile {
        tile_width: u32,
        tile_height: u32,
        dimension_order: usize,
        horizontal_order: usize,
        vertical_order: usize,
        transparent: bool,
    },
}

impl GUI {
    pub fn new(window: &Window, device: &ID3D11Device) -> Self {
        let mut context = Context::create();

        let mut platform = WinitPlatform::init(&mut context);
        platform.attach_window(context.io_mut(), &window.window, HiDpiMode::Default);

        let renderer = unsafe { Renderer::new(&mut context, &device) }.unwrap();

        let state = GUIState::load_from_file(SETTINGS_PATH);

        Self {
            window: window.window.clone(),
            state,
            context,
            renderer,
            platform
        }
    }

    pub fn update_delta(&mut self, delta: Duration) {
        self.context.io_mut().update_delta_time(delta);
    }

    pub fn update_raytracer_state(&mut self, raytracer: &Box<dyn Raytracer>) {
        let state = raytracer.get_state();

        self.state.raytracer_state = state;
    }

    pub fn handle_event(&mut self, event: Event<()>) {
        self.platform.handle_event(self.context.io_mut(), &self.window, &event);
    }

    pub fn state_mut(&mut self) -> &mut GUIState {
        &mut self.state
    }

    pub fn state(&self) -> &GUIState {
        &self.state
    }

    pub fn draw(&mut self) -> GUIResult {
        self.platform.prepare_frame(self.context.io_mut(), &self.window).unwrap();
        let ui = self.context.frame();

        let (render_start_button_clicked, render_stop_button_clicked) = Self::draw_render_window(&mut self.state, &ui);
        let export_button_clicked = Self::draw_export_window(&mut self.state, &ui);
        Self::draw_order_window(&mut self.state, &ui);

        self.platform.prepare_render(&ui, &self.window);
        self.renderer.render(ui.render()).unwrap();

        GUIResult {
            render_start_button_clicked,
            render_stop_button_clicked,
            export_button_clicked
        }
    }

    fn draw_render_window(state: &mut GUIState, ui: &Ui) -> (bool, bool) {
        let mut render_start_button_clicked: bool = false;
        let mut render_stop_button_clicked: bool = false;

        imgui::Window::new("Render")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(&ui, || {
                let mut samples_per_pixel: i32 = state.samples_per_pixel as i32;
                let mut max_bounces: i32 = state.max_bounces as i32;

                let [text_width, _] = ui.calc_text_size("Samples per Pixel");
                ui.push_item_width(-(text_width + 5.0));
                ui.input_int("Samples per Pixel", &mut samples_per_pixel).build();
                ui.input_int("Max Bounces", &mut max_bounces).build();

                if samples_per_pixel < 1 {
                    samples_per_pixel = 1;
                }
                if max_bounces < 1 {
                    max_bounces = 1;
                }

                state.samples_per_pixel = samples_per_pixel as usize;
                state.max_bounces = max_bounces as usize;

                render_start_button_clicked = ui.button("Start Render");
                render_stop_button_clicked = ui.button("Stop Render");

                ui.text("Rendering:");
                match state.raytracer_state {
                    RaytracerState::Created => {
                        ui.text("Not started...");
                    }
                    RaytracerState::Running { commissioned, completed } => {
                        let percentage = (completed as f32 / commissioned as f32) * 100.0;
                        let text = format!("{}/{} : {}%", completed, commissioned, percentage);

                        ui.text(text);
                    }
                    RaytracerState::Paused { .. } => {
                        ui.text("Paused...");
                    }
                    RaytracerState::Finished { .. } => {
                        ui.text("Done!");
                    }
                    RaytracerState::Stopped => {
                        ui.text("Aborted!");
                    }
                }
            });

        (render_start_button_clicked, render_stop_button_clicked)
    }

    fn draw_export_window(state: &mut GUIState, ui: &Ui) -> bool {
        let mut export_button_clicked: bool = false;

        imgui::Window::new("Export")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(&ui, || {
                let [text_width, _] = ui.calc_text_size("Image Path");
                ui.push_item_width(-(text_width + 5.0));
                imgui::InputText::new(&ui, "Image Path", &mut state.image_path)
                    .build();
                export_button_clicked = ui.button("Export to image");
            });

        export_button_clicked
    }

    fn draw_order_window(state: &mut GUIState, ui: &Ui) {
        imgui::Window::new("Draw Order")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(&ui, || {
                Self::draw_order_tree(0, &mut state.mode_tree, ui);
            });
    }

    fn draw_order_tree(depth: usize, tree: &mut GUIModeTree, ui: &Ui) {
        Self::draw_generation_mode_select(&mut tree.selection, ui);
        Self::update_mode_tree(tree);
        Self::draw_mode_settings(&mut tree.settings, ui);
        Self::draw_sub_tree(depth, &mut tree.sub_tree, ui);
    }

    fn draw_mode_settings(settings: &mut GUIModeSettings, ui: &Ui) {
        match settings {
            GUIModeSettings::PixelRandom => {}
            GUIModeSettings::Line { reverse_order_horizontal, random_order_vertical, transparent } => {
                ui.checkbox("Reverse Horizontal Order", reverse_order_horizontal);
                ui.checkbox("Random Vertical Order", random_order_vertical);
                ui.checkbox("Transparent", transparent);
            }
            GUIModeSettings::Stripe { random_order_horizontal, reverse_order_vertical, transparent } => {
                ui.checkbox("Random Horizontal Order", random_order_horizontal);
                ui.checkbox("Reverse Vertical Order", reverse_order_vertical);
                ui.checkbox("Transparent", transparent);
            }
            GUIModeSettings::Tile { tile_width, tile_height, dimension_order, horizontal_order, vertical_order, transparent } => {
                let mut new_tile_width = *tile_width as i32;
                let mut new_tile_height = *tile_height as i32;

                ui.input_int("Tile Width", &mut new_tile_width).build();
                ui.input_int("Tile Height", &mut new_tile_height).build();
                ui.checkbox("Transparent", transparent);

                if new_tile_width < 1 {
                    new_tile_width = 1;
                }
                if new_tile_height < 1 {
                    new_tile_height = 1;
                }
                *tile_width = new_tile_width as u32;
                *tile_height = new_tile_height as u32;

                Self::draw_tile_dimension_order_select(dimension_order, ui);

                if *dimension_order != 2 {
                    Self::draw_tile_axis_order_select("Horizontal Order", horizontal_order, ui);
                    Self::draw_tile_axis_order_select("Vertical Order", vertical_order, ui);
                }
            }
        }
    }

    fn draw_sub_tree(depth: usize, sub_tree: &mut Option<Box<GUIModeTree>>, ui: &Ui) {
        let sub_tree = match sub_tree {
            None => return,
            Some(sub_tree) => sub_tree
        };

        let id = format!("{}", depth);

        imgui::TreeNode::new(TreeNodeId::Str(&id))
            .label::<&str, &str>("Generation Mode")
            .build(&ui, ||{
                Self::draw_order_tree(depth + 1, sub_tree, ui);
            });
    }

    fn draw_tile_axis_order_select(label: &str, selected_item: &mut usize, ui: &Ui) {
        let [width, _] = ui.calc_text_size(label);
        ui.push_item_width(-(width + 5.0));
        ui.combo_simple_string(label, selected_item, &["Forward", "Reverse", "Random"]);
    }

    fn draw_tile_dimension_order_select(selected_item: &mut usize, ui: &Ui) {
        let [width, _] = ui.calc_text_size("Dimension Order");
        ui.push_item_width(-(width + 5.0));
        ui.combo_simple_string("Dimension Order", selected_item, &["Line First", "Stripe First", "Random"]);
    }

    fn draw_generation_mode_select(selected_item: &mut usize, ui: &Ui) {
        let [width, _] = ui.calc_text_size("Generation Mode");
        ui.push_item_width(-(width + 5.0));
        ui.combo_simple_string("Generation Mode", selected_item, &["Random", "Line", "Stripe", "Tile"]);
    }

    fn update_mode_tree(tree: &mut GUIModeTree) {
        if tree.selection != mode_settings_to_index(&tree.settings) {
            let new_settings = index_to_mode_settings(tree.selection);
            let new_sub_tree = index_to_sub_tree(tree.selection);

            tree.settings = new_settings;
            tree.sub_tree = new_sub_tree;
        }
    }
}

impl Drop for GUI {
    fn drop(&mut self) {
        self.state.save_to_file(SETTINGS_PATH);
    }
}

fn mode_settings_to_index(mode: &GUIModeSettings) -> usize {
    match mode {
        GUIModeSettings::PixelRandom => 0,
        GUIModeSettings::Line { .. } => 1,
        GUIModeSettings::Stripe { .. } => 2,
        GUIModeSettings::Tile { .. } => 3,
    }
}

fn index_to_mode_settings(index: usize) -> GUIModeSettings {
    match index {
        1 => {
            GUIModeSettings::Line {
                reverse_order_horizontal: false,
                random_order_vertical: false,
                transparent: false
            }
        },
        2 => {
            GUIModeSettings::Stripe {
                random_order_horizontal: false,
                reverse_order_vertical: false,
                transparent: false
            }
        }
        3 => {
            GUIModeSettings::Tile {
                tile_width: 10,
                tile_height: 10,
                dimension_order: 0,
                horizontal_order: 0,
                vertical_order: 0,
                transparent: false
            }
        }
        _ => GUIModeSettings::PixelRandom,

    }
}

fn index_to_sub_tree(index: usize) -> Option<Box<GUIModeTree>> {
    match index {
        3 => {
            Some(Box::new(GUIModeTree {
                selection: 0,
                settings: GUIModeSettings::PixelRandom,
                sub_tree: None
            }))
        }
        _ => None
    }
}