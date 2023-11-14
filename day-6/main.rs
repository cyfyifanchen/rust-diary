// Today is about Ownership 
fn main() {
    let s = "hello";
    {                           // s is not valid here, it's not yet declared
      let s = "hello";          // s is valid from this point forward

      // do something with s
    }//
                                // this scope is now over, and s is no longer valid
}