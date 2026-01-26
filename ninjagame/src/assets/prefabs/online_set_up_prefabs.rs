use std::collections::VecDeque;

use turbo::*;

use crate::turbecs;
use crate::turbecs::component_system::components::buttons::button_types::ButtonTypes;

use turbecs::{entity::Entity, component_system};
use component_system::component::{Component, ComponentData};

use turbecs::helpers::{bound_data};

// Standard Components
use component_system::components::{comp_text_box::TextBoxComponent, comp_butn::ButtonComponent};

// User defined components
use crate::assets;

use assets::components::{misc_components, game_components, online_components};

use misc_components::{comp_textbox_resizer::TextBoxResizerComponent, comp_score_text_update::ScoreTextUpdateComponent};
use misc_components::comp_logo_fade::{LogoFade, LogoFadeComponent};

use online_components::comp_code_container::CodeContainerComponent;
use online_components::comp_number::NumberComponent;
use online_components::comp_host_check::HostCheckComponent;

pub fn new_code_container () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("code container".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(10);

    
    ent_queue.push_back(
        Component::new(
            ComponentData::CodeContainer(
                CodeContainerComponent::new()
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_number (some_u32 : u32) -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("code container".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(10);

    let wl = 30;

    // handles offset per number

    ent.transform.nudge_x((screen().w() as i32) / -2 + 40);
    ent.transform.nudge_x(40);

    if some_u32 >= 5 {
        ent.transform.nudge_y(-60);
    }

    ent.transform.nudge_x((wl * 2 + 20) * (some_u32 as i32 % 5));

    // setting up button data

    let mut some_button = ButtonComponent::new();

    some_button.transform.set_width(wl);
    some_button.transform.set_height(wl);

    some_button.color = 0x000000ff;

    some_button.button_type = ButtonTypes::Number;

    some_button.border.set_size(2);
    some_button.border.set_color(0xffffffff);
    some_button.border.set_radius(2);

    ent_queue.push_back(
        Component::new(
            ComponentData::Button(
                some_button
            )
        )
    );

    let mut some_txt_box = TextBoxComponent::new(some_u32.to_string());

    some_txt_box.transform.set_width(wl);
    some_txt_box.transform.set_height(wl);

    some_txt_box.transform.nudge_y(wl / 2 + 4);

    some_txt_box.color = 0xffffffff;

    some_txt_box.transform.position.set_horizontal_pref(bound_data::Horizontal::Center);

    ent_queue.push_back(
        Component::new(
            ComponentData::TextBox(some_txt_box)
        )
    );

    ent_queue.push_back(
        Component::new(
            ComponentData::Number(
                NumberComponent::new(some_u32)
            )
        )
    );

    return (ent, ent_queue);

}

pub fn new_clear () -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Clear button".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.transform.position.nudge_y(50);
    ent.transform.position.nudge_x((screen().w() as i32) / 4);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xffffffff);

    button.transform.set_width(100);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::Clear;

    ent_queue.push_back(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Clear".to_string());

    text_box.transform.set_height(100);
    text_box.transform.set_width(100);

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent_queue.push_back(Component::new(ComponentData::TextBox(text_box)));

    ent_queue.push_back(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return (ent, ent_queue);

}

pub fn new_to_host_wait() -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("Clear button".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.transform.position.nudge_y(50);
    ent.transform.position.nudge_x(0);

    ent.transform.position.set_vertical_pref(bound_data::Vertical::Bottom);

    let mut button = ButtonComponent::new();

    button.color = 0x000000ff;

    button.border.set_size(2);
    button.border.set_radius(2);
    button.border.set_color(0xffffffff);

    button.transform.set_width(100);
    button.transform.set_height(40);

    button.button_type = ButtonTypes::ToHostWait;

    ent_queue.push_back(Component::new(ComponentData::Button(button)));

    let mut text_box = TextBoxComponent::new("Host".to_string());

    text_box.transform.set_height(100);
    text_box.transform.set_width(100);

    text_box.font = "large".to_string();
    text_box.color = 0xffffffff;

    ent_queue.push_back(Component::new(ComponentData::TextBox(text_box)));

    ent_queue.push_back(Component::new(ComponentData::TextBoxResizer(TextBoxResizerComponent::new_with_buffers(2, 2))));

    return (ent, ent_queue);

}

pub fn new_host_check_up (some_u32 : u32) -> (Entity, VecDeque<Component>) {

    let mut ent = Entity::new_base("host check up".to_string());
    let mut ent_queue = VecDeque::new();
    
    ent.set_layer(3);
    
    ent_queue.push_back(
        Component::new(
            ComponentData::HostCheck(
                HostCheckComponent::new(some_u32)
            )
        )
    );

    return (ent, ent_queue);

}