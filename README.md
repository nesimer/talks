# Talks

## Description

This repo contains some talks that I show during some tech events.

## Available talks

[Les generators: Simplifier le traitement des flux de données](./talks/generators.md)

## Commands

> **_BEFORE TO USE_**:
>
> You need to enable `corepack` to use `yarn`
>
> `corepack enable`
>
> Next you can install `yarn`
>
>`yarn install`

`yarn serve`: serve a server on `localhost:8080` that exposes all talks and assets

`yarn show [path to specific talk]`: generate html version of slides passed as argument (you need to open it with your browser)

> **NOTE** if you have a browser exec installed and want a pdf export do => `CHROME_PATH=$(which {your_browser}) yarn marp --pdf --allow-local-files talks/{wantedFile.md}`

`yarn marp [commands of marp-cli]`: exposes all available commands of `marp-cli`

## Lab

Some talks have demo, to run it:

```sh
yarn lab:start [subfolder name from lab/s]
```

### Test

Some tests can be found in demo:

```sh
yarn lab:test [subfolder name from lab/s]
```
