#![feature(never_type)]
#![feature(generic_const_exprs)]
#![feature(const_trait_impl)]
#![feature(const_precise_live_drops)]
#![feature(type_alias_impl_trait)]

// for test
#![feature(fn_traits)]
#![feature(unboxed_closures)]

//! https://www.angelfire.com/tx4/cus/combinator/birds.html
//! 
//! Combinator birds implemented in rust, using currying composition.
//! 
//! Functions are to be passed into the bird functions, yielding a function-composition.

use currycompose::*;
use currying::*;

/// B = Bluebird
/// 
/// λabc.a(bc)
/// 
/// returns a ∘ (b ∘ c)
/// 
/// possible interpretations:
/// * a(b(c))
pub const fn b<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C,
) -> Composition<A, Composition<B, C, X1A, X1B>, X2A, X2B>
where
    B: ~const Compose<C, X1A, X1B>,
    A: ~const Compose<Composition<B, C, X1A, X1B>, X2A, X2B>,
{
    a.compose(b.compose(c))
}

/// B¹ = Blackbird
/// 
/// λabcd.a(bcd)
/// 
/// returns a ∘ (b ∘ c ∘ d)
/// 
/// possible interpretations:
/// * a(b(c, d))
/// * a(b(c(d)))
pub const fn b1<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<A, Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, X3A, X3B>
where
    B: ~const Compose<C, X1A, X1B>,
    Composition<B, C, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    A: ~const Compose<Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, X3A, X3B>
{
    a.compose(b.compose(c).compose(d))
}

/// B² = Bunting
/// 
/// λabcde.a(bcde)
/// 
/// returns a ∘ (b ∘ c ∘ d ∘ e)
/// 
/// possible interpretations:
/// * a(b(c, d, e))
/// * a(b(c(d, e)))
/// * a(b(c(d(e))))
pub const fn b2<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<A, Composition<Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, E, X3A, X3B>, X4A, X4B>
where
    B: ~const Compose<C, X1A, X1B>,
    Composition<B, C, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>: ~const Compose<E, X3A, X3B>,
    A: ~const Compose<Composition<Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, E, X3A, X3B>, X4A, X4B>
{
    a.compose(b.compose(c).compose(d).compose(e))
}

/// B³ = Becard
/// 
/// λabcd.a(b(cd))
/// 
/// returns a ∘ (b ∘ (c ∘ d))
/// 
/// possible interpretations:
/// * a(b(c(d)))
pub const fn b3<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<A, Composition<B, Composition<C, D, X1A, X1B>, X2A, X2B>, X3A, X3B>
where
    C: ~const Compose<D, X1A, X1B>,
    B: ~const Compose<Composition<C, D, X1A, X1B>, X2A, X2B>,
    A: ~const Compose<Composition<B, Composition<C, D, X1A, X1B>, X2A, X2B>, X3A, X3B>
{
    a.compose(b.compose(c.compose(d)))
}

/// C = Cardinal
/// 
/// λabc.acb
/// 
/// returns a ∘ c ∘ b
/// 
/// possible interpretations:
/// * a(c, b)
/// * a(c(b))
pub const fn c<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<A, C, X1A, X1B>, B, X2A, X2B>
where
    A: ~const Compose<C, X1A, X1B>,
    Composition<A, C, X1A, X1B>: ~const Compose<B, X2A, X2B>
{
    a.compose(c).compose(b)
}

/// D = Dove
/// 
/// λabcd.ab(cd)
/// 
/// returns a ∘ b ∘ (c ∘ d)
/// 
/// possible interpretations:
/// * a(b, c(d))
/// * a(b(c(d)))
pub const fn d<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<A, B, X1A, X1B>, Composition<C, D, X2A, X2B>, X3A, X3B>
where
    A: ~const Compose<B, X1A, X1B>,
    C: ~const Compose<D, X2A, X2B>,
    Composition<A, B, X1A, X1B>: ~const Compose<Composition<C, D, X2A, X2B>, X3A, X3B>
{
    a.compose(b).compose(c.compose(d))
}

/// D¹ = Dickcissel
/// 
/// λabcde.abc(de)
/// 
/// returns a ∘ b ∘ c ∘ (d ∘ e)
/// 
/// possible interpretations:
/// * a(b, c, d(e))
/// * a(b(c, d(e)))
/// * a(b(c(d(e))))
pub const fn d1<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, Composition<D, E, X3A, X3B>, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    D: ~const Compose<E, X3A, X3B>,
    Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>: ~const Compose<Composition<D, E, X3A, X3B>, X4A, X4B>
{
    a.compose(b).compose(c).compose(d.compose(e))
}

/// D² = Dovekies
/// 
/// λabcde.a(bc)(de)
/// 
/// returns a ∘ (b ∘ c) ∘ (d ∘ e)
/// 
/// possible interpretations:
/// * a(b(c), d(e))
/// * a(b(c(d(e))))
pub const fn d2<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<A, Composition<B, C, X1A, X1B>, X2A, X2B>, Composition<D, E, X3A, X3B>, X4A, X4B>
where
    B: ~const Compose<C, X1A, X1B>,
    A: ~const Compose<Composition<B, C, X1A, X1B>, X2A, X2B>,
    D: ~const Compose<E, X3A, X3B>,
    Composition<A, Composition<B, C, X1A, X1B>, X2A, X2B>: ~const Compose<Composition<D, E, X3A, X3B>, X4A, X4B>
{
    a.compose(b.compose(c)).compose(d.compose(e))
}

/// E = Eagle
/// 
/// λabcde.ab(cde)
/// 
/// returns a ∘ b ∘ (c ∘ d ∘ e)
/// 
/// possible interpretations:
/// * a(b, c(d, e))
/// * a(b, c(d(e)))
/// * a(b(c(d, e)))
/// * a(b(c(d(e))))
pub const fn e<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<A, B, X1A, X1B>, Composition<Composition<C, D, X2A, X2B>, E, X3A, X3B>, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    C: ~const Compose<D, X2A, X2B>,
    Composition<C, D, X2A, X2B>: ~const Compose<E, X3A, X3B>,
    Composition<A, B, X1A, X1B>: ~const Compose<Composition<Composition<C, D, X2A, X2B>, E, X3A, X3B>, X4A, X4B>
{
    a.compose(b).compose(c.compose(d).compose(e))
}

