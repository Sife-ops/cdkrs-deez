use crate::deez::{DdbEntity, Deez};
use crate::entity::user::User;

// todo: too much unwrap
pub async fn onboard(d: &Deez, u: &User) {
    let a = d.query("primary", u).send().await.unwrap();
    let b = User::from_map_slice(a.items().unwrap());
    if b.len() < 1 {
        d.put(u).send().await.unwrap();
    } else {
        // todo: update user
    }
}
