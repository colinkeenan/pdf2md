[← 16 Hydrogen](16-hydrogen.md) · [18 Operators Matrices and Spin →](18-operators-matrices-and-spin.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p243" href="book.pdf#page=243" target="_blank" title="This page in the PDF"><sup>[p. 243]</sup></a>

# 17 3D Symmetric HO in Spherical Coordinates *

We have already solved the problem of a 3D harmonic oscillator by separation of variables in Cartesian coordinates (See section 13.2). It is instructive to **solve the same problem in spherical** **coordinates** and compare the results. The potential is

```math
V ( r ) = \frac{1}{2} \mu \omega^{2} r^{2} .
```

Our radial equation is

```math
\left( \frac{d^{2}}{dr^{2}} + \frac{2}{r} \frac{d}{dr} \right) R_{E\ell} ( r ) + \frac{2 \mu}{\hbar^{2}} \left( E - V ( r ) - \frac{\ell ( \ell + 1 ) \hbar^{2}}{2 \mu r^{2}} \right) R_{E\ell} ( r ) = 0
```

```math
\frac{d^{2} R}{dr^{2}} + \frac{2}{r} \frac{dR}{dr} - \frac{\mu^{2} \omega^{2}}{\hbar^{2}} r^{2} R - \frac{\ell ( \ell + 1 )}{r^{2}} R + \frac{2 \mu E}{\hbar^{2}} R = 0
```

Write the equation in terms of the dimensionless variable

```math
y = \frac{r}{\rho} .
```

```math
\rho = \sqrt{\frac{\hbar}{\mu \omega}}
```

```math
r = \rho y
```

```math
\frac{d}{dr} = \frac{dy}{dr} \frac{d}{dy} = \frac{1}{\rho} \frac{d}{dy}
```

```math
\frac{d^{2}}{dr^{2}} = \frac{1}{\rho^{2}} \frac{d}{dy^{2}}
```

Plugging these into the radial equation, we get

```math
\frac{1}{\rho^{2}} \frac{d^{2} R}{dy^{2}} + \frac{1}{\rho^{2}} \frac{2}{y} \frac{dR}{dy} - \frac{1}{\rho^{4}} \rho^{2} y^{2} R - \frac{1}{\rho^{2}} \frac{\ell ( \ell + 1 )}{y^{2}} R + \frac{2 \mu E}{\hbar^{2}} R = 0
```

```math
\frac{d^{2} R}{dy^{2}} + \frac{2}{y} \frac{dR}{dy} - y^{2} R - \frac{\ell ( \ell + 1 )}{y^{2}} R + \frac{2 E}{\hbar \omega} R = 0 .
```

Now find the behavior for large $`y`$.

```math
\frac{d^{2} R}{dy^{2}} - y^{2} R = 0
```

```math
R \approx e^{- y^{2} / 2}
```

Also, find the behavior for small $`y`$.

```math
\frac{d^{2} R}{dy^{2}} + \frac{2}{y} \frac{dR}{dy} - \frac{\ell ( \ell + 1 )}{y^{2}} R = 0
```

```math
R \approx y^{s}
```

```math
s ( s - 1 ) y^{s - 2} + 2 sy^{s - 2} = \ell ( \ell + 1 ) y^{s - 2}
```

```math
s ( s + 1 ) = \ell ( \ell + 1 )
```

```math
R \approx y^{\ell}
```

<div align="right"><a href="book.pdf#page=243" target="_blank" title="This page in the PDF"><sup>[p. 243]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p244" href="book.pdf#page=244" target="_blank" title="This page in the PDF"><sup>[p. 244]</sup></a>

Explicitly put in this behavior and use a power series expansion to solve the full equation.

```math
R = y^{\ell} \sum_{k = 0}^{\infty} a_{k} y^{k} e^{- y^{2} / 2} = \sum_{k = 0}^{\infty} a_{k} y^{\ell + k} e^{- y^{2} / 2}
```

We’ll need to compute the derivatives.

```math
\frac{dR}{dy} = \sum_{k = 0}^{\infty} a_{k} [ ( \ell + k ) y^{\ell + k - 1} - y^{\ell + k + 1} ] e^{- y^{2} / 2}
```

```math
\frac{d^{2} R}{dy^{2}} = \sum_{k = 0}^{\infty} a_{k} [ ( \ell + k ) ( \ell + k - 1 ) y^{\ell + k - 2} - ( \ell + k ) y^{\ell + k}
```

```math
- ( \ell_{\infty} + k + 1 ) y^{\ell + k} + y^{\ell + k + 2} ] e^{- y^{2} / 2}
```

```math
\frac{d^{2} R}{dy^{2}} = \sum_{k = 0} a_{k} [ ( \ell + k ) ( \ell + k - 1 ) y^{\ell + k - 2}
```

```math
- ( 2 \ell + 2 k + 1 ) y^{\ell + k} + y^{\ell + k + 2} ] e^{- y^{2} / 2}
```

We can now plug these into the radial equation.

```math
\frac{d^{2} R}{dy^{2}} + \frac{2}{y} \frac{dR}{dy} - y^{2} R - \frac{\ell ( \ell + 1 )}{y^{2}} R + \frac{2 E}{\hbar \omega} R = 0
```

Each term will contain the exponential $`e^{- y^{2} / 2}`$, so we can factor that out. We can also run a single sum over all the terms.

```math
\sum_{k = 0}^{\infty} a_{k} \left[ ( \ell + k ) ( \ell + k - 1 ) y^{\ell + k - 2} - ( 2 \ell + 2 k + 1 ) y^{\ell + k} + y^{\ell + k + 2} \right.
```

```math
\left. + 2 ( \ell + k ) y^{\ell + k - 2} - 2 y^{\ell + k} - y^{\ell + k + 2} - \ell ( \ell + 1 ) y^{\ell + k - 2} + \frac{2 E}{\hbar \omega} y^{\ell + k} \right] = 0
```

The terms for large $`y`$ which go like $`y^{\ell + k + 2}`$ and some of the terms for small $`y`$ which go like $`y^{\ell + k - 2}`$ should cancel if we did our job right.

```math
\sum_{k = 0}^{\infty} a_{k} \left[ [ ( \ell + k ) ( \ell + k - 1 ) - \ell ( \ell + 1 ) + 2 ( \ell + k ) ] y^{\ell + k - 2} \right.
```

```math
\left. \sum_{{}^{\infty} a} \left[ [ \ell ( \ell - 1 ) + k ( 2 \ell + k - 1 ) - \ell ( \ell + 1 ) + 2 \ell + 2 k ] y^{\ell + k - 2} \right. + \left[ \frac{2 E}{\hbar \omega} - 2 - ( 2 \ell + 2 k + 1 ) \right] y^{\ell + k} \right] = 0
```

```math
\left. {}^{k = 0 k} \quad + \left[ \frac{2 E}{\hbar \omega} - 2 - ( 2 \ell + 2 k + 1 ) \right] y^{\ell + k} \right] = 0
```

```math
\sum_{k = 0}^{\infty} a_{k} \left[ [ k ( 2 \ell + k + 1 ) ] y^{\ell + k - 2} + \left[ \frac{2 E}{\hbar \omega} - ( 2 \ell + 2 k + 3 ) \right] y^{\ell + k} \right] = 0
```

<div align="right"><a href="book.pdf#page=244" target="_blank" title="This page in the PDF"><sup>[p. 244]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p245" href="book.pdf#page=245" target="_blank" title="This page in the PDF"><sup>[p. 245]</sup></a>

Now as usual, the coefficient for each power of $`y`$ must be zero for this sum to be zero for all $`y`$. Before shifting terms, we must examine the first few terms of this sum to learn about conditions on $`a_{0}`$ and $`a_{1}`$. The first term in the sum runs the risk of giving us a power of $`y`$ which cannot be canceled by the second term if $`k< 2`$. For $`k`$ = 0, there is no problem because the term is zero. For $`k`$ = 1 the term is (2$`\ell`$ + 2)$`y^{\ell - 1}`$ which cannot be made zero unless

```math
a_{1} = 0 .
```

This indicates that all the odd terms in the sum will be zero, as we will see from the recursion relation.

Now we will do the usual shift of the first term of the sum so that everything has a $`y^{\ell + k}`$ in it.

```math
{}_{\infty} \quad k \to k + 2
```

```math
\sum_{k = 0} \left[ a_{k + 2} ( k + 2 ) ( 2 \ell + k + 3 ) y^{\ell + k} + a_{k} \left[ \frac{2 E}{\hbar \omega} - ( 2 \ell + 2 k + 3 ) \right] y^{\ell + k} \right] = 0
```

```math
a_{k + 2} ( k + 2 ) ( 2 \ell + k + 3 ) + a_{k} \left[ \frac{2 E}{\hbar \omega} - ( 2 \ell + 2 k + 3 ) \right] = 0
```

```math
\begin{aligned} a_{k + 2} ( k + 2 ) ( 2 \ell + k + 3 ) &= - a_{k} \left[ \frac{2 E}{\hbar \omega} - ( 2 \ell + 2 k + 3 ) \right] \\ a_{k + 2} &= - \frac{\tfrac{2 E}{\hbar \omega} - ( 2 \ell + 2 k + 3 )}{( k + 2 ) ( 2 \ell + k + 3 )} a_{k} \end{aligned}
```

For large $`k`$,

```math
a_{k + 2} \approx \frac{2}{k} a_{k} ,
```

Which will cause the wave function to diverge. We must terminate the series for some $`k = n_{r} = 0 , 2 , 4 \dots`$, by requiring

```math
\frac{2 E}{\hbar \omega} - ( 2 \ell + 2 n_{r} + 3 ) = 0
```

```math
E = \left( n_{r} + \ell + \frac{3}{2} \right) \hbar \omega
```

These are the same energies as we found in Cartesian coordinates. Lets plug this back into the recursion relation.

```math
a_{k + 2} = - \frac{( 2 \ell + 2 n_{r} + 3 ) - ( 2 \ell + 2 k + 3 )}{( k + 2 ) ( 2 \ell + k + 3 )} a_{k}
```

```math
a_{k + 2} = \frac{2 ( k - n_{r} )}{( k + 2 ) ( 2 \ell + k + 3 )} a_{k}
```

To rewrite the series in terms of $`y^{2}`$ and let $`k`$ take on every integer value, we make the substitutions $`n_{r} \to 2 n_{r}`$ and $`k \to 2 k`$ in the recursion relation for $`a_{k + 1}`$ in terms of $`a_{k}`$.

<div align="right"><a href="book.pdf#page=245" target="_blank" title="This page in the PDF"><sup>[p. 245]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p246" href="book.pdf#page=246" target="_blank" title="This page in the PDF"><sup>[p. 246]</sup></a>

```math
a_{k + 1} = \frac{( k - n_{r} )}{( k + 1 ) ( \ell + k + 3 / 2 )} a_{k}
```

```math
R_{n_{r} \ell} = \sum_{k = 0}^{\infty} a_{k} y^{\ell + 2 k} e^{- y^{2} / 2}
```

```math
E = \left( 2 n_{r} + \ell + \frac{3}{2} \right) \hbar \omega
```

The table shows the quantum numbers for the states of each energy for our separation in spherical coordinates, and for separation in Cartesian coordinates. Remember that there are 2$`\ell`$ + 1 states with different $`z`$ components of angular momentum for the spherical coordinate states.

```math
\frac{E}{\frac{\hbar \hbar \omega \omega}{\tfrac{\frac{\frac{7 2}{2}}{1 1} \frac{9}{2}}{2} \hbar \omega} \hbar \omega} \tfrac{3}{5 2} \hbar \omega \quad \frac{n_{r} \ell}{\frac{1 0 , 0 2 0 1}{2 0 , 1 2 , 0 4} 1 1 , 0 3} 0 0 \quad \frac{n_{x} n_{y} n_{z}}{0 0 3 ( 3 \text{ perm } ) , 2 1 0 ( 6 \text{ perm } ) , 1 1 1 \frac{0 0 2 ( 3 \text{ perm } ) , 0 1 1 ( 3 \text{ perm } ) 0 0 1 ( 3 \text{ perm } )}{0 0 4 ( 3 ) , 3 1 0 ( 6 ) , 2 2 0 ( 3 ) , 2 1 1 ( 3 )}} 0 0 0 \quad \frac{N_{Spherical}}{1 0 \frac{6 3}{1 5}} 1 \quad \frac{N_{Cartesian}}{1 0 \frac{6 3}{1 5}} 1
```

The number of states at each energy matches exactly. The parities of the states also match. Remember that the parity is $`( - 1 )^{\ell}`$ for the angular momentum states and that it is $`( - 1 )^{n_{x} + n_{y} + n_{z}}`$ for the Cartesian states. If we were more industrious, we could verify that the wavefunctions in spherical coordinates are just linear combinations of the solutions in Cartesian coordinates.

---

[← 16 Hydrogen](16-hydrogen.md) · [18 Operators Matrices and Spin →](18-operators-matrices-and-spin.md)
