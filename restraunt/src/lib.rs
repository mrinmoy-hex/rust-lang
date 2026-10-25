pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        pub fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        fn serve_order() {
            crate::front_of_house::hosting::seat_at_table(); // absolute path
            super::hosting::seat_at_table(); // relative
        }

        fn take_payment() {}
    }
}

fn eat_at_restraunt() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
}
