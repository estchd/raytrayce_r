use std::sync::Arc;
use std::time::Duration;
use winit::window::Window;
use imgui::{Condition, Context, TreeNodeId, Ui};
use imgui_dx11_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use windows::Win32::Graphics::Direct3D11::ID3D11Device;
use winit::event::Event;

pub struct GUIState {
    pub completed_count: usize,
    pub commissioned_count: usize,
    pub image_path: String,
    pub combo_open: bool,
    pub selected: usize,
    pub samples_per_pixel: usize,
    pub max_bounces: usize,
    pub mode_tree: GUIModeTree
}

pub struct GUIResult {
    pub render_start_button_clicked: bool,
    pub render_stop_button_clicked: bool,
    pub export_button_clicked: bool,
}

pub struct GUI {
    window: Arc<Window>,
	state: GUIState,
    context: Context,
    renderer: Renderer,
    platform: WinitPlatform
}

#[derive(Debug, Clone)]
pub struct GUIModeTree {
    selection: usize,
    pub settings: GUIModeSettings,
    pub sub_tree: Option<Box<GUIModeTree>>
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum GUIModeSettings {
    PixelRandom,
    Line {
        reverse_order_horizontal: bool,
        random_order_vertical: bool
    },
    PixelLine {
        reverse_order_horizontal: bool,
        random_order_vertical: bool
    },
    Stripe {
        random_order_horizontal: bool,
        reverse_order_vertical: bool
    },
    PixelStripe {
        random_order_horizontal: bool,
        reverse_order_vertical: bool
    },
    LineFirstTile {
        tile_width: u32,
        tile_height: u32,
        horizontal_order: usize,
        vertical_order: usize
    },
    StripeFirstTile {
        tile_width: u32,
        tile_height: u32,
        horizontal_order: usize,
        vertical_order: usize
    },
    LineFirstPixelTile {
        tile_width: u32,
        tile_height: u32,
        horizontal_order: usize,
        vertical_order: usize
    },
    StripeFirstPixelTile {
        tile_width: u32,
        tile_height: u32,
        horizontal_order: usize,
        vertical_order: usize
    }
}

impl GUI {
    pub fn new(window: Arc<Window>, device: &ID3D11Device) -> Self {
        let mut imgui_context = Context::create();

        let mut imgui_winit_platform = WinitPlatform::init(&mut imgui_context);
        imgui_winit_platform.attach_window(imgui_context.io_mut(), &window, HiDpiMode::Default);

        let imgui_renderer = unsafe { Renderer::new(&mut imgui_context, &device) }.unwrap();

        let initial_mode_tree = GUIModeTree {
            selection: 0,
            settings: GUIModeSettings::PixelRandom,
            sub_tree: None
        };
        
        let imgui_state = GUIState {
            completed_count: 0,
            commissioned_count: 0,
            image_path: "".to_string(),
            combo_open: false,
            selected: 0,
            samples_per_pixel: 100,
            max_bounces: 10,
            mode_tree: initial_mode_tree
        };

        Self {
            window,
            state: imgui_state,
            context: imgui_context,
            renderer: imgui_renderer,
            platform: imgui_winit_platform
        }
    }

