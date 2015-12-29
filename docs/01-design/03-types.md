# Types
In the last [pointers](02-pointers) chapter, we discussed the keys in the Ardite key/value pair system. Now it‘s time to discuss the values. A value could be one of a few types, these types are derived from common primitive types in various languages:

- Null: A single value, represents nothing.
- Boolean: Two values, true and false.
- Number: A numeric valuable, contains all real numbers (so integers and floats).
- String: Any number of characters.

However there are also a couple of special types:

- Reference: A pointer to another value in the database.

Before talking more about these special types, let‘s quickly modify our previous example. If you remember back, we had a key/value document which looked something like this:

```
/name = Caleb Meredith
/location = Boca Raton, Florida
/profile/about = I‘m a web developer!
/profile/likes/0 = Kittens
/profile/likes/1 = Star Wars
```

Let‘s add types to these key/value pairs. To do that we will use the JavaScript style of wrapping the value with a type. For example `Number(42)` is the number `42` whereas `String(42)` is the text `"42"`.

```
/name = String(Caleb Meredith)
/location = String(Boca Raton, Florida)
/profile/about = String(I‘m a web developer!)
/profile/likes/0 = String(Kittens)
/profile/likes/1 = String(Star Wars)
```

Better. If you don‘t know what a tuple is, it‘s just a set of a defined number of values of the same type. So in this case our tuple could be defined as: `(pointer, value, type)`.

## Reference
When your database starts to get large, you are likely going to want to reference some bits of data from other bits of data. This is what traditional relational databases excel at.

Let‘s look at another bit of JSON to understand how Ardite references work:

```json
{
  "people": [
    { "name": "Sara Smith" },
    { "name": "John Smith" },
    { "name": "Budd Deey" }
  ],
  "posts": [
    { "title": "Hello, world!", "author": 0 },
    { "title": "Lorem Ipsum.", "author": 2 }
  ]
}
```

Flattened into our above tuple format that looks something like:

```
/people/0/name = String(Sara Smith)
/people/1/name = String(John Smith)
/people/2/name = String(Budd Deey)
/posts/0/title = String(Hello, world!)
/posts/0/author = Number(0)
/posts/1/title = String(Lorem Ipsum.)
/posts/1/author = Number(2)
```

The number `0` at `/posts/0/author` is supposed to be a reference to `/people/0`, but up until this point there has been no good way to define a reference so we just use the number `0` and hope our data consumer understands what that means. However, in Ardite there is a better way, consider the following tuples instead:

```
/people/0/name = String(Sara Smith)
/people/1/name = String(John Smith)
/people/2/name = String(Budd Deey)
/posts/0/title = String(Hello, world!)
/posts/0/author = Reference(/people/0)
/posts/1/title = String(Lorem Ipsum.)
/posts/1/author = Reference(/people/2)
```

Note how we changed `/posts/0/author` and `/posts/1/author` to a reference with a pointer value. This now means anyone trying to query `/posts/1/author`, for example, will now get `/people/2`! The really exciting part is if you tried to get data with Ardite at the pointer `/posts/1/author/name` you would get the value `"Budd Deey"`.
