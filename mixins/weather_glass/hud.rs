// The HUD film: weather effects drawn while the player looks at glass.
fn weather_glass_hud(
    game: &mut crate::game::Game<'_>,
    batch: &mut horizon_client::mod_api::HudBatch,
    corners: &mut horizon_client::mod_api::Corners,
) {
    use crate::game::weather_glass as m;

    if !game.native_on(m::ID) {
        return;
    }
    let options = m::options_from(&|key| game.native_setting(m::ID, key));
    let (rain, thunder) = game.native_weather();
    let intensity = rain + thunder * 0.5;
    if intensity <= 0.02 {
        return;
    }

    let Some(player) = game.native_player() else {
        return;
    };

    let (dist, glass_pos) = {
        let yaw = (player.yaw as f64).to_radians();
        let pitch = (player.pitch as f64).to_radians();
        let dx = -yaw.sin() * pitch.cos();
        let dy = -pitch.sin();
        let dz = yaw.cos() * pitch.cos();
        let (ex, ey, ez) = (player.x, player.y + 1.62, player.z);
        let mut found = None;
        let mut dist = 0.0_f32;
        let mut t = 0.15_f64;
        while t < 5.0 {
            let pos = [
                (ex + dx * t).floor() as i32,
                (ey + dy * t).floor() as i32,
                (ez + dz * t).floor() as i32,
            ];
            let name = game.native_block(pos);
            if m::is_air(&name) {
                t += 0.1;
                continue;
            }
            if m::is_glass(&name) {
                found = Some(pos);
                dist = t as f32;
            }
            break;
        }
        match found {
            Some(p) => (dist, p),
            None => return,
        }
    };

    let w = corners.width();
    let h = corners.height();
    let tick = m::next_tick();
    let s = options.strength.mul();

    let Some(biome) = game.native_biome(glass_pos) else {
        return;
    };
    let cold = m::is_cold_biome(&biome);
    let dry = m::is_dry_biome(&biome);

    if options.rain && intensity > 0.02 {
        let drops = (26.0 * intensity * s) as u32;
        for i in 0..drops {
            let x0 = m::hash01(tick / 3, i) * w;
            let speed = 1.6 + 3.2 * m::hash01(tick, i + 1000);
            let off = m::hash01(0, i + 2000) * h;
            let y = h - ((tick as f32 * speed + off) % h);
            let len = 6.0 + 14.0 * m::hash01(tick, i + 3000);
            let x = x0 + (h - y) * 0.05 * (0.5 + thunder);
            let alpha = (48.0 + 48.0 * intensity) as u32;
            game.native_fill(batch, [x, y, 1.5, len], (alpha << 24) | 0x00A8_BDD0);
        }
        for j in 0..10u32 {
            let x = m::hash01(0, j + 4000) * w;
            let y = m::hash01(0, j + 5000) * h;
            let r = 1.0 + 2.0 * m::hash01(0, j + 6000);
            game.native_fill(batch, [x, y, r, r], 0x40A8_BDD0);
        }
    }

    if options.frost && cold {
        let boost = 1.0 + intensity * 0.6;
        let t = (8.0 + 16.0 * boost) * s;
        for layer in 0..3u32 {
            let inset = layer as f32 * t * 0.45;
            let thick = t * (1.0 - layer as f32 * 0.25);
            let alpha = (0x2E_u32.saturating_sub(layer * 0x0A)) << 24;
            let c = alpha | 0x00E8_F4FF;
            game.native_fill(batch, [0.0, inset, w, thick], c);
            game.native_fill(batch, [0.0, h - inset - thick, w, thick], c);
            game.native_fill(batch, [inset, 0.0, thick, h], c);
            game.native_fill(batch, [w - inset - thick, 0.0, thick, h], c);
        }
    }

    if options.dust && dry && rain <= 0.02 {
        for k in 0..4u32 {
            let size = w * (0.25 + 0.2 * m::hash01(0, k + 7000));
            let span = w + size * 2.0;
            let x = ((tick as f32 * (0.4 + 0.4 * m::hash01(0, k + 7100))
                + m::hash01(0, k + 7200) * span)
                % span)
                - size;
            let y = m::hash01(tick / 8, k + 7300) * h * 0.7;
            let alpha = (0x12 + (16.0 * m::hash01(tick, k + 7400)) as u32) << 24;
            game.native_fill(batch, [x, y, size, size * 0.6], alpha | 0x00C1_9A66);
        }
        for g in 0..14u32 {
            let x = m::hash01(tick / 2, g + 8000) * w;
            let y = m::hash01(tick / 2, g + 9000) * h;
            game.native_fill(batch, [x, y, 1.0, 1.0], 0x30C1_9A66);
        }
    }

    if options.breath && cold && dist < 1.7 {
        let phase = tick % 260;
        if phase < 100 {
            let t = phase as f32 / 100.0;
            let alpha = ((1.0 - t) * 34.0) as u32;
            let bw = w * 0.45 * (0.6 + 0.6 * t);
            let bh = h * 0.22 * (0.6 + 0.6 * t);
            let cx = w * 0.5 - bw * 0.5;
            let cy = h * 0.68;
            game.native_fill(batch, [cx, cy, bw, bh], (alpha << 24) | 0x00FF_FF_FF);
            game.native_fill(
                batch,
                [cx + bw * 0.2, cy - bh * 0.3, bw * 0.6, bh * 0.7],
                ((alpha / 2) << 24) | 0x00FF_FF_FF,
            );
        }
    }
}