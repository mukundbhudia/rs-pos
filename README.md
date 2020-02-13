# A Rust Point of Sale system #

### Introduction ###
Our supermarket sells three items, each of which have a code and a price. Multibuy offers are applicable to some items. Your objective is to implement our Point of Sale (POS) system using Rust.

### Pricing ###
| Item | Code | Price (each, in pence) | Offer |
|--------|------|------------------------|----------------------|
| Apple |  A | 25 | Buy two get one free |
| Banana | B | 40 | Buy three for £1 |
| Peach | P | 30 |  |

### Prerequisites ###
Rust 1.41.0 or later installed.

### Tasks ###
* Create a file named `pos.rs` containing a Rust function named `checkout` that takes a list of item codes and their current prices and returns the total price in pence, after applying any relevant offers. For example `checkout(['B', 'A', 'B', 'P', 'B'], {A: 25, B: 40, P: 30})` should return `155`. You can assume the offers themselves are long-term and therefore do not need to be configurable.

* Create a file named `pos-test.rs` that verifies the behaviour of  the `checkout` function by invoking it with a list of items and their prices and asserting that the correct total is returned. 

* Add an object type `Checkout` to `pos.rs` that can constructed as `new Checkout({A: 25, B: 40, P: 30})` and that provides two methods: `scan(itemCode)` and `total()`. `total()` should be callable at any time to obtain a running total for the previously scanned items, after applying any relevant offers. Add test(s) for `Checkout` to `pos-test.rs`.
