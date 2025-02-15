use sea_orm::DeriveIden;

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    HashedPassword,
    Skills,
    Interests
}
