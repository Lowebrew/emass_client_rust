# TestResultsGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS identifier. Will need to provide correct number | [optional]
**control** | Option<**String**> | [Read-Only] Control acronym associated with the test result. NIST SP 800-53 Revision 4 defined. | [optional]
**cci** | Option<**String**> | [Required] CCI associated with test result. | [optional]
**assessment_procedure** | Option<**String**> | [Required] The Security Control Assessment Procedure being assessed. | [optional]
**is_inherited** | Option<**bool**> | [Read-only] Indicates whether a test result is inherited. | [optional]
**tested_by** | Option<**String**> | [Required] Last Name, First Name. 100 Characters. | [optional]
**test_date** | Option<**i64**> | [Required] Unix time format. | [optional]
**description** | Option<**String**> | [Required] Include description of test result. 4000 Characters. | [optional]
**r#type** | Option<**String**> | [Read-Only] Indicates the location in the Control Approval Chain when the test result is submitted. | [optional]
**compliance_status** | Option<**ComplianceStatus**> | [Required] Test result compliance status (enum: Compliant, Non-Compliant, Not Applicable) | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


