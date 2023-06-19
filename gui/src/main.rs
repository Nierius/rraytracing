use std::time::Duration;

use gtk::cairo::Context;
use gtk::prelude::*;
use relm4::drawing::DrawHandler;
use relm4::{gtk, Component, ComponentParts, ComponentSender, RelmApp, RelmWidgetExt};
use renderer::renderer::renderer::Renderer;
use shared::traits::Render;

#[derive(Debug)]
enum Msg {
    Render,
    Resize((i32, i32)),
}

#[derive(Debug)]
struct UpdatePointsMsg;

struct App {
    width: i32,
    height: i32,
    handler: DrawHandler,
}

#[relm4::component]
impl Component for App {
    type Init = ();
    type Input = Msg;
    type Output = ();
    type CommandOutput = UpdatePointsMsg;

    view! {
      gtk::Window {
        set_default_size: (600, 300),

        gtk::Box {
          set_orientation: gtk::Orientation::Vertical,
          set_margin_all: 10,
          set_spacing: 10,
          set_hexpand: true,

          gtk::Button {
            set_label: "Render",
            connect_clicked => Msg::Render
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

    fn update(&mut self, msg: Msg, _sender: ComponentSender<Self>, _root: &Self::Root) {
        let cx = self.handler.get_context();

        match msg {
            Msg::Render => draw(&cx, self.width, self.height),
            Msg::Resize((x, y)) => {
                self.width = x;
                self.height = y;
            }
        }
    }

    fn init(
        _: Self::Init,
        root: &Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = App {
            width: 100,
            height: 100,
            handler: DrawHandler::new(),
        };

        let area = model.handler.drawing_area();
        let widgets = view_output!();

        sender.command(|out, shutdown| {
            shutdown
                .register(async move {
                    loop {
                        tokio::time::sleep(Duration::from_millis(20)).await;
                        out.send(UpdatePointsMsg).unwrap();
                    }
                })
                .drop_on_shutdown()
        });

        ComponentParts { model, widgets }
    }
}

fn draw(cx: &Context, width: i32, height: i32) {
    let renderer = Renderer::default();
    for i in 0..height {
        for j in 0..width {
            let y = height - i - 1;
            println!("Attempting to draw {} {}", y, j);
            let pixel = renderer.render_pixel(j, y, width, height);
            cx.set_source_rgb(pixel.r, pixel.g, pixel.b);
            cx.rectangle(j as f64, i as f64, 1.0, 1.0);
            cx.fill().expect("Couldn't fill rect");
        }
    }
}

fn main() {
    let app = RelmApp::new(env!("CARGO_PKG_NAME"));
    app.run::<App>(());
}
