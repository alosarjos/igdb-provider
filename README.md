# IGDB Provider [![builds.sr.ht status](https://builds.sr.ht/~alosarjos/igdb-provider.svg)](https://builds.sr.ht/~alosarjos/igdb-provider?)


This crate is intended to fetch data from the IGDB API as deserialized objects.

## How to use it

First you will need a Twitch Client ID and Secret. You can get both from https://dev.twitch.tv/ for free.

Once you have them, you create an `APIAuth` object either passing both variables or by setting them as environment vars (`TWITCH_CLIENT_ID` and `TWITCH_CLIENT_SECRET`).

Call to the `request_token` method to get the OAuth data required for the client requests.

Create a `Client` object and make requests for the supported models. (More queries and options to come on the future)

### Example

```rust
    let mut auth = APIAuth::new_from_env().unwrap();
    auth.request_token().await.unwrap();

    let client = Client::new(auth);
    let games: Vec<Game> = client.query_by_name("The Witcher 3").await.unwrap();
```

## What is working

You can either request `Collections` by ID or `Games` by ID or name.

## Note

Items can't be filled recursively to avoid infinite recursion. Reference to another games or content inside a `Game` or a `Collection` is offered as the `ID` of the item and not it's data.