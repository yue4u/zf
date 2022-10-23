use gdnative::{
    api::{Area, TextureProgress},
    prelude::*,
};

#[derive(NativeClass)]
#[inherit(Spatial)]
pub struct TDummy {
    hp: Option<Ref<TextureProgress>>,
}

#[methods]
impl TDummy {
    fn new(_base: &Spatial) -> Self {
        godot_print!("prepare TDummy");
        TDummy { hp: None }
    }

    #[method]
    fn _ready(&mut self, #[base] base: TRef<Spatial>) -> Option<()> {
        let hp = unsafe {
            base.get_node("HP/Viewport/Control")
                .expect("expect TextureProgress")
                .assume_safe()
        }
        .cast::<TextureProgress>()
        .expect("expect can cast TextureProgress")
        .claim();
        self.hp = Some(hp);
        Some(())
    }

    #[method]
    pub fn damage(&self) {
        let hp = unsafe { self.hp.unwrap().assume_safe() };
        hp.set_value(hp.value() - 0.5 * hp.max());
    }
}
