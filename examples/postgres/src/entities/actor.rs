# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "actor" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "actor")] # [graphql (complex)] # [graphql (name = "Actor")] pub struct Model { pub actor_id : i32 , pub first_name : String , pub last_name : String , pub last_update : DateTime , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { ActorId , FirstName , LastName , LastUpdate , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { ActorId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { FilmActor , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: ActorId => ColumnType :: Integer . def () , Self :: FirstName => ColumnType :: String (Some (45u32)) . def () , Self :: LastName => ColumnType :: String (Some (45u32)) . def () , Self :: LastUpdate => ColumnType :: DateTime . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: FilmActor => Entity :: has_many (super :: film_actor :: Entity) . into () , } } } impl Related < super :: film_actor :: Entity > for Entity { fn to () -> RelationDef { Relation :: FilmActor . def () } } impl ActiveModelBehavior for ActiveModel { }