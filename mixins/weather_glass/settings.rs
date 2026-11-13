fn weather_glass_page(game: &mut crate::game::Game<'_>, mod_id: &str, buttons: &mut Vec<String>) {
    use crate::game::weather_glass as m;
    if mod_id != m::ID {
        return;
    }
    for (key, label) in m::OPTION_KEYS {
        if key == "strength" {
            let s = m::strength_from(game.native_setting(m::ID, "strength").as_ref());
            buttons.push(format!("{label}: {}", s.label()));
        } else {
            let on = m::option(key, game.native_setting(m::ID, key).as_ref());
            buttons.push(format!("{label}: {}", if on { "ON" } else { "OFF" }));
        }
    }
}