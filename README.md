# GO-PC Build Official API

## About

This API was created as backend for the project on GO-AO (GO-PC Build). 
Which hosts workshops on how you can successfully and professionally build your
own computers and install an operating system.


## Documentation

### Endpoints

<details>
<summary>/status</summary>

#### About

This route can be used to check if the API is up.
No authorization is required for this route.

#### Return type

`StatusResponse`
</details>

### Responses

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
