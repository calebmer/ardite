# Structure
The Ardite database is structured to operate like one very large JSON object. This object may have an attached [schema](schema) to ensure the object is the correct shape across patches.

## Pointers
The JSON pointer spec ([RFC 6901](http://tools.ietf.org/html/rfc6901)) is a perfect way to navigate JSON objects therefore it is how all data is selected in Ardite.

## References
Ardite is not a relational database, yet it is incredibly important that Ardite be able to reference data. Therefore, Ardite has adopted the [JSON reference](https://tools.ietf.org/html/draft-pbryan-zyp-json-ref-03) spec to store references. Take the following object for instance:

```json
{
  "test": {
    "$ref": "#/hello"
  },
  "hello": {
    "world": 42
  }
}
```

Using pointers, a reference can be directly obtained using a syntax like `/test/$ref` which would return `#/hello`. But a reference may also be traversed with a JSON pointer. For example: `/test/world` returns 42. The schema defines a special [references](schema#references) type.

## Special Data Structures
There are some special data structures which Ardite defines in its schema which the user may not redefine. Such structures are all prefixed with a `$`, the user may not define any data in her schema which starts with a `$`. The special data structures are as follows:

- `$schema`: The [schema](schema) object which is used to validate data appended to Ardite.
- `$roles`: The database [roles](roles). Defines permissions.
