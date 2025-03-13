#![doc(html_root_url = "https://docs.rs/test_gmp_mpir/0.0.3")]
//! test_gmp_mpir
//!
//! # Requirements
//!
//! - [ gmp ]( https://gmplib.org/ )
//! - [ mpir ]( https://github.com/ChillMagic/MPIR-Binary )
//!
//! in the running directory
//!
//! - libgmp-10.dll
//! - libgmpxx-4.dll (optional)
//! - mpir.dll
//!
//! see also
//!
//! - [ https://crates.io/crates/mpir ]( https://crates.io/crates/mpir )
//! - [ https://github.com/nomissbowling/mpir ]( https://github.com/nomissbowling/mpir )
//!

/*
  see also _test_gmp_mpfr_.c

  to compile this source main.rs without Cargo (on Windows)
  rustc -o test_mpir.exe -L ../GMP -l libgmp-10 -l libgmpxx-4 -l mpir main.rs
  dir *rcgu.o
  del *rcgu.o
  del test_mpir.pdb
  test_mpir.exe

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int, c_double, c_void};
let c = c"abc"; // new version
unsafe { let c = CStr::from_ptr(0 as *const i8); }
let cs = CString::new("abc");

- [mpz ...](https://gmplib.org/manual/Integer-Functions)
- [mpz cmp ...](https://gmplib.org/manual/Integer-Comparisons)
- [mpz fac_ui fib_ui ...](https://gmplib.org/manual/Number-Theoretic-Functions)
- [mpz sqrt root ...](https://gmplib.org/manual/Integer-Roots)
- [mpz add sub mul addmul ...](https://gmplib.org/manual/Integer-Arithmetic)
- [mpz *div mod ...](https://gmplib.org/manual/Integer-Division)
- [mpz pow ...](https://gmplib.org/manual/Integer-Exponentiation)

- [mpf ...](https://gmplib.org/manual/Rational-Number-Functions)
- [mpf cmp ...](https://gmplib.org/manual/Float-Comparison)
- [mpf sqrt add sub mul div pow ...](https://gmplib.org/manual/Float-Arithmetic)

- [mpq ...](https://gmplib.org/manual/Rational-Number-Functions)
- [mpq cmp ...](https://gmplib.org/manual/Comparing-Rationals)
- [mpq add sub mul div inv ...](https://gmplib.org/manual/Rational-Arithmetic)
*/

// use std::env;
use std::error::Error;
use std::collections::HashMap;

use mpir::*;
use mpir::gmp::{__gmp_allocate_func, __gmp_reallocate_func, __gmp_free_func};

