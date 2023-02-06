use bevy::prelude::*;
use bevy_plot::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin {
                window: WindowDescriptor {
                    width: 800.,
                    height: 600.,
                    ..Default::default()
                },
                ..default()
            })
        )
        .add_plugin(PlotPlugin)
        .add_startup_system(setup)
        .add_system(change_bezier_metaparameters_at_runtime)
        .run();
}

use bevy_plot::UpdateBezierShaderEvent;

// Press Mouse::Right and drag the mouse to change the thickness of the curve
pub fn change_bezier_metaparameters_at_runtime(
    mut plots: ResMut<Assets<Plot>>,
    query: Query<(Entity, &Handle<Plot>, &BezierCurveNumber)>,
    mut cursor_moved_events: EventReader<CursorMoved>,
    mouse_button_input: Res<Input<MouseButton>>,
    mut event: EventWriter<UpdateBezierShaderEvent>,
) {
    for mouse_motion_event in cursor_moved_events.iter() {
        for (entity, plot_handle, curve_number) in query.iter() {
            let plot = plots.get_mut(plot_handle).unwrap();

            if mouse_button_input.pressed(MouseButton::Right) {
                if let Some(mut bezier_data) = plot.data.bezier_groups.get_mut(curve_number.0) {
                    bezier_data.size = mouse_motion_event.position.x / 100.0;

                    // If show_animation is set to true, UpdateBezierShaderEvent will be sent elsewhere anyway,
                    // so we don't need to send it twice every frame.
                    if !bezier_data.show_animation {
                        event.send(UpdateBezierShaderEvent {
                            plot_handle: plot_handle.clone(),
                            entity,
                            group_number: curve_number.0,
                        });
                    }

                    // For updating a scatter plot (markers) or a regular plot (segments), send
                    // the RespawnAllEvent event. This will despawn all the entities and respawn
                    // them with the updated information.
                }
            }
        }
    }
}

fn setup(
    mut commands: Commands,
    colors_res: Res<ColorMap>,
    mut plots: ResMut<Assets<Plot>>,
    asset_server: Res<AssetServer>,
    mut maybe_font: ResMut<TickLabelFont>,
) {
    commands.spawn(Camera2dBundle::default());
    let font: Handle<Font> = asset_server.load("fonts/Roboto-Bold.ttf");
    maybe_font.maybe_font = Some(font);

    let colors = colors_res.as_ref();

    let mut plot = Plot::default();

    // sine wave
    plot.plotopt_func(
        f3,
        vec![
            Opt::Size(2.0),
            Opt::Color(colors.get(&PlotColor::LightPink).unwrap()[1]),
        ],
    );

    let plot_handle = plots.add(plot.clone());
    commands.spawn(plot_handle);
}

pub fn f3(x: f32, t: f32) -> f32 {
    let freq = 20.0;
    let y = (x * freq + t * 0.0).sin() / 2.0 + 0.5;
    return y;
}
