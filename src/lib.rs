#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::cargo)]
#![cfg_attr(not(debug_assertions), deny(warnings))]
#![forbid(unsafe_code)]
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::char_lit_as_u8,
    clippy::checked_conversions,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::disallowed_method,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::expl_impl_clone_on_copy,
    clippy::explicit_deref_methods,
    clippy::explicit_into_iter_loop,
    clippy::fallible_impl_from,
    clippy::filter_map_next,
    clippy::flat_map_option,
    clippy::float_cmp_const,
    clippy::fn_params_excessive_bools,
    clippy::from_iter_instead_of_collect,
    clippy::if_let_mutex,
    clippy::implicit_clone,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::invalid_upcast_comparisons,
    clippy::large_digit_groups,
    clippy::large_stack_arrays,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::manual_ok_or,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wild_err_arm,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::missing_errors_doc,
    clippy::missing_safety_doc,
    clippy::mut_mut,
    clippy::mutex_integer,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::needless_for_each,
    clippy::needless_pass_by_value,
    clippy::option_option,
    clippy::path_buf_push_overwrite,
    clippy::ptr_as_ptr,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::same_functions_in_if_condition,
    clippy::semicolon_if_nothing_returned,
    clippy::single_match_else,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_lit_as_bytes,
    clippy::string_to_string,
    clippy::todo,
    clippy::trait_duplication_in_bounds,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::useless_transmute,
    clippy::verbose_file_reads,
    clippy::zero_sized_map_values,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    rustdoc::all
)]
#![deny(
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
#![no_std]
#![doc = include_str!("../README.md")]

/// Used to emulate the keywords' behavior inside the closure passed to [`ForEachRepeat::for_each_repeat`].
///
/// # Example
///
/// ```
/// use for_each_repeat::{ForEachRepeat, LoopControl};
/// let r = (2..100).for_each_repeat(|x| {
///     if 323 % x == 0 {
///         return LoopControl::Break(x)
///     }
///
///     LoopControl::Continue(())
/// });
/// assert_eq!(r, Some(17));
/// ```
///
/// It is named `LoopControl` to avoid confusion with [`ControlFlow`][core::ops::ControlFlow] enum from `std`.
#[derive(Debug, PartialEq, Eq, Copy, Clone, Ord, PartialOrd, Hash)]
pub enum LoopControl<B = (), C = (), S = ()> {
    /// Break out of the loop and optionally return a resulting value.
    Break(B),
    /// Skip the rest of the loop and continue the next iteration advancing the iterator.
    Continue(C),
    /// Come back to the beginning of the loop not advancing the iterator, with the given value for the iteration.
    Repeat(S),
}

impl<C, S> LoopControl<(), C, S> {
    /// The `Break` variant holding a unit if you don't want to write `(())` all the time.
    #[allow(dead_code)]
    pub const BREAK: Self = Self::Break(());
}
impl<B, S> LoopControl<B, (), S> {
    /// The `Continue` variant holding a unit if you don't want to write `(())` all the time.
    #[allow(dead_code)]
    pub const CONTINUE: Self = Self::Continue(());
}
impl<B, C> LoopControl<B, C, ()> {
    /// The `Repeat` variant holding a unit if you don't want to write `(())` all the time.
    #[allow(dead_code)]
    pub const REPEAT: Self = Self::Repeat(());
}

impl<B, C, S> core::fmt::Display for LoopControl<B, C, S>
where
    B: core::fmt::Display,
    C: core::fmt::Display,
    S: core::fmt::Display,
{
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            LoopControl::Break(b) => write!(f, "LoopControl::Break({b})"),
            LoopControl::Continue(c) => write!(f, "LoopControl::Continue({c})"),
            LoopControl::Repeat(s) => write!(f, "LoopControl::Repeat({s})"),
        }
    }
}

/// Consumes the iterator by calling the closure on each element, the next iteration is controlled by returned [`LoopControl`] variant.
///
/// It works similar to [`try_for_each`][Iterator::try_for_each] method on Iterator.
///
/// # Example
///
/// Basic example:
///
/// ```
/// use for_each_repeat::{LoopControl, ForEachRepeat};
///
/// let r = (2..100).for_each_repeat(|x| {
///     if 403 % x == 0 {
///         return LoopControl::Break(x);
///     }
///
///     LoopControl::CONTINUE
/// });
/// assert_eq!(r, Some(13));
/// ```
///
/// # [`Repeat`][LoopControl::Repeat] variant:
///
/// ```
/// use for_each_repeat::{LoopControl, ForEachRepeat};
///
/// let mut xs = vec![1, 2, 3, 4, 5];
///
/// let break_value: Option<()> = xs.iter_mut().for_each_repeat(|x| {
///     if *x < 5 {
///         *x += 1;
///         return LoopControl::Repeat(x);
///     }
///     LoopControl::CONTINUE
/// });
/// assert_eq!(xs, &[5; 5]);
/// assert_eq!(break_value, None);
/// ```
///
/// # Blanket `impl`
///
/// This trait is implemented for all [`Iterator`]s automatically -- you don't need to implement it. Just import it in your code:
///
/// ```
/// use for_each_repeat::ForEachRepeat;
/// ```
pub trait ForEachRepeat {
    /// Consumes the iterator, calls closure for each element. Next iteration is controlled by [`LoopControl`] variant.
    ///
    /// # Example
    ///
    /// ```
    /// use for_each_repeat::{LoopControl, ForEachRepeat};
    ///
    /// let r = (2..100).for_each_repeat(|x| {
    ///     if 403 % x == 0 {
    ///         return LoopControl::Break(x);
    ///     }
    ///
    ///     LoopControl::CONTINUE
    /// });
    /// assert_eq!(r, Some(13));
    /// ```
    ///
    /// See [trait's][ForEachRepeat] documentation for more.
    /// See also: [`Iterator::try_for_each`].
    #[inline]
    fn for_each_repeat<B, C>(
        &mut self,
        mut f: impl FnMut(Self::Item) -> LoopControl<B, C, Self::Item>,
    ) -> Option<B>
    where
        Self: Iterator,
    {
        let mut next = self.next();
        while let Some(item) = next {
            match f(item) {
                LoopControl::Break(r) => return Some(r),
                LoopControl::Continue(_) => next = self.next(),
                LoopControl::Repeat(s) => next = Some(s),
            }
        }
        None
    }
}

impl<T: Iterator> ForEachRepeat for T {}

#[cfg(test)]
mod tests {
    use super::{ForEachRepeat, LoopControl};

    #[test]
    fn it_works() {
        let mut values = 0;
        let mut repeat_counter = 0;
        let res = (0..=10).for_each_repeat(|x| {
            if x % 2 == 0 {
                return LoopControl::CONTINUE;
            }

            values *= 10;
            values += x;

            if x == 3 && repeat_counter <= 3 {
                repeat_counter += 1;
                return LoopControl::Repeat(x);
            }
            if x >= 8 {
                return LoopControl::Break(x);
            }

            LoopControl::CONTINUE
        });

        assert_eq!(res, Some(9));
        assert_eq!(values, 133_333_579);
    }
}
