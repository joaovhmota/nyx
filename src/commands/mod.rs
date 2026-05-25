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

pub fn get_commands_that_ignore_metrics() -> Vec<String> {
    vec![stringify!(usage).to_string()]
}

pub fn does_command_ignore_metrics(command: &impl ToString) -> bool {
    get_commands_that_ignore_metrics().contains(&command.to_string())
}
