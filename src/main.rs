#![windows_subsystem = "windows"]

use notan::draw::*;
use notan::prelude::*;
use notan::random::rand::prelude::SliceRandom;
use rand::thread_rng;
use winapi::um::winuser::{
    SetWindowLongW, GWL_EXSTYLE, GWL_STYLE, WS_BORDER, WS_CAPTION, WS_CLIPCHILDREN, WS_CLIPSIBLINGS, WS_EX_ACCEPTFILES,
    WS_EX_LAYERED, WS_EX_TOPMOST, WS_EX_TRANSPARENT, WS_EX_WINDOWEDGE, WS_MINIMIZEBOX, WS_SYSMENU, WS_VISIBLE, WS_EX_TOOLWINDOW
};

use winapi::shared::windef::HWND;

const IMAGE_SIZE: f32 = 200.0;


#[derive(AppState)]
struct State {
    imgs: Vec<TrollImage>,
    font: Font,
    normal_font: Font,
    colors: Vec<Color>,
    progress: f32,
}

struct TrollImage {
    increment: (i32, i32),
    position: (i32, i32),
    img: Texture,
}

#[notan_main]
fn main() -> Result<(), String> {
    // Check the documentation for more options
    let window_config = WindowConfig::new()
        .title("D2R Map hack")
        // .fullscreen(true)
        
        .size(1500, 1000)
        .decorations(false)
        .mouse_passthrough(true)
        .vsync(true)
        .transparent(true)
        .always_on_top(true);

    notan::init_with(init).add_config(window_config)
        .add_config(DrawConfig)
        .update(update)
        .draw(draw)
        .build()
}

fn update(app: &mut App, state: &mut State) {
    let mut rng = rand::thread_rng();
    let inc: f32 = rng.gen::<f32>() / 1000.0;
    state.progress += inc;
    if state.progress > 1.0 {
        state.progress = 1.0;
    }
    if app.keyboard.was_pressed(KeyCode::Space) {  
        std::process::exit(0)
    }
    for img in state.imgs.iter_mut() {
        if *&img.position.0 as f32 > app.window().width() as f32 - IMAGE_SIZE {
            img.increment.0 = -2
        }
        if *&img.position.0 < 1 {
            img.increment.0 = 2
        }
        if *&img.position.1 as f32 > app.window().height() as f32 - IMAGE_SIZE {
            img.increment.1 = -2
        }
        if *&img.position.1 < 1 {
            img.increment.1 = 2
        }
        
        img.position.0 += img.increment.0;
        img.position.1 += img.increment.1;
    }
    app.window().set_position(0, 0);
    unsafe {
        let hwnd = app.window().id() as isize as HWND;
        let mut style =
            WS_CAPTION | WS_MINIMIZEBOX | WS_BORDER | WS_CLIPSIBLINGS | WS_CLIPCHILDREN | WS_SYSMENU;
        let mut style_ex = WS_EX_WINDOWEDGE | WS_EX_ACCEPTFILES;
        style |= WS_VISIBLE;
        style_ex |= WS_EX_TOPMOST;
        style_ex |= WS_EX_TRANSPARENT | WS_EX_LAYERED | WS_EX_TOOLWINDOW;
        SetWindowLongW(hwnd, GWL_STYLE, style as i32);
        SetWindowLongW(hwnd, GWL_EXSTYLE, style_ex as i32);
    }   
}

fn init(app: &mut App, gfx: &mut Graphics) -> State {
    let font = gfx
        .create_font(include_bytes!("assets/Qdbettercomicsansbold-511d8.ttf"))
        .unwrap();
    let normal_font = gfx
        .create_font(include_bytes!("assets/Ubuntu-B.ttf"))
        .unwrap();
    
    let texture_kekw = gfx
        .create_texture()
        .from_image(include_bytes!("assets/kekw-emote.png"))
        .build()
        .unwrap();
    
    let texture_troll = gfx
        .create_texture()
        .from_image(include_bytes!("assets/troll.png"))
        .build()
        .unwrap();

    let texture_rage = gfx
        .create_texture()
        .from_image(include_bytes!("assets/rage.png"))
        .build()
        .unwrap();

    let mut imgs = vec![];
    imgs.push(TrollImage { increment: (2, -2), position: (200, 300), img: texture_kekw.clone() });
    imgs.push(TrollImage { increment: (-2, 2), position: (50, 200), img: texture_kekw.clone() });
    imgs.push(TrollImage { increment: (2, 2), position: (500, 600), img: texture_troll.clone() });
    imgs.push(TrollImage { increment: (-2, 2), position: (700, 50), img: texture_rage.clone() });
    imgs.push(TrollImage { increment: (-2, -2), position: (400, 650), img: texture_rage.clone() });

    match app.audio.create_source(include_bytes!("assets/careless.mp3")) {
        Ok(music) => {
            app.audio.play_sound(
            &music,
            1.0,
            true,
        );},
        Err(_) => (),
    }
    let colors = vec![
        Color::ORANGE,
        Color::RED,
        Color::YELLOW,
        Color::GREEN,
        Color::PURPLE,
        Color::BLUE,
        Color::PINK];
    State { imgs, font, normal_font, colors, progress: 0.1 }
}

fn draw(gfx: &mut Graphics, state: &mut State) {
    let mut draw = gfx.create_draw();
    draw.clear(Color::TRANSPARENT);
    
    for img in state.imgs.iter() {
        draw.image(&img.img).position(img.position.0 as f32, img.position.1 as f32).size(IMAGE_SIZE, IMAGE_SIZE);
    }

    let color = &state.colors.choose(&mut thread_rng()).unwrap();
    draw.text(&state.font, "REKT!")
        .position(gfx.size().0 as f32 / 2.0, gfx.size().1 as f32 / 2.0)
        .size(256.0)
        .color(**color)
        .h_align_center()
        .v_align_middle();

    let progressbox = ((gfx.size().0 / 2) as f32 - 250.0, (gfx.size().1 / 2) as f32 + 150.0);
    let progressbar = ((progressbox.0 + 10.0) as f32, (progressbox.1 + 42.0) as f32);
    let progressboxsize = (500.0, 70.0);
    let progressbarsize = (480.0 * state.progress, 12.0);
    draw.rect(progressbox, progressboxsize).corner_radius(4.0).fill().fill_color(Color::from_hex(0x222222FF));
    if state.progress == 1.0 {
        draw.text(&state.normal_font, "JUST KIDDING").position(progressbox.0 + 10.0, progressbox.1 + 40.0).size(20.0).color(Color::GREEN);
    } else {
        draw.rect(progressbar, progressbarsize).corner_radius(4.0).fill().fill_color(Color::GREEN);
    }
    draw.text(&state.normal_font, "Installing cryto miner....").position(progressbox.0 + 10.0, progressbox.1 + 10.0).size(24.0).color(Color::WHITE);
    gfx.render(&draw);
}
