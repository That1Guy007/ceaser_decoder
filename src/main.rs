fn main() {
    //let alpha = ['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's', 't', 'u', 'v', 'x', 'y', 'z'];
    let mut encrypted = String::from("HVS EIWQY PFCKB TCL XIADG CJSF HVS ZONM RCU CT QOSGOF OBR MCIF IBWEIS GCZIHWCB WG CTTTUBTSAGBG");
    unsafe {
        let v_encrypted = encrypted.as_mut_vec();
        let mut x = 1;
        let mut i = 0;
        while x != 26 {
            while i != v_encrypted.len() {
                //print!("FUCK...");
                if v_encrypted[i] != ' ' as u8 {
                    print!("{}", arithmetic(v_encrypted[i], x));
                }
                else{
                    print!(" ");
                }
                i += 1;
            }
            i = 0;
            x += 1;
            println!();
        }
        unwrap_f();
    }
}

fn arithmetic(letter: u8, delta: usize) -> char {

    if (letter + (delta as u8)) > 90 {
        (letter - (26 - delta) as u8) as char
    }
    else {
        (letter + (delta as u8)) as char
    } 
}

fn unwrap_f(){
    let var = "HVS EIWQY PFCKB TCL XIADG CJSF HVS ZONM RCU CT QOSGOF OBR MCIF IBWEIS GCZIHWCB WG CTTTUBTSAGBG";

    let mut iter = var.chars();
    let mut i = iter.next();
    while i != None {
        print!("{}", i.unwrap());
        i = iter.next();
    }
}