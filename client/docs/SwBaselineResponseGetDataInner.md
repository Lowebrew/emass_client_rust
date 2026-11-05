# SwBaselineResponseGetDataInner

## Properties

Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**system_id** | Option<**i64**> | [Required] Unique eMASS system identifier. | [optional]
**software_id** | Option<**String**> | [Read-Only] GUID identifying the specific software asset. | [optional]
**software_vendor** | Option<**String**> | [Required] Vendor of the software asset. | [optional]
**software_name** | Option<**String**> | [Required] Name of the software asset. | [optional]
**version** | Option<**String**> | [Required] Version of the software asset. | [optional]
**software_type** | Option<**String**> | [Optional] Type of the software asset. | [optional]
**parent_system** | Option<**String**> | [Optional] Parent system of the software asset. | [optional]
**subsystem** | Option<**String**> | [Optional] Subsystem of the software asset. | [optional]
**network** | Option<**String**> | [Optional] Network of the software asset. | [optional]
**hosting_environment** | Option<**String**> | [Optional] Hosting environment of the software asset. | [optional]
**software_dependencies** | Option<**String**> | [Optional] Dependencies of the software asset. | [optional]
**cryptographic_hash** | Option<**String**> | [Optional] Cryptographic hash of the software asset. | [optional]
**in_service_data** | Option<**String**> | [Optional] In-service data of the software asset. | [optional]
**it_budget_uii** | Option<**String**> | [Optional] IT budget UII of the software asset. | [optional]
**fiscal_year** | Option<**String**> | [Optional] Fiscal year (FY) of the software asset. | [optional]
**pop_end_date** | Option<**i64**> | [Optional] Period of performance (POP) end date of the software asset. | [optional]
**license_or_contract** | Option<**String**> | [Optional] License or contract number of the software asset. | [optional]
**license_term** | Option<**String**> | [Optional] License term of the software asset. | [optional]
**cost_per_license** | Option<**f64**> | [Optional] Cost per license of the software asset. Number will be converted to display 2 decimal points. | [optional]
**total_licenses** | Option<**i64**> | [Optional] Total licenses of the software asset. | [optional]
**total_license_cost** | Option<**f64**> | [Optional] Total license cost of the software asset. Number will be converted to display 2 decimal points. | [optional]
**licenses_used** | Option<**i64**> | [Optional] Number of licenses used for the software asset. | [optional]
**license_poc** | Option<**String**> | [Optional] Point of contact (POC) for the software asset. | [optional]
**license_renewal_date** | Option<**i64**> | [Optional] License renewal date for the software asset. | [optional]
**license_expiration_date** | Option<**i64**> | [Optional] License expiration date for the software asset. | [optional]
**approval_status** | Option<**String**> | [Optional] Approval status of the software asset. | [optional]
**release_date** | Option<**i64**> | [Optional] Release date of the software asset. | [optional]
**maintenance_date** | Option<**i64**> | [Optional] Maintenance date of the software asset. | [optional]
**retirement_date** | Option<**i64**> | [Optional] Retirement date of the software asset. | [optional]
**end_of_life_support_date** | Option<**i64**> | [Optional] End of life support date of the software asset. | [optional]
**extended_end_of_life_support_date** | Option<**i64**> | [Optional] If set, the Extended End of Life/Support Date cannot occur prior to the End of Life/Support Date. | [optional]
**critical_asset** | Option<**bool**> | [Optional] Indicates whether the asset is a critical information system asset. | [optional]
**location** | Option<**String**> | [Optional] Location of the software asset. | [optional]
**purpose** | Option<**String**> | [Optional] Purpose of the software asset. | [optional]
**unsupported_operating_system** | Option<**bool**> | [Optional] Unsupported operating system. VA only. | [optional]
**unapproved_software_from_trm** | Option<**bool**> | [Optional] Unapproved software from TRM. VA only | [optional]
**approved_waiver** | Option<**bool**> | [Optional] Approved waiver. VA only | [optional]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


