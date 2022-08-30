# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "customer" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "customer")] # [graphql (complex)] # [graphql (name = "Customer")] pub struct Model { pub customer_id : i32 , pub store_id : i16 , pub first_name : String , pub last_name : String , pub email : Option < String > , pub address_id : i16 , pub activebool : bool , pub create_date : Date , pub last_update : Option < DateTime > , pub active : Option < i32 > , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { CustomerId , StoreId , FirstName , LastName , Email , AddressId , Activebool , CreateDate , LastUpdate , Active , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { CustomerId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { Address , Store , Payment , Rental , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: CustomerId => ColumnType :: Integer . def () , Self :: StoreId => ColumnType :: SmallInteger . def () , Self :: FirstName => ColumnType :: String (Some (45u32)) . def () , Self :: LastName => ColumnType :: String (Some (45u32)) . def () , Self :: Email => ColumnType :: String (Some (50u32)) . def () . null () , Self :: AddressId => ColumnType :: SmallInteger . def () , Self :: Activebool => ColumnType :: Boolean . def () , Self :: CreateDate => ColumnType :: Date . def () , Self :: LastUpdate => ColumnType :: DateTime . def () . null () , Self :: Active => ColumnType :: Integer . def () . null () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: Address => Entity :: belongs_to (super :: address :: Entity) . from (Column :: AddressId) . to (super :: address :: Column :: AddressId) . into () , Self :: Store => Entity :: belongs_to (super :: store :: Entity) . from (Column :: StoreId) . to (super :: store :: Column :: StoreId) . into () , Self :: Payment => Entity :: has_many (super :: payment :: Entity) . into () , Self :: Rental => Entity :: has_many (super :: rental :: Entity) . into () , } } } impl Related < super :: address :: Entity > for Entity { fn to () -> RelationDef { Relation :: Address . def () } } impl Related < super :: store :: Entity > for Entity { fn to () -> RelationDef { Relation :: Store . def () } } impl Related < super :: payment :: Entity > for Entity { fn to () -> RelationDef { Relation :: Payment . def () } } impl Related < super :: rental :: Entity > for Entity { fn to () -> RelationDef { Relation :: Rental . def () } } impl ActiveModelBehavior for ActiveModel { }