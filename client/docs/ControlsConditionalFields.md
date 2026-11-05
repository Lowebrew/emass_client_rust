# ControlsConditionalFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**common_control_provider** | Option<**CommonControlProvider**> | [Conditional] Indicate the type of Common Control Provider for an Inherited Security Control. (enum: DoD, Component, Enclave) | [optional]
**na_justification** | Option<**String**> | [Conditional] Provide justification for Security Controls deemed Not Applicable to the system. | [optional]
**slcm_criticality** | Option<**String**> | [Conditional] Criticality of Security Control regarding SLCM. Character Limit = 2,000. | [optional]
**slcm_frequency** | Option<**SlcmFrequency**> | [Conditional] SLCM frequency (enum: Constantly, Daily, Weekly, Monthly, Quarterly, Semi-Annually, Annually, Every Two Years, Every Three Years, Undetermined) | [optional]
**slcm_method** | Option<**SlcmMethod**> | [Conditional] SLCM method utilized (enum: Automated, Semi-Automated, Manual, Undetermined) | [optional]
**slcm_reporting** | Option<**String**> | [Conditional] Method for reporting Security Control for SLCM. Character Limit = 2,000. | [optional]
**slcm_tracking** | Option<**String**> | [Conditional] How Non-Compliant Security Controls will be tracked for SLCM. Character Limit = 2,000. | [optional]
**slcm_comments** | Option<**String**> | [Conditional] Additional comments for Security Control regarding SLCM. Character Limit = 4,000. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


