mod front_of_restro {
   pub mod hosting {
       pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

   pub mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}
pub fn eat_at_restaurant(){
    crate::front_of_restro::hosting::add_to_waitlist();
    //absolute path
    front_of_restro::hosting::add_to_waitlist();
}


fn deliver_order(){}
 mod back_of_house{
    fn fix_incorrect_order(){
        cook_order();
        super::deliver_order();

    }
    fn cook_order(){}
 }

 //similirly to use scops and other structs enums according to the code logic