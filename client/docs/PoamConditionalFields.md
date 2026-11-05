# PoamConditionalFields

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**poc_first_name** | Option<**String**> | [Conditional] First name of POC. 100 Characters. | [optional]
**poc_last_name** | Option<**String**> | [Conditional] Last name of POC. 100 Characters. | [optional]
**poc_email** | Option<**String**> | [Conditional] Email address of POC. 100 Characters. | [optional]
**poc_phone_number** | Option<**String**> | [Conditional] Phone number of POC (area code) ***-**** format. 100 Characters. | [optional]
**severity** | Option<**String**> | [Conditional] Required for approved items. Values include the following options: (Very Low, Low, Moderate,High,Very High) | [optional]
**scheduled_completion_date** | Option<**i64**> | [Conditional] Required for ongoing and completed POA&M items. Unix time format. | [optional]
**completion_date** | Option<**i64**> | [Conditional] Field is required for completed POA&M items. Unix time format. | [optional]
**comments** | Option<**String**> | [Conditional] Field is required for completed and risk accepted POA&M items. 2000 Characters | [optional]
**personnel_resources_funded_base_hours** | Option<**f32**> | [Conditional] At least one of the following is required and must be completed for each POA&M Item:   Personnel Resources-> Funded Base Hours   Personnel Resources-> Unfunded Base Hours   Non-Personnel Resources-> Funded Amount   Non-Personnel Resources-> Unfunded Amount Displays numbers to the second decimal point (e.g., 100.00). VA only.  | [optional]
**personnel_resources_cost_code** | Option<**String**> | [Conditional] Required if Personnel Resources: Funded Base Hours is populated. Only accepts values present in the field's lookup table (modifiable by eMASS System Admins). VA only.  | [optional]
**personnel_resources_unfunded_base_hours** | Option<**f32**> | [Conditional] At least one of the following is required and must be completed for each POA&M Item:   Personnel Resources-> Funded Base Hours   Personnel Resources-> Unfunded Base Hours   Non-Personnel Resources-> Funded Amount   Non-Personnel Resources-> Unfunded Amount Displays numbers to the second decimal point (e.g., 100.00). VA only.  | [optional]
**personnel_resources_nonfunding_obstacle** | Option<**String**> | [Conditional] Required if Personnel Resources: Unfunded Base Hours is populated. Only accepts values present in the field's lookup table (modifiable by eMASS System Admins). VA only.  | [optional]
**personnel_resources_nonfunding_obstacle_other_reason** | Option<**String**> | [Conditional] Required if the value \"Other\" is populated for the field Personnel Resources: Non-Funding Obstacle. VA only. | [optional]
**non_personnel_resources_funded_amount** | Option<**f32**> | [Conditional] At least one of the following is required and must be completed for each POA&M Item:   Personnel Resources-> Funded Base Hours   Personnel Resources-> Unfunded Base Hours   Non-Personnel Resources-> Funded Amount   Non-Personnel Resources-> Unfunded Amount Displays numbers to the second decimal point (e.g., 100.00). VA only.  | [optional]
**non_personnel_resources_cost_code** | Option<**String**> | [Conditional] Required if Non-Personnel Resources: Funded Amount is populated. Only accepts values present in the field's lookup table (modifiable by eMASS System Admins). VA only.  | [optional]
**non_personnel_resources_unfunded_amount** | Option<**f32**> | [Conditional] At least one of the following is required and must be completed for each POA&M Item:   Personnel Resources-> Funded Base Hours   Personnel Resources-> Unfunded Base Hours   Non-Personnel Resources-> Funded Amount   Non-Personnel Resources-> Unfunded Amount Displays numbers to the second decimal point (e.g., 100.00). VA only.  | [optional]
**non_personnel_resources_nonfunding_obstacle** | Option<**String**> | [Conditional] Required if Non-Personnel Resources: Unfunded Amount is populated. Only accepts values present in the field's lookup table (modifiable by eMASS System Admins). VA only.  | [optional]
**non_personnel_resources_nonfunding_obstacle_other_reason** | Option<**String**> | [Conditional] Required if the value \"Other\" is populated for the field Non-Personnel Resources: Non-Funding Obstacle. VA only. | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


