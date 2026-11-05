# \PoamApi

All URIs are relative to *http://localhost:4010*

Method | HTTP request | Description
------------- | ------------- | -------------
[**add_poam_by_system_id**](PoamApi.md#add_poam_by_system_id) | **POST** /api/systems/{systemId}/poams | Add one or many POA&M items in a system
[**delete_poam**](PoamApi.md#delete_poam) | **DELETE** /api/systems/{systemId}/poams | Remove one or many POA&M items in a system
[**get_system_poams**](PoamApi.md#get_system_poams) | **GET** /api/systems/{systemId}/poams | Get one or many POA&M items in a system
[**get_system_poams_by_poam_id**](PoamApi.md#get_system_poams_by_poam_id) | **GET** /api/systems/{systemId}/poams/{poamId} | Get POA&M item by ID in a system
[**update_poam_by_system_id**](PoamApi.md#update_poam_by_system_id) | **PUT** /api/systems/{systemId}/poams | Update one or many POA&M items in a system



## add_poam_by_system_id

> models::PoamResponsePostPutDelete add_poam_by_system_id(system_id, poam_required_fields)
Add one or many POA&M items in a system

Add a POA&M for given `systemId`  **Request Body Required Fields** <table>   <thead>     <tr><th><b>Field</b></th><th><b>Require/Condition</b></th></tr>   </thead>   <tbody>     <tr><td><code>status</code></td><td>Always (every POST)</td></tr>     <tr><td><code>vulnerabilityDescription</code></td><td>Always (every POST)</td></tr>     <tr><td><code>sourceIdentifyingVulnerability</code></td><td>Always (every POST)</td></tr>     <tr><td><code>pocOrganization</code></td><td>Always (every POST)</td></tr>     <tr><td><code>resources</code></td><td>Always (every POST)</td></tr>     <tr><td><code>identifiedInCFOAuditOrOtherReview</code></td><td>Required for VA. Optional for Army and USCG.</td></tr>     <tr><td><code>scheduledCompletionDate</code></td><td>Required for ongoing and completed POA&M items</td></tr>     <tr><td><code>pocFirstName</code></td><td>Only if Last Name, Email, or Phone Number have data</td></tr>     <tr><td><code>pocLastName</code></td><td>Only if First Name, Email, or Phone Number have data</td></tr>     <tr><td><code>pocEmail</code></td><td>Only if First Name, Last Name, or Phone Number have data</td></tr>     <tr><td><code>pocPhoneNumber</code></td><td>Only if First Name, Last Name, or Email have data</td></tr>     <tr><td><code>completionDate</code></td><td>For completed POA&M Item only</td></tr>     <tr><td><code>comments</code></td><td>For completed or Risk Accepted POA&M Items only</td></tr>   </tbody> </table>  **NOTE**: Certain eMASS instances also require the Risk Analysis fields to be populated:   - `severity`   - `relevanceOfThreat`   - `likelihood`   - `impact`   - `residualRiskLevel`   - `mitigations`  </br> **Business Rules**  The following rules apply to the Review Status `status` field value: <table>   <thead><tr><th><b>Value</b></th><th><b>Rule</b></th></tr></thead>   <tbody>     <tr><td><b>Not Approved</b></td><td>POA&M cannot be saved if Milestone Scheduled Completion Date exceeds POA&M Item Scheduled Completion Date</td></tr>     <tr><td><b>Approved</b></td><td>POA&M can only be saved if Milestone Scheduled Completion Date exceeds POA&M Item Scheduled Completion Date</td></tr>     <tr><td></td><td>Are required to have a Severity Value assigned</td></tr>     <tr><td><b>Completed</b> or <b>Ongoing</b></td><td>Cannot be saved without Milestones</td></tr>     <tr><td><b>Risk Accepted</b></td><td>POA&M Item cannot be saved with a Scheduled Completion Date <code>scheduledCompletionDate</code> or have Milestones</td></tr>     <tr><td><b>Approved</b> or <b>Completed</b> or <b>Ongoing</b></td><td>Cannot update Scheduled Completion Date</td></tr>  </tbody> </table>  **Additional Rules** - POA&M Item cannot be saved if associated Security Control or AP is inherited. - Completed POA&M Item cannot be saved if Completion Date (`completionDate`) is in the future. - POA&M Items cannot be updated if they are included in an active package. - Archived POA&M Items cannot be updated. - POA&M Items with a status of \"Not Applicable\" will be updated through test result creation. - If the Security Control or Assessment Procedure does not exist in the system, the POA&M Item maybe imported at the System Level.   **Fields Characters Limitation** - POA&M Item cannot be saved if the Point of Contact (POC) fields exceed 100 characters:   - `pocOrganization` `pocFirstName`, `pocLastName`, `pocEmail`, `pocPhoneNumber` - POA&M Item cannot be saved if Resources (`resource`) field exceeds 250 characters - POA&M Item cannot be saved if the following fields exceeds 2,000 characters:   - `mitigations`, `sourceIdentifyingVulnerability`, `comments`   - Milestones Field: `description` - POA&M Items cannot be saved if Milestone Description (`description`) exceeds 2,000 characters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_required_fields** | [**Vec<models::PoamRequiredFields>**](PoamRequiredFields.md) | Example request body to add POA&M(s) to a system (systemId) | [required] |

### Return type

[**models::PoamResponsePostPutDelete**](PoamResponsePostPutDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_poam

> models::PoamResponsePostPutDelete delete_poam(system_id, poam_request_delete_body_inner)
Remove one or many POA&M items in a system

Remove the POA&M matching `systemId` path parameter and `poamId` Request Body<br>

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_request_delete_body_inner** | [**Vec<models::PoamRequestDeleteBodyInner>**](PoamRequestDeleteBodyInner.md) | Delete the given POA&M Id | [required] |

### Return type

[**models::PoamResponsePostPutDelete**](PoamResponsePostPutDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_poams

> models::PoamResponseGetSystems get_system_poams(system_id, scheduled_completion_date_start, scheduled_completion_date_end, control_acronyms, assessment_procedures, ccis, system_only)
Get one or many POA&M items in a system

Returns system(s) containing POA&M items for matching parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**scheduled_completion_date_start** | Option<**String**> | **Date Started**: Filter query by the scheduled completion start date (Unix date format). |  |
**scheduled_completion_date_end** | Option<**String**> | **Date Ended**: Filter query by the scheduled completion start date (Unix date format). |  |
**control_acronyms** | Option<**String**> | **Control Acronym**: Filter query by given system acronym (single value or comma separated). |  |
**assessment_procedures** | Option<**String**> | **Assessment Procedure**: Filter query by given Security Control Assessment Procedure (single value or comma separated). |  |
**ccis** | Option<**String**> | **CCI System**: Filter query by Control Correlation Identifiers (CCIs) (single value or comma separated). |  |
**system_only** | Option<**bool**> | **Systems Only**: Indicates that only system(s) information is retrieved. |  |[default to true]

### Return type

[**models::PoamResponseGetSystems**](PoamResponseGetSystems.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_system_poams_by_poam_id

> models::PoamResponseGetPoams get_system_poams_by_poam_id(system_id, poam_id)
Get POA&M item by ID in a system

Returns system(s) containing POA&M items for matching parameters.

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_id** | **i32** | **POA&M Id**: The unique POA&M record identifier. | [required] |

### Return type

[**models::PoamResponseGetPoams**](PoamResponseGetPoams.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## update_poam_by_system_id

> models::PoamResponsePostPutDelete update_poam_by_system_id(system_id, poam_ids)
Update one or many POA&M items in a system

Update a POA&M for given `systemId`<br>  **Request Body Required Fields** <table>   <thead>     <tr><th><b>Field</b></th><th><b>Require/Condition</b></th></tr>   </thead>   <tbody>     <tr><td><code>poamId</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>displayPoamId</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>status</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>vulnerabilityDescription</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>sourceIdentifyingVulnerability</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>pocOrganization</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>resources</code></td><td>Always (every PUT)</td></tr>     <tr><td><code>identifiedInCFOAuditOrOtherReview</code></td><td>Required for VA. Optional for Army and USCG.</td></tr>     <tr><td><code>scheduledCompletionDate</code></td><td>Required for ongoing and completed POA&M items</td></tr>     <tr><td><code>pocFirstName</code></td><td>Only if Last Name, Email, or Phone Number have data</td></tr>     <tr><td><code>pocLastName</code></td><td>Only if First Name, Email, or Phone Number have data</td></tr>     <tr><td><code>pocEmail</code></td><td>Only if First Name, Last Name, or Phone Number have data</td></tr>     <tr><td><code>pocPhoneNumber</code></td><td>Only if First Name, Last Name, or Email have data</td></tr>     <tr><td><code>completionDate</code></td><td>For completed POA&M Item only</td></tr>     <tr><td><code>comments</code></td><td>For completed or Risk Accepted POA&M Items only</td></tr>   </tbody> </table>  **NOTES**: - Certain eMASS instances also require the Risk Analysis fields to be populated:   - `severity`   - `relevanceOfThreat`   - `likelihood`   - `impact`   - `residualRiskLevel`   - `mitigations` - To prevent uploading duplicate/undesired milestones through the POA&M PUT include an `isActive` field for the milestone and set it to equal to false `(isActive=false)`. </br>  **Business Rules:** See business rules for the POST endpoint

### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**system_id** | **i32** | **System Id**: The unique system record identifier. | [required] |
**poam_ids** | [**Vec<models::PoamIds>**](PoamIds.md) | Example request body for updating a POA&M for a system (systemId) | [required] |

### Return type

[**models::PoamResponsePostPutDelete**](PoamResponsePostPutDelete.md)

### Authorization

[apiKey](../README.md#apiKey), [mockType](../README.md#mockType), [userId](../README.md#userId)

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

