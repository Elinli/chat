use sqlx::PgPool;

use crate::{AppError, AppState};

use super::{ChatUser, Workspace};

impl AppState {
    pub async fn create_workspace(&self, name: &str, user_id: u64) -> Result<Workspace, AppError> {
        let ws = sqlx::query_as(
            r#"
            INSERT INTO workspaces (name, owner_id)
            VALUES ($1, $2)
            RETURNING id, name, owner_id, created_at
            "#,
        )
        .bind(name)
        .bind(user_id as i64)
        .fetch_one(&self.pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_workspace_by_id(&self, id: i64) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"SELECT id, name, owner_id, created_at FROM workspaces WHERE id = $1"#,
        )
        .bind(id as i64)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_by_workspace_name(&self, name: &str) -> Result<Option<Workspace>, AppError> {
        let ws = sqlx::query_as(
            r#"SELECT id, name, owner_id, created_at FROM workspaces WHERE name = $1"#,
        )
        .bind(name)
        .fetch_optional(&self.pool)
        .await?;
        Ok(ws)
    }

    pub async fn find_users_by_ws_id(id: u64, pool: &PgPool) -> Result<Vec<ChatUser>, AppError> {
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

impl Workspace {
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
}
#[cfg(test)]
mod tests {

    use crate::models::CreateUser;

    use super::*;
    use anyhow::{Ok, Result};

    #[tokio::test]
    async fn workspace_create_and_set_owner_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("dev", 0).await?;
        let input = CreateUser::new("eEli Shi", "Eelixy@qq.com", "pwd25", &ws.name);
        let user = state.create_user(&input).await?;

        assert_eq!(ws.name, "dev");
        assert_eq!(ws.id, user.ws_id);

        Ok(())
    }

    #[tokio::test]
    async fn workspace_find_by_name_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;

        let _ws = state.create_workspace("OKM", 0).await.unwrap();

        let ws = state.find_by_workspace_name("OKM").await?;

        assert_eq!(ws.unwrap().name, "OKM");

        let _ws = state.create_workspace("productION", 0).await?;

        let ws = state.find_by_workspace_name("productION").await?;
        assert_eq!(ws.unwrap().id, 2);
        Ok(())
    }

    #[tokio::test]
    async fn workspace_find_by_id_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let _ws = state.create_workspace("test", 0).await?;
        let ws = state.find_workspace_by_id(1).await?;
        assert_eq!(ws.unwrap().name, "test");
        Ok(())
    }

    #[tokio::test]
    async fn update_owner_should_work() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("demo", 0).await?;
        let user = state
            .create_user(&CreateUser::new("Eli X Shi", "elixyY@qq.com", "pwd25", "demo"))
            .await?;
        let ws = ws.update_owner(user.id as _, &state.pool).await?;
        assert_eq!(ws.owner_id, 1);
        Ok(())
    }

    #[tokio::test]
    async fn fetch_all_chat_users_by_workspace() -> Result<()> {
        let (_tdb, state) = AppState::new_for_test().await?;
        let ws = state.create_workspace("test", 0).await?;
        let owner_user = state
            .create_user(&CreateUser::new("Eli Shi", "elixy@qq.com", "pwd25", "test"))
            .await?;
        let ws = ws.update_owner(owner_user.id as _, &state.pool).await?;
        assert_eq!(ws.owner_id, 1);
        let mem_user1 = state
            .create_user(&CreateUser::new("Yong Shi", "yong@qq.com", "pwd25", "test"))
            .await?;
        let mem_user2 = state
            .create_user(&CreateUser::new("jun Shi", "jun@qq.com", "pwd25", "test"))
            .await?;

        let users = state.fetch_chat_users_by_ws_id(ws.id as _).await.unwrap();

        assert_eq!(users.len(), 3);
        assert_eq!(ws.owner_id, 1);
        assert_eq!(2, mem_user1.id);
        assert_eq!(3, mem_user2.id);
        Ok(())
    }
}
