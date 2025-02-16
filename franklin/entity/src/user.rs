use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Salt,
    HashedPassword,
    Skills,
    Interests
}
