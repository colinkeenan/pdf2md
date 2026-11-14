[← 5 Wave Packets](05-wave-packets.md) · [7 The Schrödinger Equation →](07-the-schrodinger-equation.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p117" href="book.pdf#page=117" target="_blank" title="This page in the PDF"><sup>[p. 117]</sup></a>

# 6 Operators

Operators will be used to help us derive a differential equation that our wave-functions must satisfy. They will also be used in almost any Quantum Physics calculation.

An example of a linear operator (See section 7.5.1) is a simple differential operator like $`\frac{\partial}{\partial x}`$ , which we understand to differentiate everything to the right of it with respect to $`x`$.

# <a id="s-6-1-operators-in-position-space"></a>6.1 Operators in Position Space

To find operators for physical variables in position space, we will look at wave functions with definite momentum. Our state of definite momentum $`p_{0}`$ (and definite energy $`E_{0} )`$ is

```math
u_{p 0} ( x,t ) = \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar} .
```

We can build any other state from superpositions of these states using the Fourier Transform.

```math
\psi ( x,t ) = \int_{-\infty}^{\infty} \varphi ( p ) u_{p} ( x,t ) dp
```

## <a id="s-6-1-1-the-momentum-operator"></a>6.1.1 The Momentum Operator

We determine the momentum operator by requiring that, when we operate with $`p_{x}^{( op )}`$ on $`u_{p 0} ( x,t )`$, we get $`p_{0}`$ times the same wave function.

```math
p^{( op )} u_{p 0} ( x,t ) = p_{0} u_{p 0} ( x,t )
```

This means that for these definite momentum states, multiplying by $`p_{x}^{( op )}`$ is the same as multiplying by the variable $`p`$. We find that this is true for the following **momentum operator**.

```math
p^{( op )} = \frac{\hbar}{i} \frac{\partial}{\partial x}
```

We can verify (See section 6.6.1) that this works by explicit calculation.

If we take our momentum operator and **act on a arbitrary state**,

```math
p^{( op )} \psi ( x,t ) = p^{( op )} \int_{-\infty}^{\infty} \varphi ( p ) u_{p} ( x,t ) dp = \int_{-\infty}^{\infty} \varphi ( p ) pu_{p} ( x,t ) dp
```

it gives us the right $`p`$ for each term in the integral. This will allow us to compute expectation values for any variable we can represent by an operator.

<div align="right"><a href="book.pdf#page=117" target="_blank" title="This page in the PDF"><sup>[p. 117]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p118" href="book.pdf#page=118" target="_blank" title="This page in the PDF"><sup>[p. 118]</sup></a>

## <a id="s-6-1-2-the-energy-operator"></a>6.1.2 The Energy Operator

We can deduce and verify (See section 6.6.2) the **energy operator** in the same way.

```math
E^{( op )} = i \hbar \frac{\partial}{\partial t}
```

## <a id="s-6-1-3-the-position-operator"></a>6.1.3 The Position Operator

What about the **position operator**, $`x^{( op )}`$? The answer is simply

```math
x^{( op )} = x
```

when we are working in **position space** with $`u_{p 0} ( x,t ) = \tfrac{1}{2 \pi \hbar} \sqrt{e^{i ( p_{0} x - E_{0} t ) / \hbar}}`$ (as we have been above).

## <a id="s-6-1-4-the-hamiltonian-operator"></a>6.1.4 The Hamiltonian Operator

We can develop other operators using the basic ones. We will use the **Hamiltonian operator** which, for our purposes, is the sum of the kinetic and potential energies. This is the non-relativistic case.

```math
H = \frac{p^{2}}{2 m} + V ( x )
```

```math
H^{( op )} = - \frac{\hbar^{2}}{2 m} \frac{\partial^{2}}{\partial x^{2}} + V ( x )
```

Since the potential energy just depends on $`x`$, its easy to use. Angular momentum operators will later be simply computed from position and momentum operators.

# <a id="s-6-2-operators-in-momentum-space"></a>6.2 Operators in Momentum Space

If we want to work in **momentum space**, we need to look at the states of definite position to find our operators. The state (in momentum space) with definite position $`x_{0}`$ is

```math
v_{x 0} ( p ) = \frac{1}{\sqrt{2 \pi \hbar}} e^{- ipx_{0} / \hbar}
```

The operators are

```math
x^{( op )} = i \hbar \frac{\partial}{\partial p}
```

<div align="right"><a href="book.pdf#page=118" target="_blank" title="This page in the PDF"><sup>[p. 118]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p119" href="book.pdf#page=119" target="_blank" title="This page in the PDF"><sup>[p. 119]</sup></a>

and

```math
p^{( op )} = p.
```

The $`( op )`$ notation used above is usually dropped. If we see the variable $`p`$, use of the operator is implied (except in state written in terms of $`p`$ like $`\varphi ( p ) ) .`$

**Gasiorowicz Chapter 3**

**Griffiths doesn’t cover this.**

**Cohen-Tannoudji et al. Chapter**

# <a id="s-6-3-expectation-values"></a>6.3 Expectation Values

Operators allow us to compute the expectation value of some physics quantity given the wavefunction. If a particle is in the state $`\psi ( x,t )`$, the normal way to **compute the expectation value** of

```math
f ( x ) \text{ is }_{\infty \infty}
```

```math
\langle f ( x ) \rangle_{\psi} = \int_{-\infty} P ( x ) f ( x ) dx = \int_{-\infty} \psi^{*} ( x ) \psi ( x ) f ( x ) dx.
```

We can move the $`f ( x )`$ between just before $`\psi`$ anticipating the use of linear operators.

```math
\langle f ( x ) \rangle_{\psi} = \int_{-\infty}^{\infty} \psi^{*} ( x ) f ( x ) \psi ( x ) dx
```

If the variable we wish to compute the expectation value of (like $`p )`$ is not a simple function of $`x`$, let its operator act on $`\psi ( x )`$. The **expectation value of** $`p`$ **in the state** $`\psi`$ is

```math
\langle p \rangle_{\psi} = \langle \psi | p | \psi \rangle = \int_{-\infty}^{\infty} \psi^{*} ( x ) p^{( op )} \psi ( x ) dx
```

The Dirac Bra-ket notation (See section 6.4) shown above is a convenient way to represent the expectation value of a variable given some state.

\* See Example 6.7.1: A particle is in the state $`\psi ( x ) = \begin{pmatrix} 1 \\ 2 \pi \alpha \end{pmatrix}^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}`$. What is the expectation value of $`p`$?\*

For any physical quantity $`v`$, the expectation value of $`v`$ in an arbitrary state $`\psi`$ is

```math
\langle \psi | v | \psi \rangle = \int_{-\infty}^{\infty} \psi^{*} ( x ) v^{( op )} \psi ( x ) dx
```

<div align="right"><a href="book.pdf#page=119" target="_blank" title="This page in the PDF"><sup>[p. 119]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p120" href="book.pdf#page=120" target="_blank" title="This page in the PDF"><sup>[p. 120]</sup></a>

**The expectation values of physical quantities should be real**.

## <a id="s-gasiorowicz-chapter-3"></a>Gasiorowicz Chapter 3

## <a id="s-griffiths-chapter-1"></a>Griffiths Chapter 1

## <a id="s-cohen-tannoudji-et-al-chapter"></a>Cohen-Tannoudji et al. Chapter

# <a id="s-6-4-dirac-bra-ket-notation"></a>6.4 Dirac Bra-ket Notation

A state with definite momentum $`p . | p \rangle`$<br>
A state with definite position $`x . | x \rangle`$<br>
The **“dot product”** between two abstract states $`\psi_{1}`$ and $`\psi_{2}`$.

```math
\langle \psi_{1} | \psi_{2} \rangle = \int_{-\infty}^{\infty} \psi_{1}^{*} \psi_{2} dx
```

This dot product projects the state $`\psi_{2}`$ onto $`\psi_{1}`$ and represents the amplitude to go from $`\psi_{2}`$ to $`\psi_{1}`$.

To find the probability amplitude for our particle to by at any position $`x`$, we dot the state of definite $`x`$ into our state $`\psi . \psi ( x ) = \langle x | \psi \rangle`$

To find the probability amplitude for our particle to have a momentum $`p`$, we dot the state of definite $`x`$ into our state $`\psi . \varphi ( p ) = \langle p | \psi \rangle`$

# <a id="s-6-5-commutators"></a>6.5 Commutators

Operators (or variables in quantum mechanics) do not necessarily commute. We can see our first example of that now that we have a few operators. We **define the commutator** to be

```math
[ p,x ] \equiv px - xp
```

(using $`p`$ and $`x`$ as examples.)

We will now **compute the commutator** between $`p`$ and $`x`$. Because $`p`$ is represented by a differential operator, we must do this carefully. Lets think of the commutator as a (differential) operator too, as generally it will be. To make sure that we keep all the $`\frac{\partial}{\partial x}`$ that we need, we will compute $`[ p,x ] \psi ( x )`$ then remove the $`\psi ( x )`$ at the end to see only the commutator.

```math
[ p,x ] \psi ( x ) = px\psi ( x ) - xp\psi ( x ) = \frac{\hbar}{i} \frac{\partial}{\partial x} x\psi ( x ) - x \frac{\hbar}{i} \frac{\partial}{\partial x} \psi ( x )
```

<div align="right"><a href="book.pdf#page=120" target="_blank" title="This page in the PDF"><sup>[p. 120]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p121" href="book.pdf#page=121" target="_blank" title="This page in the PDF"><sup>[p. 121]</sup></a>

```math
[ p,x ] \psi ( x ) = \frac{\hbar}{i} \left( \psi ( x ) + x \frac{\partial \psi ( x )}{\partial x} - x \frac{\partial \psi ( x )}{\partial x} \right) = \frac{\hbar}{i} \psi ( x )
```

<div align="center">

So, removing the $`\psi ( x )`$ we used for computational purposes, we get the commutator.

</div>

```math
[ p,x ] = \frac{\hbar}{i}
```

Later we will learn to derive the uncertainty relation for two variables from their commutator. Physical variable with zero commutator have no uncertainty principle and we can know both of them at the same time.

We will also use commutators to solve several important problems.

We can compute the **same commutator in momentum space**.

```math
[ p,x ] \varphi = [ p,i \hbar \frac{d}{dp} ] \varphi = i \hbar \left( p \frac{d}{dp} \varphi - \frac{d}{dp} p\varphi \right) = i \hbar ( - \varphi ) = \frac{\hbar}{i} \varphi
```

```math
[ p,x ] = \frac{\hbar}{i}
```

The commutator is the same in any representation.

```math
\begin{array}{ll}
\text{* See Example 6.7.2:} & \text{ Compute the commutator } [ E,t ] . * \\ 
\text{* See Example 6.7.3:} & \text{ Compute the commutator } [ E,x ] . * \\ 
\text{* See Example 6.7.4:} & \text{ Compute the commutator } [ p,x^{n} ] . * \\ 
\text{* See Example 6.7.5:} & \text{ Compute the commutator of the angular momentum operators } [ L_{x} ,L_{y} ] . * \\ 
\end{array}
```

**Gasiorowicz Chapter 3**

**Griffiths Chapter 3**

**Cohen-Tannoudji et al. Chapter**

# <a id="s-6-6-derivations-and-computations"></a>6.6 Derivations and Computations

## <a id="s-6-6-1-verify-momentum-operator"></a>6.6.1 Verify Momentum Operator

```math
p^{( op )} \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar} = \frac{\hbar}{i} \frac{\partial}{\partial x} \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar}
```

```math
\frac{1}{\sqrt{2 \pi \hbar}} \frac{\hbar}{i} \frac{ip_{0}}{\hbar} e^{i ( p_{0} x - E_{0} t ) / \hbar} = p_{0} \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar}
```

<div align="right"><a href="book.pdf#page=121" target="_blank" title="This page in the PDF"><sup>[p. 121]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p122" href="book.pdf#page=122" target="_blank" title="This page in the PDF"><sup>[p. 122]</sup></a>

## <a id="s-6-6-2-verify-energy-operator"></a>6.6.2 Verify Energy Operator

```math
E^{( op )} \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar} = \frac{1}{\sqrt{2 \pi \hbar}} i \hbar \frac{- iE_{0}}{\hbar} e^{i ( p_{0} x - E_{0} t ) / \hbar}
```

```math
= E_{0} \frac{1}{\sqrt{2 \pi \hbar}} e^{i ( p_{0} x - E_{0} t ) / \hbar}
```

# <a id="s-6-7-examples"></a>6.7 Examples

## <a id="s-6-7-1-expectation-value-of-momentum-in-a-given-state"></a>6.7.1 Expectation Value of Momentum in a Given State

A particle is in the state $`\psi ( x ) = \left( \tfrac{1}{2 \pi \alpha} \right)^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}`$. What is the expectation value of $`p`$?

We will use the momentum operator to get this result.

```math
\langle p \rangle_{\psi} = \langle \psi | p | \psi \rangle = \int_{-\infty}^{\infty} \psi^{*} ( x ) p^{( op )} \psi ( x ) dx
```

```math
= \int_{-\infty}^{\infty} \left( \frac{1}{2 \pi \alpha} \right)^{1 / 4} e^{- ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} \frac{\hbar}{i} \frac{\partial}{\partial x} \left( \frac{1}{2 \pi \alpha} \right)^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} dx
```

```math
= \left( \frac{1}{2 \pi \alpha} \right)^{1 / 2} \frac{\hbar}{i} \int_{-\infty}^{\infty} e^{- ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} \frac{\partial}{\partial x} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} dx
```

```math
= \left( \frac{1}{2 \pi \alpha} \right)^{1 / 2} \frac{\hbar}{i} \int_{-\infty}^{\infty} e^{- ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} \left( ik_{0} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} - \frac{2 x}{4 \alpha} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} \right) dx
```

```math
= \left( \frac{1}{2 \pi \alpha} \right)^{1 / 2} \frac{\hbar}{i} \int_{-\infty}^{\infty} \left( ik_{0} e^{- \tfrac{x 2}{2 \alpha}} - \frac{2 x}{4 \alpha} e^{- \tfrac{x 2}{2 \alpha}} \right) dx
```

The second term gives zero because the integral is odd about $`x`$ = 0.

```math
\langle \psi | p | \psi \rangle = \left( \frac{1}{2 \pi \alpha} \right)^{1 / 2} \frac{\hbar}{i} \int_{-\infty}^{\infty} \left( ik_{0} e^{- \tfrac{x 2}{2 \alpha}} \right) dx
```

```math
\langle \psi | p | \psi \rangle = \left( \frac{1}{2 \pi \alpha} \right)^{1 / 2} \hbar k_{0} \sqrt{2 \pi \alpha} = \hbar k_{0}
```

Excellent.

## <a id="s-6-7-2-commutator-of-e-and-t"></a>6.7.2 Commutator of $`E`$ and $`t`$

Again use the crutch of keeping a wave function on the right to avoid mistakes.

```math
[ E,t ] \psi ( x,t ) = \left( i \hbar \frac{\partial}{\partial t} t - ti \hbar \frac{\partial}{\partial t} \right) \psi ( x,t )
```

<div align="right"><a href="book.pdf#page=122" target="_blank" title="This page in the PDF"><sup>[p. 122]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p123" href="book.pdf#page=123" target="_blank" title="This page in the PDF"><sup>[p. 123]</sup></a>

```math
= i \hbar \psi ( x,t ) + \left( i \hbar t \frac{\partial}{\partial t} - i \hbar t \frac{\partial}{\partial t} \right) \psi ( x,t )
```

```math
= i \hbar \psi ( x,t )
```

Removing the wave function, we have the commutator.

```math
[ E,t ] = i \hbar
```

## <a id="s-6-7-3-commutator-of-e-and-x"></a>6.7.3 Commutator of $`E`$ and $`x`$

Again use the crutch of keeping a wave function on the right to avoid mistakes.

```math
[ E,x ] \psi ( x,t ) = \left( i \hbar \frac{\partial}{\partial t} x - xi \hbar \frac{\partial}{\partial t} \right) \psi ( x,t )
```

```math
= \left( i \hbar x \frac{\partial}{\partial t} - i \hbar x \frac{\partial}{\partial t} \right) \psi ( x,t ) = 0
```

Since $`\frac{\partial x}{\partial t}`$ = 0.

## <a id="s-6-7-4-commutator-of-p-and-x-n"></a>6.7.4 Commutator of $`p`$ and $`x^{n}`$

We can use the commutator $`[ p,x ]`$ to help us. Remember that $`px = xp + [ p,x ]`$.

```math
[ p,x^{n} ] = px^{n} - x^{n} p
```

```math
= ( px ) x^{n - 1} - x^{n} p
```

```math
= xpx^{n - 1} + [ p,x ] x^{n - 1} - x^{n} p
```

```math
= x ( px ) x^{n - 2} + [ p,x ] x^{n - 1} - x^{n} p
```

```math
= x^{2} px^{n - 2} + x [ p,x ] x^{n - 2} + [ p,x ] x^{n - 1} - x^{n} p
```

```math
= x^{2} px^{n - 2} + 2 [ p,x ] x^{n - 1} - x^{n} p
```

```math
= x^{3} px^{n - 3} + 3 [ p,x ] x^{n - 1} - x^{n} p
```

```math
= x^{n} p + n [ p,x ] x^{n - 1} - x^{n} p
```

```math
= n [ p,x ] x^{n - 1} = n \frac{\hbar}{i} x^{n - 1}
```

It is usually not wise to use the differential operators and a wave function crutch to compute commutators like this one. **Use the known basic commutators when you can.** Nevertheless, we can compute it that way.

```math
[ p,x^{n} ] \psi = \frac{\hbar}{i} \frac{\partial}{\partial x} x^{n} \psi - x^{n} \frac{\hbar}{i} \frac{\partial}{\partial x} \psi = \frac{\hbar}{i} nx^{n - 1} \psi
```

```math
[ p,x^{n} ] = \frac{\hbar}{i} nx^{n - 1}
```

It works pretty well for this particular case, but not if I have $`p`$ to some power...

<div align="right"><a href="book.pdf#page=123" target="_blank" title="This page in the PDF"><sup>[p. 123]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p124" href="book.pdf#page=124" target="_blank" title="This page in the PDF"><sup>[p. 124]</sup></a>

## <a id="s-6-7-5-commutator-of-l-x-and-l-y"></a>6.7.5 Commutator of $`L_{x}`$ and $`L_{y}`$

Angular momentum is defined by

```math
\vec{L} = \vec{r} \times \vec{p} .
```

So the components of angular momentum are

```math
L_{z} = xp_{y} - yp_{x}
```

```math
L_{x} = yp_{z} - zp_{y}
```

```math
L_{y} = zp_{x} - xp_{z} .
```

We wish to compute $`[ L_{x} ,L_{y} ]`$ which has all the coordinates and momenta in it.

The only operators that do not commute are the coordinates and their conjugate momenta.

```math
[ x,y ] = 0
```

```math
[ p_{x} ,p_{y} ] = 0
```

```math
[ p_{i} ,r_{j} ] = \frac{\hbar}{i} \delta_{ij}
```

So now we just need to compute.

```math
[ L_{x} ,L_{y} ] = [ yp_{z} - zp_{y} ,zp_{x} - xp_{z} ]
```

```math
= [ yp_{z} ,zp_{x} ] - [ yp_{z} ,xp_{z} ] - [ zp_{y} ,zp_{x} ] + [ zp_{y} ,xp_{z} ]
```

```math
= y [ p_{z} ,z ] p_{x} - 0 - 0 + x [ z,p_{z} ] p_{y}
```

```math
= \frac{\hbar}{i} ( yp_{x} - xp_{y} ) = i \hbar L_{z}
```

It is not necessary (or wise) to use the differential operators and a wave function crutch to compute commutators like this one. **Use the known basic commutators when you can.**

# <a id="s-6-8-sample-test-problems"></a>6.8 Sample Test Problems

1. The absolute square of a wave function for a free particle is given as:

```math
| \psi ( x,t ) |^{2} = \sqrt{2 \pi ( a^{2} a + b^{2} t^{2} )} e^{- a ( x - v_{g} t )^{2} / 2 ( a^{2} + b^{2} t^{2} )}
```

Find the expected value of $`x`$ as a function of time. Find the expected value of $`x^{2}`$ as a function of time. Compute the RMS x-width of this wave packet as a function of time.

2. Find the commutator $`[ p,e^{ik_{0} x} ]`$ where $`k_{0}`$ is a constant and the second operator can be expanded

```math
\text{ as } e^{ik_{0} x} = {}_{n} \sum_{= 0}^{\infty} \tfrac{( ik_{0} x )^{n}}{n !} .
```

3. Which of the following are linear operators?

<div align="right"><a href="book.pdf#page=124" target="_blank" title="This page in the PDF"><sup>[p. 124]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p125" href="book.pdf#page=125" target="_blank" title="This page in the PDF"><sup>[p. 125]</sup></a>

```math
• O_{1} \psi ( x ) = 1 /\psi ( x )
```

```math
• O_{2} \psi ( x ) = \tfrac{\partial \psi ( x )}{\partial x}
```

```math
• O_{3} \psi ( x ) = x^{2} \psi ( x )
```

```math
• O_{4} \psi ( x ) = - \psi ( x + a )
```

4. For a free particle, the total energy operator H is given by $`H = p^{2} / 2 m`$. Compute the commutators [H,x] and [H,p]. If a particle is in a state of definite energy, what do these commutators tell you about how well we know the particle’s position and momentum?

5. Find the commutator $`[ x,p^{3} ]`$.

6. Compute the commutator $`[ H,x^{2} ]`$ where $`H`$ is the Hamiltonian for a free particle.

```math
7 .
```

---

[← 5 Wave Packets](05-wave-packets.md) · [7 The Schrödinger Equation →](07-the-schrodinger-equation.md)
