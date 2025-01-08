use bevy::{color::palettes::css::*, prelude::*, window::PresentMode};
use bevy_prototype_lyon::prelude::*;

mod jarvis_march_algorithm;

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn convex_hull(mut commands: Commands) {
    let points = jarvis_march_algorithm::random_points(25, WIDTH as isize, HEIGHT as isize, 50);
    let hull = jarvis_march_algorithm::find_convex_hull(&points).unwrap();

    for p in points.iter() {
        let circle = shapes::Circle {
            radius: 5.0,
            center: (p.x as f32, p.y as f32).into(),
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&circle),
                ..default()
            },
            Fill::color(WHITE),
        ));
    }

    for w in hull.windows(2) {
        let p1 = points[w[0]];
        let p2 = points[w[1]];

        let line = shapes::Line(
            (p1.x as f32, p1.y as f32).into(),
            (p2.x as f32, p2.y as f32).into(),
        );
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&line),
                ..default()
            },
            Stroke::new(RED, 2.5),
        ));
    }
}

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Gift Wrapping Algorithm".into(),
                    resizable: false,
                    resolution: (WIDTH, HEIGHT).into(),
                    present_mode: PresentMode::AutoVsync,
                    enabled_buttons: bevy::window::EnabledButtons {
                        maximize: false,
                        ..Default::default()
                    },
                    ..default()
                }),
                ..default()
            }),
            ShapePlugin,
        ))
        .add_systems(Startup, (setup_camera, convex_hull))
        .run();
}
