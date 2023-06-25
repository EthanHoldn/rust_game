extern crate sdl2;
use crate::world::{self, TileType};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator, Texture};
use sdl2::ttf::{Font, Sdl2TtfContext};
use sdl2::video::Window;
use sdl2::EventPump;
use std::time::{Duration, Instant};

pub(crate) struct Camera {
    pub x_offset: f32, // Camera pos and zoom
    pub y_offset: f32,
    pub zoom: f32,
    pub movement_speed: f32,
    pub zoom_speed: f32,
    pub window_width: f32,
    pub window_height: f32,
}
pub(crate) struct WindowContext {
    pub canvas: Canvas<Window>,
    pub event_pump: EventPump,
    pub ttf_context: Sdl2TtfContext,
}

pub(crate) fn init() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    let window = video_subsystem
        .window("game", 800, 600)
        .allow_highdpi()
        .resizable()
        .build()
        .unwrap();
    let mut wc = WindowContext{
        canvas: window.into_canvas().build().unwrap(),
        event_pump: sdl_context.event_pump().unwrap(),
        ttf_context: sdl2::ttf::init().unwrap(),
    };


    run(&mut wc);
}

fn run(wc: &mut WindowContext) {
    let target_fps = 60;
    let target_frame_time = Duration::from_secs(1) / target_fps;

    //map data
    let mut map = world::Map {
        size: 500,
        terrain: Vec::<TileType>::new(),
        image: Vec::<u8>::new(),
        plain_thresh: 0.0,
        mountain_thresh: 0.0,
    };

    //camera data
    let mut camera = Camera {
        x_offset: 0.0,
        y_offset: 0.0,
        zoom: 4.0,
        zoom_speed: 0.02,
        movement_speed: 5.0,
        window_width: 1600.0,
        window_height: 1200.0,
    };

    //generate map data
    map.generate_layers();
    map.create_image();

    //used to generate textures from a Vec<u8>
    let texture_creator = wc.canvas.texture_creator();

    //map image texture
    let mut map_texture = texture_creator
        .create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size)
        .unwrap();

    map_texture
        .update(None, &map.image, map.size as usize * 4)
        .unwrap();

    //frame rate calculation
    let mut previous_frame_start = Instant::now();

    //main  window rendering loop
    //all window related operations need to be done in here

    let mut key_states: [bool; 238] = [false; 238];
    'running: loop {
        wc.canvas.clear();

        //get user inputs
        if inputs(wc, &mut map, &mut camera, &mut key_states) {
            break 'running;
        }

        texture_creator
            .create_texture_streaming(PixelFormatEnum::RGBA32, map.size, map.size)
            .unwrap();

        map_texture
            .update(None, &map.image, map.size as usize * 4)
            .unwrap();

        wc.canvas
            .copy(
                &map_texture,
                None,
                Rect::new(
                    camera.x_offset as i32,
                    camera.y_offset as i32,
                    (camera.zoom * map.size as f32) as u32,
                    (camera.zoom * map.size as f32) as u32,
                ),
            )
            .unwrap();
        let elapsed = previous_frame_start.elapsed();
        display_text(wc, &((elapsed.as_nanos() as f64 )/1_000_000.0).to_string());
        let _ = wc.canvas.present();
        if elapsed < target_frame_time {
            std::thread::sleep(target_frame_time - elapsed);
        }
        previous_frame_start = Instant::now();
        for (index, &state) in key_states.iter().enumerate() {
            if state {
                println!("Key {:?} is pressed", index as i32);
            }
        }
    }
}

fn display_text(wc: &mut WindowContext, text: &str ){
    let font: Font = wc.ttf_context.load_font("assets/fonts/FiraSans-Bold.ttf", 50 ).unwrap();

    let text_surface = font
        .render(text)
        .blended(Color::BLACK)
        .unwrap();

        let text_texture_creator: TextureCreator<_> = wc.canvas.texture_creator();
        let text_texture: Texture = text_texture_creator
            .create_texture_from_surface(&text_surface)
            .unwrap();

        // Calculate the position of the text
        let text_width = text_surface.width();
        let text_height = text_surface.height();

        // Draw the text
        wc.canvas.copy(&text_texture, None, Rect::new(20, 20, text_width, text_height))
            .unwrap();
}

