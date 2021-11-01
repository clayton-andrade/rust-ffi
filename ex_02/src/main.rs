mod bindings;

use std::fmt;

type Complex<T> = bindings::__BindgenComplex<T>;

impl fmt::Display for Complex<f32> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.im < 0.0 {
            write!(f, "{}-{}i", self.re, -self.im)
        } else {
            write!(f, "{}+{}i", self.re, self.im)
        }
    }
}

fn main() {
    let z = Complex { re: -1.0, im: 0.0 };
    let z_sqrt = unsafe { bindings::csqrtf(z) };
    println!("the square root of {} is {}", z, z_sqrt);
    println!("cos({}) = {}", z, cos(z));
}

fn cos(c: Complex<f32>) -> Complex<f32> {
    unsafe {
        bindings::ccosf(c)
    }
}
