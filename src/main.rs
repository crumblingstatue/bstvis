use std::fmt::Display;

use bst::BinarySearchTree;
use rand::{thread_rng, Rng};
use sfml::{
    graphics::{
        CircleShape, Color, Font, Rect, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Shader, Text, Transformable, View,
    },
    system::Vector2,
    window::{ContextSettings, Event, Key, Style},
};

pub mod bst;

struct Vis<'a, T> {
    nodes: Vec<VisNode<&'a T>>,
}

struct VisNode<T> {
    value: T,
    x_offset: i32,
    y_offset: i32,
}

#[derive(Default)]
struct BuildVisCtx {
    depth: i32,
    x_offset: i32,
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
    ctx: BuildVisCtx,
) {
    vec.push(VisNode {
        value: &node.key,
        x_offset: ctx.x_offset,
        y_offset: ctx.depth,
    });
    if let Some(node) = &node.left {
        let ctx = BuildVisCtx {
            depth: ctx.depth + 1,
            x_offset: ctx.x_offset - 1,
        };
        build_vis_visit_node(node, vec, ctx);
    }
    if let Some(node) = &node.right {
        let ctx = BuildVisCtx {
            depth: ctx.depth + 1,
            x_offset: ctx.x_offset + 1,
        };
        build_vis_visit_node(node, vec, ctx);
    }
}

fn main() {
    let mut wnd = RenderWindow::new(
        (800, 600),
        "BST Visualization",
        Style::default(),
        &ContextSettings::default(),
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
    let Vector2 { x: wx, y: wy } = wnd.size();
    let mut view = View::new((0., 0.).into(), (wx as f32, wy as f32).into());

    let bg_shader = Shader::from_memory(None, None, Some(include_str!("frag.glsl"))).unwrap();
    let mut rs = RenderStates::default();
    rs.set_shader(Some(&bg_shader));

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
        let shape = RectangleShape::from_rect(Rect::new(0., 0., 800., 600.));
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
    let node_v_offset = node_size + 4.0;
    text.set_fill_color(Color::BLACK);
    let mut cshape = CircleShape::default();
    cshape.set_radius(node_radius);
    cshape.set_origin((node_radius, node_radius));
    text.set_origin((16., 16.));
    for node in &vis.nodes {
        let x = node.x_offset as f32 * node_size;
        let y = node.y_offset as f32 * node_v_offset;
        cshape.set_position((x, y));
        wnd.draw(&cshape);
        text.set_position((x, y));
        text.set_string(&node.value.to_string());
        wnd.draw(&text);
    }
}
