# ! [doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"] use sea_orm :: entity :: prelude :: * ; # [derive (Copy , Clone , Default , Debug , DeriveEntity)] pub struct Entity ; impl EntityName for Entity { fn table_name (& self) -> & str { "store" } } # [derive (Clone , Debug , PartialEq , DeriveModel , DeriveActiveModel , async_graphql :: SimpleObject , seaography_derive :: Filter)] # [sea_orm (table_name = "store")] # [graphql (complex)] # [graphql (name = "Store")] pub struct Model { pub store_id : i32 , pub manager_staff_id : i16 , pub address_id : i16 , pub last_update : DateTime , } # [derive (Copy , Clone , Debug , EnumIter , DeriveColumn)] pub enum Column { StoreId , ManagerStaffId , AddressId , LastUpdate , } # [derive (Copy , Clone , Debug , EnumIter , DerivePrimaryKey)] pub enum PrimaryKey { StoreId , } impl PrimaryKeyTrait for PrimaryKey { type ValueType = i32 ; fn auto_increment () -> bool { true } } # [derive (Copy , Clone , Debug , EnumIter)] pub enum Relation { Address , Staff , Customer , Inventory , } impl ColumnTrait for Column { type EntityName = Entity ; fn def (& self) -> ColumnDef { match self { Self :: StoreId => ColumnType :: Integer . def () , Self :: ManagerStaffId => ColumnType :: SmallInteger . def () , Self :: AddressId => ColumnType :: SmallInteger . def () , Self :: LastUpdate => ColumnType :: DateTime . def () , } } } # [seaography_derive :: relation] impl RelationTrait for Relation { fn def (& self) -> RelationDef { match self { Self :: Address => Entity :: belongs_to (super :: address :: Entity) . from (Column :: AddressId) . to (super :: address :: Column :: AddressId) . into () , Self :: Staff => Entity :: belongs_to (super :: staff :: Entity) . from (Column :: ManagerStaffId) . to (super :: staff :: Column :: StaffId) . into () , Self :: Customer => Entity :: has_many (super :: customer :: Entity) . into () , Self :: Inventory => Entity :: has_many (super :: inventory :: Entity) . into () , } } } impl Related < super :: address :: Entity > for Entity { fn to () -> RelationDef { Relation :: Address . def () } } impl Related < super :: staff :: Entity > for Entity { fn to () -> RelationDef { Relation :: Staff . def () } } impl Related < super :: customer :: Entity > for Entity { fn to () -> RelationDef { Relation :: Customer . def () } } impl Related < super :: inventory :: Entity > for Entity { fn to () -> RelationDef { Relation :: Inventory . def () } } impl ActiveModelBehavior for ActiveModel { }