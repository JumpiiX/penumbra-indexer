use sqlx::{Pool, Postgres, Result as SqlxResult};
use chrono::{DateTime, Utc};
use crate::models::stats::{BlockTimingInfo, ChartPoint};

pub struct StatsQueries;

impl StatsQueries {
    pub async fn get_latest_block_timing(pool: &Pool<Postgres>) -> SqlxResult<BlockTimingInfo> {
        let record = sqlx::query_as::<_, (i64, DateTime<Utc>)>(
            "SELECT height, time FROM blocks ORDER BY height DESC LIMIT 1"
        )
            .fetch_one(pool)
            .await?;

        Ok(BlockTimingInfo {
            height: record.0,
            timestamp: record.1,
        })
    }

    pub async fn get_previous_block_timing(
        pool: &Pool<Postgres>,
        height: i64,
    ) -> SqlxResult<BlockTimingInfo> {
        let record = sqlx::query_as::<_, (i64, DateTime<Utc>)>(
            "SELECT height, time FROM blocks WHERE height = $1"
        )
            .bind(height - 1)
            .fetch_one(pool)
            .await?;

        Ok(BlockTimingInfo {
            height: record.0,
            timestamp: record.1,
        })
    }

    pub async fn get_total_transactions(pool: &Pool<Postgres>) -> SqlxResult<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(tx_count), 0) FROM blocks"
        )
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_today_transactions(pool: &Pool<Postgres>) -> SqlxResult<i64> {
        let result = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(SUM(tx_count), 0) FROM blocks WHERE DATE(time) = CURRENT_DATE"
        )
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_transaction_history(pool: &Pool<Postgres>) -> SqlxResult<Vec<ChartPoint>> {
        // Get transaction counts for the last few days
        let records = sqlx::query_as::<_, (String, i64)>(
            "SELECT TO_CHAR(DATE(time), 'DD') as date, COALESCE(SUM(tx_count), 0) as value
             FROM blocks
             WHERE time >= CURRENT_DATE - INTERVAL '20 days'
             GROUP BY DATE(time)
             ORDER BY DATE(time)
             LIMIT 20"
        )
            .fetch_all(pool)
            .await?;

        // Create chart points
        Ok(records
            .into_iter()
            .map(|(date, value)| ChartPoint {
                date,
                value,
            })
            .collect())
    }

    pub async fn get_total_burn(pool: &Pool<Postgres>) -> SqlxResult<f64> {
        // Calculate total burn amount
        let result = sqlx::query_scalar::<_, f64>(
            "SELECT COALESCE(SUM(burn_amount), 0) FROM blocks"
        )
            .fetch_one(pool)
            .await?;

        Ok(result)
    }

    pub async fn get_burn_history(pool: &Pool<Postgres>) -> SqlxResult<Vec<ChartPoint>> {
        // Get burn amounts for display dates
        let records = sqlx::query_as::<_, (String, f64)>(
            "SELECT
                CASE
                    WHEN DATE(time) = CURRENT_DATE THEN 'Today'
                    ELSE TO_CHAR(DATE(time), 'Mon DD')
                END as date,
                COALESCE(SUM(burn_amount), 0) as value
             FROM blocks
             WHERE time >= CURRENT_DATE - INTERVAL '30 days'
             GROUP BY date, DATE(time)
             ORDER BY DATE(time)
             LIMIT 3"
        )
            .fetch_all(pool)
            .await?;

        // Format for chart display
        Ok(records
            .into_iter()
            .map(|(date, value)| ChartPoint {
                date,
                value: value as i64, // Convert to integer for display
            })
            .collect())
    }
}