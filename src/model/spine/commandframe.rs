use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::*;

pub type MsgCounterType = u64;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CmdClassifierType {
    Read,
    Reply,
    Notify,
    Write,
    Call,
    Result,
}

impl fmt::Display for CmdClassifierType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FilterIdType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilterType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_id: Option<FilterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cmd_control: Option<CmdControlType>,

	// DataSelectorsChoiceGroup
	pub alarm_list_data_selectors: Option<alarm::AlarmListDataSelectorsType>,
	pub bill_constraints_list_data_selectors: Option<bill::BillConstraintsListDataSelectorsType>,
	pub bill_description_list_data_selectors: Option<bill::BillDescriptionListDataSelectorsType>,
	pub bill_list_data_selectors: Option<bill::BillListDataSelectorsType>,
	pub binding_management_entry_list_data_selectors: Option<bindingmanagement::BindingManagementEntryListDataSelectorsType>,
	pub commodity_list_data_selectors: Option<tariffinformation::CommodityListDataSelectorsType>,
	pub device_configuration_key_value_constraints_list_data_selectors: Option<deviceconfiguration::DeviceConfigurationKeyValueConstraintsListDataSelectorsType>,
	pub device_configuration_key_value_description_list_data_selectors: Option<deviceconfiguration::DeviceConfigurationKeyValueDescriptionListDataSelectorsType>,
	pub device_configuration_key_value_list_data_selectors: Option<deviceconfiguration::DeviceConfigurationKeyValueListDataSelectorsType>,
	pub direction_control_activity_list_data_selectors: Option<directcontrol::DirectControlActivityListDataType>,
	pub electrical_connection_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionDescriptionListDataSelectorsType>,
	pub electrical_connection_parameter_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataSelectorsType>,
	pub electrical_connection_permitted_value_set_list_data_selectors: Option<electricalconnection::ElectricalConnectionPermittedValueSetListDataSelectorsType>,
	pub electrical_connection_state_list_data_selectors: Option<electricalconnection::ElectricalConnectionStateListDataSelectorsType>,
	pub hvac_operation_mode_description_list_data_selectors: Option<hvac::HvacOperationModeDescriptionListDataSelectorsType>,
	pub hvac_overrun_description_list_data_selectors: Option<hvac::HvacOverrunDescriptionListDataSelectorsType>,
	pub hvac_overrun_list_data_selectors: Option<hvac::HvacOverrunListDataSelectorsType>,
	pub hvac_system_function_description_list_data_selectors: Option<hvac::HvacSystemFunctionDescriptionListDataSelectorsType>,
	pub hvac_system_function_list_data_selectors: Option<hvac::HvacSystemFunctionListDataSelectorsType>,
	pub hvac_system_function_operation_mode_relation_list_data_selectors: Option<hvac::HvacSystemFunctionOperationModeRelationListDataSelectorsType>,
	pub hvac_system_function_power_sequence_realation_list_data_selectors: Option<hvac::HvacSystemFunctionPowerSequenceRelationListDataSelectorsType>,
	pub hvac_system_function_setpoint_relation_list_data_selectors: Option<hvac::HvacSystemFunctionSetpointRelationListDataSelectorsType>,
	pub identification_list_data_selectors: Option<identification::IdentificationListDataSelectorsType>,
	pub incentive_description_list_data_selectors: Option<tariffinformation::IncentiveDescriptionListDataSelectorsType>,
	pub incentive_list_data_selectors: Option<tariffinformation::IncentiveListDataSelectorsType>,
	pub incentive_table_constraints_data_selectors: Option<incentivetable::IncentiveTableConstraintsDataSelectorsType>,
	pub incentive_table_data_selectors: Option<incentivetable::IncentiveTableDataSelectorsType>,
	pub incentive_table_description_data_selectors: Option<incentivetable::IncentiveTableDescriptionDataSelectorsType>,
	pub load_control_event_list_data_selectors: Option<loadcontrol::LoadControlEventListDataSelectorsType>,
	pub load_control_limit_constraints_list_data_selectors: Option<loadcontrol::LoadControlLimitConstraintsListDataSelectorsType>,
	pub load_control_limit_description_list_data_selectors: Option<loadcontrol::LoadControlLimitDescriptionListDataSelectorsType>,
	pub load_control_limit_list_data_selectors: Option<loadcontrol::LoadControlLimitListDataSelectorsType>,
	pub load_control_state_list_data_selectors: Option<loadcontrol::LoadControlStateListDataSelectorsType>,
	pub measurement_constraints_list_data_selectors: Option<measurement::MeasurementConstraintsListDataSelectorsType>,
	pub measurement_description_list_data_selectors: Option<measurement::MeasurementDescriptionListDataSelectorsType>,
	pub measurement_list_data_selectors: Option<measurement::MeasurementListDataSelectorsType>,
	pub measurement_threshold_relation_list_data_selectors: Option<measurement::MeasurementThresholdRelationListDataSelectorsType>,
	pub messaging_list_data_selectors: Option<messaging::MessagingListDataSelectorsType>,
	pub network_management_device_description_list_data_selectors: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
	pub network_management_entity_description_list_data_selectors: Option<networkmanagement::NetworkManagementEntityDescriptionListDataSelectorsType>,
	pub network_management_feature_description_list_data_selectors: Option<networkmanagement::NetworkManagementFeatureDescriptionListDataSelectorsType>,
	pub node_management_binding_data_selectors: Option<nodemanagement::NodeManagementBindingDataSelectorsType>,
	pub node_management_destination_list_data_selectors: Option<nodemanagement::NodeManagementDestinationListDataSelectorsType>,
	pub node_management_detailed_discovery_data_selectors: Option<nodemanagement::NodeManagementDetailedDiscoveryDataSelectorsType>,
	pub node_management_subscription_data_selectors: Option<nodemanagement::NodeManagementSubscriptionDataSelectorsType>,
	pub node_management_use_case_data_selectors: Option<nodemanagement::NodeManagementUseCaseDataSelectorsType>,
	pub operating_constraints_duration_list_data_selectors: Option<operatingconstraints::OperatingConstraintsDurationListDataSelectorsType>,
	pub operating_constraints_interrupt_list_data_selectors: Option<operatingconstraints::OperatingConstraintsInterruptListDataSelectorsType>,
	pub operating_constraints_power_description_list_data_selectors: Option<operatingconstraints::OperatingConstraintsPowerDescriptionListDataSelectorsType>,
	pub operating_constraints_power_level_list_data_selectors: Option<operatingconstraints::OperatingConstraintsPowerLevelListDataSelectorsType>,
	pub operating_constraints_power_range_list_data_selectors: Option<operatingconstraints::OperatingConstraintsPowerRangeListDataSelectorsType>,
	pub operating_constraints_resume_implication_list_data_selectors: Option<operatingconstraints::OperatingConstraintsResumeImplicationListDataSelectorsType>,
	pub power_sequence_alternatives_relation_list_data_selectors: Option<powersequences::PowerSequenceAlternativesRelationListDataSelectorsType>,
	pub power_sequence_description_list_data_selectors: Option<powersequences::PowerSequenceDescriptionListDataSelectorsType>,
	pub power_sequence_price_list_data_selectors: Option<powersequences::PowerSequencePriceListDataSelectorsType>,
	pub power_sequence_schedule_constraints_list_data_selectors: Option<powersequences::PowerSequenceScheduleConstraintsListDataSelectorsType>,
	pub power_sequence_schedule_list_data_selectors: Option<powersequences::PowerSequenceScheduleListDataSelectorsType>,
	pub power_sequence_schedule_preference_list_data_selectors: Option<powersequences::PowerSequenceSchedulePreferenceListDataSelectorsType>,
	pub power_sequence_state_list_data_selectors: Option<powersequences::PowerSequenceStateListDataSelectorsType>,
	pub power_time_slot_schedule_constraints_list_data_selectors: Option<powersequences::PowerTimeSlotScheduleConstraintsListDataSelectorsType>,
	pub power_time_slot_schedule_list_data_selectors: Option<powersequences::PowerTimeSlotScheduleListDataSelectorsType>,
	pub power_time_slot_value_list_data_selectors: Option<powersequences::PowerTimeSlotValueListDataSelectorsType>,
	pub sensing_list_data_selectors: Option<sensing::SensingListDataSelectorsType>,
	pub setpoint_constraints_list_data_selectors: Option<setpoint::SetpointConstraintsListDataSelectorsType>,
	pub setpoint_description_list_data_selectors: Option<setpoint::SetpointDescriptionListDataSelectorsType>,
	pub setpoint_list_data_selectors: Option<setpoint::SetpointListDataSelectorsType>,
	pub smart_energy_management_ps_data_selectors: Option<smartenergymanagementps::SmartEnergyManagementPsDataSelectorsType>,
	pub smart_energy_management_ps_price_data_selectors: Option<smartenergymanagementps::SmartEnergyManagementPsPriceDataSelectorsType>,
	pub specification_version_list_data_selectors: Option<version::SpecificationVersionListDataSelectorsType>,
	pub subscription_management_entry_list_data_selectors: Option<subscriptionmanagement::SubscriptionManagementEntryListDataSelectorsType>,
	pub supply_condition_description_list_data_selectors: Option<supplycondition::SupplyConditionDescriptionListDataSelectorsType>,
	pub supply_condition_list_data_selectors: Option<supplycondition::SupplyConditionListDataSelectorsType>,
	pub supply_condition_threshold_relation_list_data_selectors: Option<supplycondition::SupplyConditionThresholdRelationListDataSelectorsType>,
	pub tariff_boundary_relation_list_data_selectors: Option<tariffinformation::TariffBoundaryRelationListDataSelectorsType>,
	pub tariff_description_list_data_selectors: Option<tariffinformation::TariffDescriptionListDataSelectorsType>,
	pub tariff_tier_relation_list_data_selectors: Option<tariffinformation::TariffTierRelationListDataSelectorsType>,
	pub task_management_job_description_list_data_selectors: Option<taskmanagement::TaskManagementJobDescriptionListDataSelectorsType>,
	pub task_management_job_list_data_selectors: Option<taskmanagement::TaskManagementJobListDataSelectorsType>,
	pub task_management_job_relation_list_data_selectors: Option<taskmanagement::TaskManagementJobRelationListDataSelectorsType>,
	pub threshold_constraints_list_data_selectors: Option<threshold::ThresholdConstraintsListDataSelectorsType>,
	pub threshold_description_list_data_selectors: Option<threshold::ThresholdDescriptionListDataSelectorsType>,
	pub threshold_list_data_selectors: Option<threshold::ThresholdListDataSelectorsType>,
	pub tier_boundary_description_list_data_selectors: Option<tariffinformation::TierBoundaryDescriptionListDataSelectorsType>,
	pub tier_boundary_list_data_selectors: Option<tariffinformation::TierBoundaryListDataSelectorsType>,
	pub tier_description_list_data_selectors: Option<tariffinformation::TierDescriptionListDataSelectorsType>,
	pub tier_incentive_relation_list_data_selectors: Option<tariffinformation::TierIncentiveRelationListDataSelectorsType>,
	pub tier_list_data_selectors: Option<tariffinformation::TierListDataSelectorsType>,
	pub time_series_constraints_list_data_selectors: Option<timeseries::TimeSeriesConstraintsListDataSelectorsType>,
	pub time_series_description_list_data_selectors: Option<timeseries::TimeSeriesDescriptionListDataSelectorsType>,
	pub time_series_list_data_selectors: Option<timeseries::TimeSeriesListDataSelectorsType>,
	pub time_table_constraints_list_data_selectors: Option<timetable::TimeTableConstraintsListDataSelectorsType>,
	pub time_table_description_list_data_selectors: Option<timetable::TimeTableDescriptionListDataSelectorsType>,
	pub time_table_list_data_selectors: Option<timetable::TimeTableListDataSelectorsType>,
	pub use_case_information_list_data_selectors: Option<usecaseinformation::UseCaseInformationListDataSelectorsType>,

	// DataElementsChoiceGroup
	pub actuator_level_data_elements: Option<actuatorlevel::ActuatorLevelDataElementsType>,
	pub actuator_level_description_data_elements: Option<actuatorlevel::ActuatorLevelDescriptionDataElementsType>,
	pub actuator_switch_data_elements: Option<actuatorswitch::ActuatorSwitchDataElementsType>,
	pub actuator_switch_description_data_elements: Option<actuatorswitch::ActuatorSwitchDescriptionDataElementsType>,
	pub alarm_data_elements: Option<alarm::AlarmDataElementsType>,
	pub bill_constraints_data_elements: Option<bill::BillConstraintsDataElementsType>,
	pub bill_data_elements: Option<bill::BillDataElementsType>,
	pub bill_description_data_elements: Option<bill::BillDescriptionDataElementsType>,
	pub binding_management_delete_call_elements: Option<bindingmanagement::BindingManagementDeleteCallElementsType>,
	pub binding_management_entry_data_elements: Option<bindingmanagement::BindingManagementEntryDataElementsType>,
	pub binding_management_request_call_elements: Option<bindingmanagement::BindingManagementRequestCallElementsType>,
	pub commodity_data_elements: Option<tariffinformation::CommodityDataElementsType>,
	pub data_tunneling_call_elements: Option<datatunneling::DataTunnelingCallElementsType>,
	pub device_classification_manufacturer_data_elements: Option<deviceclassification::DeviceClassificationManufacturerDataElementsType>,
	pub device_classification_user_data_elements: Option<deviceclassification::DeviceClassificationUserDataElementsType>,
	pub device_configuration_key_value_constraints_data_elements: Option<deviceconfiguration::DeviceConfigurationKeyValueConstraintsDataElementsType>,
	pub device_configuration_key_value_data_elements: Option<deviceconfiguration::DeviceConfigurationKeyValueDataElementsType>,
	pub device_configuration_key_value_description_data_elements: Option<deviceconfiguration::DeviceConfigurationKeyValueDescriptionDataElementsType>,
	pub device_diagnosis_heartbeat_data_elements: Option<devicediagnosis::DeviceDiagnosisHeartbeatDataElementsType>,
	pub device_diagnosis_service_data_elements: Option<devicediagnosis::DeviceDiagnosisServiceDataElementsType>,
	pub device_diagnosis_state_data_elements: Option<devicediagnosis::DeviceDiagnosisStateDataElementsType>,
	pub direct_control_activity_data_elements: Option<directcontrol::DirectControlActivityDataElementsType>,
	pub direct_control_description_data_elements: Option<directcontrol::DirectControlDescriptionDataElementsType>,
	pub electrical_connection_description_data_elements: Option<electricalconnection::ElectricalConnectionDescriptionDataElementsType>,
	pub electrical_connection_parameter_description_data_elements: Option<electricalconnection::ElectricalConnectionParameterDescriptionDataElementsType>,
	pub electrical_connection_permitted_value_set_data_elements: Option<electricalconnection::ElectricalConnectionPermittedValueSetDataElementsType>,
	pub electrical_connection_state_data_elements: Option<electricalconnection::ElectricalConnectionStateDataElementsType>,
	pub hvac_operation_mode_description_data_elements: Option<hvac::HvacOperationModeDescriptionDataElementsType>,
	pub hvac_overrun_data_elements: Option<hvac::HvacOverrunDataElementsType>,
	pub hvac_overrun_description_data_elements: Option<hvac::HvacOverrunDescriptionDataElementsType>,
	pub hvac_system_function_description_data_elements: Option<hvac::HvacSystemFunctionDescriptionDataElementsType>,
	pub hvac_system_function_operation_mode_relation_data_elements: Option<hvac::HvacSystemFunctionOperationModeRelationDataElementsType>,
	pub hvac_system_function_power_sequence_relation_data_elements: Option<hvac::HvacSystemFunctionPowerSequenceRelationDataElementsType>,
	pub hvac_system_function_setpoint_relation_data_elements: Option<hvac::HvacSystemFunctionSetpointRelationDataElementsType>,
	pub identification_data_elements: Option<identification::IdentificationDataElementsType>,
	pub incentive_data_elements: Option<tariffinformation::IncentiveDataElementsType>,
	pub incentive_description_data_elements: Option<tariffinformation::IncentiveDescriptionDataElementsType>,
	pub incentive_table_constraints_data_elements: Option<incentivetable::IncentiveTableConstraintsDataElementsType>,
	pub incentive_table_data_elements: Option<incentivetable::IncentiveTableDataElementsType>,
	pub incentive_table_description_data_elements: Option<incentivetable::IncentiveTableDescriptionDataElementsType>,
	pub load_control_event_data_elements: Option<loadcontrol::LoadControlEventDataElementsType>,
	pub load_control_limit_constraints_data_elements: Option<loadcontrol::LoadControlLimitConstraintsDataElementsType>,
	pub load_control_limit_data_elements: Option<loadcontrol::LoadControlLimitDataElementsType>,
	pub load_control_limit_description_data_elements: Option<loadcontrol::LoadControlLimitDescriptionDataElementsType>,
	pub load_control_node_data_elements: Option<loadcontrol::LoadControlNodeDataElementsType>,
	pub load_control_state_data_elements: Option<loadcontrol::LoadControlStateDataElementsType>,
	pub measurement_constraints_data_elements: Option<measurement::MeasurementConstraintsDataElementsType>,
	pub measurement_data_elements: Option<measurement::MeasurementDataElementsType>,
	pub measurement_description_data_elements: Option<measurement::MeasurementDescriptionDataElementsType>,
	pub measurement_threshold_relation_data_elements: Option<measurement::MeasurementThresholdRelationDataElementsType>,
	pub messaging_data_elements: Option<messaging::MessagingDataElementsType>,
	pub network_management_abort_call_elements: Option<networkmanagement::NetworkManagementAbortCallElementsType>,
	pub network_management_add_node_call_elements: Option<networkmanagement::NetworkManagementAddNodeCallElementsType>,
	pub network_management_device_description_data_elements: Option<networkmanagement::NetworkManagementDeviceDescriptionDataElementsType>,
	pub network_management_discover_call_elements: Option<networkmanagement::NetworkManagementDiscoverCallElementsType>,
	pub network_management_entity_description_data_elements: Option<networkmanagement::NetworkManagementEntityDescriptionDataElementsType>,
	pub network_management_feature_description_data_elements: Option<networkmanagement::NetworkManagementFeatureDescriptionDataElementsType>,
	pub network_management_joining_mode_data_elements: Option<networkmanagement::NetworkManagementJoiningModeDataElementsType>,
	pub network_management_modify_node_call_elements: Option<networkmanagement::NetworkManagementModifyNodeCallElementsType>,
	pub network_management_process_state_data_elements: Option<networkmanagement::NetworkManagementProcessStateDataElementsType>,
	pub network_management_remove_node_call_elements: Option<networkmanagement::NetworkManagementRemoveNodeCallElementsType>,
	pub network_management_report_candidate_data_elements: Option<networkmanagement::NetworkManagementReportCandidateDataElementsType>,
	pub network_management_scan_network_call_elements: Option<networkmanagement::NetworkManagementScanNetworkCallElementsType>,
	pub node_management_binding_data_elements: Option<nodemanagement::NodeManagementBindingDataElementsType>,
	pub node_management_binding_delete_call_elements: Option<nodemanagement::NodeManagementBindingDeleteCallElementsType>,
	pub node_management_binding_request_call_elements: Option<nodemanagement::NodeManagementBindingRequestCallElementsType>,
	pub node_management_destination_data_elements: Option<nodemanagement::NodeManagementDestinationDataElementsType>,
	pub node_management_detailed_discovery_data_elements: Option<nodemanagement::NodeManagementDetailedDiscoveryDataElementsType>,
	pub node_management_subscription_data_elements: Option<nodemanagement::NodeManagementSubscriptionDataElementsType>,
	pub node_management_subscription_delete_call_elements: Option<nodemanagement::NodeManagementSubscriptionDeleteCallElementsType>,
	pub node_management_subscription_request_call_elements: Option<nodemanagement::NodeManagementSubscriptionRequestCallElementsType>,
	pub node_management_use_case_data_elements: Option<nodemanagement::NodeManagementUseCaseDataElementsType>,
	pub operating_constraints_duration_data_elements: Option<operatingconstraints::OperatingConstraintsDurationDataElementsType>,
	pub operating_constraints_interrupt_data_elements: Option<operatingconstraints::OperatingConstraintsInterruptDataElementsType>,
	pub operating_constraints_power_description_data_elements: Option<operatingconstraints::OperatingConstraintsPowerDescriptionDataElementsType>,
	pub operating_constraints_power_level_data_elements: Option<operatingconstraints::OperatingConstraintsPowerLevelDataElementsType>,
	pub operating_constraints_power_range_data_elements: Option<operatingconstraints::OperatingConstraintsPowerRangeDataElementsType>,
	pub operating_constraints_resume_implication_data_elements: Option<operatingconstraints::OperatingConstraintsResumeImplicationDataElementsType>,
	pub power_sequence_alternatives_relation_data_elements: Option<powersequences::PowerSequenceAlternativesRelationDataElementsType>,
	pub power_sequence_description_data_elements: Option<powersequences::PowerSequenceDescriptionDataElementsType>,
	pub power_sequence_node_schedule_information_data_elements: Option<powersequences::PowerSequenceNodeScheduleInformationDataElementsType>,
	pub power_sequence_price_calculation_request_call_elements: Option<powersequences::PowerSequencePriceCalculationRequestCallElementsType>,
	pub power_sequence_price_data_elements: Option<powersequences::PowerSequencePriceDataElementsType>,
	pub power_sequence_schedule_configuration_request_call_elements: Option<powersequences::PowerSequenceScheduleConfigurationRequestCallElementsType>,
	pub power_sequence_schedule_constraints_data_elements: Option<powersequences::PowerSequenceScheduleConstraintsDataElementsType>,
	pub power_sequence_schedule_data_elements: Option<powersequences::PowerSequenceScheduleDataElementsType>,
	pub power_sequence_schedule_preference_data_elements: Option<powersequences::PowerSequenceSchedulePreferenceDataElementsType>,
	pub power_sequence_state_data_elements: Option<powersequences::PowerSequenceStateDataElementsType>,
	pub power_time_slot_schedule_constraints_data_elements: Option<powersequences::PowerTimeSlotScheduleConstraintsDataElementsType>,
	pub power_time_slot_schedule_data_elements: Option<powersequences::PowerTimeSlotScheduleDataElementsType>,
	pub power_time_slot_value_data_elements: Option<powersequences::PowerTimeSlotValueDataElementsType>,
	pub sensing_data_elements: Option<sensing::SensingDataElementsType>,
	pub sensing_description_data_elements: Option<sensing::SensingDescriptionDataElementsType>,
	pub setpoint_constraints_data_elements: Option<setpoint::SetpointConstraintsDataElementsType>,
	pub setpoint_data_elements: Option<setpoint::SetpointDataElementsType>,
	pub setpoint_description_data_elements: Option<setpoint::SetpointDescriptionDataElementsType>,
	pub smart_energy_management_ps_configuration_request_call_elements: Option<smartenergymanagementps::SmartEnergyManagementPsConfigurationRequestCallElementsType>,
	pub smart_energy_management_ps_data_elements: Option<smartenergymanagementps::SmartEnergyManagementPsDataElementsType>,
	pub smart_energy_management_ps_price_calculation_request_call_elements: Option<smartenergymanagementps::SmartEnergyManagementPsPriceCalculationRequestCallElementsType>,
	pub smart_energy_management_ps_price_data_elements: Option<smartenergymanagementps::SmartEnergyManagementPsPriceDataElementsType>,
	pub specification_version_data_elements: Option<version::SpecificationVersionDataElementsType>,
	pub subscription_management_delete_call_elements: Option<subscriptionmanagement::SubscriptionManagementDeleteCallElementsType>,
	pub subscription_management_entry_data_elements: Option<subscriptionmanagement::SubscriptionManagementEntryDataElementsType>,
	pub subscription_management_request_call_elements: Option<subscriptionmanagement::SubscriptionManagementRequestCallElementsType>,
	pub supply_condition_data_elements: Option<supplycondition::SupplyConditionDataElementsType>,
	pub supply_condition_description_data_elements: Option<supplycondition::SupplyConditionDescriptionDataElementsType>,
	pub supply_condition_threshold_relation_data_elements: Option<supplycondition::SupplyConditionThresholdRelationDataElementsType>,
	pub tariff_boundary_relation_data_elements: Option<tariffinformation::TariffBoundaryRelationDataElementsType>,
	pub tariff_data_elements: Option<tariffinformation::TariffDataElementsType>,
	pub tariff_description_data_elements: Option<tariffinformation::TariffDescriptionDataElementsType>,
	pub tariff_overall_constraints_data_elements: Option<tariffinformation::TariffOverallConstraintsDataElementsType>,
	pub tariff_tier_relation_data_elements: Option<tariffinformation::TariffTierRelationDataElementsType>,
	pub task_management_job_data_elements: Option<taskmanagement::TaskManagementJobDataElementsType>,
	pub task_management_job_description_data_elements: Option<taskmanagement::TaskManagementJobDescriptionDataElementsType>,
	pub task_management_job_relation_data_elements: Option<taskmanagement::TaskManagementJobRelationDataElementsType>,
	pub task_management_overview_data_elements: Option<taskmanagement::TaskManagementOverviewDataElementsType>,
	pub threshold_constraints_data_elements: Option<threshold::ThresholdConstraintsDataElementsType>,
	pub threshold_data_elements: Option<threshold::ThresholdDataElementsType>,
	pub threshold_description_data_elements: Option<threshold::ThresholdDescriptionDataElementsType>,
	pub tier_boundary_data_elements: Option<tariffinformation::TierBoundaryDataElementsType>,
	pub tier_boundary_description_data_elements: Option<tariffinformation::TierBoundaryDescriptionDataElementsType>,
	pub tier_data_elements: Option<tariffinformation::TierDataElementsType>,
	pub tier_description_data_elements: Option<tariffinformation::TierDescriptionDataElementsType>,
	pub tier_incentive_relation_data_elements: Option<tariffinformation::TierIncentiveRelationDataElementsType>,
	pub time_distributor_data_elements: Option<timeinformation::TimeDistributorDataElementsType>,
	pub time_distributor_enquiry_call_elements: Option<timeinformation::TimeDistributorEnquiryCallElementsType>,
	pub time_precision_data_elements: Option<timeinformation::TimePrecisionDataElementsType>,
	pub time_series_constraints_data_elements: Option<timeseries::TimeSeriesConstraintsDataElementsType>,
	pub time_series_data_elements: Option<timeseries::TimeSeriesDataElementsType>,
	pub time_series_description_data_elements: Option<timeseries::TimeSeriesDescriptionDataElementsType>,
	pub time_table_constraints_data_elements: Option<timetable::TimeTableConstraintsDataElementsType>,
	pub time_table_data_elements: Option<timetable::TimeTableDataElementsType>,
	pub time_table_description_data_elements: Option<timetable::TimeTableDescriptionDataElementsType>,
	pub use_case_information_data_elements: Option<usecaseinformation::UseCaseInformationDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CmdControlType {
	#[serde(skip_serializing_if = "Option::is_none")]
	delete:  Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	partial: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
	#[serde(skip_serializing_if = "Option::is_none")]
	filter_id:   Option<FilterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	cmd_control: Option<CmdControlType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CmdControl {
	#[serde(skip_serializing_if = "Option::is_none")]
	delete:  Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	partial: Option<commondatatypes::ElementTagType>
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CmdType {
	// CmdOptionGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<commondatatypes::FunctionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter:   Option<Vec<FilterType>>,

	// DataChoiceGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actuator_level_data: Option<actuatorlevel::ActuatorLevelDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actuator_level_description_data: Option<actuatorlevel::ActuatorLevelDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actuator_switch_data: Option<actuatorswitch::ActuatorSwitchDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actuator_switch_description_data: Option<actuatorswitch::ActuatorSwitchDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub alarm_list_data: Option<alarm::AlarmListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bill_constraints_list_data: Option<bill::BillConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bill_description_list_data: Option<bill::BillDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub bill_list_data: Option<bill::BillListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub binding_management_delete_call: Option<bindingmanagement::BindingManagementDeleteCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub binding_management_entry_list_call: Option<bindingmanagement::BindingManagementEntryListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub binding_management_request_call: Option<bindingmanagement::BindingManagementRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commodity_list_data: Option<tariffinformation::CommodityListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub data_tunneling_call: Option<datatunneling::DataTunnelingCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_classification_manufacturer_data: Option<deviceclassification::DeviceClassificationManufacturerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_classification_user_data: Option<deviceclassification::DeviceClassificationUserDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_configuration_key_value_constraints_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_configuration_key_value_description_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_configuration_key_value_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_diagnosis_heartbeat_data: Option<devicediagnosis::DeviceDiagnosisHeartbeatDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_diagnosis_service_data: Option<devicediagnosis::DeviceDiagnosisServiceDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_diagnosis_state_data: Option<devicediagnosis::DeviceDiagnosisStateDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub direct_control_activity_list_data: Option<directcontrol::DirectControlActivityListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub direct_control_description_data: Option<directcontrol::DirectControlDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_description_list_data: Option<electricalconnection::ElectricalConnectionDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_parameter_description_list_data: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_permitted_value_set_list_data: Option<electricalconnection::ElectricalConnectionPermittedValueSetListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_state_list_data: Option<electricalconnection::ElectricalConnectionStateListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_operation_mode_description_list_data: Option<hvac::HvacOperationModeDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_overrun_description_list_data: Option<hvac::HvacOverrunDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_overrun_list_data: Option<hvac::HvacOverrunListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_system_function_description_list_data: Option<hvac::HvacSystemFunctionDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_system_function_list_data: Option<hvac::HvacSystemFunctionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_system_function_operation_mode_relation_list_data: Option<hvac::HvacSystemFunctionOperationModeRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_system_function_power_sequence_realation_list_data: Option<hvac::HvacSystemFunctionPowerSequenceRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hvac_system_function_setpoint_relation_list_data: Option<hvac::HvacSystemFunctionSetpointRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identification_list_data: Option<identification::IdentificationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_description_list_data: Option<tariffinformation::IncentiveDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_list_data: Option<tariffinformation::IncentiveListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_constraints_data: Option<incentivetable::IncentiveTableConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_data: Option<incentivetable::IncentiveTableDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_description_data: Option<incentivetable::IncentiveTableDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub load_control_limit_constraints_list_data: Option<loadcontrol::LoadControlLimitConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub load_control_limit_description_list_data: Option<loadcontrol::LoadControlLimitDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub load_control_limit_list_data: Option<loadcontrol::LoadControlLimitListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_constraints_list_data: Option<measurement::MeasurementConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_description_list_data: Option<measurement::MeasurementDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_list_data: Option<measurement::MeasurementListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_abort_call: Option<networkmanagement::NetworkManagementAbortCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_add_node_call: Option<networkmanagement::NetworkManagementAddNodeCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_device_description_list_data: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_discover_call: Option<networkmanagement::NetworkManagementDiscoverCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_entity_description_list_data: Option<networkmanagement::NetworkManagementEntityDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_feature_description_list_data: Option<networkmanagement::NetworkManagementFeatureDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_joining_mode_data: Option<networkmanagement::NetworkManagementJoiningModeDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_modify_node_call: Option<networkmanagement::NetworkManagementModifyNodeCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_process_state_data: Option<networkmanagement::NetworkManagementProcessStateDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_remove_node_call: Option<networkmanagement::NetworkManagementRemoveNodeCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_report_candidate_data: Option<networkmanagement::NetworkManagementReportCandidateDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_scan_network_call: Option<networkmanagement::NetworkManagementScanNetworkCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_binding_data: Option<nodemanagement::NodeManagementBindingDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_binding_delete_call: Option<nodemanagement::NodeManagementBindingDeleteCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_binding_request_call: Option<nodemanagement::NodeManagementBindingRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_destination_list_data: Option<nodemanagement::NodeManagementDestinationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_detailed_discovery_data: Option<nodemanagement::NodeManagementDetailedDiscoveryDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_data: Option<nodemanagement::NodeManagementSubscriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_delete_call: Option<nodemanagement::NodeManagementSubscriptionDeleteCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_request_call: Option<nodemanagement::NodeManagementSubscriptionRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_use_case_data: Option<nodemanagement::NodeManagementUseCaseDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_duration_list_data: Option<operatingconstraints::OperatingConstraintsDurationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_interrupt_list_data: Option<operatingconstraints::OperatingConstraintsInterruptListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_power_description_list_data: Option<operatingconstraints::OperatingConstraintsPowerDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_power_level_list_data: Option<operatingconstraints::OperatingConstraintsPowerLevelListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_power_range_list_data: Option<operatingconstraints::OperatingConstraintsPowerRangeListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub operating_constraints_resume_implication_list_data: Option<operatingconstraints::OperatingConstraintsResumeImplicationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_alternatives_relation_list_data: Option<powersequences::PowerSequenceAlternativesRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_description_list_data: Option<powersequences::PowerSequenceDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_node_schedule_information_data: Option<powersequences::PowerSequenceNodeScheduleInformationDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_price_calculation_request_call: Option<powersequences::PowerSequencePriceCalculationRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_price_list_data: Option<powersequences::PowerSequencePriceListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_schedule_configuration_request_call: Option<powersequences::PowerSequenceScheduleConfigurationRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_schedule_constraints_list_data: Option<powersequences::PowerSequenceScheduleConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_schedule_list_data: Option<powersequences::PowerSequenceScheduleListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_schedule_preference_list_data: Option<powersequences::PowerSequenceSchedulePreferenceListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_sequence_state_list_data: Option<powersequences::PowerSequenceStateListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_time_slot_schedule_constraints_list_data: Option<powersequences::PowerTimeSlotScheduleConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_time_slot_schedule_list_data: Option<powersequences::PowerTimeSlotScheduleListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_time_slot_value_list_data: Option<powersequences::PowerTimeSlotValueListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub result_data: Option<result::ResultDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sensing_description_data: Option<sensing::SensingDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub sensing_list_data: Option<sensing::SensingListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub setpoint_constraints_list_data: Option<setpoint::SetpointConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub setpoint_description_list_data: Option<setpoint::SetpointDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub setpoint_list_data: Option<setpoint::SetpointListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smart_energy_management_ps_configuration_request_call: Option<smartenergymanagementps::SmartEnergyManagementPsConfigurationRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smart_energy_management_ps_data: Option<smartenergymanagementps::SmartEnergyManagementPsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smart_energy_management_ps_price_calculation_request_call: Option<smartenergymanagementps::SmartEnergyManagementPsPriceCalculationRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub smart_energy_management_ps_price_data: Option<smartenergymanagementps::SmartEnergyManagementPsPriceDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub specification_version_list_data: Option<version::SpecificationVersionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_management_delete_call: Option<subscriptionmanagement::SubscriptionManagementDeleteCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_management_entry_list_data: Option<subscriptionmanagement::SubscriptionManagementEntryListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub subscription_management_request_call: Option<subscriptionmanagement::SubscriptionManagementRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supply_condition_description_list_data: Option<supplycondition::SupplyConditionDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supply_condition_list_data: Option<supplycondition::SupplyConditionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supply_condition_threshold_relation_list_data: Option<supplycondition::SupplyConditionThresholdRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_boundary_relation_list_data: Option<tariffinformation::TariffBoundaryRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_description_list_data: Option<tariffinformation::TariffDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_list_data: Option<tariffinformation::TariffListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_overall_constraints_data: Option<tariffinformation::TariffOverallConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_tier_relation_list_data: Option<tariffinformation::TariffTierRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub task_management_job_description_list_data: Option<taskmanagement::TaskManagementJobDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub task_management_job_list_data: Option<taskmanagement::TaskManagementJobListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub task_management_job_relation_list_data: Option<taskmanagement::TaskManagementJobRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub task_management_overview_data: Option<taskmanagement::TaskManagementOverviewDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold_constraints_list_data: Option<threshold::ThresholdConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold_description_list_data: Option<threshold::ThresholdDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold_list_data: Option<threshold::ThresholdListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_boundary_description_list_data: Option<tariffinformation::TierBoundaryDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_boundary_list_data: Option<tariffinformation::TierBoundaryListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_description_list_data: Option<tariffinformation::TierDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_incentive_relation_list_data: Option<tariffinformation::TierIncentiveRelationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_list_data: Option<tariffinformation::TierListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_distributor_data: Option<timeinformation::TimeDistributorDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_distributor_enquiry_call: Option<timeinformation::TimeDistributorEnquiryCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_information_data: Option<timeinformation::TimeInformationDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_precision_data: Option<timeinformation::TimePrecisionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_constraints_data: Option<timeseries::TimeSeriesConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_constraints_list_data: Option<timeseries::TimeSeriesConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_description_list_data: Option<timeseries::TimeSeriesDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_list_data: Option<timeseries::TimeSeriesListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_constraints_list_data: Option<timetable::TimeTableConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_description_list_data: Option<timetable::TimeTableDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_list_data: Option<timetable::TimeTableListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_case_information_list_data: Option<usecaseinformation::UseCaseInformationListDataType>,

	// DataExtendGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer_specific_extension: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_update_at: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
}
