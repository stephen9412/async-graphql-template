# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "inventory" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "inventory")] # [graphql (complex)] # [graphql (name = "Inventory")] pub struct Model { pub inventory_id : i32 , pub film_id : i16 , pub store_id : i16 , pub last_update : DateTime , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { InventoryId , FilmId , StoreId , LastUpdate , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { InventoryId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { Film , Store , Rental , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: InventoryId => ColumnType :: Integer . def () , Self :: FilmId => ColumnType :: SmallInteger . def () , Self :: StoreId => ColumnType :: SmallInteger . def () , Self :: LastUpdate => ColumnType :: DateTime . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: Film => Entity :: belongs_to (super :: film :: Entity) . from (Column :: FilmId) . to (super :: film :: Column :: FilmId) . into () , Self :: Store => Entity :: belongs_to (super :: store :: Entity) . from (Column :: StoreId) . to (super :: store :: Column :: StoreId) . into () , Self :: Rental => Entity :: has_many (super :: rental :: Entity) . into () , } } } impl Related < super :: film :: Entity > for Entity { fn to () -> RelationDef { Relation :: Film . def () } } impl Related < super :: store :: Entity > for Entity { fn to () -> RelationDef { Relation :: Store . def () } } impl Related < super :: rental :: Entity > for Entity { fn to () -> RelationDef { Relation :: Rental . def () } } impl ActiveModelBehavior for ActiveModel { }