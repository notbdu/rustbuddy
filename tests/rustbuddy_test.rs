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
fn it_allocates_one_page() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
    let page_offset = buddy.allocate(1);
    assert_eq!(page_offset, 0);
    assert_eq!("\
       S\n\
       SO\n\
       SOOO\n\
       SOOOOOOO\n\
       UOOOOOOOOOOOOOOO\n",
       buddy.dump());
}

#[test]
fn it_frees_one_page() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
    let page_offset = buddy.allocate(1);
    buddy.free(1, page_offset as usize);
    assert_eq!("\
       O\n\
       OO\n\
       OOOO\n\
       OOOOOOOO\n\
       OOOOOOOOOOOOOOOO\n",
       buddy.dump());
}

#[test]
fn it_allocates_and_frees_many_pages() {
    let mut buddy = rustbuddy::BuddyAllocator::new(4);
	let page_offset = buddy.allocate(1);
    assert_eq!(page_offset, 0);
	let page_offset1 = buddy.allocate(1);
    assert_eq!(page_offset1, 1);
	let page_offset2 = buddy.allocate(1);
    assert_eq!(page_offset2, 2);
	let page_offset3 = buddy.allocate(1);
    assert_eq!(page_offset3, 3);
	let page_offset4 = buddy.allocate(1);
    assert_eq!(page_offset4, 4);
	let page_offset5 = buddy.allocate(2);
    assert_eq!(page_offset5, 6);
    assert_eq!("\
       S\n\
       SO\n\
       FSOO\n\
       FFSUOOOO\n\
       UUUUUOOOOOOOOOOO\n",
       buddy.dump());
	buddy.free(1, page_offset as usize);
	buddy.free(1, page_offset1 as usize);
	buddy.free(1, page_offset2 as usize);
	buddy.free(1, page_offset3 as usize);
	buddy.free(1, page_offset4 as usize);
	buddy.free(2, page_offset5 as usize);
    assert_eq!("\
       O\n\
       OO\n\
       OOOO\n\
       OOOOOOOO\n\
       OOOOOOOOOOOOOOOO\n",
       buddy.dump());
}