fn inputs(
    wc: &mut WindowContext,
    map: &mut world::Map,
    camera: &mut Camera,
    key_states: &mut [bool; 238],
) -> bool {
    //updates the array of all the keys that are currently held down
    for event in wc.event_pump.poll_iter() {
        match event {
            Event::Quit { .. } => return true,
            Event::KeyDown {
                keycode: Some(key), ..
            } => {
                // Key is pressed
                if let Some(index) = keycode_to_index(key) {
                    key_states[index] = true;
                }
            }
            Event::KeyUp {
                keycode: Some(key), ..
            } => {
                // Key is released
                if let Some(index) = keycode_to_index(key) {
                    key_states[index] = false;
                }
            }
            _ => {}
        }
    }

    //camera movement
    if key_states[66] { // W  up
        camera.y_offset += camera.movement_speed
    }
    if key_states[44] { // A  up
        camera.x_offset += camera.movement_speed
    }
    if key_states[62] { // S  up
        camera.y_offset -= camera.movement_speed
    }
    if key_states[47] { // D  up
        camera.x_offset -= camera.movement_speed
    }
    //camera zoom
    if key_states[48] {
        // E  zoom in

        let relative_zoom_speed = camera.zoom_speed * camera.zoom;
        camera.zoom += relative_zoom_speed;
        camera.x_offset += (relative_zoom_speed * map.size as f32) * (((camera.x_offset - (camera.window_width / 2.0)) / camera.zoom) / (map.size as f32));
        camera.y_offset += (relative_zoom_speed * map.size as f32) * (((camera.y_offset - (camera.window_height / 2.0)) / camera.zoom) / (map.size as f32));
    }
    if key_states[60] {
        // Q  zoom out

        let relative_zoom_speed = camera.zoom_speed * camera.zoom;
        camera.zoom -= relative_zoom_speed;
        camera.x_offset -= (relative_zoom_speed * map.size as f32) * (((camera.x_offset - (camera.window_width / 2.0)) / camera.zoom) / (map.size as f32));
        camera.y_offset -= (relative_zoom_speed * map.size as f32) * (((camera.y_offset - (camera.window_height / 2.0)) / camera.zoom) / (map.size as f32));
    }
    return false;
}

