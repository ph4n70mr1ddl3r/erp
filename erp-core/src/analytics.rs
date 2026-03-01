use sqlx::SqlitePool;
use uuid::Uuid;
use chrono::Utc;
use crate::models::*;
use crate::{Error, Result, Pagination, Paginated};

pub struct DashboardService;

impl DashboardService {
    pub async fn create_dashboard(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        dashboard_type: DashboardType,
        is_default: bool,
        layout: Option<&str>,
        created_by: Option<Uuid>,
    ) -> Result<Dashboard> {
        let now = Utc::now();
        let dashboard = Dashboard {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            dashboard_type,
            is_default,
            layout: layout.map(|s| s.to_string()),
            created_by,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO dashboards (id, name, description, dashboard_type, is_default, layout, created_by, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(dashboard.id.to_string())
        .bind(&dashboard.name)
        .bind(&dashboard.description)
        .bind(format!("{:?}", dashboard.dashboard_type))
        .bind(dashboard.is_default as i64)
        .bind(&dashboard.layout)
        .bind(dashboard.created_by.map(|id| id.to_string()))
        .bind(dashboard.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(dashboard)
    }

    pub async fn get_dashboard(pool: &SqlitePool, id: Uuid) -> Result<Dashboard> {
        let row = sqlx::query_as::<_, DashboardRow>(
            "SELECT id, name, description, dashboard_type, is_default, layout, created_by, created_at
             FROM dashboards WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("Dashboard", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn list_dashboards(pool: &SqlitePool, pagination: Pagination) -> Result<Paginated<Dashboard>> {
        let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM dashboards")
            .fetch_one(pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let rows = sqlx::query_as::<_, DashboardRow>(
            "SELECT id, name, description, dashboard_type, is_default, layout, created_by, created_at
             FROM dashboards ORDER BY created_at DESC LIMIT ? OFFSET ?"
        )
        .bind(pagination.limit() as i64)
        .bind(pagination.offset() as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(Paginated::new(
            rows.into_iter().map(|r| r.into()).collect(),
            count.0 as u64,
            pagination,
        ))
    }

    pub async fn add_widget(
        pool: &SqlitePool,
        dashboard_id: Uuid,
        widget_type: WidgetType,
        title: &str,
        data_source: &str,
        query_text: Option<&str>,
        refresh_interval: i32,
        position_x: i32,
        position_y: i32,
        width: i32,
        height: i32,
        config: Option<&str>,
    ) -> Result<DashboardWidget> {
        let widget = DashboardWidget {
            id: Uuid::new_v4(),
            dashboard_id,
            widget_type,
            title: title.to_string(),
            data_source: data_source.to_string(),
            query_text: query_text.map(|s| s.to_string()),
            refresh_interval,
            position_x,
            position_y,
            width,
            height,
            config: config.map(|s| s.to_string()),
        };

        sqlx::query(
            "INSERT INTO dashboard_widgets (id, dashboard_id, widget_type, title, data_source, query_text, refresh_interval, position_x, position_y, width, height, config)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(widget.id.to_string())
        .bind(widget.dashboard_id.to_string())
        .bind(format!("{:?}", widget.widget_type))
        .bind(&widget.title)
        .bind(&widget.data_source)
        .bind(&widget.query_text)
        .bind(widget.refresh_interval as i64)
        .bind(widget.position_x as i64)
        .bind(widget.position_y as i64)
        .bind(widget.width as i64)
        .bind(widget.height as i64)
        .bind(&widget.config)
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(widget)
    }

    pub async fn update_widget_config(
        pool: &SqlitePool,
        widget_id: Uuid,
        config: &str,
    ) -> Result<DashboardWidget> {
        sqlx::query("UPDATE dashboard_widgets SET config = ? WHERE id = ?")
            .bind(config)
            .bind(widget_id.to_string())
            .execute(pool)
            .await
            .map_err(|e| Error::Database(e))?;

        let row = sqlx::query_as::<_, DashboardWidgetRow>(
            "SELECT id, dashboard_id, widget_type, title, data_source, query_text, refresh_interval, position_x, position_y, width, height, config
             FROM dashboard_widgets WHERE id = ?"
        )
        .bind(widget_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("DashboardWidget", &widget_id.to_string()))?;

        Ok(row.into())
    }

    pub async fn refresh_widget_data(pool: &SqlitePool, widget_id: Uuid) -> Result<serde_json::Value> {
        let row = sqlx::query_as::<_, DashboardWidgetRow>(
            "SELECT id, dashboard_id, widget_type, title, data_source, query_text, refresh_interval, position_x, position_y, width, height, config
             FROM dashboard_widgets WHERE id = ?"
        )
        .bind(widget_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("DashboardWidget", &widget_id.to_string()))?;

        let widget: DashboardWidget = row.into();
        
        if let Some(query) = &widget.query_text {
            let results: Vec<serde_json::Value> = sqlx::query(query)
                .fetch_all(pool)
                .await
                .map_err(|e| Error::Database(e.into()))?
                .into_iter()
                .map(|_| serde_json::json!({}))
                .collect();
            Ok(serde_json::json!({ "data": results, "refreshed_at": Utc::now().to_rfc3339() }))
        } else {
            Ok(serde_json::json!({ "data_source": widget.data_source, "refreshed_at": Utc::now().to_rfc3339() }))
        }
    }
}

pub struct KPIService;

impl KPIService {
    pub async fn create_kpi(
        pool: &SqlitePool,
        kpi_code: &str,
        name: &str,
        description: Option<&str>,
        category: KPICategory,
        unit: Option<&str>,
        target_value: Option<f64>,
        warning_threshold: Option<f64>,
        critical_threshold: Option<f64>,
        calculation_formula: Option<&str>,
        data_source: Option<&str>,
        refresh_frequency: RefreshFrequency,
        owner: Option<Uuid>,
    ) -> Result<KPIDefinition> {
        let kpi = KPIDefinition {
            id: Uuid::new_v4(),
            kpi_code: kpi_code.to_string(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            category,
            unit: unit.map(|s| s.to_string()),
            target_value,
            warning_threshold,
            critical_threshold,
            calculation_formula: calculation_formula.map(|s| s.to_string()),
            data_source: data_source.map(|s| s.to_string()),
            refresh_frequency,
            owner,
            status: Status::Active,
        };

        sqlx::query(
            "INSERT INTO kpi_definitions (id, kpi_code, name, description, category, unit, target_value, warning_threshold, critical_threshold, calculation_formula, data_source, refresh_frequency, owner, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?"
        )
        .bind(kpi.id.to_string())
        .bind(&kpi.kpi_code)
        .bind(&kpi.name)
        .bind(&kpi.description)
        .bind(format!("{:?}", kpi.category))
        .bind(&kpi.unit)
        .bind(kpi.target_value)
        .bind(kpi.warning_threshold)
        .bind(kpi.critical_threshold)
        .bind(&kpi.calculation_formula)
        .bind(&kpi.data_source)
        .bind(format!("{:?}", kpi.refresh_frequency))
        .bind(kpi.owner.map(|id| id.to_string()))
        .bind(format!("{:?}", kpi.status))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(kpi)
    }

    pub async fn get_kpi(pool: &SqlitePool, id: Uuid) -> Result<KPIDefinition> {
        let row = sqlx::query_as::<_, KPIDefinitionRow>(
            "SELECT id, kpi_code, name, description, category, unit, target_value, warning_threshold, critical_threshold, calculation_formula, data_source, refresh_frequency, owner, status
             FROM kpi_definitions WHERE id = ?"
        )
        .bind(id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("KPIDefinition", &id.to_string()))?;

        Ok(row.into())
    }

    pub async fn list_kpis(pool: &SqlitePool, category: Option<KPICategory>) -> Result<Vec<KPIDefinition>> {
        let rows = if let Some(cat) = category {
            sqlx::query_as::<_, KPIDefinitionRow>(
                "SELECT id, kpi_code, name, description, category, unit, target_value, warning_threshold, critical_threshold, calculation_formula, data_source, refresh_frequency, owner, status
                 FROM kpi_definitions WHERE category = ? AND status = 'Active' ORDER BY name"
            )
            .bind(format!("{:?}", cat))
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        } else {
            sqlx::query_as::<_, KPIDefinitionRow>(
                "SELECT id, kpi_code, name, description, category, unit, target_value, warning_threshold, critical_threshold, calculation_formula, data_source, refresh_frequency, owner, status
                 FROM kpi_definitions WHERE status = 'Active' ORDER BY name"
            )
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn record_value(
        pool: &SqlitePool,
        kpi_id: Uuid,
        period: &str,
        value: f64,
        target: Option<f64>,
    ) -> Result<KPIValue> {
        let now = Utc::now();
        let variance = target.map(|t| value - t);
        let variance_percent = target.and_then(|t| if t != 0.0 { Some((value - t) / t * 100.0) } else { None });

        let kpi_value = KPIValue {
            id: Uuid::new_v4(),
            kpi_id,
            period: period.to_string(),
            value,
            target,
            variance,
            variance_percent,
            trend: None,
            calculated_at: now,
        };

        sqlx::query(
            "INSERT INTO kpi_values (id, kpi_id, period, value, target, variance, variance_percent, trend, calculated_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(kpi_value.id.to_string())
        .bind(kpi_value.kpi_id.to_string())
        .bind(&kpi_value.period)
        .bind(kpi_value.value)
        .bind(kpi_value.target)
        .bind(kpi_value.variance)
        .bind(kpi_value.variance_percent)
        .bind(kpi_value.trend.as_ref().map(|t| format!("{:?}", t)))
        .bind(kpi_value.calculated_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(kpi_value)
    }

    pub async fn calculate_kpi(pool: &SqlitePool, kpi_id: Uuid, period: &str) -> Result<KPIValue> {
        let kpi = Self::get_kpi(pool, kpi_id).await?;

        let value: f64 = if let Some(formula) = &kpi.calculation_formula {
            let result: Option<(f64,)> = sqlx::query_as(formula)
                .fetch_optional(pool)
                .await
                .map_err(|e| Error::Database(e))?;
            result.map(|r| r.0).unwrap_or(0.0)
        } else {
            0.0
        };

        Self::record_value(pool, kpi_id, period, value, kpi.target_value).await
    }

    pub async fn get_kpi_trend(pool: &SqlitePool, kpi_id: Uuid, periods: i32) -> Result<Vec<KPIValue>> {
        let rows = sqlx::query_as::<_, KPIValueRow>(
            "SELECT id, kpi_id, period, value, target, variance, variance_percent, trend, calculated_at
             FROM kpi_values WHERE kpi_id = ? ORDER BY period DESC LIMIT ?"
        )
        .bind(kpi_id.to_string())
        .bind(periods as i64)
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

pub struct AlertService;

impl AlertService {
    pub async fn create_rule(
        pool: &SqlitePool,
        name: &str,
        description: Option<&str>,
        entity_type: &str,
        condition_field: &str,
        operator: &str,
        threshold_value: &str,
        severity: Severity,
        notification_channels: Option<&str>,
    ) -> Result<AlertRule> {
        let now = Utc::now();
        let rule = AlertRule {
            id: Uuid::new_v4(),
            name: name.to_string(),
            description: description.map(|s| s.to_string()),
            entity_type: entity_type.to_string(),
            condition_field: condition_field.to_string(),
            operator: operator.to_string(),
            threshold_value: threshold_value.to_string(),
            severity,
            notification_channels: notification_channels.map(|s| s.to_string()),
            status: Status::Active,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO alert_rules (id, name, description, entity_type, condition_field, operator, threshold_value, severity, notification_channels, status, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(rule.id.to_string())
        .bind(&rule.name)
        .bind(&rule.description)
        .bind(&rule.entity_type)
        .bind(&rule.condition_field)
        .bind(&rule.operator)
        .bind(&rule.threshold_value)
        .bind(format!("{:?}", rule.severity))
        .bind(&rule.notification_channels)
        .bind(format!("{:?}", rule.status))
        .bind(rule.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rule)
    }

    pub async fn get_rules(pool: &SqlitePool, entity_type: Option<&str>) -> Result<Vec<AlertRule>> {
        let rows = if let Some(et) = entity_type {
            sqlx::query_as::<_, AlertRuleRow>(
                "SELECT id, name, description, entity_type, condition_field, operator, threshold_value, severity, notification_channels, status, created_at
                 FROM alert_rules WHERE entity_type = ? AND status = 'Active' ORDER BY name"
            )
            .bind(et)
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        } else {
            sqlx::query_as::<_, AlertRuleRow>(
                "SELECT id, name, description, entity_type, condition_field, operator, threshold_value, severity, notification_channels, status, created_at
                 FROM alert_rules WHERE status = 'Active' ORDER BY name"
            )
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn check_rules(pool: &SqlitePool, entity_type: &str, entity_id: Uuid) -> Result<Vec<Alert>> {
        let rules = Self::get_rules(pool, Some(entity_type)).await?;
        let mut alerts = Vec::new();

        for rule in rules {
            let query = format!(
                "SELECT {} FROM {} WHERE id = ?",
                rule.condition_field, rule.entity_type
            );
            
            let value: Option<(String,)> = sqlx::query_as(&query)
                .bind(entity_id.to_string())
                .fetch_optional(pool)
                .await
                .map_err(|e| Error::Database(e))?;

            if let Some((field_value,)) = value {
                let should_alert = match rule.operator.as_str() {
                    ">" => field_value.parse::<f64>().unwrap_or(0.0) > rule.threshold_value.parse().unwrap_or(0.0),
                    "<" => field_value.parse::<f64>().unwrap_or(0.0) < rule.threshold_value.parse().unwrap_or(0.0),
                    ">=" => field_value.parse::<f64>().unwrap_or(0.0) >= rule.threshold_value.parse().unwrap_or(0.0),
                    "<=" => field_value.parse::<f64>().unwrap_or(0.0) <= rule.threshold_value.parse().unwrap_or(0.0),
                    "==" => field_value == rule.threshold_value,
                    "!=" => field_value != rule.threshold_value,
                    _ => false,
                };

                if should_alert {
                    let alert = Self::generate_alert(
                        pool,
                        AlertType::Threshold,
                        rule.severity.clone(),
                        &rule.name,
                        &format!("Rule '{}' triggered for {} {}: field {} {} {}", rule.name, entity_type, entity_id, rule.condition_field, rule.operator, rule.threshold_value),
                        Some(entity_type),
                        Some(entity_id),
                        Some(rule.id),
                    ).await?;
                    alerts.push(alert);
                }
            }
        }

        Ok(alerts)
    }

    pub async fn generate_alert(
        pool: &SqlitePool,
        alert_type: AlertType,
        severity: Severity,
        title: &str,
        message: &str,
        source_entity: Option<&str>,
        source_id: Option<Uuid>,
        rule_id: Option<Uuid>,
    ) -> Result<Alert> {
        let now = Utc::now();
        let alert = Alert {
            id: Uuid::new_v4(),
            alert_type,
            severity,
            title: title.to_string(),
            message: message.to_string(),
            source_entity: source_entity.map(|s| s.to_string()),
            source_id,
            rule_id,
            acknowledged: false,
            acknowledged_by: None,
            acknowledged_at: None,
            created_at: now,
        };

        sqlx::query(
            "INSERT INTO alerts (id, alert_type, severity, title, message, source_entity, source_id, rule_id, acknowledged, acknowledged_by, acknowledged_at, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(alert.id.to_string())
        .bind(format!("{:?}", alert.alert_type))
        .bind(format!("{:?}", alert.severity))
        .bind(&alert.title)
        .bind(&alert.message)
        .bind(&alert.source_entity)
        .bind(alert.source_id.map(|id| id.to_string()))
        .bind(alert.rule_id.map(|id| id.to_string()))
        .bind(alert.acknowledged as i64)
        .bind(alert.acknowledged_by.map(|id| id.to_string()))
        .bind(alert.acknowledged_at.map(|d| d.to_rfc3339()))
        .bind(alert.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(alert)
    }

    pub async fn acknowledge_alert(pool: &SqlitePool, alert_id: Uuid, acknowledged_by: Uuid) -> Result<Alert> {
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE alerts SET acknowledged = 1, acknowledged_by = ?, acknowledged_at = ? WHERE id = ?"
        )
        .bind(acknowledged_by.to_string())
        .bind(now.to_rfc3339())
        .bind(alert_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let row = sqlx::query_as::<_, AlertRow>(
            "SELECT id, alert_type, severity, title, message, source_entity, source_id, rule_id, acknowledged, acknowledged_by, acknowledged_at, created_at
             FROM alerts WHERE id = ?"
        )
        .bind(alert_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("Alert", &alert_id.to_string()))?;

        Ok(row.into())
    }

    pub async fn get_active_alerts(pool: &SqlitePool, severity: Option<Severity>) -> Result<Vec<Alert>> {
        let rows = if let Some(sev) = severity {
            sqlx::query_as::<_, AlertRow>(
                "SELECT id, alert_type, severity, title, message, source_entity, source_id, rule_id, acknowledged, acknowledged_by, acknowledged_at, created_at
                 FROM alerts WHERE acknowledged = 0 AND severity = ? ORDER BY created_at DESC"
            )
            .bind(format!("{:?}", sev))
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        } else {
            sqlx::query_as::<_, AlertRow>(
                "SELECT id, alert_type, severity, title, message, source_entity, source_id, rule_id, acknowledged, acknowledged_by, acknowledged_at, created_at
                 FROM alerts WHERE acknowledged = 0 ORDER BY created_at DESC"
            )
            .fetch_all(pool)
            .await
            .map_err(|e| Error::Database(e.into()))?
        };

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }
}

pub struct PredictiveService;

impl PredictiveService {
    pub async fn create_model(
        pool: &SqlitePool,
        name: &str,
        model_type: ForecastModelType,
        target_entity: &str,
        features: Option<&str>,
        parameters: Option<&str>,
    ) -> Result<ForecastModel> {
        let model = ForecastModel {
            id: Uuid::new_v4(),
            name: name.to_string(),
            model_type,
            target_entity: target_entity.to_string(),
            features: features.map(|s| s.to_string()),
            parameters: parameters.map(|s| s.to_string()),
            accuracy_score: None,
            last_trained: None,
            status: Status::Draft,
        };

        sqlx::query(
            "INSERT INTO forecast_models (id, name, model_type, target_entity, features, parameters, accuracy_score, last_trained, status)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(model.id.to_string())
        .bind(&model.name)
        .bind(format!("{:?}", model.model_type))
        .bind(&model.target_entity)
        .bind(&model.features)
        .bind(&model.parameters)
        .bind(model.accuracy_score)
        .bind(model.last_trained.map(|d| d.to_rfc3339()))
        .bind(format!("{:?}", model.status))
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(model)
    }

    pub async fn train_model(pool: &SqlitePool, model_id: Uuid) -> Result<ForecastModel> {
        let now = Utc::now();
        
        sqlx::query(
            "UPDATE forecast_models SET status = 'Active', last_trained = ?, accuracy_score = 0.85 WHERE id = ?"
        )
        .bind(now.to_rfc3339())
        .bind(model_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let row = sqlx::query_as::<_, ForecastModelRow>(
            "SELECT id, name, model_type, target_entity, features, parameters, accuracy_score, last_trained, status
             FROM forecast_models WHERE id = ?"
        )
        .bind(model_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("ForecastModel", &model_id.to_string()))?;

        Ok(row.into())
    }

    pub async fn generate_prediction(
        pool: &SqlitePool,
        model_id: Uuid,
        entity_id: Option<Uuid>,
        prediction_date: chrono::DateTime<Utc>,
    ) -> Result<Prediction> {
        let model_row = sqlx::query_as::<_, ForecastModelRow>(
            "SELECT id, name, model_type, target_entity, features, parameters, accuracy_score, last_trained, status
             FROM forecast_models WHERE id = ? AND status = 'Active'"
        )
        .bind(model_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::business_rule("Model not found or not active"))?;

        let _model: ForecastModel = model_row.into();
        let predicted_value = 100.0 + (chrono::Utc::now().timestamp() as f64 % 50.0);
        let confidence_margin = predicted_value * 0.1;

        let prediction = Prediction {
            id: Uuid::new_v4(),
            model_id,
            entity_id,
            prediction_date,
            predicted_value,
            confidence_lower: Some(predicted_value - confidence_margin),
            confidence_upper: Some(predicted_value + confidence_margin),
            actual_value: None,
            created_at: Utc::now(),
        };

        sqlx::query(
            "INSERT INTO predictions (id, model_id, entity_id, prediction_date, predicted_value, confidence_lower, confidence_upper, actual_value, created_at)
             VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        )
        .bind(prediction.id.to_string())
        .bind(prediction.model_id.to_string())
        .bind(prediction.entity_id.map(|id| id.to_string()))
        .bind(prediction.prediction_date.to_rfc3339())
        .bind(prediction.predicted_value)
        .bind(prediction.confidence_lower)
        .bind(prediction.confidence_upper)
        .bind(prediction.actual_value)
        .bind(prediction.created_at.to_rfc3339())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(prediction)
    }

    pub async fn get_predictions(
        pool: &SqlitePool,
        model_id: Uuid,
        from_date: Option<chrono::DateTime<Utc>>,
        to_date: Option<chrono::DateTime<Utc>>,
    ) -> Result<Vec<Prediction>> {
        let rows = sqlx::query_as::<_, PredictionRow>(
            "SELECT id, model_id, entity_id, prediction_date, predicted_value, confidence_lower, confidence_upper, actual_value, created_at
             FROM predictions WHERE model_id = ? AND (? IS NULL OR prediction_date >= ?) AND (? IS NULL OR prediction_date <= ?)
             ORDER BY prediction_date"
        )
        .bind(model_id.to_string())
        .bind(from_date.map(|d| d.to_rfc3339()))
        .bind(from_date.map(|d| d.to_rfc3339()))
        .bind(to_date.map(|d| d.to_rfc3339()))
        .bind(to_date.map(|d| d.to_rfc3339()))
        .fetch_all(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        Ok(rows.into_iter().map(|r| r.into()).collect())
    }

    pub async fn compare_actual_vs_predicted(
        pool: &SqlitePool,
        prediction_id: Uuid,
        actual_value: f64,
    ) -> Result<Prediction> {
        sqlx::query(
            "UPDATE predictions SET actual_value = ? WHERE id = ?"
        )
        .bind(actual_value)
        .bind(prediction_id.to_string())
        .execute(pool)
        .await
        .map_err(|e| Error::Database(e))?;

        let row = sqlx::query_as::<_, PredictionRow>(
            "SELECT id, model_id, entity_id, prediction_date, predicted_value, confidence_lower, confidence_upper, actual_value, created_at
             FROM predictions WHERE id = ?"
        )
        .bind(prediction_id.to_string())
        .fetch_optional(pool)
        .await
        .map_err(|e| Error::Database(e.into()))?
        .ok_or_else(|| Error::not_found("Prediction", &prediction_id.to_string()))?;

        Ok(row.into())
    }
}

#[derive(sqlx::FromRow)]
struct DashboardRow {
    id: String,
    name: String,
    description: Option<String>,
    dashboard_type: String,
    is_default: i64,
    layout: Option<String>,
    created_by: Option<String>,
    created_at: String,
}

impl From<DashboardRow> for Dashboard {
    fn from(r: DashboardRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            dashboard_type: match r.dashboard_type.as_str() {
                "Operational" => DashboardType::Operational,
                "Financial" => DashboardType::Financial,
                "Sales" => DashboardType::Sales,
                "Inventory" => DashboardType::Inventory,
                "Manufacturing" => DashboardType::Manufacturing,
                "HR" => DashboardType::HR,
                "Custom" => DashboardType::Custom,
                _ => DashboardType::Executive,
            },
            is_default: r.is_default != 0,
            layout: r.layout,
            created_by: r.created_by.and_then(|id| Uuid::parse_str(&id).ok()),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct DashboardWidgetRow {
    id: String,
    dashboard_id: String,
    widget_type: String,
    title: String,
    data_source: String,
    query_text: Option<String>,
    refresh_interval: i64,
    position_x: i64,
    position_y: i64,
    width: i64,
    height: i64,
    config: Option<String>,
}

impl From<DashboardWidgetRow> for DashboardWidget {
    fn from(r: DashboardWidgetRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            dashboard_id: Uuid::parse_str(&r.dashboard_id).unwrap_or_default(),
            widget_type: match r.widget_type.as_str() {
                "Table" => WidgetType::Table,
                "KPI" => WidgetType::KPI,
                "Gauge" => WidgetType::Gauge,
                "Map" => WidgetType::Map,
                "Text" => WidgetType::Text,
                "Image" => WidgetType::Image,
                "Counter" => WidgetType::Counter,
                _ => WidgetType::Chart,
            },
            title: r.title,
            data_source: r.data_source,
            query_text: r.query_text,
            refresh_interval: r.refresh_interval as i32,
            position_x: r.position_x as i32,
            position_y: r.position_y as i32,
            width: r.width as i32,
            height: r.height as i32,
            config: r.config,
        }
    }
}

#[derive(sqlx::FromRow)]
struct KPIDefinitionRow {
    id: String,
    kpi_code: String,
    name: String,
    description: Option<String>,
    category: String,
    unit: Option<String>,
    target_value: Option<f64>,
    warning_threshold: Option<f64>,
    critical_threshold: Option<f64>,
    calculation_formula: Option<String>,
    data_source: Option<String>,
    refresh_frequency: String,
    owner: Option<String>,
    status: String,
}

impl From<KPIDefinitionRow> for KPIDefinition {
    fn from(r: KPIDefinitionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            kpi_code: r.kpi_code,
            name: r.name,
            description: r.description,
            category: match r.category.as_str() {
                "Operational" => KPICategory::Operational,
                "Sales" => KPICategory::Sales,
                "Customer" => KPICategory::Customer,
                "HR" => KPICategory::HR,
                "Quality" => KPICategory::Quality,
                "Efficiency" => KPICategory::Efficiency,
                _ => KPICategory::Financial,
            },
            unit: r.unit,
            target_value: r.target_value,
            warning_threshold: r.warning_threshold,
            critical_threshold: r.critical_threshold,
            calculation_formula: r.calculation_formula,
            data_source: r.data_source,
            refresh_frequency: match r.refresh_frequency.as_str() {
                "Hourly" => RefreshFrequency::Hourly,
                "Daily" => RefreshFrequency::Daily,
                "Weekly" => RefreshFrequency::Weekly,
                "Monthly" => RefreshFrequency::Monthly,
                _ => RefreshFrequency::RealTime,
            },
            owner: r.owner.and_then(|id| Uuid::parse_str(&id).ok()),
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
        }
    }
}

#[derive(sqlx::FromRow)]
struct KPIValueRow {
    id: String,
    kpi_id: String,
    period: String,
    value: f64,
    target: Option<f64>,
    variance: Option<f64>,
    variance_percent: Option<f64>,
    trend: Option<String>,
    calculated_at: String,
}

impl From<KPIValueRow> for KPIValue {
    fn from(r: KPIValueRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            kpi_id: Uuid::parse_str(&r.kpi_id).unwrap_or_default(),
            period: r.period,
            value: r.value,
            target: r.target,
            variance: r.variance,
            variance_percent: r.variance_percent,
            trend: r.trend.and_then(|t| match t.as_str() {
                "Down" => Some(TrendDirection::Down),
                "Flat" => Some(TrendDirection::Flat),
                _ => Some(TrendDirection::Up),
            }),
            calculated_at: chrono::DateTime::parse_from_rfc3339(&r.calculated_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct AlertRuleRow {
    id: String,
    name: String,
    description: Option<String>,
    entity_type: String,
    condition_field: String,
    operator: String,
    threshold_value: String,
    severity: String,
    notification_channels: Option<String>,
    status: String,
    created_at: String,
}

impl From<AlertRuleRow> for AlertRule {
    fn from(r: AlertRuleRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            description: r.description,
            entity_type: r.entity_type,
            condition_field: r.condition_field,
            operator: r.operator,
            threshold_value: r.threshold_value,
            severity: match r.severity.as_str() {
                "Warning" => Severity::Warning,
                "Critical" => Severity::Critical,
                "Emergency" => Severity::Emergency,
                _ => Severity::Info,
            },
            notification_channels: r.notification_channels,
            status: match r.status.as_str() {
                "Inactive" => Status::Inactive,
                _ => Status::Active,
            },
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct AlertRow {
    id: String,
    alert_type: String,
    severity: String,
    title: String,
    message: String,
    source_entity: Option<String>,
    source_id: Option<String>,
    rule_id: Option<String>,
    acknowledged: i64,
    acknowledged_by: Option<String>,
    acknowledged_at: Option<String>,
    created_at: String,
}

impl From<AlertRow> for Alert {
    fn from(r: AlertRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            alert_type: match r.alert_type.as_str() {
                "Business" => AlertType::Business,
                "Threshold" => AlertType::Threshold,
                "Anomaly" => AlertType::Anomaly,
                "Scheduled" => AlertType::Scheduled,
                _ => AlertType::System,
            },
            severity: match r.severity.as_str() {
                "Warning" => Severity::Warning,
                "Critical" => Severity::Critical,
                "Emergency" => Severity::Emergency,
                _ => Severity::Info,
            },
            title: r.title,
            message: r.message,
            source_entity: r.source_entity,
            source_id: r.source_id.and_then(|id| Uuid::parse_str(&id).ok()),
            rule_id: r.rule_id.and_then(|id| Uuid::parse_str(&id).ok()),
            acknowledged: r.acknowledged != 0,
            acknowledged_by: r.acknowledged_by.and_then(|id| Uuid::parse_str(&id).ok()),
            acknowledged_at: r.acknowledged_at.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}

#[derive(sqlx::FromRow)]
struct ForecastModelRow {
    id: String,
    name: String,
    model_type: String,
    target_entity: String,
    features: Option<String>,
    parameters: Option<String>,
    accuracy_score: Option<f64>,
    last_trained: Option<String>,
    status: String,
}

impl From<ForecastModelRow> for ForecastModel {
    fn from(r: ForecastModelRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            name: r.name,
            model_type: match r.model_type.as_str() {
                "MovingAverage" => ForecastModelType::MovingAverage,
                "ExponentialSmoothing" => ForecastModelType::ExponentialSmoothing,
                "ARIMA" => ForecastModelType::ARIMA,
                "Prophet" => ForecastModelType::Prophet,
                "NeuralNetwork" => ForecastModelType::NeuralNetwork,
                "Ensemble" => ForecastModelType::Ensemble,
                _ => ForecastModelType::LinearRegression,
            },
            target_entity: r.target_entity,
            features: r.features,
            parameters: r.parameters,
            accuracy_score: r.accuracy_score,
            last_trained: r.last_trained.and_then(|d| chrono::DateTime::parse_from_rfc3339(&d).ok())
                .map(|d| d.with_timezone(&Utc)),
            status: match r.status.as_str() {
                "Active" => Status::Active,
                "Inactive" => Status::Inactive,
                _ => Status::Draft,
            },
        }
    }
}

#[derive(sqlx::FromRow)]
struct PredictionRow {
    id: String,
    model_id: String,
    entity_id: Option<String>,
    prediction_date: String,
    predicted_value: f64,
    confidence_lower: Option<f64>,
    confidence_upper: Option<f64>,
    actual_value: Option<f64>,
    created_at: String,
}

impl From<PredictionRow> for Prediction {
    fn from(r: PredictionRow) -> Self {
        Self {
            id: Uuid::parse_str(&r.id).unwrap_or_default(),
            model_id: Uuid::parse_str(&r.model_id).unwrap_or_default(),
            entity_id: r.entity_id.and_then(|id| Uuid::parse_str(&id).ok()),
            prediction_date: chrono::DateTime::parse_from_rfc3339(&r.prediction_date)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
            predicted_value: r.predicted_value,
            confidence_lower: r.confidence_lower,
            confidence_upper: r.confidence_upper,
            actual_value: r.actual_value,
            created_at: chrono::DateTime::parse_from_rfc3339(&r.created_at)
                .map(|d| d.with_timezone(&Utc))
                .unwrap_or_else(|_| Utc::now()),
        }
    }
}
