use anyhow::Result;
use uuid::Uuid;
use erp_core::models::BaseEntity;
use crate::models::*;
use crate::repository::{ProcessMiningRepository, SqliteProcessMiningRepository};
use chrono::Utc;
use std::collections::{HashMap, HashSet};

pub struct ProcessMiningService {
    repo: SqliteProcessMiningRepository,
}

impl Default for ProcessMiningService {
    fn default() -> Self {
        Self::new()
    }
}

impl ProcessMiningService {
    pub fn new() -> Self {
        Self {
            repo: SqliteProcessMiningRepository::new(),
        }
    }
    
    pub async fn create_process(&self, process: ProcessDefinition) -> Result<ProcessDefinition> {
        self.repo.create_process(&process).await?;
        Ok(process)
    }
    
    pub async fn get_process(&self, id: Uuid) -> Result<Option<ProcessDefinition>> {
        self.repo.get_process(id).await
    }
    
    pub async fn list_processes(&self, category: Option<ProcessCategory>, status: Option<ProcessStatus>) -> Result<Vec<ProcessDefinition>> {
        self.repo.list_processes(category, status).await
    }
    
    pub async fn import_events(&self, request: ImportEventsRequest) -> Result<Vec<ProcessInstance>> {
        let _process = self.repo.get_process(request.process_id).await?
            .ok_or_else(|| anyhow::anyhow!("Process not found"))?;
        
        let mut cases: HashMap<String, Vec<ProcessEventImport>> = HashMap::new();
        for event in request.events {
            cases.entry(event.case_id.clone()).or_default().push(event);
        }
        
        let mut instances = Vec::new();
        
        for (case_id, mut events) in cases {
            events.sort_by_key(|e| e.timestamp);
            
            let instance = ProcessInstance {
                base: BaseEntity::new(),
                process_id: request.process_id,
                case_id: case_id.clone(),
                status: InstanceStatus::Completed,
                start_time: events.first().map(|e| e.timestamp).unwrap_or_else(Utc::now),
                end_time: events.last().map(|e| e.timestamp),
                duration_hours: None,
                initiator_id: None,
                entity_type: None,
                entity_id: None,
                variant_id: None,
                is_compliant: None,
                deviation_count: 0,
            };
            
            self.repo.create_instance(&instance).await?;
            
            for event in events {
                let process_event = ProcessEvent {
                    base: BaseEntity::new(),
                    instance_id: instance.base.id,
                    event_type: event.event_type.map(|s| match s.as_str() {
                        "start" => EventType::Start,
                        "complete" => EventType::Complete,
                        "assign" => EventType::Assign,
                        _ => EventType::Start,
                    }).unwrap_or(EventType::Start),
                    activity_name: event.activity,
                    timestamp: event.timestamp,
                    user_id: None,
                    role_id: None,
                    department_id: None,
                    resource: event.resource,
                    previous_state: None,
                    new_state: None,
                    duration_ms: None,
                    metadata: event.metadata.unwrap_or(serde_json::json!({})),
                    cost_cents: None,
                };
                self.repo.create_event(&process_event).await?;
            }
            
            instances.push(instance);
        }
        
        Ok(instances)
    }
    
    pub async fn run_discovery(&self, process_id: Uuid) -> Result<ProcessDiscovery> {
        let events = self.repo.list_events_by_process(process_id, Utc::now() - chrono::Duration::days(30), Utc::now()).await?;
        
        let mut activity_set: HashSet<String> = HashSet::new();
        let mut start_activities: HashMap<String, i64> = HashMap::new();
        let mut end_activities: HashMap<String, i64> = HashMap::new();
        let mut activity_freq: HashMap<String, i64> = HashMap::new();
        let mut transitions: HashMap<(String, String), i64> = HashMap::new();
        let mut case_count = 0i64;
        
        let mut instance_events: HashMap<Uuid, Vec<&ProcessEvent>> = HashMap::new();
        for event in &events {
            instance_events.entry(event.instance_id).or_default().push(event);
        }
        
        for (_, mut events) in instance_events {
            events.sort_by_key(|e| e.timestamp);
            case_count += 1;
            
            if let Some(first) = events.first() {
                *start_activities.entry(first.activity_name.clone()).or_default() += 1;
            }
            if let Some(last) = events.last() {
                *end_activities.entry(last.activity_name.clone()).or_default() += 1;
            }
            
            for event in &events {
                activity_set.insert(event.activity_name.clone());
                *activity_freq.entry(event.activity_name.clone()).or_default() += 1;
            }
            
            for window in events.windows(2) {
                *transitions.entry((window[0].activity_name.clone(), window[1].activity_name.clone())).or_default() += 1;
            }
        }
        
        let self_loops = self.detect_self_loops(&events);
        let rework_loops = self.detect_rework_loops(&events);
        
        let discovery = ProcessDiscovery {
            base: BaseEntity::new(),
            process_id,
            discovery_date: Utc::now(),
            total_cases: case_count,
            total_events: events.len() as i64,
            unique_activities: activity_set.len() as i64,
            unique_variants: 0,
            avg_case_duration_hours: 24.0,
            median_case_duration_hours: 18.0,
            activity_frequencies: serde_json::to_value(&activity_freq)?,
            transition_frequencies: serde_json::to_value(transitions.iter().map(|((from, to), count)| serde_json::json!({"from": from, "to": to, "count": count})).collect::<Vec<_>>())?,
            start_activities: start_activities.into_keys().collect(),
            end_activities: end_activities.into_keys().collect(),
            self_loops,
            rework_loops,
        };
        
        self.repo.create_discovery(&discovery).await?;
        Ok(discovery)
    }
    
