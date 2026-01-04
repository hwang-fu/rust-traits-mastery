//! Key Takeaways:
//! 1. Send = safe to move ownership to another thread
//! 2. Sync = safe to share references across threads
//! 3. T: Sync ⟺ &T: Send

use std::{
    cell::Cell,
    marker::PhantomData,
    rc::Rc,
    sync::{Arc, Mutex},
};

// ----------------------------
#[derive(Debug, Clone)]
pub struct ThreadSafeData {
    pub id: i32,
    pub name: String,
    pub payload: Vec<u8>,
}

impl ThreadSafeData {
    pub fn new(id: i32, name: impl Into<String>) -> Self {
        let name = name.into();
        let payload = Vec::new();
        ThreadSafeData { id, name, payload }
    }
}

// ----------------------------
#[derive(Debug)]
pub struct UnsyncCounter {
    value: Cell<i32>,
}

impl UnsyncCounter {
    pub fn new(initial: i32) -> Self {
        let value = Cell::new(initial);
        UnsyncCounter { value }
    }

    pub fn inc(&self) {
        self.value.set(self.value.get() + 1);
    }

    pub fn get(&self) -> i32 {
        self.value.get()
    }
}

// ----------------------------
#[derive(Debug, Clone)]
pub struct SharedLocal<T> {
    data: Rc<T>,
}

impl<T> SharedLocal<T> {
    pub fn new(value: T) -> Self {
        SharedLocal {
            data: Rc::new(value),
        }
    }

    pub fn get(&self) -> &T {
        &self.data
    }

    pub fn ref_count(&self) -> usize {
        Rc::strong_count(&self.data)
    }
}

// ----------------------------
#[derive(Debug)]
pub struct AtomicCounter {
    value: Arc<Mutex<i32>>,
}

impl AtomicCounter {
    pub fn new(initial: i32) -> Self {
        let value = Arc::new(Mutex::new(initial));
        AtomicCounter { value }
    }

    pub fn inc(&self) {
        let mut guard = self.value.lock().unwrap();
        *guard += 1;
        // `guard` is dropped here, releasing the lock
    }

    pub fn get(&self) -> i32 {
        let guard = self.value.lock().unwrap();
        *guard
    }

    pub fn share(&self) -> Arc<Mutex<i32>> {
        Arc::clone(&self.value)
    }
}

impl Clone for AtomicCounter {
    fn clone(&self) -> Self {
        let value = Arc::clone(&self.value);
        AtomicCounter { value }
    }
}

// ----------------------------
pub struct NotThreadSafe {
    pub data: i32,
    // PhantomData<*const ()> is neither Send nor Sync
    // This "infects" our struct with those properties
    _marker: PhantomData<*const ()>,
}

impl NotThreadSafe {
    pub fn new(data: impl Into<i32>) -> Self {
        NotThreadSafe {
            data: data.into(),
            _marker: PhantomData,
        }
    }
}

// ----------------------------
pub struct RawPtrWrapper {
    // Raw pointer - not Send or Sync by default
    ptr: *mut i32,
    // We own the data, so we're responsible for freeing it
    _owned: bool,
}

impl RawPtrWrapper {
    pub fn new(value: i32) -> Self {
        let boxed = Box::new(value);
        let ptr = Box::into_raw(boxed);
        RawPtrWrapper { ptr, _owned: true }
    }

    pub fn get(&self) -> i32 {
        // SAFETY: We own this pointer and it's always valid while self exists
        unsafe { *self.ptr }
    }

    pub fn set(&mut self, value: i32) {
        // SAFETY: We have &mut self, so exclusive access is guaranteed
        unsafe { *self.ptr = value }
    }
}

impl Drop for RawPtrWrapper {
    fn drop(&mut self) {
        // SAFETY: We created this pointer with Box::into_raw, so we must free it
        unsafe {
            drop(Box::from_raw(self.ptr));
        }
    }
}

