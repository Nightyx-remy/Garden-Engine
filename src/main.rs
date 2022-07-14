use garden::{
    app::{App, AppConfiguration, IState, UpdateCap},
    assets::{Axis, Color, EffectMode},
    logger::LogLevel,
    ui::{
        components::BackgroundComponent, get_scene, get_system, set_scene, ConstraintHeight,
        ConstraintWidth, ConstraintX, ConstraintY, ElementRef, IUIElement,
    },
    window::input::Key,
};

struct MainState {}

impl MainState {
    pub fn new() -> MainState {
        return MainState {};
    }
}

impl IState for MainState {
    fn init(&mut self) {
        let mut entity = get_system().get_entity_mut("A".to_string());
        let mut constraints = entity.get_constraints();
        constraints.constraint_x = ConstraintX::pixel_centered(0.0, ElementRef::Screen);
        constraints.constraint_y = ConstraintY::pixel_in_top(20.0, ElementRef::Screen);
        constraints.constraint_width = ConstraintWidth::percent(0.1, (Axis::X, ElementRef::Screen));
        constraints.constraint_height = ConstraintHeight::percent(1.0, (Axis::X, ElementRef::This));
        let mut scene = entity.get_scene();
        scene.0 = Some("Bye".to_string());
        entity.add_ui_component({
            let mut component = BackgroundComponent::new(Color::blue());
            component.color.set_animation(Color::blue(), Color::red(), 2.0, EffectMode::EaseOut, true).start();
            component
        });

        let mut entity2 = get_system().get_entity_mut("B".to_string());
        let mut constraints2 = entity2.get_constraints();
        constraints2.constraint_x = ConstraintX::pixel_from_right(20.0, ElementRef::Other("A".to_string()));
        constraints2.constraint_y = ConstraintY::pixel_in_top(0.0, ElementRef::Other("A".to_string()));
        constraints2.constraint_width = ConstraintWidth::percent(0.5, (Axis::X, ElementRef::Other("A".to_string())));
        constraints2.constraint_height = ConstraintHeight::percent(1.0, (Axis::X, ElementRef::This));
        let mut scene2 = entity2.get_scene();
        scene2.0 = Some("Hello".to_string());
        entity2.add_ui_component(BackgroundComponent::new(Color::red()));

        set_scene(Some("Hello".to_string()));
    }

    fn open(&mut self) {
        println!("Open");
    }

    fn update(&mut self, delta: f64) {
        if App::get().window().input().keyboard().get_key(Key::A).is_pressed() {
            println!("A pressed: {}", 1.0 / delta);
        }

        if App::get().window().input().keyboard().get_key(Key::Q).is_pressed() {
            if get_scene() == Some("Bye".to_string()) {
                set_scene(Some("Hello".to_string()));
            } else {
                set_scene(Some("Bye".to_string()));
            }
        }
    }

    fn close(&mut self) {
        println!("Close");
    }

    fn dispose(&mut self) {
        println!("Dispose");
    }
}

fn main() {
    let mut config = AppConfiguration::default();
    config.log_level = LogLevel::Debug;
    config.update_cap = UpdateCap::Unlimited;

    App::get().state_manager().register(MainState::new());
    App::get().state_manager().open::<MainState>();

    App::get().setup(config).start();
}
