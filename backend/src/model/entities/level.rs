//! `SeaORM` Entity. Generated by sea-orm-codegen 0.12.15

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "level")]
pub struct Model {
    #[sea_orm(column_name = "Level_id", primary_key)]
    pub level_id: i32,
    #[sea_orm(column_name = "Description", column_type = "Text", nullable)]
    pub description: Option<String>,
    #[sea_orm(column_name = "Attempt_times")]
    pub attempt_times: Option<i32>,
    #[sea_orm(column_name = "Pass_times")]
    pub pass_times: Option<i32>,
    #[sea_orm(column_name = "Indicator")]
    pub indicator: Option<i32>,
    #[sea_orm(column_name = "Initial_indicator")]
    pub initial_indicator: Option<i32>,
    #[sea_orm(column_name = "Threshold")]
    pub threshold: Option<i32>,
    #[sea_orm(column_name = "Content_pr", column_type = "Text", nullable)]
    pub content_pr: Option<String>,
    #[sea_orm(column_name = "Quantity_pr", column_type = "Text", nullable)]
    pub quantity_pr: Option<String>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

impl ActiveModelBehavior for ActiveModel {}