pub fn main() -> Result<(), Box<dyn Error>> {
/*
  // let dir = env::var("CARGO_MANIFEST_DIR")?; // NotPresent
  let dir = ".";
  println!(r"cargo:rustc-link-search=native={}/../GMP", dir);
  println!("cargo:rustc-link-lib=dylib={}", "libgmp-10");
  println!("cargo:rustc-link-lib=dylib={}", "mpir");
*/

  // mpz (c style)
  let a = &mut mpz_s::new();
  mpz_init(a);
  mpz_set_ui(a, 1234567890); // _set_si _set_d
  gmp_printf("a [%Zd]\n", a);
  println!("dbg a {:?}\nfmt a {}", a, a);

  let b = &mut mpz_s::new();
  // mpz_init_set_ui(b, 9876543210); // overflow
  // mpz_set_str(b, "9876543210", 10); // must init before
  mpz_init_set_str(b, "9876543210", 10);
  gmp_printf("b [%Zd]\n", b);
  println!("dbg b {:?}\nfmt b {}", b, b);

  let c = &mut mpz_s::new();
  // mpz_init_set_ui(c, 1234987654321); // overflow
  // mpz_set_str(c, "1234987654321", 10); // must init before
  mpz_init_set_str(c, "1234987654321", 10);
  gmp_printf("c [%Zd]\n", c);
  println!("dbg c {:?}\nfmt c {}", c, c);

  mpz_set(b, a);
  gmp_printf("b <- a [%Zd]\n", b);

  mpz_set_str(a, "12394872039846520983745092837458238947528346283745802837468238947520137489572463589", 10);
  mpz_set_str(b, "39846520983745092837458238947528346283745802837468238947520137489572463589", 10);
  mpz_set_str(c, "12394872039846520983745092837458238947528346283745802837468238947520137489572463589", 10);
  gmp_printf_u8z(b"a %Zd\n\0", a);
  gmp_printf_u8z(to_u8z!("b %Zd\n"), b);
  gmp_printf_u8z(term_z!(b"c %Zd\n"), c);
  println!("dbg a {:?}\nfmt a {}", a, a);
  println!("dbg b {:?}\nfmt b {}", b, b);
  println!("dbg c {:?}\nfmt c {}", c, c);

  let r = &mut mpz_s::new();
  mpz_init_set_si(r, -1234567890);
  gmp_printf("r [%Zd]\n", r);
  println!("dbg r {:?}\nfmt r {}", r, r);

  // mpf (c style)
  let f = &mut mpf_s::new();
  mpf_init_set_d(f, 5.0);
  gmp_printf_1f("f [%.*Ff]\n", 17, f);
  println!("dbg f {:?}\nfmt f {}", f, f);
  let g = &mut mpf_s::new();
  mpf_init(g);
  mpf_sqrt(g, f);
  gmp_printf_2f("sqrt(%.*Ff) = %.*Ff\n", 17, f, 17, g);
  println!("dbg g {:?}\nfmt g {}", g, g);

  // mpq (c style)
  let q = &mut mpq_s::new();
  mpq_init(q);
  mpq_set_ui(q, 2, 8);
  gmp_printf("q hex rational [%#40Qx]\n", q);
  println!("dbg q {:?}\nfmt q {}", q, q);

  // test function
  let mut mp_alloc: FnPtrAllocate = __gmp_allocate_func; // dummy
  let mut mp_realloc: FnPtrReallocate = __gmp_reallocate_func; // dummy
  let mut mp_free: FnPtrFree = __gmp_free_func; // dummy
  println!("{:016x}, {:016x}, {:016x}",
    mp_alloc as u64, mp_realloc as u64, mp_free as u64);
  mp_get_memory_functions(&mut mp_alloc, &mut mp_realloc, &mut mp_free);
  println!("{:016x}, {:016x}, {:016x}",
    mp_alloc as u64, mp_realloc as u64, mp_free as u64);

  // mpz (c style) convert to string
  mpz_set_ui(c, 65);
  if let Ok(s) = mpz_get_str(None, 10, c) {
    println!("c [{}]", s);
  }
  let mut buf = String::from_utf8(vec![37u8; 48]).unwrap();
  println!("buf [{:?}]", buf);
  let s = match mpz_get_str(Some(&mut buf), 10, c) {
  Err(e) => e.to_string(),
  Ok(s) => s
  };
  println!("buf [{:?}]", buf);
  println!("c [{}]", s);

  // mpf (c style)
  let t = &mut mpf_s::new();
  mpf_init(t);
  mpf_set_str(t, "-1234.56789012345678901234567890", 10);
  let e = &mut (0 as mp_exp_t);
  if let Ok(s) = mpf_get_str(None, e, 10, 20, t) {
    println!("t [{}] [{}]", e, s);
  }
  println!("dbg t {:?}\nfmt t {}", t, t);

  let z = &mut mpf_s::new();
  mpf_init(z);
  mpf_set_d(z, -0.0);
  let e = &mut 0i32; // mp_exp_t;
  if let Ok(s) = mpf_get_str(None, e, 10, 20, z) {
    println!("z [{}] [{}]", e, s);
  }
  println!("dbg z {:?}\nfmt z {}", z, z);

  let y = &mut mpf_s::new();
  mpf_init(y);
  mpf_set_d(y, -0.3);
  let e = &mut 0i32; // mp_exp_t;
  if let Ok(s) = mpf_get_str(None, e, 10, 20, y) {
    println!("y [{}] [{}]", e, s);
  }
  println!("dbg y {:?}\nfmt y {}", y, y);

  let x = &mut mpf_s::new();
  mpf_init(x);
  mpf_set_d(x, -3.0);
  let e = &mut 0i32; // mp_exp_t;
  if let Ok(s) = mpf_get_str(None, e, 10, 20, x) {
    println!("x [{}] [{}]", e, s);
  }
  println!("dbg x {:?}\nfmt x {}", x, x);

  let w = &mut mpf_s::new();
  mpf_init(w);
  mpf_set_d(w, -30.0);
  let e = &mut 0i32; // mp_exp_t;
  if let Ok(s) = mpf_get_str(None, e, 10, 20, w) {
    println!("w [{}] [{}]", e, s);
  }
  println!("dbg w {:?}\nfmt w {}", w, w);

  let v = &mut mpf_s::new();
  mpf_init(v);
  mpf_set_d(v, -0.03);
  let e = &mut 0i32; // mp_exp_t;
  if let Ok(s) = mpf_get_str(None, e, 10, 20, v) {
    println!("v [{}] [{}]", e, s);
  }
  println!("dbg v {:?}\nfmt v {}", v, v);

  // mpq (c style)
  let q = &mut mpq_s::new();
  mpq_init(q);
  mpq_set_ui(q, 1, 3);
  if let Ok(s) = mpq_get_str(None, 10, q) {
    println!("q [{}]", s);
  }

  // mpz and mpf (prepare and reset)
  let a = &mut mpz_s::init();
  let f = &mut mpf_s::init();
  let g = &mut mpf_s::init();
/*
  // ***must NOT call*** auto called clear
  a.clear(); mpz_init(a);
  f.clear(); mpf_init(f);
  g.clear(); mpf_init(g);
*/
/*
  // ***must NOT call*** auto called clear
  mpz_clears(&mut vec![a]); mpz_init(a);
  mpf_clears(&mut vec![g, f]); mpf_init(f); mpf_init(g);
*/

  // mpz (to be operator)
  a.set_str("987654321098765432109", 10);
  println!("dbg a {:?}\nfmt a {}", a, a);

  // mpf (to be operator)
  println!("f {}", f.set_z(a).div(g.set_str("1.0e+11", 10)));
  println!("f {}", f.fmtstr(10, 22));

  // mpz fac_ui (to be operator)
  (0..=20).into_iter().for_each(|n: usize| {
    let t = &mut mpz_s::fac_ui(n as ui_t);
    println!("{}! = {}", n, t);
  });

  // mpz fact (to be operator)
  (0..=20).into_iter().for_each(|n: usize| {
    let t = &mut mpz_s::fact(n as ui_t);
    println!("{}! = {}", n, t);
  });

  // mpz fact (to be operator) cached
  let m = &mut HashMap::<ui_t, mpz_s>::new();
  (0..=20).into_iter().for_each(|n: usize| {
    let t = &mut mpz_s::fact_cached(n as ui_t, m);
    println!("{}! = {}", n, t);
  });

  // mpf prec (c style)
  println!("prec {}", mpf_get_default_prec()); // may be 64
  mpf_set_default_prec(100); // 100 set to 128 bits (step by 2**n)
  println!("prec {}", mpf_get_default_prec()); // may be 128 (about 38 digits)
  let digits = mpf_s::calc_digits_from_bits(128);
  println!("digits {}", digits); // may be 38

  // mpf calc napier (to be operator)
  let digits = 40;
  mpf_set_default_prec(mpf_s::calc_bits_from_digits(digits + 3));
  let e = &mut mpf_s::calc_napier(&mut mpf_s::init_set_d(1.0), digits);
  assert_eq!(format!("{}", e),
    "0.27182818284590452354e+1");
  assert_eq!(e.fmtstr(10, digits),
    "0.2718281828459045235360287471352662497757e+1");

  let digits = 150;
  mpf_set_default_prec(mpf_s::calc_bits_from_digits(digits + 3));
  let e = &mut mpf_s::calc_napier(&mut mpf_s::init_set_d(1.0), digits);
  assert_eq!(format!("{}", e),
    "0.27182818284590452354e+1");
  assert_eq!(e.fmtstr(10, digits),
    "0.271828182845904523536028747135266249775724709369995957496696762772407663035354759457138217852516642742746639193200305992181741359662904357290033429526e+1");
/*
  2.
  7182818284 5904523536 0287471352 6624977572 4709369995
  9574966967 6277240766 3035354759 4571382178 5251664274
  2746639193 2003059921 8174135966 2904357290 0334295260
  ...
*/

  println!("done");
  Ok(())
}
