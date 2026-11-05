# CmmcGet

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**operation** | Option<**Operation**> | [Read-Only] Indicates the action that should be taken on the assessment record since the provided sinceDate. (enum: ADDED, UPDATED, DELETED) | [optional]
**hq_organization_name** | Option<**String**> | [Read-Only] The name of the DIB Company. | [optional]
**uei** | Option<**String**> | [Read-Only] The Unique Entity Identifier assigned to the DIB Company. | [optional]
**osc_name** | Option<**String**> | [Read-Only] The name of the Organization Seeking Certification. | [optional]
**highest_level_owner_cage_code** | Option<**String**> | [Read-Only] Identifies the highest-level CAGE Code associated with a given organization. | [optional]
**cage_codes_in_scope** | Option<**String**> | [Read-Only] The five position code(s) associated with the Organization Seeking Certification (OSC). | [optional]
**number_of_employees** | Option<**i64**> | [Read-Only] The number of employees affiliated with the Organization Seeking Certification. | [optional]
**scope** | Option<**String**> | [Read-Only] The scope of the OSC assessment. | [optional]
**scope_description** | Option<**String**> | [Read-Only] Brief description of the scope of the OSC assessment | [optional]
**assessment_standard** | Option<**String**> | [Read-Only] Version of the CMMC Model used as part of the assessment. | [optional]
**assessment_id** | Option<**String**> | [Read-Only] Unique identifier for the assessment/certificate. | [optional]
**cmmc_uid** | Option<**String**> | [Read-Only] Identifies the unique ID that is associated with a given CMMC certification for an organization. | [optional]
**overall_score** | Option<**i64**> | [Read-Only] Identifies the overall calculated score for the assessment based on the assigned values to each applicable security requirement. | [optional]
**cmmc_status** | Option<**String**> | [Read-Only] The status of the CMMC certification. | [optional]
**cmmc_status_date** | Option<**i64**> | [Read-Only] Date of the CMMC status. | [optional]
**cmmc_status_expiration_date** | Option<**i64**> | [Read-Only] Expiration date of the CMMC status. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