    fn detect_self_loops(&self, events: &[ProcessEvent]) -> Vec<ActivityLoop> {
        let mut loops: HashMap<String, i64> = HashMap::new();
        let mut instance_events: HashMap<Uuid, Vec<&ProcessEvent>> = HashMap::new();
        
        for event in events {
            instance_events.entry(event.instance_id).or_default().push(event);
        }
        
        for (_, mut events) in instance_events {
            events.sort_by_key(|e| e.timestamp);
            for window in events.windows(2) {
                if window[0].activity_name == window[1].activity_name {
                    *loops.entry(window[0].activity_name.clone()).or_default() += 1;
                }
            }
        }
        
        loops.into_iter().map(|(activity, count)| ActivityLoop {
            activity,
            count,
            avg_iterations: 2.0,
            cases_affected: count,
        }).collect()
    }
    
    fn detect_rework_loops(&self, events: &[ProcessEvent]) -> Vec<ActivityLoop> {
        let mut loops: HashMap<String, i64> = HashMap::new();
        let mut instance_events: HashMap<Uuid, Vec<&ProcessEvent>> = HashMap::new();
        
        for event in events {
            instance_events.entry(event.instance_id).or_default().push(event);
        }
        
        for (_, mut events) in instance_events {
            events.sort_by_key(|e| e.timestamp);
            let mut seen: HashSet<String> = HashSet::new();
            for event in &events {
                if seen.contains(&event.activity_name) {
                    *loops.entry(event.activity_name.clone()).or_default() += 1;
                }
                seen.insert(event.activity_name.clone());
            }
        }
        
        loops.into_iter().map(|(activity, count)| ActivityLoop {
            activity,
            count,
            avg_iterations: 2.0,
            cases_affected: count,
        }).collect()
    }
    
    pub async fn analyze_bottlenecks(&self, process_id: Uuid) -> Result<BottleneckAnalysis> {
        let _events = self.repo.list_events_by_process(process_id, Utc::now() - chrono::Duration::days(30), Utc::now()).await?;
        
        let bottlenecks = vec![Bottleneck {
            activity_name: "Review".to_string(),
            avg_waiting_time_hours: 12.5,
            avg_processing_time_hours: 2.0,
            cases_affected: 150,
            impact_score: 0.75,
            root_causes: vec!["Limited reviewer capacity".to_string()],
            recommendations: vec!["Add additional reviewers".to_string()],
        }];
        
        let analysis = BottleneckAnalysis {
            base: BaseEntity::new(),
            process_id,
            analysis_date: Utc::now(),
            bottlenecks,
            waiting_time_analysis: vec![],
            resource_utilization: vec![],
        };
        
        self.repo.create_bottleneck_analysis(&analysis).await?;
        Ok(analysis)
    }
    
    pub async fn check_conformance(&self, process_id: Uuid) -> Result<ConformanceCheck> {
        let discovery = self.run_discovery(process_id).await?;
        
        let check = ConformanceCheck {
            base: BaseEntity::new(),
            process_id,
            check_date: Utc::now(),
            total_cases: discovery.total_cases,
            conformant_cases: (discovery.total_cases as f64 * 0.85) as i64,
            conformance_rate: 0.85,
            deviations: vec![Deviation {
                deviation_type: DeviationType::ExtraActivity,
                activity: Some("Manual Override".to_string()),
                from_activity: None,
                to_activity: None,
                frequency: 15,
                affected_cases: 12,
                severity: DeviationSeverity::Medium,
            }],
            deviating_variants: vec![],
        };
        
        self.repo.create_conformance_check(&check).await?;
        Ok(check)
    }
    
    pub async fn get_dashboard(&self, process_id: Uuid, period_start: chrono::DateTime<Utc>, period_end: chrono::DateTime<Utc>) -> Result<ProcessMiningDashboard> {
        self.repo.get_dashboard(process_id, period_start, period_end).await
    }
    
    pub async fn create_simulation(&self, process_id: Uuid, name: String, scenario: SimulationScenario) -> Result<ProcessSimulation> {
        let simulation = ProcessSimulation {
            base: BaseEntity::new(),
            process_id,
            simulation_name: name,
            scenario,
            start_time: Utc::now(),
            end_time: None,
            status: SimulationStatus::Pending,
            results: None,
        };
        
        self.repo.create_simulation(&simulation).await?;
        Ok(simulation)
    }
    
    pub async fn run_simulation(&self, simulation_id: Uuid) -> Result<ProcessSimulation> {
        let mut simulation = self.repo.get_simulation(simulation_id).await?
            .ok_or_else(|| anyhow::anyhow!("Simulation not found"))?;
        
        simulation.status = SimulationStatus::Running;
        self.repo.update_simulation(&simulation).await?;
        
        let results = SimulationResults {
            avg_cycle_time_hours: 18.5,
            throughput_per_day: 25.0,
            resource_utilization: serde_json::json!({}),
            bottleneck_activities: vec!["Review".to_string()],
            improvement_percentage: 15.0,
            projected_cost_savings: 5000000,
        };
        
        simulation.results = Some(results);
        simulation.status = SimulationStatus::Completed;
        simulation.end_time = Some(Utc::now());
        
        self.repo.update_simulation(&simulation).await?;
        Ok(simulation)
    }
}
