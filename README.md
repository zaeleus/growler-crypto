# growler-crypto

This is a quick and dirty program to map the colors of a particular image to
ASCII values. Manual verification is required.

## Prerequisites

  * [rust](https://www.rust-lang.org/) >= 1.3.0
  * [cargo](https://crates.io/) >= 0.4.0-nightly

## Usage

```
cargo run --release -- /path/to/image.png t_y t_o t_p t_g
```

where `t_y`, `t_o`, `t_p`, and `t_g` are the yellow, orange, purple, and green
threshold values, respectively. Good defaults values for these thresholds are
`60 65 20 40`.

An image will colored cells is saved to `out.png` of the working directory, and
an ASCII representation will be printed to `stdout`. Unknown values are marked
as a question mark (`?`).

### Example

  * `git clone https://github.com/zaeleus/growler-crypto`
  * `cd growler-crypto`
  * `cargo run --release -- tests/fixtures/goodwood.png 60 65 20 40`

![goodwood.png](http://i.imgur.com/yMZenQSm.png)
![out.png](http://i.imgur.com/Od6etHVm.png)
