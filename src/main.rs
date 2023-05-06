use std::{f32::consts::PI, sync::Arc};

use glam::Quat;
use stardust_xr_fusion::{
    client::{Client, FrameInfo, RootHandler},
    core::values::Transform,
    drawable::{Alignment, Text, TextStyle},
};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let (client, event_loop) = Client::connect_with_async_loop().await.unwrap();

    let _root = client.wrap_root(Lightspeed::new(&client));

    tokio::select! {
        biased;
        _ = tokio::signal::ctrl_c() => (),
        e = event_loop => e.unwrap().unwrap(),
    };
}

struct Lightspeed(Text);
impl Lightspeed {
    fn new(client: &Arc<Client>) -> Self {
        let text = Text::create(
            client.get_hmd(),
            Transform::from_position_rotation([0.0, 0.0, -1.0], Quat::from_rotation_y(PI)),
            "test",
            TextStyle {
                character_height: 0.05,
                text_align: Alignment::Center.into(),
                ..Default::default()
            },
        )
        .unwrap();

        Lightspeed(text)
    }
}
impl RootHandler for Lightspeed {
    fn frame(&mut self, info: FrameInfo) {
        self.0.set_text(format!("{:.1}", 1.0 / info.delta)).unwrap();
    }
}
