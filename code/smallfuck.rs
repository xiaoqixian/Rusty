// Date: Sun Dec 24 21:46:27 2023
// Mail: lunar_ubuntu@qq.com
// Author: https://github.com/xiaoqixian

use std::marker::PhantomData;

trait Bit {}

struct One;
struct Zero;

impl Bit for One {}
impl Bit for Zero {}

trait List {}

struct Cons<B: Bit, L: List> (PhantomData<(B, L)>);
struct Nil;

impl<B: Bit, L: List> List for Cons<B, L> {}
impl List for Nil {}

struct State<L: List, C: Bit, R: List>(
    PhantomData<(L, C, R)>
);

struct Empty;
struct Left<T>(PhantomData<T>);
struct Right<T>(PhantomData<T>);
struct Flip<T>(PhantomData<T>);
struct Judge<P, Q>(PhantomData<(P, Q)>);

/// The input type parameter is the State
trait Transform<S = Empty> {
    type Output;
}

impl<P: Bit, C: Bit, L: List, R: List, T: Transform> 
Transform<State<Cons<P, L>, C, R>> for Left<T> 
where T: Transform<State<L, P, Cons<C, R>>> {
    type Output = <T as Transform<State<L, P, Cons<C, R>>>>::Output;
}

impl<N: Bit, C: Bit, L: List, R: List, T: Transform> 
Transform<State<L, C, Cons<N, R>>> for Right<T> 
where T: Transform<State<Cons<C, L>, N, R>> {
    type Output = <T as Transform<State<Cons<C, L>, N, R>>>::Output;
}

impl<L: List, R: List, T: Transform>
Transform<State<L, One, R>> for Flip<T> 
where T: Transform<State<L, Zero, R>> {
    type Output = <T as Transform<State<L, Zero, R>>>::Output;
}

impl<L: List, R: List, T: Transform>
Transform<State<L, Zero, R>> for Flip<T> 
where T: Transform<State<L, One, R>> {
    type Output = <T as Transform<State<L, One, R>>>::Output;
}

/// Jump to P if the current bit is One
/// Otherwise jump to Q
impl<L: List, R: List, P: Transform, Q: Transform>
Transform<State<L, One, R>> for Judge<P, Q> 
where P: Transform<State<L, One, R>> {
    type Output = <P as Transform<State<L, One, R>>>::Output;
}

impl<L: List, R: List, P: Transform, Q: Transform>
Transform<State<L, Zero, R>> for Judge<P, Q> 
where Q: Transform<State<L, Zero, R>> {
    type Output = <Q as Transform<State<L, Zero, R>>>::Output;
}

fn main() {

}
