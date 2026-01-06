## @vexillum/reqx@1.0.0

This generator creates TypeScript/JavaScript client that utilizes [axios](https://github.com/axios/axios). The generated Node module can be used in the following environments:

Environment
* Node.js
* Webpack
* Browserify

Language level
* ES5 - you must have a Promises/A+ library installed
* ES6

Module system
* CommonJS
* ES6 module system

It can be used in both TypeScript and JavaScript. In TypeScript, the definition will be automatically resolved via `package.json`. ([Reference](https://www.typescriptlang.org/docs/handbook/declaration-files/consumption.html))

### Building

To build and compile the typescript sources to javascript use:
```
npm install
npm run build
```

### Publishing

First build the package then run `npm publish`

### Consuming

navigate to the folder of your consuming project and run one of the following commands.

_published:_

```
npm install @vexillum/reqx@1.0.0 --save
```

_unPublished (not recommended):_

```
npm install PATH_TO_GENERATED_PACKAGE --save
```

### Documentation for API Endpoints

All URIs are relative to *http://localhost*

Class | Method | HTTP request | Description
------------ | ------------- | ------------- | -------------
*AuthenticationApi* | [**getCurrentUser**](docs/AuthenticationApi.md#getcurrentuser) | **GET** /v1/auth/me | Get current authenticated user information
*AuthenticationApi* | [**getKeys**](docs/AuthenticationApi.md#getkeys) | **GET** /v1/auth/keys | Get public JWT key for token verification
*AuthenticationApi* | [**login**](docs/AuthenticationApi.md#login) | **POST** /v1/auth/login | User login with email and password
*AuthenticationApi* | [**logout**](docs/AuthenticationApi.md#logout) | **POST** /v1/auth/logout | Logout user by clearing refresh token
*AuthenticationApi* | [**refreshToken**](docs/AuthenticationApi.md#refreshtoken) | **POST** /v1/auth/refresh | Refresh access token using refresh token cookie
*AuthenticationApi* | [**register**](docs/AuthenticationApi.md#register) | **POST** /v1/auth/register | User registration with email and password
*AuthenticationApi* | [**requestMagicLink**](docs/AuthenticationApi.md#requestmagiclink) | **POST** /v1/auth/magic-link/request | Request a magic link for passwordless authentication
*AuthenticationApi* | [**verifyMagicLink**](docs/AuthenticationApi.md#verifymagiclink) | **POST** /v1/auth/magic-link/verify | Verify magic link token and authenticate user
*HealthApi* | [**healthz**](docs/HealthApi.md#healthz) | **GET** /healthz | Health check endpoint
*HealthApi* | [**readyz**](docs/HealthApi.md#readyz) | **GET** /readyz | Readiness check endpoint - checks database and redis connectivity


### Documentation For Models

 - [AuthResponse](docs/AuthResponse.md)
 - [DataResponseAuthResponse](docs/DataResponseAuthResponse.md)
 - [DataResponseAuthResponseData](docs/DataResponseAuthResponseData.md)
 - [DataResponsePublicKeyResponse](docs/DataResponsePublicKeyResponse.md)
 - [DataResponsePublicKeyResponseData](docs/DataResponsePublicKeyResponseData.md)
 - [DataResponseUsers](docs/DataResponseUsers.md)
 - [DataResponseUsersData](docs/DataResponseUsersData.md)
 - [DataResponseValue](docs/DataResponseValue.md)
 - [HealthRes](docs/HealthRes.md)
 - [LoginRequest](docs/LoginRequest.md)
 - [MagicLinkRequest](docs/MagicLinkRequest.md)
 - [MagicLinkVerifyRequest](docs/MagicLinkVerifyRequest.md)
 - [PublicKeyResponse](docs/PublicKeyResponse.md)
 - [RegisterRequest](docs/RegisterRequest.md)
 - [UserRole](docs/UserRole.md)
 - [Users](docs/Users.md)
 - [XReq](docs/XReq.md)


<a id="documentation-for-authorization"></a>
## Documentation For Authorization

Endpoints do not require authorization.

