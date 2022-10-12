# rssc-intract

## building
- install cargo and npm
- clone the repo
- run `build.sh`

## setting up database
install postgresql

switch to the postgres user and create a user with `createuser --pwprompt username`

create a database with `createdb -O username database`

set database url in config file to `postgres://username:password@localhost/database`

## running
when running from the command line and not specifying a config path
[confy](https://crates.io/crates/confy) will automatically look for a config file from your default location.
for testig purposes there is a default config in this git repo.
if you want to use this with cargo you can do `cargo run -- --c config.toml ${SUBCOMMAND}`

## about
### rssc
rssc stands for really simple syndication curation.
we mainly picked this name because it sounded funny.
it's the protocol that this server works with.
this part is dual licensed under the MIT and Apache license.
so feel free to implement the spec yourself!

### intract
intract is the server and a reference implementation of rssc.
we picked this name because of the concept of an intractable problem in computer science.
generally these problems are too hard to be solved by an algorithm, which is exactly what we're trying to replace.