    pub fn update_delta(&mut self, delta: Duration) {
        self.context.io_mut().update_delta_time(delta);
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
                if state.completed_count >= state.commissioned_count {
                    ui.text("Done!");
                }
                else {
                    let percentage = (state.completed_count as f32 / state.commissioned_count as f32) * 100.0;
                    let text = format!("{}/{} : {}%", state.completed_count, state.commissioned_count, percentage);

                    ui.text(text);
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
            GUIModeSettings::Line { reverse_order_horizontal, random_order_vertical } |
            GUIModeSettings::PixelLine { reverse_order_horizontal, random_order_vertical } => {
                ui.checkbox("Reverse Horizontal Order", reverse_order_horizontal);
                ui.checkbox("Random Vertical Order", random_order_vertical);
            }
            GUIModeSettings::Stripe { random_order_horizontal, reverse_order_vertical } |
            GUIModeSettings::PixelStripe { random_order_horizontal, reverse_order_vertical } => {
                ui.checkbox("Random Horizontal Order", random_order_horizontal);
                ui.checkbox("Reverse Vertical Order", reverse_order_vertical);
            }
            GUIModeSettings::LineFirstTile { tile_width, tile_height, horizontal_order, vertical_order } |
            GUIModeSettings::StripeFirstTile { tile_width, tile_height, horizontal_order, vertical_order } |
            GUIModeSettings::LineFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order } |
            GUIModeSettings::StripeFirstPixelTile { tile_width, tile_height, horizontal_order, vertical_order } => {
                let mut new_tile_width = *tile_width as i32;
                let mut new_tile_height = *tile_height as i32;

                ui.input_int("Tile Width", &mut new_tile_width).build();
                ui.input_int("Tile Height", &mut new_tile_height).build();

                if new_tile_width < 1 {
                    new_tile_width = 1;
                }
                if new_tile_height < 1 {
                    new_tile_height = 1;
                }
                *tile_width = new_tile_width as u32;
                *tile_height = new_tile_height as u32;

                Self::draw_tile_axis_order_select("Horizontal Order", horizontal_order, ui);
                Self::draw_tile_axis_order_select("Vertical Order", vertical_order, ui);
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

    fn draw_generation_mode_select(selected_item: &mut usize, ui: &Ui) {
        let [width, _] = ui.calc_text_size("Generation Mode");
        ui.push_item_width(-(width + 5.0));
        ui.combo_simple_string("Generation Mode", selected_item, &["PixelRandom", "Line", "PixelLine", "Stripe", "PixelStripe", "LineFirstTile", "StripeFirstTile", "LineFirstPixelTile", "StripeFirstPixelTile"]);
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

fn mode_settings_to_index(mode: &GUIModeSettings) -> usize {
    match mode {
        GUIModeSettings::PixelRandom => 0,
        GUIModeSettings::Line { .. } => 1,
        GUIModeSettings::PixelLine { .. } => 2,
        GUIModeSettings::Stripe { .. } => 3,
        GUIModeSettings::PixelStripe { .. } => 4,
        GUIModeSettings::LineFirstTile { .. } => 5,
        GUIModeSettings::StripeFirstTile { .. } => 6,
        GUIModeSettings::LineFirstPixelTile { .. } => 7,
        GUIModeSettings::StripeFirstPixelTile { .. } => 8
    }
}

fn index_to_mode_settings(index: usize) -> GUIModeSettings {
    match index {
        1 => {
            GUIModeSettings::Line {
                reverse_order_horizontal: false,
                random_order_vertical: false
            }
        },
        2 => {
            GUIModeSettings::PixelLine {
                reverse_order_horizontal: false,
                random_order_vertical: false
            }
        }
        3 => {
            GUIModeSettings::Stripe {
                random_order_horizontal: false,
                reverse_order_vertical: false
            }
        }
        4 => {
            GUIModeSettings::PixelStripe {
                random_order_horizontal: false,
                reverse_order_vertical: false
            }
        }
        5 => {
            GUIModeSettings::LineFirstTile {
                tile_width: 10,
                tile_height: 10,
                horizontal_order: 0,
                vertical_order: 0
            }
        }
        6 => {
            GUIModeSettings::StripeFirstTile {
                tile_width: 10,
                tile_height: 10,
                horizontal_order: 0,
                vertical_order: 0
            }
        }
        7 => {
            GUIModeSettings::LineFirstPixelTile {
                tile_width: 10,
                tile_height: 10,
                horizontal_order: 0,
                vertical_order: 0
            }
        }
        8 => {
            GUIModeSettings::StripeFirstPixelTile {
                tile_width: 10,
                tile_height: 10,
                horizontal_order: 0,
                vertical_order: 0
            }
        }
        _ => GUIModeSettings::PixelRandom,

    }
}

fn index_to_sub_tree(index: usize) -> Option<Box<GUIModeTree>> {
    match index {
        5 | 6 | 7 | 8 => {
            Some(Box::new(GUIModeTree {
                selection: 0,
                settings: GUIModeSettings::PixelRandom,
                sub_tree: None
            }))
        }
        _ => None
    }
}