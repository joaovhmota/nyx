use crate::{
    commands::{
        about::about, profile_picture::profile_picture, roll_dice::roll_dice, usage::usage,
    },
    types::Error,
};
use poise::Command;

pub mod about;
pub mod profile_picture;
pub mod roll_dice;
pub mod usage;

pub fn avaliable_commands() -> Vec<Command<(), Error>> {
    vec![about(), profile_picture(), roll_dice(), usage()]
}
