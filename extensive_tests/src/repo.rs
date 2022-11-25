trait VersionControl {
    fn add();
    fn commit();
    fn push();
    fn pull();
    fn revert();
}

struct Git {}
struct Mercury {}

impl VersionControl for Git {
    fn add() {
        println!("Add!");
    }
    fn commit() {
        println!("Commit");
    }
    fn push() {
        todo!()
    }
    fn pull() {
        todo!()
    }
    fn revert() {
        todo!()
    }
}
impl VersionControl for Mercury {
    fn add() {
        println!("Add!");
    }
    fn commit() {
        println!("Commit");
    }
    fn push() {
        todo!()
    }
    fn pull() {
        todo!()
    }
    fn revert() {
        todo!()
    }
}
