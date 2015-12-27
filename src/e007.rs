//! Testify
//!
//!   - **Date:** December 13, 2015
//!   - **Subject:** Testing and benchmarking, and compiler attributes.
//!   - **Audio:**
//!       + [M4A](http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e007.m4a)
//!       + [MP3](http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e007.mp3)
//!       + [Ogg](http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e007.ogg)
//!
//! <audio title="Modularize this!" controls preload=metadata>
//!   <source src="http://www.podtrac.com/pts/redirect.m4a/cdn.newrustacean.com/e007.m4a">
//!   <source src="http://www.podtrac.com/pts/redirect.mp3/cdn.newrustacean.com/e007.mp3">
//!   <source src="http://www.podtrac.com/pts/redirect.ogg/cdn.newrustacean.com/e007.ogg">
//! </audio>
//!
//! Notes
//! -----
//!
//! All about testing in Rust! In order, we take a look at:
//!
//!   - Why you need tests.
//!   - Unit tests in other (dynamically-typed) languages vs. in Rust.
//!   - How to write unit tests in Rust.
//!   - How and why to write integration tests in Rust.
//!   - How and why to use benchmarks in Rust.
//!
//! The detailed code samples for this episode are heavy on showing; because of
//! the nature of test functions, you will be best off just [reading the source]
//! rather than leaning heavily on the descriptions generated by **rustdoc**.
//! (The descriptions are still *there*, but they're much less useful than they
//! have been in previous episodes.) In particular, the `test` module here is
//! excluded because of the use of the `#[cfg(test)]` attribute marker on it.
//!
//! [reading the source]: /src/show_notes/e007.rs.html
//!
//! Because we are using the feature-gated benchmarking functionality, the
//! show notes "library" can now only be compiled with the Rust nightly (as of
//! 1.5, the version current as this episode is produced).
//!
//! One thing that isn't necessarily obvious from reading the test documentation
//! in the Rust book and Rust reference: the `extern crate test` statement needs
//! to be not in this module, but at the module (`lib.rs`) which defines the
//! library/crate; in this case, `show_notes/lib.rs`.
//!
//!
//! Links
//! -----
//!
//! - Rust Book:
//!     + [Testing][links-1]
//!     + [Attributes][links-2]
//!     + [Benchmark tests][links-3]
//! - Rust reference: [Attributes][links-4]
//! - [Diesel (Rust ORM)][links-5]
//!     + [31: Oxidizing an ORM][links-6]
//!     + [32: Bug for Bug Compatibility][links-7]
//!
//! [links-1]: https://doc.rust-lang.org/book/testing.html
//! [links-2]: https://doc.rust-lang.org/book/attributes.html
//! [links-3]: https://doc.rust-lang.org/book/benchmark-tests.html
//! [links-4]: https://doc.rust-lang.org/reference.html#attributes
//! [links-5]: https://github.com/sgrif/diesel
//! [links-6]: http://bikeshed.fm/31
//! [links-7]: http://bikeshed.fm/32
//!
//!
//! Sponsors
//! --------
//!
//!   - Chris Palmer
//!   - [Derek Morr][sponsors-2]
//!   - Luca Schmid
//!   - Micael Bergeron
//!   - Ralph Giles ("rillian")
//!   - reddraggone9
//!   - [William Roe][sponsors-7]
//!
//! [sponsors-2]: https://twitter.com/derekmorr
//! [sponsors-7]: http://willroe.me
//!
//! ### Become a sponsor
//!
//!   - [Patreon](https://www.patreon.com/newrustacean)
//!   - [Venmo](https://venmo.com/chriskrycho)
//!   - [Dwolla](https://www.dwolla.com/hub/chriskrycho)
//!   - [Cash.me](https://cash.me/$chriskrycho)
//!
//!
//! Follow
//! ------
//!
//!   - New Rustacean:
//!       + Twitter: [@newrustacean](https://www.twitter.com/newrustacean)
//!       + App.net: [@newrustacean](https://alpha.app.net/newrustacean)
//!       + Email: [hello@newrustacean.com](mailto:hello@newrustacean.com)
//!   - Chris Krycho
//!       + Twitter: [@chriskrycho](https://www.twitter.com/chriskrycho)
//!       + App.net: [@chriskrycho](https://alpha.app.net/chriskrycho)


/// A trivial function for a trivial test. See the [source](/src/show_notes/e007.rs.html)!
pub fn add(a: f64, b: f64) -> f64 {
    a + b
}

/// A trivial test of a trivial function, demonstrating `#[test]`.
///
/// This test function will not be compiled into a binary; it will *only* be
/// used with the test binary. Note that you *can* create standalone test
/// functions as long as they are marked with the `#[test]` attribute, but in
/// general, you should put it in the `test` module, see below.
#[test]
fn test_add() {
    assert_eq!(add(2.0, 2.0), 4.0);
}


/// A test module!
///
/// A few comments:
///
///   - Note the `#[cfg(test)]` attribute marker on the module.
///   - Note the distinction between `#[test]` and `#[bench]`.
#[cfg(test)]
mod tests {
    // This statement gives us access to everything in the parent module, so we
    // don't have to use the full namespace (`show_notes::e007::add`) to get
    // access to the functions we want to test.
    use super::*;

    // `Bencher` is the `struct` which has the benchmarking functionality.
    use test::Bencher;

    // We'll use this for demonstrating benchmarks later.
    use std::thread::sleep;
    use std::time::Duration;

    /// Another, equally trivial, test, this one for `#[should_panic]`.
    ///
    /// In a more meaningful scenario, we might use the `#[should_panic]` attribute
    /// to verify that a given function call triggers an error under conditions
    /// where it should, e.g. if we tried to `.unwrap` a value which didn't exist.
    #[test]
    #[should_panic]
    fn test_add_badly() {
        assert_eq!(add(2.0, 2.0), 5.0);
    }


    /// A yet more sophisticated example: `#[should_panic]` with `expected`.
    ///
    /// As the Rust book comments:
    ///
    /// > ...it's hard to guarantee that the test didn't fail for an unexpected
    /// > reason...
    ///
    /// The `#[should_panic]` annotation has an `expected` attribute.
    #[test]
    #[should_panic(expected = "Crazed monkeys!")]
    fn test_will_panic() {
        panic!("Crazed monkeys!");
    }

    /// Benchmark our addition function.
    ///
    /// Note: it's trivial, so it's probably pretty quick (`0 ns/iter (+/- 0)`).
    /// The point is simply that it does what it says on the tin.
    #[bench]
    fn demonstrate_benchmarking(bencher: &mut Bencher) {
        bencher.iter(|| add(2.0, 2.0));
    }

    /// We can also have secondary functions used to help with testing.
    ///
    /// This particular function is *stupid*; the way to do this, of course, is
    /// just to get the Duration directly. The only reason to have it here is to
    /// show that (1) `support_function()` doesn't end up in the compiled
    /// library, which you can check by inspecting the binary; and (2) that it
    /// is available for use with the benchmarker below.
    fn support_function(ns: u32) -> Duration {
        Duration::new(0, ns)
    }

    /// Benchmark a function that sleeps for 1ms every time you call it.
    ///
    /// One of the things this highlights: we have a *tiny* duration (10 ns)...
    /// and the test takes much, *much* longer. (I'm going to discuss this with
    /// the Rust community, because I don't actually understand it yet!)
    #[bench]
    fn demonstrate_benchmarking_with_sleep(bencher: &mut Bencher) {
        let duration = support_function(10);
        bencher.iter(|| sleep(duration));
    }
}
