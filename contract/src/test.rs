use super::EscrowStore;

#[test]
fn test_happy_path() {
    let mut store = EscrowStore::new();
    store.create_escrow("esc1", "buyer1", "farmer1", 1000);
    store.confirm_delivery("esc1", "buyer1");
    let released = store.release_funds("esc1");
    assert_eq!(released, 1000);
}

#[test]
#[should_panic(expected = "Not authorized")]
fn test_unauthorized_confirm() {
    let mut store = EscrowStore::new();
    store.create_escrow("esc2", "buyer1", "farmer1", 1000);
    store.confirm_delivery("esc2", "attacker");
}

#[test]
#[should_panic(expected = "Delivery not confirmed")]
fn test_release_without_delivery() {
    let mut store = EscrowStore::new();
    store.create_escrow("esc3", "buyer1", "farmer1", 1000);
    store.release_funds("esc3");
}

#[test]
fn test_multiple_escrows() {
    let mut store = EscrowStore::new();
    store.create_escrow("a1", "buyer1", "farmer1", 500);
    store.create_escrow("a2", "buyer1", "farmer1", 800);

    let esc1 = store.get_escrow("a1");
    let esc2 = store.get_escrow("a2");

    assert_eq!(esc1.amount, 500);
    assert_eq!(esc2.amount, 800);
}

#[test]
#[should_panic(expected = "Escrow not found")]
fn test_release_removes_escrow() {
    let mut store = EscrowStore::new();
    store.create_escrow("a4", "buyer1", "farmer1", 1000);
    store.confirm_delivery("a4", "buyer1");
    let released = store.release_funds("a4");
    assert_eq!(released, 1000);

    // After releasing funds, the escrow should no longer exist.
    let _ = store.get_escrow("a4");
}
