test_gmp_mpir
=============

test gmp mpir for Rust


Sample
------

calc pi/4 sum arctan Gregory
- arctan x = sigma[n=0-&gt;inf]{x**(2n+1)*(-1**n)/(2n+1)}
- sum_n = sigma[k=0-&gt;ax.len](a_k * arctan_n x_k)
- result = sigma[n=0-&gt;m]sum_n
- inner loop may be slow (should mul a_k outer) to check stop iterator

```Rust
  pub fn sum_arctan_gregory(ax: &[(si_t, si_t)], m: ui_t) -> mpf_s {
    let mut sa = mpf_s::init_set_ui(0);
    let ax = ax.into_iter().map(|&(a, x)|
      (mpf_s::init_set_si(a), mpf_s::init_set_si(x).inv())
    ).collect::<Vec<_>>();
    let _s = (0..=m).try_fold(&mut sa, |mut sa, n| {
      let pre = &mpf_s::init_set(sa);
      let k = 2 * n + 1;
      let mut sn = mpf_s::init_set_ui(0);
      let _a = ax.iter().fold(&mut sn, |mut sn, (a, x)| {
        let s = a * mpf_s::pow_ui(&x, k) / k; // move mul a outer to be faster
        if n & 1 == 1 { sn -= s; } else { sn += s; }
        sn
      });
      sa += sn;
      if sa.cmp(pre) == 0 { None } else { Some(sa) }
    });
    sa
  }
```


Links
-----

- [https://crates.io/crates/test_gmp_mpir](https://crates.io/crates/test_gmp_mpir)
- [https://github.com/nomissbowling/test_gmp_mpir](https://github.com/nomissbowling/test_gmp_mpir)


Requirements
------------

- [gmp](https://gmplib.org)
- [mpir](https://github.com/ChillMagic/MPIR-Binary)


License
-------

MIT License