// SAFETY: RawPtrWrapper can be sent to another thread because:
// 1. It owns the pointed-to data exclusively
// 2. The pointer remains valid until Drop
// 3. No other references to the data exist
unsafe impl Send for RawPtrWrapper {}

// SAFETY: RawPtrWrapper can be shared across threads because:
// 1. &self only allows reading via get()
// 2. The data is never mutated through &self
// 3. Reading an i32 is atomic on all platforms we care about
unsafe impl Sync for RawPtrWrapper {}

// ----------------------------
/// Helper function to assert a type is Send at compile time.
/// This function is never called - it only exists for the compiler check.
fn _assert_send<T: Send>() {}

/// Helper function to assert a type is Sync at compile time.
fn _assert_sync<T: Sync>() {}

/// Combined assertion for Send + Sync.
fn _assert_send_sync<T: Send + Sync>() {}

/// Helper to assert a type is Send but NOT Sync.
/// We can only assert Send here; we'll use a negative test for !Sync.
fn _assert_send_not_sync<T: Send>() {}

// These compile successfully because ThreadSafeData is Send + Sync
#[allow(dead_code)]
fn _static_assertions() {
    _assert_send::<ThreadSafeData>();
    _assert_sync::<ThreadSafeData>();
    _assert_send_sync::<ThreadSafeData>();

    // UnsyncCounter is Send but NOT Sync
    _assert_send::<UnsyncCounter>();
    // _assert_sync::<UnsyncCounter>();  // UNCOMMENT TO SEE COMPILE ERROR!

    // SharedLocal is NEITHER Send NOR Sync
    // _assert_send::<SharedLocal<i32>>();  // WOULD FAIL: Rc is not Send
    // _assert_sync::<SharedLocal<i32>>();  // WOULD FAIL: Rc is not Sync

    // AtomicCounter IS Send + Sync (Arc<Mutex<T>> where T: Send)
    _assert_send::<AtomicCounter>();
    _assert_sync::<AtomicCounter>();
    _assert_send_sync::<AtomicCounter>();

    // NotThreadSafe explicitly opts out of Send + Sync via PhantomData
    // _assert_send::<NotThreadSafe>();  // WOULD FAIL: *const () is not Send
    // _assert_sync::<NotThreadSafe>();  // WOULD FAIL: *const () is not Sync

    // RawPtrWrapper: manually implemented Send + Sync
    _assert_send::<RawPtrWrapper>();
    _assert_sync::<RawPtrWrapper>();
    _assert_send_sync::<RawPtrWrapper>();
}

