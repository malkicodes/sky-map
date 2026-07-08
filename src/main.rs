use std::ops::Neg;

use sfml::{
    graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, Shape, Text, Transformable},
    window::{
        ContextSettings, Event, Key, Style,
        mouse::{Button, Wheel},
    },
};
use sky_map::{
    SCREEN_SIZE,
    drawables::{constellation::load_constellations, grid::Grid},
    rad_to_dms, rad_to_hms,
    settings::{DisplaySettings, NameSetting},
    star::Star,
    view::View,
};

fn update_star_positions(stars: &[Star], star_circles: &mut [CircleShape], view: &View) {
    for (star, star_circle) in stars.iter().zip(star_circles) {
        star_circle.set_position(view.project_to_screen(star.spherical_coordinates()));

        star_circle.set_radius(star.graphical_size() * view.zoom_v());
        star_circle.set_origin(star_circle.radius());
    }
}

fn update_star_names(
    stars: &[Star],
    star_names: &mut [String],
    display_settings: &DisplaySettings,
) {
    for (star, name) in stars.iter().zip(star_names.iter_mut()) {
        *name = match display_settings.names() {
            NameSetting::Proper => star.star_name(),
            NameSetting::BayerFlamsteed => {
                star.bayerflamsteed_name().unwrap_or_else(|| star.hd_name())
            }
            NameSetting::HD => star.hd_name(),
            NameSetting::Hidden => break,
        }
    }
}

fn main() {
    let font =
        Font::from_memory_static(include_bytes!("../assets/RobotoMono-Regular.ttf")).unwrap();

    let mut window = RenderWindow::new(
        (SCREEN_SIZE, SCREEN_SIZE),
        "Sky Map",
        Style::CLOSE,
        &ContextSettings {
            antialiasing_level: 4,
            ..Default::default()
        },
    )
    .unwrap();

    window.set_framerate_limit(30);

    let mut view = View::default();
    let mut settings = DisplaySettings::default();

    let stars: Vec<Star> = {
        let mut s: Vec<Star> = serde_json::from_str(include_str!("../assets/stars.json")).unwrap();
        s.sort_by(|a, b| a.apparent_magnitude().total_cmp(&b.apparent_magnitude()));

        s
    };

    let mut constellations = load_constellations(&stars);

    let mut star_names = vec![String::new(); stars.len()];
    update_star_names(&stars, &mut star_names, &settings);

    let mut star_circles: Vec<CircleShape> = Vec::with_capacity(stars.len());
    for star in stars.iter() {
        let mut c = CircleShape::new(
            star.graphical_size(),
            if star.apparent_magnitude() < 2. {
                48
            } else {
                24
            },
        );
        c.set_origin(c.radius());

        c.set_fill_color(star.graphical_color());

        star_circles.push(c);
    }

    update_star_positions(&stars, &mut star_circles, &view);

    eprintln!("Loaded {} stars", stars.len());

    let mut text = Text::new("", &font, 12);
    let mut grid = Grid::new().unwrap();

    let mut mouse = (0, 0);
    let mut mouse_down = false;

    'mainloop: loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed
                | Event::KeyPressed {
                    code: Key::Q | Key::Escape,
                    ..
                } => break 'mainloop,
                Event::KeyPressed { code: Key::R, .. } => view = View::default(),
                Event::KeyPressed {
                    code: Key::Num1, ..
                } => view.set_zoom(0.),
                Event::KeyPressed { code: Key::N, .. } => {
                    settings.cycle_names();
                    update_star_names(&stars, &mut star_names, &settings);
                }
                Event::KeyPressed { code: Key::G, .. } => {
                    settings.cycle_grid();
                }
                Event::MouseMoved { x, y } => {
                    let (deltax, deltay) = (x - mouse.0, y - mouse.1);
                    mouse = (x, y);

                    if mouse_down {
                        const SENSITIVITY: f64 = 1. / 512.;

                        view.change_latlng((
                            -deltay as f64 * SENSITIVITY * view.zoom_v().recip() as f64,
                            -deltax as f64 * SENSITIVITY * view.zoom_v().recip() as f64,
                        ));
                    }
                }
                Event::MouseButtonPressed {
                    button: Button::Left,
                    x,
                    y,
                } => {
                    mouse = (x, y);
                    mouse_down = true;
                }
                Event::MouseButtonReleased {
                    button: Button::Left,
                    x,
                    y,
                } => {
                    mouse = (x, y);
                    mouse_down = false;
                }
                Event::MouseWheelScrolled {
                    wheel: Wheel::VerticalWheel,
                    delta,
                    ..
                } => {
                    const WHEEL_SENSITIVITY: f32 = 0.125;
                    const MIN_ZOOM: f32 = 0.;

                    view.set_zoom((view.zoom() + delta * WHEEL_SENSITIVITY).max(MIN_ZOOM));
                }
                _ => (),
            }
        }

        update_star_positions(&stars, &mut star_circles, &view);

        window.clear(Color::BLACK);

        grid.update(&view, &settings).unwrap();
        window.draw(&grid);

        for constellation in constellations.iter_mut() {
            constellation.update(&view, &settings).unwrap();
            window.draw(constellation);
        }

        for ((s, name), star) in star_circles
            .iter()
            .zip(star_names.iter())
            .zip(stars.iter())
            .filter(|((c, _), _)| {
                let pos = c.position();

                c.radius() > 0.5
                    && pos.x > -2. * c.radius()
                    && pos.x < SCREEN_SIZE as f32 + c.radius()
                    && pos.y > -2. * c.radius()
                    && pos.y < SCREEN_SIZE as f32 + c.radius()
            })
        {
            window.draw(s);

            if settings.names() == NameSetting::Hidden
                || star.apparent_magnitude() > (view.zoom() + 1.75).max(2.)
            {
                // skip name
                continue;
            }

            text.set_string(name);

            let mut position = s.position() + s.radius().into();
            let bounds = text.local_bounds();
            position.x -= bounds.width * 0.5;
            position.y += s.radius() + bounds.height * 0.5;
            text.set_position((position.x.round_ties_even(), position.y.round_ties_even()));

            window.draw(&text);
        }

        let (lat, lng) = view.latlng();

        let ra = rad_to_hms(lng.neg() as f32); // why neg?
        let dec = rad_to_dms(lat as f32);

        text.set_string(&format!("RA:  {:02}h{:02}m{:06.3}s", ra.0, ra.1, ra.2));
        text.set_position((
            0.,
            SCREEN_SIZE as f32 - (text.character_size() * 3) as f32 - 6.,
        ));
        window.draw(&text);

        text.set_string(&format!("DC: {:+03}°{:02}'{:06.3}\"", dec.0, dec.1, dec.2));
        text.set_position((
            0.,
            SCREEN_SIZE as f32 - (text.character_size() * 2) as f32 - 4.,
        ));
        window.draw(&text);

        text.set_string(&format!(
            "Zoom: {:.2}° ({})",
            view.zoom_v().recip() * 90.,
            view.zoom() * 8.
        ));
        text.set_position((0., SCREEN_SIZE as f32 - text.character_size() as f32 - 2.));
        window.draw(&text);

        window.display();
    }

    window.close();
}
