use chrono::{DateTime, NaiveDate, Utc};
use erp_core::{BaseEntity, Status};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmissionScope {
    Scope1,
    Scope2,
    Scope3,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EmissionCategory {
    StationaryCombustion,
    MobileCombustion,
    ProcessEmissions,
    FugitiveEmissions,
    PurchasedElectricity,
    PurchasedHeat,
    PurchasedSteam,
    PurchasedCooling,
    UpstreamTransport,
    DownstreamTransport,
    BusinessTravel,
    EmployeeCommuting,
    WasteDisposal,
    PurchasedGoods,
    CapitalGoods,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonAccount {
    pub base: BaseEntity,
    pub account_number: String,
    pub name: String,
    pub description: Option<String>,
    pub emission_scope: EmissionScope,
    pub emission_category: EmissionCategory,
    pub location_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub responsible_person_id: Option<Uuid>,
    pub reporting_standard: String,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonEmission {
    pub base: BaseEntity,
    pub emission_number: String,
    pub account_id: Uuid,
    pub emission_date: NaiveDate,
    pub reporting_period: String,
    pub activity_data: f64,
    pub activity_unit: String,
    pub emission_factor: f64,
    pub emission_factor_unit: String,
    pub emission_factor_source: Option<String>,
    pub co2_equivalent: f64,
    pub co2_equivalent_unit: String,
    pub uncertainty_percent: Option<f64>,
    pub data_quality_score: Option<i32>,
    pub source_type: Option<String>,
    pub source_id: Option<Uuid>,
    pub verification_status: VerificationStatus,
    pub verified_by: Option<Uuid>,
    pub verified_at: Option<DateTime<Utc>>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum VerificationStatus {
    Unverified,
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmissionFactor {
    pub base: BaseEntity,
    pub factor_code: String,
    pub name: String,
    pub description: Option<String>,
    pub emission_category: EmissionCategory,
    pub fuel_type: Option<String>,
    pub region: Option<String>,
    pub year: i32,
    pub factor_value: f64,
    pub factor_unit: String,
    pub co2_factor: Option<f64>,
    pub ch4_factor: Option<f64>,
    pub n2o_factor: Option<f64>,
    pub gwp_co2: Option<f64>,
    pub gwp_ch4: Option<f64>,
    pub gwp_n2o: Option<f64>,
    pub source: String,
    pub source_url: Option<String>,
    pub uncertainty_percent: Option<f64>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonOffset {
    pub base: BaseEntity,
    pub offset_number: String,
    pub name: String,
    pub project_name: String,
    pub project_type: OffsetProjectType,
    pub standard: CarbonOffsetStandard,
    pub registry: String,
    pub registry_id: Option<String>,
    pub vintage_year: i32,
    pub quantity_tonnes: i64,
    pub remaining_tonnes: i64,
    pub price_per_tonne: i64,
    pub currency: String,
    pub purchase_date: NaiveDate,
    pub retirement_date: Option<NaiveDate>,
    pub project_location: Option<String>,
    pub co_benefits: Option<String>,
    pub sdg_contributions: Option<String>,
    pub verification_body: Option<String>,
    pub verification_date: Option<NaiveDate>,
    pub status: CarbonOffsetStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum OffsetProjectType {
    Reforestation,
    Afforestation,
    AvoidedDeforestation,
    RenewableEnergy,
    EnergyEfficiency,
    MethaneCapture,
    CarbonCapture,
    BlueCarbon,
    SoilCarbon,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CarbonOffsetStandard {
    VCS,
    GoldStandard,
    CDM,
    CAR,
    ACR,
    PlanVivo,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum CarbonOffsetStatus {
    Available,
    Retired,
    Expired,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CarbonOffsetRetirement {
    pub base: BaseEntity,
    pub retirement_number: String,
    pub offset_id: Uuid,
    pub retirement_date: NaiveDate,
    pub quantity_tonnes: i64,
    pub beneficiary: String,
    pub reason: Option<String>,
    pub retirement_beneficiary_account: Option<String>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnergyConsumption {
    pub base: BaseEntity,
    pub consumption_number: String,
    pub facility_id: Option<Uuid>,
    pub consumption_date: NaiveDate,
    pub reporting_period: String,
    pub energy_type: EnergyType,
    pub consumption_value: f64,
    pub consumption_unit: String,
    pub supplier: Option<String>,
    pub meter_id: Option<String>,
    pub cost: i64,
    pub currency: String,
    pub renewable_percent: f64,
    pub renewable_source: Option<String>,
    pub location_based_emissions: Option<f64>,
    pub market_based_emissions: Option<f64>,
    pub data_source: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum EnergyType {
    Electricity,
    NaturalGas,
    HeatingOil,
    Propane,
    Diesel,
    Gasoline,
    Steam,
    ChilledWater,
    Solar,
    Wind,
    Hydro,
    Biomass,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WaterConsumption {
    pub base: BaseEntity,
    pub consumption_number: String,
    pub facility_id: Option<Uuid>,
    pub consumption_date: NaiveDate,
    pub reporting_period: String,
    pub water_source: WaterSource,
    pub consumption_cubic_meters: f64,
    pub withdrawal_cubic_meters: f64,
    pub discharge_cubic_meters: f64,
    pub recycled_cubic_meters: f64,
    pub supplier: Option<String>,
    pub meter_id: Option<String>,
    pub cost: i64,
    pub currency: String,
    pub water_stress_area: bool,
    pub quality_treatment_required: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WaterSource {
    Municipal,
    Groundwater,
    SurfaceWater,
    Rainwater,
    Seawater,
    Recycled,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WasteRecord {
    pub base: BaseEntity,
    pub waste_number: String,
    pub facility_id: Option<Uuid>,
    pub waste_date: NaiveDate,
    pub reporting_period: String,
    pub waste_type: WasteType,
    pub waste_category: WasteCategory,
    pub quantity_kg: f64,
    pub disposal_method: DisposalMethod,
    pub contractor: Option<String>,
    pub manifest_number: Option<String>,
    pub cost: i64,
    pub currency: String,
    pub hazardous: bool,
    pub recycled_kg: Option<f64>,
    pub recovery_rate_percent: Option<f64>,
    pub destination_facility: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WasteType {
    General,
    Paper,
    Plastic,
    Metal,
    Glass,
    Organic,
    Electronic,
    Hazardous,
    Medical,
    Construction,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum WasteCategory {
    NonHazardous,
    Hazardous,
    Radioactive,
    Biological,
    Chemical,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum DisposalMethod {
    Landfill,
    Incineration,
    Recycling,
    Composting,
    AnaerobicDigestion,
    WasteToEnergy,
    Reuse,
    Treatment,
    Export,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityTarget {
    pub base: BaseEntity,
    pub target_number: String,
    pub name: String,
    pub description: Option<String>,
    pub target_type: SustainabilityTargetType,
    pub baseline_year: i32,
    pub target_year: i32,
    pub baseline_value: f64,
    pub target_value: f64,
    pub current_value: Option<f64>,
    pub progress_percent: Option<f64>,
    pub unit: String,
    pub scope: Option<String>,
    pub alignment: Option<String>,
    pub science_based: bool,
    pub status: TargetStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SustainabilityTargetType {
    CarbonReduction,
    CarbonNeutral,
    NetZero,
    RenewableEnergy,
    WaterReduction,
    WasteReduction,
    RecyclingRate,
    EnergyEfficiency,
    SupplierEngagement,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum TargetStatus {
    Planned,
    Active,
    OnTrack,
    AtRisk,
    OffTrack,
    Achieved,
    Missed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityMetric {
    pub base: BaseEntity,
    pub metric_date: NaiveDate,
    pub reporting_period: String,
    pub metric_type: SustainabilityMetricType,
    pub category: Option<String>,
    pub value: f64,
    pub unit: String,
    pub previous_value: Option<f64>,
    pub change_percent: Option<f64>,
    pub target_id: Option<Uuid>,
    pub facility_id: Option<Uuid>,
    pub department_id: Option<Uuid>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SustainabilityMetricType {
    CarbonIntensity,
    EnergyIntensity,
    WaterIntensity,
    WasteDiversionRate,
    RenewableEnergyPercent,
    CircularEconomyScore,
    SupplierESGScore,
    EmployeeEngagementScore,
    CommunityInvestment,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ESGReport {
    pub base: BaseEntity,
    pub report_number: String,
    pub name: String,
    pub reporting_period: String,
    pub period_start: NaiveDate,
    pub period_end: NaiveDate,
    pub report_type: ESGReportType,
    pub framework: ESGFramework,
    pub scope_1_emissions: Option<f64>,
    pub scope_2_emissions_location: Option<f64>,
    pub scope_2_emissions_market: Option<f64>,
    pub scope_3_emissions: Option<f64>,
    pub total_energy_consumed: Option<f64>,
    pub renewable_energy_percent: Option<f64>,
    pub total_water_withdrawn: Option<f64>,
    pub total_waste_generated: Option<f64>,
    pub waste_diverted_percent: Option<f64>,
    pub employee_count: Option<i32>,
    pub employee_turnover_percent: Option<f64>,
    pub diversity_data: Option<String>,
    pub safety_incidents: Option<i32>,
    pub training_hours: Option<f64>,
    pub community_investment: Option<i64>,
    pub currency: String,
    pub assurance_status: Option<AssuranceStatus>,
    pub assurance_provider: Option<String>,
    pub status: ReportStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ESGReportType {
    Annual,
    Sustainability,
    Integrated,
    Climate,
    TCFD,
    GRI,
    CDP,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ESGFramework {
    GRI,
    SASB,
    TCFD,
    CDP,
    UNSDG,
    IR,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssuranceStatus {
    None,
    Limited,
    Reasonable,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ReportStatus {
    Draft,
    InReview,
    Approved,
    Published,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SupplierESGAssessment {
    pub base: BaseEntity,
    pub assessment_number: String,
    pub vendor_id: Uuid,
    pub assessment_date: NaiveDate,
    pub questionnaire_version: String,
    pub environmental_score: Option<f64>,
    pub social_score: Option<f64>,
    pub governance_score: Option<f64>,
    pub overall_score: Option<f64>,
    pub rating: Option<ESGRating>,
    pub certifications: Option<String>,
    pub carbon_footprint_reported: Option<bool>,
    pub carbon_footprint_tonnes: Option<f64>,
    pub science_based_target: bool,
    pub modern_slavery_statement: bool,
    pub next_assessment_date: Option<NaiveDate>,
    pub status: AssessmentStatus,
    pub assessed_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum ESGRating {
    AAA,
    AA,
    A,
    BBB,
    BB,
    B,
    CCC,
    NotRated,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum AssessmentStatus {
    Pending,
    InProgress,
    Completed,
    Verified,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SocialMetric {
    pub base: BaseEntity,
    pub metric_date: NaiveDate,
    pub reporting_period: String,
    pub category: SocialMetricCategory,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub previous_value: Option<f64>,
    pub target_value: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum SocialMetricCategory {
    Diversity,
    Inclusion,
    HealthSafety,
    Training,
    Wellbeing,
    Community,
    HumanRights,
    LaborStandards,
    SupplyChain,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GovernanceMetric {
    pub base: BaseEntity,
    pub metric_date: NaiveDate,
    pub reporting_period: String,
    pub category: GovernanceMetricCategory,
    pub metric_name: String,
    pub value: f64,
    pub unit: String,
    pub previous_value: Option<f64>,
    pub target_value: Option<f64>,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum GovernanceMetricCategory {
    BoardComposition,
    ExecutiveCompensation,
    Ethics,
    AntiCorruption,
    DataPrivacy,
    Cybersecurity,
    RiskManagement,
    ShareholderRights,
    TaxTransparency,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RenewableEnergyCertificate {
    pub base: BaseEntity,
    pub certificate_number: String,
    pub energy_source: EnergyType,
    pub generation_facility: String,
    pub facility_location: Option<String>,
    pub generation_date_start: NaiveDate,
    pub generation_date_end: NaiveDate,
    pub mwh_quantity: f64,
    pub remaining_mwh: f64,
    pub registry: String,
    pub registry_id: Option<String>,
    pub tracking_number: Option<String>,
    pub purchase_date: NaiveDate,
    pub price_per_mwh: i64,
    pub currency: String,
    pub retired_mwh: f64,
    pub status: RECStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "TEXT")]
pub enum RECStatus {
    Active,
    Retired,
    Expired,
}
