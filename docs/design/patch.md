# Patch
The patch objects are inspired by the JSON patch standard (RFC 6902). There are two types of patches to consider. One which is the patches sent by the client, which follows the patch standard exactly. The other is the patches physically saved by Ardite.

## Client Patches
This is the main type of patch a user will deal with. When stored a few extra keys will be added for identification. A client patch looks like the following:

```json
{
  "op": "add",
  "path": "/person/name",
  "value": "Caleb Meredith"
}
```

### Validation
Every single patch must be able to be validated on its own. This means data must not in a reduced state (the exception is the `test` operation).

## Saved Patches
A saved patch object looks like the following with different keys and values as is required by RFC 6902:

```json
{
  "at": 1451335810724,
  "op": "add",
  "path": "/person/name",
  "value": "Caleb Meredith"
}
```

Required fields in every patch object are as follows:

- `at`: The timestamp of the operation for ordering purposes. It is an integer representing milliseconds since the unix epoch.
- `op`: The operation to be performed with the following path and value payloads. Valid operations would be `add`, `remove`, `replace`, `copy`, and `move` which perform the operation defined by RFC 6902.
- `path`: The path upon which the operation will be applied. This should be a JSON pointer.

### Deviance from RFC 6902
When a client sends a patch to Ardite the standard is followed exactly, however when the patch is saved there are naturally some deviations from the JSON patch standard.

The first deviation is that Ardite adds context to the patch object. The original JSON patch standard mainly defines an atomic update without any regards to who, when, and where the patch was performed. For storage in Ardite, this context is required.

Another deviation is that Ardite does not internally support the `test` operation as it requires the data to be in a reduced state. Also, one failing `test` patch would mean all sub-sequential patches would not be applied. However, again the Ardite client accepts patches which define a `test` patch.
