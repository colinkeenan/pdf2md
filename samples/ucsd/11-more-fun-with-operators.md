[← 10 Harmonic Oscillator Solution using Operators](10-harmonic-oscillator-solution-using-operators.md) · [12 Extending QM to Two Particles and Three Dimensions →](12-extending-qm-to-two-particles-and-three-dimensions.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p182" href="book.pdf#page=182" target="_blank" title="This page in the PDF"><sup>[p. 182]</sup></a>

# 11 More Fun with Operators

# <a id="s-11-1-operators-in-a-vector-space"></a>11.1 Operators in a Vector Space

## <a id="s-11-1-1-review-of-operators"></a>11.1.1 Review of Operators

First, a little review. Recall that the **square integrable functions form a vector space**, much like the familiar 3D vector space.

```math
\vec{r} = a v_{\vec{1}} + b \vec{v}_{2}
```

in 3D space becomes

```math
| \psi \rangle = \lambda_{1} | \psi_{1} \rangle + \lambda_{2} | \psi_{2} \rangle .
```

The **scalar product** is defined as

```math
\langle \varphi | \psi \rangle = \int_{-\infty}^{\infty} dx\varphi^{*} ( x ) \psi ( x )
```

and many of its properties can be easily deduced from the integral.

```math
\langle \varphi | \psi \rangle^{*} = \langle \psi | \varphi \rangle
```

As in 3D space,

```math
\vec{a} \cdot \vec{b} \leq | a || b |
```

the magnitude of the dot product is limited by the magnitude of the vectors.

```math
\langle \psi | \varphi \rangle \leq \sqrt{\langle \psi | \psi \rangle \langle \varphi | \varphi \rangle}
```

This is called the **Schwartz inequality**.

**Operators are associative** but not commutative.

```math
AB | \psi \rangle = A ( B | \psi \rangle ) = ( AB ) | \psi \rangle
```

An **operator transforms one vector into another vector**.

```math
| \varphi^{\prime} \rangle = O | \varphi \rangle
```

Eigenfunctions of **Hermitian operators**

```math
H | i \rangle = E_{i} | i \rangle
```

form an **orthonormal**

```math
\langle i | j \rangle = \delta_{ij}
```

**complete set**

```math
| \psi \rangle = \sum_{i} \langle i | \psi \rangle | i \rangle = \sum_{i} | i \rangle \langle i | \psi \rangle .
```

<div align="right"><a href="book.pdf#page=182" target="_blank" title="This page in the PDF"><sup>[p. 182]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p183" href="book.pdf#page=183" target="_blank" title="This page in the PDF"><sup>[p. 183]</sup></a>

Note that we can simply describe the $`j^{th}`$ eigenstate at $`| j \rangle`$.

Expanding the vectors $`| \varphi \rangle`$ and $`| \psi \rangle`$,

```math
| \varphi \rangle = \sum_{i} b_{i} | i \rangle
```

```math
| \psi \rangle = \sum_{i} c_{i} | i \rangle
```

we can take the dot product by multiplying the components, as in 3D space.

```math
\langle \varphi | \psi \rangle = \sum_{i} b_{i}^{*} c_{i}
```

The expansion in energy eigenfunctions is a very nice way to do the **time development of a wave** **function**.

```math
| \psi ( t ) \rangle = \sum_{i} \langle i | \psi ( 0 ) \rangle | i \rangle e^{- iE_{i} t/ \hbar}
```

The basis of **definite momentum states** is not in the vector space, yet we can use this basis to form any state in the vector space.

```math
| \psi \rangle = \frac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} dp\varphi ( p ) | p \rangle
```

Any of these amplitudes can be used to define the state.

```math
c_{i} = \langle i | \psi \rangle
```

```math
\psi ( x ) = \langle x | \psi \rangle
```

```math
\varphi ( p ) = \langle p | \psi \rangle
```

## <a id="s-11-1-2-projection-operators-j-rangle-langle-j-and-completeness"></a>11.1.2 Projection Operators $`| j \rangle \langle j |`$ and Completeness

Now we move on a little with our understanding of operators. A ket vector followed by a bra vector is an example of an operator. For example the **operator which projects a vector onto the** $`j^{th}`$ **eigenstate** is

```math
| j \rangle \langle j |
```

First the bra vector dots into the state, giving the coefficient of $`| j \rangle`$ in the state, then its multiplied by the unit vector $`| j \rangle`$, turning it back into a vector, with the right length to be a projection. An operator maps one vector into another vector, so this is an operator.

The sum of the projection operators is 1, if we **sum over a complete set of states**, like the eigenstates of a Hermitian operator.

```math
\sum | i \rangle \langle i | = 1
```

<div align="right"><a href="book.pdf#page=183" target="_blank" title="This page in the PDF"><sup>[p. 183]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p184" href="book.pdf#page=184" target="_blank" title="This page in the PDF"><sup>[p. 184]</sup></a>

This is an extremely useful identity for solving problems. We could already see this in the decomposition of $`| \psi \rangle`$ above.

```math
| \psi \rangle = \sum_{i} | i \rangle \langle i | \psi \rangle .
```

The same is true for definite momentum states.

```math
\int_{-\infty}^{\infty} | p \rangle \langle p | dp = 1
```

We can form a projection operator into a **subspace**.

```math
P = \sum_{subspace} | i \rangle \langle i |
```

We could use this to project out the odd parity states, for example.

## <a id="s-11-1-3-unitary-operators"></a>11.1.3 Unitary Operators

**Unitary operators** preserve a scalar product.

```math
\langle \varphi | \psi \rangle = \langle U\varphi | U\psi \rangle = \langle \varphi | U^{\dagger} U\psi \rangle
```

This means that

```math
U^{\dagger} U = 1 .
```

Unitary operators will be important for the matrix representation of operators. The will allow us to change from one orthonormal basis to another.

# <a id="s-11-2-a-complete-set-of-mutually-commuting-operators"></a>11.2 A Complete Set of Mutually Commuting Operators

If an operator commutes with $`H`$, we can make **simultaneous eigenfunctions** of energy and that operator. This is an important tool both for solving the problem and for labeling the eigenfunctions.

A **complete set of mutually commuting operators** will allow us to define a state in terms of the quantum numbers of those operators. Usually, we will need one quantum number for each degree of freedom in the problem.

For example, the Hydrogen atom in three dimensions has 3 coordinates for the internal problem, (the vector displacement between the proton and the electron). We will need three quantum numbers to describe the state. We will use an energy index, and two angular momentum quantum numbers to describe Hydrogen states. The operators will all commute with each other. The Hydrogen atom also has 3 coordinates for the position of the atom. We will might use $`p_{x} , p_{y}`$ and $`p_{z}`$ to describe that state. The operators commute with each other.

<div align="right"><a href="book.pdf#page=184" target="_blank" title="This page in the PDF"><sup>[p. 184]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p185" href="book.pdf#page=185" target="_blank" title="This page in the PDF"><sup>[p. 185]</sup></a>

If we also consider the spin of the electron in the Hydrogen atom, we find that we need to add one more commuting operator to label the states and to compute the energies accurately. If we also add the spin of the proton to the problem, the we need still one more quantum number to describe the state.

If it is possible, identifying the commuting operators to be used before solving the problem will usually save time.

# <a id="s-11-3-uncertainty-principle-for-non-commuting-operators"></a>11.3 Uncertainty Principle for Non-Commuting Operators

Let us now **derive the uncertainty relation for non-commuting operators** $`A`$ and $`B`$. First, given a state $`\psi`$, the **Mean Square uncertainty** in the physical quantity represented is defined as

```math
( \Delta A )^{2} = \langle \psi | ( A - \langle A \rangle )^{2} \psi \rangle = \langle \psi | U^{2} \psi \rangle
```

```math
( \Delta B )^{2} = \langle \psi | ( B - \langle B \rangle )^{2} \psi \rangle = \langle \psi | V^{2} \psi \rangle
```

where we define (just to keep our expressions small)

```math
U = A - \langle \psi | A\psi \rangle
```

```math
V = B - \langle \psi | B\psi \rangle .
```

Since $`\langle A \rangle`$ and $`\langle B \rangle`$ are just constants, notice that

```math
[ U,V ] = [ A,B ]
```

OK, so much for the definitions.

Now we will dot $`U\psi + i\lambda V\psi`$ into itself to get some information about the uncertainties. The dot product must be greater than or equal to zero.

```math
\langle U\psi + i\lambda V\psi | U\psi + i\lambda V\psi \rangle \geq 0
```

```math
\langle \psi | U^{2} \psi \rangle + \lambda^{2} \langle \psi | V^{2} \psi \rangle + i\lambda \langle U\psi | V\psi \rangle - i\lambda \langle V\psi | U\psi \rangle \geq 0
```

This expression contains the uncertainties, so lets identify them.

```math
( \Delta A )^{2} + \lambda^{2} ( \Delta B )^{2} + i\lambda \langle \psi | [ U,V ] | \psi \rangle \geq 0
```

Choose a $`\lambda`$ to minimize the expression, to get the strongest inequality.

```math
\frac{\partial}{\partial \lambda} = 0
```

```math
\begin{aligned} 2 \lambda ( \Delta B )^{2} + i \langle \psi | [ U,V ] | \psi \rangle &= 0 \\ \lambda &= \frac{- i \langle \psi | [ U,V ] | \psi \rangle}{2 ( \Delta B )^{2}} \end{aligned}
```

<div align="right"><a href="book.pdf#page=185" target="_blank" title="This page in the PDF"><sup>[p. 185]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p186" href="book.pdf#page=186" target="_blank" title="This page in the PDF"><sup>[p. 186]</sup></a>

Plug in that $`\lambda`$.

```math
( \Delta A )^{2} - \frac{1}{4} \frac{\langle \psi | [ U,V ] | \psi \rangle^{2}}{( \Delta B )^{2}} + \frac{\langle \psi | [ U,V ] | \psi \rangle^{2}}{2 ( \Delta B )^{2}} \geq 0
```

```math
( \Delta A )^{2} ( \Delta B )^{2} \geq - \frac{1}{4} \langle \psi | [ U,V ] | \psi \rangle^{2} = \langle \psi | \frac{i}{2} [ U,V ] | \psi \rangle^{2}
```

This result is the **uncertainty for non-commuting operators**.

```math
( \Delta A ) ( \Delta B ) \geq \frac{i}{2} \langle [ A,B ] \rangle
```

If the commutator is a constant, as in the case of $`[ p,x ]`$, the expectation values can be removed.

```math
( \Delta A ) ( \Delta B ) \geq \frac{i}{2} [ A,B ]
```

For momentum and position, this agrees with the uncertainty principle we know.

```math
( \Delta p ) ( \Delta x ) \geq \frac{i}{2} \langle [ p,x ] \rangle = \frac{\hbar}{2}
```

(Note that we could have simplified the proof by just stating that we choose to dot $`( U + \tfrac{i \langle [ U,V ] \rangle}{2 ( \Delta B )^{2}} V ) \psi`$ into itself and require that its positive. It would not have been clear that this was the strongest condition we could get.)

# <a id="s-11-4-time-derivative-of-expectation-values"></a>11.4 Time Derivative of Expectation Values \*

We wish to compute the time derivative of the expectation value of an operator $`A`$ in the state $`\psi`$. Thinking about the integral, this has three terms.

```math
\begin{aligned} \frac{d}{dt} \langle \psi | A | \psi \rangle &= \left\langle \frac{d\psi}{dt} | A | \psi \right\rangle + \left\langle \psi \left| \frac{\partial A}{\partial t} \right| \psi \right\rangle + \left\langle \psi | A | \frac{d\psi}{dt} \right\rangle \\ &= \frac{- 1}{i \hbar} \langle H\psi | A | \psi \rangle + \frac{1}{i \hbar} \langle \psi | A | H\psi \rangle + \left\langle \psi \left| \frac{\partial A}{\partial t} \right| \psi \right\rangle \end{aligned}
```

```math
= \frac{i}{\hbar} \langle \psi | [ H,A ] | \psi \rangle + \left\langle \psi \left| \frac{\partial A}{\partial t} \right| \psi \right\rangle
```

This is an important general result for the **time derivative of expectation values**.

```math
\frac{d}{dt} \langle \psi | A | \psi \rangle = \frac{i}{\hbar} \langle \psi | [ H,A ] | \psi \rangle + \left\langle \psi \left| \frac{\partial A}{\partial t} \right| \psi \right\rangle
```

<div align="right"><a href="book.pdf#page=186" target="_blank" title="This page in the PDF"><sup>[p. 186]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p187" href="book.pdf#page=187" target="_blank" title="This page in the PDF"><sup>[p. 187]</sup></a>

which becomes simple if the operator itself does not explicitly depend on time.

```math
\frac{d}{dt} \langle \psi | A | \psi \rangle = \frac{i}{\hbar} \langle \psi | [ H,A ] | \psi \rangle
```

Expectation values of operators that commute with the Hamiltonian are constants of the motion.

We can apply this to verify that the expectation value of $`x`$ behaves as we would expect for a classical particle.

```math
\frac{d \langle x \rangle}{dt} = \frac{i}{\hbar} \langle [ H,x ] \rangle = \frac{i}{\hbar} \left\langle \left[ \frac{p^{2}}{2 m} ,x \right] \right\rangle = \left\langle \frac{p}{m} \right\rangle
```

This is a good result. This is called the **Ehrenfest Theorem**.

For momentum,

```math
\frac{d \langle p \rangle}{dt} = \frac{i}{\hbar} \langle [ H,p ] \rangle = \frac{i}{\hbar} \left\langle \left[ V ( x ) , \frac{\hbar}{i} \frac{d}{dx} \right] \right\rangle = - \left\langle \frac{dV ( x )}{dx} \right\rangle
```

which **Mr. Newton** told us a long time ago.

# <a id="s-11-5-the-time-development-operator"></a>11.5 The Time Development Operator \*

We can actually make an **operator that does the time development of a wave function**. We just make the simple exponential solution to the Schrödinger equation using operators.

```math
i \hbar \frac{\partial \psi}{\partial t} = H\psi
```

```math
\psi ( t ) = e^{- iHt/ \hbar} \psi ( 0 )
```

where $`H`$ is the operator. We can expand this exponential to understand its meaning a bit.

```math
e^{- iHt/ \hbar} = \sum_{n = 0}^{\infty} \frac{( - iHt/ \hbar )^{n}}{n !}
```

This is an infinite series containing all powers of the Hamiltonian. In some cases, it can be easily computed.

<div align="center">

$`e^{- iHt/ \hbar}`$ is the time development operator. It takes a state from time 0 to time $`t`$.

</div>

<div align="right"><a href="book.pdf#page=187" target="_blank" title="This page in the PDF"><sup>[p. 187]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p188" href="book.pdf#page=188" target="_blank" title="This page in the PDF"><sup>[p. 188]</sup></a>

# <a id="s-11-6-the-heisenberg-picture"></a>11.6 The Heisenberg Picture \*

To begin, lets compute the expectation value of an operator $`B`$.

```math
\langle \psi ( t ) | B | \psi ( t ) \rangle = \langle e^{- iHt/ \hbar} \psi ( 0 ) | B | e^{- iHt/ \hbar} \psi ( 0 ) \rangle
```

```math
= \langle \psi ( 0 ) | e^{iHt/ \hbar} Be^{- iHt/ \hbar} | \psi ( 0 ) \rangle
```

According to our rules, we can multiply operators together before using them. We can then define the operator that depends on time.

```math
B ( t ) = e^{iHt/ \hbar} Be^{- iHt/ \hbar}
```

If we use this operator, we don’t need to do the time development of the wavefunctions!

This is called the **Heisenberg Picture**. In it, the **operators evolve with time** and the wave-functions remain constant.

The usual Schrödinger picture has the states evolving and the operators constant.

We can now compute the time derivative of an operator.

```math
\frac{d}{dt} B ( t ) = \frac{iH}{\hbar} e^{iHt/ \hbar} Be^{- iHt/ \hbar} - e^{iHt/ \hbar} B \frac{iH}{\hbar} e^{- iHt/ \hbar}
```

```math
= \frac{i}{\hbar} e^{iHt/ \hbar} [ H,B ] e^{- iHt/ \hbar} = \frac{i}{\hbar} [ H,B ( t ) ]
```

It is governed by the commutator with the Hamiltonian.

As an example, we may look at the HO operators $`A`$ and $`A^{\dagger}`$. We have already computed the commutator.

```math
[ H,A ] = - \hbar \omega A
```

```math
\frac{dA}{dt} = - \frac{i}{\hbar} \hbar \omega A = - i\omega A
```

We can integrate this.

```math
A ( t ) = e^{- i\omega t} A ( 0 )
```

Take the Hermitian conjugate.

```math
A^{\dagger} ( t ) = e^{i\omega t} A^{\dagger} ( 0 )
```

We can combine these to get the momentum and position operators in the Heisenberg picture.

```math
p ( t ) = p ( 0 ) \cos ( \omega t ) - m\omega x ( 0 ) \sin ( \omega t )
```

```math
x ( t ) = x ( 0 ) \cos ( \omega t ) + \frac{p ( 0 )}{m\omega} \sin ( \omega t )
```

<div align="right"><a href="book.pdf#page=188" target="_blank" title="This page in the PDF"><sup>[p. 188]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p189" href="book.pdf#page=189" target="_blank" title="This page in the PDF"><sup>[p. 189]</sup></a>

# <a id="s-11-7-examples"></a>11.7 Examples

## <a id="s-11-7-1-time-development-example"></a>11.7.1 Time Development Example

Start off in the state.

```math
\psi ( t = 0 ) = \frac{1}{\sqrt{2}} ( u_{1} + u_{2} )
```

In the Schrödinger picture,

```math
\psi ( t ) = \frac{1}{\sqrt{2}} ( u_{1} e^{- i \tfrac{3}{2} \omega t} + u_{2} e^{- i \tfrac{5}{2} \omega t} ) = \frac{1}{\sqrt{2}} e^{- i \tfrac{3}{2} \omega t} ( u_{1} + e^{- i\omega t} u_{2} )
```

We can compute the expectation value of $`x`$.

```math
\langle \psi | x | \psi \rangle = \frac{1}{2} \sqrt{\frac{\hbar}{2 m\omega}} \langle u_{1} + e^{i\omega t} u_{2} | A + A^{\dagger} | u_{1} + e^{i\omega t} u_{2} \rangle
```

```math
= \frac{1}{2} \sqrt{\frac{\hbar}{2 m\omega}} \left( \langle u_{1} | A | u_{2} \rangle e^{- i\omega t} + \langle u_{2} | A^{\dagger} | u_{1} \rangle e^{i\omega t} \right)
```

```math
= \frac{1}{2} \sqrt{\frac{\hbar}{2 m\omega}} \left( \sqrt{2} e^{- i\omega t} + \sqrt{2} e^{i\omega t} \right)
```

```math
= \sqrt{\frac{\hbar}{m\omega}} \cos ( \omega t )
```

In the Heisenberg picture

```math
\langle \psi | x ( t ) | \psi \rangle = \frac{1}{2} \sqrt{\frac{\hbar}{2 m\omega}} \langle \psi | e^{- i\omega t} A + e^{i\omega t} A^{\dagger} | \psi \rangle
```

This gives the same answer with about the same amount of work.

# <a id="s-11-8-sample-test-problems"></a>11.8 Sample Test Problems

1. Calculate the commutator $`[ L_{x} ,L_{z} ]`$ where $`L_{x} = yp_{z} - zp_{y}`$ and $`L_{z} = xp_{y} - yp_{x}`$. State the uncertainty principle for $`L_{x}`$ and $`L_{z}`$.

**Answer**

```math
[ L_{x} ,L_{z} ] = [ yp_{z} - zp_{y} ,xp_{y} - yp_{x} ] = x [ y,p_{y} ] p_{z} + z [ p_{y} ,y ] p_{x}
```

```math
= \frac{\hbar}{i} ( - xp_{z} + zp_{x} ) = i \hbar ( xp_{z} - zp_{x} ) = - i \hbar L_{y}
```

```math
\Delta L_{x} \Delta L_{z} \geq \frac{i}{2} \langle [ L_{x} ,L_{z} ] \rangle = \frac{i}{2} ( - i \hbar ) \langle L_{y} \rangle = \frac{\hbar}{2} \langle L_{y} \rangle
```

2. **\*** A particle of mass $`m`$ is in a **1 dimensional potential** $`V ( x )`$. Calculate the rate of change of the expected values of $`x`$ and $`p , ( \tfrac{d \langle x \rangle}{dt}`$ and $`\tfrac{d \langle p \rangle}{dt} )`$. Your answer will obviously depend on the

<div align="right"><a href="book.pdf#page=189" target="_blank" title="This page in the PDF"><sup>[p. 189]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p190" href="book.pdf#page=190" target="_blank" title="This page in the PDF"><sup>[p. 190]</sup></a>

state of the particle and on the potential.

**Answer**

```math
\frac{d \langle A \rangle}{dt} = \frac{1}{i \hbar} \langle [ A,H ] \rangle
```

```math
\frac{d \langle x \rangle}{dt} = \frac{1}{i \hbar} \left\langle \left[ x, \frac{p^{2}}{2 m} \right] \right\rangle
```

```math
= \frac{1}{i \hbar} \left\langle \frac{p}{m} \left( \frac{- \hbar}{i} \right) \right\rangle = \frac{\langle p \rangle}{m}
```

```math
\frac{d \langle p \rangle}{dt} = \frac{1}{i \hbar} \langle [ p,V ( x ) ] \rangle = \frac{1}{i \hbar} \frac{\hbar}{i} \langle [ \frac{d}{dx} ,V ( x ) ] \rangle
```

```math
= - \left\langle \frac{dV}{dx} \right\rangle
```

3. Compute the commutators $`[ A^{\dagger} ,A^{n} ]`$ and [$`A,e^{iHt} ]`$ for the 1D harmonic oscillator.

**Answer**

```math
[ A^{\dagger} ,A^{n} ] = n [ A^{\dagger} ,A ] A^{n - 1} = - nA^{n - 1}
```

```math
[ A,e^{iHt} ] = [ A, \sum_{n = 0}^{\infty} \frac{( it )^{n} H^{n}}{n !} ] = \sum_{n = 0}^{\infty} \frac{( it )^{n} [ A,H^{n} ]}{n !}
```

```math
= \sum_{n = 0}^{\infty} \frac{n ( it )^{n} [ A,H ] H^{n - 1}}{n !} = it \sum_{n = 1}^{\infty} \frac{( it )^{n - 1} \hbar \omega AH^{n - 1}}{( n - 1 ) !}
```

```math
it \sum_{n = 1}^{\infty} \frac{( it )^{n - 1} \hbar \omega AH^{n - 1}}{( n - 1 ) !} = it \hbar \omega A \sum_{n = 1}^{\infty} \frac{( it )^{n - 1} H^{n - 1}}{( n - 1 ) !}
```

```math
= it \hbar \omega A \sum_{n = 0}^{\infty} \frac{( it )^{n} H^{n}}{( n ) !} = it \hbar \omega Ae^{iHt}
```

4. **\*** Assume that the states $`| u_{i} >`$ are the eigenstates of the Hamiltonian with eigenvalues $`E_{i}`$,

```math
( H | u_{i} > = E_{i} | u_{i} > ) .
```

a) Prove that $`<u_{i} | [ H,A ] | u_{i} >`$= 0 for an arbitrary linear operator $`A`$.

b) For a particle of mass $`m`$ moving in 1-dimension, the Hamiltonian is given by $`H = \tfrac{p^{2}}{2 m}`$ +

$`V ( x )`$. Compute the commutator [H,X] where $`X`$ is the position operator.

c) Compute $`<u_{i} | P | u_{i} >`$ the mean momentum in the state $`| u_{i} >`$.

5. **\*** At $`t`$ = 0, a particle of mass $`m`$ is in the Harmonic Oscillator state $`\psi ( t`$ = 0) = $`\tfrac{1}{\sqrt{2}} ( u_{0} + u_{1} )`$. Use the Heisenberg picture to find the expected value of $`x`$ as a function of time.

---

[← 10 Harmonic Oscillator Solution using Operators](10-harmonic-oscillator-solution-using-operators.md) · [12 Extending QM to Two Particles and Three Dimensions →](12-extending-qm-to-two-particles-and-three-dimensions.md)
