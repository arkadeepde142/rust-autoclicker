mod autoclicker;
use autoclicker::Autoclicker;
use std::rc::Rc;
use druid::widget::{Button, Controller, Flex, Label, TextBox};
use druid::{
    AppLauncher, Env, Event, EventCtx, PlatformError, Widget, WidgetExt, WindowDesc, KeyCode
};

#[derive(druid::Data, Clone, std::fmt::Debug, druid::Lens)]
struct Model {
    val: u32,
    title: String,
    clicker: Rc<Autoclicker>,
}

fn main() -> Result<(), PlatformError> {
    let main_window = WindowDesc::new(ui_builder);
    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(Model{val: 0, title: String::from("Hello"), clicker: Rc::new(Autoclicker::new(std::time::Duration::from_millis(300)))})
}

struct KeyControl;

impl<T, W: Widget<T>> Controller<T, W> for KeyControl {
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut T, env: &Env) {
        // println!("{:?}", event);
        if let Event::KeyDown(_) = event {
            println!("{:?}", event);
        }
        match event {
            Event::KeyDown(k) if k.key_code == KeyCode::Return => {
                println!("No")
            }
            _ => {}
        }
        child.event(ctx, event, data, env)
        
    }
}

fn ui_builder() -> impl Widget<Model> {
    // The label text will be computed dynamically based on the current locale and count
    let text = |data: &Model, _: &_| format!("{:?}", data);
    // LocalizedString::new("hello-counter").with_arg("count", |data: &u32, _env| (*data).into());
    let label = Label::new(text).controller(KeyControl {}).padding(5.0).center();
    let button = Button::new("Increment")
        .on_click(|_ctx, data: &mut Model, _env| (*data).val += 1)
        .padding(5.0);

    let textbox = TextBox::new().lens(Model::title).padding(5.0).controller(KeyControl{});

    Flex::column().with_child(label).with_child(button).with_child(textbox)
}

// fn main() {
//     let mut autoclicker = Autoclicker::new(Duration::from_millis(100));
//     autoclicker.start().unwrap();
//     thread::sleep(Duration::from_millis(500));
//     autoclicker.stop().unwrap();
// }
