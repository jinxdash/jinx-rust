#![feature(trait_alias)]

trait A =;
trait A = std::fmt::Debug + Send;
