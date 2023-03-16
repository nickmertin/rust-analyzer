use std::{cell::UnsafeCell, ops::Deref};

use replace_with::replace_with;

pub struct LazyCell<T, F: FnOnce() -> T> {
    cell: UnsafeCell<LazyCellState<T, F>>,
}

enum LazyCellState<T, F: FnOnce() -> T> {
    Initialized(T),
    Uninitialized(F),
    Poisoned,
}

impl<T, F: FnOnce() -> T> LazyCellState<T, F> {
    pub fn is_uninitialized(&self) -> bool {
        if let Self::Uninitialized(_) = self {
            true
        } else {
            false
        }
    }
}

impl<T, F: FnOnce() -> T> LazyCell<T, F> {
    pub fn new(init: F) -> Self {
        Self { cell: UnsafeCell::new(LazyCellState::Uninitialized(init)) }
    }
}

impl<T, F: FnOnce() -> T> Deref for LazyCell<T, F> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // if let Err(_) = self.locked.compare_exchange(
        //     false,
        //     true,
        //     std::sync::atomic::Ordering::AcqRel,
        //     std::sync::atomic::Ordering::Acquire,
        // ) {
        //     panic!("Cell already in use!");
        // }

        let inner = unsafe { &mut *self.cell.get() };
        if inner.is_uninitialized() {
            replace_with(
                inner,
                || LazyCellState::Poisoned,
                |state| {
                    if let LazyCellState::Uninitialized(init) = state {
                        LazyCellState::Initialized(init())
                    } else {
                        unreachable!()
                    }
                },
            )
        }
        match inner {
            LazyCellState::Initialized(x) => return x,
            LazyCellState::Uninitialized(_) => unreachable!(),
            LazyCellState::Poisoned => panic!("poisoned LazyCell"),
        }
    }
}
