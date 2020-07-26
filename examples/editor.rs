use piston::*;
use graphics::*;
use opengl_graphics::*;
use sdl2_window::*;
use avatar_graph::*;

fn main() {
    println!("=== Avatar Graph Editor ===");
    println!("Space - Add new node");
    println!("S - Select start node");
    println!("C - Connect to closest node");

    let opengl = OpenGL::V3_2;
    let settings = WindowSettings::new("Avatar Graph Editor", [512; 2])
        .graphics_api(opengl)
        .samples(4)
        .exit_on_esc(true);
    let mut window: Sdl2Window = settings.build().unwrap();

    let mut gl = GlGraphics::new(opengl);
    let mut events = Events::new(EventSettings::new().lazy(true));

    let mut graph = Graph::new();
    let mut node_pos: Vec<[f64; 2]> = vec![];
    let mut cursor: [f64; 2] = [0.0; 2];
    let mut selected = 0;
    let mut hide = false;

    while let Some(e) = events.next(&mut window) {
        if let Some(args) = e.render_args() {
            gl.draw(args.viewport(), |c, g| {
                clear([1.0; 4], g);

                let border = ellipse::Ellipse::new_border([0.0, 0.0, 0.0, 1.0], 1.0);
                let radius = 10.0;

                for i in 0..graph.nodes.len() {
                    if let Some(j) = graph.nodes[i].uniq {
                        let a = node_pos[i];
                        let b = node_pos[j];
                        line::Line::new([0.0, 0.0, 0.0, 0.3], 2.0)
                            .draw_from_to(a, b, &c.draw_state, c.transform, g);
                    }
                }

                for &(a, b) in &graph.edges {
                    let a = node_pos[a];
                    let b = node_pos[b];
                    line::Line::new([0.0, 0.0, 0.0, 1.0], 2.0)
                        .draw_from_to(a, b, &c.draw_state, c.transform, g);
                }

                for i in 0..node_pos.len() {
                    let color = if graph.nodes[i].core {[0.0, 0.0, 0.0, 1.0]} else {[1.0; 4]};
                    ellipse::Ellipse::new(color).draw([
                            node_pos[i][0] - radius,
                            node_pos[i][1] - radius,
                            radius * 2.0,
                            radius * 2.0,
                        ], &c.draw_state, c.transform, g);
                    border.draw([
                            node_pos[i][0] - radius,
                            node_pos[i][1] - radius,
                            radius * 2.0,
                            radius * 2.0,
                        ], &c.draw_state, c.transform, g);
                }

                if !hide {
                    if node_pos.len() > 0 {
                        line::Line::new([0.0, 0.0, 1.0, 0.5], 5.0)
                        .draw_from_to(node_pos[selected], cursor, &c.draw_state, c.transform, g);
                    }
                }
            })
        }
        if let Some(pos) = e.mouse_cursor_args() {
            cursor = pos;
        }
        if let Some(button) = e.press_args() {
            if let Button::Keyboard(Key::Space) = button {
                // Add new node.
                node_pos.push(cursor);
                graph.add_node(Node::new(false));
                graph.corify();
            }
            if let Button::Keyboard(Key::S) = button {
                let min = min_pos(&node_pos, cursor);
                if min.is_some() {
                    selected = min.unwrap().0;
                }
            }
            if let Button::Keyboard(Key::C) = button {
                let min = min_pos(&node_pos, cursor);
                if min.is_some() {
                    let i = min.unwrap().0;
                    if i != selected {
                        graph.add_edge(selected, i);
                        graph.corify();
                    }
                }
            }
            if let Button::Keyboard(Key::H) = button {
                hide = !hide;
            }
            if let Button::Mouse(MouseButton::Left) = button {
                if node_pos.len() > 0 {
                    node_pos[selected] = cursor;
                }
            }
            // println!("{:?}", button);
        }
    }

    println!("{:?}", graph);
}

fn min_pos(node_pos: &[[f64; 2]], cursor: [f64; 2]) -> Option<(usize, f64)> {
    // Select new start node.
    let mut min: Option<(usize, f64)> = None;
    for i in 0..node_pos.len() {
        let dist = math::square_len(math::sub(node_pos[i], cursor));
        if min.is_none() || min.unwrap().1 > dist {
            min = Some((i, dist));
        }
    }
    min
}
