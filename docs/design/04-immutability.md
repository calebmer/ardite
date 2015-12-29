# Immutability
Now we get to another exciting parts of Ardite. Immutability.

Immutability is a fancy functional programming term which basically means that any data you have can not be mutated. Data can *not* be changed.

At first that seems like a crazy concept for a database, wouldn‘t you (and your users) like to change data in the database? Of course you would. Good thing Ardite does let you do that (in an indirect way).

Whenever you “change” data in Ardite the old data doesn‘t actually ever go away. It is always stored in the Ardite “ledger”. Let’s learn more about this ledger and how it allows for immutability.

## Ledger
Let‘s take our example from before, we have the following data stored in our Ardite database:

```
/name = String(Caleb Meredith)
/location = String(Boca Raton, Florida)
/profile/about = String(I‘m a web developer!)
/profile/likes/0 = String(Kittens)
/profile/likes/1 = String(Star Wars)
```

If we tried to select `/location` we would get `"Boca Raton, Florida"`. But, uh oh, Caleb just moved to Ohio. We need to change his location field. Except, we can‘t, at least directly. Our data is immutable, remember? So we do the following:

```
/name = String(Caleb Meredith)
/location = String(Boca Raton, Florida)
/profile/about = String(I‘m a web developer!)
/profile/likes/0 = String(Kittens)
/profile/likes/1 = String(Star Wars)
/location = String(Chagrin Falls, Ohio)
```

See how we added one line at the end with the new location? We didn’t actually mutate our data, rather we changed the value in the final output by adding another key/value pair. Now if we try to select `/location` we first get the value `"Boca Raton, Florida"`, but then we see the latest key/value pair and we know `/location` is actually `"Chagrin Falls, Ohio"`.

Each line of the ledger is considered an immutable “fact”. It happened, that is a fact. Even if we change the data later, it doesn‘t change the fact that it was previously another value.

Also important to note is that each fact needs to have another value associated with it, the time which it was applied. This way the ledger can never be confused about which fact comes first in the rare occasion facts get scrambled up.

## Compaction
One feature of the ledger is that it can be compacted in file storage critical scenarios. Often you shouldn‘t need the ledger to be small, most of the time you will be only operating on the head (we will talk about this later in views). However, the ledger does provide the guarantee that if a new value with the same key appears on the ledger, it is the most recent value (no questions asked). Therefore, we could safely remove the old key/value pair.

Ardite does not compact the ledger at runtime, this would not be as performant. Ardite also does not recommend compacting the ledger without backing up the old facts somewhere. Because every fact has a time associated with it, if it were to be removed from the ledger it could be added back in at a later time for an audit of the database.