/// Ê = Bald Eagle
/// 
/// λabcdefg.a(bcd)(efg)
/// 
/// returns a ∘ (b ∘ c ∘ d) ∘ (e ∘ f ∘ g)
/// 
/// possible interpretations:
/// * a(b(c, d), e(f, g))
/// * a(b(c, d), e(f(g)))
/// * a(b(c(d)), e(f, g))
/// * a(b(c(d)), e(f(g)))
/// * a(b(c(d(e(f, g)))))
/// * a(b(c(d(e(f(g))))))
pub const fn ê<A, B, C, D, E, F, G, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B, X5A, X5B, X6A, X6B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E,
    f: F,
    g: G
) -> Composition<Composition<A, Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, X3A, X3B>, Composition<Composition<E, F, X4A, X4B>, G, X5A, X5B>, X6A, X6B>
where
    B: ~const Compose<C, X1A, X1B>,
    Composition<B, C, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    A: ~const Compose<Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, X3A, X3B>,
    E: ~const Compose<F, X4A, X4B>,
    Composition<E, F, X4A, X4B>: ~const Compose<G, X5A, X5B>,
    Composition<A, Composition<Composition<B, C, X1A, X1B>, D, X2A, X2B>, X3A, X3B>: ~const Compose<Composition<Composition<E, F, X4A, X4B>, G, X5A, X5B>, X6A, X6B>
{
    a.compose(b.compose(c).compose(d)).compose(e.compose(f).compose(g))
}

/// F = Finch
/// 
/// λabc.cba
/// 
/// returns c ∘ b ∘ a
/// 
/// possible interpretations:
/// * c(b, a)
/// * c(b(a))
pub const fn f<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<C, B, X1A, X1B>, A, X2A, X2B>
where
    C: ~const Compose<B, X1A, X1B>,
    Composition<C, B, X1A, X1B>: ~const Compose<A, X2A, X2B>
{
    c.compose(b).compose(a)
}

/// G = Goldfinch
/// 
/// λabcd.ad(bc)
/// 
/// returns a ∘ d ∘ (b ∘ c)
/// 
/// possible interpretations:
/// * a(d, b(c))
/// * a(d(b(c)))
pub const fn g<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<A, D, X1A, X1B>, Composition<B, C, X2A, X2B>, X3A, X3B>
where
    A: ~const Compose<D, X1A, X1B>,
    B: ~const Compose<C, X2A, X2B>,
    Composition<A, D, X1A, X1B>: ~const Compose<Composition<B, C, X2A, X2B>, X3A, X3B>
{
    a.compose(d).compose(b.compose(c))
}

/// H = Hummingbird
/// 
/// λabc.abcb
/// 
/// returns a ∘ b ∘ c ∘ b
/// 
/// possible interpretations:
/// * a(b, c, b)
/// * a(b(c, b))
/// * a(b(c(b)))
pub const fn h<A, B, C, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, B, X3A, X3B>
where
    B: Copy,
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>: ~const Compose<B, X3A, X3B>
{
    a.compose(b).compose(c).compose(b)
}

/// I = Identity Bird aka Idiot
/// 
/// λa.a
/// 
/// returns a
/// 
/// possible interpretations:
/// * a
pub const fn i<A>(
    a: A
) -> A
{
    a
}

/// J = Jay
/// 
/// λabcd.ab(adc)
/// 
/// returns a ∘ b ∘ (a ∘ d ∘ c)
/// 
/// possible interpretations:
/// * a(b, a(d, c))
/// * a(b, a(d(c)))
/// * a(b(a(d, c)))
/// * a(b(a(d(c))))
pub const fn j<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<A, B, X1A, X1B>, Composition<Composition<A, D, X2A, X2B>, C, X3A, X3B>, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B> + ~const Compose<D, X2A, X2B> + Copy,
    Composition<A, D, X2A, X2B>: ~const Compose<C, X3A, X3B>,
    Composition<A, B, X1A, X1B>: ~const Compose<Composition<Composition<A, D, X2A, X2B>, C, X3A, X3B>, X4A, X4B>
{
    a.compose(b).compose(a.compose(d).compose(c))
}

/// K = Kestrel (True)
/// 
/// λab.a
/// 
/// returns a
/// 
/// possible interpretations:
/// * a
#[allow(unused)]
pub fn k<A, B>(
    a: A,
    b: B
) -> A
{
    a
}

/// L = Lark
/// 
/// λab.a(bb)
/// 
/// returns a ∘ (b ∘ b)
/// 
/// possible interpretations:
/// * a(b(b))
pub const fn l<A, B, X1A, X1B, X2A, X2B>(
    a: A,
    b: B
) -> Composition<A, Composition<B, B, X1A, X1B>, X2A, X2B>
where
    B: ~const Compose<B, X1A, X1B> + Copy,
    A: ~const Compose<Composition<B, B, X1A, X1B>, X2A, X2B>
{
    a.compose(b.compose(b))
}

/// M = Mockingbird
/// 
/// λa.aa
/// 
/// returns a ∘ a
/// 
/// possible interpretations:
/// * a(a)
pub const fn m<A, X1A, X1B>(
    a: A
) -> Composition<A, A, X1A, X1B>
where
    A: ~const Compose<A, X1A, X1B> + Copy
{
    a.compose(a)
}
/*pub fn m<A, X1>(
    a: A
) -> <A as Curry<A, X1>>::Output
where
    A: Curry<A, X1> + Copy
{
    a.curry(a)
}*/

/// M² = Double Mockingbird
/// 
/// λab.ab(ab)
/// 
/// returns a ∘ b ∘ (a ∘ b)
/// 
/// possible interpretations:
/// * a(b, a(b))
/// * a(b(a(b)))
pub const fn m2<A, B, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B
) -> Composition<Composition<A, B, X1A, X1B>, Composition<A, B, X2A, X2B>, X3A, X3B>
where
    A: ~const Compose<B, X1A, X1B> + ~const Compose<B, X2A, X2B> + Copy,
    B: Copy,
    Composition<A, B, X1A, X1B>: ~const Compose<Composition<A, B, X2A, X2B>, X3A, X3B>
{
    a.compose(b).compose(a.compose(b))
}

/// O = Owl
/// 
/// λab.b(ab)
/// 
/// returns b ∘ (a ∘ b)
/// 
/// possible interpretations:
/// * b(a(b))
pub const fn o<A, B, X1A, X1B, X2A, X2B>(
    a: A,
    b: B
) -> Composition<B, Composition<A, B, X1A, X1B>, X2A, X2B>
where
    A: ~const Compose<B, X1A, X1B>,
    B: ~const Compose<Composition<A, B, X1A, X1B>, X2A, X2B> + Copy
{
    b.compose(a.compose(b))
}

/// Q = Queer Bird
/// 
/// λabc.b(ac)
/// 
/// returns b ∘ (a ∘ c)
/// 
/// possible interpretations:
/// * b(a(c))
pub const fn q<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<B, Composition<A, C, X1A, X1B>, X2A, X2B>
where
    A: ~const Compose<C, X1A, X1B>,
    B: ~const Compose<Composition<A, C, X1A, X1B>, X2A, X2B>
{
    b.compose(a.compose(c))
}

/// Q¹ = Quixotic Bird
/// 
/// λabc.a(cb)
/// 
/// returns a ∘ (c ∘ b)
/// 
/// possible interpretations:
/// * a(c(b))
pub const fn q1<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<A, Composition<C, B, X1A, X1B>, X2A, X2B>
where
    C: ~const Compose<B, X1A, X1B>,
    A: ~const Compose<Composition<C, B, X1A, X1B>, X2A, X2B>
{
    a.compose(c.compose(b))
}

