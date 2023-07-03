use std::time::Duration;

use gtk::cairo::Context;
use gtk::prelude::RangeExt;
use gtk::prelude::*;
use relm4::component::{AsyncComponent, AsyncComponentParts};
use relm4::drawing::DrawHandler;
use relm4::{gtk, AsyncComponentSender, RelmApp, RelmWidgetExt};
use renderer::renderer::renderer::Renderer;
use shared::traits::Render;

const DEFAULT_SAMPLES_PER_PIXEL_VALUE: i16 = 10;

#[derive(Debug)]
enum Msg {
    Render,
    Resize((i32, i32)),
    SampleAmountChanged(f64),
}

struct App {
    width: i32,
    height: i32,
    handler: DrawHandler,
    samples_per_pixel: i16,
}

#[relm4::component(async)]
impl AsyncComponent for App {
    type Init = ();
    type Input = Msg;
    type Output = ();
    type CommandOutput = ();

    view! {
      gtk::Window {
        set_default_size: (900, 450),

        gtk::Box {
          set_orientation: gtk::Orientation::Vertical,
          set_margin_all: 10,
          set_spacing: 10,
          set_hexpand: true,
          set_vexpand: true,

          gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_spacing: 10,

            gtk::Box {
              set_orientation: gtk::Orientation::Vertical,

              gtk::Label {
                set_label: "Samples per pixel (anti-aliasing)"
              },

              gtk::Scale::with_range(gtk::Orientation::Horizontal, 5.0, 200.0, 5.0) {
                set_draw_value: true,
                set_digits: 0,
                set_value: DEFAULT_SAMPLES_PER_PIXEL_VALUE as f64,
                set_hexpand: true,
                connect_value_changed[sender] => move |scale| {
                    sender.input(Msg::SampleAmountChanged(scale.value()));
                }
              },
            },

            gtk::Button {
              set_label: "Render",
              connect_clicked => Msg::Render
            },
          },

          #[local_ref]
          area -> gtk::DrawingArea {
            set_vexpand: true,
            set_hexpand: true,

            connect_resize[sender] => move |_, x, y| {
                sender.input(Msg::Resize((x, y)));
            }
          },
        }
      }
    }

    async fn update(&mut self, msg: Msg, _sender: AsyncComponentSender<Self>, _root: &Self::Root) {
        let cx = self.handler.get_context();

        match msg {
            Msg::Render => {
                let frame = tokio::spawn(render(self.width, self.height, self.samples_per_pixel))
                    .await
                    .unwrap();
                draw(&cx, frame);
            }
            Msg::Resize((x, y)) => {
                self.width = x;
                self.height = y;
            }
            Msg::SampleAmountChanged(new_val) => {
                self.samples_per_pixel = new_val as i16;
                println!("Samples changed to {}", self.samples_per_pixel);
            }
        }
    }

    async fn init(
        _: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = App {
            width: 100,
            height: 100,
            handler: DrawHandler::new(),
            samples_per_pixel: DEFAULT_SAMPLES_PER_PIXEL_VALUE,
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    loop {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                        out.send(()).unwrap();
                    }
                })
                .drop_on_shutdown()
        });

        AsyncComponentParts { model, widgets }
    }
}

async fn render(width: i32, height: i32, samples_per_pixel: i16) -> shared::data::Frame {
    let renderer = Renderer::default();
    renderer.render(width, height, samples_per_pixel)
}

fn draw(cx: &Context, frame: shared::data::Frame) {
    let frame_iter = frame.pixels.into_iter();
    let mut y = frame.height - 1;
    let mut x = 0;

    for pixel in frame_iter {
        cx.set_source_rgb(pixel.r, pixel.g, pixel.b);
        cx.rectangle(x as f64, y as f64, 1.0, 1.0);
        cx.fill().expect("Couldn't fill rect");

        x += 1;
        if x == frame.width {
            y -= 1;
            x = 0;
        }
    }
}

fn main() {
    let app = RelmApp::new(env!("CARGO_PKG_NAME"));
    app.run_async::<App>(());
}
