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
    nodes: Vec<VisNode<&'a T>>,
}

struct VisNode<T> {
    value: T,
    off_x: i32,
    off_y: i32,
    parent_off: Option<(i32, i32)>,
}

struct BuildVisCtx {
    depth: i32,
    x_offset: i32,
    parent_off: Option<(i32, i32)>,
    dir_from_parent: Dir,
}

impl Default for BuildVisCtx {
    fn default() -> Self {
        Self {
            depth: 0,
            x_offset: 0,
            parent_off: None,
            dir_from_parent: Dir::Left,
        }
    }
}

enum Dir {
    Left,
    Right,
}

fn build_vis<T>(tree: &BinarySearchTree<T>) -> Vis<T> {
    let mut vec = Vec::new();
    match &tree.root {
        Some(root_node) => {
            build_vis_visit_node(root_node, &mut vec, BuildVisCtx::default());
        }
        None => {
            return Vis {
                nodes: Vec::default(),
            }
        }
    }
    Vis { nodes: vec }
}

fn build_vis_visit_node<'a, T>(
    node: &'a bst::Node<T>,
    vec: &mut Vec<VisNode<&'a T>>,
    mut ctx: BuildVisCtx,
) {
    vec.push(VisNode {
        value: &node.key,
        off_x: ctx.x_offset,
        off_y: ctx.depth,
        parent_off: ctx.parent_off,
    });
    if let Some(node) = &node.left {
        let ctx = BuildVisCtx {
            depth: ctx.depth + 1,
            x_offset: ctx.x_offset - 1,
            parent_off: Some((ctx.x_offset, ctx.depth)),
            dir_from_parent: Dir::Left,
        };
        build_vis_visit_node(node, vec, ctx);
    }
    if let Some(node) = &node.right {
        let ctx = BuildVisCtx {
            depth: ctx.depth + 1,
            x_offset: ctx.x_offset + 1,
            parent_off: Some((ctx.x_offset, ctx.depth)),
            dir_from_parent: Dir::Right,
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
        Style::default(),
        &ctx_settings,
    );
    wnd.set_vertical_sync_enabled(true);
    // Use whatever font you wanna use
    let font = Font::from_file("DejaVuSans.ttf").unwrap();
    let mut tree = BinarySearchTree::default();
    let mut rng = thread_rng();
    for _ in 0..100 {
        tree.insert(rng.gen_range(0..1000));
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
        draw_vis(&mut wnd, &font, &vis);
        wnd.display();
    }
}

fn draw_vis<T: Display>(wnd: &mut RenderWindow, font: &Font, vis: &Vis<T>) {
    let node_radius = 32;
    let mut text = Text::new("", font, node_radius - 8);
    let node_radius = node_radius as f32;
    let node_size = node_radius * 2.0;
    let node_v_offset = node_size * 1.5;
    text.set_fill_color(Color::BLACK);
    let mut cshape = CircleShape::default();
    cshape.set_radius(node_radius);
    cshape.set_origin((node_radius, node_radius));
    cshape.set_outline_color(Color::BLACK);
    cshape.set_outline_thickness(-2.0);
    text.set_origin((16., 16.));
    for node in &vis.nodes {
        let x = node.off_x as f32 * node_size;
        let y = node.off_y as f32 * node_v_offset;
        cshape.set_position((x, y));
        cshape.set_fill_color(Color::rgb(220, 200, 70));
        wnd.draw(&cshape);
        if let Some((x_off, y_off)) = node.parent_off {
            let line = [
                Vertex::with_pos_color(
                    (x_off as f32 * node_size, y_off as f32 * node_v_offset).into(),
                    Color::WHITE,
                ),
                Vertex::with_pos_color((x, y).into(), Color::BLACK),
            ];
            wnd.draw_primitives(&line, PrimitiveType::LINES, &RenderStates::default());
        }
        text.set_position((x, y));
        text.set_string(&node.value.to_string());
        wnd.draw(&text);
    }
}
