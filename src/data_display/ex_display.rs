// Copyright (c) 2024 INVAP, open@invap.com.ar
// SPDX-License-Identifier: AGPL-3.0-or-later OR Fundacion-Sadosky-Commercial

#[derive(Debug, Clone, Copy)]
pub struct RGB {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

pub fn display_rect(_height: u32, _width: u32, _w: u32, _h: u32, _color: RGB){
}
pub fn display_box(_height: u32, _width: u32, _w: u32, _h: u32, _color: RGB){
}
pub fn display_set_text_color(_color: RGB){
}
pub fn display_set_text_bgcolor(_color: RGB){
}
pub fn display_set_text_scale(_scale: u8){
}
pub fn display_set_text_pos(_height: u16, _width: u16){
}
pub fn display_set_text_pos2(_height: u16, _width: u16){
}
pub fn display_set_text_origin_position(_width: u16){
}
pub fn display_write_text(_text: &str){
}
pub fn display_Show_RGB(_r: u8, _g: u8, _b: u8, _height0: u32, _height1: u32, _width0: u32, _width1: u32){
}
