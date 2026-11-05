use crate::database::DbPool;
use crate::error::{AppError, Result};
use crate::models::bankroll::BankrollAccount;
use uuid::Uuid;

pub struct BankrollManager;

impl BankrollManager {
    pub async fn get_primary_account(pool: &DbPool) -> Result<BankrollAccount> {
        let account = sqlx::query_as!(
            BankrollAccount,
            r#"
            SELECT 
                id, name, currency, initial_balance, current_balance, 
                peak_balance, total_staked, total_profit, max_drawdown_pct, 
                created_at, updated_at
            FROM bankroll_accounts
            ORDER BY created_at ASC
            LIMIT 1
            "#
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        Ok(account)
    }

    pub async fn get_pending_exposure(pool: &DbPool) -> Result<f64> {
        let pending = sqlx::query_scalar!(
            r#"
            SELECT COALESCE(SUM(stake_amount), 0.0) as "sum!"
            FROM paper_bets
            WHERE status = 'PENDING'
            "#
        )
        .fetch_one(pool)
        .await?;

        Ok(pending)
    }

    pub async fn record_bet_placed(
        pool: &DbPool,
        account_id: Uuid,
        bet_id: Uuid,
        stake: f64,
    ) -> Result<()> {
        let mut tx = pool.begin().await?;

        // 1. Debit account balance & add to total_staked
        let account = sqlx::query!(
            r#"
            UPDATE bankroll_accounts
            SET current_balance = current_balance - $1,
                total_staked = total_staked + $1,
                updated_at = NOW()
            WHERE id = $2
            RETURNING current_balance
            "#,
            stake,
            account_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // 2. Insert transaction
        sqlx::query!(
            r#"
            INSERT INTO bankroll_transactions (account_id, paper_bet_id, transaction_type, amount, balance_after, description)
            VALUES ($1, $2, 'BET_PLACED', $3, $4, 'Placed paper bet')
            "#,
            account_id,
            bet_id,
            -stake,
            account.current_balance
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }

    pub async fn record_bet_settled(
        pool: &DbPool,
        account_id: Uuid,
        bet_id: Uuid,
        status: &str, // WON, LOST, VOID
        stake: f64,
        payout: f64,
        profit_loss: f64,
    ) -> Result<()> {
        let mut tx = pool.begin().await?;

        // 1. Credit returns if any (payout = stake + profit for WON, stake for VOID, 0 for LOST)
        let credit_amount = match status {
            "WON" => payout,
            "VOID" => stake,
            _ => 0.0,
        };

        let updated = sqlx::query!(
            r#"
            UPDATE bankroll_accounts
            SET current_balance = current_balance + $1,
                total_profit = total_profit + $2,
                peak_balance = GREATEST(peak_balance, current_balance + $1),
                updated_at = NOW()
            WHERE id = $3
            RETURNING current_balance, peak_balance
            "#,
            credit_amount,
            profit_loss,
            account_id
        )
        .fetch_one(&mut *tx)
        .await?;

        // Calculate drawdown percentage
        let drawdown_pct = if updated.peak_balance > 0.0 {
            ((updated.peak_balance - updated.current_balance) / updated.peak_balance).max(0.0)
        } else {
            0.0
        };

        sqlx::query!(
            r#"
            UPDATE bankroll_accounts
            SET max_drawdown_pct = GREATEST(max_drawdown_pct, $1)
            WHERE id = $2
            "#,
            drawdown_pct,
            account_id
        )
        .execute(&mut *tx)
        .await?;

        // Record transaction
        let tx_type = match status {
            "WON" => "BET_WON",
            "LOST" => "BET_LOST",
            "VOID" => "BET_VOID",
            _ => "ADJUSTMENT",
        };

        sqlx::query!(
            r#"
            INSERT INTO bankroll_transactions (account_id, paper_bet_id, transaction_type, amount, balance_after, description)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            account_id,
            bet_id,
            tx_type,
            profit_loss,
            updated.current_balance,
            format!("Settled paper bet as {}", status)
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(())
    }
}

