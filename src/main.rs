use sfml::{
    graphics::{CircleShape, Color, Font, RenderTarget, RenderWindow, Text, Transformable},
    system::Vector2f,
    window::{ContextSettings, Event, Key, Style, mouse::Button},
};
use sky_map::{View, star::Star, sterejec};

fn update_star_positions(stars: &[Star], star_circles: &mut [CircleShape], view: &View) {
    for (star, star_circle) in stars.iter().zip(star_circles) {
        let pos: Vector2f = sterejec(&star, &view).into();

        star_circle.set_position(pos * 500. + Vector2f::new(500., 500.));
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

    let stars: Vec<Star> = {
        let mut s: Vec<Star> = serde_json::from_str(include_str!("../assets/ybsc5.json")).unwrap();
        s.sort_by(|a, b| a.vmag.total_cmp(&b.vmag));

        s
    };
    let star_names = stars.iter().map(|s| s.star_name()).collect::<Vec<_>>();

    let mut view = View::default();

    let mut star_circles: Vec<CircleShape> = Vec::with_capacity(stars.len());
    for star in stars.iter() {
        let mut c = CircleShape::new(star.graphical_size(), 24);
        c.set_origin(c.radius());

        star_circles.push(c);
    }

    update_star_positions(&stars, &mut star_circles, &view);

    eprintln!("Loaded {} stars", stars.len());

    let mut text = Text::new("", &font, 10);

    let mut mouse = (0, 0);
    let mut mouse_down = false;

    'mainloop: loop {
        while let Some(ev) = window.poll_event() {
            match ev {
                Event::Closed | Event::KeyPressed { code: Key::Q, .. } => break 'mainloop,
                Event::KeyPressed { code: Key::R, .. } => view = View::default(),
                Event::MouseMoved { x, y } => {
                    let (deltax, deltay) = (x - mouse.0, y - mouse.1);
                    mouse = (x, y);

                    if mouse_down {
                        const SENSITIVITY: f64 = 0.001953125;

                        view.change_latlng((
                            -deltay as f64 * SENSITIVITY,
                            -deltax as f64 * SENSITIVITY,
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

                pos.x > 0. && pos.x < 1000. && pos.y > 0. && pos.y < 1000.
            })
        {
            window.draw(s);

            if star.vmag <= 2.5 {
                text.set_string(name);

                let mut position = s.position();
                position.x -= text.global_bounds().width * 0.5;
                position.y += 10.;
                text.set_position((position.x.round_ties_even(), position.y.round_ties_even()));

                window.draw(&text);
            }
        }

        window.display();
    }

    window.close();
}
