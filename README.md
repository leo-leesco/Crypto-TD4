# ChaCha20 and integration with Poly1305

Please check the latest version on [GitHub](github.com/leo-leesco/Crypto-TD4).

## Build

`cargo build` produces `x25519` in `target/debug`.

If you want the optimized version, run `cargo build --release`, and the executables can then be found in `target/release`.

## Requirements

`x25519` expects :
- `m` as a 64-hexadecimal string
- (optional) `u` as a 64-hexadecimal string; if it is not provided, will be set to the default `u=9`

## Design philosophy

### Curve25519

We consider the elliptic curve :
$$\mathcal E:By^2=x^3+Ax^2+x\qquad(\mathbb F_p)$$

Firstly, since $\F_p$ is a field, with $p=2^{255}-19$, we introduce the projective plane :
$$(x:y:1)\sim(X:Y:Z)\iff x=X/Z\quad\text{et}\quad y=Y/Z$$

Thus, $\mathcal E:BY^2Z = X(X^2 + AXZ + Z^2)$.

The objective of this part is to compute $[m](x,\bullet)$, where the $y$ coordinate is disregarded. This is to be understood as the multiplication by $m\in\Z$ in the group defined by the non-degenerated elliptic curve addition.

Using the projective coordinates, we have to compute $[m](X,\bullet,Z)$. The technique that is used to achieve this is the *Montgomery ladder*.
