# wbtimeline Contribution Guide

Do you have something to contribute to the wbtimeline but don't know where to start?
Well, you've come to the right place.

If you just want to add an item to the timeline,
simply [submit a form](https://github.com/developomp/wbtimeline/issues/new?assignees=&labels=new+data&template=new_data.yml) with the necessary information,
and the developers can add them for you!

If you have some development skills, and want to work on the code that powers the website itself,
we've got you covered for that too. Just continue reading.

## How to set up wbtimeline

### Pre-requirements

It is expected that you are familiar with web development and the [rust](https://www.rust-lang.org) programming language.
You should also be somewhat familiar with the [Node.JS](https://nodejs.org) ecosystem and the basics of [Deno](https://deno.land).

Install the following tools to get started with wbtimeline.

- [deno](https://deno.land)
- [yarn](https://yarnpkg.com)
- [cargo](https://doc.rust-lang.org/cargo)

## Commands

wbtimeline uses package.json scripts for project management.
Simply run `yarn <script identifier>` to run a command.

### Installing dependencies

This command performs the initial setup for development and building.
You only have to run this once.

```
yarn setup
```

### Generating data

This command executes the [`generate.ts` script](./generate.ts) which takes the markdown files in the [data directory](./data/) and converts them into a json file which can then be used by the website.
You probably never have to use this command since it is automatically executed during the building process.

```bash
yarn gen
```

### Building for production

This command builds an optimized, production-ready site in the `dist` directory which can directly be pushed to production.

```bash
yarn build
```

### Starting local server

This command starts a local server at http://localhost:3000 which serves the built website.

```bash
yarn start
```
