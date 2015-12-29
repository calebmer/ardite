# Reducing
When Ardite data is in the ledger it is not the most efficient to read. This is because every single fact must be applied in order for the data to be meaningful at a certain point in history. When every fact has been applied the data is thought to be in a “reduced” state.

We will use the following ledger example to explain two methods of reduction. Its the same one we used before except the second `/location` fact has been moved and another `/location` fact has been added to the end:

```
/name = String(Caleb Meredith)
/location = String(Boca Raton, Florida)
/profile/about = String(I‘m a web developer!)
/location = String(Chagrin Falls, Ohio)
/profile/likes/0 = String(Kittens)
/profile/likes/1 = String(Star Wars)
/location = String(Marrietta, Georgia)
```

## From the Tail
The first method of reduction is best when you want to see data in its current state. In order to do this you start at the tail of the ledger (in the example above, the `/name` fact) and you go down the ledger applying every single fact and replacing facts with the same key.

In the above example the algorithm would do the following:

1. Add `/name`, `/location`, and `/profile/about` to the reduced data.
2. Remove `/location` in the reduced data and add the value to this point.
3. Add `/profile/likes/0` and `/profile/likes/1` to the reduced data.
4. Remove `/location` in the reduced data and add the newest value.

So the final reduced data would look like:

```
/name = String(Caleb Meredith)
/profile/about = String(I‘m a web developer!)
/profile/likes/0 = String(Kittens)
/profile/likes/1 = String(Star Wars)
/location = String(Marrietta, Georgia)
```

This method is best when you want to get the state of the database to a certain point and you do not know the exact key you are searching for.

This method is the most accurate when reducing data, however it is also the slowest.

## From the Head
The second method of reduction only works when you know the exact key you are searching for. With this approach, you start at the head (the most recent fact) and search up the ledger for the key. As soon as you find the key, you return it. If you do not find a key, the value is null.

In the above example, finding `/location` would look like the following:

1. Look at the head, it is `/location`, return that value.

Easy! Obviously this approach will be most performant for values recently added to the ledger. Let‘s try the algorithm one more time, this time searching for `/profile/about`.

1. Skip `/location`, `/profile/likes/1`, `/profile/likes/0`, and `/location` in that order.
2. See `/profile/about`, return that value.

This method is also not very performant in large ledgers. For a super performant way to read data from Ardite, read the next design document on views.

One other behavior of the head reduction which is important to understand is that it cannot get all the key/value pairs in a higher hierarchy. For example, trying to reduce `/profile` from the head will *fail*. This is because we do not know when to stop. Trying to do this will throw an error as soon as the algorithm hits `/profile/likes/1` on the ledger (or another hierarchical value). This error can be preemptively caught when using a schema.

## Hierarchical Conflicts
This is a continuation of the discussion in the pointers design document discussing how data is reduced (with both methods) when there are hierarchical conflicts. Take the following contrived example (without types this time):

```
/foo = 1
/foo/bar = 2
/foo/qux = 3
/foo/bar = 4

/buz/qux = 5
/buz = 6
```

Now let‘s reduce it!

### From the Tail
A tail reduction would look like the following:

```
/foo/qux = 3
/foo/bar = 4
/buz = 6
```

When we see that `/foo/bar` should be set, not only do we remove old references to `/foo/bar` in our final result, but also references to `/foo`. Likewise, when we see that `/buz` was set, not only to remove old references to `/buz`, but also old references to its children like `/buz/qux`.

### From the Head
Asking for `/buz/qux` from the head would return null. This is because we immediately saw that `/buz` was set, which overrides its children.

Asking for `/foo` throws an error, because in order to find all the sub values of `/foo` we need a tail reduction.

It is important to remember that hierarchical conflicts will be much rarer in a situation where you implement a strict schema.
