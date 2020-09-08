use crate::utils::{max, min};
use image::Rgba;

pub fn blend_alpha((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = atf + abf * (1.0 - atf);

    let mut r = if af == 0.0 {
        0.0
    } else {
        (rb as f32 * abf + rt as f32 * atf * (1.0 - abf)) / af
    };
    let mut g = if af == 0.0 {
        0.0
    } else {
        (gb as f32 * abf + gt as f32 * atf * (1.0 - abf)) / af
    };
    let mut b = if af == 0.0 {
        0.0
    } else {
        (bb as f32 * abf + bt as f32 * atf * (1.0 - abf)) / af
    };
    let a = max(0.0, min(255.0, (abf + atf * (1.0 - abf)) * 255.0));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_multiplicative((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let atf = 1.0 * (at as f32 / 255.0);

    let mut r = rb as f32 * (1.0 - atf) + rt as f32 * atf;
    let mut g = gb as f32 * (1.0 - atf) + gt as f32 * atf;
    let mut b = bb as f32 * (1.0 - atf) + bt as f32 * atf;
    let a = max(0, min(255, at as u16 + ab as u16));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_source_over((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = abf + atf * (1.0 - abf);

    let mut r = if af == 0.0 {
        0.0
    } else {
        (rb as f32 * abf + rt as f32 * atf * (1.0 - abf)) / af
    };
    let mut g = if af == 0.0 {
        0.0
    } else {
        (gb as f32 * abf + gt as f32 * atf * (1.0 - abf)) / af
    };
    let mut b = if af == 0.0 {
        0.0
    } else {
        (bb as f32 * abf + bt as f32 * atf * (1.0 - abf)) / af
    };
    let a = max(0.0, min(255.0, af * 255.0));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_destination_over((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);
    let af = atf + abf * (1.0 - atf);

    let mut r = if af == 0.0 {
        0.0
    } else {
        (rt as f32 * atf + rb as f32 * abf * (1.0 - atf)) / af
    };
    let mut g = if af == 0.0 {
        0.0
    } else {
        (gt as f32 * atf + gb as f32 * abf * (1.0 - atf)) / af
    };
    let mut b = if af == 0.0 {
        0.0
    } else {
        (bt as f32 * atf + bb as f32 * abf * (1.0 - atf)) / af
    };
    let a = max(0.0, min(255.0, af * 255.0));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_first_top((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let mut r = if at == 0 { rb } else { rt };
    let mut g = if at == 0 { gb } else { gt };
    let mut b = if at == 0 { bb } else { bt };
    let mut a = if at == 0 { ab } else { at };

    r = max(0, min(255, r));
    g = max(0, min(255, g));
    b = max(0, min(255, b));
    a = max(0, min(255, a));

    bot_pixel[0] = r;
    bot_pixel[1] = g;
    bot_pixel[2] = b;
    bot_pixel[3] = a;
}

pub fn blend_first_bottom((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let mut r = if ab == 0 { rt } else { rb };
    let mut g = if ab == 0 { gt } else { gb };
    let mut b = if ab == 0 { bt } else { bb };
    let mut a = if ab == 0 { at } else { ab };

    r = max(0, min(255, r));
    g = max(0, min(255, g));
    b = max(0, min(255, b));
    a = max(0, min(255, a));

    bot_pixel[0] = r;
    bot_pixel[1] = g;
    bot_pixel[2] = b;
    bot_pixel[3] = a;
}

//TODO: debug different result from original pconvert
pub fn blend_disjoint_over((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);

    let mut r = if atf + abf < 1.0 {
        rt as f32 + rb as f32 * (1.0 - atf) / abf
    } else {
        rt as f32 + rb as f32
    };
    let mut g = if atf + abf < 1.0 {
        gt as f32 + gb as f32 * (1.0 - atf) / abf
    } else {
        gt as f32 + gb as f32
    };
    let mut b = if atf + abf < 1.0 {
        bt as f32 + bb as f32 * (1.0 - atf) / abf
    } else {
        bt as f32 + bb as f32
    };
    let a = max(0, min(255, at as u16 + ab as u16));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_disjoint_under((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let (rb, gb, bb, ab) = (bot_pixel[0], bot_pixel[1], bot_pixel[2], bot_pixel[3]);
    let (rt, gt, bt, at) = (top_pixel[0], top_pixel[1], top_pixel[2], top_pixel[3]);

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);

    let mut r = if atf * abf > 0.0 {
        rt as f32 / atf * (1.0 - abf) + rb as f32
    } else {
        rt as f32 * (1.0 - abf) + rb as f32
    };
    let mut g = if atf * abf > 0.0 {
        gt as f32 / atf * (1.0 - abf) + gb as f32
    } else {
        gt as f32 * (1.0 - abf) + gb as f32
    };
    let mut b = if atf * abf > 0.0 {
        bt as f32 / atf * (1.0 - abf) + bb as f32
    } else {
        bt as f32 * (1.0 - abf) + bb as f32
    };
    let a = max(0, min(255, at as u16 + ab as u16));

    r = max(0.0, min(255.0, r));
    g = max(0.0, min(255.0, g));
    b = max(0.0, min(255.0, b));

    bot_pixel[0] = r as u8;
    bot_pixel[1] = g as u8;
    bot_pixel[2] = b as u8;
    bot_pixel[3] = a as u8;
}

pub fn blend_disjoint_debug((bot_pixel, top_pixel): (&mut Rgba<u8>, &Rgba<u8>)) {
    let ab = bot_pixel[3];
    let at = top_pixel[3];

    let abf = 1.0 * (ab as f32 / 255.0);
    let atf = 1.0 * (at as f32 / 255.0);

    let mut r = if atf + abf < 1.0 { 0 } else { 255 };
    let mut g = if atf + abf < 1.0 { 255 } else { 0 };
    let mut b = 0;
    let a = max(0, min(255, at as u16 + ab as u16));

    r = max(0, min(255, r));
    g = max(0, min(255, g));
    b = max(0, min(255, b));

    bot_pixel[0] = r;
    bot_pixel[1] = g;
    bot_pixel[2] = b;
    bot_pixel[3] = a as u8;
}
