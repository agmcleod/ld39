use std::fs;

use conrod::Ui;
use loader::get_exe_path;
use serde_json;

#[derive(Serialize, Deserialize)]
pub struct Settings {
    #[serde(default)]
    pub music_volume: f32,
    #[serde(default)]
    pub sound_volume: f32,
    #[serde(default)]
    pub mute_music: bool,
    #[serde(default)]
    pub mute_sound_effects: bool,
    #[serde(default)]
    pub completed_tutorial: bool,
}

impl Settings {
    pub fn set_music_volume(&mut self, volume: f32) {
        self.music_volume = volume;
        self.save();
    }

    pub fn set_sound_volume(&mut self, volume: f32) {
        self.sound_volume = volume;
        self.save();
    }

    pub fn set_mute_music(&mut self, mute: bool) {
        self.mute_music = mute;
        self.save();
    }

    pub fn set_mute_sound_effects(&mut self, mute: bool) {
        self.mute_sound_effects = mute;
        self.save();
    }

    pub fn set_completed_tutorial(&mut self, completed: bool) {
        self.completed_tutorial = completed;
        self.save();
    }

    fn save(&self) {
        let text = serde_json::to_string(&self).unwrap();
        fs::write(get_exe_path().join("settings.json").to_str().unwrap(), text)
            .expect("Unable to write settings");
    }
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            music_volume: 0.5,
            sound_volume: 0.5,
            mute_music: false,
            mute_sound_effects: false,
            completed_tutorial: false,
        }
    }
}

widget_ids! {
    pub struct Ids {
        music_volume,
        sound_volume,
        mute_music,
        mute_sound_effects,
        mute_music_label,
        mute_sound_effects_label,
        settings_label,
        close_button,
    }
}

pub fn create_ui(ui: &mut Ui, ids: &mut Ids, settings: &mut Settings) -> Option<String> {
    use conrod::{color, position, widget, Colorable, Labelable, Positionable, Sizeable, UiBuilder, Widget, position::Relative};

    let ui = &mut ui.set_widgets();

    if widget::Button::new()
        .top_right_with_margin_on(ui.window, 20.0)
        .w_h(30.0, 30.0)
        .label("X")
        .label_color(color::rgb(0.0, 1.0, 0.0))
        .label_x(Relative::Scalar(1.0))
        .label_y(Relative::Scalar(2.0))
        .color(color::rgb(16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0))
        .set(ids.close_button, ui)
        .was_clicked()
    {
        return Some("resume_game".to_string());
    }

    widget::Text::new("Settings")
        .mid_top_with_margin_on(ui.window, 50.0)
        .font_size(32)
        .rgb(0.0, 1.0, 0.0)
        .set(ids.settings_label, ui);

    if let Some(volume) = widget::Slider::new(settings.music_volume, 0.0, 1.0)
        .middle_of(ui.window)
        .down_from(ids.settings_label, 50.0)
        .x_align(position::Align::Middle)
        .color(color::rgb(0.0, 1.0, 0.0))
        .w_h(350.0, 35.0)
        .label("Music Volume")
        .set(ids.music_volume, ui)
    {
        settings.set_music_volume(volume);
    }

    if let Some(volume) = widget::Slider::new(settings.sound_volume, 0.0, 1.0)
        .middle_of(ui.window)
        .color(color::rgb(0.0, 1.0, 0.0))
        .w_h(350.0, 35.0)
        .label("Sound Volume")
        .down_from(ids.music_volume, 25.0)
        .set(ids.sound_volume, ui)
    {
        settings.set_sound_volume(volume);
    }

    widget::Text::new("Mute music")
        .down_from(ids.sound_volume, 25.0)
        .color(color::rgb(0.0, 1.0, 0.0))
        .font_size(20)
        .set(ids.mute_music_label, ui);

    if let Some(state) = widget::Toggle::new(settings.mute_music)
        .down_from(ids.mute_music_label, 25.0)
        .color(color::rgb(0.0, 1.0, 0.0))
        .w_h(35.0, 35.0)
        .label(if settings.mute_music { "X" } else { "" })
        .label_x(Relative::Scalar(0.0))
        .label_y(Relative::Scalar(1.0))
        .label_color(color::rgb(0.0, 0.0, 0.0))
        .set(ids.mute_music, ui)
        .last()
    {
        settings.set_mute_music(state);
    }

    widget::Text::new("Mute sound effects")
        .right_from(ids.mute_music_label, 100.0)
        .color(color::rgb(0.0, 1.0, 0.0))
        .font_size(20)
        .set(ids.mute_sound_effects_label, ui);

    if let Some(state) = widget::Toggle::new(settings.mute_sound_effects)
        .down_from(ids.mute_sound_effects_label, 25.0)
        .color(color::rgb(0.0, 1.0, 0.0))
        .w_h(35.0, 35.0)
        .label(if settings.mute_sound_effects { "X" } else { "" })
        .label_x(Relative::Scalar(0.0))
        .label_y(Relative::Scalar(1.0))
        .label_color(color::rgb(0.0, 0.0, 0.0))
        .set(ids.mute_sound_effects, ui)
        .last()
    {
        settings.set_mute_sound_effects(state);
    }

    None
}
