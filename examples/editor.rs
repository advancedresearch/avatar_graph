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
    println!("H - Hide selected start node");
    println!("P - Proof mode (shows why node is not a core)");
    println!("A - Show avatar distance");
    println!("");
    println!("Proof mode colors:");
    println!("Red - Contractible");
    println!("Blue - Unreachable");
    println!("Green - Non-unique max avatar");
    println!("Torquoise - Not universal reachable (along)");
    println!("Orange - Avatar connectivity failures");
    println!("");

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
    // Show why selected node is not a core.
    let mut proof_mode = false;
    let mut avatar_distance = false;

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

                if proof_mode && node_pos.len() > 0 {
                    if !graph.nodes[selected].core {
                        let mut nodes = vec![];
                        let mut color = [1.0; 4];
                        let contractibles = graph.contractibles_of(selected);
                        let mut max_avatar: Option<usize> = None;
                        if contractibles.len() > 0 {
                            // Show nodes that are contractible.
                            nodes = contractibles;
                            color = [1.0, 0.0, 0.0, 1.0];
                        }
                        if nodes.len() == 0 {
                            let dist = graph.distance(selected);
                            match dist {
                                Err(dist) => {
                                    // Show nodes that are unreachable from selected node.
                                    let mut res = vec![];
                                    for i in 0..graph.nodes.len() {
                                        if dist.binary_search_by(|n| n.0.cmp(&i)).is_err() {
                                            res.push(i);
                                        }
                                    }
                                    nodes = res;
                                    color = [0.0, 0.0, 1.0, 1.0];
                                }
                                Ok(_) => {}
                            }
                        }
                        if nodes.len() == 0 {
                            let max_avatars = graph.max_avatars(selected);
                            if max_avatars.1.len() > 1 {
                                // Show max avatars.
                                nodes = max_avatars.1;
                                color = [0.0, 1.0, 0.0, 1.0];
                            }
                        }
                        if nodes.len() == 0 {
                            let max_avatars = graph.max_avatars(selected);
                            max_avatar = Some(max_avatars.1[0]);
                            if let Ok(along) = graph.along(max_avatars.1[0], selected) {
                                // Show nodes that are not reachable along the path.
                                let mut res = vec![];
                                for i in 0..graph.nodes.len() {
                                    if along.binary_search_by(|n| n.cmp(&i)).is_err() {
                                        res.push(i);
                                    }
                                }
                                nodes = res;
                                color = [0.0, 0.7, 1.0, 1.0];
                            }
                        }
                        if nodes.len() == 0 {
                            // Show avatar connectivity failures.
                            nodes = graph.avatar_connectivity_failures_of(selected);
                            color = [1.0, 0.7, 0.0, 1.0];
                        }

                        for &i in &nodes {
                            ellipse::Ellipse::new_border(color, 2.0).draw([
                                    node_pos[i][0] - radius,
                                    node_pos[i][1] - radius,
                                    radius * 2.0,
                                    radius * 2.0,
                                ], &c.draw_state, c.transform, g);
                        }
                        if let Some(i) = max_avatar {
                            ellipse::Ellipse::new_border([0.5, 0.5, 0.5, 1.0], 3.0).draw([
                                    node_pos[i][0] - radius,
                                    node_pos[i][1] - radius,
                                    radius * 2.0,
                                    radius * 2.0,
                                ], &c.draw_state, c.transform, g);
                        }
                    }
                }

                if avatar_distance && node_pos.len() > 0 {
                    let dist = graph.avatar_distance(selected);
                    let max = dist.iter().map(|n| n.1).max().unwrap();
                    let radius = radius + 6.0;
                    let herm = |f: f32| 3.0 * f.powi(2) - 2.0 * f.powi(3);
                    for &(i, v) in &dist {
                        let f = v as f32 / max as f32;
                        ellipse::Ellipse::new_border([1.0 - herm(f), herm(f), 0.5, 1.0], 3.0 - f as f64)
                            .draw([
                                node_pos[i][0] - radius,
                                node_pos[i][1] - radius,
                                radius * 2.0,
                                radius * 2.0,
                            ], &c.draw_state, c.transform, g);
                    }
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
                    println!("Selected {}", selected);
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
            if let Button::Keyboard(Key::P) = button {
                proof_mode = !proof_mode;
            }
            if let Button::Keyboard(Key::A) = button {
                avatar_distance = !avatar_distance;
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
