use clarith::protocol;
use clarith::Clog;
use clarith::Number;

fn main() {
    for n in 0..17 {
        print!("{0: >2}.0: ", n);
        logn(Number::ratio(n, 1));
        println!("");
        print!("{0: >2}.5: ", n);
        logn(Number::ratio(n * 10 + 5, 10));
        println!("");
    }

    print!("{}: ", isize::max_value());
    logn(Number::ratio(isize::max_value(), 1));
    println!("");

    print!("1/{}: ", isize::max_value());
    logn(Number::ratio(1, isize::max_value()));
    println!("");

    print!("-2: ");
    logn(Number::ratio(-2, 1));
    println!("");
}

fn logn(x: Number) {
    match x {
        Number::Special(s) => {
            logs(s);
        }
        Number::Other(op, c) => {
            if let Some(p) = op {
                logp(p);
            }
            logc(c);
        }
    }
}

fn logs(s: protocol::Special) {
    match s {
        protocol::Special::Zero => {
            print!("Z");
        }
        protocol::Special::PosOne => {
            print!("P");
        }
        protocol::Special::NegOne => {
            print!("N");
        }
    }
}

fn logp(p: protocol::Primer) {
    match p {
        protocol::Primer::Turn => {
            print!("T");
        }
        protocol::Primer::Reflect => {
            print!("R");
        }
        protocol::Primer::Ground => {
            print!("G");
        }
    }
}

fn logc(mut c: Clog) {
    loop {
        match c.egest() {
            None => {
                print!("H");
                break;
            }
            Some(r) => {
                logr(r);
            }
        }
    }
}

fn logr(r: protocol::Reduction) {
    match r {
        protocol::Reduction::Amplify => {
            print!("A");
        }
        protocol::Reduction::Uncover => {
            print!("U");
        }
    }
}
