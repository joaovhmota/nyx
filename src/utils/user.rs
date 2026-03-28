use serenity::all::User;

pub fn get_user_name(user: &User) -> &String {
    match &user.global_name {
        Some(user_global_name) => user_global_name,
        None => &user.name,
    }
}
