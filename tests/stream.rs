extern crate fluent;

use fluent::syntax::stream::parserstream;

#[test]
fn next() {
    let mut ps = parserstream("abcd".chars());

    assert_eq!(None, ps.current());
    assert_eq!(None, ps.get_index());

    assert_eq!(Some('a'), ps.next());
    assert_eq!(Some('a'), ps.current());
    assert_eq!(Some(0), ps.get_index());

    assert_eq!(Some('b'), ps.next());
    assert_eq!(Some('b'), ps.current());
    assert_eq!(Some(1), ps.get_index());

    assert_eq!(Some('c'), ps.next());
    assert_eq!(Some('c'), ps.current());
    assert_eq!(Some(2), ps.get_index());

    assert_eq!(Some('d'), ps.next());
    assert_eq!(Some('d'), ps.current());
    assert_eq!(Some(3), ps.get_index());

    assert_eq!(None, ps.next());
    assert_eq!(Some('d'), ps.current());
    assert_eq!(Some(3), ps.get_index());
}

#[test]
fn peek() {
    let mut ps = parserstream("abcd".chars());

    assert_eq!(None, ps.current_peek());
    assert_eq!(None, ps.get_peek_index());

    assert_eq!(Some('a'), ps.peek());
    assert_eq!(Some('a'), ps.current_peek());
    assert_eq!(Some(0), ps.get_peek_index());

    assert_eq!(Some('b'), ps.peek());
    assert_eq!(Some('b'), ps.current_peek());
    assert_eq!(Some(1), ps.get_peek_index());

    assert_eq!(Some('c'), ps.peek());
    assert_eq!(Some('c'), ps.current_peek());
    assert_eq!(Some(2), ps.get_peek_index());

    assert_eq!(Some('d'), ps.peek());
    assert_eq!(Some('d'), ps.current_peek());
    assert_eq!(Some(3), ps.get_peek_index());

    assert_eq!(None, ps.peek());
    assert_eq!(Some('d'), ps.current_peek());
    assert_eq!(Some(3), ps.get_peek_index());
}

#[test]
fn peek_and_next() {
    let mut ps = parserstream("abcd".chars());

    assert_eq!(Some('a'), ps.peek());
    assert_eq!(Some(0), ps.get_peek_index());
    assert_eq!(None, ps.get_index());

    assert_eq!(Some('a'), ps.next());
    assert_eq!(Some(0), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());

    assert_eq!(Some('b'), ps.peek());
    assert_eq!(Some(1), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());

    assert_eq!(Some('c'), ps.peek());
    assert_eq!(Some(2), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());

    assert_eq!(Some('b'), ps.next());
    assert_eq!(Some(1), ps.get_peek_index());
    assert_eq!(Some(1), ps.get_index());
    assert_eq!(Some('b'), ps.current());
    assert_eq!(Some('b'), ps.current_peek());

    assert_eq!(Some('c'), ps.next());
    assert_eq!(Some(2), ps.get_peek_index());
    assert_eq!(Some(2), ps.get_index());
    assert_eq!(Some('c'), ps.current());
    assert_eq!(Some('c'), ps.current_peek());

    assert_eq!(Some('d'), ps.peek());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(2), ps.get_index());

    assert_eq!(Some('d'), ps.next());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(3), ps.get_index());
    assert_eq!(Some('d'), ps.current());
    assert_eq!(Some('d'), ps.current_peek());

    assert_eq!(None, ps.peek());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(3), ps.get_index());
    assert_eq!(Some('d'), ps.current());
    assert_eq!(Some('d'), ps.current_peek());

    assert_eq!(None, ps.peek());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(3), ps.get_index());

    assert_eq!(None, ps.next());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(3), ps.get_index());
}

#[test]
fn skip_to_peek() {
    let mut ps = parserstream("abcd".chars());

    ps.peek();
    ps.peek();
    ps.peek();

    ps.skip_to_peek();

    assert_eq!(Some('c'), ps.current());
    assert_eq!(Some('c'), ps.current_peek());
    assert_eq!(Some(2), ps.get_peek_index());
    assert_eq!(Some(2), ps.get_index());

    ps.peek();

    assert_eq!(Some('c'), ps.current());
    assert_eq!(Some('d'), ps.current_peek());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(2), ps.get_index());

    ps.next();

    assert_eq!(Some('d'), ps.current());
    assert_eq!(Some('d'), ps.current_peek());
    assert_eq!(Some(3), ps.get_peek_index());
    assert_eq!(Some(3), ps.get_index());
}

#[test]
fn reset_peek() {
    let mut ps = parserstream("abcd".chars());

    ps.next();
    ps.peek();
    ps.peek();
    ps.reset_peek();

    assert_eq!(Some('a'), ps.current());
    assert_eq!(Some('a'), ps.current_peek());
    assert_eq!(Some(0), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());

    ps.peek();

    assert_eq!(Some('a'), ps.current());
    assert_eq!(Some('b'), ps.current_peek());
    assert_eq!(Some(1), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());

    ps.peek();
    ps.peek();
    ps.peek();
    ps.peek();
    ps.reset_peek();

    assert_eq!(Some('a'), ps.current());
    assert_eq!(Some('a'), ps.current_peek());
    assert_eq!(Some(0), ps.get_peek_index());
    assert_eq!(Some(0), ps.get_index());
}

#[test]
fn peek_char_is() {
    let mut ps = parserstream("abcd".chars());

    ps.next();
    ps.peek();

    assert_eq!(ps.peek_char_is('c'), true);

    assert_eq!(Some('a'), ps.current());
    assert_eq!(Some('b'), ps.current_peek());

    ps.skip_to_peek();

    assert_eq!(Some('b'), ps.current());
}
