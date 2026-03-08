#[cfg(test)]
#[allow(clippy::module_inception)]
mod tests {
    use crate::models::*;
    use crate::repository::*;
    
    use async_trait::async_trait;
    use chrono::{NaiveDate, Utc};
    use sqlx::SqlitePool;
    use uuid::Uuid;
    use std::sync::Mutex;

    #[allow(dead_code)]
    struct MockOEERepository {
        metrics: Mutex<Vec<OEEMetric>>,
    }

    #[async_trait]
    impl OEERepository for MockOEERepository {
        async fn find_by_equipment(&self, _: &SqlitePool, eq_id: Uuid, start: NaiveDate, end: NaiveDate) -> erp_core::Result<Vec<OEEMetric>> {
            let metrics = self.metrics.lock().unwrap();
            Ok(metrics.iter()
                .filter(|m| m.equipment_id == eq_id && m.date >= start && m.date <= end)
                .cloned()
                .collect())
        }
        async fn create(&self, _: &SqlitePool, metric: OEEMetric) -> erp_core::Result<OEEMetric> {
            self.metrics.lock().unwrap().push(metric.clone());
            Ok(metric)
        }
    }

    #[allow(dead_code)]
    struct MockMachineStateRepository {
        logs: Mutex<Vec<MachineStateLog>>,
    }

    #[async_trait]
    impl MachineStateRepository for MockMachineStateRepository {
        async fn find_by_equipment(&self, _: &SqlitePool, eq_id: Uuid, limit: i64) -> erp_core::Result<Vec<MachineStateLog>> {
            let logs = self.logs.lock().unwrap();
            let filtered: Vec<_> = logs.iter().filter(|l| l.equipment_id == eq_id).cloned().collect();
            Ok(filtered.into_iter().take(limit as usize).collect())
        }
        async fn create(&self, _: &SqlitePool, log: MachineStateLog) -> erp_core::Result<MachineStateLog> {
            self.logs.lock().unwrap().push(log.clone());
            Ok(log)
        }
        async fn update_end_time(&self, _: &SqlitePool, id: Uuid, ended_at: chrono::DateTime<Utc>, duration: i64) -> erp_core::Result<()> {
            let mut logs = self.logs.lock().unwrap();
            if let Some(log) = logs.iter_mut().find(|l| l.id == id) {
                log.ended_at = Some(ended_at);
                log.duration_seconds = Some(duration);
            }
            Ok(())
        }
    }

    #[test]
    fn test_oee_calculation_logic() {
        // Availability = Run Time / Planned Production Time
        let planned_time = 480.0; // 8 hours
        let downtime = 60.0; // 1 hour
        let availability = (planned_time - downtime) / planned_time;
        assert_eq!(availability, 0.875);

        // Performance = (Total Count * Ideal Cycle Time) / Run Time
        let run_time = 420.0 * 60.0; // 7 hours in seconds
        let total_count = 400;
        let ideal_cycle_time = 60.0; // 1 minute
        let performance = (total_count as f64 * ideal_cycle_time) / run_time;
        // 400 * 60 / 25200 = 24000 / 25200 = 0.95238
        assert!((performance - 0.95238).abs() < 0.00001);

        // Quality = Good Count / Total Count
        let good_count = 380;
        let quality = good_count as f64 / total_count as f64;
        assert_eq!(quality, 0.95);

        let oee = availability * performance * quality;
        // 0.875 * 0.95238 * 0.95 = 0.79166
        assert!((oee - 0.79166).abs() < 0.00001);
    }
}
