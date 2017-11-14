extern crate rustbuddy;

#[test]
fn it_creates_a_new_instance() {
    let buddy = rustbuddy::BuddyAllocator::new(4);
    assert_eq!("\
       O\n\
       OO\n\
       OOOO\n\
       OOOOOOOO\n\
       OOOOOOOOOOOOOOOO\n",
       buddy.dump());
}

#[test]
fn it_allocates_one_block() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
    buddy.allocate(1);
    assert_eq!("\
       S\n\
       SO\n\
       SOOO\n\
       SOOOOOOO\n\
       UOOOOOOOOOOOOOOO\n",
       buddy.dump());
}

#[test]
fn it_frees_one_block() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
    buddy.allocate(1);
    buddy.free(15);
    assert_eq!("\
       O\n\
       OO\n\
       OOOO\n\
       OOOOOOOO\n\
       OOOOOOOOOOOOOOOO\n",
       buddy.dump());
}

#[test]
fn it_allocates_and_frees_many_blocks() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
	buddy.allocate(1);
	buddy.allocate(1);
	buddy.allocate(1);
	buddy.allocate(1);
	buddy.allocate(1);
	buddy.allocate(2);
    assert_eq!("\
       S\n\
       SO\n\
       FSOO\n\
       FFSUOOOO\n\
       UUUUUOOOOOOOOOOO\n",
       buddy.dump());
	buddy.free(19);
	buddy.free(18);
	buddy.free(17);
	buddy.free(10);
	buddy.free(16);
	buddy.free(15);
    assert_eq!("\
       O\n\
       OO\n\
       OOOO\n\
       OOOOOOOO\n\
       OOOOOOOOOOOOOOOO\n",
       buddy.dump());
}
