use crate::database::DbPool;
use crate::error::{AppError, Result};
use crate::models::bet::{PaperBet, PlaceBetRequest};
use crate::risk::bankroll::BankrollManager;
use crate::risk::limits::{RiskLimits, RiskValidator};
use tracing::info;
use uuid::Uuid;

pub struct PaperBettingEngine;

impl PaperBettingEngine {
    pub async fn execute_order(
        pool: &DbPool,
        req: PlaceBetRequest,
        limits: &RiskLimits,
    ) -> Result<PaperBet> {
        let account = BankrollManager::get_primary_account(pool).await?;
        let pending_exposure = BankrollManager::get_pending_exposure(pool).await?;

        let current_drawdown = if account.peak_balance > 0.0 {
            (account.peak_balance - account.current_balance) / account.peak_balance
        } else {
            0.0
        };

        // 1. Validate risk management boundaries
        RiskValidator::validate_order(
            limits,
            account.current_balance,
            pending_exposure,
            req.stake,
            req.odds,
            current_drawdown,
        )?;

        // 2. Insert paper bet record
        let potential_return = req.stake * req.odds;
        let strategy_tag = req.strategy_tag.unwrap_or_else(|| "kelly_standard_v1".to_string());

        let bet = sqlx::query_as!(
            PaperBet,
            r#"
            INSERT INTO paper_bets (
                prediction_id, event_id, market_id, outcome_name, 
                odds_placed, stake_amount, potential_return, status, strategy_tag
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, 'PENDING', $8)
            RETURNING 
                id, prediction_id, event_id, market_id, outcome_name,
                odds_placed, stake_amount, potential_return, status,
                profit_loss, closing_odds, closing_line_value, strategy_tag,
                placed_at, settled_at
            "#,
            req.prediction_id,
            req.event_id,
            req.market_id,
            req.outcome_name,
            req.odds,
            req.stake,
            potential_return,
            strategy_tag
        )
        .fetch_one(pool)
        .await
        .map_err(AppError::Database)?;

        // 3. Update Bankroll balance & write ledger entry
        BankrollManager::record_bet_placed(pool, account.id, bet.id, req.stake).await?;

        info!(
            "Placed virtual paper bet {} on outcome '{}' @ {:.2} with stake €{:.2}",
            bet.id, bet.outcome_name, bet.odds_placed, bet.stake_amount
        );

        Ok(bet)
    }
}

