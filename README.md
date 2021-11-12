
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

```
kwarf: !q https://open.spotify.com/track/4cOdK2wGLETKBW3PvgPWqT?si=d351ea353d404342
spotbot: Queued: Rick Astley - Never Gonna Give You Up
```

# Running

I recommend running `spotbot` using my Docker image unless you have a specific reason not to. The following
`docker-compose.yml` can be used as a starting point where you replace the
[environment variables](#environment-variables) with your own.

```yml
version: '3'
services:
  spotbot:
    image: kwarf/spotbot
    volumes:
      - ./spotbot:/spotbot
    environment:
      - MATRIX_HOMESERVER_URL=https://matrix-client.matrix.org
      - MATRIX_USERNAME=...
      - MATRIX_PASSWORD=...
      - MATRIX_ROOM_ID=...
      - RSPOTIFY_CLIENT_ID=...
      - RSPOTIFY_CLIENT_SECRET=...
      - RSPOTIFY_REDIRECT_URI=http://localhost:8000/callback
```

Since the Spotify API uses OAuth you will have to run the container interactively the first time to generate a token.

```sh
docker compose run -i spotbot
```

You will be prompted to navigate to a Spotify URL, copy this and then paste it into your browser. This will show a
prompt asking you to allow `spotbot` to access your account, with the permissions shown on that page.

Click _Agree_ and you will be redirected to a URL that will not be found _(localhost:8000)_. This is fine and expected,
simply copy that URL from your browsers address bar and paste it into the running container where it's promping