//! Implementation of a critical section based mutex that also implements the `mutex-trait`.
//!
//! ## Safety
//!
//! Note that this is only safe in single core applications.

use core::cell::UnsafeCell;

pub extern crate mutex_trait;
pub use mutex_trait::Mutex;

/// A spinlock and critical section section based mutex.
pub struct CriticalSectionSpinLockMutex<T> {
    data: spin::Mutex<T>,
}

impl<T> CriticalSectionSpinLockMutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        CriticalSectionSpinLockMutex {
            data: spin::Mutex::new(data),
        }
    }
}

impl<T> mutex_trait::Mutex for &'_ CriticalSectionSpinLockMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        crate::interrupt::free(|_| f(&mut (*self.data.lock())))
    }
}

// NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g. interrupts)
unsafe impl<T> Sync for CriticalSectionSpinLockMutex<T> where T: Send {}

/// A critical section based mutex.
pub struct CriticalSectionMutex<T> {
    data: UnsafeCell<T>,
}

impl<T> CriticalSectionMutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        CriticalSectionMutex {
            data: UnsafeCell::new(data),
        }
    }
}

impl<T> mutex_trait::Mutex for &'_ CriticalSectionMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        crate::interrupt::free(|_| f(unsafe { &mut *self.data.get() }))
    }
}

// NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g. interrupts)
unsafe impl<T> Sync for CriticalSectionMutex<T> where T: Send {}

/// A spinlock based mutex.
pub struct SpinLockMutex<T> {
    data: spin::Mutex<T>,
}

impl<T> SpinLockMutex<T> {
    /// Create a new mutex
    pub const fn new(data: T) -> Self {
        SpinLockMutex {
            data: spin::Mutex::new(data),
        }
    }
}

impl<T> mutex_trait::Mutex for &'_ SpinLockMutex<T> {
    type Data = T;

    fn lock<R>(&mut self, f: impl FnOnce(&mut Self::Data) -> R) -> R {
        f(&mut (*self.data.lock()))
    }
}

// NOTE A `Mutex` can be used as a channel so the protected data must be `Send`
// to prevent sending non-Sendable stuff (e.g. access tokens) across different
// execution contexts (e.g. interrupts)
unsafe impl<T> Sync for SpinLockMutex<T> where T: Send {}
