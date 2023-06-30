use lambda_runtime::Error;
use lib::discord::{CommandOptionValue, InteractionBody};
use lib::entity::user::User;

pub fn get_member_user(e: &InteractionBody) -> Result<&User, Error> {
    Ok(&e.member.as_ref().ok_or("missing member")?.user)
}

pub fn get_memeber_user_id(e: &InteractionBody) -> Result<String, Error> {
    Ok(get_member_user(e)?
        .userid
        .as_ref()
        .ok_or("missing userid")?
        .clone())
}

pub fn get_option_value<'a>(
    b: &'a InteractionBody,
    name: &str,
) -> Result<&'a CommandOptionValue, Error> {
    Ok(&b
        .data
        .as_ref()
        .ok_or("missing data")?
        .options
        .as_ref()
        .ok_or("missing options")?
        .iter()
        .find(|&x| x.name == name)
        .ok_or("missing option")?
        .value)
}
