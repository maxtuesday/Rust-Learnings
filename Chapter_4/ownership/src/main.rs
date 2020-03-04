fn main() {
    println!("Ownership");

    // String literal
    {
        let mut s = String::from("hello");

        s.push_str(", world!");

        println!("{}", s);
    }

    // Move
    {
        let x = 5;
        // Makes a copy the value of x and bind it to y
        let _y = x;
    }

    {
        let s1 = String::from("hello");
        // Make a copy of the value of s1 and bind it to s2?...
        // Does not work the same way as above
        let _s2 = s1;

        /*
          (data on the stack)           (data on the heap)
                  s1
         --------------------            ---------------
        |   name   |  value  |          | index | value |
         --------------------            ---------------
        |   ptr    |    -----|--------->|   0   |   h   |
         --------------------            ---------------
        |   len    |    5    |          |   1   |   e   |
         --------------------            ---------------
        | capacity |    5    |          |   2   |   l   |
         --------------------            ---------------
                                        |   3   |   l   |
                                         ---------------
                                        |   4   |   o   |
                                         ---------------

          length: How much memory, in bytes, the contents of
                  the String is currently using
          capacity: Total amount of memory, in bytes, that the String
                    has received from the operating system
        */

        /*

        When we assign s1 to s2, the String data is copied
        Meaning we copy the pointer, the length, and the capacity
        that are on the stack.
        We do not copy the data on the heap that the pointer refers to.

                  s1
         --------------------            ---------------
        |   name   |  value  |          | index | value |
         --------------------            ---------------
        |   ptr    |    -----|-----/--->|   0   |   h   |
         --------------------      |     ---------------
        |   len    |    5    |     |    |   1   |   e   |
         --------------------      |     ---------------
        | capacity |    5    |     |    |   2   |   l   |
         --------------------      |     ---------------
                                   |    |   3   |   l   |
                                   |     ---------------
                                   |    |   4   |   o   |
                  s2               |     ---------------
         --------------------      |
        |   name   |  value  |     |
         --------------------      |
        |   ptr    |    -----|-----/
         --------------------
        |   len    |    5    |
         --------------------
        | capacity |    5    |
         --------------------

        */

        /*

        s1 has been invalidated
        ########################
        #          s1          #
        # -------------------- #         ---------------
        #|   name   |  value  |#        | index | value |
        # -------------------- #         ---------------
        #|   ptr    |    -----|#---/--->|   0   |   h   |
        # -------------------- #   |     ---------------
        #|   len    |    5    |#   |    |   1   |   e   |
        # -------------------- #   |     ---------------
        #| capacity |    5    |#   |    |   2   |   l   |
        # -------------------- #   |     ---------------
        ########################   |    |   3   |   l   |
                                   |     ---------------
                                   |    |   4   |   o   |
                  s2               |     ---------------
         --------------------      |
        |   name   |  value  |     |
         --------------------      |
        |   ptr    |    -----|-----/
         --------------------
        |   len    |    5    |
         --------------------
        | capacity |    5    |
         --------------------

        */
    }

    // Clone
    {
        let s1 = String::from("hello");
        let s2 = s1.clone();

        println!("s1 = {}, s2 = {}", s1, s2);
        // Heap data IS copied
    }


    // Stack-Only Data: Copy
    {
        let x = 5;
        let y = x;

        println!("x = {}, y = {}", x, y);

    }

    // Ownership and Functions
    {
        let s = String::from("hello"); // s comes into scope

        println!("{}", s);
        takes_ownership(s); // s's value moves into the function...
                                        // ... and so is no longer valid here


        let x = 5; // x comes into scope

        makes_copy(x); // x would move into the function,
                                    // but i32 is Copy, so it's okay to still
                                    // use x afterward

    } // Here, x goes out of scope, then s. But because s's value was moved,
      // nothing special happens.
}

fn takes_ownership(some_string: String) { // some_string comes into scope
    println!("{}", some_string);
} // Here, some_string goes out of scope and `drop` is called. The backing
  // memory is freed.

fn makes_copy(some_integer: i32) { // some_integer comes into scope
    println!("{}", some_integer);
} // Here, some_integer goes out of scope Nothing special happens.