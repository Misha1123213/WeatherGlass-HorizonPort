fn weather_glass_click(game: &mut crate::game::Game<'_>, mod_id: &str, button: usize) {
    use crate::game::weather_glass as m;
    if mod_id != m::ID {
        return;
    }
    let Some((key, _)) = m::OPTION_KEYS.get(button) else {
        return;
    };
    if *key == "strength" {
        let next = m::strength_from(game.native_setting(m::ID, "strength").as_ref()).cycle();
        game.native_set_setting(m::ID, "strength", next.label());
    } else {
        let on = m::option(key, game.native_setting(m::ID, key).as_ref());
        game.native_set_setting(m::ID, key, if on { "false" } else { "true" });
    }
}