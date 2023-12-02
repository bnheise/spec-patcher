# \ObjectFolderApi

All URIs are relative to *http://localhost:8080/o/object-admin*

Method | HTTP request | Description
------------- | ------------- | -------------
[**delete_object_folder**](ObjectFolderApi.md#delete_object_folder) | **DELETE** /v1.0/object-folders/{objectFolderId} | 
[**delete_object_folder_batch**](ObjectFolderApi.md#delete_object_folder_batch) | **DELETE** /v1.0/object-folders/batch | 
[**get_object_folder**](ObjectFolderApi.md#get_object_folder) | **GET** /v1.0/object-folders/{objectFolderId} | 
[**get_object_folder_by_external_reference_code**](ObjectFolderApi.md#get_object_folder_by_external_reference_code) | **GET** /v1.0/object-folders/by-external-reference-code/{externalReferenceCode} | 
[**get_object_folders_page**](ObjectFolderApi.md#get_object_folders_page) | **GET** /v1.0/object-folders | 
[**patch_object_folder**](ObjectFolderApi.md#patch_object_folder) | **PATCH** /v1.0/object-folders/{objectFolderId} | 
[**post_object_folder**](ObjectFolderApi.md#post_object_folder) | **POST** /v1.0/object-folders | 
[**post_object_folder_batch**](ObjectFolderApi.md#post_object_folder_batch) | **POST** /v1.0/object-folders/batch | 
[**post_object_folders_page_export_batch**](ObjectFolderApi.md#post_object_folders_page_export_batch) | **POST** /v1.0/object-folders/export-batch | 
[**put_object_folder**](ObjectFolderApi.md#put_object_folder) | **PUT** /v1.0/object-folders/{objectFolderId} | 
[**put_object_folder_batch**](ObjectFolderApi.md#put_object_folder_batch) | **PUT** /v1.0/object-folders/batch | 
[**put_object_folder_by_external_reference_code**](ObjectFolderApi.md#put_object_folder_by_external_reference_code) | **PUT** /v1.0/object-folders/by-external-reference-code/{externalReferenceCode} | 



## delete_object_folder

> delete_object_folder(object_folder_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_folder_id** | **String** |  | [required] |

### Return type

 (empty response body)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## delete_object_folder_batch

> delete_object_folder_batch(callback_url, body)


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


## get_object_folder

> crate::models::ObjectFolder get_object_folder(object_folder_id)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_folder_id** | **String** |  | [required] |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_folder_by_external_reference_code

> crate::models::ObjectFolder get_object_folder_by_external_reference_code(external_reference_code)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## get_object_folders_page

> crate::models::PageObjectFolder get_object_folders_page(page, page_size, search)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**page** | Option<**String**> |  |  |
**page_size** | Option<**String**> |  |  |
**search** | Option<**String**> |  |  |

### Return type

[**crate::models::PageObjectFolder**](PageObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: Not defined
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## patch_object_folder

> crate::models::ObjectFolder patch_object_folder(object_folder_id, object_folder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_folder_id** | **String** |  | [required] |
**object_folder** | Option<[**ObjectFolder**](ObjectFolder.md)> |  |  |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_folder

> crate::models::ObjectFolder post_object_folder(object_folder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_folder** | Option<[**ObjectFolder**](ObjectFolder.md)> |  |  |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## post_object_folder_batch

> post_object_folder_batch(callback_url, body)


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


## post_object_folders_page_export_batch

> post_object_folders_page_export_batch(search, callback_url, content_type, field_names)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**search** | Option<**String**> |  |  |
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


## put_object_folder

> crate::models::ObjectFolder put_object_folder(object_folder_id, object_folder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**object_folder_id** | **String** |  | [required] |
**object_folder** | Option<[**ObjectFolder**](ObjectFolder.md)> |  |  |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)


## put_object_folder_batch

> put_object_folder_batch(callback_url, body)


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


## put_object_folder_by_external_reference_code

> crate::models::ObjectFolder put_object_folder_by_external_reference_code(external_reference_code, object_folder)


### Parameters


Name | Type | Description  | Required | Notes
------------- | ------------- | ------------- | ------------- | -------------
**external_reference_code** | **String** |  | [required] |
**object_folder** | Option<[**ObjectFolder**](ObjectFolder.md)> |  |  |

### Return type

[**crate::models::ObjectFolder**](ObjectFolder.md)

### Authorization

No authorization required

### HTTP request headers

- **Content-Type**: application/json, application/xml
- **Accept**: application/json, application/xml

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

