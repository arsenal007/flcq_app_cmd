/*use std::io::{stdin, stdout, Read, Write};*/

extern crate question;
use self::question::{Answer, Question};

pub fn pause() {
    Question::new("Press Enter to continue...")
        .yes_no()
        .until_acceptable()
        .default(Answer::YES)
        .ask();

    /*let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0]).unwrap();*/
}
