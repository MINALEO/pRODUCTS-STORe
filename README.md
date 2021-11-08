
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