use crate::server::repositories::repositories_role::get_roles;

pub fn read_roles() -> String{
    get_roles()
}