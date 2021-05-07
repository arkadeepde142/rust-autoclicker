mod singletonbuilder;
use autoclicker::Autoclicker;
use druid::text::format::{Formatter, Validation, ValidationError};
use druid::text::selection::Selection;
use druid::widget::{Button, CrossAxisAlignment, Flex, MainAxisAlignment, TextBox};
use druid::{
    AppDelegate, AppLauncher, Data, Lens, LocalizedString, PlatformError, Widget, WidgetExt,
    WindowDesc, DelegateCtx, WindowId, Env
};
use singletonbuilder::singleton;

// const VERTICAL_WIDGET_SPACING: f64 = 20.0;
const TEXT_BOX_WIDTH: f64 = 200.0;
const WINDOW_TITLE: LocalizedString<Model> = LocalizedString::new("Autoclicker");
#[derive(Clone, Data, Lens)]
struct Model {
    delay: u64,
}

struct DelayFormatter;

impl Formatter<u64> for DelayFormatter {
    fn format(&self, value: &u64) -> String {
        value.to_string()
    }

    fn validate_partial_input(&self, input: &str, _sel: &Selection) -> Validation {
        match input.parse::<u64>() {
            Ok(_) => Validation::success(),
            Err(err) => {
                if input == "" {
                    return Validation::success();
                }
                Validation::failure(err)
            }
        }
    }

    fn value(&self, input: &str) -> Result<u64, ValidationError> {
        match input.parse::<u64>() {
            Ok(value) => {
                println!("{}", value);
                return Ok(value);
            }
            Err(err) => {
                return Err(ValidationError::new(err));
            }
        }
    }
}

struct Delegate;

impl AppDelegate<Model> for Delegate {
    fn window_removed(
        &mut self,
        _id: WindowId,
        _data: &mut Model,
        _env: &Env,
        _ctx: &mut DelegateCtx<'_>,
    ) {
        std::process::exit(0);
    }
}

fn main() -> Result<(), PlatformError> {
    singleton();
    let main_window = WindowDesc::new(_ui_builder)
        .title(WINDOW_TITLE)
        .resizable(false)
        .window_size((400., 200.));

    let model = Model { delay: 200 };
    AppLauncher::with_window(main_window)
        .delegate(Delegate {})
        // .use_simple_logger()
        .launch(model)
}

fn _ui_builder() -> impl Widget<Model> {
    let textbox = TextBox::new()
        .with_placeholder("Delay")
        .with_formatter(DelayFormatter {})
        .update_data_while_editing(true)
        .lens(Model::delay)
        .fix_width(TEXT_BOX_WIDTH)
        .padding(5.0);
    let button = Button::new("Set Delay")
        .on_click(|_ctx, data: &mut Model, _env| {
            _update_delay(data.delay);
        })
        .padding(5.0);

    Flex::row()
        .main_axis_alignment(MainAxisAlignment::Center)
        .cross_axis_alignment(CrossAxisAlignment::Center)
        .with_child(textbox)
        .with_default_spacer()
        .with_child(button)
}

fn _update_delay(delay: u64) {
    let bot = singleton();
    *(bot.autoclicker.lock().unwrap()) = Autoclicker::new(delay);
}
