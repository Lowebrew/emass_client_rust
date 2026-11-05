# ControlsOptionalFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
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

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


