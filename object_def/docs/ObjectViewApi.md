# \ObjectViewApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_view**](ObjectViewApi.md#delete_object_view) | **DELETE** /v1.0/object-views/{objectViewId} | 
[**delete_object_view_batch**](ObjectViewApi.md#delete_object_view_batch) | **DELETE** /v1.0/object-views/batch | 
[**get_object_definition_by_external_reference_code_object_views_page**](ObjectViewApi.md#get_object_definition_by_external_reference_code_object_views_page) | **GET** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-views | 
[**get_object_definition_object_views_page**](ObjectViewApi.md#get_object_definition_object_views_page) | **GET** /v1.0/object-definitions/{objectDefinitionId}/object-views | 
[**get_object_view**](ObjectViewApi.md#get_object_view) | **GET** /v1.0/object-views/{objectViewId} | 
[**post_object_definition_by_external_reference_code_object_view**](ObjectViewApi.md#post_object_definition_by_external_reference_code_object_view) | **POST** /v1.0/object-definitions/by-external-reference-code/{externalReferenceCode}/object-views | 
[**post_object_definition_object_view**](ObjectViewApi.md#post_object_definition_object_view) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-views | 
[**post_object_definition_object_view_batch**](ObjectViewApi.md#post_object_definition_object_view_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-views/batch | 
[**post_object_definition_object_views_page_export_batch**](ObjectViewApi.md#post_object_definition_object_views_page_export_batch) | **POST** /v1.0/object-definitions/{objectDefinitionId}/object-views/export-batch | 
[**post_object_view_copy**](ObjectViewApi.md#post_object_view_copy) | **POST** /v1.0/object-views/{objectViewId}/copy | 
[**put_object_view**](ObjectViewApi.md#put_object_view) | **PUT** /v1.0/object-views/{objectViewId} | 
[**put_object_view_batch**](ObjectViewApi.md#put_object_view_batch) | **PUT** /v1.0/object-views/batch | 



## delete_object_view

> delete_object_view(object_view_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_view_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_view_batch

> delete_object_view_batch(callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_by_external_reference_code_object_views_page

> crate::models::PageObjectView get_object_definition_by_external_reference_code_object_views_page(external_reference_code, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectView**](PageObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_definition_object_views_page

> crate::models::PageObjectView get_object_definition_object_views_page(object_definition_id, page, page_size, search, sort)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectView**](PageObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_view

> crate::models::ObjectView get_object_view(object_view_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_view_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectView**](ObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_by_external_reference_code_object_view

> crate::models::ObjectView post_object_definition_by_external_reference_code_object_view(external_reference_code, object_view)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_view** | Option<[**ObjectView**](ObjectView.md)> |  |  |

### Return type

[**crate::models::ObjectView**](ObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_view

> crate::models::ObjectView post_object_definition_object_view(object_definition_id, object_view)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**object_view** | Option<[**ObjectView**](ObjectView.md)> |  |  |

### Return type

[**crate::models::ObjectView**](ObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_view_batch

> post_object_definition_object_view_batch(object_definition_id, callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_definition_object_views_page_export_batch

> post_object_definition_object_views_page_export_batch(object_definition_id, search, sort, callback_url, content_type, field_names)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_definition_id** | **String** |  | [required] |
**search** | Option<**String**> |  |  |
**sort** | Option<**String**> |  |  |
**callback_url** | Option<**String**> |  |  |
**content_type** | Option<**String**> |  |  |
**field_names** | Option<**String**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_view_copy

> crate::models::ObjectView post_object_view_copy(object_view_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_view_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectView**](ObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_view

> crate::models::ObjectView put_object_view(object_view_id, object_view)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_view_id** | **String** |  | [required] |
**object_view** | Option<[**ObjectView**](ObjectView.md)> |  |  |

### Return type

[**crate::models::ObjectView**](ObjectView.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_view_batch

> put_object_view_batch(callback_url, body)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**callback_url** | Option<**String**> |  |  |
**body** | Option<**serde_json::Value**> |  |  |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json
- **Accept**: application/json

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

