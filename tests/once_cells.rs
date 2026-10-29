use std::{cell::OnceCell, sync::OnceLock};

use dbg_pls::{DebugPls, DebugWith, Formatter};

#[test]
fn empty_cells_stay_uninitialized() {
    let cell = OnceCell::<u32>::new();
    let lock = OnceLock::<u32>::new();
    assert_eq!(dbg_pls::pretty(&cell).to_string(), "OnceCell(\"<uninit>\")");
    assert_eq!(dbg_pls::pretty(&lock).to_string(), "OnceLock(\"<uninit>\")");
    assert!(cell.get().is_none());
    assert!(lock.get().is_none());
}

#[test]
fn derived_structs_can_contain_cells() {
    #[derive(DebugPls)]
    struct Cells(OnceCell<u32>, OnceLock<u32>);

    let value = Cells(OnceCell::new(), OnceLock::from(42));
    assert_eq!(
        dbg_pls::pretty(&value).to_string(),
        "Cells(OnceCell(\"<uninit>\"), OnceLock(42))"
    );
}

// Deliberately implements neither std::fmt::Debug nor DebugWith<()>.
struct Payload;

impl DebugWith<u32> for Payload {
    fn fmt(&self, with: &u32, f: Formatter<'_>) {
        f.debug_tuple_struct("Payload").field(with).finish();
    }
}

struct WithContext<'a, T>(&'a T, u32);

impl<T: DebugWith<u32>> DebugPls for WithContext<'_, T> {
    fn fmt(&self, f: Formatter<'_>) {
        self.0.fmt(&self.1, f);
    }
}

#[test]
fn initialized_cells_format_their_contents() {
    assert_eq!(
        dbg_pls::pretty(&OnceCell::from(42_u32)).to_string(),
        "OnceCell(42)"
    );
    assert_eq!(
        dbg_pls::pretty(&OnceLock::from(42_u32)).to_string(),
        "OnceLock(42)"
    );
}

#[test]
fn cells_forward_the_formatting_context() {
    assert_eq!(
        dbg_pls::pretty(&WithContext(&OnceCell::from(Payload), 42)).to_string(),
        "OnceCell(Payload(42))"
    );
    assert_eq!(
        dbg_pls::pretty(&WithContext(&OnceLock::from(Payload), 42)).to_string(),
        "OnceLock(Payload(42))"
    );
}

#[test]
fn once_cell_can_be_formatted_during_initialization() {
    let cell = OnceCell::new();
    cell.get_or_init(|| {
        assert_eq!(dbg_pls::pretty(&cell).to_string(), "OnceCell(\"<uninit>\")");
        42_u32
    });
    assert_eq!(dbg_pls::pretty(&cell).to_string(), "OnceCell(42)");
}

#[test]
fn once_lock_formatting_does_not_wait_for_initialization() {
    use std::{sync::mpsc, sync::Arc, thread, time::Duration};

    let lock = Arc::new(OnceLock::new());
    let (started_tx, started_rx) = mpsc::channel();
    let (release_tx, release_rx) = mpsc::channel();
    let initializing = Arc::clone(&lock);
    let initializer = thread::spawn(move || {
        initializing.get_or_init(|| {
            started_tx.send(()).unwrap();
            release_rx.recv().unwrap();
            42_u32
        });
    });
    started_rx.recv_timeout(Duration::from_secs(5)).unwrap();

    let (formatted_tx, formatted_rx) = mpsc::channel();
    let formatting = Arc::clone(&lock);
    let formatter = thread::spawn(move || {
        formatted_tx
            .send(dbg_pls::pretty(&formatting).to_string())
            .unwrap();
    });
    let output = formatted_rx.recv_timeout(Duration::from_secs(5));
    // Release initialization before asserting, so a blocking regression can fail
    // and join both threads rather than hanging the test process.
    release_tx.send(()).unwrap();
    initializer.join().unwrap();
    formatter.join().unwrap();
    assert_eq!(output.unwrap(), "OnceLock(\"<uninit>\")");
    assert_eq!(dbg_pls::pretty(&lock).to_string(), "OnceLock(42)");
}
