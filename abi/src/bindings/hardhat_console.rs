pub use hardhat_console::*;
/// This module was auto-generated with ethers-rs Abigen.
/// More information at: <https://github.com/gakonst/ethers-rs>
#[allow(
    clippy::enum_variant_names,
    clippy::too_many_arguments,
    clippy::upper_case_acronyms,
    clippy::type_complexity,
    dead_code,
    non_camel_case_types,
)]
pub mod hardhat_console {
    #[rustfmt::skip]
    const __ABI: &str = "[{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"bool\",\"name\":\"p2\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p3\",\"type\":\"uint256\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"bool\",\"name\":\"p1\",\"type\":\"bool\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"string\",\"name\":\"p1\",\"type\":\"string\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"bool\",\"name\":\"p3\",\"type\":\"bool\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p2\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"},{\"internalType\":\"address\",\"name\":\"p1\",\"type\":\"address\"},{\"internalType\":\"uint256\",\"name\":\"p2\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p3\",\"type\":\"string\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"},{\"internalType\":\"uint256\",\"name\":\"p1\",\"type\":\"uint256\"},{\"internalType\":\"string\",\"name\":\"p2\",\"type\":\"string\"},{\"internalType\":\"address\",\"name\":\"p3\",\"type\":\"address\"}],\"name\":\"log\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"address\",\"name\":\"p0\",\"type\":\"address\"}],\"name\":\"logAddress\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bool\",\"name\":\"p0\",\"type\":\"bool\"}],\"name\":\"logBool\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes\",\"name\":\"p0\",\"type\":\"bytes\"}],\"name\":\"logBytes\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes1\",\"name\":\"p0\",\"type\":\"bytes1\"}],\"name\":\"logBytes1\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes10\",\"name\":\"p0\",\"type\":\"bytes10\"}],\"name\":\"logBytes10\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes11\",\"name\":\"p0\",\"type\":\"bytes11\"}],\"name\":\"logBytes11\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes12\",\"name\":\"p0\",\"type\":\"bytes12\"}],\"name\":\"logBytes12\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes13\",\"name\":\"p0\",\"type\":\"bytes13\"}],\"name\":\"logBytes13\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes14\",\"name\":\"p0\",\"type\":\"bytes14\"}],\"name\":\"logBytes14\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes15\",\"name\":\"p0\",\"type\":\"bytes15\"}],\"name\":\"logBytes15\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes16\",\"name\":\"p0\",\"type\":\"bytes16\"}],\"name\":\"logBytes16\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes17\",\"name\":\"p0\",\"type\":\"bytes17\"}],\"name\":\"logBytes17\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes18\",\"name\":\"p0\",\"type\":\"bytes18\"}],\"name\":\"logBytes18\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes19\",\"name\":\"p0\",\"type\":\"bytes19\"}],\"name\":\"logBytes19\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes2\",\"name\":\"p0\",\"type\":\"bytes2\"}],\"name\":\"logBytes2\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes20\",\"name\":\"p0\",\"type\":\"bytes20\"}],\"name\":\"logBytes20\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes21\",\"name\":\"p0\",\"type\":\"bytes21\"}],\"name\":\"logBytes21\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes22\",\"name\":\"p0\",\"type\":\"bytes22\"}],\"name\":\"logBytes22\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes23\",\"name\":\"p0\",\"type\":\"bytes23\"}],\"name\":\"logBytes23\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes24\",\"name\":\"p0\",\"type\":\"bytes24\"}],\"name\":\"logBytes24\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes25\",\"name\":\"p0\",\"type\":\"bytes25\"}],\"name\":\"logBytes25\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes26\",\"name\":\"p0\",\"type\":\"bytes26\"}],\"name\":\"logBytes26\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes27\",\"name\":\"p0\",\"type\":\"bytes27\"}],\"name\":\"logBytes27\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes28\",\"name\":\"p0\",\"type\":\"bytes28\"}],\"name\":\"logBytes28\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes29\",\"name\":\"p0\",\"type\":\"bytes29\"}],\"name\":\"logBytes29\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes3\",\"name\":\"p0\",\"type\":\"bytes3\"}],\"name\":\"logBytes3\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes30\",\"name\":\"p0\",\"type\":\"bytes30\"}],\"name\":\"logBytes30\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes31\",\"name\":\"p0\",\"type\":\"bytes31\"}],\"name\":\"logBytes31\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes32\",\"name\":\"p0\",\"type\":\"bytes32\"}],\"name\":\"logBytes32\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes4\",\"name\":\"p0\",\"type\":\"bytes4\"}],\"name\":\"logBytes4\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes5\",\"name\":\"p0\",\"type\":\"bytes5\"}],\"name\":\"logBytes5\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes6\",\"name\":\"p0\",\"type\":\"bytes6\"}],\"name\":\"logBytes6\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes7\",\"name\":\"p0\",\"type\":\"bytes7\"}],\"name\":\"logBytes7\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes8\",\"name\":\"p0\",\"type\":\"bytes8\"}],\"name\":\"logBytes8\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"bytes9\",\"name\":\"p0\",\"type\":\"bytes9\"}],\"name\":\"logBytes9\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"p0\",\"type\":\"int256\"}],\"name\":\"logInt\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"}],\"name\":\"logString\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"uint256\",\"name\":\"p0\",\"type\":\"uint256\"}],\"name\":\"logUint\",\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\"},{\"inputs\":[{\"internalType\":\"int256\",\"name\":\"p0\",\"type\":\"int256\"}],\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"log\"},{\"inputs\":[{\"internalType\":\"string\",\"name\":\"p0\",\"type\":\"string\"},{\"internalType\":\"int256\",\"name\":\"p1\",\"type\":\"int256\"}],\"outputs\":[],\"stateMutability\":\"view\",\"type\":\"function\",\"name\":\"log\"}]";
    ///The parsed JSON ABI of the contract.
    pub static HARDHATCONSOLE_ABI: ::ethers_contract::Lazy<::ethers_core::abi::Abi> = ::ethers_contract::Lazy::new(||
    ::ethers_core::utils::__serde_json::from_str(__ABI).expect("ABI is always valid"));
    pub struct HardhatConsole<M>(::ethers_contract::Contract<M>);
    impl<M> ::core::clone::Clone for HardhatConsole<M> {
        fn clone(&self) -> Self {
            Self(::core::clone::Clone::clone(&self.0))
        }
    }
    impl<M> ::core::ops::Deref for HardhatConsole<M> {
        type Target = ::ethers_contract::Contract<M>;
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    impl<M> ::core::ops::DerefMut for HardhatConsole<M> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            &mut self.0
        }
    }
    impl<M> ::core::fmt::Debug for HardhatConsole<M> {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            f.debug_tuple(stringify!(HardhatConsole)).field(&self.address()).finish()
        }
    }
    impl<M: ::ethers_providers::Middleware> HardhatConsole<M> {
        /// Creates a new contract instance with the specified `ethers` client at
        /// `address`. The contract derefs to a `ethers::Contract` object.
        pub fn new<T: Into<::ethers_core::types::Address>>(
            address: T,
            client: ::std::sync::Arc<M>,
        ) -> Self {
            Self(
                ::ethers_contract::Contract::new(
                    address.into(),
                    HARDHATCONSOLE_ABI.clone(),
                    client,
                ),
            )
        }
        ///Calls the contract's `log` (0x007150be) function
        pub fn log_23(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 113, 80, 190], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x00dd87b9) function
        pub fn log_87(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 221, 135, 185], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x018c84c2) function
        pub fn log_24(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([1, 140, 132, 194], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x031c6f73) function
        pub fn log_88(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([3, 28, 111, 115], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0454c079) function
        pub fn log_89(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([4, 84, 192, 121], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x078287f5) function
        pub fn log_90(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 130, 135, 245], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x07831502) function
        pub fn log_91(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([7, 131, 21, 2], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x088ef9d2) function
        pub fn log_25(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([8, 142, 249, 210], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x091ffaf5) function
        pub fn log_92(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 31, 250, 245], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0aa6cfad) function
        pub fn log_93(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([10, 166, 207, 173], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0bb00eab) function
        pub fn log_94(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 176, 14, 171], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0c66d1be) function
        pub fn log_95(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 102, 209, 190], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0c9cd9c1) function
        pub fn log_96(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([12, 156, 217, 193], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0d26b925) function
        pub fn log_26(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 38, 185, 37], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0d36fa20) function
        pub fn log_97(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 54, 250, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0df12b76) function
        pub fn log_98(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([13, 241, 43, 118], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0e378994) function
        pub fn log_99(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 55, 137, 148], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x0ef7e050) function
        pub fn log_100(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([14, 247, 224, 80], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x100f650e) function
        pub fn log_101(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 15, 101, 14], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1023f7b2) function
        pub fn log_102(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 35, 247, 178], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1078f68d) function
        pub fn log_27(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 120, 246, 141], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1093ee11) function
        pub fn log_28(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([16, 147, 238, 17], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x12d6c788) function
        pub fn log_103(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 214, 199, 136], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x12f21602) function
        pub fn log_29(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([18, 242, 22, 2], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x136b05dd) function
        pub fn log_104(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([19, 107, 5, 221], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1537dc87) function
        pub fn log_105(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 55, 220, 135], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1596a1ce) function
        pub fn log_106(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 150, 161, 206], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x159f8927) function
        pub fn log_107(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 159, 137, 39], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x15c127b5) function
        pub fn log_108(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 193, 39, 181], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x15cac476) function
        pub fn log_109(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([21, 202, 196, 118], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1606a393) function
        pub fn log_110(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([22, 6, 163, 147], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1762e32a) function
        pub fn log_111(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 98, 227, 42], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x17fe6185) function
        pub fn log_30(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([23, 254, 97, 133], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x18c9c746) function
        pub fn log_31(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([24, 201, 199, 70], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x193fb800) function
        pub fn log_112(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 63, 184, 0], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x19fd4956) function
        pub fn log_113(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([25, 253, 73, 86], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1ad96de6) function
        pub fn log_114(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([26, 217, 109, 230], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1bb3b09a) function
        pub fn log_115(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([27, 179, 176, 154], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1c41a336) function
        pub fn log_116(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 65, 163, 54], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1c7ec448) function
        pub fn log_32(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 126, 196, 72], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1c9d7eb3) function
        pub fn log_6(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([28, 157, 126, 179], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1d14d001) function
        pub fn log_117(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 20, 208, 1], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1da986ea) function
        pub fn log_118(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 169, 134, 234], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1dc8e1b8) function
        pub fn log_119(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([29, 200, 225, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x1e4b87e5) function
        pub fn log_120(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([30, 75, 135, 229], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x20098014) function
        pub fn log_33(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 9, 128, 20], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x205871c2) function
        pub fn log_121(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 88, 113, 194], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x20718650) function
        pub fn log_34(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 113, 134, 80], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x20e3984d) function
        pub fn log_122(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([32, 227, 152, 77], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x212255cc) function
        pub fn log_35(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 34, 85, 204], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x21ad0683) function
        pub fn log_123(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 173, 6, 131], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x21bdaf25) function
        pub fn log_124(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([33, 189, 175, 37], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x223603bd) function
        pub fn log_125(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 54, 3, 189], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x22f6b999) function
        pub fn log_126(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([34, 246, 185, 153], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x245986f2) function
        pub fn log_127(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 89, 134, 242], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2488b414) function
        pub fn log_128(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 136, 180, 20], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x24f91465) function
        pub fn log_129(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([36, 249, 20, 101], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2555fa46) function
        pub fn log_36(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([37, 85, 250, 70], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x26f560a8) function
        pub fn log_130(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([38, 245, 96, 168], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x27d8afd2) function
        pub fn log_131(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([39, 216, 175, 210], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x28863fcb) function
        pub fn log_132(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([40, 134, 63, 203], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2a110e83) function
        pub fn log_7(
            &self,
            p_0: bool,
            p_1: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 17, 14, 131], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2ae408d4) function
        pub fn log_133(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([42, 228, 8, 212], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2b2b18dc) function
        pub fn log_134(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([43, 43, 24, 220], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2c1754ed) function
        pub fn log_135(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 23, 84, 237], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2c1d0746) function
        pub fn log_136(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 29, 7, 70], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2c2ecbc2) function
        pub fn log_1(
            &self,
            p_0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 46, 203, 194], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2cd4134a) function
        pub fn log_137(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 212, 19, 74], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2ced7cef) function
        pub fn log_37(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([44, 237, 124, 239], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2d8e33a4) function
        pub fn log_138(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 142, 51, 164], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2dd778e6) function
        pub fn log_139(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 215, 120, 230], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x319af333) function
        pub fn log_8(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([49, 154, 243, 51], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x32458eed) function
        pub fn log_2(
            &self,
            p_0: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([50, 69, 142, 237], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x33e9dd1d) function
        pub fn log_140(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([51, 233, 221, 29], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x34f0e636) function
        pub fn log_141(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 240, 230, 54], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x35085f7b) function
        pub fn log_38(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 8, 95, 123], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x354c36d6) function
        pub fn log_142(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 76, 54, 214], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x35a5071f) function
        pub fn log_143(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([53, 165, 7, 31], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x37103367) function
        pub fn log_39(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 16, 51, 103], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x374bb4b2) function
        pub fn log_144(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 75, 180, 178], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x37aa7d4c) function
        pub fn log_40(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([55, 170, 125, 76], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x386ff5f4) function
        pub fn log_145(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 111, 245, 244], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3971e78c) function
        pub fn log_146(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 113, 231, 140], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x399174d3) function
        pub fn log_9(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([57, 145, 116, 211], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3b2279b4) function
        pub fn log_147(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 34, 121, 180], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3b2a5ce0) function
        pub fn log_148(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 42, 92, 224], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3bf5e537) function
        pub fn log_149(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([59, 245, 229, 55], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3e128ca3) function
        pub fn log_150(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 18, 140, 163], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3e9f866a) function
        pub fn log_151(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([62, 159, 134, 106], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3f8a701d) function
        pub fn log_152(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([63, 138, 112, 29], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x40785869) function
        pub fn log_153(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([64, 120, 88, 105], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x41304fac) function
        pub fn log_3(
            &self,
            p_0: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([65, 48, 79, 172], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x42d21db7) function
        pub fn log_154(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([66, 210, 29, 183], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x439c7bef) function
        pub fn log_155(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([67, 156, 123, 239], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x448830a8) function
        pub fn log_156(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([68, 136, 48, 168], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x454d54a5) function
        pub fn log_157(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 77, 84, 165], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x457fe3cf) function
        pub fn log_158(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 127, 227, 207], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x46600be0) function
        pub fn log_159(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 96, 11, 224], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x46826b5d) function
        pub fn log_160(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([70, 130, 107, 93], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x475c5c33) function
        pub fn log_161(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 92, 92, 51], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4766da72) function
        pub fn log_41(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 102, 218, 114], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x478d1c62) function
        pub fn log_162(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([71, 141, 28, 98], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x483d0416) function
        pub fn log_163(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([72, 61, 4, 22], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4a28c017) function
        pub fn log_164(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 40, 192, 23], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4a66cb34) function
        pub fn log_165(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([74, 102, 203, 52], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4b5c4277) function
        pub fn log_10(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([75, 92, 66, 119], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4c123d57) function
        pub fn log_166(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 18, 61, 87], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4ceda75a) function
        pub fn log_42(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([76, 237, 167, 90], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x4f04fdc6) function
        pub fn log_167(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([79, 4, 253, 198], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x50709698) function
        pub fn log_43(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 112, 150, 152], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x50ad461d) function
        pub fn log_168(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 173, 70, 29], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x515e38b6) function
        pub fn log_169(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 94, 56, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x51973ec9) function
        pub fn log_0(&self) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 151, 62, 201], ())
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x51f09ff8) function
        pub fn log_170(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([81, 240, 159, 248], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x538e06ab) function
        pub fn log_171(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([83, 142, 6, 171], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x54a7a9a0) function
        pub fn log_172(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([84, 167, 169, 160], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x56a5d1b1) function
        pub fn log_173(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([86, 165, 209, 177], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5821efa1) function
        pub fn log_44(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([88, 33, 239, 161], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5970e089) function
        pub fn log_45(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 112, 224, 137], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x59cfcbe3) function
        pub fn log_174(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 207, 203, 227], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5a477632) function
        pub fn log_175(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 71, 118, 50], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5a9b5ed5) function
        pub fn log_46(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 155, 94, 213], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5ab84e1f) function
        pub fn log_176(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 184, 78, 31], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5abd992a) function
        pub fn log_177(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([90, 189, 153, 42], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5c430d47) function
        pub fn log_178(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 67, 13, 71], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5c96b331) function
        pub fn log_47(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 150, 179, 49], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5ccd4e37) function
        pub fn log_179(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([92, 205, 78, 55], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5d02c50b) function
        pub fn log_180(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 2, 197, 11], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5d08bb05) function
        pub fn log_181(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 8, 187, 5], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5d1a971a) function
        pub fn log_182(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 26, 151, 26], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5da297eb) function
        pub fn log_183(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([93, 162, 151, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5e84b0ea) function
        pub fn log_184(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 132, 176, 234], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5ea2b7ae) function
        pub fn log_185(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([94, 162, 183, 174], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f15d28c) function
        pub fn log_186(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 21, 210, 140], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f1d5c9f) function
        pub fn log_187(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 29, 92, 159], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f743a7c) function
        pub fn log_188(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 116, 58, 124], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x5f7b9afb) function
        pub fn log_48(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 123, 154, 251], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6168ed61) function
        pub fn log_189(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 104, 237, 97], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x619e4d0e) function
        pub fn log_190(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([97, 158, 77, 14], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x63183678) function
        pub fn log_191(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 24, 54, 120], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x63cb41f9) function
        pub fn log_49(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 203, 65, 249], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x63fb8bc5) function
        pub fn log_192(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([99, 251, 139, 197], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x643fd0df) function
        pub fn log_11(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 63, 208, 223], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x64b5bb67) function
        pub fn log_193(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([100, 181, 187, 103], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x660375dd) function
        pub fn log_194(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 3, 117, 221], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x665bf134) function
        pub fn log_195(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 91, 241, 52], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x66f1bc67) function
        pub fn log_196(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([102, 241, 188, 103], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x678209a8) function
        pub fn log_50(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 130, 9, 168], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x67dd6ff1) function
        pub fn log_51(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([103, 221, 111, 241], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x68c8b8bd) function
        pub fn log_197(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([104, 200, 184, 189], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x691a8f74) function
        pub fn log_198(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 26, 143, 116], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x69276c86) function
        pub fn log_12(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 39, 108, 134], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x69640b59) function
        pub fn log_199(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([105, 100, 11, 89], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6a1199e2) function
        pub fn log_200(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 17, 153, 226], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6a9c478b) function
        pub fn log_201(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([106, 156, 71, 139], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6b0e5d53) function
        pub fn log_202(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([107, 14, 93, 83], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6cde40b8) function
        pub fn log_203(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([108, 222, 64, 184], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d1e8751) function
        pub fn log_204(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 30, 135, 81], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d572f44) function
        pub fn log_205(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 87, 47, 68], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6d7045c1) function
        pub fn log_206(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 112, 69, 193], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6dd434ca) function
        pub fn log_207(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([109, 212, 52, 202], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6f1a594e) function
        pub fn log_208(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 26, 89, 78], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x6f7c603e) function
        pub fn log_209(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 124, 96, 62], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7190a529) function
        pub fn log_210(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 144, 165, 41], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x71d04af2) function
        pub fn log_52(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([113, 208, 74, 242], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x736efbb6) function
        pub fn log_211(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([115, 110, 251, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x742d6ee7) function
        pub fn log_212(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 45, 110, 231], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7464ce23) function
        pub fn log_213(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([116, 100, 206, 35], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x759f86bb) function
        pub fn log_13(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 159, 134, 187], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x75b605d3) function
        pub fn log_14(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([117, 182, 5, 211], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7626db92) function
        pub fn log_214(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 38, 219, 146], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x79884c2b) function
        pub fn log_215(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([121, 136, 76, 43], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7af6ab25) function
        pub fn log_216(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 246, 171, 37], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7afac959) function
        pub fn log_53(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 250, 201, 89], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7bc0d848) function
        pub fn log_54(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 192, 216, 72], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7be0c3eb) function
        pub fn log_217(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 224, 195, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7bf181a1) function
        pub fn log_218(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([123, 241, 129, 161], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7c4632a4) function
        pub fn log_219(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 70, 50, 164], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7cc3c607) function
        pub fn log_220(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([124, 195, 198, 7], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7d24491d) function
        pub fn log_221(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 36, 73, 29], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7dd4d0e0) function
        pub fn log_222(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([125, 212, 208, 224], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x7f9bbca2) function
        pub fn log_223(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([127, 155, 188, 162], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x800a1c67) function
        pub fn log_224(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 10, 28, 103], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x80e6a20b) function
        pub fn log_225(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 230, 162, 11], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x82112a42) function
        pub fn log_226(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 17, 42, 66], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x82c25b74) function
        pub fn log_227(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([130, 194, 91, 116], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8309e8a8) function
        pub fn log_15(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([131, 9, 232, 168], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x850b7ad6) function
        pub fn log_55(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 11, 122, 214], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x853c4849) function
        pub fn log_16(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 60, 72, 73], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x854b3496) function
        pub fn log_228(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 75, 52, 150], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x85775021) function
        pub fn log_56(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([133, 119, 80, 33], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x88a8c406) function
        pub fn log_229(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 168, 196, 6], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x88cb6041) function
        pub fn log_230(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 203, 96, 65], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x88f6e4b2) function
        pub fn log_231(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([136, 246, 228, 178], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x895af8c5) function
        pub fn log_232(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([137, 90, 248, 197], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8af7cf8a) function
        pub fn log_233(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([138, 247, 207, 138], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8c329b1a) function
        pub fn log_234(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 50, 155, 26], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8c4e5de6) function
        pub fn log_235(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([140, 78, 93, 230], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8da6def5) function
        pub fn log_236(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([141, 166, 222, 245], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8e3f78a9) function
        pub fn log_237(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 63, 120, 169], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8e69fb5d) function
        pub fn log_238(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 105, 251, 93], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8eafb02b) function
        pub fn log_239(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 175, 176, 43], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8ef3f399) function
        pub fn log_240(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([142, 243, 243, 153], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8f736d16) function
        pub fn log_241(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 115, 109, 22], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x8feac525) function
        pub fn log_17(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([143, 234, 197, 37], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x90c30a56) function
        pub fn log_242(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 195, 10, 86], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x90fb06aa) function
        pub fn log_243(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([144, 251, 6, 170], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9143dbb1) function
        pub fn log_244(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 67, 219, 177], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x91a02e2a) function
        pub fn log_245(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 160, 46, 42], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x91d1112e) function
        pub fn log_246(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([145, 209, 17, 46], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x932bbb38) function
        pub fn log_57(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 43, 187, 56], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x935e09bf) function
        pub fn log_247(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([147, 94, 9, 191], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x94250d77) function
        pub fn log_248(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([148, 37, 13, 119], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x958c28c6) function
        pub fn log_249(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 140, 40, 198], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9591b953) function
        pub fn log_58(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 145, 185, 83], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x95ed0195) function
        pub fn log_59(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([149, 237, 1, 149], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x97d394d8) function
        pub fn log_250(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([151, 211, 148, 216], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9a816a83) function
        pub fn log_251(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 129, 106, 131], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9acd3616) function
        pub fn log_252(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([154, 205, 54, 22], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9b4254e2) function
        pub fn log_253(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 66, 84, 226], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9b6ec042) function
        pub fn log_60(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 110, 192, 66], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9c3adfa1) function
        pub fn log_254(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 58, 223, 161], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9c4f99fb) function
        pub fn log_61(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 79, 153, 251], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9cba8fff) function
        pub fn log_255(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([156, 186, 143, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9d22d5dd) function
        pub fn log_256(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 34, 213, 221], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9f1bc36e) function
        pub fn log_257(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 27, 195, 110], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x9ffb2f93) function
        pub fn log_258(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([159, 251, 47, 147], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa04e2f87) function
        pub fn log_259(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 78, 47, 135], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa0a47963) function
        pub fn log_260(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([160, 164, 121, 99], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa1bcc9b3) function
        pub fn log_261(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 188, 201, 179], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa1ef4cbb) function
        pub fn log_262(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 239, 76, 187], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa1f2e8aa) function
        pub fn log_62(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([161, 242, 232, 170], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa31bfdcc) function
        pub fn log_263(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([163, 27, 253, 204], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa5b4fc99) function
        pub fn log_264(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 180, 252, 153], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa5cada94) function
        pub fn log_265(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([165, 202, 218, 148], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa6f50b0f) function
        pub fn log_266(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([166, 245, 11, 15], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa73c1db6) function
        pub fn log_267(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 60, 29, 182], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa75c59de) function
        pub fn log_268(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 92, 89, 222], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa7a87853) function
        pub fn log_269(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([167, 168, 120, 83], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xa826caeb) function
        pub fn log_270(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([168, 38, 202, 235], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xaa6540c8) function
        pub fn log_271(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 101, 64, 200], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xaabc9a31) function
        pub fn log_272(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([170, 188, 154, 49], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xab085ae6) function
        pub fn log_273(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 8, 90, 230], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xabf73a98) function
        pub fn log_274(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([171, 247, 58, 152], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xade052c7) function
        pub fn log_275(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([173, 224, 82, 199], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xae2ec581) function
        pub fn log_276(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 46, 197, 129], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb028c9bd) function
        pub fn log_277(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 40, 201, 189], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb076847f) function
        pub fn log_63(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 118, 132, 127], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb0e0f9b5) function
        pub fn log_64(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([176, 224, 249, 181], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb115611f) function
        pub fn log_65(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([177, 21, 97, 31], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb3a6b6bd) function
        pub fn log_278(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([179, 166, 182, 189], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb4c314ff) function
        pub fn log_279(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([180, 195, 20, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb59dbd60) function
        pub fn log_280(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([181, 157, 189, 96], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb60e72cc) function
        pub fn log_18(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 14, 114, 204], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb69bcaf6) function
        pub fn log_66(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 155, 202, 246], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb6f577a1) function
        pub fn log_281(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([182, 245, 119, 161], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb7b914ca) function
        pub fn log_282(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([183, 185, 20, 202], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xb857163a) function
        pub fn log_283(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([184, 87, 22, 58], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xba535d9c) function
        pub fn log_284(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 83, 93, 156], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbc0b61fe) function
        pub fn log_285(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 11, 97, 254], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbcfd9be0) function
        pub fn log_67(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([188, 253, 155, 224], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbe553481) function
        pub fn log_286(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 85, 52, 129], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbe984353) function
        pub fn log_287(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([190, 152, 67, 83], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xbf01f891) function
        pub fn log_288(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([191, 1, 248, 145], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc0a302d8) function
        pub fn log_289(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([192, 163, 2, 216], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc21f64c7) function
        pub fn log_290(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([194, 31, 100, 199], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc371c7db) function
        pub fn log_291(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 113, 199, 219], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc3a8a654) function
        pub fn log_292(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 168, 166, 84], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc3b55635) function
        pub fn log_19(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 181, 86, 53], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc3fc3970) function
        pub fn log_68(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([195, 252, 57, 112], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc4643e20) function
        pub fn log_293(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 100, 62, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc598d185) function
        pub fn log_294(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 152, 209, 133], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc5ad85f9) function
        pub fn log_295(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([197, 173, 133, 249], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc67ea9d1) function
        pub fn log_296(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 126, 169, 209], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc6acc7a8) function
        pub fn log_297(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([198, 172, 199, 168], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc91d5ed4) function
        pub fn log_69(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 29, 94, 212], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xc95958d6) function
        pub fn log_70(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([201, 89, 88, 214], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xca47c4eb) function
        pub fn log_71(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 71, 196, 235], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xca7733b1) function
        pub fn log_72(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 119, 51, 177], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcac43479) function
        pub fn log_298(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([202, 196, 52, 121], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcc32ab07) function
        pub fn log_299(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 50, 171, 7], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xccf790a1) function
        pub fn log_300(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([204, 247, 144, 161], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xceb5f4d7) function
        pub fn log_301(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([206, 181, 244, 215], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcf009880) function
        pub fn log_302(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 0, 152, 128], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcf020fb1) function
        pub fn log_73(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 2, 15, 177], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcf18105c) function
        pub fn log_303(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 24, 16, 92], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xcf394485) function
        pub fn log_304(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([207, 57, 68, 133], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd1ed7a3c) function
        pub fn log_74(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([209, 237, 122, 60], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd2763667) function
        pub fn log_75(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 118, 54, 103], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd2d423cd) function
        pub fn log_305(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([210, 212, 35, 205], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd583c602) function
        pub fn log_306(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([213, 131, 198, 2], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd6019f1c) function
        pub fn log_307(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 1, 159, 28], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd6aefad2) function
        pub fn log_308(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([214, 174, 250, 210], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xd812a167) function
        pub fn log_309(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 18, 161, 103], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdaf0d4aa) function
        pub fn log_20(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([218, 240, 212, 170], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdbb4c247) function
        pub fn log_76(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([219, 180, 194, 71], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdc5e935b) function
        pub fn log_310(
            &self,
            p_0: bool,
            p_1: ::std::string::String,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 94, 147, 91], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xddb06521) function
        pub fn log_311(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 176, 101, 33], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdddb9561) function
        pub fn log_312(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([221, 219, 149, 97], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xde03e774) function
        pub fn log_313(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 3, 231, 116], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xde68f20a) function
        pub fn log_314(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 104, 242, 10], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xde9a9270) function
        pub fn log_77(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([222, 154, 146, 112], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xdfc4a2e8) function
        pub fn log_315(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: bool,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([223, 196, 162, 232], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe0625b29) function
        pub fn log_316(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 98, 91, 41], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe0e95b98) function
        pub fn log_317(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 233, 91, 152], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe0e9ad4f) function
        pub fn log_78(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([224, 233, 173, 79], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe21de278) function
        pub fn log_318(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 29, 226, 120], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe298f47d) function
        pub fn log_79(
            &self,
            p_0: ::std::string::String,
            p_1: bool,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 152, 244, 125], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe2bfd60b) function
        pub fn log_319(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([226, 191, 214, 11], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe351140f) function
        pub fn log_320(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 81, 20, 15], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe3a9ca2f) function
        pub fn log_321(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([227, 169, 202, 47], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe41b6f6f) function
        pub fn log_322(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([228, 27, 111, 111], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe5e70b2b) function
        pub fn log_323(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([229, 231, 11, 43], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe8d3018d) function
        pub fn log_324(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 211, 1, 141], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xe8defba9) function
        pub fn log_80(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([232, 222, 251, 169], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xeb1bff80) function
        pub fn log_325(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 27, 255, 128], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xeb7f6fd2) function
        pub fn log_326(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: bool,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 127, 111, 210], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xeb830c92) function
        pub fn log_81(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 131, 12, 146], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xeb928d7f) function
        pub fn log_327(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([235, 146, 141, 127], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xed8f28f6) function
        pub fn log_328(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([237, 143, 40, 246], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xef1cefe7) function
        pub fn log_329(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 28, 239, 231], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xef529018) function
        pub fn log_330(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 82, 144, 24], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xef72c513) function
        pub fn log_331(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([239, 114, 197, 19], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf08744e8) function
        pub fn log_82(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([240, 135, 68, 232], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf11699ed) function
        pub fn log_83(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([241, 22, 153, 237], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf2a66286) function
        pub fn log_84(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([242, 166, 98, 134], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf45d7d2c) function
        pub fn log_332(
            &self,
            p_0: ::std::string::String,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 93, 125, 44], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf4880ea4) function
        pub fn log_333(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::ethers_core::types::Address,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([244, 136, 14, 164], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf5bc2249) function
        pub fn log_334(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([245, 188, 34, 73], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf666715a) function
        pub fn log_21(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([246, 102, 113, 90], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf7e36245) function
        pub fn log_335(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([247, 227, 98, 69], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf808da20) function
        pub fn log_336(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 8, 218, 32], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf82c50f1) function
        pub fn log_4(
            &self,
            p_0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 44, 80, 241], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf8f51b1e) function
        pub fn log_337(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 245, 27, 30], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xf9ad2b89) function
        pub fn log_338(
            &self,
            p_0: bool,
            p_1: bool,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([249, 173, 43, 137], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfa8185af) function
        pub fn log_339(
            &self,
            p_0: ::ethers_core::types::U256,
            p_1: ::ethers_core::types::U256,
            p_2: ::ethers_core::types::U256,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([250, 129, 133, 175], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfb772265) function
        pub fn log_85(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::std::string::String,
            p_2: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 119, 34, 101], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfc4845f0) function
        pub fn log_340(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 72, 69, 240], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfcec75e0) function
        pub fn log_86(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 236, 117, 224], (p_0, p_1, p_2))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfdb4f990) function
        pub fn log_341(
            &self,
            p_0: ::ethers_core::types::Address,
            p_1: ::ethers_core::types::Address,
            p_2: ::ethers_core::types::U256,
            p_3: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([253, 180, 249, 144], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0xfedd1fff) function
        pub fn log_342(
            &self,
            p_0: bool,
            p_1: ::ethers_core::types::U256,
            p_2: ::std::string::String,
            p_3: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([254, 221, 31, 255], (p_0, p_1, p_2, p_3))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x2d5b6cb9) function
        pub fn log_5(
            &self,
            p_0: ::ethers_core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 91, 108, 185], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `log` (0x3ca6268e) function
        pub fn log_22(
            &self,
            p_0: ::std::string::String,
            p_1: ::ethers_core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 166, 38, 142], (p_0, p_1))
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logAddress` (0x5f91b0af) function
        pub fn log_address(
            &self,
            p_0: ::ethers_core::types::Address,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([95, 145, 176, 175], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBool` (0xba7ab84e) function
        pub fn log_bool(
            &self,
            p_0: bool,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([186, 122, 184, 78], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes` (0xe17bf956) function
        pub fn log_bytes(
            &self,
            p_0: ::ethers_core::types::Bytes,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([225, 123, 249, 86], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes1` (0x6f4171c9) function
        pub fn log_bytes_1(
            &self,
            p_0: [u8; 1],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([111, 65, 113, 201], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes10` (0x9dc2a897) function
        pub fn log_bytes_10(
            &self,
            p_0: [u8; 10],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([157, 194, 168, 151], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes11` (0xdc08b6a7) function
        pub fn log_bytes_11(
            &self,
            p_0: [u8; 11],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([220, 8, 182, 167], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes12` (0x7656d6c7) function
        pub fn log_bytes_12(
            &self,
            p_0: [u8; 12],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([118, 86, 214, 199], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes13` (0x34c1d81b) function
        pub fn log_bytes_13(
            &self,
            p_0: [u8; 13],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([52, 193, 216, 27], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes14` (0x3ceaba65) function
        pub fn log_bytes_14(
            &self,
            p_0: [u8; 14],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([60, 234, 186, 101], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes15` (0x591a3da2) function
        pub fn log_bytes_15(
            &self,
            p_0: [u8; 15],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([89, 26, 61, 162], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes16` (0x1f8d7312) function
        pub fn log_bytes_16(
            &self,
            p_0: [u8; 16],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([31, 141, 115, 18], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes17` (0xf89a532f) function
        pub fn log_bytes_17(
            &self,
            p_0: [u8; 17],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([248, 154, 83, 47], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes18` (0xd8652642) function
        pub fn log_bytes_18(
            &self,
            p_0: [u8; 18],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([216, 101, 38, 66], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes19` (0x00f56bc9) function
        pub fn log_bytes_19(
            &self,
            p_0: [u8; 19],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([0, 245, 107, 201], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes2` (0x9b5e943e) function
        pub fn log_bytes_2(
            &self,
            p_0: [u8; 2],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([155, 94, 148, 62], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes20` (0xecb8567e) function
        pub fn log_bytes_20(
            &self,
            p_0: [u8; 20],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([236, 184, 86, 126], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes21` (0x3052c08f) function
        pub fn log_bytes_21(
            &self,
            p_0: [u8; 21],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([48, 82, 192, 143], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes22` (0x807ab434) function
        pub fn log_bytes_22(
            &self,
            p_0: [u8; 22],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([128, 122, 180, 52], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes23` (0x4979b037) function
        pub fn log_bytes_23(
            &self,
            p_0: [u8; 23],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 121, 176, 55], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes24` (0x0977aefc) function
        pub fn log_bytes_24(
            &self,
            p_0: [u8; 24],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([9, 119, 174, 252], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes25` (0xaea9963f) function
        pub fn log_bytes_25(
            &self,
            p_0: [u8; 25],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([174, 169, 150, 63], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes26` (0xd3635628) function
        pub fn log_bytes_26(
            &self,
            p_0: [u8; 26],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([211, 99, 86, 40], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes27` (0xfc372f9f) function
        pub fn log_bytes_27(
            &self,
            p_0: [u8; 27],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([252, 55, 47, 159], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes28` (0x382f9a34) function
        pub fn log_bytes_28(
            &self,
            p_0: [u8; 28],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([56, 47, 154, 52], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes29` (0x7a187641) function
        pub fn log_bytes_29(
            &self,
            p_0: [u8; 29],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([122, 24, 118, 65], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes3` (0x7782fa2d) function
        pub fn log_bytes_3(
            &self,
            p_0: [u8; 3],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([119, 130, 250, 45], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes30` (0xc4340ef6) function
        pub fn log_bytes_30(
            &self,
            p_0: [u8; 30],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([196, 52, 14, 246], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes31` (0x81fc8648) function
        pub fn log_bytes_31(
            &self,
            p_0: [u8; 31],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([129, 252, 134, 72], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes32` (0x2d21d6f7) function
        pub fn log_bytes_32(
            &self,
            p_0: [u8; 32],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([45, 33, 214, 247], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes4` (0xfba3ad39) function
        pub fn log_bytes_4(
            &self,
            p_0: [u8; 4],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([251, 163, 173, 57], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes5` (0x5583be2e) function
        pub fn log_bytes_5(
            &self,
            p_0: [u8; 5],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([85, 131, 190, 46], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes6` (0x4942adc6) function
        pub fn log_bytes_6(
            &self,
            p_0: [u8; 6],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([73, 66, 173, 198], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes7` (0x4574afab) function
        pub fn log_bytes_7(
            &self,
            p_0: [u8; 7],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([69, 116, 175, 171], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes8` (0x9902e47f) function
        pub fn log_bytes_8(
            &self,
            p_0: [u8; 8],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 2, 228, 127], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logBytes9` (0x50a138df) function
        pub fn log_bytes_9(
            &self,
            p_0: [u8; 9],
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([80, 161, 56, 223], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logInt` (0x6525b5f5) function
        pub fn log_int(
            &self,
            p_0: ::ethers_core::types::I256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([101, 37, 181, 245], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logString` (0x0bb563d6) function
        pub fn log_string(
            &self,
            p_0: ::std::string::String,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([11, 181, 99, 214], p_0)
                .expect("method not found (this should never happen)")
        }
        ///Calls the contract's `logUint` (0x9905b744) function
        pub fn log_uint(
            &self,
            p_0: ::ethers_core::types::U256,
        ) -> ::ethers_contract::builders::ContractCall<M, ()> {
            self.0
                .method_hash([153, 5, 183, 68], p_0)
                .expect("method not found (this should never happen)")
        }
    }
    impl<M: ::ethers_providers::Middleware> From<::ethers_contract::Contract<M>>
    for HardhatConsole<M> {
        fn from(contract: ::ethers_contract::Contract<M>) -> Self {
            Self::new(contract.address(), contract.client())
        }
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string)` and selector `0x007150be`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string)")]
    pub struct Log23Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,address)` and selector `0x00dd87b9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,address)")]
    pub struct Log87Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address)` and selector `0x018c84c2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address)")]
    pub struct Log24Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,string)` and selector `0x031c6f73`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,string)")]
    pub struct Log88Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,string)` and selector `0x0454c079`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,string)")]
    pub struct Log89Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,uint256)` and selector `0x078287f5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,uint256)")]
    pub struct Log90Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,uint256)` and selector `0x07831502`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,uint256)")]
    pub struct Log91Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address)` and selector `0x088ef9d2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address)")]
    pub struct Log25Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,bool)` and selector `0x091ffaf5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,bool)")]
    pub struct Log92Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,string)` and selector `0x0aa6cfad`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,string)")]
    pub struct Log93Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,uint256)` and selector `0x0bb00eab`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,uint256)")]
    pub struct Log94Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,uint256)` and selector `0x0c66d1be`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,uint256)")]
    pub struct Log95Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,uint256)` and selector `0x0c9cd9c1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,uint256)")]
    pub struct Log96Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256)` and selector `0x0d26b925`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256)")]
    pub struct Log26Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,address)` and selector `0x0d36fa20`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,address)")]
    pub struct Log97Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,bool)` and selector `0x0df12b76`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,bool)")]
    pub struct Log98Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,bool)` and selector `0x0e378994`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,bool)")]
    pub struct Log99Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,bool)` and selector `0x0ef7e050`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,bool)")]
    pub struct Log100Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,uint256)` and selector `0x100f650e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,uint256)")]
    pub struct Log101Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,address)` and selector `0x1023f7b2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,address)")]
    pub struct Log102Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address)` and selector `0x1078f68d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address)")]
    pub struct Log27Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256)` and selector `0x1093ee11`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256)")]
    pub struct Log28Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,string)` and selector `0x12d6c788`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,string)")]
    pub struct Log103Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256)` and selector `0x12f21602`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256)")]
    pub struct Log29Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,address)` and selector `0x136b05dd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,address)")]
    pub struct Log104Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,uint256)` and selector `0x1537dc87`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,uint256)")]
    pub struct Log105Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,address)` and selector `0x1596a1ce`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,address)")]
    pub struct Log106Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,uint256)` and selector `0x159f8927`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,uint256)")]
    pub struct Log107Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,address)` and selector `0x15c127b5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,address)")]
    pub struct Log108Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,bool)` and selector `0x15cac476`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,bool)")]
    pub struct Log109Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,uint256)` and selector `0x1606a393`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,uint256)")]
    pub struct Log110Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,string)` and selector `0x1762e32a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,string)")]
    pub struct Log111Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256)` and selector `0x17fe6185`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256)")]
    pub struct Log30Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool)` and selector `0x18c9c746`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool)")]
    pub struct Log31Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,uint256)` and selector `0x193fb800`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,uint256)")]
    pub struct Log112Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,address)` and selector `0x19fd4956`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,address)")]
    pub struct Log113Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,string)` and selector `0x1ad96de6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,string)")]
    pub struct Log114Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,string)` and selector `0x1bb3b09a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,string)")]
    pub struct Log115Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,address)` and selector `0x1c41a336`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,address)")]
    pub struct Log116Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address)` and selector `0x1c7ec448`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address)")]
    pub struct Log32Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool)` and selector `0x1c9d7eb3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool)")]
    pub struct Log6Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,address)` and selector `0x1d14d001`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,address)")]
    pub struct Log117Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,string)` and selector `0x1da986ea`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,string)")]
    pub struct Log118Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,uint256)` and selector `0x1dc8e1b8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,uint256)")]
    pub struct Log119Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,bool)` and selector `0x1e4b87e5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,bool)")]
    pub struct Log120Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256)` and selector `0x20098014`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256)")]
    pub struct Log33Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,address)` and selector `0x205871c2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,address)")]
    pub struct Log121Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool)` and selector `0x20718650`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool)")]
    pub struct Log34Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,address)` and selector `0x20e3984d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,address)")]
    pub struct Log122Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string)` and selector `0x212255cc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string)")]
    pub struct Log35Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,string)` and selector `0x21ad0683`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,string)")]
    pub struct Log123Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,string)` and selector `0x21bdaf25`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,string)")]
    pub struct Log124Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,address)` and selector `0x223603bd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,address)")]
    pub struct Log125Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,uint256)` and selector `0x22f6b999`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,uint256)")]
    pub struct Log126Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,string)` and selector `0x245986f2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,string)")]
    pub struct Log127Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,address)` and selector `0x2488b414`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,address)")]
    pub struct Log128Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,uint256)` and selector `0x24f91465`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,uint256)")]
    pub struct Log129Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string)` and selector `0x2555fa46`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string)")]
    pub struct Log36Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,address)` and selector `0x26f560a8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,address)")]
    pub struct Log130Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,string)` and selector `0x27d8afd2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,string)")]
    pub struct Log131Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,uint256)` and selector `0x28863fcb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,uint256)")]
    pub struct Log132Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool)` and selector `0x2a110e83`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool)")]
    pub struct Log7Call {
        pub p_0: bool,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,string)` and selector `0x2ae408d4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,string)")]
    pub struct Log133Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,address)` and selector `0x2b2b18dc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,address)")]
    pub struct Log134Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,bool)` and selector `0x2c1754ed`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,bool)")]
    pub struct Log135Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,uint256)` and selector `0x2c1d0746`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,uint256)")]
    pub struct Log136Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address)` and selector `0x2c2ecbc2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address)")]
    pub struct Log1Call {
        pub p_0: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,bool)` and selector `0x2cd4134a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,bool)")]
    pub struct Log137Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string)` and selector `0x2ced7cef`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string)")]
    pub struct Log37Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,string)` and selector `0x2d8e33a4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,string)")]
    pub struct Log138Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,string)` and selector `0x2dd778e6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,string)")]
    pub struct Log139Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address)` and selector `0x319af333`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address)")]
    pub struct Log8Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool)` and selector `0x32458eed`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool)")]
    pub struct Log2Call {
        pub p_0: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,address)` and selector `0x33e9dd1d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,address)")]
    pub struct Log140Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,uint256)` and selector `0x34f0e636`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,uint256)")]
    pub struct Log141Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address)` and selector `0x35085f7b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address)")]
    pub struct Log38Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,bool)` and selector `0x354c36d6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,bool)")]
    pub struct Log142Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,bool)` and selector `0x35a5071f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,bool)")]
    pub struct Log143Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256)` and selector `0x37103367`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256)")]
    pub struct Log39Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,uint256)` and selector `0x374bb4b2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,uint256)")]
    pub struct Log144Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256)` and selector `0x37aa7d4c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256)")]
    pub struct Log40Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,uint256)` and selector `0x386ff5f4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,uint256)")]
    pub struct Log145Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,uint256)` and selector `0x3971e78c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,uint256)")]
    pub struct Log146Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256)` and selector `0x399174d3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256)")]
    pub struct Log9Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,address)` and selector `0x3b2279b4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,address)")]
    pub struct Log147Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,bool)` and selector `0x3b2a5ce0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,bool)")]
    pub struct Log148Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,bool)` and selector `0x3bf5e537`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,bool)")]
    pub struct Log149Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,string)` and selector `0x3e128ca3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,string)")]
    pub struct Log150Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,uint256)` and selector `0x3e9f866a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,uint256)")]
    pub struct Log151Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,bool)` and selector `0x3f8a701d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,bool)")]
    pub struct Log152Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,bool)` and selector `0x40785869`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,bool)")]
    pub struct Log153Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string)` and selector `0x41304fac`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string)")]
    pub struct Log3Call {
        pub p_0: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,address)` and selector `0x42d21db7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,address)")]
    pub struct Log154Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,address)` and selector `0x439c7bef`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,address)")]
    pub struct Log155Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,string)` and selector `0x448830a8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,string)")]
    pub struct Log156Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,bool)` and selector `0x454d54a5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,bool)")]
    pub struct Log157Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,uint256)` and selector `0x457fe3cf`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,uint256)")]
    pub struct Log158Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,bool)` and selector `0x46600be0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,bool)")]
    pub struct Log159Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,uint256)` and selector `0x46826b5d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,uint256)")]
    pub struct Log160Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,string)` and selector `0x475c5c33`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,string)")]
    pub struct Log161Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool)` and selector `0x4766da72`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool)")]
    pub struct Log41Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,address)` and selector `0x478d1c62`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,address)")]
    pub struct Log162Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,string)` and selector `0x483d0416`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,string)")]
    pub struct Log163Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,string)` and selector `0x4a28c017`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,string)")]
    pub struct Log164Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,string)` and selector `0x4a66cb34`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,string)")]
    pub struct Log165Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string)` and selector `0x4b5c4277`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string)")]
    pub struct Log10Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,uint256)` and selector `0x4c123d57`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,uint256)")]
    pub struct Log166Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool)` and selector `0x4ceda75a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool)")]
    pub struct Log42Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,uint256)` and selector `0x4f04fdc6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,uint256)")]
    pub struct Log167Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool)` and selector `0x50709698`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool)")]
    pub struct Log43Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,bool)` and selector `0x50ad461d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,bool)")]
    pub struct Log168Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,uint256)` and selector `0x515e38b6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,uint256)")]
    pub struct Log169Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log()` and selector `0x51973ec9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log()")]
    pub struct Log0Call;
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,string)` and selector `0x51f09ff8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,string)")]
    pub struct Log170Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,address)` and selector `0x538e06ab`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,address)")]
    pub struct Log171Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,address)` and selector `0x54a7a9a0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,address)")]
    pub struct Log172Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,address)` and selector `0x56a5d1b1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,address)")]
    pub struct Log173Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256)` and selector `0x5821efa1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256)")]
    pub struct Log44Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string)` and selector `0x5970e089`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string)")]
    pub struct Log45Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,string)` and selector `0x59cfcbe3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,string)")]
    pub struct Log174Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,string)` and selector `0x5a477632`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,string)")]
    pub struct Log175Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256)` and selector `0x5a9b5ed5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256)")]
    pub struct Log46Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,string)` and selector `0x5ab84e1f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,string)")]
    pub struct Log176Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,uint256)` and selector `0x5abd992a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,uint256)")]
    pub struct Log177Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,address)` and selector `0x5c430d47`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,address)")]
    pub struct Log178Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address)` and selector `0x5c96b331`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address)")]
    pub struct Log47Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,bool)` and selector `0x5ccd4e37`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,bool)")]
    pub struct Log179Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,string)` and selector `0x5d02c50b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,string)")]
    pub struct Log180Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,uint256)` and selector `0x5d08bb05`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,uint256)")]
    pub struct Log181Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,string)` and selector `0x5d1a971a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,string)")]
    pub struct Log182Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,uint256)` and selector `0x5da297eb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,uint256)")]
    pub struct Log183Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,string)` and selector `0x5e84b0ea`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,string)")]
    pub struct Log184Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,address)` and selector `0x5ea2b7ae`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,address)")]
    pub struct Log185Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,bool)` and selector `0x5f15d28c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,bool)")]
    pub struct Log186Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,bool)` and selector `0x5f1d5c9f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,bool)")]
    pub struct Log187Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,bool)` and selector `0x5f743a7c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,bool)")]
    pub struct Log188Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256)` and selector `0x5f7b9afb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256)")]
    pub struct Log48Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,address)` and selector `0x6168ed61`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,address)")]
    pub struct Log189Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,bool)` and selector `0x619e4d0e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,bool)")]
    pub struct Log190Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256,address)` and selector `0x63183678`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256,address)")]
    pub struct Log191Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string)` and selector `0x63cb41f9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string)")]
    pub struct Log49Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,address)` and selector `0x63fb8bc5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,address)")]
    pub struct Log192Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string)` and selector `0x643fd0df`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string)")]
    pub struct Log11Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,uint256)` and selector `0x64b5bb67`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,uint256)")]
    pub struct Log193Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,address)` and selector `0x660375dd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,address)")]
    pub struct Log194Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,address)` and selector `0x665bf134`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,address)")]
    pub struct Log195Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256,bool)` and selector `0x66f1bc67`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256,bool)")]
    pub struct Log196Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool)` and selector `0x678209a8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool)")]
    pub struct Log50Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,uint256)` and selector `0x67dd6ff1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,uint256)")]
    pub struct Log51Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,string)` and selector `0x68c8b8bd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,string)")]
    pub struct Log197Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,bool)` and selector `0x691a8f74`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,bool)")]
    pub struct Log198Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address)` and selector `0x69276c86`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address)")]
    pub struct Log12Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,address)` and selector `0x69640b59`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,address)")]
    pub struct Log199Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,uint256)` and selector `0x6a1199e2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,uint256)")]
    pub struct Log200Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,bool,bool)` and selector `0x6a9c478b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,bool,bool)")]
    pub struct Log201Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,uint256,bool)` and selector `0x6b0e5d53`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,uint256,bool)")]
    pub struct Log202Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,string)` and selector `0x6cde40b8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,string)")]
    pub struct Log203Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,string)` and selector `0x6d1e8751`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,string)")]
    pub struct Log204Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,address)` and selector `0x6d572f44`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,address)")]
    pub struct Log205Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,uint256)` and selector `0x6d7045c1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,uint256)")]
    pub struct Log206Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,bool)` and selector `0x6dd434ca`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,bool)")]
    pub struct Log207Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,bool)` and selector `0x6f1a594e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,bool)")]
    pub struct Log208Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,address)` and selector `0x6f7c603e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,address)")]
    pub struct Log209Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,address)` and selector `0x7190a529`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,address)")]
    pub struct Log210Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string)` and selector `0x71d04af2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string)")]
    pub struct Log52Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address,uint256)` and selector `0x736efbb6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address,uint256)")]
    pub struct Log211Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,string)` and selector `0x742d6ee7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,string)")]
    pub struct Log212Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,uint256)` and selector `0x7464ce23`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,uint256)")]
    pub struct Log213Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string)` and selector `0x759f86bb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string)")]
    pub struct Log13Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool)` and selector `0x75b605d3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool)")]
    pub struct Log14Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,bool)` and selector `0x7626db92`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,bool)")]
    pub struct Log214Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool,bool)` and selector `0x79884c2b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool,bool)")]
    pub struct Log215Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,string,bool)` and selector `0x7af6ab25`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,string,bool)")]
    pub struct Log216Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address)` and selector `0x7afac959`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address)")]
    pub struct Log53Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address)` and selector `0x7bc0d848`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address)")]
    pub struct Log54Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,uint256)` and selector `0x7be0c3eb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,uint256)")]
    pub struct Log217Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,uint256)` and selector `0x7bf181a1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,uint256)")]
    pub struct Log218Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,address)` and selector `0x7c4632a4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,address)")]
    pub struct Log219Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,uint256)` and selector `0x7cc3c607`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,uint256)")]
    pub struct Log220Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,bool)` and selector `0x7d24491d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,bool)")]
    pub struct Log221Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,uint256,string)` and selector `0x7dd4d0e0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,uint256,string)")]
    pub struct Log222Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,uint256)` and selector `0x7f9bbca2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,uint256)")]
    pub struct Log223Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,string)` and selector `0x800a1c67`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,string)")]
    pub struct Log224Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,string,uint256)` and selector `0x80e6a20b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,string,uint256)")]
    pub struct Log225Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,bool)` and selector `0x82112a42`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,bool)")]
    pub struct Log226Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,uint256)` and selector `0x82c25b74`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,uint256)")]
    pub struct Log227Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256)` and selector `0x8309e8a8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256)")]
    pub struct Log15Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool)` and selector `0x850b7ad6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool)")]
    pub struct Log55Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address)` and selector `0x853c4849`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address)")]
    pub struct Log16Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,string)` and selector `0x854b3496`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,string)")]
    pub struct Log228Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string)` and selector `0x85775021`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string)")]
    pub struct Log56Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,string)` and selector `0x88a8c406`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,string)")]
    pub struct Log229Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,address)` and selector `0x88cb6041`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,address)")]
    pub struct Log230Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,address,uint256)` and selector `0x88f6e4b2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,address,uint256)")]
    pub struct Log231Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,bool)` and selector `0x895af8c5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,bool)")]
    pub struct Log232Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,bool)` and selector `0x8af7cf8a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,bool)")]
    pub struct Log233Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,bool,address)` and selector `0x8c329b1a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,bool,address)")]
    pub struct Log234Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,uint256)` and selector `0x8c4e5de6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,uint256)")]
    pub struct Log235Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,address)` and selector `0x8da6def5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,address)")]
    pub struct Log236Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,uint256)` and selector `0x8e3f78a9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,uint256)")]
    pub struct Log237Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,string)` and selector `0x8e69fb5d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,string)")]
    pub struct Log238Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,uint256)` and selector `0x8eafb02b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,uint256)")]
    pub struct Log239Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,uint256)` and selector `0x8ef3f399`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,uint256)")]
    pub struct Log240Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,address)` and selector `0x8f736d16`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,address)")]
    pub struct Log241Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string)` and selector `0x8feac525`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string)")]
    pub struct Log17Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,bool)` and selector `0x90c30a56`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,bool)")]
    pub struct Log242Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,string)` and selector `0x90fb06aa`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,string)")]
    pub struct Log243Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,string)` and selector `0x9143dbb1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,string)")]
    pub struct Log244Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,bool)` and selector `0x91a02e2a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,bool)")]
    pub struct Log245Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,uint256)` and selector `0x91d1112e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,uint256)")]
    pub struct Log246Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address)` and selector `0x932bbb38`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address)")]
    pub struct Log57Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256,address)` and selector `0x935e09bf`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256,address)")]
    pub struct Log247Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,uint256)` and selector `0x94250d77`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,uint256)")]
    pub struct Log248Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,address,bool)` and selector `0x958c28c6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,address,bool)")]
    pub struct Log249Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address)` and selector `0x9591b953`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address)")]
    pub struct Log58Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address)` and selector `0x95ed0195`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address)")]
    pub struct Log59Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string,address)` and selector `0x97d394d8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string,address)")]
    pub struct Log250Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,address)` and selector `0x9a816a83`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,address)")]
    pub struct Log251Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,address)` and selector `0x9acd3616`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,address)")]
    pub struct Log252Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,bool)` and selector `0x9b4254e2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,bool)")]
    pub struct Log253Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool)` and selector `0x9b6ec042`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool)")]
    pub struct Log60Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,string)` and selector `0x9c3adfa1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,string)")]
    pub struct Log254Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256)` and selector `0x9c4f99fb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256)")]
    pub struct Log61Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,address)` and selector `0x9cba8fff`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,address)")]
    pub struct Log255Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,bool,string)` and selector `0x9d22d5dd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,bool,string)")]
    pub struct Log256Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,address)` and selector `0x9f1bc36e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,address)")]
    pub struct Log257Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,address,string)` and selector `0x9ffb2f93`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,address,string)")]
    pub struct Log258Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string,address)` and selector `0xa04e2f87`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string,address)")]
    pub struct Log259Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,string)` and selector `0xa0a47963`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,string)")]
    pub struct Log260Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,address,bool)` and selector `0xa1bcc9b3`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,address,bool)")]
    pub struct Log261Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,address)` and selector `0xa1ef4cbb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,address)")]
    pub struct Log262Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string)` and selector `0xa1f2e8aa`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string)")]
    pub struct Log62Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,address)` and selector `0xa31bfdcc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,address)")]
    pub struct Log263Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,string)` and selector `0xa5b4fc99`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,string)")]
    pub struct Log264Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,address,uint256)` and selector `0xa5cada94`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,address,uint256)")]
    pub struct Log265Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,bool)` and selector `0xa6f50b0f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,bool)")]
    pub struct Log266Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,string)` and selector `0xa73c1db6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,string)")]
    pub struct Log267Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address,uint256)` and selector `0xa75c59de`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address,uint256)")]
    pub struct Log268Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,uint256)` and selector `0xa7a87853`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,uint256)")]
    pub struct Log269Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,string)` and selector `0xa826caeb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,string)")]
    pub struct Log270Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool,string)` and selector `0xaa6540c8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool,string)")]
    pub struct Log271Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string,address)` and selector `0xaabc9a31`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string,address)")]
    pub struct Log272Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,bool)` and selector `0xab085ae6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,bool)")]
    pub struct Log273Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,string)` and selector `0xabf73a98`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,string)")]
    pub struct Log274Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,address,string)` and selector `0xade052c7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,address,string)")]
    pub struct Log275Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,address)` and selector `0xae2ec581`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,address)")]
    pub struct Log276Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,uint256)` and selector `0xb028c9bd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,uint256)")]
    pub struct Log277Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,string)` and selector `0xb076847f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,string)")]
    pub struct Log63Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool)` and selector `0xb0e0f9b5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool)")]
    pub struct Log64Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string)` and selector `0xb115611f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string)")]
    pub struct Log65Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,bool)` and selector `0xb3a6b6bd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,bool)")]
    pub struct Log278Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,address,bool)` and selector `0xb4c314ff`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,address,bool)")]
    pub struct Log279Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,bool)` and selector `0xb59dbd60`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,bool)")]
    pub struct Log280Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256)` and selector `0xb60e72cc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256)")]
    pub struct Log18Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,uint256)` and selector `0xb69bcaf6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,uint256)")]
    pub struct Log66Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,bool)` and selector `0xb6f577a1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,bool)")]
    pub struct Log281Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,uint256,string)` and selector `0xb7b914ca`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,uint256,string)")]
    pub struct Log282Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,bool)` and selector `0xb857163a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,bool)")]
    pub struct Log283Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,bool)` and selector `0xba535d9c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,bool)")]
    pub struct Log284Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool,string)` and selector `0xbc0b61fe`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool,string)")]
    pub struct Log285Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,address)` and selector `0xbcfd9be0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,address)")]
    pub struct Log67Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,uint256)` and selector `0xbe553481`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,uint256)")]
    pub struct Log286Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,uint256,bool)` and selector `0xbe984353`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,uint256,bool)")]
    pub struct Log287Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,uint256)` and selector `0xbf01f891`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,uint256)")]
    pub struct Log288Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,bool)` and selector `0xc0a302d8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,bool)")]
    pub struct Log289Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,uint256)` and selector `0xc21f64c7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,uint256)")]
    pub struct Log290Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,address)` and selector `0xc371c7db`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,address)")]
    pub struct Log291Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,bool)` and selector `0xc3a8a654`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,bool)")]
    pub struct Log292Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool)` and selector `0xc3b55635`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool)")]
    pub struct Log19Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string)` and selector `0xc3fc3970`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string)")]
    pub struct Log68Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,bool)` and selector `0xc4643e20`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,bool)")]
    pub struct Log293Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,bool)` and selector `0xc598d185`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,bool)")]
    pub struct Log294Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,bool,string)` and selector `0xc5ad85f9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,bool,string)")]
    pub struct Log295Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,string,uint256)` and selector `0xc67ea9d1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,string,uint256)")]
    pub struct Log296Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,uint256)` and selector `0xc6acc7a8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,uint256)")]
    pub struct Log297Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,bool)` and selector `0xc91d5ed4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,bool)")]
    pub struct Log69Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,uint256)` and selector `0xc95958d6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,uint256)")]
    pub struct Log70Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256)` and selector `0xca47c4eb`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256)")]
    pub struct Log71Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool)` and selector `0xca7733b1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool)")]
    pub struct Log72Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,bool)` and selector `0xcac43479`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,bool)")]
    pub struct Log298Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,string,bool)` and selector `0xcc32ab07`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,string,bool)")]
    pub struct Log299Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,uint256,address)` and selector `0xccf790a1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,uint256,address)")]
    pub struct Log300Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool,bool)` and selector `0xceb5f4d7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool,bool)")]
    pub struct Log301Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,uint256)` and selector `0xcf009880`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,uint256)")]
    pub struct Log302Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,bool)` and selector `0xcf020fb1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,bool)")]
    pub struct Log73Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,uint256,string,bool)` and selector `0xcf18105c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,uint256,string,bool)")]
    pub struct Log303Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,address)` and selector `0xcf394485`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,address)")]
    pub struct Log304Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256)` and selector `0xd1ed7a3c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256)")]
    pub struct Log74Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address)` and selector `0xd2763667`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address)")]
    pub struct Log75Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,bool,string)` and selector `0xd2d423cd`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,bool,string)")]
    pub struct Log305Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,string,address)` and selector `0xd583c602`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,string,address)")]
    pub struct Log306Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,uint256,bool)` and selector `0xd6019f1c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,uint256,bool)")]
    pub struct Log307Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,bool,uint256)` and selector `0xd6aefad2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,bool,uint256)")]
    pub struct Log308Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,address,string)` and selector `0xd812a167`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,address,string)")]
    pub struct Log309Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address)` and selector `0xdaf0d4aa`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address)")]
    pub struct Log20Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool)` and selector `0xdbb4c247`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool)")]
    pub struct Log76Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,string,bool,bool)` and selector `0xdc5e935b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,string,bool,bool)")]
    pub struct Log310Call {
        pub p_0: bool,
        pub p_1: ::std::string::String,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,uint256,string)` and selector `0xddb06521`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,uint256,string)")]
    pub struct Log311Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,bool,string)` and selector `0xdddb9561`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,bool,string)")]
    pub struct Log312Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,uint256,string)` and selector `0xde03e774`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,uint256,string)")]
    pub struct Log313Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,string,string)` and selector `0xde68f20a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,string,string)")]
    pub struct Log314Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string)` and selector `0xde9a9270`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string)")]
    pub struct Log77Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool,string)` and selector `0xdfc4a2e8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool,string)")]
    pub struct Log315Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string,address)` and selector `0xe0625b29`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string,address)")]
    pub struct Log316Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,address)` and selector `0xe0e95b98`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,address)")]
    pub struct Log317Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,string)` and selector `0xe0e9ad4f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,string)")]
    pub struct Log78Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,uint256,address)` and selector `0xe21de278`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,uint256,address)")]
    pub struct Log318Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,bool,string)` and selector `0xe298f47d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,bool,string)")]
    pub struct Log79Call {
        pub p_0: ::std::string::String,
        pub p_1: bool,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,address,string,bool)` and selector `0xe2bfd60b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,address,string,bool)")]
    pub struct Log319Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,bool)` and selector `0xe351140f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,bool)")]
    pub struct Log320Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,uint256)` and selector `0xe3a9ca2f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,uint256)")]
    pub struct Log321Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,uint256,bool,uint256)` and selector `0xe41b6f6f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,uint256,bool,uint256)")]
    pub struct Log322Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,bool)` and selector `0xe5e70b2b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,bool)")]
    pub struct Log323Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,string,address,uint256)` and selector `0xe8d3018d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,string,address,uint256)")]
    pub struct Log324Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,bool)` and selector `0xe8defba9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,bool)")]
    pub struct Log80Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,address,string)` and selector `0xeb1bff80`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,address,string)")]
    pub struct Log325Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,bool,uint256)` and selector `0xeb7f6fd2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,bool,uint256)")]
    pub struct Log326Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,bool)` and selector `0xeb830c92`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,bool)")]
    pub struct Log81Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,bool)` and selector `0xeb928d7f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,bool)")]
    pub struct Log327Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address,address)` and selector `0xed8f28f6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address,address)")]
    pub struct Log328Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,string,uint256)` and selector `0xef1cefe7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,string,uint256)")]
    pub struct Log329Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,bool,string,address)` and selector `0xef529018`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,bool,string,address)")]
    pub struct Log330Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,address,bool,address)` and selector `0xef72c513`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,address,bool,address)")]
    pub struct Log331Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address)` and selector `0xf08744e8`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address)")]
    pub struct Log82Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,bool,address)` and selector `0xf11699ed`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,bool,address)")]
    pub struct Log83Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,bool)` and selector `0xf2a66286`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,bool)")]
    pub struct Log84Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,string,uint256,uint256)` and selector `0xf45d7d2c`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,string,uint256,uint256)")]
    pub struct Log332Call {
        pub p_0: ::std::string::String,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,address,address)` and selector `0xf4880ea4`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,address,address)")]
    pub struct Log333Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,string)` and selector `0xf5bc2249`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,string)")]
    pub struct Log334Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256)` and selector `0xf666715a`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256)")]
    pub struct Log21Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,address,string)` and selector `0xf7e36245`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,address,string)")]
    pub struct Log335Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,address,string)` and selector `0xf808da20`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,address,string)")]
    pub struct Log336Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256)` and selector `0xf82c50f1`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256)")]
    pub struct Log4Call {
        pub p_0: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,uint256)` and selector `0xf8f51b1e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,uint256)")]
    pub struct Log337Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::U256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,bool,string,address)` and selector `0xf9ad2b89`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,bool,string,address)")]
    pub struct Log338Call {
        pub p_0: bool,
        pub p_1: bool,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(uint256,uint256,uint256,address)` and selector `0xfa8185af`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(uint256,uint256,uint256,address)")]
    pub struct Log339Call {
        pub p_0: ::ethers_core::types::U256,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,string,string)` and selector `0xfb772265`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,string,string)")]
    pub struct Log85Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::std::string::String,
        pub p_2: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,uint256,bool)` and selector `0xfc4845f0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,uint256,bool)")]
    pub struct Log340Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: bool,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,address,address)` and selector `0xfcec75e0`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,address,address)")]
    pub struct Log86Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(address,address,uint256,string)` and selector `0xfdb4f990`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(address,address,uint256,string)")]
    pub struct Log341Call {
        pub p_0: ::ethers_core::types::Address,
        pub p_1: ::ethers_core::types::Address,
        pub p_2: ::ethers_core::types::U256,
        pub p_3: ::std::string::String,
    }
    ///Container type for all input parameters for the `log` function with signature `log(bool,uint256,string,address)` and selector `0xfedd1fff`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(bool,uint256,string,address)")]
    pub struct Log342Call {
        pub p_0: bool,
        pub p_1: ::ethers_core::types::U256,
        pub p_2: ::std::string::String,
        pub p_3: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `log` function with signature `log(int256)` and selector `0x2d5b6cb9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(int256)")]
    pub struct Log5Call {
        pub p_0: ::ethers_core::types::I256,
    }
    ///Container type for all input parameters for the `log` function with signature `log(string,int256)` and selector `0x3ca6268e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "log", abi = "log(string,int256)")]
    pub struct Log22Call {
        pub p_0: ::std::string::String,
        pub p_1: ::ethers_core::types::I256,
    }
    ///Container type for all input parameters for the `logAddress` function with signature `logAddress(address)` and selector `0x5f91b0af`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logAddress", abi = "logAddress(address)")]
    pub struct LogAddressCall {
        pub p_0: ::ethers_core::types::Address,
    }
    ///Container type for all input parameters for the `logBool` function with signature `logBool(bool)` and selector `0xba7ab84e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBool", abi = "logBool(bool)")]
    pub struct LogBoolCall {
        pub p_0: bool,
    }
    ///Container type for all input parameters for the `logBytes` function with signature `logBytes(bytes)` and selector `0xe17bf956`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes", abi = "logBytes(bytes)")]
    pub struct LogBytesCall {
        pub p_0: ::ethers_core::types::Bytes,
    }
    ///Container type for all input parameters for the `logBytes1` function with signature `logBytes1(bytes1)` and selector `0x6f4171c9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes1", abi = "logBytes1(bytes1)")]
    pub struct LogBytes1Call {
        pub p_0: [u8; 1],
    }
    ///Container type for all input parameters for the `logBytes10` function with signature `logBytes10(bytes10)` and selector `0x9dc2a897`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes10", abi = "logBytes10(bytes10)")]
    pub struct LogBytes10Call {
        pub p_0: [u8; 10],
    }
    ///Container type for all input parameters for the `logBytes11` function with signature `logBytes11(bytes11)` and selector `0xdc08b6a7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes11", abi = "logBytes11(bytes11)")]
    pub struct LogBytes11Call {
        pub p_0: [u8; 11],
    }
    ///Container type for all input parameters for the `logBytes12` function with signature `logBytes12(bytes12)` and selector `0x7656d6c7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes12", abi = "logBytes12(bytes12)")]
    pub struct LogBytes12Call {
        pub p_0: [u8; 12],
    }
    ///Container type for all input parameters for the `logBytes13` function with signature `logBytes13(bytes13)` and selector `0x34c1d81b`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes13", abi = "logBytes13(bytes13)")]
    pub struct LogBytes13Call {
        pub p_0: [u8; 13],
    }
    ///Container type for all input parameters for the `logBytes14` function with signature `logBytes14(bytes14)` and selector `0x3ceaba65`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes14", abi = "logBytes14(bytes14)")]
    pub struct LogBytes14Call {
        pub p_0: [u8; 14],
    }
    ///Container type for all input parameters for the `logBytes15` function with signature `logBytes15(bytes15)` and selector `0x591a3da2`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes15", abi = "logBytes15(bytes15)")]
    pub struct LogBytes15Call {
        pub p_0: [u8; 15],
    }
    ///Container type for all input parameters for the `logBytes16` function with signature `logBytes16(bytes16)` and selector `0x1f8d7312`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes16", abi = "logBytes16(bytes16)")]
    pub struct LogBytes16Call {
        pub p_0: [u8; 16],
    }
    ///Container type for all input parameters for the `logBytes17` function with signature `logBytes17(bytes17)` and selector `0xf89a532f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes17", abi = "logBytes17(bytes17)")]
    pub struct LogBytes17Call {
        pub p_0: [u8; 17],
    }
    ///Container type for all input parameters for the `logBytes18` function with signature `logBytes18(bytes18)` and selector `0xd8652642`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes18", abi = "logBytes18(bytes18)")]
    pub struct LogBytes18Call {
        pub p_0: [u8; 18],
    }
    ///Container type for all input parameters for the `logBytes19` function with signature `logBytes19(bytes19)` and selector `0x00f56bc9`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes19", abi = "logBytes19(bytes19)")]
    pub struct LogBytes19Call {
        pub p_0: [u8; 19],
    }
    ///Container type for all input parameters for the `logBytes2` function with signature `logBytes2(bytes2)` and selector `0x9b5e943e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes2", abi = "logBytes2(bytes2)")]
    pub struct LogBytes2Call {
        pub p_0: [u8; 2],
    }
    ///Container type for all input parameters for the `logBytes20` function with signature `logBytes20(bytes20)` and selector `0xecb8567e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes20", abi = "logBytes20(bytes20)")]
    pub struct LogBytes20Call {
        pub p_0: [u8; 20],
    }
    ///Container type for all input parameters for the `logBytes21` function with signature `logBytes21(bytes21)` and selector `0x3052c08f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes21", abi = "logBytes21(bytes21)")]
    pub struct LogBytes21Call {
        pub p_0: [u8; 21],
    }
    ///Container type for all input parameters for the `logBytes22` function with signature `logBytes22(bytes22)` and selector `0x807ab434`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes22", abi = "logBytes22(bytes22)")]
    pub struct LogBytes22Call {
        pub p_0: [u8; 22],
    }
    ///Container type for all input parameters for the `logBytes23` function with signature `logBytes23(bytes23)` and selector `0x4979b037`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes23", abi = "logBytes23(bytes23)")]
    pub struct LogBytes23Call {
        pub p_0: [u8; 23],
    }
    ///Container type for all input parameters for the `logBytes24` function with signature `logBytes24(bytes24)` and selector `0x0977aefc`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes24", abi = "logBytes24(bytes24)")]
    pub struct LogBytes24Call {
        pub p_0: [u8; 24],
    }
    ///Container type for all input parameters for the `logBytes25` function with signature `logBytes25(bytes25)` and selector `0xaea9963f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes25", abi = "logBytes25(bytes25)")]
    pub struct LogBytes25Call {
        pub p_0: [u8; 25],
    }
    ///Container type for all input parameters for the `logBytes26` function with signature `logBytes26(bytes26)` and selector `0xd3635628`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes26", abi = "logBytes26(bytes26)")]
    pub struct LogBytes26Call {
        pub p_0: [u8; 26],
    }
    ///Container type for all input parameters for the `logBytes27` function with signature `logBytes27(bytes27)` and selector `0xfc372f9f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes27", abi = "logBytes27(bytes27)")]
    pub struct LogBytes27Call {
        pub p_0: [u8; 27],
    }
    ///Container type for all input parameters for the `logBytes28` function with signature `logBytes28(bytes28)` and selector `0x382f9a34`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes28", abi = "logBytes28(bytes28)")]
    pub struct LogBytes28Call {
        pub p_0: [u8; 28],
    }
    ///Container type for all input parameters for the `logBytes29` function with signature `logBytes29(bytes29)` and selector `0x7a187641`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes29", abi = "logBytes29(bytes29)")]
    pub struct LogBytes29Call {
        pub p_0: [u8; 29],
    }
    ///Container type for all input parameters for the `logBytes3` function with signature `logBytes3(bytes3)` and selector `0x7782fa2d`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes3", abi = "logBytes3(bytes3)")]
    pub struct LogBytes3Call {
        pub p_0: [u8; 3],
    }
    ///Container type for all input parameters for the `logBytes30` function with signature `logBytes30(bytes30)` and selector `0xc4340ef6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes30", abi = "logBytes30(bytes30)")]
    pub struct LogBytes30Call {
        pub p_0: [u8; 30],
    }
    ///Container type for all input parameters for the `logBytes31` function with signature `logBytes31(bytes31)` and selector `0x81fc8648`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes31", abi = "logBytes31(bytes31)")]
    pub struct LogBytes31Call {
        pub p_0: [u8; 31],
    }
    ///Container type for all input parameters for the `logBytes32` function with signature `logBytes32(bytes32)` and selector `0x2d21d6f7`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes32", abi = "logBytes32(bytes32)")]
    pub struct LogBytes32Call {
        pub p_0: [u8; 32],
    }
    ///Container type for all input parameters for the `logBytes4` function with signature `logBytes4(bytes4)` and selector `0xfba3ad39`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes4", abi = "logBytes4(bytes4)")]
    pub struct LogBytes4Call {
        pub p_0: [u8; 4],
    }
    ///Container type for all input parameters for the `logBytes5` function with signature `logBytes5(bytes5)` and selector `0x5583be2e`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes5", abi = "logBytes5(bytes5)")]
    pub struct LogBytes5Call {
        pub p_0: [u8; 5],
    }
    ///Container type for all input parameters for the `logBytes6` function with signature `logBytes6(bytes6)` and selector `0x4942adc6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes6", abi = "logBytes6(bytes6)")]
    pub struct LogBytes6Call {
        pub p_0: [u8; 6],
    }
    ///Container type for all input parameters for the `logBytes7` function with signature `logBytes7(bytes7)` and selector `0x4574afab`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes7", abi = "logBytes7(bytes7)")]
    pub struct LogBytes7Call {
        pub p_0: [u8; 7],
    }
    ///Container type for all input parameters for the `logBytes8` function with signature `logBytes8(bytes8)` and selector `0x9902e47f`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes8", abi = "logBytes8(bytes8)")]
    pub struct LogBytes8Call {
        pub p_0: [u8; 8],
    }
    ///Container type for all input parameters for the `logBytes9` function with signature `logBytes9(bytes9)` and selector `0x50a138df`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logBytes9", abi = "logBytes9(bytes9)")]
    pub struct LogBytes9Call {
        pub p_0: [u8; 9],
    }
    ///Container type for all input parameters for the `logInt` function with signature `logInt(int256)` and selector `0x6525b5f5`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logInt", abi = "logInt(int256)")]
    pub struct LogIntCall {
        pub p_0: ::ethers_core::types::I256,
    }
    ///Container type for all input parameters for the `logString` function with signature `logString(string)` and selector `0x0bb563d6`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logString", abi = "logString(string)")]
    pub struct LogStringCall {
        pub p_0: ::std::string::String,
    }
    ///Container type for all input parameters for the `logUint` function with signature `logUint(uint256)` and selector `0x9905b744`
    #[derive(
        Clone,
        ::ethers_contract::EthCall,
        ::ethers_contract::EthDisplay,
        foundry_macros::ConsoleFmt,
        Default,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    #[ethcall(name = "logUint", abi = "logUint(uint256)")]
    pub struct LogUintCall {
        pub p_0: ::ethers_core::types::U256,
    }
    ///Container type for all of the contract's call
    #[derive(
        Clone,
        ::ethers_contract::EthAbiType,
        foundry_macros::ConsoleFmt,
        Debug,
        PartialEq,
        Eq,
        Hash
    )]
    pub enum HardhatConsoleCalls {
        Log23(Log23Call),
        Log87(Log87Call),
        Log24(Log24Call),
        Log88(Log88Call),
        Log89(Log89Call),
        Log90(Log90Call),
        Log91(Log91Call),
        Log25(Log25Call),
        Log92(Log92Call),
        Log93(Log93Call),
        Log94(Log94Call),
        Log95(Log95Call),
        Log96(Log96Call),
        Log26(Log26Call),
        Log97(Log97Call),
        Log98(Log98Call),
        Log99(Log99Call),
        Log100(Log100Call),
        Log101(Log101Call),
        Log102(Log102Call),
        Log27(Log27Call),
        Log28(Log28Call),
        Log103(Log103Call),
        Log29(Log29Call),
        Log104(Log104Call),
        Log105(Log105Call),
        Log106(Log106Call),
        Log107(Log107Call),
        Log108(Log108Call),
        Log109(Log109Call),
        Log110(Log110Call),
        Log111(Log111Call),
        Log30(Log30Call),
        Log31(Log31Call),
        Log112(Log112Call),
        Log113(Log113Call),
        Log114(Log114Call),
        Log115(Log115Call),
        Log116(Log116Call),
        Log32(Log32Call),
        Log6(Log6Call),
        Log117(Log117Call),
        Log118(Log118Call),
        Log119(Log119Call),
        Log120(Log120Call),
        Log33(Log33Call),
        Log121(Log121Call),
        Log34(Log34Call),
        Log122(Log122Call),
        Log35(Log35Call),
        Log123(Log123Call),
        Log124(Log124Call),
        Log125(Log125Call),
        Log126(Log126Call),
        Log127(Log127Call),
        Log128(Log128Call),
        Log129(Log129Call),
        Log36(Log36Call),
        Log130(Log130Call),
        Log131(Log131Call),
        Log132(Log132Call),
        Log7(Log7Call),
        Log133(Log133Call),
        Log134(Log134Call),
        Log135(Log135Call),
        Log136(Log136Call),
        Log1(Log1Call),
        Log137(Log137Call),
        Log37(Log37Call),
        Log138(Log138Call),
        Log139(Log139Call),
        Log8(Log8Call),
        Log2(Log2Call),
        Log140(Log140Call),
        Log141(Log141Call),
        Log38(Log38Call),
        Log142(Log142Call),
        Log143(Log143Call),
        Log39(Log39Call),
        Log144(Log144Call),
        Log40(Log40Call),
        Log145(Log145Call),
        Log146(Log146Call),
        Log9(Log9Call),
        Log147(Log147Call),
        Log148(Log148Call),
        Log149(Log149Call),
        Log150(Log150Call),
        Log151(Log151Call),
        Log152(Log152Call),
        Log153(Log153Call),
        Log3(Log3Call),
        Log154(Log154Call),
        Log155(Log155Call),
        Log156(Log156Call),
        Log157(Log157Call),
        Log158(Log158Call),
        Log159(Log159Call),
        Log160(Log160Call),
        Log161(Log161Call),
        Log41(Log41Call),
        Log162(Log162Call),
        Log163(Log163Call),
        Log164(Log164Call),
        Log165(Log165Call),
        Log10(Log10Call),
        Log166(Log166Call),
        Log42(Log42Call),
        Log167(Log167Call),
        Log43(Log43Call),
        Log168(Log168Call),
        Log169(Log169Call),
        Log0(Log0Call),
        Log170(Log170Call),
        Log171(Log171Call),
        Log172(Log172Call),
        Log173(Log173Call),
        Log44(Log44Call),
        Log45(Log45Call),
        Log174(Log174Call),
        Log175(Log175Call),
        Log46(Log46Call),
        Log176(Log176Call),
        Log177(Log177Call),
        Log178(Log178Call),
        Log47(Log47Call),
        Log179(Log179Call),
        Log180(Log180Call),
        Log181(Log181Call),
        Log182(Log182Call),
        Log183(Log183Call),
        Log184(Log184Call),
        Log185(Log185Call),
        Log186(Log186Call),
        Log187(Log187Call),
        Log188(Log188Call),
        Log48(Log48Call),
        Log189(Log189Call),
        Log190(Log190Call),
        Log191(Log191Call),
        Log49(Log49Call),
        Log192(Log192Call),
        Log11(Log11Call),
        Log193(Log193Call),
        Log194(Log194Call),
        Log195(Log195Call),
        Log196(Log196Call),
        Log50(Log50Call),
        Log51(Log51Call),
        Log197(Log197Call),
        Log198(Log198Call),
        Log12(Log12Call),
        Log199(Log199Call),
        Log200(Log200Call),
        Log201(Log201Call),
        Log202(Log202Call),
        Log203(Log203Call),
        Log204(Log204Call),
        Log205(Log205Call),
        Log206(Log206Call),
        Log207(Log207Call),
        Log208(Log208Call),
        Log209(Log209Call),
        Log210(Log210Call),
        Log52(Log52Call),
        Log211(Log211Call),
        Log212(Log212Call),
        Log213(Log213Call),
        Log13(Log13Call),
        Log14(Log14Call),
        Log214(Log214Call),
        Log215(Log215Call),
        Log216(Log216Call),
        Log53(Log53Call),
        Log54(Log54Call),
        Log217(Log217Call),
        Log218(Log218Call),
        Log219(Log219Call),
        Log220(Log220Call),
        Log221(Log221Call),
        Log222(Log222Call),
        Log223(Log223Call),
        Log224(Log224Call),
        Log225(Log225Call),
        Log226(Log226Call),
        Log227(Log227Call),
        Log15(Log15Call),
        Log55(Log55Call),
        Log16(Log16Call),
        Log228(Log228Call),
        Log56(Log56Call),
        Log229(Log229Call),
        Log230(Log230Call),
        Log231(Log231Call),
        Log232(Log232Call),
        Log233(Log233Call),
        Log234(Log234Call),
        Log235(Log235Call),
        Log236(Log236Call),
        Log237(Log237Call),
        Log238(Log238Call),
        Log239(Log239Call),
        Log240(Log240Call),
        Log241(Log241Call),
        Log17(Log17Call),
        Log242(Log242Call),
        Log243(Log243Call),
        Log244(Log244Call),
        Log245(Log245Call),
        Log246(Log246Call),
        Log57(Log57Call),
        Log247(Log247Call),
        Log248(Log248Call),
        Log249(Log249Call),
        Log58(Log58Call),
        Log59(Log59Call),
        Log250(Log250Call),
        Log251(Log251Call),
        Log252(Log252Call),
        Log253(Log253Call),
        Log60(Log60Call),
        Log254(Log254Call),
        Log61(Log61Call),
        Log255(Log255Call),
        Log256(Log256Call),
        Log257(Log257Call),
        Log258(Log258Call),
        Log259(Log259Call),
        Log260(Log260Call),
        Log261(Log261Call),
        Log262(Log262Call),
        Log62(Log62Call),
        Log263(Log263Call),
        Log264(Log264Call),
        Log265(Log265Call),
        Log266(Log266Call),
        Log267(Log267Call),
        Log268(Log268Call),
        Log269(Log269Call),
        Log270(Log270Call),
        Log271(Log271Call),
        Log272(Log272Call),
        Log273(Log273Call),
        Log274(Log274Call),
        Log275(Log275Call),
        Log276(Log276Call),
        Log277(Log277Call),
        Log63(Log63Call),
        Log64(Log64Call),
        Log65(Log65Call),
        Log278(Log278Call),
        Log279(Log279Call),
        Log280(Log280Call),
        Log18(Log18Call),
        Log66(Log66Call),
        Log281(Log281Call),
        Log282(Log282Call),
        Log283(Log283Call),
        Log284(Log284Call),
        Log285(Log285Call),
        Log67(Log67Call),
        Log286(Log286Call),
        Log287(Log287Call),
        Log288(Log288Call),
        Log289(Log289Call),
        Log290(Log290Call),
        Log291(Log291Call),
        Log292(Log292Call),
        Log19(Log19Call),
        Log68(Log68Call),
        Log293(Log293Call),
        Log294(Log294Call),
        Log295(Log295Call),
        Log296(Log296Call),
        Log297(Log297Call),
        Log69(Log69Call),
        Log70(Log70Call),
        Log71(Log71Call),
        Log72(Log72Call),
        Log298(Log298Call),
        Log299(Log299Call),
        Log300(Log300Call),
        Log301(Log301Call),
        Log302(Log302Call),
        Log73(Log73Call),
        Log303(Log303Call),
        Log304(Log304Call),
        Log74(Log74Call),
        Log75(Log75Call),
        Log305(Log305Call),
        Log306(Log306Call),
        Log307(Log307Call),
        Log308(Log308Call),
        Log309(Log309Call),
        Log20(Log20Call),
        Log76(Log76Call),
        Log310(Log310Call),
        Log311(Log311Call),
        Log312(Log312Call),
        Log313(Log313Call),
        Log314(Log314Call),
        Log77(Log77Call),
        Log315(Log315Call),
        Log316(Log316Call),
        Log317(Log317Call),
        Log78(Log78Call),
        Log318(Log318Call),
        Log79(Log79Call),
        Log319(Log319Call),
        Log320(Log320Call),
        Log321(Log321Call),
        Log322(Log322Call),
        Log323(Log323Call),
        Log324(Log324Call),
        Log80(Log80Call),
        Log325(Log325Call),
        Log326(Log326Call),
        Log81(Log81Call),
        Log327(Log327Call),
        Log328(Log328Call),
        Log329(Log329Call),
        Log330(Log330Call),
        Log331(Log331Call),
        Log82(Log82Call),
        Log83(Log83Call),
        Log84(Log84Call),
        Log332(Log332Call),
        Log333(Log333Call),
        Log334(Log334Call),
        Log21(Log21Call),
        Log335(Log335Call),
        Log336(Log336Call),
        Log4(Log4Call),
        Log337(Log337Call),
        Log338(Log338Call),
        Log339(Log339Call),
        Log85(Log85Call),
        Log340(Log340Call),
        Log86(Log86Call),
        Log341(Log341Call),
        Log342(Log342Call),
        Log5(Log5Call),
        Log22(Log22Call),
        LogAddress(LogAddressCall),
        LogBool(LogBoolCall),
        LogBytes(LogBytesCall),
        LogBytes1(LogBytes1Call),
        LogBytes10(LogBytes10Call),
        LogBytes11(LogBytes11Call),
        LogBytes12(LogBytes12Call),
        LogBytes13(LogBytes13Call),
        LogBytes14(LogBytes14Call),
        LogBytes15(LogBytes15Call),
        LogBytes16(LogBytes16Call),
        LogBytes17(LogBytes17Call),
        LogBytes18(LogBytes18Call),
        LogBytes19(LogBytes19Call),
        LogBytes2(LogBytes2Call),
        LogBytes20(LogBytes20Call),
        LogBytes21(LogBytes21Call),
        LogBytes22(LogBytes22Call),
        LogBytes23(LogBytes23Call),
        LogBytes24(LogBytes24Call),
        LogBytes25(LogBytes25Call),
        LogBytes26(LogBytes26Call),
        LogBytes27(LogBytes27Call),
        LogBytes28(LogBytes28Call),
        LogBytes29(LogBytes29Call),
        LogBytes3(LogBytes3Call),
        LogBytes30(LogBytes30Call),
        LogBytes31(LogBytes31Call),
        LogBytes32(LogBytes32Call),
        LogBytes4(LogBytes4Call),
        LogBytes5(LogBytes5Call),
        LogBytes6(LogBytes6Call),
        LogBytes7(LogBytes7Call),
        LogBytes8(LogBytes8Call),
        LogBytes9(LogBytes9Call),
        LogInt(LogIntCall),
        LogString(LogStringCall),
        LogUint(LogUintCall),
    }
    impl ::ethers_core::abi::AbiDecode for HardhatConsoleCalls {
        fn decode(
            data: impl AsRef<[u8]>,
        ) -> ::core::result::Result<Self, ::ethers_core::abi::AbiError> {
            let data = data.as_ref();
            if let Ok(decoded)
                = <Log23Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log23(decoded));
            }
            if let Ok(decoded)
                = <Log87Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log87(decoded));
            }
            if let Ok(decoded)
                = <Log24Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log24(decoded));
            }
            if let Ok(decoded)
                = <Log88Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log88(decoded));
            }
            if let Ok(decoded)
                = <Log89Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log89(decoded));
            }
            if let Ok(decoded)
                = <Log90Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log90(decoded));
            }
            if let Ok(decoded)
                = <Log91Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log91(decoded));
            }
            if let Ok(decoded)
                = <Log25Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log25(decoded));
            }
            if let Ok(decoded)
                = <Log92Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log92(decoded));
            }
            if let Ok(decoded)
                = <Log93Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log93(decoded));
            }
            if let Ok(decoded)
                = <Log94Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log94(decoded));
            }
            if let Ok(decoded)
                = <Log95Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log95(decoded));
            }
            if let Ok(decoded)
                = <Log96Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log96(decoded));
            }
            if let Ok(decoded)
                = <Log26Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log26(decoded));
            }
            if let Ok(decoded)
                = <Log97Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log97(decoded));
            }
            if let Ok(decoded)
                = <Log98Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log98(decoded));
            }
            if let Ok(decoded)
                = <Log99Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log99(decoded));
            }
            if let Ok(decoded)
                = <Log100Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log100(decoded));
            }
            if let Ok(decoded)
                = <Log101Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log101(decoded));
            }
            if let Ok(decoded)
                = <Log102Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log102(decoded));
            }
            if let Ok(decoded)
                = <Log27Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log27(decoded));
            }
            if let Ok(decoded)
                = <Log28Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log28(decoded));
            }
            if let Ok(decoded)
                = <Log103Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log103(decoded));
            }
            if let Ok(decoded)
                = <Log29Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log29(decoded));
            }
            if let Ok(decoded)
                = <Log104Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log104(decoded));
            }
            if let Ok(decoded)
                = <Log105Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log105(decoded));
            }
            if let Ok(decoded)
                = <Log106Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log106(decoded));
            }
            if let Ok(decoded)
                = <Log107Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log107(decoded));
            }
            if let Ok(decoded)
                = <Log108Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log108(decoded));
            }
            if let Ok(decoded)
                = <Log109Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log109(decoded));
            }
            if let Ok(decoded)
                = <Log110Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log110(decoded));
            }
            if let Ok(decoded)
                = <Log111Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log111(decoded));
            }
            if let Ok(decoded)
                = <Log30Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log30(decoded));
            }
            if let Ok(decoded)
                = <Log31Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log31(decoded));
            }
            if let Ok(decoded)
                = <Log112Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log112(decoded));
            }
            if let Ok(decoded)
                = <Log113Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log113(decoded));
            }
            if let Ok(decoded)
                = <Log114Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log114(decoded));
            }
            if let Ok(decoded)
                = <Log115Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log115(decoded));
            }
            if let Ok(decoded)
                = <Log116Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log116(decoded));
            }
            if let Ok(decoded)
                = <Log32Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log32(decoded));
            }
            if let Ok(decoded)
                = <Log6Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log6(decoded));
            }
            if let Ok(decoded)
                = <Log117Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log117(decoded));
            }
            if let Ok(decoded)
                = <Log118Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log118(decoded));
            }
            if let Ok(decoded)
                = <Log119Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log119(decoded));
            }
            if let Ok(decoded)
                = <Log120Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log120(decoded));
            }
            if let Ok(decoded)
                = <Log33Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log33(decoded));
            }
            if let Ok(decoded)
                = <Log121Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log121(decoded));
            }
            if let Ok(decoded)
                = <Log34Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log34(decoded));
            }
            if let Ok(decoded)
                = <Log122Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log122(decoded));
            }
            if let Ok(decoded)
                = <Log35Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log35(decoded));
            }
            if let Ok(decoded)
                = <Log123Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log123(decoded));
            }
            if let Ok(decoded)
                = <Log124Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log124(decoded));
            }
            if let Ok(decoded)
                = <Log125Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log125(decoded));
            }
            if let Ok(decoded)
                = <Log126Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log126(decoded));
            }
            if let Ok(decoded)
                = <Log127Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log127(decoded));
            }
            if let Ok(decoded)
                = <Log128Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log128(decoded));
            }
            if let Ok(decoded)
                = <Log129Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log129(decoded));
            }
            if let Ok(decoded)
                = <Log36Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log36(decoded));
            }
            if let Ok(decoded)
                = <Log130Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log130(decoded));
            }
            if let Ok(decoded)
                = <Log131Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log131(decoded));
            }
            if let Ok(decoded)
                = <Log132Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log132(decoded));
            }
            if let Ok(decoded)
                = <Log7Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log7(decoded));
            }
            if let Ok(decoded)
                = <Log133Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log133(decoded));
            }
            if let Ok(decoded)
                = <Log134Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log134(decoded));
            }
            if let Ok(decoded)
                = <Log135Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log135(decoded));
            }
            if let Ok(decoded)
                = <Log136Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log136(decoded));
            }
            if let Ok(decoded)
                = <Log1Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log1(decoded));
            }
            if let Ok(decoded)
                = <Log137Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log137(decoded));
            }
            if let Ok(decoded)
                = <Log37Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log37(decoded));
            }
            if let Ok(decoded)
                = <Log138Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log138(decoded));
            }
            if let Ok(decoded)
                = <Log139Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log139(decoded));
            }
            if let Ok(decoded)
                = <Log8Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log8(decoded));
            }
            if let Ok(decoded)
                = <Log2Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log2(decoded));
            }
            if let Ok(decoded)
                = <Log140Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log140(decoded));
            }
            if let Ok(decoded)
                = <Log141Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log141(decoded));
            }
            if let Ok(decoded)
                = <Log38Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log38(decoded));
            }
            if let Ok(decoded)
                = <Log142Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log142(decoded));
            }
            if let Ok(decoded)
                = <Log143Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log143(decoded));
            }
            if let Ok(decoded)
                = <Log39Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log39(decoded));
            }
            if let Ok(decoded)
                = <Log144Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log144(decoded));
            }
            if let Ok(decoded)
                = <Log40Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log40(decoded));
            }
            if let Ok(decoded)
                = <Log145Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log145(decoded));
            }
            if let Ok(decoded)
                = <Log146Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log146(decoded));
            }
            if let Ok(decoded)
                = <Log9Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log9(decoded));
            }
            if let Ok(decoded)
                = <Log147Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log147(decoded));
            }
            if let Ok(decoded)
                = <Log148Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log148(decoded));
            }
            if let Ok(decoded)
                = <Log149Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log149(decoded));
            }
            if let Ok(decoded)
                = <Log150Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log150(decoded));
            }
            if let Ok(decoded)
                = <Log151Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log151(decoded));
            }
            if let Ok(decoded)
                = <Log152Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log152(decoded));
            }
            if let Ok(decoded)
                = <Log153Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log153(decoded));
            }
            if let Ok(decoded)
                = <Log3Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log3(decoded));
            }
            if let Ok(decoded)
                = <Log154Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log154(decoded));
            }
            if let Ok(decoded)
                = <Log155Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log155(decoded));
            }
            if let Ok(decoded)
                = <Log156Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log156(decoded));
            }
            if let Ok(decoded)
                = <Log157Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log157(decoded));
            }
            if let Ok(decoded)
                = <Log158Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log158(decoded));
            }
            if let Ok(decoded)
                = <Log159Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log159(decoded));
            }
            if let Ok(decoded)
                = <Log160Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log160(decoded));
            }
            if let Ok(decoded)
                = <Log161Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log161(decoded));
            }
            if let Ok(decoded)
                = <Log41Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log41(decoded));
            }
            if let Ok(decoded)
                = <Log162Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log162(decoded));
            }
            if let Ok(decoded)
                = <Log163Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log163(decoded));
            }
            if let Ok(decoded)
                = <Log164Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log164(decoded));
            }
            if let Ok(decoded)
                = <Log165Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log165(decoded));
            }
            if let Ok(decoded)
                = <Log10Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log10(decoded));
            }
            if let Ok(decoded)
                = <Log166Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log166(decoded));
            }
            if let Ok(decoded)
                = <Log42Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log42(decoded));
            }
            if let Ok(decoded)
                = <Log167Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log167(decoded));
            }
            if let Ok(decoded)
                = <Log43Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log43(decoded));
            }
            if let Ok(decoded)
                = <Log168Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log168(decoded));
            }
            if let Ok(decoded)
                = <Log169Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log169(decoded));
            }
            if let Ok(decoded)
                = <Log0Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log0(decoded));
            }
            if let Ok(decoded)
                = <Log170Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log170(decoded));
            }
            if let Ok(decoded)
                = <Log171Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log171(decoded));
            }
            if let Ok(decoded)
                = <Log172Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log172(decoded));
            }
            if let Ok(decoded)
                = <Log173Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log173(decoded));
            }
            if let Ok(decoded)
                = <Log44Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log44(decoded));
            }
            if let Ok(decoded)
                = <Log45Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log45(decoded));
            }
            if let Ok(decoded)
                = <Log174Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log174(decoded));
            }
            if let Ok(decoded)
                = <Log175Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log175(decoded));
            }
            if let Ok(decoded)
                = <Log46Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log46(decoded));
            }
            if let Ok(decoded)
                = <Log176Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log176(decoded));
            }
            if let Ok(decoded)
                = <Log177Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log177(decoded));
            }
            if let Ok(decoded)
                = <Log178Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log178(decoded));
            }
            if let Ok(decoded)
                = <Log47Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log47(decoded));
            }
            if let Ok(decoded)
                = <Log179Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log179(decoded));
            }
            if let Ok(decoded)
                = <Log180Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log180(decoded));
            }
            if let Ok(decoded)
                = <Log181Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log181(decoded));
            }
            if let Ok(decoded)
                = <Log182Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log182(decoded));
            }
            if let Ok(decoded)
                = <Log183Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log183(decoded));
            }
            if let Ok(decoded)
                = <Log184Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log184(decoded));
            }
            if let Ok(decoded)
                = <Log185Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log185(decoded));
            }
            if let Ok(decoded)
                = <Log186Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log186(decoded));
            }
            if let Ok(decoded)
                = <Log187Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log187(decoded));
            }
            if let Ok(decoded)
                = <Log188Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log188(decoded));
            }
            if let Ok(decoded)
                = <Log48Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log48(decoded));
            }
            if let Ok(decoded)
                = <Log189Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log189(decoded));
            }
            if let Ok(decoded)
                = <Log190Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log190(decoded));
            }
            if let Ok(decoded)
                = <Log191Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log191(decoded));
            }
            if let Ok(decoded)
                = <Log49Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log49(decoded));
            }
            if let Ok(decoded)
                = <Log192Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log192(decoded));
            }
            if let Ok(decoded)
                = <Log11Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log11(decoded));
            }
            if let Ok(decoded)
                = <Log193Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log193(decoded));
            }
            if let Ok(decoded)
                = <Log194Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log194(decoded));
            }
            if let Ok(decoded)
                = <Log195Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log195(decoded));
            }
            if let Ok(decoded)
                = <Log196Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log196(decoded));
            }
            if let Ok(decoded)
                = <Log50Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log50(decoded));
            }
            if let Ok(decoded)
                = <Log51Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log51(decoded));
            }
            if let Ok(decoded)
                = <Log197Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log197(decoded));
            }
            if let Ok(decoded)
                = <Log198Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log198(decoded));
            }
            if let Ok(decoded)
                = <Log12Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log12(decoded));
            }
            if let Ok(decoded)
                = <Log199Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log199(decoded));
            }
            if let Ok(decoded)
                = <Log200Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log200(decoded));
            }
            if let Ok(decoded)
                = <Log201Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log201(decoded));
            }
            if let Ok(decoded)
                = <Log202Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log202(decoded));
            }
            if let Ok(decoded)
                = <Log203Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log203(decoded));
            }
            if let Ok(decoded)
                = <Log204Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log204(decoded));
            }
            if let Ok(decoded)
                = <Log205Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log205(decoded));
            }
            if let Ok(decoded)
                = <Log206Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log206(decoded));
            }
            if let Ok(decoded)
                = <Log207Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log207(decoded));
            }
            if let Ok(decoded)
                = <Log208Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log208(decoded));
            }
            if let Ok(decoded)
                = <Log209Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log209(decoded));
            }
            if let Ok(decoded)
                = <Log210Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log210(decoded));
            }
            if let Ok(decoded)
                = <Log52Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log52(decoded));
            }
            if let Ok(decoded)
                = <Log211Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log211(decoded));
            }
            if let Ok(decoded)
                = <Log212Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log212(decoded));
            }
            if let Ok(decoded)
                = <Log213Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log213(decoded));
            }
            if let Ok(decoded)
                = <Log13Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log13(decoded));
            }
            if let Ok(decoded)
                = <Log14Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log14(decoded));
            }
            if let Ok(decoded)
                = <Log214Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log214(decoded));
            }
            if let Ok(decoded)
                = <Log215Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log215(decoded));
            }
            if let Ok(decoded)
                = <Log216Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log216(decoded));
            }
            if let Ok(decoded)
                = <Log53Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log53(decoded));
            }
            if let Ok(decoded)
                = <Log54Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log54(decoded));
            }
            if let Ok(decoded)
                = <Log217Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log217(decoded));
            }
            if let Ok(decoded)
                = <Log218Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log218(decoded));
            }
            if let Ok(decoded)
                = <Log219Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log219(decoded));
            }
            if let Ok(decoded)
                = <Log220Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log220(decoded));
            }
            if let Ok(decoded)
                = <Log221Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log221(decoded));
            }
            if let Ok(decoded)
                = <Log222Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log222(decoded));
            }
            if let Ok(decoded)
                = <Log223Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log223(decoded));
            }
            if let Ok(decoded)
                = <Log224Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log224(decoded));
            }
            if let Ok(decoded)
                = <Log225Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log225(decoded));
            }
            if let Ok(decoded)
                = <Log226Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log226(decoded));
            }
            if let Ok(decoded)
                = <Log227Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log227(decoded));
            }
            if let Ok(decoded)
                = <Log15Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log15(decoded));
            }
            if let Ok(decoded)
                = <Log55Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log55(decoded));
            }
            if let Ok(decoded)
                = <Log16Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log16(decoded));
            }
            if let Ok(decoded)
                = <Log228Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log228(decoded));
            }
            if let Ok(decoded)
                = <Log56Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log56(decoded));
            }
            if let Ok(decoded)
                = <Log229Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log229(decoded));
            }
            if let Ok(decoded)
                = <Log230Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log230(decoded));
            }
            if let Ok(decoded)
                = <Log231Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log231(decoded));
            }
            if let Ok(decoded)
                = <Log232Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log232(decoded));
            }
            if let Ok(decoded)
                = <Log233Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log233(decoded));
            }
            if let Ok(decoded)
                = <Log234Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log234(decoded));
            }
            if let Ok(decoded)
                = <Log235Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log235(decoded));
            }
            if let Ok(decoded)
                = <Log236Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log236(decoded));
            }
            if let Ok(decoded)
                = <Log237Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log237(decoded));
            }
            if let Ok(decoded)
                = <Log238Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log238(decoded));
            }
            if let Ok(decoded)
                = <Log239Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log239(decoded));
            }
            if let Ok(decoded)
                = <Log240Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log240(decoded));
            }
            if let Ok(decoded)
                = <Log241Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log241(decoded));
            }
            if let Ok(decoded)
                = <Log17Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log17(decoded));
            }
            if let Ok(decoded)
                = <Log242Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log242(decoded));
            }
            if let Ok(decoded)
                = <Log243Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log243(decoded));
            }
            if let Ok(decoded)
                = <Log244Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log244(decoded));
            }
            if let Ok(decoded)
                = <Log245Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log245(decoded));
            }
            if let Ok(decoded)
                = <Log246Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log246(decoded));
            }
            if let Ok(decoded)
                = <Log57Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log57(decoded));
            }
            if let Ok(decoded)
                = <Log247Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log247(decoded));
            }
            if let Ok(decoded)
                = <Log248Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log248(decoded));
            }
            if let Ok(decoded)
                = <Log249Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log249(decoded));
            }
            if let Ok(decoded)
                = <Log58Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log58(decoded));
            }
            if let Ok(decoded)
                = <Log59Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log59(decoded));
            }
            if let Ok(decoded)
                = <Log250Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log250(decoded));
            }
            if let Ok(decoded)
                = <Log251Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log251(decoded));
            }
            if let Ok(decoded)
                = <Log252Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log252(decoded));
            }
            if let Ok(decoded)
                = <Log253Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log253(decoded));
            }
            if let Ok(decoded)
                = <Log60Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log60(decoded));
            }
            if let Ok(decoded)
                = <Log254Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log254(decoded));
            }
            if let Ok(decoded)
                = <Log61Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log61(decoded));
            }
            if let Ok(decoded)
                = <Log255Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log255(decoded));
            }
            if let Ok(decoded)
                = <Log256Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log256(decoded));
            }
            if let Ok(decoded)
                = <Log257Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log257(decoded));
            }
            if let Ok(decoded)
                = <Log258Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log258(decoded));
            }
            if let Ok(decoded)
                = <Log259Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log259(decoded));
            }
            if let Ok(decoded)
                = <Log260Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log260(decoded));
            }
            if let Ok(decoded)
                = <Log261Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log261(decoded));
            }
            if let Ok(decoded)
                = <Log262Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log262(decoded));
            }
            if let Ok(decoded)
                = <Log62Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log62(decoded));
            }
            if let Ok(decoded)
                = <Log263Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log263(decoded));
            }
            if let Ok(decoded)
                = <Log264Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log264(decoded));
            }
            if let Ok(decoded)
                = <Log265Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log265(decoded));
            }
            if let Ok(decoded)
                = <Log266Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log266(decoded));
            }
            if let Ok(decoded)
                = <Log267Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log267(decoded));
            }
            if let Ok(decoded)
                = <Log268Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log268(decoded));
            }
            if let Ok(decoded)
                = <Log269Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log269(decoded));
            }
            if let Ok(decoded)
                = <Log270Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log270(decoded));
            }
            if let Ok(decoded)
                = <Log271Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log271(decoded));
            }
            if let Ok(decoded)
                = <Log272Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log272(decoded));
            }
            if let Ok(decoded)
                = <Log273Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log273(decoded));
            }
            if let Ok(decoded)
                = <Log274Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log274(decoded));
            }
            if let Ok(decoded)
                = <Log275Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log275(decoded));
            }
            if let Ok(decoded)
                = <Log276Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log276(decoded));
            }
            if let Ok(decoded)
                = <Log277Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log277(decoded));
            }
            if let Ok(decoded)
                = <Log63Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log63(decoded));
            }
            if let Ok(decoded)
                = <Log64Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log64(decoded));
            }
            if let Ok(decoded)
                = <Log65Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log65(decoded));
            }
            if let Ok(decoded)
                = <Log278Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log278(decoded));
            }
            if let Ok(decoded)
                = <Log279Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log279(decoded));
            }
            if let Ok(decoded)
                = <Log280Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log280(decoded));
            }
            if let Ok(decoded)
                = <Log18Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log18(decoded));
            }
            if let Ok(decoded)
                = <Log66Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log66(decoded));
            }
            if let Ok(decoded)
                = <Log281Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log281(decoded));
            }
            if let Ok(decoded)
                = <Log282Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log282(decoded));
            }
            if let Ok(decoded)
                = <Log283Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log283(decoded));
            }
            if let Ok(decoded)
                = <Log284Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log284(decoded));
            }
            if let Ok(decoded)
                = <Log285Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log285(decoded));
            }
            if let Ok(decoded)
                = <Log67Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log67(decoded));
            }
            if let Ok(decoded)
                = <Log286Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log286(decoded));
            }
            if let Ok(decoded)
                = <Log287Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log287(decoded));
            }
            if let Ok(decoded)
                = <Log288Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log288(decoded));
            }
            if let Ok(decoded)
                = <Log289Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log289(decoded));
            }
            if let Ok(decoded)
                = <Log290Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log290(decoded));
            }
            if let Ok(decoded)
                = <Log291Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log291(decoded));
            }
            if let Ok(decoded)
                = <Log292Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log292(decoded));
            }
            if let Ok(decoded)
                = <Log19Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log19(decoded));
            }
            if let Ok(decoded)
                = <Log68Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log68(decoded));
            }
            if let Ok(decoded)
                = <Log293Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log293(decoded));
            }
            if let Ok(decoded)
                = <Log294Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log294(decoded));
            }
            if let Ok(decoded)
                = <Log295Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log295(decoded));
            }
            if let Ok(decoded)
                = <Log296Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log296(decoded));
            }
            if let Ok(decoded)
                = <Log297Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log297(decoded));
            }
            if let Ok(decoded)
                = <Log69Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log69(decoded));
            }
            if let Ok(decoded)
                = <Log70Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log70(decoded));
            }
            if let Ok(decoded)
                = <Log71Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log71(decoded));
            }
            if let Ok(decoded)
                = <Log72Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log72(decoded));
            }
            if let Ok(decoded)
                = <Log298Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log298(decoded));
            }
            if let Ok(decoded)
                = <Log299Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log299(decoded));
            }
            if let Ok(decoded)
                = <Log300Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log300(decoded));
            }
            if let Ok(decoded)
                = <Log301Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log301(decoded));
            }
            if let Ok(decoded)
                = <Log302Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log302(decoded));
            }
            if let Ok(decoded)
                = <Log73Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log73(decoded));
            }
            if let Ok(decoded)
                = <Log303Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log303(decoded));
            }
            if let Ok(decoded)
                = <Log304Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log304(decoded));
            }
            if let Ok(decoded)
                = <Log74Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log74(decoded));
            }
            if let Ok(decoded)
                = <Log75Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log75(decoded));
            }
            if let Ok(decoded)
                = <Log305Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log305(decoded));
            }
            if let Ok(decoded)
                = <Log306Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log306(decoded));
            }
            if let Ok(decoded)
                = <Log307Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log307(decoded));
            }
            if let Ok(decoded)
                = <Log308Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log308(decoded));
            }
            if let Ok(decoded)
                = <Log309Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log309(decoded));
            }
            if let Ok(decoded)
                = <Log20Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log20(decoded));
            }
            if let Ok(decoded)
                = <Log76Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log76(decoded));
            }
            if let Ok(decoded)
                = <Log310Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log310(decoded));
            }
            if let Ok(decoded)
                = <Log311Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log311(decoded));
            }
            if let Ok(decoded)
                = <Log312Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log312(decoded));
            }
            if let Ok(decoded)
                = <Log313Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log313(decoded));
            }
            if let Ok(decoded)
                = <Log314Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log314(decoded));
            }
            if let Ok(decoded)
                = <Log77Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log77(decoded));
            }
            if let Ok(decoded)
                = <Log315Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log315(decoded));
            }
            if let Ok(decoded)
                = <Log316Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log316(decoded));
            }
            if let Ok(decoded)
                = <Log317Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log317(decoded));
            }
            if let Ok(decoded)
                = <Log78Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log78(decoded));
            }
            if let Ok(decoded)
                = <Log318Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log318(decoded));
            }
            if let Ok(decoded)
                = <Log79Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log79(decoded));
            }
            if let Ok(decoded)
                = <Log319Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log319(decoded));
            }
            if let Ok(decoded)
                = <Log320Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log320(decoded));
            }
            if let Ok(decoded)
                = <Log321Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log321(decoded));
            }
            if let Ok(decoded)
                = <Log322Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log322(decoded));
            }
            if let Ok(decoded)
                = <Log323Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log323(decoded));
            }
            if let Ok(decoded)
                = <Log324Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log324(decoded));
            }
            if let Ok(decoded)
                = <Log80Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log80(decoded));
            }
            if let Ok(decoded)
                = <Log325Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log325(decoded));
            }
            if let Ok(decoded)
                = <Log326Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log326(decoded));
            }
            if let Ok(decoded)
                = <Log81Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log81(decoded));
            }
            if let Ok(decoded)
                = <Log327Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log327(decoded));
            }
            if let Ok(decoded)
                = <Log328Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log328(decoded));
            }
            if let Ok(decoded)
                = <Log329Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log329(decoded));
            }
            if let Ok(decoded)
                = <Log330Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log330(decoded));
            }
            if let Ok(decoded)
                = <Log331Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log331(decoded));
            }
            if let Ok(decoded)
                = <Log82Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log82(decoded));
            }
            if let Ok(decoded)
                = <Log83Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log83(decoded));
            }
            if let Ok(decoded)
                = <Log84Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log84(decoded));
            }
            if let Ok(decoded)
                = <Log332Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log332(decoded));
            }
            if let Ok(decoded)
                = <Log333Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log333(decoded));
            }
            if let Ok(decoded)
                = <Log334Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log334(decoded));
            }
            if let Ok(decoded)
                = <Log21Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log21(decoded));
            }
            if let Ok(decoded)
                = <Log335Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log335(decoded));
            }
            if let Ok(decoded)
                = <Log336Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log336(decoded));
            }
            if let Ok(decoded)
                = <Log4Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log4(decoded));
            }
            if let Ok(decoded)
                = <Log337Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log337(decoded));
            }
            if let Ok(decoded)
                = <Log338Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log338(decoded));
            }
            if let Ok(decoded)
                = <Log339Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log339(decoded));
            }
            if let Ok(decoded)
                = <Log85Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log85(decoded));
            }
            if let Ok(decoded)
                = <Log340Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log340(decoded));
            }
            if let Ok(decoded)
                = <Log86Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log86(decoded));
            }
            if let Ok(decoded)
                = <Log341Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log341(decoded));
            }
            if let Ok(decoded)
                = <Log342Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log342(decoded));
            }
            if let Ok(decoded)
                = <Log5Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log5(decoded));
            }
            if let Ok(decoded)
                = <Log22Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::Log22(decoded));
            }
            if let Ok(decoded)
                = <LogAddressCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogAddress(decoded));
            }
            if let Ok(decoded)
                = <LogBoolCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBool(decoded));
            }
            if let Ok(decoded)
                = <LogBytesCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes(decoded));
            }
            if let Ok(decoded)
                = <LogBytes1Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes1(decoded));
            }
            if let Ok(decoded)
                = <LogBytes10Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes10(decoded));
            }
            if let Ok(decoded)
                = <LogBytes11Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes11(decoded));
            }
            if let Ok(decoded)
                = <LogBytes12Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes12(decoded));
            }
            if let Ok(decoded)
                = <LogBytes13Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes13(decoded));
            }
            if let Ok(decoded)
                = <LogBytes14Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes14(decoded));
            }
            if let Ok(decoded)
                = <LogBytes15Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes15(decoded));
            }
            if let Ok(decoded)
                = <LogBytes16Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes16(decoded));
            }
            if let Ok(decoded)
                = <LogBytes17Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes17(decoded));
            }
            if let Ok(decoded)
                = <LogBytes18Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes18(decoded));
            }
            if let Ok(decoded)
                = <LogBytes19Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes19(decoded));
            }
            if let Ok(decoded)
                = <LogBytes2Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes2(decoded));
            }
            if let Ok(decoded)
                = <LogBytes20Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes20(decoded));
            }
            if let Ok(decoded)
                = <LogBytes21Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes21(decoded));
            }
            if let Ok(decoded)
                = <LogBytes22Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes22(decoded));
            }
            if let Ok(decoded)
                = <LogBytes23Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes23(decoded));
            }
            if let Ok(decoded)
                = <LogBytes24Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes24(decoded));
            }
            if let Ok(decoded)
                = <LogBytes25Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes25(decoded));
            }
            if let Ok(decoded)
                = <LogBytes26Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes26(decoded));
            }
            if let Ok(decoded)
                = <LogBytes27Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes27(decoded));
            }
            if let Ok(decoded)
                = <LogBytes28Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes28(decoded));
            }
            if let Ok(decoded)
                = <LogBytes29Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes29(decoded));
            }
            if let Ok(decoded)
                = <LogBytes3Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes3(decoded));
            }
            if let Ok(decoded)
                = <LogBytes30Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes30(decoded));
            }
            if let Ok(decoded)
                = <LogBytes31Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes31(decoded));
            }
            if let Ok(decoded)
                = <LogBytes32Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes32(decoded));
            }
            if let Ok(decoded)
                = <LogBytes4Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes4(decoded));
            }
            if let Ok(decoded)
                = <LogBytes5Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes5(decoded));
            }
            if let Ok(decoded)
                = <LogBytes6Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes6(decoded));
            }
            if let Ok(decoded)
                = <LogBytes7Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes7(decoded));
            }
            if let Ok(decoded)
                = <LogBytes8Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes8(decoded));
            }
            if let Ok(decoded)
                = <LogBytes9Call as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogBytes9(decoded));
            }
            if let Ok(decoded)
                = <LogIntCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogInt(decoded));
            }
            if let Ok(decoded)
                = <LogStringCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogString(decoded));
            }
            if let Ok(decoded)
                = <LogUintCall as ::ethers_core::abi::AbiDecode>::decode(data) {
                return Ok(Self::LogUint(decoded));
            }
            Err(::ethers_core::abi::Error::InvalidData.into())
        }
    }
    impl ::ethers_core::abi::AbiEncode for HardhatConsoleCalls {
        fn encode(self) -> Vec<u8> {
            match self {
                Self::Log23(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log87(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log24(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log88(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log89(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log90(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log91(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log25(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log92(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log93(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log94(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log95(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log96(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log26(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log97(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log98(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log99(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log100(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log101(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log102(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log27(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log28(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log103(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log29(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log104(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log105(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log106(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log107(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log108(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log109(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log110(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log111(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log30(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log31(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log112(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log113(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log114(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log115(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log116(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log32(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log6(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log117(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log118(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log119(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log120(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log33(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log121(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log34(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log122(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log35(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log123(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log124(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log125(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log126(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log127(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log128(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log129(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log36(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log130(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log131(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log132(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log7(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log133(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log134(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log135(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log136(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log1(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log137(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log37(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log138(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log139(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log8(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log2(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log140(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log141(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log38(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log142(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log143(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log39(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log144(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log40(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log145(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log146(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log9(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log147(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log148(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log149(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log150(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log151(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log152(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log153(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log3(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log154(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log155(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log156(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log157(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log158(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log159(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log160(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log161(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log41(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log162(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log163(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log164(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log165(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log10(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log166(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log42(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log167(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log43(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log168(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log169(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log0(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log170(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log171(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log172(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log173(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log44(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log45(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log174(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log175(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log46(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log176(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log177(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log178(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log47(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log179(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log180(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log181(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log182(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log183(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log184(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log185(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log186(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log187(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log188(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log48(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log189(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log190(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log191(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log49(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log192(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log11(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log193(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log194(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log195(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log196(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log50(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log51(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log197(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log198(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log12(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log199(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log200(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log201(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log202(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log203(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log204(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log205(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log206(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log207(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log208(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log209(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log210(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log52(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log211(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log212(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log213(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log13(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log14(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log214(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log215(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log216(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log53(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log54(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log217(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log218(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log219(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log220(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log221(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log222(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log223(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log224(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log225(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log226(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log227(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log15(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log55(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log16(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log228(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log56(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log229(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log230(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log231(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log232(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log233(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log234(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log235(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log236(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log237(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log238(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log239(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log240(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log241(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log17(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log242(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log243(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log244(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log245(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log246(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log57(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log247(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log248(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log249(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log58(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log59(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log250(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log251(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log252(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log253(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log60(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log254(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log61(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log255(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log256(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log257(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log258(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log259(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log260(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log261(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log262(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log62(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log263(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log264(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log265(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log266(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log267(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log268(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log269(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log270(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log271(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log272(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log273(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log274(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log275(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log276(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log277(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log63(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log64(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log65(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log278(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log279(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log280(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log18(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log66(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log281(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log282(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log283(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log284(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log285(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log67(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log286(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log287(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log288(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log289(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log290(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log291(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log292(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log19(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log68(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log293(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log294(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log295(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log296(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log297(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log69(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log70(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log71(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log72(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log298(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log299(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log300(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log301(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log302(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log73(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log303(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log304(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log74(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log75(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log305(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log306(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log307(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log308(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log309(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log20(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log76(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log310(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log311(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log312(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log313(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log314(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log77(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log315(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log316(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log317(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log78(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log318(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log79(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log319(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log320(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log321(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log322(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log323(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log324(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log80(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log325(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log326(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log81(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log327(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log328(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log329(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log330(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log331(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log82(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log83(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log84(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log332(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log333(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log334(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log21(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log335(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log336(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log4(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log337(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log338(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log339(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log85(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log340(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log86(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log341(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log342(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log5(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::Log22(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LogAddress(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBool(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LogBytes(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LogBytes1(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes10(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes11(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes12(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes13(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes14(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes15(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes16(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes17(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes18(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes19(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes2(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes20(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes21(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes22(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes23(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes24(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes25(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes26(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes27(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes28(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes29(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes3(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes30(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes31(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes32(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes4(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes5(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes6(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes7(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes8(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogBytes9(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogInt(element) => ::ethers_core::abi::AbiEncode::encode(element),
                Self::LogString(element) => {
                    ::ethers_core::abi::AbiEncode::encode(element)
                }
                Self::LogUint(element) => ::ethers_core::abi::AbiEncode::encode(element),
            }
        }
    }
    impl ::core::fmt::Display for HardhatConsoleCalls {
        fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            match self {
                Self::Log23(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log87(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log24(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log88(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log89(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log90(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log91(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log25(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log92(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log93(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log94(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log95(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log96(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log26(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log97(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log98(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log99(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log100(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log101(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log102(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log27(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log28(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log103(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log29(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log104(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log105(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log106(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log107(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log108(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log109(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log110(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log111(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log30(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log31(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log112(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log113(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log114(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log115(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log116(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log32(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log6(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log117(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log118(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log119(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log120(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log33(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log121(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log34(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log122(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log35(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log123(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log124(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log125(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log126(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log127(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log128(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log129(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log36(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log130(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log131(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log132(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log7(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log133(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log134(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log135(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log136(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log1(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log137(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log37(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log138(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log139(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log8(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log2(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log140(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log141(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log38(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log142(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log143(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log39(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log144(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log40(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log145(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log146(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log9(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log147(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log148(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log149(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log150(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log151(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log152(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log153(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log3(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log154(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log155(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log156(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log157(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log158(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log159(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log160(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log161(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log41(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log162(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log163(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log164(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log165(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log10(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log166(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log42(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log167(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log43(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log168(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log169(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log0(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log170(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log171(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log172(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log173(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log44(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log45(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log174(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log175(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log46(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log176(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log177(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log178(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log47(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log179(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log180(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log181(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log182(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log183(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log184(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log185(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log186(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log187(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log188(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log48(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log189(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log190(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log191(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log49(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log192(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log11(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log193(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log194(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log195(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log196(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log50(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log51(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log197(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log198(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log12(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log199(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log200(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log201(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log202(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log203(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log204(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log205(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log206(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log207(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log208(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log209(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log210(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log52(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log211(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log212(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log213(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log13(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log14(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log214(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log215(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log216(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log53(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log54(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log217(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log218(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log219(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log220(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log221(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log222(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log223(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log224(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log225(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log226(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log227(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log15(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log55(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log16(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log228(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log56(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log229(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log230(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log231(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log232(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log233(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log234(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log235(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log236(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log237(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log238(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log239(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log240(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log241(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log17(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log242(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log243(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log244(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log245(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log246(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log57(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log247(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log248(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log249(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log58(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log59(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log250(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log251(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log252(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log253(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log60(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log254(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log61(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log255(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log256(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log257(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log258(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log259(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log260(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log261(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log262(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log62(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log263(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log264(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log265(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log266(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log267(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log268(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log269(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log270(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log271(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log272(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log273(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log274(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log275(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log276(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log277(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log63(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log64(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log65(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log278(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log279(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log280(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log18(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log66(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log281(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log282(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log283(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log284(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log285(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log67(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log286(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log287(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log288(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log289(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log290(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log291(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log292(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log19(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log68(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log293(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log294(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log295(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log296(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log297(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log69(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log70(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log71(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log72(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log298(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log299(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log300(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log301(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log302(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log73(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log303(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log304(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log74(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log75(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log305(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log306(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log307(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log308(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log309(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log20(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log76(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log310(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log311(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log312(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log313(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log314(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log77(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log315(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log316(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log317(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log78(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log318(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log79(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log319(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log320(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log321(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log322(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log323(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log324(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log80(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log325(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log326(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log81(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log327(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log328(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log329(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log330(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log331(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log82(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log83(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log84(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log332(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log333(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log334(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log21(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log335(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log336(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log4(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log337(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log338(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log339(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log85(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log340(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log86(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log341(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log342(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log5(element) => ::core::fmt::Display::fmt(element, f),
                Self::Log22(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogAddress(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBool(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes1(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes10(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes11(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes12(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes13(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes14(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes15(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes16(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes17(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes18(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes19(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes2(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes20(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes21(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes22(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes23(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes24(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes25(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes26(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes27(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes28(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes29(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes3(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes30(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes31(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes32(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes4(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes5(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes6(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes7(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes8(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogBytes9(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogInt(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogString(element) => ::core::fmt::Display::fmt(element, f),
                Self::LogUint(element) => ::core::fmt::Display::fmt(element, f),
            }
        }
    }
    impl ::core::convert::From<Log23Call> for HardhatConsoleCalls {
        fn from(value: Log23Call) -> Self {
            Self::Log23(value)
        }
    }
    impl ::core::convert::From<Log87Call> for HardhatConsoleCalls {
        fn from(value: Log87Call) -> Self {
            Self::Log87(value)
        }
    }
    impl ::core::convert::From<Log24Call> for HardhatConsoleCalls {
        fn from(value: Log24Call) -> Self {
            Self::Log24(value)
        }
    }
    impl ::core::convert::From<Log88Call> for HardhatConsoleCalls {
        fn from(value: Log88Call) -> Self {
            Self::Log88(value)
        }
    }
    impl ::core::convert::From<Log89Call> for HardhatConsoleCalls {
        fn from(value: Log89Call) -> Self {
            Self::Log89(value)
        }
    }
    impl ::core::convert::From<Log90Call> for HardhatConsoleCalls {
        fn from(value: Log90Call) -> Self {
            Self::Log90(value)
        }
    }
    impl ::core::convert::From<Log91Call> for HardhatConsoleCalls {
        fn from(value: Log91Call) -> Self {
            Self::Log91(value)
        }
    }
    impl ::core::convert::From<Log25Call> for HardhatConsoleCalls {
        fn from(value: Log25Call) -> Self {
            Self::Log25(value)
        }
    }
    impl ::core::convert::From<Log92Call> for HardhatConsoleCalls {
        fn from(value: Log92Call) -> Self {
            Self::Log92(value)
        }
    }
    impl ::core::convert::From<Log93Call> for HardhatConsoleCalls {
        fn from(value: Log93Call) -> Self {
            Self::Log93(value)
        }
    }
    impl ::core::convert::From<Log94Call> for HardhatConsoleCalls {
        fn from(value: Log94Call) -> Self {
            Self::Log94(value)
        }
    }
    impl ::core::convert::From<Log95Call> for HardhatConsoleCalls {
        fn from(value: Log95Call) -> Self {
            Self::Log95(value)
        }
    }
    impl ::core::convert::From<Log96Call> for HardhatConsoleCalls {
        fn from(value: Log96Call) -> Self {
            Self::Log96(value)
        }
    }
    impl ::core::convert::From<Log26Call> for HardhatConsoleCalls {
        fn from(value: Log26Call) -> Self {
            Self::Log26(value)
        }
    }
    impl ::core::convert::From<Log97Call> for HardhatConsoleCalls {
        fn from(value: Log97Call) -> Self {
            Self::Log97(value)
        }
    }
    impl ::core::convert::From<Log98Call> for HardhatConsoleCalls {
        fn from(value: Log98Call) -> Self {
            Self::Log98(value)
        }
    }
    impl ::core::convert::From<Log99Call> for HardhatConsoleCalls {
        fn from(value: Log99Call) -> Self {
            Self::Log99(value)
        }
    }
    impl ::core::convert::From<Log100Call> for HardhatConsoleCalls {
        fn from(value: Log100Call) -> Self {
            Self::Log100(value)
        }
    }
    impl ::core::convert::From<Log101Call> for HardhatConsoleCalls {
        fn from(value: Log101Call) -> Self {
            Self::Log101(value)
        }
    }
    impl ::core::convert::From<Log102Call> for HardhatConsoleCalls {
        fn from(value: Log102Call) -> Self {
            Self::Log102(value)
        }
    }
    impl ::core::convert::From<Log27Call> for HardhatConsoleCalls {
        fn from(value: Log27Call) -> Self {
            Self::Log27(value)
        }
    }
    impl ::core::convert::From<Log28Call> for HardhatConsoleCalls {
        fn from(value: Log28Call) -> Self {
            Self::Log28(value)
        }
    }
    impl ::core::convert::From<Log103Call> for HardhatConsoleCalls {
        fn from(value: Log103Call) -> Self {
            Self::Log103(value)
        }
    }
    impl ::core::convert::From<Log29Call> for HardhatConsoleCalls {
        fn from(value: Log29Call) -> Self {
            Self::Log29(value)
        }
    }
    impl ::core::convert::From<Log104Call> for HardhatConsoleCalls {
        fn from(value: Log104Call) -> Self {
            Self::Log104(value)
        }
    }
    impl ::core::convert::From<Log105Call> for HardhatConsoleCalls {
        fn from(value: Log105Call) -> Self {
            Self::Log105(value)
        }
    }
    impl ::core::convert::From<Log106Call> for HardhatConsoleCalls {
        fn from(value: Log106Call) -> Self {
            Self::Log106(value)
        }
    }
    impl ::core::convert::From<Log107Call> for HardhatConsoleCalls {
        fn from(value: Log107Call) -> Self {
            Self::Log107(value)
        }
    }
    impl ::core::convert::From<Log108Call> for HardhatConsoleCalls {
        fn from(value: Log108Call) -> Self {
            Self::Log108(value)
        }
    }
    impl ::core::convert::From<Log109Call> for HardhatConsoleCalls {
        fn from(value: Log109Call) -> Self {
            Self::Log109(value)
        }
    }
    impl ::core::convert::From<Log110Call> for HardhatConsoleCalls {
        fn from(value: Log110Call) -> Self {
            Self::Log110(value)
        }
    }
    impl ::core::convert::From<Log111Call> for HardhatConsoleCalls {
        fn from(value: Log111Call) -> Self {
            Self::Log111(value)
        }
    }
    impl ::core::convert::From<Log30Call> for HardhatConsoleCalls {
        fn from(value: Log30Call) -> Self {
            Self::Log30(value)
        }
    }
    impl ::core::convert::From<Log31Call> for HardhatConsoleCalls {
        fn from(value: Log31Call) -> Self {
            Self::Log31(value)
        }
    }
    impl ::core::convert::From<Log112Call> for HardhatConsoleCalls {
        fn from(value: Log112Call) -> Self {
            Self::Log112(value)
        }
    }
    impl ::core::convert::From<Log113Call> for HardhatConsoleCalls {
        fn from(value: Log113Call) -> Self {
            Self::Log113(value)
        }
    }
    impl ::core::convert::From<Log114Call> for HardhatConsoleCalls {
        fn from(value: Log114Call) -> Self {
            Self::Log114(value)
        }
    }
    impl ::core::convert::From<Log115Call> for HardhatConsoleCalls {
        fn from(value: Log115Call) -> Self {
            Self::Log115(value)
        }
    }
    impl ::core::convert::From<Log116Call> for HardhatConsoleCalls {
        fn from(value: Log116Call) -> Self {
            Self::Log116(value)
        }
    }
    impl ::core::convert::From<Log32Call> for HardhatConsoleCalls {
        fn from(value: Log32Call) -> Self {
            Self::Log32(value)
        }
    }
    impl ::core::convert::From<Log6Call> for HardhatConsoleCalls {
        fn from(value: Log6Call) -> Self {
            Self::Log6(value)
        }
    }
    impl ::core::convert::From<Log117Call> for HardhatConsoleCalls {
        fn from(value: Log117Call) -> Self {
            Self::Log117(value)
        }
    }
    impl ::core::convert::From<Log118Call> for HardhatConsoleCalls {
        fn from(value: Log118Call) -> Self {
            Self::Log118(value)
        }
    }
    impl ::core::convert::From<Log119Call> for HardhatConsoleCalls {
        fn from(value: Log119Call) -> Self {
            Self::Log119(value)
        }
    }
    impl ::core::convert::From<Log120Call> for HardhatConsoleCalls {
        fn from(value: Log120Call) -> Self {
            Self::Log120(value)
        }
    }
    impl ::core::convert::From<Log33Call> for HardhatConsoleCalls {
        fn from(value: Log33Call) -> Self {
            Self::Log33(value)
        }
    }
    impl ::core::convert::From<Log121Call> for HardhatConsoleCalls {
        fn from(value: Log121Call) -> Self {
            Self::Log121(value)
        }
    }
    impl ::core::convert::From<Log34Call> for HardhatConsoleCalls {
        fn from(value: Log34Call) -> Self {
            Self::Log34(value)
        }
    }
    impl ::core::convert::From<Log122Call> for HardhatConsoleCalls {
        fn from(value: Log122Call) -> Self {
            Self::Log122(value)
        }
    }
    impl ::core::convert::From<Log35Call> for HardhatConsoleCalls {
        fn from(value: Log35Call) -> Self {
            Self::Log35(value)
        }
    }
    impl ::core::convert::From<Log123Call> for HardhatConsoleCalls {
        fn from(value: Log123Call) -> Self {
            Self::Log123(value)
        }
    }
    impl ::core::convert::From<Log124Call> for HardhatConsoleCalls {
        fn from(value: Log124Call) -> Self {
            Self::Log124(value)
        }
    }
    impl ::core::convert::From<Log125Call> for HardhatConsoleCalls {
        fn from(value: Log125Call) -> Self {
            Self::Log125(value)
        }
    }
    impl ::core::convert::From<Log126Call> for HardhatConsoleCalls {
        fn from(value: Log126Call) -> Self {
            Self::Log126(value)
        }
    }
    impl ::core::convert::From<Log127Call> for HardhatConsoleCalls {
        fn from(value: Log127Call) -> Self {
            Self::Log127(value)
        }
    }
    impl ::core::convert::From<Log128Call> for HardhatConsoleCalls {
        fn from(value: Log128Call) -> Self {
            Self::Log128(value)
        }
    }
    impl ::core::convert::From<Log129Call> for HardhatConsoleCalls {
        fn from(value: Log129Call) -> Self {
            Self::Log129(value)
        }
    }
    impl ::core::convert::From<Log36Call> for HardhatConsoleCalls {
        fn from(value: Log36Call) -> Self {
            Self::Log36(value)
        }
    }
    impl ::core::convert::From<Log130Call> for HardhatConsoleCalls {
        fn from(value: Log130Call) -> Self {
            Self::Log130(value)
        }
    }
    impl ::core::convert::From<Log131Call> for HardhatConsoleCalls {
        fn from(value: Log131Call) -> Self {
            Self::Log131(value)
        }
    }
    impl ::core::convert::From<Log132Call> for HardhatConsoleCalls {
        fn from(value: Log132Call) -> Self {
            Self::Log132(value)
        }
    }
    impl ::core::convert::From<Log7Call> for HardhatConsoleCalls {
        fn from(value: Log7Call) -> Self {
            Self::Log7(value)
        }
    }
    impl ::core::convert::From<Log133Call> for HardhatConsoleCalls {
        fn from(value: Log133Call) -> Self {
            Self::Log133(value)
        }
    }
    impl ::core::convert::From<Log134Call> for HardhatConsoleCalls {
        fn from(value: Log134Call) -> Self {
            Self::Log134(value)
        }
    }
    impl ::core::convert::From<Log135Call> for HardhatConsoleCalls {
        fn from(value: Log135Call) -> Self {
            Self::Log135(value)
        }
    }
    impl ::core::convert::From<Log136Call> for HardhatConsoleCalls {
        fn from(value: Log136Call) -> Self {
            Self::Log136(value)
        }
    }
    impl ::core::convert::From<Log1Call> for HardhatConsoleCalls {
        fn from(value: Log1Call) -> Self {
            Self::Log1(value)
        }
    }
    impl ::core::convert::From<Log137Call> for HardhatConsoleCalls {
        fn from(value: Log137Call) -> Self {
            Self::Log137(value)
        }
    }
    impl ::core::convert::From<Log37Call> for HardhatConsoleCalls {
        fn from(value: Log37Call) -> Self {
            Self::Log37(value)
        }
    }
    impl ::core::convert::From<Log138Call> for HardhatConsoleCalls {
        fn from(value: Log138Call) -> Self {
            Self::Log138(value)
        }
    }
    impl ::core::convert::From<Log139Call> for HardhatConsoleCalls {
        fn from(value: Log139Call) -> Self {
            Self::Log139(value)
        }
    }
    impl ::core::convert::From<Log8Call> for HardhatConsoleCalls {
        fn from(value: Log8Call) -> Self {
            Self::Log8(value)
        }
    }
    impl ::core::convert::From<Log2Call> for HardhatConsoleCalls {
        fn from(value: Log2Call) -> Self {
            Self::Log2(value)
        }
    }
    impl ::core::convert::From<Log140Call> for HardhatConsoleCalls {
        fn from(value: Log140Call) -> Self {
            Self::Log140(value)
        }
    }
    impl ::core::convert::From<Log141Call> for HardhatConsoleCalls {
        fn from(value: Log141Call) -> Self {
            Self::Log141(value)
        }
    }
    impl ::core::convert::From<Log38Call> for HardhatConsoleCalls {
        fn from(value: Log38Call) -> Self {
            Self::Log38(value)
        }
    }
    impl ::core::convert::From<Log142Call> for HardhatConsoleCalls {
        fn from(value: Log142Call) -> Self {
            Self::Log142(value)
        }
    }
    impl ::core::convert::From<Log143Call> for HardhatConsoleCalls {
        fn from(value: Log143Call) -> Self {
            Self::Log143(value)
        }
    }
    impl ::core::convert::From<Log39Call> for HardhatConsoleCalls {
        fn from(value: Log39Call) -> Self {
            Self::Log39(value)
        }
    }
    impl ::core::convert::From<Log144Call> for HardhatConsoleCalls {
        fn from(value: Log144Call) -> Self {
            Self::Log144(value)
        }
    }
    impl ::core::convert::From<Log40Call> for HardhatConsoleCalls {
        fn from(value: Log40Call) -> Self {
            Self::Log40(value)
        }
    }
    impl ::core::convert::From<Log145Call> for HardhatConsoleCalls {
        fn from(value: Log145Call) -> Self {
            Self::Log145(value)
        }
    }
    impl ::core::convert::From<Log146Call> for HardhatConsoleCalls {
        fn from(value: Log146Call) -> Self {
            Self::Log146(value)
        }
    }
    impl ::core::convert::From<Log9Call> for HardhatConsoleCalls {
        fn from(value: Log9Call) -> Self {
            Self::Log9(value)
        }
    }
    impl ::core::convert::From<Log147Call> for HardhatConsoleCalls {
        fn from(value: Log147Call) -> Self {
            Self::Log147(value)
        }
    }
    impl ::core::convert::From<Log148Call> for HardhatConsoleCalls {
        fn from(value: Log148Call) -> Self {
            Self::Log148(value)
        }
    }
    impl ::core::convert::From<Log149Call> for HardhatConsoleCalls {
        fn from(value: Log149Call) -> Self {
            Self::Log149(value)
        }
    }
    impl ::core::convert::From<Log150Call> for HardhatConsoleCalls {
        fn from(value: Log150Call) -> Self {
            Self::Log150(value)
        }
    }
    impl ::core::convert::From<Log151Call> for HardhatConsoleCalls {
        fn from(value: Log151Call) -> Self {
            Self::Log151(value)
        }
    }
    impl ::core::convert::From<Log152Call> for HardhatConsoleCalls {
        fn from(value: Log152Call) -> Self {
            Self::Log152(value)
        }
    }
    impl ::core::convert::From<Log153Call> for HardhatConsoleCalls {
        fn from(value: Log153Call) -> Self {
            Self::Log153(value)
        }
    }
    impl ::core::convert::From<Log3Call> for HardhatConsoleCalls {
        fn from(value: Log3Call) -> Self {
            Self::Log3(value)
        }
    }
    impl ::core::convert::From<Log154Call> for HardhatConsoleCalls {
        fn from(value: Log154Call) -> Self {
            Self::Log154(value)
        }
    }
    impl ::core::convert::From<Log155Call> for HardhatConsoleCalls {
        fn from(value: Log155Call) -> Self {
            Self::Log155(value)
        }
    }
    impl ::core::convert::From<Log156Call> for HardhatConsoleCalls {
        fn from(value: Log156Call) -> Self {
            Self::Log156(value)
        }
    }
    impl ::core::convert::From<Log157Call> for HardhatConsoleCalls {
        fn from(value: Log157Call) -> Self {
            Self::Log157(value)
        }
    }
    impl ::core::convert::From<Log158Call> for HardhatConsoleCalls {
        fn from(value: Log158Call) -> Self {
            Self::Log158(value)
        }
    }
    impl ::core::convert::From<Log159Call> for HardhatConsoleCalls {
        fn from(value: Log159Call) -> Self {
            Self::Log159(value)
        }
    }
    impl ::core::convert::From<Log160Call> for HardhatConsoleCalls {
        fn from(value: Log160Call) -> Self {
            Self::Log160(value)
        }
    }
    impl ::core::convert::From<Log161Call> for HardhatConsoleCalls {
        fn from(value: Log161Call) -> Self {
            Self::Log161(value)
        }
    }
    impl ::core::convert::From<Log41Call> for HardhatConsoleCalls {
        fn from(value: Log41Call) -> Self {
            Self::Log41(value)
        }
    }
    impl ::core::convert::From<Log162Call> for HardhatConsoleCalls {
        fn from(value: Log162Call) -> Self {
            Self::Log162(value)
        }
    }
    impl ::core::convert::From<Log163Call> for HardhatConsoleCalls {
        fn from(value: Log163Call) -> Self {
            Self::Log163(value)
        }
    }
    impl ::core::convert::From<Log164Call> for HardhatConsoleCalls {
        fn from(value: Log164Call) -> Self {
            Self::Log164(value)
        }
    }
    impl ::core::convert::From<Log165Call> for HardhatConsoleCalls {
        fn from(value: Log165Call) -> Self {
            Self::Log165(value)
        }
    }
    impl ::core::convert::From<Log10Call> for HardhatConsoleCalls {
        fn from(value: Log10Call) -> Self {
            Self::Log10(value)
        }
    }
    impl ::core::convert::From<Log166Call> for HardhatConsoleCalls {
        fn from(value: Log166Call) -> Self {
            Self::Log166(value)
        }
    }
    impl ::core::convert::From<Log42Call> for HardhatConsoleCalls {
        fn from(value: Log42Call) -> Self {
            Self::Log42(value)
        }
    }
    impl ::core::convert::From<Log167Call> for HardhatConsoleCalls {
        fn from(value: Log167Call) -> Self {
            Self::Log167(value)
        }
    }
    impl ::core::convert::From<Log43Call> for HardhatConsoleCalls {
        fn from(value: Log43Call) -> Self {
            Self::Log43(value)
        }
    }
    impl ::core::convert::From<Log168Call> for HardhatConsoleCalls {
        fn from(value: Log168Call) -> Self {
            Self::Log168(value)
        }
    }
    impl ::core::convert::From<Log169Call> for HardhatConsoleCalls {
        fn from(value: Log169Call) -> Self {
            Self::Log169(value)
        }
    }
    impl ::core::convert::From<Log0Call> for HardhatConsoleCalls {
        fn from(value: Log0Call) -> Self {
            Self::Log0(value)
        }
    }
    impl ::core::convert::From<Log170Call> for HardhatConsoleCalls {
        fn from(value: Log170Call) -> Self {
            Self::Log170(value)
        }
    }
    impl ::core::convert::From<Log171Call> for HardhatConsoleCalls {
        fn from(value: Log171Call) -> Self {
            Self::Log171(value)
        }
    }
    impl ::core::convert::From<Log172Call> for HardhatConsoleCalls {
        fn from(value: Log172Call) -> Self {
            Self::Log172(value)
        }
    }
    impl ::core::convert::From<Log173Call> for HardhatConsoleCalls {
        fn from(value: Log173Call) -> Self {
            Self::Log173(value)
        }
    }
    impl ::core::convert::From<Log44Call> for HardhatConsoleCalls {
        fn from(value: Log44Call) -> Self {
            Self::Log44(value)
        }
    }
    impl ::core::convert::From<Log45Call> for HardhatConsoleCalls {
        fn from(value: Log45Call) -> Self {
            Self::Log45(value)
        }
    }
    impl ::core::convert::From<Log174Call> for HardhatConsoleCalls {
        fn from(value: Log174Call) -> Self {
            Self::Log174(value)
        }
    }
    impl ::core::convert::From<Log175Call> for HardhatConsoleCalls {
        fn from(value: Log175Call) -> Self {
            Self::Log175(value)
        }
    }
    impl ::core::convert::From<Log46Call> for HardhatConsoleCalls {
        fn from(value: Log46Call) -> Self {
            Self::Log46(value)
        }
    }
    impl ::core::convert::From<Log176Call> for HardhatConsoleCalls {
        fn from(value: Log176Call) -> Self {
            Self::Log176(value)
        }
    }
    impl ::core::convert::From<Log177Call> for HardhatConsoleCalls {
        fn from(value: Log177Call) -> Self {
            Self::Log177(value)
        }
    }
    impl ::core::convert::From<Log178Call> for HardhatConsoleCalls {
        fn from(value: Log178Call) -> Self {
            Self::Log178(value)
        }
    }
    impl ::core::convert::From<Log47Call> for HardhatConsoleCalls {
        fn from(value: Log47Call) -> Self {
            Self::Log47(value)
        }
    }
    impl ::core::convert::From<Log179Call> for HardhatConsoleCalls {
        fn from(value: Log179Call) -> Self {
            Self::Log179(value)
        }
    }
    impl ::core::convert::From<Log180Call> for HardhatConsoleCalls {
        fn from(value: Log180Call) -> Self {
            Self::Log180(value)
        }
    }
    impl ::core::convert::From<Log181Call> for HardhatConsoleCalls {
        fn from(value: Log181Call) -> Self {
            Self::Log181(value)
        }
    }
    impl ::core::convert::From<Log182Call> for HardhatConsoleCalls {
        fn from(value: Log182Call) -> Self {
            Self::Log182(value)
        }
    }
    impl ::core::convert::From<Log183Call> for HardhatConsoleCalls {
        fn from(value: Log183Call) -> Self {
            Self::Log183(value)
        }
    }
    impl ::core::convert::From<Log184Call> for HardhatConsoleCalls {
        fn from(value: Log184Call) -> Self {
            Self::Log184(value)
        }
    }
    impl ::core::convert::From<Log185Call> for HardhatConsoleCalls {
        fn from(value: Log185Call) -> Self {
            Self::Log185(value)
        }
    }
    impl ::core::convert::From<Log186Call> for HardhatConsoleCalls {
        fn from(value: Log186Call) -> Self {
            Self::Log186(value)
        }
    }
    impl ::core::convert::From<Log187Call> for HardhatConsoleCalls {
        fn from(value: Log187Call) -> Self {
            Self::Log187(value)
        }
    }
    impl ::core::convert::From<Log188Call> for HardhatConsoleCalls {
        fn from(value: Log188Call) -> Self {
            Self::Log188(value)
        }
    }
    impl ::core::convert::From<Log48Call> for HardhatConsoleCalls {
        fn from(value: Log48Call) -> Self {
            Self::Log48(value)
        }
    }
    impl ::core::convert::From<Log189Call> for HardhatConsoleCalls {
        fn from(value: Log189Call) -> Self {
            Self::Log189(value)
        }
    }
    impl ::core::convert::From<Log190Call> for HardhatConsoleCalls {
        fn from(value: Log190Call) -> Self {
            Self::Log190(value)
        }
    }
    impl ::core::convert::From<Log191Call> for HardhatConsoleCalls {
        fn from(value: Log191Call) -> Self {
            Self::Log191(value)
        }
    }
    impl ::core::convert::From<Log49Call> for HardhatConsoleCalls {
        fn from(value: Log49Call) -> Self {
            Self::Log49(value)
        }
    }
    impl ::core::convert::From<Log192Call> for HardhatConsoleCalls {
        fn from(value: Log192Call) -> Self {
            Self::Log192(value)
        }
    }
    impl ::core::convert::From<Log11Call> for HardhatConsoleCalls {
        fn from(value: Log11Call) -> Self {
            Self::Log11(value)
        }
    }
    impl ::core::convert::From<Log193Call> for HardhatConsoleCalls {
        fn from(value: Log193Call) -> Self {
            Self::Log193(value)
        }
    }
    impl ::core::convert::From<Log194Call> for HardhatConsoleCalls {
        fn from(value: Log194Call) -> Self {
            Self::Log194(value)
        }
    }
    impl ::core::convert::From<Log195Call> for HardhatConsoleCalls {
        fn from(value: Log195Call) -> Self {
            Self::Log195(value)
        }
    }
    impl ::core::convert::From<Log196Call> for HardhatConsoleCalls {
        fn from(value: Log196Call) -> Self {
            Self::Log196(value)
        }
    }
    impl ::core::convert::From<Log50Call> for HardhatConsoleCalls {
        fn from(value: Log50Call) -> Self {
            Self::Log50(value)
        }
    }
    impl ::core::convert::From<Log51Call> for HardhatConsoleCalls {
        fn from(value: Log51Call) -> Self {
            Self::Log51(value)
        }
    }
    impl ::core::convert::From<Log197Call> for HardhatConsoleCalls {
        fn from(value: Log197Call) -> Self {
            Self::Log197(value)
        }
    }
    impl ::core::convert::From<Log198Call> for HardhatConsoleCalls {
        fn from(value: Log198Call) -> Self {
            Self::Log198(value)
        }
    }
    impl ::core::convert::From<Log12Call> for HardhatConsoleCalls {
        fn from(value: Log12Call) -> Self {
            Self::Log12(value)
        }
    }
    impl ::core::convert::From<Log199Call> for HardhatConsoleCalls {
        fn from(value: Log199Call) -> Self {
            Self::Log199(value)
        }
    }
    impl ::core::convert::From<Log200Call> for HardhatConsoleCalls {
        fn from(value: Log200Call) -> Self {
            Self::Log200(value)
        }
    }
    impl ::core::convert::From<Log201Call> for HardhatConsoleCalls {
        fn from(value: Log201Call) -> Self {
            Self::Log201(value)
        }
    }
    impl ::core::convert::From<Log202Call> for HardhatConsoleCalls {
        fn from(value: Log202Call) -> Self {
            Self::Log202(value)
        }
    }
    impl ::core::convert::From<Log203Call> for HardhatConsoleCalls {
        fn from(value: Log203Call) -> Self {
            Self::Log203(value)
        }
    }
    impl ::core::convert::From<Log204Call> for HardhatConsoleCalls {
        fn from(value: Log204Call) -> Self {
            Self::Log204(value)
        }
    }
    impl ::core::convert::From<Log205Call> for HardhatConsoleCalls {
        fn from(value: Log205Call) -> Self {
            Self::Log205(value)
        }
    }
    impl ::core::convert::From<Log206Call> for HardhatConsoleCalls {
        fn from(value: Log206Call) -> Self {
            Self::Log206(value)
        }
    }
    impl ::core::convert::From<Log207Call> for HardhatConsoleCalls {
        fn from(value: Log207Call) -> Self {
            Self::Log207(value)
        }
    }
    impl ::core::convert::From<Log208Call> for HardhatConsoleCalls {
        fn from(value: Log208Call) -> Self {
            Self::Log208(value)
        }
    }
    impl ::core::convert::From<Log209Call> for HardhatConsoleCalls {
        fn from(value: Log209Call) -> Self {
            Self::Log209(value)
        }
    }
    impl ::core::convert::From<Log210Call> for HardhatConsoleCalls {
        fn from(value: Log210Call) -> Self {
            Self::Log210(value)
        }
    }
    impl ::core::convert::From<Log52Call> for HardhatConsoleCalls {
        fn from(value: Log52Call) -> Self {
            Self::Log52(value)
        }
    }
    impl ::core::convert::From<Log211Call> for HardhatConsoleCalls {
        fn from(value: Log211Call) -> Self {
            Self::Log211(value)
        }
    }
    impl ::core::convert::From<Log212Call> for HardhatConsoleCalls {
        fn from(value: Log212Call) -> Self {
            Self::Log212(value)
        }
    }
    impl ::core::convert::From<Log213Call> for HardhatConsoleCalls {
        fn from(value: Log213Call) -> Self {
            Self::Log213(value)
        }
    }
    impl ::core::convert::From<Log13Call> for HardhatConsoleCalls {
        fn from(value: Log13Call) -> Self {
            Self::Log13(value)
        }
    }
    impl ::core::convert::From<Log14Call> for HardhatConsoleCalls {
        fn from(value: Log14Call) -> Self {
            Self::Log14(value)
        }
    }
    impl ::core::convert::From<Log214Call> for HardhatConsoleCalls {
        fn from(value: Log214Call) -> Self {
            Self::Log214(value)
        }
    }
    impl ::core::convert::From<Log215Call> for HardhatConsoleCalls {
        fn from(value: Log215Call) -> Self {
            Self::Log215(value)
        }
    }
    impl ::core::convert::From<Log216Call> for HardhatConsoleCalls {
        fn from(value: Log216Call) -> Self {
            Self::Log216(value)
        }
    }
    impl ::core::convert::From<Log53Call> for HardhatConsoleCalls {
        fn from(value: Log53Call) -> Self {
            Self::Log53(value)
        }
    }
    impl ::core::convert::From<Log54Call> for HardhatConsoleCalls {
        fn from(value: Log54Call) -> Self {
            Self::Log54(value)
        }
    }
    impl ::core::convert::From<Log217Call> for HardhatConsoleCalls {
        fn from(value: Log217Call) -> Self {
            Self::Log217(value)
        }
    }
    impl ::core::convert::From<Log218Call> for HardhatConsoleCalls {
        fn from(value: Log218Call) -> Self {
            Self::Log218(value)
        }
    }
    impl ::core::convert::From<Log219Call> for HardhatConsoleCalls {
        fn from(value: Log219Call) -> Self {
            Self::Log219(value)
        }
    }
    impl ::core::convert::From<Log220Call> for HardhatConsoleCalls {
        fn from(value: Log220Call) -> Self {
            Self::Log220(value)
        }
    }
    impl ::core::convert::From<Log221Call> for HardhatConsoleCalls {
        fn from(value: Log221Call) -> Self {
            Self::Log221(value)
        }
    }
    impl ::core::convert::From<Log222Call> for HardhatConsoleCalls {
        fn from(value: Log222Call) -> Self {
            Self::Log222(value)
        }
    }
    impl ::core::convert::From<Log223Call> for HardhatConsoleCalls {
        fn from(value: Log223Call) -> Self {
            Self::Log223(value)
        }
    }
    impl ::core::convert::From<Log224Call> for HardhatConsoleCalls {
        fn from(value: Log224Call) -> Self {
            Self::Log224(value)
        }
    }
    impl ::core::convert::From<Log225Call> for HardhatConsoleCalls {
        fn from(value: Log225Call) -> Self {
            Self::Log225(value)
        }
    }
    impl ::core::convert::From<Log226Call> for HardhatConsoleCalls {
        fn from(value: Log226Call) -> Self {
            Self::Log226(value)
        }
    }
    impl ::core::convert::From<Log227Call> for HardhatConsoleCalls {
        fn from(value: Log227Call) -> Self {
            Self::Log227(value)
        }
    }
    impl ::core::convert::From<Log15Call> for HardhatConsoleCalls {
        fn from(value: Log15Call) -> Self {
            Self::Log15(value)
        }
    }
    impl ::core::convert::From<Log55Call> for HardhatConsoleCalls {
        fn from(value: Log55Call) -> Self {
            Self::Log55(value)
        }
    }
    impl ::core::convert::From<Log16Call> for HardhatConsoleCalls {
        fn from(value: Log16Call) -> Self {
            Self::Log16(value)
        }
    }
    impl ::core::convert::From<Log228Call> for HardhatConsoleCalls {
        fn from(value: Log228Call) -> Self {
            Self::Log228(value)
        }
    }
    impl ::core::convert::From<Log56Call> for HardhatConsoleCalls {
        fn from(value: Log56Call) -> Self {
            Self::Log56(value)
        }
    }
    impl ::core::convert::From<Log229Call> for HardhatConsoleCalls {
        fn from(value: Log229Call) -> Self {
            Self::Log229(value)
        }
    }
    impl ::core::convert::From<Log230Call> for HardhatConsoleCalls {
        fn from(value: Log230Call) -> Self {
            Self::Log230(value)
        }
    }
    impl ::core::convert::From<Log231Call> for HardhatConsoleCalls {
        fn from(value: Log231Call) -> Self {
            Self::Log231(value)
        }
    }
    impl ::core::convert::From<Log232Call> for HardhatConsoleCalls {
        fn from(value: Log232Call) -> Self {
            Self::Log232(value)
        }
    }
    impl ::core::convert::From<Log233Call> for HardhatConsoleCalls {
        fn from(value: Log233Call) -> Self {
            Self::Log233(value)
        }
    }
    impl ::core::convert::From<Log234Call> for HardhatConsoleCalls {
        fn from(value: Log234Call) -> Self {
            Self::Log234(value)
        }
    }
    impl ::core::convert::From<Log235Call> for HardhatConsoleCalls {
        fn from(value: Log235Call) -> Self {
            Self::Log235(value)
        }
    }
    impl ::core::convert::From<Log236Call> for HardhatConsoleCalls {
        fn from(value: Log236Call) -> Self {
            Self::Log236(value)
        }
    }
    impl ::core::convert::From<Log237Call> for HardhatConsoleCalls {
        fn from(value: Log237Call) -> Self {
            Self::Log237(value)
        }
    }
    impl ::core::convert::From<Log238Call> for HardhatConsoleCalls {
        fn from(value: Log238Call) -> Self {
            Self::Log238(value)
        }
    }
    impl ::core::convert::From<Log239Call> for HardhatConsoleCalls {
        fn from(value: Log239Call) -> Self {
            Self::Log239(value)
        }
    }
    impl ::core::convert::From<Log240Call> for HardhatConsoleCalls {
        fn from(value: Log240Call) -> Self {
            Self::Log240(value)
        }
    }
    impl ::core::convert::From<Log241Call> for HardhatConsoleCalls {
        fn from(value: Log241Call) -> Self {
            Self::Log241(value)
        }
    }
    impl ::core::convert::From<Log17Call> for HardhatConsoleCalls {
        fn from(value: Log17Call) -> Self {
            Self::Log17(value)
        }
    }
    impl ::core::convert::From<Log242Call> for HardhatConsoleCalls {
        fn from(value: Log242Call) -> Self {
            Self::Log242(value)
        }
    }
    impl ::core::convert::From<Log243Call> for HardhatConsoleCalls {
        fn from(value: Log243Call) -> Self {
            Self::Log243(value)
        }
    }
    impl ::core::convert::From<Log244Call> for HardhatConsoleCalls {
        fn from(value: Log244Call) -> Self {
            Self::Log244(value)
        }
    }
    impl ::core::convert::From<Log245Call> for HardhatConsoleCalls {
        fn from(value: Log245Call) -> Self {
            Self::Log245(value)
        }
    }
    impl ::core::convert::From<Log246Call> for HardhatConsoleCalls {
        fn from(value: Log246Call) -> Self {
            Self::Log246(value)
        }
    }
    impl ::core::convert::From<Log57Call> for HardhatConsoleCalls {
        fn from(value: Log57Call) -> Self {
            Self::Log57(value)
        }
    }
    impl ::core::convert::From<Log247Call> for HardhatConsoleCalls {
        fn from(value: Log247Call) -> Self {
            Self::Log247(value)
        }
    }
    impl ::core::convert::From<Log248Call> for HardhatConsoleCalls {
        fn from(value: Log248Call) -> Self {
            Self::Log248(value)
        }
    }
    impl ::core::convert::From<Log249Call> for HardhatConsoleCalls {
        fn from(value: Log249Call) -> Self {
            Self::Log249(value)
        }
    }
    impl ::core::convert::From<Log58Call> for HardhatConsoleCalls {
        fn from(value: Log58Call) -> Self {
            Self::Log58(value)
        }
    }
    impl ::core::convert::From<Log59Call> for HardhatConsoleCalls {
        fn from(value: Log59Call) -> Self {
            Self::Log59(value)
        }
    }
    impl ::core::convert::From<Log250Call> for HardhatConsoleCalls {
        fn from(value: Log250Call) -> Self {
            Self::Log250(value)
        }
    }
    impl ::core::convert::From<Log251Call> for HardhatConsoleCalls {
        fn from(value: Log251Call) -> Self {
            Self::Log251(value)
        }
    }
    impl ::core::convert::From<Log252Call> for HardhatConsoleCalls {
        fn from(value: Log252Call) -> Self {
            Self::Log252(value)
        }
    }
    impl ::core::convert::From<Log253Call> for HardhatConsoleCalls {
        fn from(value: Log253Call) -> Self {
            Self::Log253(value)
        }
    }
    impl ::core::convert::From<Log60Call> for HardhatConsoleCalls {
        fn from(value: Log60Call) -> Self {
            Self::Log60(value)
        }
    }
    impl ::core::convert::From<Log254Call> for HardhatConsoleCalls {
        fn from(value: Log254Call) -> Self {
            Self::Log254(value)
        }
    }
    impl ::core::convert::From<Log61Call> for HardhatConsoleCalls {
        fn from(value: Log61Call) -> Self {
            Self::Log61(value)
        }
    }
    impl ::core::convert::From<Log255Call> for HardhatConsoleCalls {
        fn from(value: Log255Call) -> Self {
            Self::Log255(value)
        }
    }
    impl ::core::convert::From<Log256Call> for HardhatConsoleCalls {
        fn from(value: Log256Call) -> Self {
            Self::Log256(value)
        }
    }
    impl ::core::convert::From<Log257Call> for HardhatConsoleCalls {
        fn from(value: Log257Call) -> Self {
            Self::Log257(value)
        }
    }
    impl ::core::convert::From<Log258Call> for HardhatConsoleCalls {
        fn from(value: Log258Call) -> Self {
            Self::Log258(value)
        }
    }
    impl ::core::convert::From<Log259Call> for HardhatConsoleCalls {
        fn from(value: Log259Call) -> Self {
            Self::Log259(value)
        }
    }
    impl ::core::convert::From<Log260Call> for HardhatConsoleCalls {
        fn from(value: Log260Call) -> Self {
            Self::Log260(value)
        }
    }
    impl ::core::convert::From<Log261Call> for HardhatConsoleCalls {
        fn from(value: Log261Call) -> Self {
            Self::Log261(value)
        }
    }
    impl ::core::convert::From<Log262Call> for HardhatConsoleCalls {
        fn from(value: Log262Call) -> Self {
            Self::Log262(value)
        }
    }
    impl ::core::convert::From<Log62Call> for HardhatConsoleCalls {
        fn from(value: Log62Call) -> Self {
            Self::Log62(value)
        }
    }
    impl ::core::convert::From<Log263Call> for HardhatConsoleCalls {
        fn from(value: Log263Call) -> Self {
            Self::Log263(value)
        }
    }
    impl ::core::convert::From<Log264Call> for HardhatConsoleCalls {
        fn from(value: Log264Call) -> Self {
            Self::Log264(value)
        }
    }
    impl ::core::convert::From<Log265Call> for HardhatConsoleCalls {
        fn from(value: Log265Call) -> Self {
            Self::Log265(value)
        }
    }
    impl ::core::convert::From<Log266Call> for HardhatConsoleCalls {
        fn from(value: Log266Call) -> Self {
            Self::Log266(value)
        }
    }
    impl ::core::convert::From<Log267Call> for HardhatConsoleCalls {
        fn from(value: Log267Call) -> Self {
            Self::Log267(value)
        }
    }
    impl ::core::convert::From<Log268Call> for HardhatConsoleCalls {
        fn from(value: Log268Call) -> Self {
            Self::Log268(value)
        }
    }
    impl ::core::convert::From<Log269Call> for HardhatConsoleCalls {
        fn from(value: Log269Call) -> Self {
            Self::Log269(value)
        }
    }
    impl ::core::convert::From<Log270Call> for HardhatConsoleCalls {
        fn from(value: Log270Call) -> Self {
            Self::Log270(value)
        }
    }
    impl ::core::convert::From<Log271Call> for HardhatConsoleCalls {
        fn from(value: Log271Call) -> Self {
            Self::Log271(value)
        }
    }
    impl ::core::convert::From<Log272Call> for HardhatConsoleCalls {
        fn from(value: Log272Call) -> Self {
            Self::Log272(value)
        }
    }
    impl ::core::convert::From<Log273Call> for HardhatConsoleCalls {
        fn from(value: Log273Call) -> Self {
            Self::Log273(value)
        }
    }
    impl ::core::convert::From<Log274Call> for HardhatConsoleCalls {
        fn from(value: Log274Call) -> Self {
            Self::Log274(value)
        }
    }
    impl ::core::convert::From<Log275Call> for HardhatConsoleCalls {
        fn from(value: Log275Call) -> Self {
            Self::Log275(value)
        }
    }
    impl ::core::convert::From<Log276Call> for HardhatConsoleCalls {
        fn from(value: Log276Call) -> Self {
            Self::Log276(value)
        }
    }
    impl ::core::convert::From<Log277Call> for HardhatConsoleCalls {
        fn from(value: Log277Call) -> Self {
            Self::Log277(value)
        }
    }
    impl ::core::convert::From<Log63Call> for HardhatConsoleCalls {
        fn from(value: Log63Call) -> Self {
            Self::Log63(value)
        }
    }
    impl ::core::convert::From<Log64Call> for HardhatConsoleCalls {
        fn from(value: Log64Call) -> Self {
            Self::Log64(value)
        }
    }
    impl ::core::convert::From<Log65Call> for HardhatConsoleCalls {
        fn from(value: Log65Call) -> Self {
            Self::Log65(value)
        }
    }
    impl ::core::convert::From<Log278Call> for HardhatConsoleCalls {
        fn from(value: Log278Call) -> Self {
            Self::Log278(value)
        }
    }
    impl ::core::convert::From<Log279Call> for HardhatConsoleCalls {
        fn from(value: Log279Call) -> Self {
            Self::Log279(value)
        }
    }
    impl ::core::convert::From<Log280Call> for HardhatConsoleCalls {
        fn from(value: Log280Call) -> Self {
            Self::Log280(value)
        }
    }
    impl ::core::convert::From<Log18Call> for HardhatConsoleCalls {
        fn from(value: Log18Call) -> Self {
            Self::Log18(value)
        }
    }
    impl ::core::convert::From<Log66Call> for HardhatConsoleCalls {
        fn from(value: Log66Call) -> Self {
            Self::Log66(value)
        }
    }
    impl ::core::convert::From<Log281Call> for HardhatConsoleCalls {
        fn from(value: Log281Call) -> Self {
            Self::Log281(value)
        }
    }
    impl ::core::convert::From<Log282Call> for HardhatConsoleCalls {
        fn from(value: Log282Call) -> Self {
            Self::Log282(value)
        }
    }
    impl ::core::convert::From<Log283Call> for HardhatConsoleCalls {
        fn from(value: Log283Call) -> Self {
            Self::Log283(value)
        }
    }
    impl ::core::convert::From<Log284Call> for HardhatConsoleCalls {
        fn from(value: Log284Call) -> Self {
            Self::Log284(value)
        }
    }
    impl ::core::convert::From<Log285Call> for HardhatConsoleCalls {
        fn from(value: Log285Call) -> Self {
            Self::Log285(value)
        }
    }
    impl ::core::convert::From<Log67Call> for HardhatConsoleCalls {
        fn from(value: Log67Call) -> Self {
            Self::Log67(value)
        }
    }
    impl ::core::convert::From<Log286Call> for HardhatConsoleCalls {
        fn from(value: Log286Call) -> Self {
            Self::Log286(value)
        }
    }
    impl ::core::convert::From<Log287Call> for HardhatConsoleCalls {
        fn from(value: Log287Call) -> Self {
            Self::Log287(value)
        }
    }
    impl ::core::convert::From<Log288Call> for HardhatConsoleCalls {
        fn from(value: Log288Call) -> Self {
            Self::Log288(value)
        }
    }
    impl ::core::convert::From<Log289Call> for HardhatConsoleCalls {
        fn from(value: Log289Call) -> Self {
            Self::Log289(value)
        }
    }
    impl ::core::convert::From<Log290Call> for HardhatConsoleCalls {
        fn from(value: Log290Call) -> Self {
            Self::Log290(value)
        }
    }
    impl ::core::convert::From<Log291Call> for HardhatConsoleCalls {
        fn from(value: Log291Call) -> Self {
            Self::Log291(value)
        }
    }
    impl ::core::convert::From<Log292Call> for HardhatConsoleCalls {
        fn from(value: Log292Call) -> Self {
            Self::Log292(value)
        }
    }
    impl ::core::convert::From<Log19Call> for HardhatConsoleCalls {
        fn from(value: Log19Call) -> Self {
            Self::Log19(value)
        }
    }
    impl ::core::convert::From<Log68Call> for HardhatConsoleCalls {
        fn from(value: Log68Call) -> Self {
            Self::Log68(value)
        }
    }
    impl ::core::convert::From<Log293Call> for HardhatConsoleCalls {
        fn from(value: Log293Call) -> Self {
            Self::Log293(value)
        }
    }
    impl ::core::convert::From<Log294Call> for HardhatConsoleCalls {
        fn from(value: Log294Call) -> Self {
            Self::Log294(value)
        }
    }
    impl ::core::convert::From<Log295Call> for HardhatConsoleCalls {
        fn from(value: Log295Call) -> Self {
            Self::Log295(value)
        }
    }
    impl ::core::convert::From<Log296Call> for HardhatConsoleCalls {
        fn from(value: Log296Call) -> Self {
            Self::Log296(value)
        }
    }
    impl ::core::convert::From<Log297Call> for HardhatConsoleCalls {
        fn from(value: Log297Call) -> Self {
            Self::Log297(value)
        }
    }
    impl ::core::convert::From<Log69Call> for HardhatConsoleCalls {
        fn from(value: Log69Call) -> Self {
            Self::Log69(value)
        }
    }
    impl ::core::convert::From<Log70Call> for HardhatConsoleCalls {
        fn from(value: Log70Call) -> Self {
            Self::Log70(value)
        }
    }
    impl ::core::convert::From<Log71Call> for HardhatConsoleCalls {
        fn from(value: Log71Call) -> Self {
            Self::Log71(value)
        }
    }
    impl ::core::convert::From<Log72Call> for HardhatConsoleCalls {
        fn from(value: Log72Call) -> Self {
            Self::Log72(value)
        }
    }
    impl ::core::convert::From<Log298Call> for HardhatConsoleCalls {
        fn from(value: Log298Call) -> Self {
            Self::Log298(value)
        }
    }
    impl ::core::convert::From<Log299Call> for HardhatConsoleCalls {
        fn from(value: Log299Call) -> Self {
            Self::Log299(value)
        }
    }
    impl ::core::convert::From<Log300Call> for HardhatConsoleCalls {
        fn from(value: Log300Call) -> Self {
            Self::Log300(value)
        }
    }
    impl ::core::convert::From<Log301Call> for HardhatConsoleCalls {
        fn from(value: Log301Call) -> Self {
            Self::Log301(value)
        }
    }
    impl ::core::convert::From<Log302Call> for HardhatConsoleCalls {
        fn from(value: Log302Call) -> Self {
            Self::Log302(value)
        }
    }
    impl ::core::convert::From<Log73Call> for HardhatConsoleCalls {
        fn from(value: Log73Call) -> Self {
            Self::Log73(value)
        }
    }
    impl ::core::convert::From<Log303Call> for HardhatConsoleCalls {
        fn from(value: Log303Call) -> Self {
            Self::Log303(value)
        }
    }
    impl ::core::convert::From<Log304Call> for HardhatConsoleCalls {
        fn from(value: Log304Call) -> Self {
            Self::Log304(value)
        }
    }
    impl ::core::convert::From<Log74Call> for HardhatConsoleCalls {
        fn from(value: Log74Call) -> Self {
            Self::Log74(value)
        }
    }
    impl ::core::convert::From<Log75Call> for HardhatConsoleCalls {
        fn from(value: Log75Call) -> Self {
            Self::Log75(value)
        }
    }
    impl ::core::convert::From<Log305Call> for HardhatConsoleCalls {
        fn from(value: Log305Call) -> Self {
            Self::Log305(value)
        }
    }
    impl ::core::convert::From<Log306Call> for HardhatConsoleCalls {
        fn from(value: Log306Call) -> Self {
            Self::Log306(value)
        }
    }
    impl ::core::convert::From<Log307Call> for HardhatConsoleCalls {
        fn from(value: Log307Call) -> Self {
            Self::Log307(value)
        }
    }
    impl ::core::convert::From<Log308Call> for HardhatConsoleCalls {
        fn from(value: Log308Call) -> Self {
            Self::Log308(value)
        }
    }
    impl ::core::convert::From<Log309Call> for HardhatConsoleCalls {
        fn from(value: Log309Call) -> Self {
            Self::Log309(value)
        }
    }
    impl ::core::convert::From<Log20Call> for HardhatConsoleCalls {
        fn from(value: Log20Call) -> Self {
            Self::Log20(value)
        }
    }
    impl ::core::convert::From<Log76Call> for HardhatConsoleCalls {
        fn from(value: Log76Call) -> Self {
            Self::Log76(value)
        }
    }
    impl ::core::convert::From<Log310Call> for HardhatConsoleCalls {
        fn from(value: Log310Call) -> Self {
            Self::Log310(value)
        }
    }
    impl ::core::convert::From<Log311Call> for HardhatConsoleCalls {
        fn from(value: Log311Call) -> Self {
            Self::Log311(value)
        }
    }
    impl ::core::convert::From<Log312Call> for HardhatConsoleCalls {
        fn from(value: Log312Call) -> Self {
            Self::Log312(value)
        }
    }
    impl ::core::convert::From<Log313Call> for HardhatConsoleCalls {
        fn from(value: Log313Call) -> Self {
            Self::Log313(value)
        }
    }
    impl ::core::convert::From<Log314Call> for HardhatConsoleCalls {
        fn from(value: Log314Call) -> Self {
            Self::Log314(value)
        }
    }
    impl ::core::convert::From<Log77Call> for HardhatConsoleCalls {
        fn from(value: Log77Call) -> Self {
            Self::Log77(value)
        }
    }
    impl ::core::convert::From<Log315Call> for HardhatConsoleCalls {
        fn from(value: Log315Call) -> Self {
            Self::Log315(value)
        }
    }
    impl ::core::convert::From<Log316Call> for HardhatConsoleCalls {
        fn from(value: Log316Call) -> Self {
            Self::Log316(value)
        }
    }
    impl ::core::convert::From<Log317Call> for HardhatConsoleCalls {
        fn from(value: Log317Call) -> Self {
            Self::Log317(value)
        }
    }
    impl ::core::convert::From<Log78Call> for HardhatConsoleCalls {
        fn from(value: Log78Call) -> Self {
            Self::Log78(value)
        }
    }
    impl ::core::convert::From<Log318Call> for HardhatConsoleCalls {
        fn from(value: Log318Call) -> Self {
            Self::Log318(value)
        }
    }
    impl ::core::convert::From<Log79Call> for HardhatConsoleCalls {
        fn from(value: Log79Call) -> Self {
            Self::Log79(value)
        }
    }
    impl ::core::convert::From<Log319Call> for HardhatConsoleCalls {
        fn from(value: Log319Call) -> Self {
            Self::Log319(value)
        }
    }
    impl ::core::convert::From<Log320Call> for HardhatConsoleCalls {
        fn from(value: Log320Call) -> Self {
            Self::Log320(value)
        }
    }
    impl ::core::convert::From<Log321Call> for HardhatConsoleCalls {
        fn from(value: Log321Call) -> Self {
            Self::Log321(value)
        }
    }
    impl ::core::convert::From<Log322Call> for HardhatConsoleCalls {
        fn from(value: Log322Call) -> Self {
            Self::Log322(value)
        }
    }
    impl ::core::convert::From<Log323Call> for HardhatConsoleCalls {
        fn from(value: Log323Call) -> Self {
            Self::Log323(value)
        }
    }
    impl ::core::convert::From<Log324Call> for HardhatConsoleCalls {
        fn from(value: Log324Call) -> Self {
            Self::Log324(value)
        }
    }
    impl ::core::convert::From<Log80Call> for HardhatConsoleCalls {
        fn from(value: Log80Call) -> Self {
            Self::Log80(value)
        }
    }
    impl ::core::convert::From<Log325Call> for HardhatConsoleCalls {
        fn from(value: Log325Call) -> Self {
            Self::Log325(value)
        }
    }
    impl ::core::convert::From<Log326Call> for HardhatConsoleCalls {
        fn from(value: Log326Call) -> Self {
            Self::Log326(value)
        }
    }
    impl ::core::convert::From<Log81Call> for HardhatConsoleCalls {
        fn from(value: Log81Call) -> Self {
            Self::Log81(value)
        }
    }
    impl ::core::convert::From<Log327Call> for HardhatConsoleCalls {
        fn from(value: Log327Call) -> Self {
            Self::Log327(value)
        }
    }
    impl ::core::convert::From<Log328Call> for HardhatConsoleCalls {
        fn from(value: Log328Call) -> Self {
            Self::Log328(value)
        }
    }
    impl ::core::convert::From<Log329Call> for HardhatConsoleCalls {
        fn from(value: Log329Call) -> Self {
            Self::Log329(value)
        }
    }
    impl ::core::convert::From<Log330Call> for HardhatConsoleCalls {
        fn from(value: Log330Call) -> Self {
            Self::Log330(value)
        }
    }
    impl ::core::convert::From<Log331Call> for HardhatConsoleCalls {
        fn from(value: Log331Call) -> Self {
            Self::Log331(value)
        }
    }
    impl ::core::convert::From<Log82Call> for HardhatConsoleCalls {
        fn from(value: Log82Call) -> Self {
            Self::Log82(value)
        }
    }
    impl ::core::convert::From<Log83Call> for HardhatConsoleCalls {
        fn from(value: Log83Call) -> Self {
            Self::Log83(value)
        }
    }
    impl ::core::convert::From<Log84Call> for HardhatConsoleCalls {
        fn from(value: Log84Call) -> Self {
            Self::Log84(value)
        }
    }
    impl ::core::convert::From<Log332Call> for HardhatConsoleCalls {
        fn from(value: Log332Call) -> Self {
            Self::Log332(value)
        }
    }
    impl ::core::convert::From<Log333Call> for HardhatConsoleCalls {
        fn from(value: Log333Call) -> Self {
            Self::Log333(value)
        }
    }
    impl ::core::convert::From<Log334Call> for HardhatConsoleCalls {
        fn from(value: Log334Call) -> Self {
            Self::Log334(value)
        }
    }
    impl ::core::convert::From<Log21Call> for HardhatConsoleCalls {
        fn from(value: Log21Call) -> Self {
            Self::Log21(value)
        }
    }
    impl ::core::convert::From<Log335Call> for HardhatConsoleCalls {
        fn from(value: Log335Call) -> Self {
            Self::Log335(value)
        }
    }
    impl ::core::convert::From<Log336Call> for HardhatConsoleCalls {
        fn from(value: Log336Call) -> Self {
            Self::Log336(value)
        }
    }
    impl ::core::convert::From<Log4Call> for HardhatConsoleCalls {
        fn from(value: Log4Call) -> Self {
            Self::Log4(value)
        }
    }
    impl ::core::convert::From<Log337Call> for HardhatConsoleCalls {
        fn from(value: Log337Call) -> Self {
            Self::Log337(value)
        }
    }
    impl ::core::convert::From<Log338Call> for HardhatConsoleCalls {
        fn from(value: Log338Call) -> Self {
            Self::Log338(value)
        }
    }
    impl ::core::convert::From<Log339Call> for HardhatConsoleCalls {
        fn from(value: Log339Call) -> Self {
            Self::Log339(value)
        }
    }
    impl ::core::convert::From<Log85Call> for HardhatConsoleCalls {
        fn from(value: Log85Call) -> Self {
            Self::Log85(value)
        }
    }
    impl ::core::convert::From<Log340Call> for HardhatConsoleCalls {
        fn from(value: Log340Call) -> Self {
            Self::Log340(value)
        }
    }
    impl ::core::convert::From<Log86Call> for HardhatConsoleCalls {
        fn from(value: Log86Call) -> Self {
            Self::Log86(value)
        }
    }
    impl ::core::convert::From<Log341Call> for HardhatConsoleCalls {
        fn from(value: Log341Call) -> Self {
            Self::Log341(value)
        }
    }
    impl ::core::convert::From<Log342Call> for HardhatConsoleCalls {
        fn from(value: Log342Call) -> Self {
            Self::Log342(value)
        }
    }
    impl ::core::convert::From<Log5Call> for HardhatConsoleCalls {
        fn from(value: Log5Call) -> Self {
            Self::Log5(value)
        }
    }
    impl ::core::convert::From<Log22Call> for HardhatConsoleCalls {
        fn from(value: Log22Call) -> Self {
            Self::Log22(value)
        }
    }
    impl ::core::convert::From<LogAddressCall> for HardhatConsoleCalls {
        fn from(value: LogAddressCall) -> Self {
            Self::LogAddress(value)
        }
    }
    impl ::core::convert::From<LogBoolCall> for HardhatConsoleCalls {
        fn from(value: LogBoolCall) -> Self {
            Self::LogBool(value)
        }
    }
    impl ::core::convert::From<LogBytesCall> for HardhatConsoleCalls {
        fn from(value: LogBytesCall) -> Self {
            Self::LogBytes(value)
        }
    }
    impl ::core::convert::From<LogBytes1Call> for HardhatConsoleCalls {
        fn from(value: LogBytes1Call) -> Self {
            Self::LogBytes1(value)
        }
    }
    impl ::core::convert::From<LogBytes10Call> for HardhatConsoleCalls {
        fn from(value: LogBytes10Call) -> Self {
            Self::LogBytes10(value)
        }
    }
    impl ::core::convert::From<LogBytes11Call> for HardhatConsoleCalls {
        fn from(value: LogBytes11Call) -> Self {
            Self::LogBytes11(value)
        }
    }
    impl ::core::convert::From<LogBytes12Call> for HardhatConsoleCalls {
        fn from(value: LogBytes12Call) -> Self {
            Self::LogBytes12(value)
        }
    }
    impl ::core::convert::From<LogBytes13Call> for HardhatConsoleCalls {
        fn from(value: LogBytes13Call) -> Self {
            Self::LogBytes13(value)
        }
    }
    impl ::core::convert::From<LogBytes14Call> for HardhatConsoleCalls {
        fn from(value: LogBytes14Call) -> Self {
            Self::LogBytes14(value)
        }
    }
    impl ::core::convert::From<LogBytes15Call> for HardhatConsoleCalls {
        fn from(value: LogBytes15Call) -> Self {
            Self::LogBytes15(value)
        }
    }
    impl ::core::convert::From<LogBytes16Call> for HardhatConsoleCalls {
        fn from(value: LogBytes16Call) -> Self {
            Self::LogBytes16(value)
        }
    }
    impl ::core::convert::From<LogBytes17Call> for HardhatConsoleCalls {
        fn from(value: LogBytes17Call) -> Self {
            Self::LogBytes17(value)
        }
    }
    impl ::core::convert::From<LogBytes18Call> for HardhatConsoleCalls {
        fn from(value: LogBytes18Call) -> Self {
            Self::LogBytes18(value)
        }
    }
    impl ::core::convert::From<LogBytes19Call> for HardhatConsoleCalls {
        fn from(value: LogBytes19Call) -> Self {
            Self::LogBytes19(value)
        }
    }
    impl ::core::convert::From<LogBytes2Call> for HardhatConsoleCalls {
        fn from(value: LogBytes2Call) -> Self {
            Self::LogBytes2(value)
        }
    }
    impl ::core::convert::From<LogBytes20Call> for HardhatConsoleCalls {
        fn from(value: LogBytes20Call) -> Self {
            Self::LogBytes20(value)
        }
    }
    impl ::core::convert::From<LogBytes21Call> for HardhatConsoleCalls {
        fn from(value: LogBytes21Call) -> Self {
            Self::LogBytes21(value)
        }
    }
    impl ::core::convert::From<LogBytes22Call> for HardhatConsoleCalls {
        fn from(value: LogBytes22Call) -> Self {
            Self::LogBytes22(value)
        }
    }
    impl ::core::convert::From<LogBytes23Call> for HardhatConsoleCalls {
        fn from(value: LogBytes23Call) -> Self {
            Self::LogBytes23(value)
        }
    }
    impl ::core::convert::From<LogBytes24Call> for HardhatConsoleCalls {
        fn from(value: LogBytes24Call) -> Self {
            Self::LogBytes24(value)
        }
    }
    impl ::core::convert::From<LogBytes25Call> for HardhatConsoleCalls {
        fn from(value: LogBytes25Call) -> Self {
            Self::LogBytes25(value)
        }
    }
    impl ::core::convert::From<LogBytes26Call> for HardhatConsoleCalls {
        fn from(value: LogBytes26Call) -> Self {
            Self::LogBytes26(value)
        }
    }
    impl ::core::convert::From<LogBytes27Call> for HardhatConsoleCalls {
        fn from(value: LogBytes27Call) -> Self {
            Self::LogBytes27(value)
        }
    }
    impl ::core::convert::From<LogBytes28Call> for HardhatConsoleCalls {
        fn from(value: LogBytes28Call) -> Self {
            Self::LogBytes28(value)
        }
    }
    impl ::core::convert::From<LogBytes29Call> for HardhatConsoleCalls {
        fn from(value: LogBytes29Call) -> Self {
            Self::LogBytes29(value)
        }
    }
    impl ::core::convert::From<LogBytes3Call> for HardhatConsoleCalls {
        fn from(value: LogBytes3Call) -> Self {
            Self::LogBytes3(value)
        }
    }
    impl ::core::convert::From<LogBytes30Call> for HardhatConsoleCalls {
        fn from(value: LogBytes30Call) -> Self {
            Self::LogBytes30(value)
        }
    }
    impl ::core::convert::From<LogBytes31Call> for HardhatConsoleCalls {
        fn from(value: LogBytes31Call) -> Self {
            Self::LogBytes31(value)
        }
    }
    impl ::core::convert::From<LogBytes32Call> for HardhatConsoleCalls {
        fn from(value: LogBytes32Call) -> Self {
            Self::LogBytes32(value)
        }
    }
    impl ::core::convert::From<LogBytes4Call> for HardhatConsoleCalls {
        fn from(value: LogBytes4Call) -> Self {
            Self::LogBytes4(value)
        }
    }
    impl ::core::convert::From<LogBytes5Call> for HardhatConsoleCalls {
        fn from(value: LogBytes5Call) -> Self {
            Self::LogBytes5(value)
        }
    }
    impl ::core::convert::From<LogBytes6Call> for HardhatConsoleCalls {
        fn from(value: LogBytes6Call) -> Self {
            Self::LogBytes6(value)
        }
    }
    impl ::core::convert::From<LogBytes7Call> for HardhatConsoleCalls {
        fn from(value: LogBytes7Call) -> Self {
            Self::LogBytes7(value)
        }
    }
    impl ::core::convert::From<LogBytes8Call> for HardhatConsoleCalls {
        fn from(value: LogBytes8Call) -> Self {
            Self::LogBytes8(value)
        }
    }
    impl ::core::convert::From<LogBytes9Call> for HardhatConsoleCalls {
        fn from(value: LogBytes9Call) -> Self {
            Self::LogBytes9(value)
        }
    }
    impl ::core::convert::From<LogIntCall> for HardhatConsoleCalls {
        fn from(value: LogIntCall) -> Self {
            Self::LogInt(value)
        }
    }
    impl ::core::convert::From<LogStringCall> for HardhatConsoleCalls {
        fn from(value: LogStringCall) -> Self {
            Self::LogString(value)
        }
    }
    impl ::core::convert::From<LogUintCall> for HardhatConsoleCalls {
        fn from(value: LogUintCall) -> Self {
            Self::LogUint(value)
        }
    }
}
