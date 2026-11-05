# \MilestonesApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_milestone_by_system_id_and_poam_id**](MilestonesApi.md#add_milestone_by_system_id_and_poam_id) | **POST** /api/systems/{systemId}/poams/{poamId}/milestones | Add milestones to one or many POA&M items in a system
[**delete_milestone**](MilestonesApi.md#delete_milestone) | **DELETE** /api/systems/{systemId}/poams/{poamId}/milestones | Remove milestones in a system for one or many POA&M items
[**get_system_milestones_by_poam_id**](MilestonesApi.md#get_system_milestones_by_poam_id) | **GET** /api/systems/{systemId}/poams/{poamId}/milestones | Get milestones in one or many POA&M items in a system
[**get_system_milestones_by_poam_id_and_milestone_id**](MilestonesApi.md#get_system_milestones_by_poam_id_and_milestone_id) | **GET** /api/systems/{systemId}/poams/{poamId}/milestones/{milestoneId} | Get milestone by ID in POA&M item in a system
[**update_milestone_by_system_id_and_poam_id**](MilestonesApi.md#update_milestone_by_system_id_and_poam_id) | **PUT** /api/systems/{systemId}/poams/{poamId}/milestones | Update one or many POA&M items in a system



## add_milestone_by_system_id_and_poam_id

> models::MilestoneResponsePost add_milestone_by_system_id_and_poam_id(system_id, poam_id, serde_json_colon_colon_value)
Add milestones to one or many POA&M items in a system

Adds a milestone for given `systemId` and `poamId` path parameters  **Request Body Required Fields** - `description` - `scheduledCompletionDate`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for adding milestones to an existing System (systemId) POA&M (poamId) | [required] |

### Return type

[**models::MilestoneResponsePost**](MilestoneResponsePost.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_milestone

> models::MilestoneResponseDelete delete_milestone(system_id, poam_id, milestones_request_delete_body_inner)
Remove milestones in a system for one or many POA&M items

Remove the POA&M matching `systemId` and `poamId` for path parameters and `milstoneId` provide in the Request Body  **NOTE**<br> To delete a milestone the record must be inactive by having the field isActive set to false (`isActive=false`).

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |
**milestones_request_delete_body_inner** | [**Vec<models::MilestonesRequestDeleteBodyInner>**](MilestonesRequestDeleteBodyInner.md) | Delete the given Milestone Id | [required] |

### Return type

[**models::MilestoneResponseDelete**](MilestoneResponseDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_milestones_by_poam_id

> models::MilestoneResponseGet get_system_milestones_by_poam_id(system_id, poam_id, scheduled_completion_date_start, scheduled_completion_date_end)
Get milestones in one or many POA&M items in a system

Returns system containing milestones for matching parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |
**scheduled_completion_date_start** | Option<**String**> | **Date Started**: Filter query by the scheduled completion start date (Unix date format). |  |
**scheduled_completion_date_end** | Option<**String**> | **Date Ended**: Filter query by the scheduled completion start date (Unix date format). |  |

### Return type

[**models::MilestoneResponseGet**](MilestoneResponseGet.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_milestones_by_poam_id_and_milestone_id

> models::MilestoneResponseGetMilestone get_system_milestones_by_poam_id_and_milestone_id(system_id, poam_id, milestone_id)
Get milestone by ID in POA&M item in a system

Returns systems containing milestones for matching parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |
**milestone_id** | **i32** | **Milestone Id**: The unique milestone record identifier. | [required] |

### Return type

[**models::MilestoneResponseGetMilestone**](MilestoneResponseGetMilestone.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_milestone_by_system_id_and_poam_id

> models::MilestoneResponsePut update_milestone_by_system_id_and_poam_id(system_id, poam_id, serde_json_colon_colon_value)
Update one or many POA&M items in a system

Updates a milestone for given `systemId` and `poamId` path parameters  **Request Body Required Fields** - `milestoneId` - `description` - `scheduledCompletionDate`

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |
**serde_json_colon_colon_value** | [**Vec<serde_json::Value>**](SerdeJson__Value.md) | Example request body for updating milestones (milestoneId) of an existing System (systemId) POA&M (poamId) | [required] |

### Return type

[**models::MilestoneResponsePut**](MilestoneResponsePut.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