/// Q² = Quizzical Bird
/// 
/// λabc.b(ca)
/// 
/// returns b ∘ (c ∘ a)
/// 
/// possible interpretations:
/// * b(c(a))
pub const fn q2<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<B, Composition<C, A, X1A, X1B>, X2A, X2B>
where
    C: ~const Compose<A, X1A, X1B>,
    B: ~const Compose<Composition<C, A, X1A, X1B>, X2A, X2B>
{
    b.compose(c.compose(a))
}

/// Q³ = Quirky Bird
/// 
/// λabc.c(ab)
/// 
/// returns c ∘ (a ∘ b)
/// 
/// possible interpretations:
/// * c(a(b))
pub const fn q3<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<C, Composition<A, B, X1A, X1B>, X2A, X2B>
where
    A: ~const Compose<B, X1A, X1B>,
    C: ~const Compose<Composition<A, B, X1A, X1B>, X2A, X2B>
{
    c.compose(a.compose(b))
}

/// Q⁴ = Quacky Bird
/// 
/// λabc.c(ba)
/// 
/// returns c ∘ (b ∘ a)
/// 
/// possible interpretations:
/// * c(b(a))
pub const fn q4<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<C, Composition<B, A, X1A, X1B>, X2A, X2B>
where
    B: ~const Compose<A, X1A, X1B>,
    C: ~const Compose<Composition<B, A, X1A, X1B>, X2A, X2B>
{
    c.compose(b.compose(a))
}

/// R = Robin
/// 
/// λabc.bca
/// 
/// returns b ∘ c ∘ a
/// 
/// possible interpretations:
/// * b(c, a)
/// * b(c(a))
pub const fn r<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<B, C, X1A, X1B>, A, X2A, X2B>
where
    B: ~const Compose<C, X1A, X1B>,
    Composition<B, C, X1A, X1B>: ~const Compose<A, X2A, X2B>
{
    b.compose(c).compose(a)
}

/// S = Starling
/// 
/// λabc.ac(bc)
/// 
/// returns a ∘ c ∘ (b ∘ c)
/// 
/// possible interpretations:
/// * a(c, b(c))
/// * a(c(b(c)))
pub const fn s<A, B, C, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<A, C, X1A, X1B>, Composition<B, C, X2A, X2B>, X3A, X3B>
where
    A: ~const Compose<C, X1A, X1B>,
    B: ~const Compose<C, X2A, X2B>,
    Composition<A, C, X1A, X1B>: ~const Compose<Composition<B, C, X2A, X2B>, X3A, X3B>,
    C: Copy
{
    a.compose(c).compose(b.compose(c))
}

/// T = Thrush
/// 
/// λab.ba
/// 
/// returns b ∘ a
/// 
/// possible interpretations:
/// * b(a)
pub const fn t<A, B, X1A, X1B>(
    a: A,
    b: B
) -> Composition<B, A, X1A, X1B>
where
    B: ~const Compose<A, X1A, X1B>
{
    b.compose(a)
}

/// U = Turing
/// 
/// λab.b(aab)
/// 
/// returns b ∘ (a ∘ a ∘ b)
/// 
/// possible interpretations:
/// * b(a(a, b))
/// * b(a(a(b)))
pub const fn u<A, B, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B
) -> Composition<B, Composition<Composition<A, A, X1A, X1B>, B, X2A, X2B>, X3A, X3B>
where
    A: ~const Compose<A, X1A, X1B> + Copy,
    Composition<A, A, X1A, X1B>: ~const Compose<B, X2A, X2B>,
    B: ~const Compose<Composition<Composition<A, A, X1A, X1B>, B, X2A, X2B>, X3A, X3B> + Copy
{
    b.compose(a.compose(a).compose(b))
}

/// V = Vireo aka Pairing
/// 
/// λabc.cab
/// 
/// returns c ∘ a ∘ b
/// 
/// possible interpretations:
/// * c(a, b)
/// * c(a(b))
pub const fn v<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<C, A, X1A, X1B>, B, X2A, X2B>
where
    C: ~const Compose<A, X1A, X1B>,
    Composition<C, A, X1A, X1B>: ~const Compose<B, X2A, X2B>
{
    c.compose(a).compose(b)
}

/// W = Warbler
/// 
/// λab.abb
/// 
/// returns a ∘ b ∘ b
/// 
/// possible interpretations:
/// * a(b, b)
/// * a(b(b))
pub const fn w<A, B, X1A, X1B, X2A, X2B>(
    a: A,
    b: B
) -> Composition<Composition<A, B, X1A, X1B>, B, X2A, X2B>
where
    B: Copy,
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<B, X2A, X2B>
{
    a.compose(b).compose(b)
}

/// W¹ = Converse Warbler
/// 
/// λab.baa
/// 
/// returns b ∘ a ∘ a
/// 
/// possible interpretations:
/// * b(a, a)
/// * b(a(a))
pub const fn w1<A, B, X1A, X1B, X2A, X2B>(
    a: A,
    b: B
) -> Composition<Composition<B, A, X1A, X1B>, A, X2A, X2B>
where
    A: Copy,
    B: ~const Compose<A, X1A, X1B>,
    Composition<B, A, X1A, X1B>: ~const Compose<A, X2A, X2B>
{
    b.compose(a).compose(a)
}

/// Y = Why Bird
/// 
/// λa.a(λa)
/// 
/// returns a ∘ a ∘ a ∘ ...
/// 
/// possible interpretations:
/// * a(a(a(...)))
pub const fn y<A>(
    a: impl Fn(A) -> A + Copy
) -> impl Fn(A) -> !
{
    move |mut x| loop
    {
        x = a(x)
    }
}

/// I* = Identity Bird Once Removed
/// 
/// λab.ab
/// 
/// returns a ∘ b
/// 
/// possible interpretations:
/// * a(b)
pub const fn i_star<A, B, X1A, X1B>(
    a: A,
    b: B
) -> Composition<A, B, X1A, X1B>
where
    A: ~const Compose<B, X1A, X1B>
{
    a.compose(b)
}

/// W* = Warbled Once Removed
/// 
/// λabc.abcc
/// 
/// returns a ∘ b ∘ c ∘ c
/// 
/// possible interpretations:
/// * a(b, c, c)
/// * a(b, c(c))
/// * a(b(c, c))
/// * a(b(c(c)))
pub const fn w_star<A, B, C, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, C, X3A, X3B>
where
    C: Copy,
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>: ~const Compose<C, X3A, X3B>
{
    a.compose(b).compose(c).compose(c)
}

