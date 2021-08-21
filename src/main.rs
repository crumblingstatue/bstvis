use std::fmt::Display;

use bst::BinarySearchTree;
use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        CircleShape, Color, Font, PrimitiveType, Rect, RectangleShape, RenderStates, RenderTarget,
        RenderWindow, Shader, Shape, Text, Transformable, Vertex, View,
    },
    system::Vector2,
    window::{ContextSettings, Event, Key, Style, VideoMode},
};

pub mod bst;

struct Vis<'a, T> {
    nodes: Vec<VisChildNode<&'a T>>,
    depth: usize,
}

struct VisChildNode<T> {
    value: T,
    x_offset: f32,
    y_offset: u32,
    parent_x_offset: f32,
}

struct BuildVisCtx {
    x_offset: f32,
    y_offset: u32,
    parent_x_offset: f32,
}

fn build_vis<T>(tree: &BinarySearchTree<T>) -> Vis<T> {
    let mut vec = Vec::new();
    match &tree.root {
        Some(root_node) => {
            if let Some(left) = &root_node.left {
                build_vis_visit_node(
                    left,
                    &mut vec,
                    BuildVisCtx {
                        parent_x_offset: 0.0,
                        x_offset: -1.0,
                        y_offset: 1,
                    },
                );
            }
            if let Some(right) = &root_node.right {
                build_vis_visit_node(
                    right,
                    &mut vec,
                    BuildVisCtx {
                        parent_x_offset: 0.0,
                        x_offset: 1.0,
                        y_offset: 1,
                    },
                );
            }
        }
        None => {
            return Vis {
                nodes: Vec::default(),
                depth: 0,
            }
        }
    }
    Vis {
        nodes: vec,
        depth: tree.depth(),
    }
}

fn build_vis_visit_node<'a, T>(
    node: &'a bst::Node<T>,
    vec: &mut Vec<VisChildNode<&'a T>>,
    ctx: BuildVisCtx,
) {
    vec.push(VisChildNode {
        value: &node.key,
        x_offset: ctx.x_offset,
        y_offset: ctx.y_offset,
        parent_x_offset: ctx.parent_x_offset,
    });
    let x_offset_factor = 2.0 / 2.0f32.powi((ctx.y_offset + 1) as i32);
    if let Some(node) = &node.left {
        let ctx = BuildVisCtx {
            parent_x_offset: ctx.x_offset,
            x_offset: ctx.x_offset - x_offset_factor,
            y_offset: ctx.y_offset + 1,
        };
        build_vis_visit_node(node, vec, ctx);
    }
    if let Some(node) = &node.right {
        let ctx = BuildVisCtx {
            parent_x_offset: ctx.x_offset,
            x_offset: ctx.x_offset + x_offset_factor,
            y_offset: ctx.y_offset + 1,
        };
        build_vis_visit_node(node, vec, ctx);
    }
}

fn main() {
    let mut ctx_settings = ContextSettings::default();
    ctx_settings.set_antialiasing_level(8);
    let mut wnd = RenderWindow::new(
        VideoMode::desktop_mode(),
        "BST Visualization",
        Style::NONE,
        &ctx_settings,
    );
    wnd.set_position((0, 0).into());
    wnd.set_vertical_sync_enabled(true);
    let font = Font::from_file("DejaVuSans.ttf").unwrap();
    let mut tree = BinarySearchTree::default();
    // Try to generate a random tree of manageable depth
    let mut rng = thread_rng();
    loop {
        tree.insert(50);
        for _ in 0..50 {
            tree.insert(rng.gen_range(0..100));
        }
        if tree.depth() > 7 {
            tree = BinarySearchTree::default();
        } else {
            break;
        }
    }
    let vis = build_vis(&tree);
    let Vector2 { x: ww, y: wh } = wnd.size();
    let mut view = View::new((0., 0.).into(), (ww as f32, wh as f32).into());

    let mut bg_shader = Shader::from_memory(None, None, Some(include_str!("frag.glsl"))).unwrap();
    bg_shader.set_uniform_vec2("res", (ww as f32, wh as f32).into());

    while wnd.is_open() {
        while let Some(ev) = wnd.poll_event() {
            if ev == Event::Closed {
                wnd.close();
            }
        }
        let speed = 10.;
        if Key::LEFT.is_pressed() {
            view.move_((-speed, 0.));
        }
        if Key::RIGHT.is_pressed() {
            view.move_((speed, 0.));
        }
        if Key::UP.is_pressed() {
            view.move_((0., -speed));
        }
        if Key::DOWN.is_pressed() {
            view.move_((0., speed));
        }
        let def_view = wnd.default_view().to_owned();
        wnd.set_view(&def_view);
        bg_shader.set_uniform_vec2("view_cen", view.center());
        let mut rs = RenderStates::default();
        let shape = RectangleShape::from_rect(Rect::new(0., 0., ww as f32, wh as f32));
        rs.set_shader(Some(&bg_shader));
        wnd.draw_rectangle_shape(&shape, &rs);
        wnd.set_view(&view);
        draw_vis(&mut wnd, &font, tree.root().unwrap(), &vis);
        wnd.display();
    }
}

fn draw_vis<T: Display>(wnd: &mut RenderWindow, font: &Font, root_value: &T, vis: &Vis<T>) {
    let node_radius = 32;
    let mut text = Text::new("", font, node_radius - 14);
    let node_radius = node_radius as f32;
    let node_size = node_radius * 2.0;
    let node_v_offset = node_size * 1.5;
    text.set_fill_color(Color::BLACK);
    let mut cshape = CircleShape::default();
    cshape.set_radius(node_radius);
    cshape.set_origin((node_radius, node_radius));
    cshape.set_outline_color(Color::BLACK);
    cshape.set_outline_thickness(-2.0);
    cshape.set_fill_color(Color::rgb(220, 200, 70));
    text.set_origin((16., 16.));
    // Draw root node
    wnd.draw(&cshape);
    text.set_string(&root_value.to_string());
    wnd.draw(&text);
    let vis_depth_scaling_factor = 2u32.pow(vis.depth as u32 - 1) as f32;
    // Draw all children
    for node in &vis.nodes {
        let x = (node.x_offset * vis_depth_scaling_factor) * (node_radius / 2.0);
        let y = node.y_offset as f32 * node_v_offset;
        cshape.set_position((x, y));
        wnd.draw(&cshape);
        let line = [
            Vertex::with_pos_color(
                (
                    node.parent_x_offset * vis_depth_scaling_factor * (node_radius / 2.0),
                    (node.y_offset - 1) as f32 * node_v_offset,
                )
                    .into(),
                Color::WHITE,
            ),
            Vertex::with_pos_color((x, y).into(), Color::BLACK),
        ];
        wnd.draw_primitives(&line, PrimitiveType::LINES, &RenderStates::default());
        text.set_position((x, y));
        text.set_string(&node.value.to_string());
        wnd.draw(&text);
    }
}
