use sqlx::PgPool;

use crate::AppError;

use super::{ChatUser, Workspace};

impl Workspace {
    pub async fn create(name: &str, user_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let ws = sqlx::query_as(
            r#"
            INSERT INTO workspaces (name, owner_id)
            VALUES ($1, $2)
            RETURNING id, name, owner_id, created_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(pool)
        .await?;
        Ok(ws)
    }

    pub async fn update_owner(&self, owner_id: u64, pool: &PgPool) -> Result<Self, AppError> {
        let ws = sqlx::query_as(
            r#"
        UPDATE workspaces
        SET owner_id = $1
        WHERE id = $2 and (SELECT ws_id FROM users WHERE id = $1) = $2
        RETURNING id, name, owner_id, created_at
        "#,
        )
        .bind(owner_id as i64)
        .bind(self.id)
        .fetch_one(pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_by_id(id: i64, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let ws = sqlx::query_as(
            r#"SELECT id, name, owner_id, created_at FROM workspaces WHERE id = $1"#,
        )
        .bind(id as i64)
        .fetch_optional(pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_by_name(name: &str, pool: &sqlx::PgPool) -> Result<Option<Self>, AppError> {
        let ws = sqlx::query_as(
            r#"SELECT id, name, owner_id, created_at FROM workspaces WHERE name = $1"#,
        )
        .bind(name)
        .fetch_optional(pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_all_chat_users(id: u64, pool: &PgPool) -> Result<Vec<ChatUser>, AppError> {
        let users = sqlx::query_as(
            r#"
        SELECT id, fullname, email
        FROM users
        WHERE ws_id = $1 order by id
        "#,
        )
        .bind(id as i64)
        .fetch_all(pool)
        .await?;

        Ok(users)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{models::CreateUser, User};

    use super::*;
    use anyhow::{Ok, Result};
    use sqlx_db_tester::TestPg;

    #[tokio::test]
    async fn workspace_create_and_set_owner_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:admin@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("development", 0, &pool).await?;
        let input = CreateUser::new("Eli Shi", "elixy@qq.com", "pwd25", &ws.name);
        let user = User::create(&input, &pool).await?;

        assert_eq!(ws.name, "development");
        assert_eq!(ws.id, user.ws_id);

        // let ws = ws.update_owner(user.id as _, &pool).await?;
        // assert_eq!(ws.owner_id, user.id);
        Ok(())
    }

    #[tokio::test]
    async fn workspace_find_by_name_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:admin@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let _ws = Workspace::create("test", 0, &pool).await.unwrap();
        let ws = Workspace::find_by_name("test", &pool).await?;
        assert_eq!(ws.unwrap().name, "test");
        let _ws = Workspace::create("product", 0, &pool).await?;
        let ws = Workspace::find_by_name("product", &pool).await?;
        assert_eq!(ws.unwrap().id, 2);
        Ok(())
    }

    #[tokio::test]
    async fn workspace_find_by_id_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:admin@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let _ws = Workspace::create("test", 0, &pool).await?;
        let ws = Workspace::find_by_id(1, &pool).await?;
        assert_eq!(ws.unwrap().name, "test");
        Ok(())
    }

    #[tokio::test]
    async fn update_owner_should_work() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:admin@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("test", 0, &pool).await?;
        let user = User::create(
            &CreateUser::new("Eli Shi", "elixy@qq.com", "pwd25", "test"),
            &pool,
        )
        .await?;
        let ws = ws.update_owner(user.id as _, &pool).await?;
        assert_eq!(ws.owner_id, 1);
        Ok(())
    }

    #[tokio::test]
    async fn fetch_all_chat_users_by_workspace() -> Result<()> {
        let tdb = TestPg::new(
            "postgres://postgres:admin@localhost:5432".to_string(),
            Path::new("../migrations"),
        );
        let pool = tdb.get_pool().await;
        let ws = Workspace::create("test", 0, &pool).await?;
        let owner_user = User::create(
            &CreateUser::new("Eli Shi", "elixy@qq.com", "pwd25", "test"),
            &pool,
        )
        .await?;
        let ws = ws.update_owner(owner_user.id as _, &pool).await?;
        assert_eq!(ws.owner_id, 1);
        let mem_user1 = User::create(
            &CreateUser::new("Yong Shi", "yong@qq.com", "pwd25", "test"),
            &pool,
        )
        .await?;
        let mem_user2 = User::create(
            &CreateUser::new("jun Shi", "jun@qq.com", "pwd25", "test"),
            &pool,
        )
        .await?;

        let users = Workspace::find_all_chat_users(ws.id as _, &pool)
            .await
            .unwrap();

        assert_eq!(users.len(), 3);
        assert_eq!(ws.owner_id, 1);
        assert_eq!(2, mem_user1.id);
        assert_eq!(3, mem_user2.id);
        Ok(())
    }
}
