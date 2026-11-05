# \VaSystemDashboardsApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**get_va_system_a2_summary**](VaSystemDashboardsApi.md#get_va_system_a2_summary) | **GET** /api/dashboards/va-system-a2-summary | VA System A2.0 Summary
[**get_va_system_aa_summary**](VaSystemDashboardsApi.md#get_va_system_aa_summary) | **GET** /api/dashboards/va-system-aa-summary | VA System A&A Summary
[**get_va_system_fisma_invetory_crypto_summary**](VaSystemDashboardsApi.md#get_va_system_fisma_invetory_crypto_summary) | **GET** /api/dashboards/va-system-fisma-inventory-crypto-summary | VA System FISMA Inventory Crypto Summary
[**get_va_system_fisma_invetory_summary**](VaSystemDashboardsApi.md#get_va_system_fisma_invetory_summary) | **GET** /api/dashboards/va-system-fisma-inventory-summary | VA System FISMA Inventory Summary
[**get_va_system_icamp_tableau_poam_details**](VaSystemDashboardsApi.md#get_va_system_icamp_tableau_poam_details) | **GET** /api/dashboards/va-system-icamp-tableau-poam-details | VA System ICAMP Tableau POAM Details
[**get_va_system_pl109_reporting_summary**](VaSystemDashboardsApi.md#get_va_system_pl109_reporting_summary) | **GET** /api/dashboards/va-system-pl-109-reporting-summary | VA System P.L. 109 Reporting Summary
[**get_va_system_threat_architecture_details**](VaSystemDashboardsApi.md#get_va_system_threat_architecture_details) | **GET** /api/dashboards/va-system-threat-architecture-details | VA System Threat Architecture Details
[**get_va_system_threat_risk_summary**](VaSystemDashboardsApi.md#get_va_system_threat_risk_summary) | **GET** /api/dashboards/va-system-threat-risks-summary | VA System Threat Risks Summary
[**get_va_system_threat_source_details**](VaSystemDashboardsApi.md#get_va_system_threat_source_details) | **GET** /api/dashboards/va-system-threat-sources-details | VA System Threat Sources Details



## get_va_system_a2_summary

> models::GetSystemStatusDetails200Response get_va_system_a2_summary(org_id, excludeinherited, page_index, page_size)
VA System A2.0 Summary

Get VA system A2.0 summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_aa_summary

> models::GetSystemStatusDetails200Response get_va_system_aa_summary(org_id, excludeinherited, page_index, page_size)
VA System A&A Summary

Get VA system A&A summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_fisma_invetory_crypto_summary

> models::GetSystemStatusDetails200Response get_va_system_fisma_invetory_crypto_summary(org_id, excludeinherited, page_index, page_size)
VA System FISMA Inventory Crypto Summary

Get VA system FISMA inventory crypto summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_fisma_invetory_summary

> models::GetSystemStatusDetails200Response get_va_system_fisma_invetory_summary(org_id, excludeinherited, page_index, page_size)
VA System FISMA Inventory Summary

Get VA system FISMA inventory summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_icamp_tableau_poam_details

> models::GetSystemStatusDetails200Response get_va_system_icamp_tableau_poam_details(org_id, excludeinherited, page_index, page_size)
VA System ICAMP Tableau POAM Details

Get VA system ICAMP Tableau POAM details dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_pl109_reporting_summary

> models::GetSystemStatusDetails200Response get_va_system_pl109_reporting_summary(org_id, excludeinherited, page_index, page_size)
VA System P.L. 109 Reporting Summary

Get VA system P.L. 109 reporting summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_threat_architecture_details

> models::GetSystemStatusDetails200Response get_va_system_threat_architecture_details(org_id, excludeinherited, page_index, page_size)
VA System Threat Architecture Details

Get VA system threat architecture details dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_threat_risk_summary

> models::GetSystemStatusDetails200Response get_va_system_threat_risk_summary(org_id, excludeinherited, page_index, page_size)
VA System Threat Risks Summary

Get VA system threat risk summary dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_va_system_threat_source_details

> models::GetSystemStatusDetails200Response get_va_system_threat_source_details(org_id, excludeinherited, page_index, page_size)
VA System Threat Sources Details

Get VA system threat source details dashboard information.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**org_id** | **i32** | **Organization Id**: The unique organization identifier. | [required] |
**excludeinherited** | Option<**bool**> | **Exclude Inherited**: If no value is specified, the default returns false to include inherited data.  |  |[default to false]
**page_index** | Option<**i32**> | **Page Index**: If no value is specified, the default returns results from the first page with an index of 0.  |  |[default to 0]
**page_size** | Option<**i32**> | **Page Size**: If no value is specified, the default returns 20000 per page.  |  |[default to 20000]

### Return type

[**models::GetSystemStatusDetails200Response**](getSystemStatusDetails_200_response.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

