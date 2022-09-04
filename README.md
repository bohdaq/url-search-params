# Welcome to url-search-params!
`url-search-params` provides the ability to create search params from HashMap and vice versa.

In [URL](https://en.wikipedia.org/wiki/URL) (web address) search params correspond to [query string](https://en.wikipedia.org/wiki/Query_string).

Keep in mind it works with the query string part of the URL, it is not intended to work on the whole URL by design.
As per specification, the question mark `?` URL delimiter is not part of a query string.



Also hash mark `#` url delimiter and fragment part of URL is not the parts of a query string.
In practice, it means, the fragment and preceding hash mark won't be sent in a request to a server.


## Features
1. Convert given string into a HashMap containing query string parameters as key-value pairs
2. Convert given HashMap into a query string



## Configuration
No additional configuration required.


## Demo

[Tests](https://github.com/bohdaq/url-search-params/blob/main/src/lib.rs)
are available in the repository.

## Documentation
Public functions definitions and usage can be found at [git repository](https://github.com/bohdaq/url-search-params/blob/main/src/lib.rs).


## Build
If you want to build `url-search-params` on your own, make sure you have [Rust installed](https://www.rust-lang.org/tools/install).

> $ cargo build


## Test
If you want to test `url-search-params`.

> $ cargo test


## Community
Contact me on [Discord](https://discordapp.com/users/952173191659393025/) where you can ask questions and share ideas. Follow the [Rust code of conduct](https://www.rust-lang.org/policies/code-of-conduct).

## Donations
If you appreciate my work and want to support it, feel free to do it via [PayPal](https://www.paypal.com/donate/?hosted_button_id=7J69SYZWSP6HJ).

