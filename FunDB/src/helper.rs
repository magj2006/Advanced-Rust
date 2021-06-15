//!
//! # Common Types and Macros
//!

use lazy_static::lazy_static;
use ruc::*;
use serde::{de::DeserializeOwned, Serialize};
use std::{borrow::Cow, cmp::Ordering, convert::TryInto, env, fmt, fs, mem, ops::Deref};

// define a cache directory
lazy_static! {
    /// read from env, set /tmp if not exist
    pub static ref CACHE_DIR: String = env::var("FUNDB_DIR").unwrap_or_else(|_| "/tmp".to_owned());
}

/// try print and panic twice
#[macro_export]
macro_rules! try_twice {
    // `expr` is an expression
    ($ops: expr) => {
        // `expr` is an expression
        ruc::pnk!($ops.c(d!()).or_else(|e| {
            e.print();
            $ops.c(d!())
        }))
    };
}

/// a macro for generating unique path
#[macro_export]
macro_rules! unique_path {
    // return an unique path
    () => {
        format!(
            "{}/.fundb/{}/{}_{}_{}_{}",
            *$crate::helper::CACHE_DIR,
            ts!(),
            file!(),
            line!(),
            column!(),
            rand::random::<u32>()
        )
    };
}

/// a macro for create instance of Vecx
#[macro_export]
macro_rules! new_vecx {
    // input parametor requires data type and in_mem_cnt
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($ty, $in_mem_cnt, false)
    };
    // input parametor requires data type
    ($ty: ty) => {
        $crate::new_vecx_custom!($ty, None, false)
    };
    // input parametor requires in_mem_cnt
    ($in_mem_cnt: expr) => {
        $crate::new_vecx_custom!($in_mem_cnt, false)
    };
    // no input parametor
    () => {
        $crate::new_vecx_custom!(false)
    };
}

/// a macro for create custom instance of Vecx
#[macro_export]
macro_rules! new_vecx_custom {
    // create instance of Vecx by data type and in_mem_cnt and is_tmp
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> = $crate::try_twice!($crate::Vecx::new(
            $crate::unique_path!(),
            Some($in_mem_cnt)
            $is_tmp,
        ));
        obj
    }};
    // create instance of Vecx by data type and is_tmp
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Vecx<$ty> =
            $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp));
        obj
    }};
    // create instance of Vecx by in_mem_cnt and is_tmp
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), Some($in_mem_cnt), $is_tmp))
    };
    // create instance of Vecx by is_tmp
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Vecx::new($crate::unique_path!(), None, $is_tmp))
    };
}

/// a macro for create instance of Mapx
#[macro_export]
macro_rules! new_mapx {
    // input parametor requires data type and in_mem_cnt
    ($ty: ty, $in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($ty, $in_mem_cnt, false)
    };
    // input parametor requires data type
    ($ty: ty) => {
        $crate::new_mapx_custom!($ty, None, false)
    };
    // input parametor requires in_mem_cnt
    ($in_mem_cnt: expr) => {
        $crate::new_mapx_custom!($in_mem_cnt, false)
    };
    // no input parametor
    () => {
        $crate::new_mapx_custom!(false)
    };
}

/// a macro for create custom instance of Mapx
#[macro_export]
macro_rules! new_mapx_custom {
    // create instance of Mapx by data type and in_mem_cnt and is_tmp
    ($ty: ty, $in_mem_cnt: expr, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> = $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp,
        ));
        obj
    }};
    // create instance of Mapx by data type and is_tmp
    ($ty: ty, $is_tmp: expr) => {{
        let obj: $crate::Mapx<$ty> =
            $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,));
        obj
    }};
    // create instance of Mapx by in_mem_cnt and is_tmp
    ($in_mem_cnt: expr, $is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new(
            $crate::unique_path!(),
            $in_mem_cnt,
            $is_tmp
        ))
    };
    // create instance of Mapx by is_tmp
    ($is_tmp: expr) => {
        $crate::try_twice!($crate::Mapx::new($crate::unique_path!(), None, $is_tmp,))
    };
}

////////////////////////////////////////////////////////////////////////////////
// Begin of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
/******************************************************************************/

/// Returned by `.get(...)`
#[derive(Eq, Debug, Clone)]
pub struct Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    value: Cow<'a, V>,
}

impl<'a, V> Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    pub(crate) fn new(value: Cow<'a, V>) -> Self {
        Value { value }
    }

    /// Comsume the ownship and get the inner value.
    pub fn into_inner(self) -> Cow<'a, V> {
        self.value
    }
}

impl<'a, V> Deref for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    type Target = V;

    fn deref(&self) -> &Self::Target {
        // todo!()
        self.value.deref()
    }
}

impl<'a, V> PartialEq for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &Value<'a, V>) -> bool {
        // todo!()
        self.value.eq(&other.value)
    }
}

impl<'a, V> PartialEq<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn eq(&self, other: &V) -> bool {
        // todo!()
        self.value.eq(&Cow::Borrowed(other))
    }
}

impl<'a, V> PartialOrd<V> for Value<'a, V>
where
    V: fmt::Debug + Clone + Eq + PartialEq + Ord + PartialOrd + Serialize + DeserializeOwned,
{
    fn partial_cmp(&self, other: &V) -> Option<Ordering> {
        // todo!()
        self.value.partial_cmp(&Cow::Borrowed(other))
    }
}

impl<'a, V> From<V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: V) -> Self {
        // todo!()
        Self {
            value: Cow::Owned(v),
        }
    }
}

impl<'a, V> From<Cow<'a, V>> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Cow<'a, V>) -> Self {
        // todo!()
        Self { value: v }
    }
}

impl<'a, V> From<Value<'a, V>> for Cow<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: Value<'a, V>) -> Self {
        // todo!()
        v.value
    }
}

impl<'a, V> From<&V> for Value<'a, V>
where
    V: Clone + Eq + PartialEq + Serialize + DeserializeOwned + fmt::Debug,
{
    fn from(v: &V) -> Self {
        // todo!()
        Value::new(Cow::Owned(v.clone()))
    }
}

/****************************************************************************/
// End of the implementation of Value(returned by `self.get`) for Vecx/Mapx //
//////////////////////////////////////////////////////////////////////////////

#[inline(always)]
pub(crate) fn sled_open(path: &str, is_tmp: bool) -> Result<sled::Db> {
    // todo!()
    pnk!(fs::DirBuilder::new().recursive(true).create(path));
    let config = sled::Config::default()
        .path(path.to_owned())
        .temporary(is_tmp);
    config
        .open()
        .c(d!(format!("Failed to open db on path: {}", path)))
}

#[inline(always)]
pub(crate) fn read_db_len(path: &str) -> Result<usize> {
    // todo!()
    Ok(fs::read(path)
        .c(d!("read file failed."))
        .map(|len| usize::from_le_bytes(len[..mem::size_of::<usize>()].try_into().unwrap()))?)
}

#[inline(always)]
pub(crate) fn write_db_len(path: &str, len: usize) -> Result<()> {
    // todo!()
    fs::write(path, &usize::to_le_bytes(len)[..]).c(d!("write file failed"))
}