/// C* = Cardinal Once Removed
/// 
/// λabcd.abdc
/// 
/// returns a ∘ b ∘ d ∘ c
/// 
/// possible interpretations:
/// * a(b, d, c)
/// * a(b(d, c))
/// * a(b(d(c)))
pub const fn c_star<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<Composition<A, B, X1A, X1B>, D, X2A, X2B>, C, X3A, X3B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, D, X2A, X2B>: ~const Compose<C, X3A, X3B>
{
    a.compose(b).compose(d).compose(c)
}

/// R* = Robin Once Removed
/// 
/// λabcd.acdb
/// 
/// returns a ∘ c ∘ d ∘ b
/// 
/// possible interpretations:
/// * a(c, d, b)
/// * a(c(d, b))
/// * a(c(d(b)))
pub const fn r_star<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<Composition<A, C, X1A, X1B>, D, X2A, X2B>, B, X3A, X3B>
where
    A: ~const Compose<C, X1A, X1B>,
    Composition<A, C, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    Composition<Composition<A, C, X1A, X1B>, D, X2A, X2B>: ~const Compose<B, X3A, X3B>
{
    a.compose(c).compose(d).compose(b)
}

/// F* = Finch Once Removed
/// 
/// λabcd.adcb
/// 
/// returns a ∘ d ∘ c ∘ b
/// 
/// possible interpretations:
/// * a(d, c, b)
/// * a(d(c, b))
/// * a(d(c(b)))
pub const fn f_star<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<Composition<A, D, X1A, X1B>, C, X2A, X2B>, B, X3A, X3B>
where
    A: ~const Compose<D, X1A, X1B>,
    Composition<A, D, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    Composition<Composition<A, D, X1A, X1B>, C, X2A, X2B>: ~const Compose<B, X3A, X3B>
{
    a.compose(d).compose(c).compose(b)
}

/// V* = Vireo Once Removed
/// 
/// λabcd.acbd
/// 
/// returns a ∘ c ∘ b ∘ d
/// 
/// possible interpretations:
/// * a(c, b, d)
/// * a(c(b, d))
/// * a(c(b(d)))
pub const fn v_star<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<Composition<A, C, X1A, X1B>, B, X2A, X2B>, D, X3A, X3B>
where
    A: ~const Compose<C, X1A, X1B>,
    Composition<A, C, X1A, X1B>: ~const Compose<B, X2A, X2B>,
    Composition<Composition<A, C, X1A, X1B>, B, X2A, X2B>: ~const Compose<D, X3A, X3B>
{
    a.compose(c).compose(b).compose(d)
}

/// I** = Identity Bird Twice Removed
/// 
/// λabc.abc
/// 
/// returns a ∘ b ∘ c
/// 
/// possible interpretations:
/// * a(b, c)
/// * a(b(c))
pub const fn i_star_star<A, B, C, X1A, X1B, X2A, X2B>(
    a: A,
    b: B,
    c: C
) -> Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>
{
    a.compose(b).compose(c)
}

/// W** = Warbler Twice Removed
/// 
/// λabcd.abcdd
/// 
/// returns a ∘ b ∘ c ∘ d ∘ d
/// 
/// possible interpretations:
/// * a(b, c, d, d)
/// * a(b, c, d(d))
/// * a(b, c(d, d))
/// * a(b, c(d(d)))
/// * a(b(c, d, d))
/// * a(b(c, d(d)))
/// * a(b(c(d, d)))
/// * a(b(c(d(d))))
pub const fn w_star_star<A, B, C, D, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D
) -> Composition<Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, D, X3A, X3B>, D, X4A, X4B>
where
    D: Copy,
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>: ~const Compose<D, X3A, X3B>,
    Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, D, X3A, X3B>: ~const Compose<D, X4A, X4B>
{
    a.compose(b).compose(c).compose(d).compose(d)
}

/// C** = Cardinal Twice Removed
/// 
/// λabcde.abced
/// 
/// returns a ∘ b ∘ c ∘ e ∘ d
/// 
/// possible interpretations:
/// * a(b, c, e, d)
/// * a(b(c, e, d))
/// * a(b(c(e, d)))
/// * a(b(c(e(d))))
pub const fn c_star_star<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, E, X3A, X3B>, D, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<C, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>: ~const Compose<E, X3A, X3B>,
    Composition<Composition<Composition<A, B, X1A, X1B>, C, X2A, X2B>, E, X3A, X3B>: ~const Compose<D, X4A, X4B>
{
    a.compose(b).compose(c).compose(e).compose(d)
}

/// R** = Robin Twice Removed
/// 
/// λabcde.abdec
/// 
/// returns a ∘ b ∘ d ∘ e ∘ c
/// 
/// possible interpretations:
/// * a(b, d, e, c)
/// * a(b(d, e, c))
/// * a(b(d(e, c)))
/// * a(b(d(e(c))))
pub const fn r_star_star<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<Composition<Composition<A, B, X1A, X1B>, D, X2A, X2B>, E, X3A, X3B>, C, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<D, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, D, X2A, X2B>: ~const Compose<E, X3A, X3B>,
    Composition<Composition<Composition<A, B, X1A, X1B>, D, X2A, X2B>, E, X3A, X3B>: ~const Compose<C, X4A, X4B>
{
    a.compose(b).compose(d).compose(e).compose(c)
}

/// F** = Finch Twice Removed
/// 
/// λabcde.abedc
/// 
/// returns a ∘ b ∘ e ∘ d ∘ c
/// 
/// possible interpretations:
/// * a(b, e, d, c)
/// * a(b(e, d, c))
/// * a(b(e(d, c)))
/// * a(b(e(d(c))))
pub const fn f_star_star<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>, D, X3A, X3B>, C, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<E, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>: ~const Compose<D, X3A, X3B>,
    Composition<Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>, D, X3A, X3B>: ~const Compose<C, X4A, X4B>
{
    a.compose(b).compose(e).compose(d).compose(c)
}

/// V** = Vireo Twice Removed
/// 
/// λabcde.abecd
/// 
/// returns a ∘ b ∘ e ∘ c ∘ d
/// 
/// possible interpretations:
/// * a(b, e, c, d)
/// * a(b(e, c, d))
/// * a(b(e(c, d)))
/// * a(b(e(c(d))))
pub const fn v_star_star<A, B, C, D, E, X1A, X1B, X2A, X2B, X3A, X3B, X4A, X4B>(
    a: A,
    b: B,
    c: C,
    d: D,
    e: E
) -> Composition<Composition<Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>, C, X3A, X3B>, D, X4A, X4B>
where
    A: ~const Compose<B, X1A, X1B>,
    Composition<A, B, X1A, X1B>: ~const Compose<E, X2A, X2B>,
    Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>: ~const Compose<C, X3A, X3B>,
    Composition<Composition<Composition<A, B, X1A, X1B>, E, X2A, X2B>, C, X3A, X3B>: ~const Compose<D, X4A, X4B>
{
    a.compose(b).compose(e).compose(c).compose(d)
}

/// KI = Kite (False)
/// 
/// λab.b
/// 
/// returns b
/// 
/// possible interpretations:
/// * b
#[allow(unused)]
pub fn ki<A, B>(
    a: A,
    b: B
) -> B
{
    b
}

