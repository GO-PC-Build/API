# GO-PC Build Official API

## About

This API was created as backend for the project on GO-AO (GO-PC Build). Which hosts workshops on how you can
successfully and professionally build your own computers and install an operating system.

## Documentation

### Endpoints

<details>
<summary>/status</summary>

#### About

This route can be used to check if the API is up. No authorization is required for this route.

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
<summary>/auth/register</summary>

#### About

This endpoint provides a way for new users to create an account.

#### Request Body

```json
{
  "username": "string",
  "email": "string",
  "password": "string",
  "first_name": "string",
  "last_name": "string"
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
  "error": "No 'Authorization' header was present on the request"
}
```

</details>

### Headers

<details>
<summary>Authorization</summary>

#### Format

|      Name     | Value  |
|:-------------:|:------:|
| Authorization | string |


##### Example

|      Name     |                                   Value                                   |
|:-------------:|:-------------------------------------------------------------------------:|
| Authorization | GsRl67eiDZt4oskOmJqFa256okMu6aNDSHVmJRJSsEv6koS9jfn9M8aelIZM92GA.qljU4k7k |

</details>

## License

This application is under a [CCO 1.0 License](./LICENSE).
