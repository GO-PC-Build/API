# GO-PC Build Official API

## About

This API was created as backend for the project on GO-AO (GO-PC Build). Which hosts workshops on how you can
successfully and professionally build your own computers and install an operating system.

## Documentation

### Endpoints

<details>
<summary>/status</summary>

#### About

This route can be used to check if the API is up.

#### Return type

`StatusResponse`

</details>

<details>
<summary>/auth/login</summary>

#### About

This endpoint can be used for existing users to receive a token with their credentials.

#### Request Body

```json
{
  "username": "string",
  "password": "string"
}
```

#### Return type

`TokenResponse`

</details>

<details>
<summary>/auth/extern/login</summary>

#### About

Login with a third party service.

#### Request Body

```json
{
  "value": "string (client id)",
  "token": "optional[Auth token, eg for discord]"
}
```

#### Return type

`TokenResponse`

#### Exception type

`BaseException`

</details>

<details>
<summary>/auth/register</summary>

#### About

This endpoint provides a way for new users to create an account.

#### Request Body

```json
{
  "username": "string",
  "email": "string",
  "password": "string"
}
```

#### Return type

`TokenResponse`

</details>

<details>
<summary>/auth/revoke</summary>

#### About

This endpoint permanently deletes a token from the database.

#### Request Header

`Authorization`

#### Return type

`StatusResponse`

#### Exception type

`BaseException`

</details>

<details>
<summary>/user/@me</summary>

#### About

This endpoint fetches the base data from a user.

#### Request Header

`Authorization`

#### Return type

`User`

#### Exception type

`BaseException`

</details>

<details>
<summary>/user/connect/{platform}</summary>

#### About

Connect an account with a third party platform.

#### Request Body

```json
{
  "value": "string"
}
```

#### Request Header

`Authorization`

#### Return type

`StatusResponse`

#### Exception type

`BaseException`

</details>

### Responses

#### Success

<details>
<summary>StatusResponse</summary>

#### Format:

```json
{
  "message": "string"
}
```

#### Example response:

```json
{
  "message": "API is fully operational!"
}
```

</details>

<details>
<summary>TokenResponse</summary>

#### Format:

```json
{
  "token": "string"
}
```

#### Example response:

```json
{
  "token": "GsRl67eiDZt4oskOmJqFa256okMu6aNDSHVmJRJSsEv6koS9jfn9M8aelIZM92GA.qljU4k7k"
}
```

</details>

<details>
<summary>User</summary>

#### Format:

```json
{
  "id": "string",
  "nickname": "string",
  "email": "string",
  "avatar": "string",
  "date": "string"
}
```

#### Example response:

```json
{
  "id": "123example321",
  "nickname": "example lord",
  "email": "example@example.com",
  "avatar": "http://cdn.example.com/pfp/123example321",
  "date": "2020-12-28T13:18:23.986284700+00:00"
}
```

</details>

#### Exceptions

<details>
<summary>BaseException</summary>

#### Format:

```json
{
  "message": "string",
  "error": "string"
}
```

#### Example response:

```json
{
  "message": "Oops... You did something wrong! (See error for more information)",
  "error": "No or an invalid 'Authorization' header was present on the request"
}
```

</details>

### Headers

<details>
<summary>Authorization</summary>

#### Format

| Name          | Value  |
|:-------------:|:------:|
| Authorization | string |

##### Example

| Name          | Value                                                                     |
|:-------------:|:-------------------------------------------------------------------------:|
| Authorization | GsRl67eiDZt4oskOmJqFa256okMu6aNDSHVmJRJSsEv6koS9jfn9M8aelIZM92GA.qljU4k7k |

</details>

## License

This application is under a [CCO 1.0 License](./LICENSE).
