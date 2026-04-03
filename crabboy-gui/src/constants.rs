// ImGui window constants
pub const SCALE: i32 = 3;
pub const TILE_SCALE: i32 = 2;

pub const WINDOW_WIDTH: u32 = 1200;
pub const WINDOW_HEIGHT: u32 = 700;

pub const GB_SCREEN_WIDTH: u32 = 160 * (SCALE as u32);
pub const GB_SCREEN_HEIGHT: u32 = 150 * (SCALE as u32);
pub const GB_SCREEN_X: f32 = 0.0;
pub const GB_SCREEN_Y: f32 = 50.0;
pub const GB_POS: [f32; 2] = [GB_SCREEN_X, GB_SCREEN_Y];
pub const GB_SCREEN_SIZE: [f32; 2] = [
    (GB_SCREEN_WIDTH + 10) as f32,
    (GB_SCREEN_HEIGHT + 10) as f32,
];

pub const DISPLAY_INFO_WIDTH: f32 = 200.0;
pub const DISPLAY_INFO_HEIGHT: f32 = 400.0;
pub const DISPLAY_INFO_X: f32 = GB_SCREEN_SIZE[0] + DISPLAY_INFO_WIDTH;
pub const DISPLAY_INFO_Y: f32 = GB_SCREEN_Y;

pub const TILE_SCREEN_WIDTH: u32 = 18 * 8 * (TILE_SCALE as u32);
pub const TILE_SCREEN_HEIGHT: u32 = 28 * 8 * (TILE_SCALE as u32);
pub const TILE_SCREEN_X: f32 = DISPLAY_INFO_X + DISPLAY_INFO_WIDTH;
pub const TILE_SCREEN_Y: f32 = GB_SCREEN_Y;

pub const DEBUG_WINDOW_WIDTH: f32 = 150.0;
pub const DEBUG_WINDOW_HEIGHT: f32 = 200.0;
pub const DEBUG_WINDOW_X: f32 = GB_SCREEN_SIZE[0];
pub const DEBUG_WINDOW_Y: f32 = GB_SCREEN_Y;
