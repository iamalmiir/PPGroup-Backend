use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, DbErr, EntityTrait, ModelTrait,
    PaginatorTrait, QueryFilter, Set,
};

use crate::database::realtors;

#[derive(Debug, serde::Deserialize)]
pub struct Realtor {
    full_name: String,
    email: String,
    photo: Option<String>,
    phone: String,
    is_mvp: Option<bool>,
    description: Option<String>,
}

impl Realtor {
    pub async fn add_realtor(
        db: &DatabaseConnection,
        realtor: Realtor,
    ) -> Result<realtors::Model, DbErr> {
        let new_realtor = realtors::ActiveModel {
            id: Set(nanoid::nanoid!()),
            full_name: Set(realtor.full_name),
            email: Set(realtor.email),
            photo: Set(realtor.photo),
            phone: Set(realtor.phone),
            is_mvp: Set(realtor.is_mvp),
            ..Default::default()
        };

        match new_realtor.insert(db).await {
            Ok(realtor) => Ok(realtor),
            Err(err) => Err(err),
        }
    }

    pub async fn fetch_all(
        db: &DatabaseConnection,
        page: u64,
        page_size: u64,
    ) -> Result<Vec<realtors::Model>, DbErr> {
        match realtors::Entity::find()
            .paginate(db, page_size)
            .fetch_page(page - 1)
            .await
        {
            Ok(realtors) => Ok(realtors),

            Err(err) => Err(err),
        }
    }

    pub async fn delete_realtor_by_email(
        db: &DatabaseConnection,
        email: String,
    ) -> Result<String, DbErr> {
        let realtor_to_delete = realtors::Entity::find()
            .filter(realtors::Column::Email.eq(email))
            .one(db)
            .await?;

        if let Some(realtor) = realtor_to_delete {
            realtor.delete(db).await?;

            Ok("OK".to_owned())
        } else {
            Err(DbErr::Custom("Realtor not found".to_owned()))
        }
    }
}