//don't open this
fn keycode_to_index(keycode: Keycode) -> Option<usize> {
    match keycode {
        Keycode::Backspace => Some(1),
        Keycode::Tab => Some(2),
        Keycode::Return => Some(3),
        Keycode::Escape => Some(4),
        Keycode::Space => Some(5),
        Keycode::Exclaim => Some(6),
        Keycode::Quotedbl => Some(7),
        Keycode::Hash => Some(8),
        Keycode::Dollar => Some(9),
        Keycode::Percent => Some(10),
        Keycode::Ampersand => Some(11),
        Keycode::Quote => Some(12),
        Keycode::LeftParen => Some(13),
        Keycode::RightParen => Some(14),
        Keycode::Asterisk => Some(15),
        Keycode::Plus => Some(16),
        Keycode::Comma => Some(17),
        Keycode::Minus => Some(18),
        Keycode::Period => Some(19),
        Keycode::Slash => Some(20),
        Keycode::Num0 => Some(21),
        Keycode::Num1 => Some(22),
        Keycode::Num2 => Some(23),
        Keycode::Num3 => Some(24),
        Keycode::Num4 => Some(25),
        Keycode::Num5 => Some(26),
        Keycode::Num6 => Some(27),
        Keycode::Num7 => Some(28),
        Keycode::Num8 => Some(29),
        Keycode::Num9 => Some(30),
        Keycode::Colon => Some(31),
        Keycode::Semicolon => Some(32),
        Keycode::Less => Some(33),
        Keycode::Equals => Some(34),
        Keycode::Greater => Some(35),
        Keycode::Question => Some(36),
        Keycode::At => Some(37),
        Keycode::LeftBracket => Some(38),
        Keycode::Backslash => Some(39),
        Keycode::RightBracket => Some(40),
        Keycode::Caret => Some(41),
        Keycode::Underscore => Some(42),
        Keycode::Backquote => Some(43),
        Keycode::A => Some(44),
        Keycode::B => Some(45),
        Keycode::C => Some(46),
        Keycode::D => Some(47),
        Keycode::E => Some(48),
        Keycode::F => Some(49),
        Keycode::G => Some(50),
        Keycode::H => Some(51),
        Keycode::I => Some(52),
        Keycode::J => Some(53),
        Keycode::K => Some(54),
        Keycode::L => Some(55),
        Keycode::M => Some(56),
        Keycode::N => Some(57),
        Keycode::O => Some(58),
        Keycode::P => Some(59),
        Keycode::Q => Some(60),
        Keycode::R => Some(61),
        Keycode::S => Some(62),
        Keycode::T => Some(63),
        Keycode::U => Some(64),
        Keycode::V => Some(65),
        Keycode::W => Some(66),
        Keycode::X => Some(67),
        Keycode::Y => Some(68),
        Keycode::Z => Some(69),
        Keycode::Delete => Some(70),
        Keycode::CapsLock => Some(71),
        Keycode::F1 => Some(72),
        Keycode::F2 => Some(73),
        Keycode::F3 => Some(74),
        Keycode::F4 => Some(75),
        Keycode::F5 => Some(76),
        Keycode::F6 => Some(77),
        Keycode::F7 => Some(78),
        Keycode::F8 => Some(79),
        Keycode::F9 => Some(80),
        Keycode::F10 => Some(81),
        Keycode::F11 => Some(82),
        Keycode::F12 => Some(83),
        Keycode::PrintScreen => Some(84),
        Keycode::ScrollLock => Some(85),
        Keycode::Pause => Some(86),
        Keycode::Insert => Some(87),
        Keycode::Home => Some(88),
        Keycode::PageUp => Some(89),
        Keycode::End => Some(90),
        Keycode::PageDown => Some(91),
        Keycode::Right => Some(92),
        Keycode::Left => Some(93),
        Keycode::Down => Some(94),
        Keycode::Up => Some(95),
        Keycode::NumLockClear => Some(96),
        Keycode::KpDivide => Some(97),
        Keycode::KpMultiply => Some(98),
        Keycode::KpMinus => Some(99),
        Keycode::KpPlus => Some(100),
        Keycode::KpEnter => Some(101),
        Keycode::Kp1 => Some(102),
        Keycode::Kp2 => Some(103),
        Keycode::Kp3 => Some(104),
        Keycode::Kp4 => Some(105),
        Keycode::Kp5 => Some(106),
        Keycode::Kp6 => Some(107),
        Keycode::Kp7 => Some(108),
        Keycode::Kp8 => Some(109),
        Keycode::Kp9 => Some(110),
        Keycode::Kp0 => Some(111),
        Keycode::KpPeriod => Some(112),
        Keycode::Application => Some(113),
        Keycode::Power => Some(114),
        Keycode::KpEquals => Some(115),
        Keycode::F13 => Some(116),
        Keycode::F14 => Some(117),
        Keycode::F15 => Some(118),
        Keycode::F16 => Some(119),
        Keycode::F17 => Some(120),
        Keycode::F18 => Some(121),
        Keycode::F19 => Some(122),
        Keycode::F20 => Some(123),
        Keycode::F21 => Some(124),
        Keycode::F22 => Some(125),
        Keycode::F23 => Some(126),
        Keycode::F24 => Some(127),
        Keycode::Execute => Some(128),
        Keycode::Help => Some(129),
        Keycode::Menu => Some(130),
        Keycode::Select => Some(131),
        Keycode::Stop => Some(132),
        Keycode::Again => Some(133),
        Keycode::Undo => Some(134),
        Keycode::Cut => Some(135),
        Keycode::Copy => Some(136),
        Keycode::Paste => Some(137),
        Keycode::Find => Some(138),
        Keycode::Mute => Some(139),
        Keycode::VolumeUp => Some(140),
        Keycode::VolumeDown => Some(141),
        Keycode::KpComma => Some(142),
        Keycode::KpEqualsAS400 => Some(143),
        Keycode::AltErase => Some(144),
        Keycode::Sysreq => Some(145),
        Keycode::Cancel => Some(146),
        Keycode::Clear => Some(147),
        Keycode::Prior => Some(148),
        Keycode::Return2 => Some(149),
        Keycode::Separator => Some(150),
        Keycode::Out => Some(151),
        Keycode::Oper => Some(152),
        Keycode::ClearAgain => Some(153),
        Keycode::CrSel => Some(154),
        Keycode::ExSel => Some(155),
        Keycode::Kp00 => Some(156),
        Keycode::Kp000 => Some(157),
        Keycode::ThousandsSeparator => Some(158),
        Keycode::DecimalSeparator => Some(159),
        Keycode::CurrencyUnit => Some(160),
        Keycode::CurrencySubUnit => Some(161),
        Keycode::KpLeftParen => Some(162),
        Keycode::KpRightParen => Some(163),
        Keycode::KpLeftBrace => Some(164),
        Keycode::KpRightBrace => Some(165),
        Keycode::KpTab => Some(166),
        Keycode::KpBackspace => Some(167),
        Keycode::KpA => Some(168),
        Keycode::KpB => Some(169),
        Keycode::KpC => Some(170),
        Keycode::KpD => Some(171),
        Keycode::KpE => Some(172),
        Keycode::KpF => Some(173),
        Keycode::KpXor => Some(174),
        Keycode::KpPower => Some(175),
        Keycode::KpPercent => Some(176),
        Keycode::KpLess => Some(177),
        Keycode::KpGreater => Some(178),
        Keycode::KpAmpersand => Some(179),
        Keycode::KpDblAmpersand => Some(180),
        Keycode::KpVerticalBar => Some(181),
        Keycode::KpDblVerticalBar => Some(182),
        Keycode::KpColon => Some(183),
        Keycode::KpHash => Some(184),
        Keycode::KpSpace => Some(185),
        Keycode::KpAt => Some(186),
        Keycode::KpExclam => Some(187),
        Keycode::KpMemStore => Some(188),
        Keycode::KpMemRecall => Some(189),
        Keycode::KpMemClear => Some(190),
        Keycode::KpMemAdd => Some(191),
        Keycode::KpMemSubtract => Some(192),
        Keycode::KpMemMultiply => Some(193),
        Keycode::KpMemDivide => Some(194),
        Keycode::KpPlusMinus => Some(195),
        Keycode::KpClear => Some(196),
        Keycode::KpClearEntry => Some(197),
        Keycode::KpBinary => Some(198),
        Keycode::KpOctal => Some(199),
        Keycode::KpDecimal => Some(200),
        Keycode::KpHexadecimal => Some(201),
        Keycode::LCtrl => Some(202),
        Keycode::LShift => Some(203),
        Keycode::LAlt => Some(204),
        Keycode::LGui => Some(205),
        Keycode::RCtrl => Some(206),
        Keycode::RShift => Some(207),
        Keycode::RAlt => Some(208),
        Keycode::RGui => Some(209),
        Keycode::Mode => Some(210),
        Keycode::AudioNext => Some(211),
        Keycode::AudioPrev => Some(212),
        Keycode::AudioStop => Some(213),
        Keycode::AudioPlay => Some(214),
        Keycode::AudioMute => Some(215),
        Keycode::MediaSelect => Some(216),
        Keycode::Www => Some(217),
        Keycode::Mail => Some(218),
        Keycode::Calculator => Some(219),
        Keycode::Computer => Some(220),
        Keycode::AcSearch => Some(221),
        Keycode::AcHome => Some(222),
        Keycode::AcBack => Some(223),
        Keycode::AcForward => Some(224),
        Keycode::AcStop => Some(225),
        Keycode::AcRefresh => Some(226),
        Keycode::AcBookmarks => Some(227),
        Keycode::BrightnessDown => Some(228),
        Keycode::BrightnessUp => Some(229),
        Keycode::DisplaySwitch => Some(230),
        Keycode::KbdIllumToggle => Some(231),
        Keycode::KbdIllumDown => Some(232),
        Keycode::KbdIllumUp => Some(233),
        Keycode::Eject => Some(234),
        Keycode::Sleep => Some(235),
        _ => None,
    }
}
