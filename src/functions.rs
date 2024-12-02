// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

use crate::data_display::ex_display::{display_Show_RGB, display_box, display_rect, display_set_text_bgcolor, display_set_text_color, display_set_text_origin_position, display_set_text_pos, display_set_text_pos2, display_set_text_scale, display_write_text, RGB};

static COLOR_G: RGB = RGB {r: 0x00, g: 0xFF, b: 0x00};
static COLOR_BLUE: RGB = RGB {r: 0x00, g: 0x00, b: 0xFF};
static COLOR_WHITE: RGB = RGB {r: 0xFF, g: 0xFF, b: 0xFF};
static COLOR_BLACK: RGB = RGB {r: 0x00, g: 0x00, b: 0x00};

pub fn measurement(dato: u16) {
    let dato_ing: f32 = 0.00524590164*(dato as f32);
    // INSTRUMENTACION:
    // StateEvent: variable_value_assigned,measurement_dato_ing,{ dato_ing }
    let dato_ing2:f32 = 1E-13 * 2.71828f32.powf(1.1231*dato_ing);
    // INSTRUMENTACION:
    // StateEvent: variable_value_assigned,measurement_dato_ing2,{ dato_ing2 }
    let pl: u16 =30;
    let ll: u16 =20;
    let scale: u8 = 2;
    display_set_text_scale (scale);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_scale,{ scale }
    display_set_text_pos2(pl, ll);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ pl },{ ll }
    display_set_text_color(COLOR_G);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_color,{ COLOR_G.r },{ COLOR_G.g },{ COLOR_G.b }
    display_set_text_bgcolor(COLOR_BLACK);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_bgcolor,{ COLOR_BLACK.r },{ COLOR_BLACK.g },{ COLOR_BLACK.b }
    if dato_ing > 20.98 {
        display_write_text("**H    \n");
        // INSTRUMENTACION:
        // ComponentEvent: display,display_write_text,**H    \n
    } else {
        if dato_ing < 3.5 {
            display_write_text("**L    \n");
            // INSTRUMENTACION:
            // ComponentEvent: display,display_write_text,**L    \n
        } else {
            display_write_text(format!("{:2.0E} \n", dato_ing2).as_str());
            // INSTRUMENTACION:
            // ComponentEvent: display,display_write_text,{:2.0E dato_ing2 } \n
        }
    }
}

pub fn bar(dato: u16, dato_old: u16) {
    let dato_ing: f32 = 0.00524590164 * dato as f32;
    // INSTRUMENTACION:
    // StateEvent: variable_value_assigned,bar_dato_ing,{:.7 dato_ing }
    let dato_ing_old: f32 = 0.00524590164 * dato_old as f32;
    let g: i16 = (24 as f32 * dato_ing - 96 as f32) as i16;
    let h: i16 = (24 as f32 * dato_ing_old - 96 as f32) as i16;
    // INSTRUMENTACION:
    // StateEvent: variable_value_assigned,bar_point,{ g }
    for i in 66..g+66 {
        if (i >= 49) && (i < 178) {
            display_Show_RGB(0x00, 0xff, 0x00, i as u32, i as u32, 155, 190); //00ff00  1c542d
            // INSTRUMENTACION:
            // ComponentEvent: display,display_Show_RGB,{ 0 },{ 255 },{ 0 },{ i },{ i },{ 150 },{ 190 }
        }
        if (i >= 178) && (i < 400) { //336
            display_Show_RGB(0xff, 0xff, 0x00, i as u32, i as u32, 155, 190);//FFFF00
            // INSTRUMENTACION:
            // ComponentEvent: display,display_Show_RGB,{ 0 },{ 255 },{ 0 },{ i },{ i },{ 150 },{ 190 }
        }
        if (i >= 400) && (i < 450) {
            display_Show_RGB(0xff, 0x00, 0x00, i as u32, i as u32, 155, 190);
            // INSTRUMENTACION:
            // ComponentEvent: display,display_Show_RGB,{ 0 },{ 255 },{ 0 },{ i },{ i },{ 150 },{ 190 }
        }
    }
    if g < h {
        for i in ((g + 67)..=(h + 66)).rev() {
            if (i >= 49) && (i < 450) {
                display_Show_RGB(0x00, 0x00, 0x00, i as u32, i as u32, 155, 190);
                // INSTRUMENTACION:
                // ComponentEvent: display,display_Show_RGB,{ 0 },{ 0 },{ 0 },{ i },{ i },{ 150 },{ 190 }
            }
        }
    }
}

pub fn background() {
    let xl: u16 = 29;
    let yl: u16 = 1;
    let scale: u8 = 2;

    display_set_text_scale(scale);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_scale,{ scale }
    display_set_text_pos(xl, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos,{ xl },{ yl }
    display_set_text_origin_position(yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_origin_position,{ yl }
    display_set_text_color(COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_color,{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_write_text("         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n         \n
    let x0: u16 = 60;
    let yl: u16 = 0;
    let x2: u16 = 108;
    let x3: u16 = 156;
    let x4: u16 = 204;
    let x5: u16 = 252;
    let x6: u16 = 300;
    let x7: u16 = 348;
    let x8: u16 = 396;
    let x9: u16 = 444;
    display_set_text_pos2(x0, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x0 },{ yl }
    display_write_text("1E-11\n");
    // INSTRUMENTACION: ComponentEvent: display,display_write_text,1E-11\n
    display_set_text_pos2(x2, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x2 },{ yl }
    display_write_text("1E-10\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-10\n
    display_set_text_pos2(x3, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x3 },{ yl }
    display_write_text(" 1E-9\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-9\n
    display_set_text_pos2(x4, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x4 },{ yl }
    display_write_text(" 1E-8\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-8\n
    display_set_text_pos2(x5, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x5 },{ yl }
    display_write_text(" 1E-7\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-7\n
    display_set_text_pos2(x6, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x6 },{ yl }
    display_write_text(" 1E-6\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-6\n
    display_set_text_pos2(x7, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x7 },{ yl }
    display_write_text(" 1E-5\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-5\n
    display_set_text_pos2(x8, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x8 },{ yl }
    display_write_text(" 1E-4\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-4\n
    display_set_text_pos2(x9, yl);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_set_text_pos2,{ x9 },{ yl }
    display_write_text(" 1E-3\n");
    // INSTRUMENTACION:
    // ComponentEvent: display,display_write_text,1E-3\n
    display_box(0, 0, 479, 127, COLOR_BLUE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_box,{ 0 },{ 0 },{ 479 },{ 127 },{ COLOR_BLUE.r },{ COLOR_BLUE.g },{ COLOR_BLUE.b }
    display_rect(0, 20, 1, 87, COLOR_BLACK);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 0 },{ 20 },{ 1 },{ 87 },{ COLOR_BLACK.r },{ COLOR_BLACK.g },{ COLOR_BLACK.b }
    display_rect(478, 20, 1, 87, COLOR_BLACK);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 478 },{ 20 },{ 1 },{ 87 },{ COLOR_BLACK.r },{ COLOR_BLACK.g },{ COLOR_BLACK.b }
    display_rect(50 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 50+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(98 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 98+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(146 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 146+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(194 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 194+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(242 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 242+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(290 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 290+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(338 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 338+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(386 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 386+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
    display_rect(434 + 14, 60, 2, 20, COLOR_WHITE);
    // INSTRUMENTACION:
    // ComponentEvent: display,display_rect,{ 434+14 },{ 60 },{ 2 },{ 20 },{ COLOR_WHITE.r },{ COLOR_WHITE.g },{ COLOR_WHITE.b }
}
