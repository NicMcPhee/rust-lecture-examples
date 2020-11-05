

void h(x) {
    printf("...", x);
    // h is done with x, so f can use it
    // or free it
}

void h(x) {
    global_x = x;
    // or h() passes x to another thread
    
    // Now other people are pointing at x
    // and f can't free it, and at best
    // needs to be careful about *using* it.
}

void g() {
    // make x
    // g can't free x because other
    // functions will need it
    return x;
}

void f() {
    x = g();
    h(x);
    // Who owns x now?
    // Can we free it?
}

