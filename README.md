
# :radio: Spotbot :speaker::notes:

A Matrix bot to collaboratively queue music on Spotify.

# Usage

### !q

Invoking the command without any arguments will show the current playback queue.

```
kwarf: !q
spotbot:
    Vau Boy, S3RL - Break The Music
    Juju Rush - Catching Fire
    GHOST DATA - Become God
```

### !q _search term_

All arguments here are passed to the
[/search](https://developer.spotify.com/documentation/web-api/reference/#/operations/search) API, requesting **tracks**,
and `spotbot` will queue the first search result.

```
kwarf: !q Vertex - Get Down
spotbot: Queued: Vertex - Get Down
kwarf: !q fire and flames
spotbot: Queued: DragonForce - Through The Fire And Flames
```

### !q _track-URL_

If the search pattern matches the regular expression `https://open.spotify.com/track/([^\?]+)` that URL will be queued
immediately without performing a search. This can be useful if a user wants to queue a niche track that is hard to
search for.
