use super::tag::BTag;
use crate::{alloc::ObjectAllocator, bucket::Bucket};
use flize::{
    ebr::Ebr,
    function_runner::{Function, FunctionRunner},
    Atomic, Shared, Shield,
};
use std::hash::Hash;

trait BucketCas {
    type K: Eq + Hash;
    type V;
    type A: ObjectAllocator<Bucket<Self::K, Self::V, Self::A>>;
}
