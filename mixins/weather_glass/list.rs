fn weather_glass_listed(mods: &mut Vec<crate::game::native_hooks::NativeMod>) {
    use crate::game::weather_glass as m;
    mods.push(crate::game::native_hooks::NativeMod {
        id: m::ID.into(),
        title: format!("Weather Glass {}", m::VERSION),
        description: "Weather films on glass. Port of WeatherGlass.".into(),
        keys: Vec::new(),
    });
}