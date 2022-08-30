#![doc = " SeaORM Entity. Generated by sea-orm-codegen 0.9.1"]
use sea_orm::entity::prelude::*;
#[derive(Copy, Clone, Default, Debug, DeriveEntity)]
pub struct Entity;
impl EntityName for Entity {
    fn table_name(&self) -> &str {
        "playlist_track"
    }
}
#[derive(
    Clone,
    Debug,
    PartialEq,
    DeriveModel,
    DeriveActiveModel,
    async_graphql :: SimpleObject,
    seaography_derive :: Filter,
)]
#[sea_orm(table_name = "playlist_track")]
#[graphql(complex)]
#[graphql(name = "PlaylistTrack")]
pub struct Model {
    pub playlist_id: i32,
    pub track_id: i32,
}
#[derive(Copy, Clone, Debug, EnumIter, DeriveColumn)]
pub enum Column {
    #[sea_orm(column_name = "PlaylistId")]
    PlaylistId,
    #[sea_orm(column_name = "TrackId")]
    TrackId,
}
#[derive(Copy, Clone, Debug, EnumIter, DerivePrimaryKey)]
pub enum PrimaryKey {
    PlaylistId,
    TrackId,
}
impl PrimaryKeyTrait for PrimaryKey {
    type ValueType = (i32, i32);
    fn auto_increment() -> bool {
        false
    }
}
#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Tracks,
    Playlists,
}
impl ColumnTrait for Column {
    type EntityName = Entity;
    fn def(&self) -> ColumnDef {
        match self {
            Self::PlaylistId => ColumnType::Integer.def(),
            Self::TrackId => ColumnType::Integer.def(),
        }
    }
}
#[seaography_derive::relation]
impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Self::Tracks => Entity::belongs_to(super::tracks::Entity)
                .from(Column::TrackId)
                .to(super::tracks::Column::TrackId)
                .into(),
            Self::Playlists => Entity::belongs_to(super::playlists::Entity)
                .from(Column::PlaylistId)
                .to(super::playlists::Column::PlaylistId)
                .into(),
        }
    }
}
impl Related<super::tracks::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Tracks.def()
    }
}
impl Related<super::playlists::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Playlists.def()
    }
}
impl ActiveModelBehavior for ActiveModel {}
