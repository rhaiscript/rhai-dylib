/// Libloading module resolver
#[cfg(feature = "libloading")]
pub mod libloading;

// NOTE: Both of the following functions are rhai's internal and can't be accessed.
// TODO: Ask to make this API public.

/// Read-only lock guard for synchronized shared object.
#[cfg(not(feature = "sync"))]
pub type LockGuard<'a, T> = std::cell::Ref<'a, T>;

/// Mutable lock guard for synchronized shared object.
#[cfg(not(feature = "sync"))]
pub type LockGuardMut<'a, T> = std::cell::RefMut<'a, T>;

/// Read-only lock guard for synchronized shared object.
#[cfg(feature = "sync")]
#[allow(dead_code)]
pub type LockGuard<'a, T> = std::sync::RwLockReadGuard<'a, T>;

/// Mutable lock guard for synchronized shared object.
#[cfg(feature = "sync")]
#[allow(dead_code)]
pub type LockGuardMut<'a, T> = std::sync::RwLockWriteGuard<'a, T>;

/// Lock a [`Locked`] resource for mutable access.
///
/// # Panics
///
/// This function will return an error if the `RwLock` is poisoned.
#[allow(dead_code)]
pub fn locked_write<T>(value: &'_ rhai::Locked<T>) -> LockGuardMut<'_, T> {
    #[cfg(not(feature = "sync"))]
    return value.borrow_mut();

    #[cfg(feature = "sync")]
    return value.write().unwrap();
}

/// Lock a [`Locked`] resource for mutable access.
///
/// # Panics
///
/// This function will return an error if the `RwLock` is poisoned.
#[allow(dead_code)]
pub fn locked_read<T>(value: &'_ rhai::Locked<T>) -> LockGuard<'_, T> {
    #[cfg(not(feature = "sync"))]
    return value.borrow();

    #[cfg(feature = "sync")]
    return value.read().unwrap();
}
