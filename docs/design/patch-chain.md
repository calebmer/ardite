# Patch Chain
The patch chain is what Ardite actually stores. It is the immutable “facts” which are used to build the views. A patch chain may be visualized as follows:

```json
[
  { "op": "add", "path": "/", "value": { "baz": "qux", "foo": "bar" } },
  { "op": "replace", "path": "/baz", "value": "boo" },
  { "op": "add", "path": "/hello", "value": ["world"] },
  { "op": "remove", "path": "/foo"}
]
```

The reduced state of this patch chain would be:

```json
{
  "baz": "boo",
  "hello": ["world"]
}
```

The patch chain above was constructed (for illustrative purposes) using [client patches](patch#client-patches). However, they are actually stored in the [saved patch](path#saved-patches) format.

The “head” of the chain is the latest patch applied, whereas the “tail” is the very first patch applied. All other patches are ordered in increasing application order. So in the above example, the very first `add` operation would be the patch chain tail and the `remove` operation would be the patch chain head (since the `at` field is not included it is assumed it is increasing).

## Selection Guarantee
One interesting intrinsic ability of the patch chain is that it allows for fast selection for recently applied values. This is because when a user specifies a JSON pointer to get (like `/baz`) we only need to search for the most recent change of that path (in the above example we would start at the `remove`, last, operation and end at the `replace`, second, operation without needing to search the entire chain).

While this is not faster than an indexed search in a normalized data structure, it is an interesting mathematical guarantee.

## Speed
This implementation of atomic updates is naturally faster than traditional databases as appending to a file (patch chain file) will always be faster than random file access.