/// Ω = Omega
/// 
/// λ
/// 
/// returns infinite loop closure
#[allow(non_snake_case)]
pub const fn Ω() -> impl Fn() -> !
{
    || loop {}
}

/// KM = Constant Mocker
/// 
/// λab.bb
/// 
/// returns b ∘ b
/// 
/// possible interpretations:
/// * b(b)
#[allow(unused)]
pub fn km<A, B, X1A, X1B>(
    a: A,
    b: B
) -> Composition<B, B, X1A, X1B>
where
    B: Compose<B, X1A, X1B> + Copy
{
    b.compose(b)
}

/// C(KM) = Crossed Constant Mocker
/// 
/// λab.aa
/// 
/// returns a ∘ a
/// 
/// possible interpretations:
/// * a(a)
#[allow(unused)]
pub fn ckm<A, B, X1A, X1B>(
    a: A,
    b: B
) -> Composition<A, A, X1A, X1B>
where
    A: Compose<A, X1A, X1B> + Copy
{
    a.compose(a)
}

/// Θ = Theta
/// 
/// (λab.b(aab))(λab.b(aab))
/// 
/// λa.a(Θa)
/// 
/// returns a ∘ a ∘ a ∘ ...
/// 
/// possible interpretations:
/// * a(a(a(...)))
#[allow(non_snake_case)]
pub const fn Θ<A>(
    a: impl Fn(A) -> A
) -> impl Fn(A) -> !
{
    move |mut x| loop
    {
        x = a(x)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_b()
    {
        let a = |x: f32| x as u8;
        let b = |x: f32| x.sqrt();
        let c = |x: u8| x as f32;
        
        let f = crate::b(a, b, c);
        let f_eqv = |x| a(b(c(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }

    #[test]
    fn test_b1()
    {
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32| x + y/2.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
    
            let f = crate::b1(a, b, c, d);
            let f_eqv = |x, y| a(b(c(x), d(y)));
            
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*x;
            let d = |x: u8| x as f32;
    
            let f = crate::b1(a, b, c, d);
            let f_eqv = |x| a(b(c(d(x))));
            
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_b2()
    {
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: u8| (x as f32*3.0).powi(2);
    
            let f = crate::b2(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(c(x), d(y), e(z)));
            
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: f32, y: f32| x + y/5.0;
            let d = |x: u8| (x as f32).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
    
            let f = crate::b2(a, b, c, d, e);
            let f_eqv = |x, y| a(b(c(d(x), e(y))));
            
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*2.0;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32;
    
            let f = crate::b2(a, b, c, d, e);
            let f_eqv = |x| a(b(c(d(e(x)))));
            
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_b3()
    {
        let a = |x: f32| x.sqrt();
        let b = |x: f32| x + 1.0;
        let c = |x: f32| x*x;
        let d = |x: u8| x as f32;

        let f = crate::b3(a, b, c, d);
        let f_eqv = |x| a(b(c(d(x))));
        
        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }

    #[test]
    fn test_c()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::c(a, b, c);
            let f_eqv = |x, y| a(c(x), b(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x as u8;
            let b = |x: u8| x as f32;
            let c = |x: f32| x.sqrt();
            
            let f = crate::c(a, b, c);
            let f_eqv = |x| a(c(b(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_d()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: f32| x*x;
            let d = |x: u8| x as f32*2.0;
            
            let f = crate::d(a, b, c, d);
            let f_eqv = |x, y| a(b(x), c(d(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*x;
            let d = |x: u8| x as f32;
    
            let f = crate::d(a, b, c, d);
            let f_eqv = |x| a(b(c(d(x))));
            
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_d1()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32*3.0;
            
            let f = crate::d1(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(x), c(y), d(e(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32| x + y/2.0;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32*2.0;
            
            let f = crate::d1(a, b, c, d, e);
            let f_eqv = |x, y| a(b(c(x), d(e(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*2.0;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32;
            
            let f = crate::d1(a, b, c, d, e);
            let f_eqv = |x| a(b(c(d(e(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_d2()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x*3.0;
            let e = |x: u8| x as f32*2.0;
            
            let f = crate::d2(a, b, c, d, e);
            let f_eqv = |x, y| a(b(c(x)), d(e(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*2.0;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32;
            
            let f = crate::d2(a, b, c, d, e);
            let f_eqv = |x| a(b(c(d(e(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_e()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: f32, y: f32| x + y/3.0;
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: u8| (x as f32*3.0).powi(2);
            
            let f = crate::e(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(x), c(d(y), e(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32| (x + y/5.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: f32| x*2.0;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32*3.0;
            
            let f = crate::e(a, b, c, d, e);
            let f_eqv = |x, y| a(b(x), c(d(e(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32, y: f32| x + y/2.0;
            let d = |x: u8| (x as f32).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::e(a, b, c, d, e);
            let f_eqv = |x, y| a(b(c(d(x), e(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x + 1.0;
            let c = |x: f32| x*2.0;
            let d = |x: f32| x*x;
            let e = |x: u8| x as f32;
            
            let f = crate::e(a, b, c, d, e);
            let f_eqv = |x| a(b(c(d(e(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_ê()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32, y: f32| x + y/3.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: f32, y: f32| x + y/4.0;
            let f = |x: u8| (x as f32*3.0).powi(2);
            let g = |x: u8| (x as f32*4.0).powi(2);
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x, y, z, æ| a(b(c(x), d(y)), e(f(z), g(æ)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f_(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32, y: f32| x + y/3.0;
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            let e = |x: f32| x*x;
            let f = |x: f32| x - 1.0;
            let g = |x: u8| x as f32;
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x, y, z| a(b(c(x), d(y)), e(f(g(z))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f_(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32| x*x;
            let c = |x: f32| x - 1.0;
            let d = |x: u8| x as f32;
            let e = |x: f32, y: f32| x + y/3.0;
            let f = |x: u8| (x as f32*2.0).powi(2);
            let g = |x: u8| (x as f32*3.0).powi(2);
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x, y, z| a(b(c(d(x))), e(f(y), g(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f_(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32| x*x*3.0;
            let c = |x: f32| x - 1.0;
            let d = |x: u8| x as f32;
            let e = |x: f32| x*x;
            let f = |x: f32| x - 5.0;
            let g = |x: u8| x as f32*2.0;
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x, y| a(b(c(d(x))), e(f(g(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f_(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x % 1.0;
            let c = |x: f32| x - 1.0;
            let d = |x: f32| x*4.0;
            let e = |x: f32, y: f32| x + y/2.0;
            let f = |x: u8| (x as f32).powi(2);
            let g = |x: u8| (x as f32*2.0).powi(2);
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x, y| a(b(c(d(e(f(x), g(y))))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f_(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x % 1.0;
            let c = |x: f32| x - 1.0;
            let d = |x: f32| x*4.0;
            let e = |x: f32| x - 3.0;
            let f = |x: f32| x*x;
            let g = |x: u8| x as f32;
            
            let f_ = crate::ê(a, b, c, d, e, f, g);
            let f_eqv = |x| a(b(c(d(e(f(g(x)))))));
    
            for i in 0..=255
            {
                assert_eq!(f_(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_f()
    {
        {
            let a = |x: u8| (x as f32*2.0).powi(2);
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: f32, y: f32| (x + y/2.0).sqrt();
            
            let f = crate::f(a, b, c);
            let f_eqv = |x, y| c(b(x), a(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: u8| x as f32;
            let b = |x: f32| x*x*2.0;
            let c = |x: f32| x.sqrt();
            
            let f = crate::f(a, b, c);
            let f_eqv = |x| c(b(a(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_g()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32*2.0;
            let d = |x: u8| (x as f32).powi(2);
            
            let f = crate::g(a, b, c, d);
            let f_eqv = |x, y| a(d(x), b(c(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x - 1.0;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x;
            
            let f = crate::g(a, b, c, d);
            let f_eqv = |x| a(d(b(c(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_h()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::h(a, b, c);
            let f_eqv = |x, y, z| a(b(x), c(y), b(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: u8| x as f32;
            let b = |x: u8, y: u8| x.overflowing_add(y.overflowing_mul(3).0).0;
            let c = |x: u8| x / 2;
            
            let f = crate::h(a, b, c);
            let f_eqv = |x, y, z| a(b(c(x), b(y, z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: u8| x as f32;
            let b = |x: u8| x.overflowing_add(10).0;
            let c = |x: u8| x / 2;
            
            let f = crate::h(a, b, c);
            let f_eqv = |x| a(b(c(b(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_i()
    {
        let a = |x: u8| x as f32;
        
        let f = crate::i(a);
        let f_eqv = a;

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }

    #[test]
    fn test_j()
    {
        {
            let a = |x: f32, y: f32| (x + y*y).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| x as f32*3.0;
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::j(a, b, c, d);
            let f_eqv = |x, y, z| a(b(x), a(d(y), c(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            // Overload f_a
            fn a_2(x: f32, y: f32) -> f32
            {
                (x + y*y).sqrt()
            }
            fn a_1(x: f32) -> f32
            {
                x.sqrt()
            }
            #[derive(Clone, Copy)]
            struct A;
            impl FnOnce<(f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32,)) -> Self::Output
                {
                    a_1.call_once(args)
                }
            }
            impl FnOnce<(f32, f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32, f32,)) -> Self::Output
                {
                    a_2.call_once(args)
                }
            }

            let a = A;
            let b = |x: u8| (x as f32*2.0).powi(2);
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x;

            let f = crate::j::<_, _, _, _, (f32, f32,), _, (f32,), _, _, _, _, _>(a, b, c, d);
            let f_eqv = |x, y| a(b(x), a(d(c(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            // Overload f_a
            fn a_2(x: f32, y: f32) -> f32
            {
                (x + y*y).sqrt()
            }
            fn a_1(x: f32) -> f32
            {
                x.sqrt()
            }
            #[derive(Clone, Copy)]
            struct A;
            impl FnOnce<(f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32,)) -> Self::Output
                {
                    a_1.call_once(args)
                }
            }
            impl FnOnce<(f32, f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32, f32,)) -> Self::Output
                {
                    a_2.call_once(args)
                }
            }
            
            let a = A;
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::j::<_, _, _, _, (f32,), _, (f32, f32,), _, _, _, _, _>(a, b, c, d);
            let f_eqv = |x, y| a(b(a(d(x), c(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x*2.0;
            
            let f = crate::j(a, b, c, d);
            let f_eqv = |x| a(b(a(d(c(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_k()
    {
        let a = |x: u8| x as f32;
        let b = |x: u8| x as f32*2.0;
        
        let f = crate::k(a, b);
        let f_eqv = a;

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_l()
    {
        let a = |x: u8| x as f32;
        let b = |x: u8| x / 2;
        
        let f = crate::l(a, b);
        let f_eqv = |x| a(b(b(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_m()
    {
        let a = |x: u8| x / 2;
        
        let f = crate::m(a);
        let f_eqv = |x| a(a(x));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_m2()
    {
        {
            // Overload f_a
            fn a_2(x: f32, y: f32) -> f32
            {
                (x + y*y).sqrt()
            }
            fn a_1(x: f32) -> f32
            {
                x.sqrt()
            }
            #[derive(Clone, Copy)]
            struct A;
            impl FnOnce<(f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32,)) -> Self::Output
                {
                    a_1.call_once(args)
                }
            }
            impl FnOnce<(f32, f32,)> for A
            {
                type Output = f32;
    
                extern "rust-call" fn call_once(self, args: (f32, f32,)) -> Self::Output
                {
                    a_2.call_once(args)
                }
            }
            
            let a = A;
            let b = |x: u8| (x as f32*2.0).powi(2);

            let f = crate::m2::<_, _, (f32, f32,), _, (f32,), _, _, _>(a, b);
            let f_eqv = |x, y| a(b(x), a(b(y)));

            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: u8| x * 2;
            let b = |x: u8| x / 3;
            
            let f = crate::m2(a, b);
            let f_eqv = |x| a(b(a(b(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_o()
    {
        let a = |x: f32| (x * 2.0) as u8;
        let b = |x: u8| x as f32 / 3.0;
        
        let f = crate::o(a, b);
        let f_eqv = |x| b(a(b(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_q()
    {
        let a = |x: f32| x*x;
        let b = |x: f32| x.sqrt();
        let c = |x: u8| x as f32;
        
        let f = crate::q(a, b, c);
        let f_eqv = |x| b(a(c(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_q1()
    {
        let a = |x: f32| x.sqrt();
        let b = |x: u8| x as f32;
        let c = |x: f32| x*x;
        
        let f = crate::q1(a, b, c);
        let f_eqv = |x| a(c(b(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_q2()
    {
        let a = |x: u8| x as f32;
        let b = |x: f32| x.sqrt();
        let c = |x: f32| x*x;
        
        let f = crate::q2(a, b, c);
        let f_eqv = |x| b(c(a(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_q3()
    {
        let a = |x: f32| x*x;
        let b = |x: u8| x as f32;
        let c = |x: f32| x.sqrt();
        
        let f = crate::q3(a, b, c);
        let f_eqv = |x| c(a(b(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_q4()
    {
        let a = |x: u8| x as f32;
        let b = |x: f32| x*x;
        let c = |x: f32| x.sqrt();
        
        let f = crate::q4(a, b, c);
        let f_eqv = |x| c(b(a(x)));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_r()
    {
        {
            let a = |x: u8| (x as f32*2.0).powi(2);
            let b = |x: f32, y: f32| (x + y/2.0).sqrt();
            let c = |x: u8| (x as f32).powi(2);
            
            let f = crate::r(a, b, c);
            let f_eqv = |x, y| b(c(x), a(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: u8| x as f32;
            let b = |x: f32| x.sqrt();
            let c = |x: f32| x*x;
            
            let f = crate::r(a, b, c);
            let f_eqv = |x| b(c(a(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_s()
    {
        {
            let a = |x: f32, y: f32| (x*x + y).sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;
            
            let f = crate::s(a, b, c);
            let f_eqv = |x, y| a(c(x), b(c(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| (x.sqrt() as u8).overflowing_add(1).0;
            let c = |x: u8| (x as f32).powi(2);

            let f = crate::s(a, b, c);
            let f_eqv = |x| a(c(b(c(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_t()
    {
        let a = |x: u8| (x as f32).powi(2);
        let b = |x: f32| x.sqrt();
        
        let f = crate::t(a, b);
        let f_eqv = |x| b(a(x));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_u()
    {
        {
            let a = |x: u8, y: u8| x.overflowing_add(y.overflowing_mul(2).0).0;
            let b = |x: u8| x.overflowing_add(1).0;
            
            let f = crate::u(a, b);
            let f_eqv = |x, y, z| b(a(a(x, y), b(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: u8| x.overflowing_sub(3).0;
            let b = |x: u8| x.overflowing_add(1).0;

            let f = crate::u(a, b);
            let f_eqv = |x| b(a(a(b(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_v()
    {
        {
            let a = |x: u8| (x as f32).powi(2);
            let b = |x: u8| (x as f32*2.0).powi(2);
            let c = |x: f32, y: f32| (x + y/2.0).sqrt();
            
            let f = crate::v(a, b, c);
            let f_eqv = |x, y| c(a(x), b(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x*x;
            let b = |x: u8| x as f32;
            let c = |x: f32| x.sqrt();

            let f = crate::v(a, b, c);
            let f_eqv = |x| c(a(b(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_w()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            
            let f = crate::w(a, b);
            let f_eqv = |x, y| a(b(x), b(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: u8| x as f32;
            let b = |x: u8| x.overflowing_add(1).0;

            let f = crate::w(a, b);
            let f_eqv = |x| a(b(b(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_w1()
    {
        {
            let a = |x: u8| (x as f32).powi(2);
            let b = |x: f32, y: f32| (x + y/2.0).sqrt();
            
            let f = crate::w1(a, b);
            let f_eqv = |x, y| b(a(x), a(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: u8| x.overflowing_add(1).0;
            let b = |x: u8| x as f32;

            let f = crate::w1(a, b);
            let f_eqv = |x| b(a(a(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_y()
    {
        use std::rc::Rc;
        use std::cell::RefCell;

        let result = Rc::new(RefCell::new(vec![]));
        let a = |i: u8| {
            result.borrow_mut().push(i);
            if i == 255
            {
                assert_eq!(result.borrow().clone(), (0..=255).collect::<Vec<u8>>());
                std::process::exit(0)
            }
            else
            {
                i + 1
            }
        };

        let f = crate::y(a);
        f(0);
        panic!("unreachable");
    }
    
    #[test]
    fn test_i_star()
    {
        let a = |x: f32| x.sqrt();
        let b = |x: u8| (x as f32).powi(2);
        
        let f = crate::i_star(a, b);
        let f_eqv = |x| a(b(x));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_w_star()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::w_star(a, b, c);
            let f_eqv = |x, y, z| a(b(x), c(y), c(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32, y: u8| (x + (y as f32).powi(2)).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| x.overflowing_add(1).0;
            
            let f = crate::w_star(a, b, c);
            let f_eqv = |x, y| a(b(x), c(c(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32| x + y/2.0;
            let c = |x: u8| (x as f32).powi(2);
            
            let f = crate::w_star(a, b, c);
            let f_eqv = |x, y| a(b(c(x), c(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| x.overflowing_add(1).0;

            let f = crate::w_star(a, b, c);
            let f_eqv = |x| a(b(c(c(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_c_star()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*3.0).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::c_star(a, b, c, d);
            let f_eqv = |x, y, z| a(b(x), d(y), c(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32| x + y/2.0;
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32).powi(2);
            
            let f = crate::c_star(a, b, c, d);
            let f_eqv = |x, y| a(b(d(x), c(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x.overflowing_add(1).0;
            let d = |x: u8| x as f32;

            let f = crate::c_star(a, b, c, d);
            let f_eqv = |x| a(b(d(c(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_r_star()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32*3.0).powi(2);
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::r_star(a, b, c, d);
            let f_eqv = |x, y, z| a(c(x), d(y), b(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| (x as f32*2.0).powi(2);
            let c = |x: f32, y: f32| x + y/2.0;
            let d = |x: u8| (x as f32).powi(2);
            
            let f = crate::r_star(a, b, c, d);
            let f_eqv = |x, y| a(c(d(x), b(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| x.overflowing_add(1).0;
            let c = |x: f32| x*x;
            let d = |x: u8| x as f32;

            let f = crate::r_star(a, b, c, d);
            let f_eqv = |x| a(c(d(b(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_f_star()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32*3.0).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32).powi(2);
            
            let f = crate::f_star(a, b, c, d);
            let f_eqv = |x, y, z| a(d(x), c(y), b(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| (x as f32*2.0).powi(2);
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: f32, y: f32| x + y/2.0;
            
            let f = crate::f_star(a, b, c, d);
            let f_eqv = |x, y| a(d(c(x), b(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| x.overflowing_add(1).0;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x;

            let f = crate::f_star(a, b, c, d);
            let f_eqv = |x| a(d(c(b(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_v_star()
    {
        {
            let a = |x: f32, y: f32, z: f32| (x + y/2.0 + z/3.0).sqrt();
            let b = |x: u8| (x as f32*2.0).powi(2);
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            
            let f = crate::v_star(a, b, c, d);
            let f_eqv = |x, y, z| a(c(x), b(y), d(z));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: f32, y: f32| x + y/2.0;
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::v_star(a, b, c, d);
            let f_eqv = |x, y| a(c(b(x), d(y)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: u8| x as f32;
            let c = |x: f32| x*x;
            let d = |x: u8| x.overflowing_add(1).0;

            let f = crate::v_star(a, b, c, d);
            let f_eqv = |x| a(c(b(d(x))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_i_star_star()
    {
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::i_star_star(a, b, c);
            let f_eqv = |x, y| a(b(x), c(y));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;

            let f = crate::i_star_star(a, b, c);
            let f_eqv = |x| a(b(c(x)));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_w_star_star()
    {
        {
            let a = |x: f32, y: f32, z: f32, æ: f32| (x + y/2.0 + z/3.0 + æ/4.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            
            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y, z, æ| a(b(x), c(y), d(z), d(æ));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32, z: u8| (x + y/2.0 + (z as f32).powi(2)).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| x.overflowing_add(1).0;

            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y, z| a(b(x), c(y), d(d(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32, y: f32| (x + y/2.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| x.overflowing_add(1).0;

            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y| a(b(x), c(d(d(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y, z| a(b(c(x), d(y), d(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: u8| x + (y as f32).powi(2);
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| x.overflowing_add(1).0;
            
            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y| a(b(c(x), d(d(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: f32, y: f32| x + y/2.0;
            let d = |x: u8| (x as f32).powi(2);
            
            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x, y| a(b(c(d(x), d(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*x;
            let c = |x: u8| x as f32;
            let d = |x: u8| x.overflowing_add(1).0;
            
            let f = crate::w_star_star(a, b, c, d);
            let f_eqv = |x| a(b(c(d(d(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_c_star_star()
    {
        {
            let a = |x: f32, y: f32, z: f32, æ: f32| (x + y/2.0 + z/3.0 + æ/4.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32*4.0).powi(2);
            let e = |x: u8| (x as f32*3.0).powi(2);
            
            let f = crate::c_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z, æ| a(b(x), c(y), e(z), d(æ));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::c_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(c(x), e(y), d(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: f32, y: f32| x + y/2.0;
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: u8| (x as f32).powi(2);
            
            let f = crate::c_star_star(a, b, c, d, e);
            let f_eqv = |x, y| a(b(c(e(x), d(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: f32| x*3.0;
            let d = |x: u8| x as f32;
            let e = |x: f32| x*x;
            
            let f = crate::c_star_star(a, b, c, d, e);
            let f_eqv = |x| a(b(c(e(d(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    #[test]
    fn test_r_star_star()
    {
        {
            let a = |x: f32, y: f32, z: f32, æ: f32| (x + y/2.0 + z/3.0 + æ/4.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*4.0).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: u8| (x as f32*3.0).powi(2);
            
            let f = crate::r_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z, æ| a(b(x), d(y), e(z), c(æ));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32*3.0).powi(2);
            let d = |x: u8| (x as f32).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::r_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(d(x), e(y), c(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: f32, y: f32| x + y/2.0;
            let e = |x: u8| (x as f32).powi(2);
            
            let f = crate::r_star_star(a, b, c, d, e);
            let f_eqv = |x, y| a(b(d(e(x), c(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*3.0;
            let e = |x: f32| x*x;
            
            let f = crate::r_star_star(a, b, c, d, e);
            let f_eqv = |x| a(b(d(e(c(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }

    #[test]
    fn test_f_star_star()
    {
        {
            let a = |x: f32, y: f32, z: f32, æ: f32| (x + y*2.0 + z*3.0 + æ*4.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*4.0).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::f_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z, æ| a(b(x), e(y), d(z), c(æ));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32*3.0).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: u8| (x as f32).powi(2);
            
            let f = crate::f_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(e(x), d(y), c(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32).powi(2);
            let e = |x: f32, y: f32| x + y/2.0;
            
            let f = crate::f_star_star(a, b, c, d, e);
            let f_eqv = |x, y| a(b(e(d(x), c(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: u8| x as f32;
            let d = |x: f32| x*x;
            let e = |x: f32| x*3.0;
            
            let f = crate::f_star_star(a, b, c, d, e);
            let f_eqv = |x| a(b(e(d(c(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_v_star_star()
    {
        {
            let a = |x: f32, y: f32, z: f32, æ: f32| (x + y/2.0 + z/3.0 + æ/4.0).sqrt();
            let b = |x: u8| (x as f32).powi(2);
            let c = |x: u8| (x as f32*3.0).powi(2);
            let d = |x: u8| (x as f32*4.0).powi(2);
            let e = |x: u8| (x as f32*2.0).powi(2);
            
            let f = crate::v_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z, æ| a(b(x), e(y), c(z), d(æ));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        for l in 0..=255
                        {
                            assert_eq!(f(i, j, k, l), f_eqv(i, j, k, l))
                        }
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32, y: f32, z: f32| x + y/2.0 + z/3.0;
            let c = |x: u8| (x as f32*2.0).powi(2);
            let d = |x: u8| (x as f32*3.0).powi(2);
            let e = |x: u8| (x as f32).powi(2);
            
            let f = crate::v_star_star(a, b, c, d, e);
            let f_eqv = |x, y, z| a(b(e(x), c(y), d(z)));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    for k in 0..=255
                    {
                        assert_eq!(f(i, j, k), f_eqv(i, j, k))
                    }
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: u8| (x as f32).powi(2);
            let d = |x: u8| (x as f32*2.0).powi(2);
            let e = |x: f32, y: f32| x + y/2.0;
            
            let f = crate::v_star_star(a, b, c, d, e);
            let f_eqv = |x, y| a(b(e(c(x), d(y))));
    
            for i in 0..=255
            {
                for j in 0..=255
                {
                    assert_eq!(f(i, j), f_eqv(i, j))
                }
            }
        }
        {
            let a = |x: f32| x.sqrt();
            let b = |x: f32| x*2.0;
            let c = |x: f32| x*x;
            let d = |x: u8| x as f32;
            let e = |x: f32| x*3.0;
            
            let f = crate::v_star_star(a, b, c, d, e);
            let f_eqv = |x| a(b(e(c(d(x)))));
    
            for i in 0..=255
            {
                assert_eq!(f(i), f_eqv(i))
            }
        }
    }
    
    #[test]
    fn test_ki()
    {
        let a = |x: u8| x as f32;
        let b = |x: u8| x as f32*2.0;
        
        let f = crate::ki(a, b);
        let f_eqv = b;

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_km()
    {
        let a = |x: u8| x as f32;
        let b = |x: u8| x.overflowing_add(1).0;
        
        let f = crate::km(a, b);
        let f_eqv = |x| b(b(x));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }
    
    #[test]
    fn test_ckm()
    {
        let a = |x: u8| x.overflowing_add(1).0;
        let b = |x: u8| x as f32;
        
        let f = crate::ckm(a, b);
        let f_eqv = |x| a(a(x));

        for i in 0..=255
        {
            assert_eq!(f(i), f_eqv(i))
        }
    }

    #[test]
    fn test_Θ()
    {
        use std::rc::Rc;
        use std::cell::RefCell;

        let result = Rc::new(RefCell::new(vec![]));
        let a = |i: u8| {
            result.borrow_mut().push(i);
            if i == 255
            {
                assert_eq!(result.borrow().clone(), (0..=255).collect::<Vec<u8>>());
                std::process::exit(0)
            }
            else
            {
                i + 1
            }
        };

        let f = crate::Θ(a);
        f(0);
        panic!("unreachable");
    }

    #[test]
    fn it_works()
    {
        let f0 = |x: u8| x/3;
        let f1 = |x: u8| x*2;
        let f2 = |a: u8, b: u8| a + b;

        let f2c = f2.compose(|| 1);
        let y = f2c(1);

        let f = i_star_star(f0, f1, f2);
        let y = f(1, 1);
        println!("{y}");

        let f = o(f0, f1);
        let y = f(1);
        println!("{y}");
    }
}