// ----------------------------
#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_thread_safe_data_basic() {
        let data = ThreadSafeData::new(1, "test");
        assert_eq!(data.id, 1);
        assert_eq!(data.name, "test");
    }

    #[test]
    fn test_thread_safe_data_send() {
        // Demonstrate Send: move ownership to another thread
        let data = ThreadSafeData::new(42, "hello");

        let handle = thread::spawn(move || {
            // data was moved here - this works because ThreadSafeData: Send
            assert_eq!(data.id, 42);
            assert_eq!(data.name, "hello");
            data.id // return something to prove we used it
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_thread_safe_data_sync_via_arc() {
        // Demonstrate Sync: share reference across threads via Arc
        let data = Arc::new(ThreadSafeData::new(100, "shared"));

        let mut handles = vec![];

        for i in 0..3 {
            let data_clone = Arc::clone(&data);
            handles.push(thread::spawn(move || {
                // Multiple threads can read &ThreadSafeData safely
                // This works because ThreadSafeData: Sync
                assert_eq!(data_clone.id, 100);
                i
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }
    }

    #[test]
    fn test_unsync_counter_basic() {
        let counter = UnsyncCounter::new(0);
        assert_eq!(counter.get(), 0);

        counter.inc();
        counter.inc();
        counter.inc();

        assert_eq!(counter.get(), 3);
    }

    #[test]
    fn test_unsync_counter_send() {
        // UnsyncCounter IS Send - we can move it to another thread
        let counter = UnsyncCounter::new(10);

        let handle = thread::spawn(move || {
            // counter was moved here
            counter.inc();
            counter.get()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 11);
    }

    // NOTE: We cannot write a test that shares &UnsyncCounter across threads
    // because it's not Sync. The compiler would reject it!
    //
    // This WOULD NOT COMPILE:
    // ```
    // let counter = UnsyncCounter::new(0);
    // let counter_ref = &counter;
    // thread::spawn(move || {
    //     counter_ref.inc();  // ERROR: UnsyncCounter is not Sync
    // });
    // ```

    #[test]
    fn test_shared_local_basic() {
        let shared = SharedLocal::new(42);
        assert_eq!(*shared.get(), 42);
        assert_eq!(shared.ref_count(), 1);
    }

    #[test]
    fn test_shared_local_clone() {
        let shared1 = SharedLocal::new("hi");
        let shared2 = shared1.clone();

        assert_eq!(*shared1.get(), "hi");
        assert_eq!(*shared2.get(), "hi");

        assert_eq!(shared1.ref_count(), 2);
        assert_eq!(shared2.ref_count(), 2);
    }

    // NOTE: We CANNOT move SharedLocal to another thread because Rc is not Send.
    // This WOULD NOT COMPILE:
    // ```
    // let shared = SharedLocal::new(42);
    // thread::spawn(move || {
    //     println!("{}", shared.get());  // ERROR: Rc<i32> cannot be sent between threads
    // });
    // ```

    #[test]
    fn test_atomic_counter_basic() {
        let counter = AtomicCounter::new(0);
        assert_eq!(counter.get(), 0);

        counter.inc();
        counter.inc();

        assert_eq!(counter.get(), 2);
    }

    #[test]
    fn test_atomic_counter_shared_mutation() {
        let counter = AtomicCounter::new(0);

        let mut handles = vec![];

        for _ in 1..=10 {
            let cloned_counter = counter.clone();
            handles.push(thread::spawn(move || {
                for _ in 1..=100 {
                    cloned_counter.inc();
                }
            }));
        }

        for h in handles {
            h.join().unwrap();
        }

        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_atomic_counter_sync_via_reference() {
        let counter = AtomicCounter::new(0);
        let shared = Arc::new(counter);

        let mut handles = vec![];

        for _ in 0..5 {
            let shared_clone = Arc::clone(&shared);
            handles.push(thread::spawn(move || {
                shared_clone.inc();
            }));
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(shared.get(), 5);
    }

    #[test]
    fn test_not_thread_safe_basic() {
        let nts = NotThreadSafe::new(42);
        assert_eq!(nts.data, 42);
    }

    // NOTE: Cannot spawn threads with these types!
    // The following would NOT compile:
    // ```
    // let nts = NotThreadSafe::new(42);
    // thread::spawn(move || {
    //     println!("{}", nts.data);  // ERROR: NotThreadSafe cannot be sent
    // });
    // ```

    #[test]
    fn test_raw_pointer_wrapper_basic() {
        let mut wrapper = RawPtrWrapper::new(42);
        assert_eq!(wrapper.get(), 42);

        wrapper.set(100);
        assert_eq!(wrapper.get(), 100);
    }

    #[test]
    fn test_raw_pointer_wrapper_send() {
        // We can move RawPtrWrapper to another thread
        let wrapper = RawPtrWrapper::new(123);

        let handle = thread::spawn(move || {
            // wrapper moved here - works because we impl Send
            wrapper.get()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 123);
    }

    #[test]
    fn test_raw_pointer_wrapper_sync() {
        // We can share &RawPtrWrapper across threads
        let wrapper = Arc::new(RawPtrWrapper::new(456));

        let mut handles = vec![];

        for _ in 0..5 {
            let wrapper_clone = Arc::clone(&wrapper);
            handles.push(thread::spawn(move || {
                // Multiple threads reading - works because we impl Sync
                wrapper_clone.get()
            }));
        }

        for handle in handles {
            let result = handle.join().unwrap();
            assert_eq!(result, 456);
        }
    }
}
