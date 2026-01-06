# AuthenticationApi

All URIs are relative to *http://localhost*

|Method | HTTP request | Description|
|------------- | ------------- | -------------|
|[**getCurrentUser**](#getcurrentuser) | **GET** /v1/auth/me | Get current authenticated user information|
|[**getKeys**](#getkeys) | **GET** /v1/auth/keys | Get public JWT key for token verification|
|[**login**](#login) | **POST** /v1/auth/login | User login with email and password|
|[**logout**](#logout) | **POST** /v1/auth/logout | Logout user by clearing refresh token|
|[**refreshToken**](#refreshtoken) | **POST** /v1/auth/refresh | Refresh access token using refresh token cookie|
|[**register**](#register) | **POST** /v1/auth/register | User registration with email and password|
|[**requestMagicLink**](#requestmagiclink) | **POST** /v1/auth/magic-link/request | Request a magic link for passwordless authentication|
|[**verifyMagicLink**](#verifymagiclink) | **POST** /v1/auth/magic-link/verify | Verify magic link token and authenticate user|

# **getCurrentUser**
> DataResponseUsers getCurrentUser()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.getCurrentUser();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**DataResponseUsers**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Current user data |  -  |
|**401** | Unauthorized |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **getKeys**
> DataResponsePublicKeyResponse getKeys()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.getKeys();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**DataResponsePublicKeyResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Public key |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **login**
> DataResponseAuthResponse login(loginRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    LoginRequest
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let loginRequest: LoginRequest; //

const { status, data } = await apiInstance.login(
    loginRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **loginRequest** | **LoginRequest**|  | |


### Return type

**DataResponseAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Login successful |  -  |
|**401** | Invalid credentials |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **logout**
> DataResponseValue logout()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.logout();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**DataResponseValue**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Logged out successfully |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **refreshToken**
> DataResponseAuthResponse refreshToken()


### Example

```typescript
import {
    AuthenticationApi,
    Configuration
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

const { status, data } = await apiInstance.refreshToken();
```

### Parameters
This endpoint does not have any parameters.


### Return type

**DataResponseAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: Not defined
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Token refreshed |  -  |
|**401** | Refresh token not found or invalid |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **register**
> DataResponseAuthResponse register(registerRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    RegisterRequest
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let registerRequest: RegisterRequest; //

const { status, data } = await apiInstance.register(
    registerRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **registerRequest** | **RegisterRequest**|  | |


### Return type

**DataResponseAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Registration successful |  -  |
|**409** | User already exists |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **requestMagicLink**
> DataResponseValue requestMagicLink(magicLinkRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    MagicLinkRequest
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let magicLinkRequest: MagicLinkRequest; //

const { status, data } = await apiInstance.requestMagicLink(
    magicLinkRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **magicLinkRequest** | **MagicLinkRequest**|  | |


### Return type

**DataResponseValue**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Magic link sent |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

# **verifyMagicLink**
> DataResponseAuthResponse verifyMagicLink(magicLinkVerifyRequest)


### Example

```typescript
import {
    AuthenticationApi,
    Configuration,
    MagicLinkVerifyRequest
} from '@vexillum/reqx';

const configuration = new Configuration();
const apiInstance = new AuthenticationApi(configuration);

let magicLinkVerifyRequest: MagicLinkVerifyRequest; //

const { status, data } = await apiInstance.verifyMagicLink(
    magicLinkVerifyRequest
);
```

### Parameters

|Name | Type | Description  | Notes|
|------------- | ------------- | ------------- | -------------|
| **magicLinkVerifyRequest** | **MagicLinkVerifyRequest**|  | |


### Return type

**DataResponseAuthResponse**

### Authorization

No authorization required

### HTTP request headers

 - **Content-Type**: application/json
 - **Accept**: application/json


### HTTP response details
| Status code | Description | Response headers |
|-------------|-------------|------------------|
|**200** | Magic link verified |  -  |
|**401** | Invalid or expired magic link |  -  |

[[Back to top]](#) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to Model list]](../README.md#documentation-for-models) [[Back to README]](../README.md)

