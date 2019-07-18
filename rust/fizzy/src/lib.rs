use std::ops::Rem;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    matcher: Box<dyn Fn(T) -> bool>,
    subs: String
}

impl<T> Matcher<T> {
    pub fn new<F: 'static + Fn(T) -> bool, S: ToString>(matcher: F, subs: S) -> Matcher<T> {
        Matcher {
            matcher: Box::new(matcher),
            subs: subs.to_string()
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T: ToString + Copy>(Vec<Matcher<T>>);

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Fizzy(Vec::new())
    }

    // feel free to change the signature to `mut self` if you like
    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.0.push(matcher);
        self
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply<I: IntoIterator<Item=T>>(self, iter: I) -> impl Iterator<Item = String> {
        FizzyIter {
            iter: iter.into_iter(),
            matchers: self.0
        }
    }
}

struct FizzyIter<I, T> {
    iter: I,
    matchers: Vec<Matcher<T>>,
}

impl<I: Iterator<Item=T>, T: ToString + Copy> Iterator for FizzyIter<I, T> {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|n| {
            let mut out = String::new();
            for matcher in &self.matchers {
                if (matcher.matcher)(n) {
                    out += &matcher.subs;
                }
            }
            if out.is_empty() {
                n.to_string()
            } else {
                out
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: ToString + Rem<T, Output=T> + PartialEq<T> + From<u8> + Copy>() -> Fizzy<T> {
    Fizzy::<T>::new()
        .add_matcher(Matcher::new(|n| n % 3.into() == 0.into(), "fizz"))
        .add_matcher(Matcher::new(|n| n % 5.into() == 0.into(), "buzz"))
}
