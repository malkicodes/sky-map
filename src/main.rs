use sfml::{
    graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, Shape, Text, Transformable},
    system::Vector2f,
    window::{
        ContextSettings, Event, Key, Style,
        mouse::{Button, Wheel},
    },
};
use sky_map::{DisplaySettings, NameSetting, View, star::Star, sterejec};

fn update_star_positions(stars: &[Star], star_circles: &mut [CircleShape], view: &View) {
    for (star, star_circle) in stars.iter().zip(star_circles) {
        let pos: Vector2f = sterejec(&star, &view).into();

        let zoom = (2_f32).powf(view.zoom());

        star_circle.set_position(pos * 500. * zoom + Vector2f::new(500., 500.));

        star_circle.set_radius(star.graphical_size() * zoom);
    }
}

fn update_star_names(stars: &[Star], star_names: &mut [String], settings: &DisplaySettings) {
    for (star, name) in stars.iter().zip(star_names.iter_mut()) {
        *name = match settings.names() {
            sky_map::NameSetting::Proper => star.star_name(),
            sky_map::NameSetting::BayerFlamsteed => star.bayerflamsteed_name(),
            sky_map::NameSetting::HR => star.hr_name(),
            sky_map::NameSetting::HD => star.hd_name(),
            sky_map::NameSetting::Hidden => break,
        }
    }
}

fn main() {
    let font =
        Font::from_memory_static(include_bytes!("../assets/Inter_18pt-Regular.ttf")).unwrap();

    let mut window = RenderWindow::new(
        (1000, 1000),
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
        let mut s: Vec<Star> = serde_json::from_str(include_str!("../assets/ybsc5.json")).unwrap();
        s.sort_by(|a, b| a.vmag.total_cmp(&b.vmag));

        s
    };

    let mut star_names = vec![String::new(); stars.len()];
    update_star_names(&stars, &mut star_names, &settings);

    let mut star_circles: Vec<CircleShape> = Vec::with_capacity(stars.len());
    for star in stars.iter() {
        let mut c = CircleShape::new(star.graphical_size(), 24);
        c.set_origin(c.radius());

        if let Some(color) = star.graphical_color() {
            c.set_fill_color(color);
        }

        star_circles.push(c);
    }

    update_star_positions(&stars, &mut star_circles, &view);

    eprintln!("Loaded {} stars", stars.len());

    let mut text = Text::new("", &font, 12);

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
                    println!("{:?}", settings.names());
                    update_star_names(&stars, &mut star_names, &settings);
                }
                Event::MouseMoved { x, y } => {
                    let (deltax, deltay) = (x - mouse.0, y - mouse.1);
                    mouse = (x, y);

                    if mouse_down {
                        const SENSITIVITY: f64 = 0.001953125;

                        view.change_latlng((
                            -deltay as f64 * SENSITIVITY * (0.5_f64).powf(view.zoom() as f64),
                            -deltax as f64 * SENSITIVITY * (0.5_f64).powf(view.zoom() as f64),
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
                    const MIN_ZOOM: f32 = -1.25;

                    view.set_zoom((view.zoom() + delta * WHEEL_SENSITIVITY).max(MIN_ZOOM));
                }
                _ => (),
            }
        }

        update_star_positions(&stars, &mut star_circles, &view);

        window.clear(Color::BLACK);

        for ((s, name), star) in star_circles
            .iter()
            .zip(star_names.iter())
            .zip(stars.iter())
            .filter(|((c, _), _)| {
                let pos = c.position();

                pos.x > -2. * c.radius()
                    && pos.x < 1000. + c.radius()
                    && pos.y > -2. * c.radius()
                    && pos.y < 1000. + c.radius()
            })
        {
            window.draw(s);

            if settings.names() == NameSetting::Hidden || star.vmag > (view.zoom() + 2.5).max(2.) {
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

        text.set_string(&format!(
            "Zoom: {:.2}x ({})",
            2_f32.powf(view.zoom()),
            view.zoom() * 8.
        ));
        text.set_position((0., 1000. - text.global_bounds().height * 2.));
        window.draw(&text);

        window.display();
    }

    window.close();
}
