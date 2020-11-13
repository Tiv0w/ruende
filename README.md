# ruende

*RUst ENcoder & DEcoder*

**ruende** is a general-purpose compression system.


## How it works

It uses the LZW algorithm and a block-size of 2 bytes.

So if you give it a file without any potential to use the dictionary, you compress data by a whopping -100% amount!!!
*(It may sound incredible, but you effectively double the size of your input. Not bad for a first time, Billy!)*

## Usage

Encode data with this powerful command:
```sh
$ ruende SRC DEST
```

Decode data with this powerfuler command:
```sh
$ ruende SRC DEST -d
```

Read The Fricking Manual with this almighty command:
```sh
$ ruende -h
```
or its english and *certainly more distinguished* cousin:
```sh
$ ruende --help
```

## You're a hipster and you want to build it from source ?

Phase 1.
```sh
$ echo "At this point you should really know how to clone a Git project. \
Otherwise, learn it, you won't regret it."
$ cargo build --release
```

Phase 2.
?

Phase 3.
Profit

## Legal thingies
Obligatory legal chit-chat.

### Warranty
I like my softwares like I like my girls, insecure and full of vulnerabilities.

So this software comes with NO WARRANTY.

Read in details the terms of the license in the [COPYING](./COPYING) file.

### License

This software is licensed under the terms of the GNU GPLv3 license.

Please refer to the [COPYING](./COPYING) file for the full license.
