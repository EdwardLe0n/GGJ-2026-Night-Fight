use std::collections::VecDeque;

use turbo::*;

use crate::turbecs;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

// Standard Components
use component_system::components::{comp_text::TextComponent, comp_text_box::TextBoxComponent, comp_butn::ButtonComponent};
use component_system::components::buttons::button_types::ButtonTypes;

use component_system::components::{comp_rect::RectangleComponent, comp_spr::SpriteComponent};

use turbecs::helpers::{bound_data};

// User defined components
use crate::assets;

use assets::components::{misc_components, game_components};

const PLAYER_BUTTON_WIDTH : i32 = 100;
const PLAYER_BUTTON_WIDTH_OFFSET : i32 = 16;
const PLAYER_BUTTON_HEIGHT : i32 = 80;
const PLAYER_BUTTON_HEIGHT_OFFSET : i32 = 20;

fn new_player_button(some_x : i32, some_y : i32, some_type : ButtonTypes) -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Title".to_string());
    let mut ent_queue = VecDeque::new();

    ent.transform.nudge_x(screen().w() as i32 / -2 + PLAYER_BUTTON_WIDTH / 2 + PLAYER_BUTTON_WIDTH_OFFSET);
    ent.transform.nudge_x(some_x * (PLAYER_BUTTON_WIDTH + PLAYER_BUTTON_WIDTH_OFFSET));

    ent.transform.nudge_y(PLAYER_BUTTON_HEIGHT / 2);
    ent.transform.nudge_y(-some_y * (PLAYER_BUTTON_HEIGHT + PLAYER_BUTTON_HEIGHT_OFFSET));

    let mut some_button = ButtonComponent::new();

    some_button.button_type = some_type; 

    some_button.color = 0x000000ff;

    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(4);
    some_button.border.set_size(4);

    some_button.transform.set_height(PLAYER_BUTTON_HEIGHT);
    some_button.transform.set_width(PLAYER_BUTTON_WIDTH);

    ent_queue.push_back(
        Component::new(
            ComponentData::Button(
                some_button
            )
        )
    );

    let mut some_spr = SpriteComponent::new("game/player_buttons/player_input_sheet".to_string());

    some_spr.cover = false;

    some_spr.transform.nudge_x(PLAYER_BUTTON_WIDTH / -2);
    some_spr.transform.nudge_y(PLAYER_BUTTON_HEIGHT / 2);

    some_spr.transform.set_width(PLAYER_BUTTON_WIDTH);
    some_spr.transform.set_height(PLAYER_BUTTON_HEIGHT);

    some_spr.tx = PLAYER_BUTTON_WIDTH * some_x;
    some_spr.ty = PLAYER_BUTTON_HEIGHT * some_y;

    ent_queue.push_back(
        Component::new(
            ComponentData::Sprite(
                some_spr
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_offense_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Offense);

}

pub fn new_agile_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Agile);

}

pub fn new_wait_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Idle);

}

pub fn new_mixed_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Mixed);

}

pub fn new_tank_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Tank);

}

pub fn new_nimble_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Nimble);

}

pub fn new_aggro_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Aggro);

}

pub fn new_parry_button (some_x : i32, some_y : i32) -> (Entity, VecDeque<Component>) {

    return new_player_button(some_x, some_y, ButtonTypes::Parry);

}