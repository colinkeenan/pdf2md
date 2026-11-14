[← 21 Addition of Angular Momentum](21-addition-of-angular-momentum.md) · [23 Fine Structure in Hydrogen →](23-fine-structure-in-hydrogen.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p326" href="book.pdf#page=326" target="_blank" title="This page in the PDF"><sup>[p. 326]</sup></a>

# 22 Time Independent Perturbation Theory

Perturbation Theory is developed to deal with **small corrections to problems which we have** **solved exactly**, like the harmonic oscillator and the hydrogen atom. We will make a series expansion of the energies and eigenstates for cases where there is only a small correction to the exactly soluble problem.

First order perturbation theory will give quite accurate answers if the energy shifts calculated are (nonzero and) much smaller than the zeroth order energy differences between eigenstates. If the first order correction is zero, we will go to second order. If the eigenstates are (nearly) degenerate to zeroth order, we will diagonalize the full Hamiltonian using only the (nearly) degenerate states.

Cases in which the Hamiltonian is time dependent will be handled later.

This material is covered in **Gasiorowicz Chapter 16,** in **Cohen-Tannoudji et al. Chapter XI,** and in Griffiths Chapters 6 and 7.

# <a id="s-22-1-the-perturbation-series"></a>22.1 The Perturbation Series

Assume that the energy eigenvalue problem for the Hamiltonian $`H_{0}`$ can be **solved exactly**

```math
H_{0} \varphi_{n} = E_{n}^{( 0 )} \varphi_{n}
```

but that the true Hamiltonian has a small additional term or **perturbation** $`H_{1}`$.

```math
H = H_{0} + H_{1}
```

The Schrödinger equation for the **full problem** is

```math
( H_{0} + H_{1} ) \psi_{n} = E_{n} \psi_{n}
```

Presumably this full problem, like most problems, cannot be solved exactly. To solve it using a **perturbation series,** we will expand both our energy eigenvalues and eigenstates in powers of the small perturbation.

```math
E_{n} = E_{n}^{( 0 )} + E_{n}^{( 1 )} + E_{n}^{( 2 )} + \dots
```

```math
\psi_{n} = N \left( \varphi_{n} + \sum_{k \neq n} c_{nk} \varphi_{k} \right)
```

```math
c_{nk} = c_{nk}^{( 1 )} + c_{nk}^{( 2 )} + \dots
```

where the superscript (0), (1), (2) are the zeroth, first, and second order terms in the series. $`N`$ is there to keep the wave function normalized but will not play an important role in our results.

By solving the Schrödinger equation at each order of the perturbation series, we **compute the** **corrections to the energies and eigenfunctions.** (see section 22.4.1)

```math
E_{n}^{( 1 )} = \langle \varphi_{n} | H_{1} | \varphi_{n} \rangle
```

<div align="right"><a href="book.pdf#page=326" target="_blank" title="This page in the PDF"><sup>[p. 326]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p327" href="book.pdf#page=327" target="_blank" title="This page in the PDF"><sup>[p. 327]</sup></a>

```math
c_{nk}^{( 1 )} = \frac{\langle \varphi_{k} | H_{1} | \varphi_{n} \rangle}{E_{n}^{( 0 )} - E_{k}^{( 0 )}}
```

```math
E_{n}^{( 2 )} = \sum_{k \neq n} \frac{| \langle \varphi_{k} | H_{1} | \varphi_{n} \rangle |^{2}}{E_{n}^{( 0 )} - E_{k}^{( 0 )}}
```

audio

So the **first order correction to the energy** of the $`n^{th}`$ eigenstate, $`E_{n}^{( 1 )}`$, is just the expectation value of the perturbation in the unperturbed state. The first order admixture of $`\varphi_{k}`$ in $`\psi_{n} , c_{nk}^{( 1 )}`$, depends on a matrix element and the energy difference between states. The **second order correc-** **tion to the energy**, $`E_{n}^{( 2 )}`$, has a similar dependence. Note that the higher order corrections may not be small if states are nearby in energy.

The application of the first order perturbation equations is quite simple in principal. The actual calculation of the matrix elements depends greatly on the problem being solved.

\* See Example 22.3.1: H.O. with anharmonic perturbation $`( ax^{4} ) . *`$

Sometimes the first order correction to the energy is zero. Then we will need to use the second order term $`E_{n}^{( 2 )}`$ to estimate the correction. This is true when we apply an electric field to a hydrogen atom.

\* See Example 22.3.2: Hydrogen Atom in a E-field, the Stark Effect.\*

We will exercise the use of perturbation theory in section 23 when we compute the fine structure, and other effects in Hydrogen.

# <a id="s-22-2-degenerate-state-perturbation-theory"></a>22.2 Degenerate State Perturbation Theory

The perturbation expansion has a problem for states very close in energy. The energy difference in the denominators goes to zero and the **corrections are no longer small.** The series does not converge. We can very effectively solve this problem by **treating all the (nearly) degenerate** **states like we did** $`\varphi_{n}`$ in the regular perturbation expansion. That is, the zeroth order state will be allowed to be an arbitrary linear combination of the degenerate states and the eigenvalue problem will be solved.

Assume that two or more states are (nearly) degenerate. Define $`\mathcal{N}`$ to be the set of those nearly degenerate states. Choose a set of basis state in $`\mathcal{N}`$ which are orthonormal

```math
\langle \varphi^{( j )} | \varphi^{( i )} \rangle = \delta_{ji}
```

where $`i`$ and $`j`$ are in the set $`\mathcal{N}`$. We will use the indices $`i`$ and $`j`$ to label the states in $`\mathcal{N}`$.

By looking at the zeroth and first order terms in the Schrödinger equation and dotting it into one of the degenerate states $`\varphi^{( j )}`$, we **derive** (see section 22.4.2) the energy equation for first order (nearly)<span class="page" style="float:left;width:100%;margin:0.4em 0 0.6em"><span style="display:block;text-align:right"><a href="book.pdf#page=327" target="_blank" title="This page in the PDF"><sup>[p. 327]</sup></a></span><span style="display:block;height:2px;background:currentColor;opacity:0.25"></span><a id="p328" href="book.pdf#page=328" target="_blank" title="This page in the PDF"><sup>[p. 328]</sup></a></span> degenerate state perturbation theory

```math
\sum_{i \in \mathcal{N}} \langle \varphi^{( j )} | H_{0} + H_{1} | \varphi^{( i )} \rangle \alpha_{i} = E\alpha_{j} ,
```

This is an eigenvalue equation with as many solutions as there are degnerate states in our set. audio

We recognize this as simply the (matrix) energy eigenvalue equation limited the list of degenerate states. We solve the equation to get the energy eigenvalues and energy eigenstates, correct to first order.

Written as a matrix, the equation is

```math
\begin{pmatrix} H_{1 1} & H_{1 2} & \dots & H_{1 n} \\ H_{2 1} & H_{2 2} & \dots & H_{2 n} \\ \dots & \dots & \dots & \dots \\ H_{n 1} & H_{n 2} & \dots & H_{nn} \end{pmatrix} \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \dots \\ \alpha_{n} \end{pmatrix} = E \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \dots \\ \alpha_{n} \end{pmatrix}
```

where $`H_{ji} = \langle \varphi^{( j )} | H_{0} + H_{1} | \varphi^{( i )} \rangle`$ is the matrix element of the full Hamiltonian. If there are n nearly degenerate states, there are n solutions to this equation.

The Stark effect for the (principle quantum number) n=2 states of hydrogen requires the use of degenerate state perturbation theory since there are four states with (nearly) the same energies. For our first calculation, we will ignore the hydrogen fine structure and assume that the four states are exactly degenerate, each with unperturbed energy of $`E_{0}`$. That is $`H_{0} \varphi_{2 \ell m} = E_{0} \varphi_{2 \ell m}`$. The degenerate states $`\varphi_{2 0 0} , \varphi_{2 1 1} , \varphi_{2 1 0}`$, and $`\varphi_{2 1 ( - 1 )}`$.

\* See Example 22.3.3: The Stark Effect for n=2 States.\*

The perturbation due to an electric field in the z direction is $`H_{1}`$ = +$`e \mathcal{E} z`$. The linear combinations that are found to diagonalize the full Hamiltonian in the subspace of degenerate states are: $`\varphi_{2 1 1}`$, $`\varphi_{2 1 ( - 1 )}`$ and $`\tfrac{1}{\sqrt{2}} ( \varphi_{2 0 0} \pm \varphi_{2 1 0} )`$ with energies of $`E_{2} , E_{2}`$, and $`E_{2} \mp 3 e \mathcal{E} a_{0}`$.

# <a id="s-22-3-examples"></a>22.3 Examples

## <a id="s-22-3-1-h-o-with-anharmonic-perturbation-ax-4"></a>22.3.1 H.O. with anharmonic perturbation $`\boldsymbol{(} ax^{4} \boldsymbol{)}`$.

We add an anharmonic perturbation to the Harmonic Oscillator problem.

```math
H_{1} = ax^{4}
```

Since this is a symmetric perturbation we expect that it will give a nonzero result in first order perturbation theory. First, write x in terms of $`A`$ and $`A^{\dagger}`$ and compute the expectation value as we have done before.

```math
\Delta E_{n}^{( 1 )} = a \langle n | x^{4} | n \rangle = \frac{a \hbar^{2}}{4 m^{2} \omega^{2}} \langle n | ( A + A^{\dagger} )^{4} | n \rangle
```

```math
= \frac{a \hbar^{2}}{4 m^{2} \omega^{2}} \langle n | ( AAA^{\dagger} A^{\dagger} + AA^{\dagger} AA^{\dagger} + AA^{\dagger} A^{\dagger} A + A^{\dagger} AAA^{\dagger} + A^{\dagger} AA^{\dagger} A + A^{\dagger} A^{\dagger} AA ) | n \rangle
```

```math
= \frac{a \hbar^{2}}{4 m^{2} \omega^{2}} \left[ ( n + 1 ) ( n + 2 ) + ( n + 1 )^{2} + n ( n + 1 ) + n ( n + 1 ) + n^{2} + n ( n - 1 ) \right]
```

```math
= \frac{3 a \hbar^{2}}{4 m^{2} \omega^{2}} ( 2 n^{2} + 2 n + 1 )
```

<div align="right"><a href="book.pdf#page=328" target="_blank" title="This page in the PDF"><sup>[p. 328]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p329" href="book.pdf#page=329" target="_blank" title="This page in the PDF"><sup>[p. 329]</sup></a>

## <a id="s-22-3-2-hydrogen-atom-ground-state-in-a-e-field-the-stark-effect"></a>22.3.2 Hydrogen Atom Ground State in a E-field, the Stark Effect.

We have solved the Hydrogen problem with the following Hamiltonian.

```math
H_{0} = \frac{p^{2}}{2 \mu} - \frac{Ze^{2}}{r}
```

Now we want to find the correction to that solution if an **Electric field is applied to the atom**. We choose the axes so that the Electric field is in the z direction. The **perturbtion** is then.

```math
H_{1} = e \mathcal{E} z
```

It is typically a small perturbation. **For non-degenerate states**, the first order correction to the energy is zero because the expectation value of z is an odd function.

```math
E_{nlm}^{( 1 )} = e \mathcal{E} \langle \varphi_{nlm} | z | \varphi_{nlm} \rangle = 0
```

We therefore need to calculate the **second order correction**. This involves a sum over all the other states.

```math
E_{1 0 0}^{( 2 )} = e^{2} \mathcal{E}^{2} \sum_{nlm \neq 1 0 0} \frac{| \langle \varphi_{nlm} | z | \varphi_{1 0 0} \rangle |^{2}}{E_{1}^{( 0 )} - E_{n}^{( 0 )}}
```

We need to compute all the matrix elements of z between the ground state and the other Hydrogen states.

```math
\langle \varphi_{nlm} | z | \varphi_{1 0 0} \rangle = \int d^{3} rR_{nl}^{*} ( r \cos \theta ) R_{1 0} Y_{lm}^{*} Y_{0 0}
```

We can do the angular integral by converting the $`\cos \theta`$ term into a spherical harmonic.

```math
Y_{0 0} \cos \theta = \frac{1}{\sqrt{4 \pi}} \sqrt{\frac{4 \pi}{3}} Y_{1 0}
```

The we can just use the orthonormality of the spherical harmonics to do the angular integral, leaving us with a radial integral to do.

```math
\langle \varphi_{nlm} | z | \varphi_{1 0 0} \rangle = \frac{1}{\sqrt{3}} \int r^{3} drR_{nl}^{*} R_{1 0} \int d \Omega Y_{lm}^{*} Y_{1 0}
```

```math
= \frac{\delta_{\ell 1} \delta_{m 0}}{\sqrt{3}} \int r^{3} R_{nl}^{*} R_{1 0} dr
```

The radial part of the integral can be done with some work, yielding.

```math
| \langle \varphi_{nlm} | z | \varphi_{1 0 0} \rangle |^{2} = \frac{1}{3} \frac{2^{8} n^{7} ( n - 1 )^{2 n - 5}}{( n + 1 )^{2 n + 5}} a_{0}^{2} \delta_{\ell 0} \delta m 0 \equiv f ( n ) a_{0}^{2} \delta_{\ell 0} \delta m 0
```

We put this back into the sum. The Kronecker deltas eliminate the sums over $`\ell`$ and $`m`$. We write the energy denominators in terms of the Bohr radius.

```math
E_{1 0 0}^{( 2 )} = e^{2} \mathcal{E}^{2} \sum_{n = 2}^{\infty} \frac{f ( n ) a_{0}^{2}}{\tfrac{e^{2}}{2 a_{0}}^{-} + \tfrac{e^{2}}{2 a_{0} n^{2}}}
```

<div align="right"><a href="book.pdf#page=329" target="_blank" title="This page in the PDF"><sup>[p. 329]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p330" href="book.pdf#page=330" target="_blank" title="This page in the PDF"><sup>[p. 330]</sup></a>

```math
= a_{0}^{3} \mathcal{E}^{2} \sum_{n = 2}^{\infty} \frac{2 f ( n )}{- 1 + \tfrac{1}{n^{2}}}
```

```math
= - 2 a_{0}^{3} \mathcal{E}^{2} \sum_{n = 2}^{\infty} \frac{n^{2} f ( n )}{n^{2} - 1}
```

This is all a little dissatisfying because we had to insert the general formula for the radial integral and it just goes into a nasty sum. In fact, we could just start with the first few states to get a good idea of the size of the effect. The result comes out to be.

```math
E_{1 0 0}^{( 2 )} = - 2 a_{0}^{3} \mathcal{E}^{2} ( 0 . 7 4 + 0 . 1 0 + \dots ) = - 2 . 2 5 a_{0}^{3} \mathcal{E}^{2}
```

The first two terms of the sum get us pretty close to the right answer. We could have just done those radial integrals.

Now we compute $`d`$, the **electric dipole moment** of the atom which is induced by the electric field.

```math
d = - \frac{\partial \Delta E}{\partial \mathcal{E}} = 4 ( 1 . 1 2 5 ) a_{0}^{3} \mathcal{E}
```

The dipole moment is proportional to the Electric field, indicating that it is induced. The E field induces the dipole moment, then the dipole moment interacts with the E field causing a energy shift. This indicates why the energy shift is second order.

## <a id="s-22-3-3-the-stark-effect-for-n-2-hydrogen"></a>22.3.3 The Stark Effect for n=2 Hydrogen.

The Stark effect for the n=2 states of hydrogen requires the use of degenerate state perturbation theory since there are four states with (nearly) the same energies. For our first calculation, we will ignore the hydrogen fine structure and assume that the four states are exactly degenerate, each with unperturbed energy of $`E_{0}`$. That is $`H_{0} \varphi_{2 \ell m} = E_{0} \varphi_{2 \ell m}`$. The degenerate states $`\varphi_{2 0 0} , \varphi_{2 1 1} , \varphi_{2 1 0}`$, and $`\varphi_{2 1 ( - 1 )}`$.

The perturbation due to an electric field in the z direction is $`H_{1}`$ = +$`e \mathcal{E} z`$. So our first order degenerate state perturbation theory equation is

```math
\sum_{i} \alpha_{i} \left\langle \varphi^{( j )} | H_{0} + e \mathcal{E} z | \varphi^{( i )} \right\rangle = ( E_{0} + E^{( 1 )} ) \alpha_{j} .
```

This is esentially a 4X4 matrix eigenvalue equation. There are 4 eigenvalues $`( E_{0} + E^{( 1 )} )`$, distinguished by the index $`n`$.

Because of the exact degeneracy $`( H_{0} \varphi^{( j )} = E_{0} \varphi^{( j )} ) , H_{0}`$ and $`E_{0}`$ can be eliminated from the equation.

```math
\sum_{i} \alpha_{i} ( E_{0} \delta_{ij} + \left\langle \varphi^{( j )} | e \mathcal{E} z | \varphi^{( i )} \right\rangle ) = ( E_{0} + E^{( 1 )} ) \alpha_{j}
```

```math
E_{0} \alpha_{j} + \sum_{i} \alpha_{i} \left\langle \varphi^{( j )} | e \mathcal{E} z | \varphi^{( i )} \right\rangle = E_{0} \alpha_{j} + E^{( 1 )} \alpha_{j}
```

```math
\sum \alpha_{i} \left\langle \varphi^{( j )} | e \mathcal{E} z | \varphi^{( i )} \right\rangle = E^{( 1 )} \alpha_{j}
```

<div align="right"><a href="book.pdf#page=330" target="_blank" title="This page in the PDF"><sup>[p. 330]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p331" href="book.pdf#page=331" target="_blank" title="This page in the PDF"><sup>[p. 331]</sup></a>

This is just the eigenvalue equation for $`H_{1}`$ which we can write out in (pseudo)matrix form

```math
\left( H_{1} \right) \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \alpha_{3} \\ \alpha_{4} \end{pmatrix} = E^{( 1 )} \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \alpha_{3} \\ \alpha_{4} \end{pmatrix}
```

Now, in fact, most of the matrix elements of $`H_{1}`$ are zero. We will show that because $`[ L_{z} ,z ]`$ = 0, that all the matrix elements between states of unequal $`m`$ are zero. Another way of saying this is that the operator z doesn’t “change” $`m`$. Here is a little proof.

```math
\langle Y_{lm} | [ L_{z} ,z ] | Y_{l^{\prime} m^{\prime}} \rangle = 0 = ( m - m^{\prime} ) \langle Y_{lm} | z | Y_{l^{\prime} m^{\prime}} \rangle
```

This implies that $`\langle Y_{lm} | z | Y_{l^{\prime} m^{\prime}} \rangle`$ = 0 unless $`m = m^{\prime}`$.

Lets define the one remaining nonzero (real) matrix element to be $`\gamma`$.

```math
\gamma = e \mathcal{E} \langle \varphi_{2 0 0} | z | \varphi_{2 1 0} \rangle
```

The equation (labeled with the basis states to define the order) is.

```math
\frac{\varphi \varphi_{2 0 0 2 1 1}}{\varphi \varphi_{2 1 0 2 1 - 1}} \begin{pmatrix} 0 & 0 & \gamma & 0 \\ 0 & 0 & 0 & 0 \\ \gamma & 0 & 0 & 0 \\ 0 & 0 & 0 & 0 \end{pmatrix} \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \alpha_{3} \\ \alpha_{4} \end{pmatrix} = E^{( 1 )} \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \\ \alpha_{3} \\ \alpha_{4} \end{pmatrix}
```

We can see by inspection that the eigenfunctions of this operator are $`\varphi_{2 1 1} , \varphi_{2 1 - 1}`$, and $`\tfrac{1}{2}^{\surd} ( \varphi_{2 0 0} \pm \varphi_{2 1 0} )`$ with eigenvalues (of $`H_{1} )`$ of 0, 0, and $`\pm \gamma`$.

What remains is to compute $`\gamma`$. Recall $`Y_{0 0} = \tfrac{1}{\sqrt{4 \pi}}`$ and $`Y_{1 0} = \sqrt{\frac{3}{4 \pi}} \cos \theta`$.

```math
\gamma = e \mathcal{E} \int ( 2 a_{0} )^{- 3 / 2} 2 \left( 1 - \frac{r}{2 a_{0}} \right) e^{- r/ 2 a_{0}} Y_{0 0} z ( 2 a_{0} )^{- 3 / 2} \frac{1}{\sqrt{3}} \left( \frac{r}{a_{0}} \right) e^{- r/ 2 a_{0}} Y_{1 0} d^{3} r
```

```math
= 2 e \mathcal{E} ( 2 a_{0} )^{- 3} \frac{1}{\sqrt{3}} \int r^{3} d^{3} r \left( 1 - \frac{r}{2 a_{0}} \right) \left( \frac{r}{a_{0}} \right) e^{- r/a_{0}} \int \frac{1}{\sqrt{4 \pi}} \cos \theta Y_{1 0} d \Omega
```

```math
= 2 eE ( 2 )^{- 3} \frac{1}{\sqrt{3}} \frac{1}{\sqrt{3}} \int_{0}^{\infty} \left( \frac{r^{4}}{a_{0}^{4}} - \frac{r^{5}}{2 a_{0}^{5}} \right) e^{- r/a_{0}} dr
```

```math
= \frac{a_{0} e \mathcal{E}}{1 2} \left[ \int_{0}^{\infty} x^{4} e^{- x} dx - \frac{1}{2} \int_{0}^{\infty} x^{5} e^{- x} dx \right]
```

```math
= \frac{a_{0} e \mathcal{E}}{1 2} \left[ 4 \cdot 3 \cdot 2 \cdot 1 - \frac{5 \cdot 4 \cdot 3 \cdot 2 \cdot 1}{2} \right]
```

```math
= \frac{a_{0} e \mathcal{E}}{1 2} ( - 3 6 )
```

```math
= - 3 e \mathcal{E} a_{0} \quad \Rightarrow \quad E^{( 1 )} = \mp 3 e \mathcal{E} a_{0}
```

This is first order in the electric field, as we would expect in first order (degenerate) perturbation theory.

If the states are not exactly degenerate, we have to leave in the diagonal terms of $`H_{0}`$. Assume that the energies of the two (mixed) states are $`E_{0} \pm`$ ∆, where ∆ comes from some other perturbation, like the hydrogen fine structure. (The $`\varphi_{2 1 1}`$ and $`\varphi_{2 1 ( - 1 )}`$ are still not mixed by the electric field.)

```math
\left( E_{0} \gamma - \Delta \quad E_{0} + \Delta \gamma \right) \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \end{pmatrix} = E \begin{pmatrix} \alpha_{1} \\ \alpha_{2} \end{pmatrix}
```

<div align="right"><a href="book.pdf#page=331" target="_blank" title="This page in the PDF"><sup>[p. 331]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p332" href="book.pdf#page=332" target="_blank" title="This page in the PDF"><sup>[p. 332]</sup></a>

```math
E = E_{0} \pm \sqrt{\gamma^{2} + \Delta^{2}}
```

This is OK in both limits, ∆ $`\gg \gamma`$, and $`\gamma \gg`$ ∆. It is also correct when the two corrections are of the same order.

# <a id="s-22-4-derivations-and-computations"></a>22.4 Derivations and Computations

## <a id="s-22-4-1-derivation-of-1st-and-2nd-order-perturbation-equations"></a>22.4.1 Derivation of 1st and 2nd Order Perturbation Equations

To keep track of powers of the perturbation in this derivation we will make the substitution $`H_{1} \to`$ $`\lambda H_{1}`$ where $`\lambda`$ is assumed to be a small parameter in which we are making the series expansion of our energy eigenvalues and eigenstates. It is there to do the book-keeping correctly and can go away at the end of the derivations.

To solve the problem using a **perturbation series**, we will expand both our energy eigenvalues and eigenstates in powers of $`\lambda`$.

```math
E_{n} = E_{n}^{( 0 )} + \lambda E_{n}^{( 1 )} + \lambda^{2} E_{n}^{( 2 )} + \dots
```

```math
\psi_{n} = N ( \lambda ) \left( \varphi_{n} + \sum_{k \neq n} c_{nk} ( \lambda ) \varphi_{k} \right)
```

```math
c_{nk} ( \lambda ) = \lambda c_{nk}^{( 1 )} + \lambda^{2} c_{nk}^{( 2 )} + \dots
```

The **full Schrödinger equation** is

```math
( H_{0} + \lambda H_{1} ) \left( \varphi_{n} + \sum_{k \neq n} c_{nk} ( \lambda ) \varphi_{k} \right) = ( E_{n}^{( 0 )} + \lambda E_{n}^{( 1 )} + \lambda^{2} E_{n}^{( 2 )} + \dots ) \left( \varphi_{n} + \sum_{k \neq n} c_{nk} ( \lambda ) \varphi_{k} \right)
```

where the $`N ( \lambda )`$ has been factored out on both sides. For this equation to hold as we vary $`\lambda`$, it must hold for each power of $`\lambda`$. We will investigate the first three terms.

```math
\lambda^{0} \quad H_{0} \varphi_{n} = E_{n}^{( 0 )} \varphi_{n}
```

```math
\lambda^{1} \quad \lambda H_{1} \varphi_{n} + H_{0} \lambda \sum_{k \neq n} c_{nk}^{( 1 )} \varphi_{k} = \lambda E_{n}^{( 1 )} \varphi_{n} + \lambda E_{n}^{( 0 )} \sum_{k \neq n} c_{nk}^{( 1 )} \varphi_{k}
```

```math
\lambda^{2} \quad H_{0} \sum_{k \neq n} \lambda^{2} c_{nk}^{( 2 )} \varphi_{k} + \lambda H_{1} \sum_{k \neq n} \lambda c_{nk}^{( 1 )} \varphi_{k} = E_{n}^{( 0 )} \sum_{k \neq n} \lambda^{2} c_{nk}^{( 2 )} \varphi_{k} + \lambda E_{n}^{( 1 )} \sum_{k \neq n} \lambda c_{nk}^{( 1 )} \varphi_{k} + \lambda^{2} E_{n}^{( 2 )} \varphi_{n}
```

The zero order term is just the solution to the unperturbed problem so there is no new information there. The other two terms contain linear combinations of the orthonormal functions $`\varphi_{i}`$. This means we can dot the equations into each of the $`\varphi_{i}`$ to get information, much like getting the components of a vector individually. Since $`\varphi_{n}`$ is treated separately in this analysis, we will dot the equation into $`\varphi_{n}`$ and separately into all the other functions $`\varphi_{k}`$.

The first order equation dotted into $`\varphi_{n}`$ yields

```math
\langle \varphi_{n} | \lambda H_{1} | \varphi_{n} \rangle = \lambda E_{n}^{( 1 )}
```

<div align="right"><a href="book.pdf#page=332" target="_blank" title="This page in the PDF"><sup>[p. 332]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p333" href="book.pdf#page=333" target="_blank" title="This page in the PDF"><sup>[p. 333]</sup></a>

and dotted into $`\varphi_{k}`$ yields

```math
\langle \varphi_{k} | \lambda H_{1} | \varphi_{n} \rangle + E_{k}^{( 0 )} \lambda c_{nk}^{( 1 )} = E_{n}^{( 0 )} \lambda c_{nk}^{( 1 )} .
```

From these it is simple to derive **the first order corrections**

```math
\lambda E_{n}^{( 1 )} = \langle \varphi_{n} | \lambda H_{1} | \varphi_{n} \rangle
```

```math
\lambda c_{nk}^{( 1 )} = \frac{\langle \varphi_{k} | \lambda H_{1} | \varphi_{n} \rangle}{E_{n}^{( 0 )} - E_{k}^{( 0 )}}
```

The second order equation projected on $`\varphi_{n}`$ yields

```math
\sum_{k \neq n} \lambda c_{nk}^{( 1 )} \langle \varphi_{n} | \lambda H_{1} | \varphi_{k} \rangle = \lambda^{2} E_{N}^{( 2 )} .
```

We will not need the projection on $`\varphi_{k}`$ but could proceed with it to get the second order correction to the wave function, if that were needed. Solving for the **second order correction to the energy** and substituting for $`c_{nk}^{( 1 )}`$, we have

```math
\lambda^{2} E_{n}^{( 2 )} = \sum_{k \neq n} \frac{| \langle \varphi_{k} | \lambda H_{1} | \varphi_{n} \rangle |^{2}}{E_{n}^{( 0 )} - E_{k}^{( 0 )}} .
```

The **normalization factor** $`N ( \lambda )`$ played no role in the solutions to the Schrödinger equation since that equation is independent of normalization. We do need to go back and check whether the first order corrected wavefunction needs normalization.

```math
\tfrac{1}{N ( \lambda )^{2}} = \langle \varphi_{n} + \sum_{k \neq n} \lambda c_{nk}^{( 1 )} \varphi_{k} | \varphi_{n} + \sum_{k \neq n} \lambda c_{nk}^{( 1 )} \varphi_{k} \rangle = 1 + \sum_{k \neq n} \lambda^{2} | c_{nk}^{( 1 )} |^{2}
```

```math
N ( \lambda ) \approx 1 - \tfrac{1}{2}_{k} \sum_{\not = n} \lambda^{2} | c_{nk}^{( 1 )} |^{2}
```

<div align="center">

The correction is of order $`\lambda^{2}`$ and **can be neglected** at this level of approximation.

</div>

These results are rewritten with all the $`\lambda`$ removed in section 22.1.

## <a id="s-22-4-2-derivation-of-1st-order-degenerate-perturbation-equations"></a>22.4.2 Derivation of 1st Order Degenerate Perturbation Equations

To deal with the problem of degenerate states, we will allow an arbitrary linear combination of those states at zeroth order. In the following equation, the sum over $`i`$ is the sum over all the states degenerate with $`\varphi_{n}`$ and the sum over k runs over all the other states.

```math
\psi_{n} = N ( \lambda ) \left( \sum_{i \in \mathcal{N}} \alpha_{i} \varphi^{( i )} + \sum_{k 6 \in \mathcal{N}} \lambda c_{nk}^{( 1 )} \varphi_{k} + \dots \right)
```

where $`\mathcal{N}`$ is the set of zeroth order states which are (nearly) degenerate with $`\varphi_{n}`$. We will only go to first order in this derivation and we will use $`\lambda`$ as in the previous derivation to keep track of the order in the perturbation.

<div align="right"><a href="book.pdf#page=333" target="_blank" title="This page in the PDF"><sup>[p. 333]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p334" href="book.pdf#page=334" target="_blank" title="This page in the PDF"><sup>[p. 334]</sup></a>

The full Schrödinger equation is.

```math
( H_{0} + \lambda H_{1} ) \left( \sum_{i \in \mathcal{N}} \alpha_{i} \varphi^{( i )} + \sum_{k 6 \in \mathcal{N}} c_{nk} ( \lambda ) \varphi_{k} \right) = ( E_{n}^{( 0 )} + \lambda E^{( 1 )} + \dots ) \left( \sum_{i \in \mathcal{N}} \alpha_{i} \varphi^{( i )} + \sum_{k 6 \in \mathcal{N}} c_{nk} ( \lambda ) \varphi_{k} \right)
```

If we keep the zeroth and first order terms, we have

```math
( H_{0} + \lambda H_{1} ) \sum_{i \in \mathcal{N}} \alpha_{i} \varphi^{( i )} + H_{0} \sum_{k 6 \in \mathcal{N}} \lambda c_{nk}^{( 1 )} \varphi_{k} = ( E_{n}^{( 0 )} + \lambda E^{( 1 )} ) \sum_{i \in \mathcal{N}} \alpha_{i} \varphi^{( i )} + E_{n}^{( 0 )} \sum_{k 6 \in \mathcal{N}} \lambda c_{nk}^{( 1 )} \varphi_{k} .
```

Projecting this onto one of the degenerate states $`\varphi^{( j )}`$, we get

```math
\sum_{i \in \mathcal{N}} \langle \varphi^{( j )} | H_{0} + \lambda H_{1} | \varphi^{( i )} \rangle \alpha_{i} = ( E_{n}^{( 0 )} + \lambda E^{( 1 )} ) \alpha_{j} .
```

By putting both terms together, our calculation gives us the full energy to first order, not just the correction. It is useful both for degenerate states and for nearly degenerate states. The result may be simplified to

```math
\sum_{i \in \mathcal{N}} \langle \varphi^{( j )} | H | \varphi^{( i )} \rangle \alpha_{i} = E\alpha_{j} .
```

This is just the standard eigenvalue problem for the full Hamiltonian in the **subspace of (nearly)** **degenerate states.**

# <a id="s-22-5-homework-problems"></a>22.5 Homework Problems

1. An electron is bound in a harmonic oscillator potential $`V_{0} = \tfrac{1}{2} m\omega^{2} x^{2}`$. Small electric fields in the $`x`$ direction are applied to the system. Find the lowest order nonzero shifts in the energies of the ground state and the first excited state if a constant field $`E_{1}`$ is applied. Find the same shifts if a field $`E_{1} x^{3}`$ is applied.

2. A particle is in a box from $`- a`$ to $`a`$ in one dimension. A small additional potential $`V_{1} = \lambda`$ cos$`( \tfrac{\pi x}{2 b} )`$ is applied. Calculate the energies of the first and second excited states in this new potential.

3. The proton in the hydrogen nucleus is not really a point particle like the electron is. It has a complicated structure, but, a good approximation to its charge distribution is a uniform charge density over a sphere of radius 0.5 fermis. Calculate the effect of this potential change for the energy of the ground state of hydrogen. Calculate the effect for the $`n`$ = 2 states.

4. Consider a two dimensional harmonic oscillator problem described by the Hamiltonian $`H_{0} = \tfrac{p_{x}^{2} + p_{y}^{2}}{2 m} + \tfrac{1}{2} m\omega^{2} ( x^{2} + y^{2} )`$. Calculate the energy shifts of the ground state and the degenerate first excited states, to first order, if the additional potential $`V`$ = 2$`\lambda xy`$ is applied. Now solve the problem exactly. Compare the exact result for the ground state to that from second order perturbation theory.

5. Prove that $`\sum ( E_{n} - E_{a} ) | \langle n | x | a \rangle |^{2} = \tfrac{\hbar^{2}}{2 m}`$ by starting from the expectation value of the commu-

```math
n
```

tator $`[ p,x ]`$ in the state $`a`$ and summing over all energy eigenstates. Assume $`p = m \tfrac{dx}{dt}`$ and write $`\frac{dx}{dt}`$ in terms of the commutator $`[ H,x ]`$ to get the result.

6. If the general form of the spin-orbit coupling for a particle of mass $`m`$ and spin $`\vec{S}`$ moving in a potential $`V ( r )`$ is $`H_{SO} = \tfrac{1}{2 m^{2} c^{2}} \vec{L} \cdot \vec{S} \tfrac{1}{r} \tfrac{dV ( r )}{dr}`$, what is the effect of that coupling on the spectrum of a three dimensional harmonic oscillator? Compute the relativistic correction for the ground state of the three dimensional harmonic oscillator.

<div align="right"><a href="book.pdf#page=334" target="_blank" title="This page in the PDF"><sup>[p. 334]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p335" href="book.pdf#page=335" target="_blank" title="This page in the PDF"><sup>[p. 335]</sup></a>

# <a id="s-22-6-sample-test-problems"></a>22.6 Sample Test Problems

1. Assume an electron is bound to a heavy positive particle with a harmonic potential $`V ( x ) =`$

$`\tfrac{1}{2} m\omega^{2} x^{2}`$. Calculate the energy shifts to all the energy eigenstates in an electric field $`E`$ (in the $`x`$ direction).

2. Find the energies of the $`n`$ = 2 hydrogen states in a strong uniform electric field in the z-direction. (Note, since spin plays no role here there are just 4 degenerate states. Ignore the fine structure corrections to the energy since the E-field is strong. Remember to use the fact that $`[ L_{z} ,z ]`$ = 0. If you are pressed for time, don’t bother to evaluate the radial integrals.)

3. An electron is in a three dimensional harmonic oscillator potential $`V ( r ) = \tfrac{1}{2} m\omega^{2} r^{2}`$. A small electric field, of strength $`E_{z}`$, is applied in the $`z`$ direction. Calculate the lowest order nonzero correction to the ground state energy.

4. Hydrogen atoms in the $`n`$ = 2 state are put in a strong Electric field. Assume that the 2s and 2p states of Hydrogen are degenerate and spin is not important. Under these assumptions, there are 4 states: the 2s and three 2p states. Calculate the shifts in energy due to the E-field and give the states that have those energies. Please work out the problem in principle before attempting any integrals.

---

[← 21 Addition of Angular Momentum](21-addition-of-angular-momentum.md) · [23 Fine Structure in Hydrogen →](23-fine-structure-in-hydrogen.md)
