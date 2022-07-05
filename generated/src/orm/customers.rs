//! SeaORM Entity. Generated by sea-orm-codegen 0.8.0

use sea_orm::entity::prelude::*;

#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;

impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "customers"
    }
}

#[derive(Clone, Debug, PartialEq, DeriveModel, DeriveActiveModel)]
pub struct Model {
    pub customer_id: i32,
    pub first_name: String,
    pub last_name: String,
    pub company: Option<String>,
    pub address: Option<String>,
    pub city: Option<String>,
    pub state: Option<String>,
    pub country: Option<String>,
    pub postal_code: Option<String>,
    pub phone: Option<String>,
    pub fax: Option<String>,
    pub email: String,
    pub support_rep_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    #[sea_orm(column_name = "CustomerId")]
    CustomerId,
    #[sea_orm(column_name = "FirstName")]
    FirstName,
    #[sea_orm(column_name = "LastName")]
    LastName,
    #[sea_orm(column_name = "Company")]
    Company,
    #[sea_orm(column_name = "Address")]
    Address,
    #[sea_orm(column_name = "City")]
    City,
    #[sea_orm(column_name = "State")]
    State,
    #[sea_orm(column_name = "Country")]
    Country,
    #[sea_orm(column_name = "PostalCode")]
    PostalCode,
    #[sea_orm(column_name = "Phone")]
    Phone,
    #[sea_orm(column_name = "Fax")]
    Fax,
    #[sea_orm(column_name = "Email")]
    Email,
    #[sea_orm(column_name = "SupportRepId")]
    SupportRepId,
}

#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    CustomerId,
}

impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = i32;
    fn auto_increment() -> bool {
        true
    }
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Employees,
    Invoices,
}

impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::CustomerId => ColumnType::Integer.def(),
            Self::FirstName => ColumnType::String(None).def(),
            Self::LastName => ColumnType::String(None).def(),
            Self::Company => ColumnType::String(None).def().null(),
            Self::Address => ColumnType::String(None).def().null(),
            Self::City => ColumnType::String(None).def().null(),
            Self::State => ColumnType::String(None).def().null(),
            Self::Country => ColumnType::String(None).def().null(),
            Self::PostalCode => ColumnType::String(None).def().null(),
            Self::Phone => ColumnType::String(None).def().null(),
            Self::Fax => ColumnType::String(None).def().null(),
            Self::Email => ColumnType::String(None).def(),
            Self::SupportRepId => ColumnType::Integer.def().null(),
        }
    }
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Employees => Entity::belongs_to(super::employees::Entity)
                .from(Column::SupportRepId)
                .to(super::employees::Column::EmployeeId)
                .into(),
            Self::Invoices => Entity::has_many(super::invoices::Entity).into(),
        }
    }
}

impl Related<super::employees::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Employees.def()
    }
}

impl Related<super::invoices::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Invoices.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}