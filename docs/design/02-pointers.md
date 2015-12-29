# Pointers
If you really wanted to, you could think of Ardite as a very very complex key/value store. At the end of the day, you are setting values to keys and retrieving from, again, a set of keys. Although there is a special format to Ardite keys. Pointers are Ardite keys, and the collection of pointers and values is the Ardite document.

Pointers take after a traditional file system in that they are nested and delimited by forward slashes (`/`). This allows data stored in Ardite to be hierarchical, an important feature. Each piece between slashes is referred to as a pointer segment.

To better understand how pointers work, take this JSON document for instance:

```json
{
  "name": "Caleb Meredith",
  "location": "Boca Raton, Florida",
  "profile": {
    "about": "I‘m a web developer!",
    "likes": [
      "Kittens",
      "Star Wars"
    ]
  }
}
```

A JSON document is just a complex set of key/value pairs as well, therefore it‘s helpful to us when trying to understand Ardite pointers. Let‘s see the key/value pairs that JSON document would be represented as in an Ardite database using pointers:

```
/name = Caleb Meredith
/location = Boca Raton, Florida
/profile/about = I‘m a web developer!
/profile/likes/0 = Kittens
/profile/likes/1 = Star Wars
```

Each JSON key became a pointer segment, and when there was a nested object it was flattened into another pointer segment. This flattened data format has many advantages, first it is super easy to update in an immutable way (as we will see later). But second it can be turned into a wide variety of hierarchal data formats like JSON (as we have seen), YAML, or TOML to name a few.

## Naming
There are a few limitations on what you can name your pointers. A pointer can consist of any letter or number character (a-z or 0-9) in addition to underscores (\_). A pointer, unlike many SQL databases, *is* case sensitive, so `givenName` is not the same as `givenname` (hooray for camel case!).

Special Ardite constructs will always be prefixed with a dollar sign (`$`). You cannot define a custom pointer using a dollar sign, sorry.

## Hierarchical Conflicts
When designing the shape of your Ardite document you need to consider the following scenario:

```
/profile = Budd
/profile/about = I‘m a web developer!
```

The above is completely possible, however how do you turn that back into JSON? Does the document look like this?

```json
{
  "profile": "Budd"
}
```

Like this?

```json
{
  "profile": {
    "about": "I‘m a web developer!"
  }
}
```

Like this?

```json
{
  "profile": "Budd",
  "profile": {
    "about": "I‘m a web developer!"
  }
}
```

As you can see, none of these interpretations are very helpful. When turning into JSON, Ardite will prefer the third format where the second value will be whatever is the most recent (recency will be better explained when we talk about the ledger).

Because of this weirdness, this scenario should be avoided unless there is a really good reason. Eventually, you will be able to use a schema to force this scenario to never happen.
