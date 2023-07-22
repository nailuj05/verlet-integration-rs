use raylib::prelude::*;

struct Point {
    position: Vector2,
    is_fixed: bool,
}

struct Stick {
    pointA: Point,
    pointB: Point,
}

struct Simulation {
    points: Vec<Point>,
    sticks: Vec<Stick>,
}

const RADIUS: f32 = 5.0;

fn main() {
    let mut builder = raylib::init();
    builder.msaa_4x();
    builder.resizable();

    let (mut rl, thread) = builder.size(640, 480).title("Verlet Integration").build();

    rl.set_target_fps(60);

    let mut sim = Simulation {
        points: vec![],
        sticks: vec![],
    };

    while !rl.window_should_close() {
        update(&rl, &mut sim);
        render(&mut rl, &thread, &sim);
    }
}

fn update(rl: &RaylibHandle, sim: &mut Simulation) {
    let ballposition = rl.get_mouse_position();

    if rl.is_mouse_button_down(MouseButton::MOUSE_LEFT_BUTTON) {
        match get_closest_point(sim, &ballposition) {
            Some((closest_index, closest_distance)) => {
                // did we click on an already existing point?
                if closest_distance <= RADIUS * 2.0 {
                    //Clicked on point
                } else {
                    //Clicked on empty space
                }
            }
            None => return,
        }

        let new_point = Point {
            position: ballposition,
            is_fixed: false,
        };
        sim.points.push(new_point)
    }
}

fn render(rl: &mut RaylibHandle, thread: &RaylibThread, sim: &Simulation) {
    let mut d = rl.begin_drawing(thread);

    d.clear_background(Color::from_hex("410952").unwrap());
    d.draw_fps(10, 10);

    for point in sim.points.iter() {
        d.draw_circle_v(point.position, RADIUS, Color::WHITE);
    }
}

fn get_closest_point(sim: &Simulation, position: &Vector2) -> Option<(usize, f32)> {
    let res: Option<(usize, f32)> = sim
        .points
        .iter()
        .map(|p| position.distance_to(p.position))
        .enumerate()
        .min_by(|a, b| a.partial_cmp(b).unwrap());

    res
}
