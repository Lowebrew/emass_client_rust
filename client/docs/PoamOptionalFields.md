# PoamOptionalFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**external_uid** | Option<**String**> | [Optional] Unique identifier external to the eMASS application for use with associating POA&Ms. 100 Characters. | [optional]
**control_acronym** | Option<**String**> | [Optional] Control acronym associated with the POA&M Item. NIST SP 800-53 Revision 4 defined. | [optional]
**assessment_procedure** | Option<**String**> | [Optional] The Security Control Assessment Procedure being associated with the POA&M Item. | [optional]
**security_checks** | Option<**String**> | [Optional] Security Checks that are associated with the POA&M. | [optional]
**raw_severity** | Option<**String**> | [Optional] Scan vulnerability ratting Values include the following options: (Very Low, Low, Moderate,High,Very High) | [optional]
**relevance_of_threat** | Option<**String**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) | [optional]
**likelihood** | Option<**String**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) | [optional]
**impact** | Option<**String**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) | [optional]
**impact_description** | Option<**String**> | [Optional] Include description of Security Control's impact. | [optional]
**residual_risk_level** | Option<**String**> | [Optional] Values include the following options (Very Low, Low, Moderate,High,Very High) | [optional]
**recommendations** | Option<**String**> | [Optional] Include recommendations. Character Limit = 2,000. | [optional]
**mitigations** | Option<**String**> | [Optional] Include mitigation explanation. 2000 Characters. | [optional]
**resulting_residual_risk_level_after_proposed_mitigations** | Option<**ResultingResidualRiskLevelAfterProposedMitigations**> | [Optional] Indicate the risk level expected after any proposed mitigations are implemented. Proposed mitigations should be appropriately documented as POA&M milestones. Navy only. (enum: Very Low, Low, Moderate, High, Very High) | [optional]
**predisposing_conditions** | Option<**String**> | [Optional] A predisposing condition is a condition existing within an organization, a mission or business process, enterprise architecture, information system/PIT, or environment of operation, which affects (i.e., increases or decreases) the likelihood that threat events, once initiated, result in adverse impacts. Navy only. | [optional]
**threat_description** | Option<**String**> | [Optional] Describe the identified threat(s) and relevance to the information system. Navy only. | [optional]
**devices_affected** | Option<**String**> | [Optional] List any affected devices by hostname. If all devices in the information system are affected, state 'system' or 'all'. Navy only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


