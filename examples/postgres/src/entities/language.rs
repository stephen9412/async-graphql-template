# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "language" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "language")] # [graphql (complex)] # [graphql (name = "Language")] pub struct Model { pub language_id : i32 , pub name : String , pub last_update : DateTime , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { LanguageId , Name , LastUpdate , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { LanguageId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: LanguageId => ColumnType :: Integer . def () , Self :: Name => ColumnType :: Char (Some (20u32)) . def () , Self :: LastUpdate => ColumnType :: DateTime . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { panic ! ("No RelationDef") } } impl ActiveModelBehavior for ActiveModel { }