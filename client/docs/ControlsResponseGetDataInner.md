# ControlsResponseGetDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**acronym** | Option<**String**> | [Required] Acronym of the system record. | [optional]
**responsible_entities** | Option<**String**> | [Required] Include written description of Responsible Entities that are responsible for the Security Control. Character Limit 2,000. | [optional]
**control_designation** | Option<**ControlDesignation**> | [Required] Control designations (enum: Common, System-Specific, Hybrid) | [optional]
**estimated_completion_date** | Option<**i64**> | [Required] Field is required for Implementation Plan. Unix time format. | [optional]
**implementation_narrative** | Option<**String**> | [Required] Includes security control comments. Character Limit 2,000. | [optional]
**common_control_provider** | Option<**CommonControlProvider**> | [Conditional] Indicate the type of Common Control Provider for an Inherited Security Control. (enum: DoD, Component, Enclave) | [optional]
**na_justification** | Option<**String**> | [Conditional] Provide justification for Security Controls deemed Not Applicable to the system. | [optional]
**slcm_criticality** | Option<**String**> | [Conditional] Criticality of Security Control regarding SLCM. Character Limit = 2,000. | [optional]
**slcm_frequency** | Option<**SlcmFrequency**> | [Conditional] SLCM frequency (enum: Constantly, Daily, Weekly, Monthly, Quarterly, Semi-Annually, Annually, Every Two Years, Every Three Years, Undetermined) | [optional]
**slcm_method** | Option<**SlcmMethod**> | [Conditional] SLCM method utilized (enum: Automated, Semi-Automated, Manual, Undetermined) | [optional]
**slcm_reporting** | Option<**String**> | [Conditional] Method for reporting Security Control for SLCM. Character Limit = 2,000. | [optional]
**slcm_tracking** | Option<**String**> | [Conditional] How Non-Compliant Security Controls will be tracked for SLCM. Character Limit = 2,000. | [optional]
**slcm_comments** | Option<**String**> | [Conditional] Additional comments for Security Control regarding SLCM. Character Limit = 4,000. | [optional]
**implementation_status** | Option<**ImplementationStatus**> | [Optional] Implementation Status of the Security Control for the information system. (enum: Planned, Implemented, Inherited, Not Applicable, Manually Inherited) | [optional]
**severity** | Option<**Severity**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**vulnerabilty_summary** | Option<**String**> | [Optional] Include vulnerability summary. Character Limit = 2,000. | [optional]
**recommendations** | Option<**String**> | [Optional] Include recommendations. Character Limit = 2,000. | [optional]
**relevance_of_threat** | Option<**RelevanceOfThreat**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**likelihood** | Option<**Likelihood**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**impact** | Option<**Impact**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**impact_description** | Option<**String**> | [Optional] Include description of Security Control's impact. | [optional]
**residual_risk_level** | Option<**ResidualRiskLevel**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**test_method** | Option<**TestMethod**> | [Optional] Identifies the assessment method / combination that will determine if the security requirements are implemented correctly. (enum: Test, Interview, Examine, Test, Interview, Test, Examine, Interview, Examine, Test, Interview, Examine) | [optional]
**mitigations** | Option<**String**> | [Optional] Identify any mitigations in place for the Non-Compliant Security Control's vulnerabilities. Character Limit = 2,000. | [optional]
**application_layer** | Option<**String**> | [Optional] If the Financial Management (Navy) overlay is applied to the system, this field appears and can be populated. Character Limit = 2,000. Navy only. | [optional]
**database_layer** | Option<**String**> | [Optional] If the Financial Management (Navy) overlay is applied to the system, this field appears and can be populated. Navy only. | [optional]
**operating_system_layer** | Option<**String**> | [Optional] If the Financial Management (Navy) overlay is applied to the system, this field appears and can be populated. Navy only. | [optional]
**name** | Option<**String**> | [Read-only] Name of the system record. | [optional]
**ccis** | Option<**String**> | [Read-only] Comma separated list of CCIs associated with the control. | [optional]
**is_inherited** | Option<**bool**> | [Read-only] Indicates whether a control is inherited. | [optional]
**modified_by_overlays** | Option<**String**> | [Read-only] List of overlays that affect the control. | [optional]
**included_status** | Option<**String**> | [Read-only] Indicates the manner by which a control was included in the system's categorization. | [optional]
**compliance_status** | Option<**String**> | [Read-only] Compliance of the control. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


