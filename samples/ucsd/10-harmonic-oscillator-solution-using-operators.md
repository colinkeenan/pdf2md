[← 9 One Dimensional Potentials](09-one-dimensional-potentials.md) · [11 More Fun with Operators →](11-more-fun-with-operators.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p172" href="book.pdf#page=172" target="_blank" title="This page in the PDF"><sup>[p. 172]</sup></a>

# 10 Harmonic Oscillator Solution using Operators

Operator methods are very useful both for solving the Harmonic Oscillator problem and for any type of computation for the HO potential. The operators we develop will also be useful in quantizing the electromagnetic field.

The Hamiltonian for the **1D Harmonic Oscillator**

```math
H = \frac{p^{2}}{2 m} + \frac{1}{2} m\omega^{2} x^{2}
```

looks like it could be written as the square of a operator. It can be rewritten in terms of the operator $`A`$ (See section 10.1)

```math
A \equiv \begin{pmatrix} \sqrt{\frac{m\omega}{2 \hbar}} \\ x + i \frac{p}{\sqrt{2 m \hbar \omega}} \end{pmatrix}
```

and its Hermitian conjugate $`A^{\dagger}`$.

```math
H = \hbar \omega \left( A^{\dagger} A + \frac{1}{2} \right)
```

We will use the commutators (See section 10.2) between $`A , A^{\dagger}`$ and $`H`$ to solve the HO problem.

```math
[ A,A^{\dagger} ] = 1
```

The commutators with the Hamiltonian are easily computed.

```math
[ H,A ] = - \hbar \omega A
```

```math
[ H,A^{\dagger} ] = \hbar \omega A^{\dagger}
```

From these commutators we can show that $`A^{\dagger}`$ is a raising operator (See section 10.3) for Harmonic Oscillator states

```math
A^{\dagger} u_{n} = \sqrt{n + 1} u_{n + 1}
```

and that $`A`$ is a **lowering operator**.

```math
Au_{n} = \sqrt{n} u_{n - 1}
```

<div align="right"><a href="book.pdf#page=172" target="_blank" title="This page in the PDF"><sup>[p. 172]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p173" href="book.pdf#page=173" target="_blank" title="This page in the PDF"><sup>[p. 173]</sup></a>

Because the lowering must stop at a ground state with positive energy, we can show that the allowed energies are

```math
E_{n} = \left( n + \frac{1}{2} \right) \hbar \omega .
```

The actual wavefunctions (See section 10.5) can be deduced by using the differential operators for $`A`$ and $`A^{\dagger}`$, but often it is more useful to define the $`n^{th}`$ eigenstate in terms of the ground state and raising operators.

```math
u_{n} = \frac{1}{\sqrt{n !}} ( A^{\dagger} )^{n} u_{0}
```

Almost **any calculation** of interest can be done without actual functions since we can express the operators for position and momentum.

```math
x = \sqrt{\frac{\hbar}{2 m\omega}} ( A + A^{\dagger} )
```

```math
p = - i \sqrt{\frac{m \hbar \omega}{2}} ( A - A^{\dagger} )
```

# <a id="s-10-1-introducing-a-and-a-dagger"></a>10.1 Introducing $`A`$ and $`A^{\dagger}`$

The Hamiltonian for the **1D Harmonic Oscillator**

```math
H = \frac{p^{2}}{2 m} + \frac{1}{2} m\omega^{2} x^{2}
```

can be rewritten in terms of **the operator** $`A`$

```math
A \equiv \left( \sqrt{\frac{m\omega}{2 \hbar}} x + i \frac{p}{\sqrt{2 m \hbar \omega}} \right)
```

and its Hermitian conjugate

```math
A^{\dagger} = \begin{pmatrix} \sqrt{\frac{m\omega}{2 \hbar}} \\ x - i \frac{p}{\sqrt{2 m \hbar \omega}} \end{pmatrix}
```

Both terms in the Harmonic Oscillator Hamiltonian are squares of operators. Note that $`A`$ is chosen so that $`A^{\dagger} A`$ is close to the Hamiltonian. First just compute the quantity

```math
A^{\dagger} A = \frac{m\omega}{2 \hbar} x^{2} + \frac{p^{2}}{2 m \hbar \omega} + \frac{i}{2 \hbar} ( xp - px )
```

```math
A^{\dagger} A = \frac{m\omega}{2 \hbar} x^{2} + \frac{p^{2}}{2 m \hbar \omega} - \frac{i}{2 \hbar} [ p,x ]
```

```math
\hbar \omega ( A^{\dagger} A ) = \frac{p^{2}}{2 m} + \frac{1}{2} m\omega^{2} x^{2} - \frac{1}{2} \hbar \omega .
```

From this we can see that the **Hamiltonian can be written in terms of** $`A^{\dagger} A`$ and some constants.

```math
H = \hbar \omega \left( A^{\dagger} A + \frac{1}{2} \right) .
```

<div align="right"><a href="book.pdf#page=173" target="_blank" title="This page in the PDF"><sup>[p. 173]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p174" href="book.pdf#page=174" target="_blank" title="This page in the PDF"><sup>[p. 174]</sup></a>

# <a id="s-10-2-commutators-of-a-a-dagger-and-h"></a>10.2 Commutators of $`A \boldsymbol{,} A^{\dagger}`$ and $`H`$

We will use the commutator between $`A`$ and $`A^{\dagger}`$ to solve the HO problem. The operators are defined to be

```math
A = \begin{pmatrix} \sqrt{\frac{m\omega}{2 \hbar}} \\ x + i \frac{p}{\sqrt{2 m \hbar \omega}} \end{pmatrix}
```

```math
A^{\dagger} = \begin{pmatrix} \sqrt{\frac{m\omega}{2 \hbar}} \\ x - i \frac{p}{\sqrt{2 m \hbar \omega}} \end{pmatrix} .
```

The **commutator** is

```math
[ A,A^{\dagger} ] = \frac{m\omega}{2 \hbar} [ x,x ] + \frac{1}{2 m \hbar \omega} [ p,p ] - \frac{i}{2 \hbar} [ x,p ] + \frac{i}{2 \hbar} [ p,x ]
```

```math
= \frac{i}{2 \hbar} ( - [ x,p ] + [ p,x ] ) = \frac{i}{\hbar} [ p,x ] = 1 .
```

Lets use this simple commutator

```math
[ A,A^{\dagger} ] = 1
```

to compute **commutators with the Hamiltonian**. This is easy if $`H`$ is written in terms of $`A`$ and $`A^{\dagger}`$.

```math
[ H,A ] = \hbar \omega [ A^{\dagger} A,A ] = \hbar \omega [ A^{\dagger} ,A ] A = - \hbar \omega A
```

```math
[ H,A^{\dagger} ] = \hbar \omega [ A^{\dagger} A,A^{\dagger} ] = \hbar \omega A^{\dagger} [ A,A^{\dagger} ] = \hbar \omega A^{\dagger}
```

# <a id="s-10-3-use-commutators-to-derive-ho-energies"></a>10.3 Use Commutators to Derive HO Energies

We have computed the commutators

```math
[ H,A ] = - \hbar \omega A
```

```math
[ H,A^{\dagger} ] = \hbar \omega A^{\dagger}
```

Apply $`[ H,A ]`$ to the energy eigenfunction $`u_{n}`$.

```math
[ H,A ] u_{n} = - \hbar \omega Au_{n}
```

```math
HAu_{n} - AHu_{n} = - \hbar \omega Au_{n}
```

```math
H ( Au_{n} ) - E_{n} ( Au_{n} ) = - \hbar \omega Au_{n}
```

```math
H ( Au_{n} ) = ( E_{n} - \hbar \omega ) ( Au_{n} )
```

This equation shows that $`Au_{n}`$ is an eigenfunction of $`H`$ with eigenvalue $`E_{n} - \hbar \omega`$. Therefore,$`A`$ **lowers** **the energy** by $`\hbar \omega`$.

<div align="right"><a href="book.pdf#page=174" target="_blank" title="This page in the PDF"><sup>[p. 174]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p175" href="book.pdf#page=175" target="_blank" title="This page in the PDF"><sup>[p. 175]</sup></a>

Now, apply $`[ H,A^{\dagger} ]`$ to the energy eigenfunction $`u_{n}`$.

```math
[ H,A^{\dagger} ] u_{n} = \hbar \omega A^{\dagger} u_{n}
```

```math
HA^{\dagger} u_{n} - A^{\dagger} Hu_{n} = \hbar \omega A^{\dagger} u_{n}
```

```math
H ( A^{\dagger} u_{n} ) - E_{n} ( A^{\dagger} u_{n} ) = \hbar \omega ( A^{\dagger} u_{n} )
```

```math
H ( A^{\dagger} u_{n} ) = ( E_{n} + \hbar \omega ) ( A^{\dagger} u_{n} )
```

$`A^{\dagger} u_{n}`$ is an eigenfunction of $`H`$ with eigenvalue $`E_{n} + \hbar \omega . A^{\dagger}`$ **raises the energy** by $`\hbar \omega`$.

We cannot keep lowering the energy because **the HO energy cannot go below zero**.

```math
\langle \psi | H | \psi \rangle = \frac{1}{2 m} \langle p\psi | p\psi \rangle + \frac{1}{2} m\omega^{2} \langle x\psi | x\psi \rangle \geq 0
```

The only way to stop the lowering operator from taking the energy negative, is for the lowering to give zero for the wave function. Because this will be at the lowest energy, this must happen for the ground state. **When we lower the ground state, we must get zero**.

```math
Au_{0} = 0
```

Since the Hamiltonian contains $`A`$ in a convenient place, we can **deduce the ground state energy**.

```math
Hu_{0} = \hbar \omega ( A^{\dagger} A + \frac{1}{2} ) u_{0} = \frac{1}{2} \hbar \omega u_{0}
```

The ground state energy is $`E_{0} = \tfrac{1}{2} \hbar \omega`$ and the states in general have energies

```math
E = \left( n + \frac{1}{2} \right) \hbar \omega
```

since we have shown raising and lowering in steps of $`\hbar \omega`$. Only a state with energy $`E_{0} = \tfrac{1}{2} \hbar \omega`$ can stop the lowering so the **only energies allowed** are

```math
E = \left( n + \frac{1}{2} \right) \hbar \omega .
```

It is interesting to note that we have a **number operator** for $`n`$

```math
H = \left( A^{\dagger} A + \frac{1}{2} \right) \hbar \omega
```

```math
N_{op} = A^{\dagger} A
```

```math
H = ( N_{op} + \frac{1}{2} ) \hbar \omega
```

## <a id="s-10-3-1-raising-and-lowering-constants"></a>10.3.1 Raising and Lowering Constants

We know that $`A^{\dagger}`$ **raises the energy of an eigenstate** but we do not know what coefficient it produces in front of the new state.

```math
A^{\dagger} u_{n} = Cu_{n + 1}
```

<div align="right"><a href="book.pdf#page=175" target="_blank" title="This page in the PDF"><sup>[p. 175]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p176" href="book.pdf#page=176" target="_blank" title="This page in the PDF"><sup>[p. 176]</sup></a>

We can **compute the coefficient** using our operators.

```math
| C |^{2} = \langle A^{\dagger} u_{n} | A^{\dagger} u_{n} \rangle = \langle AA^{\dagger} u_{n} | u_{n} \rangle
```

```math
= \langle ( A^{\dagger} A + [ A,A^{\dagger} ] ) u_{n} | u_{n} \rangle = ( n + 1 ) \langle u_{n} | u_{n} \rangle = n + 1
```

The effect of the **raising operator** is

```math
A^{\dagger} u_{n} = \sqrt{n + 1} u_{n + 1} .
```

Similarly, the effect of the **lowering operator** is

```math
Au_{n} = \sqrt{n} u_{n - 1} .
```

These are extremely important equations for any computation in the HO problem.

We can also **write any energy eigenstate in terms of the ground state and the raising** **operator**.

```math
u_{n} = \frac{1}{\sqrt{n !}} ( A^{\dagger} )^{n} u_{0}
```

# <a id="s-10-4-expectation-values-of-p-and-x"></a>10.4 Expectation Values of $`p`$ and $`x`$

It is important to realize that we can just use the definition of $`A`$ to write $`x`$ and $`p`$ in terms of the raising and lowering operators.

```math
x = \sqrt{\frac{\hbar}{2 m\omega}} ( A + A^{\dagger} )
```

```math
p = - i \sqrt{\frac{m \hbar \omega}{2}} ( A - A^{\dagger} )
```

This will allow for any computation.

\* See Example 10.6.1: The expectation value of $`x`$ for any energy eigenstate is zero.\*<br>
\* See Example 10.6.2: The expectation value of $`p`$ for any energy eigenstate is zero.\*<br>
\* See Example 10.6.3: The expectation value of $`x`$ in the state $`\frac{\sqrt{1}}{2} ( u_{0} + u_{1} ) . *`$<br>
\* See Example 10.6.4: The expectation value of $`\frac{1}{2} m\omega^{2} x^{2}`$ for any energy eigenstate is $`\frac{1}{2} \left( n + \tfrac{1}{2} \right) \hbar \omega . *`$ \* See Example 10.6.5: The expectation value of $`2 p^{2} m`$ for any energy eigenstate is $`\frac{1}{2} \left( n + \tfrac{1}{2} \right) \hbar \omega . *`$<br>
\* See Example 10.6.6: The expectation value of $`p`$ as a function of time for the state $`\psi ( t`$ = 0) =

```math
\frac{\sqrt{1}}{2} ( u_{1} + u_{2} ) \text{ is } - \sqrt{m \hbar \omega \sin ( \omega t ) . *}
```

# <a id="s-10-5-the-wavefunction-for-the-ho-ground-state"></a>10.5 The Wavefunction for the HO Ground State

The equation

```math
Au_{0} = 0
```

<div align="right"><a href="book.pdf#page=176" target="_blank" title="This page in the PDF"><sup>[p. 176]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p177" href="book.pdf#page=177" target="_blank" title="This page in the PDF"><sup>[p. 177]</sup></a>

can be used to **find the ground state wavefunction**. Write $`A`$ in terms of $`x`$ and $`p`$ and try it.

```math
\left( \sqrt{\frac{m\omega}{2 \hbar}} x + i \frac{p}{\sqrt{2 m \hbar \omega}} \right) u_{0} = 0
```

```math
\left( m\omega x + \hbar \frac{d}{dx} \right) u_{0} = 0
```

```math
\frac{du_{0}}{dx} = - \frac{m\omega x}{\hbar} u_{0}
```

This first order differential equation can be solved to get the wavefunction.

```math
u_{0} = Ce^{- m\omega x^{2} / 2 \hbar} .
```

We could continue with the raising operator to get excited states.

```math
\sqrt{1} u_{1} = \left( \sqrt{\frac{m\omega}{2 \hbar}} x - \sqrt{\frac{\hbar}{2 m\omega}} \frac{d}{dx} \right) u_{0}
```

Usually we will not need the actual wave functions for our calculations.

# <a id="s-10-6-examples"></a>10.6 Examples

## <a id="s-10-6-1-the-expectation-value-of-x-in-eigenstate"></a>10.6.1 The expectation value of $`x`$ in eigenstate

We can compute the expectation value of $`x`$ simply.

```math
\langle u_{n} | x | u_{n} \rangle = \sqrt{\frac{\hbar}{2 m\omega}} \langle u_{n} | A + A^{\dagger} | u_{n} \rangle = \sqrt{\frac{\hbar}{2 m\omega}} ( \langle u_{n} | Au_{n} \rangle + \langle u_{n} | A^{\dagger} u_{n} \rangle )
```

```math
= \sqrt{\frac{\hbar}{2 m\omega}} ( \sqrt{n} \langle u_{n} | u_{n - 1} \rangle + \sqrt{n + 1} \langle u_{n} | u_{n + 1} \rangle ) = 0
```

We should have seen that coming. Since each term in the $`x`$ operator changes the eigenstate, the dot product with the original (orthogonal) state must give zero.

## <a id="s-10-6-2-the-expectation-value-of-p-in-eigenstate"></a>10.6.2 The expectation value of $`p`$ in eigenstate

(See the previous example is you want to see all the steps.) The expectation value of $`p`$ also gives zero.

```math
\langle u_{n} | p | u_{n} \rangle = - i \sqrt{\frac{m \hbar \omega}{2}} \langle u_{n} | A - A^{\dagger} | u_{n} \rangle = 0
```

<div align="right"><a href="book.pdf#page=177" target="_blank" title="This page in the PDF"><sup>[p. 177]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p178" href="book.pdf#page=178" target="_blank" title="This page in the PDF"><sup>[p. 178]</sup></a>

## <a id="s-10-6-3-the-expectation-value-of-x-in-the-state-frac-sqrt-1-2-u-0-u-1"></a>10.6.3 The expectation value of $`x`$ in the state $`\tfrac{1}{\sqrt{2}} ( u_{0} + u_{1} )`$.

```math
\frac{1}{2} \langle u_{0} + u_{1} | x | u_{0} + u_{1} \rangle = \frac{1}{2} \sqrt{\frac{\hbar}{2 m\omega}} \langle u_{0} + u_{1} | A + A^{\dagger} | u_{0} + u_{1} \rangle
```

```math
= \sqrt{\frac{\hbar}{8 m\omega}} \langle u_{0} + u_{1} | Au_{0} + Au_{1} + A^{\dagger} u_{0} + A^{\dagger} u_{1} \rangle
```

```math
= \sqrt{\frac{\hbar}{8 m\omega}} \langle u_{0} + u_{1} | 0 + \sqrt{1} u_{0} + \sqrt{1} u_{1} + \sqrt{2} u_{2} \rangle
```

```math
\begin{aligned} &= \sqrt{\frac{\hbar}{8 m\omega}} ( \sqrt{1} \langle u_{0} | u_{0} \rangle + \sqrt{1} \langle u_{0} | u_{1} \rangle \\ & \quad + \quad \sqrt{2} \sqrt{\langle u_{0} | u_{2} \rangle + \sqrt{1} \surd \langle u_{1} | u_{0} \rangle} \\ & \quad + \quad \sqrt{1 \langle u_{1} | u_{1} \rangle + \quad 2 \langle u_{1} | u_{2} \rangle )} \end{aligned}
```

```math
= \frac{\hbar}{8 m\omega} ( 1 + 1 )
```

```math
= \sqrt{\frac{\hbar}{2 m\omega}}
```

## <a id="s-10-6-4-the-expectation-value-of-frac-1-2-m-omega-2-x-2-in-eigenstate"></a>10.6.4 The expectation value of $`\tfrac{1}{2} m\omega^{2} x^{2}`$ in eigenstate

The expectation of $`x^{2}`$ will have some nonzero terms.

```math
\langle u_{n} | x^{2} | u_{n} \rangle = \frac{\hbar}{2 m\omega} \langle u_{n} | AA + AA^{\dagger} + A^{\dagger} A + A^{\dagger} A^{\dagger} | u_{n} \rangle
```

```math
= \frac{\hbar}{2 m\omega} \langle u_{n} | AA^{\dagger} + A^{\dagger} A | u_{n} \rangle
```

We could drop the $`AA`$ term and the $`A^{\dagger} A^{\dagger}`$ term since they will produce 0 when the dot product is taken.

```math
\langle u_{n} | x^{2} | u_{n} \rangle = \frac{\hbar}{2 m\omega} ( \langle u_{n} | \sqrt{n + 1} Au_{n + 1} \rangle + \langle u_{n} | \sqrt{n} A^{\dagger} u_{n - 1} \rangle )
```

```math
= \frac{\hbar}{2 m\omega} ( \langle u_{n} | \sqrt{n + 1} \sqrt{n + 1} u_{n} \rangle + \langle u_{n} | \sqrt{n} \sqrt{n} u_{n} \rangle )
```

```math
= \frac{\hbar}{2 m\omega} ( ( n + 1 ) + n ) = \left( n + \frac{1}{2} \right) \frac{\hbar}{m\omega}
```

With this we can compute the expected value of the potential energy.

```math
\langle u_{n} | \frac{1}{2} m\omega^{2} x^{2} | u_{n} \rangle = \frac{1}{2} m\omega^{2} \left( n + \frac{1}{2} \right) \frac{\hbar}{m\omega} = \frac{1}{2} \left( n + \frac{1}{2} \right) \hbar \omega = \frac{1}{2} E_{n}
```

<div align="right"><a href="book.pdf#page=178" target="_blank" title="This page in the PDF"><sup>[p. 178]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p179" href="book.pdf#page=179" target="_blank" title="This page in the PDF"><sup>[p. 179]</sup></a>

## <a id="s-10-6-5-the-expectation-value-of-2-p-2-m-in-eigenstate"></a>10.6.5 The expectation value of $`\frac{p^{2}}{2 m}`$ in eigenstate

The expectation value of $`\frac{p^{2}}{2 m}`$ is

```math
\langle u_{n} | \frac{p^{2}}{2 m} | u_{n} \rangle = \frac{- 1}{2 m} \frac{m \hbar \omega}{2} \langle u_{n} |- AA^{\dagger} - A^{\dagger} A | u_{n} \rangle
```

```math
= \frac{\hbar \omega}{4} \langle u_{n} | AA^{\dagger} + A^{\dagger} A | u_{n} \rangle
```

```math
= \frac{\hbar \omega}{4} ( ( n + 1 ) + n ) = \frac{1}{2} E_{n}
```

(See the previous section for a more detailed computation of the same kind.)

## <a id="s-10-6-6-time-development-example"></a>10.6.6 Time Development Example

Start off in the state at $`t`$ = 0.

```math
\psi ( t = 0 ) = \frac{1}{\sqrt{2}} ( u_{1} + u_{2} )
```

Now put in the simple time dependence of the energy eigenstates, $`e^{- iEt/ \hbar}`$.

```math
\psi ( t ) = \frac{1}{\sqrt{2}} ( u_{1} e^{- i \tfrac{3}{2} \omega t} + u_{2} e^{- i \tfrac{5}{2} \omega t} ) = \frac{1}{\sqrt{2}} e^{- i \tfrac{3}{2} \omega t} ( u_{1} + e^{- i\omega t} u_{2} )
```

We can compute the expectation value of $`p`$.

```math
\langle \psi | p | \psi \rangle = - i \sqrt{\frac{m \hbar \omega}{2}} \frac{1}{2} \langle u_{1} + e^{- i\omega t} u_{2} | A - A^{\dagger} | u_{1} + e^{- i\omega t} u_{2} \rangle
```

```math
= \sqrt{\frac{m \hbar \omega}{2}} \frac{1}{2 i} \left( \langle u_{1} | A | u_{2} \rangle e^{- i\omega t} - \langle u_{2} | A^{\dagger} | u_{1} \rangle e^{i\omega t} \right)
```

```math
= \sqrt{\frac{m \hbar \omega}{2}} \frac{1}{2 i} \left( \sqrt{2} e^{- i\omega t} - \sqrt{2} e^{i\omega t} \right)
```

```math
= - \sqrt{m \hbar \omega} \sin ( \omega t )
```

# <a id="s-10-7-sample-test-problems"></a>10.7 Sample Test Problems

1. A 1D harmonic oscillator is in a linear combination of the energy eigenstates

```math
\psi = \sqrt{\frac{2}{3}} u_{0} - i \sqrt{\frac{1}{3}} u_{1}
```

Find the expected value of $`p`$.

**Answer**

<div align="right"><a href="book.pdf#page=179" target="_blank" title="This page in the PDF"><sup>[p. 179]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p180" href="book.pdf#page=180" target="_blank" title="This page in the PDF"><sup>[p. 180]</sup></a>

```math
\langle \psi | p | \psi \rangle = - i \sqrt{\frac{m \hbar \omega}{2}} \begin{pmatrix} \sqrt{\frac{2}{3}} \quad \sqrt{\frac{1}{3}} \quad \sqrt{\frac{2}{3}} \quad \sqrt{\frac{1}{3}} \\ u_{0} - i \quad u_{1} | A - A^{\dagger} | \quad u_{0} - i \quad u_{1} \end{pmatrix}
```

```math
= - i \sqrt{\frac{m \hbar \omega}{2}} \left\langle \sqrt{\frac{2}{3}} u_{0} - i \sqrt{\frac{1}{3}} u_{1} |- \sqrt{\frac{2}{3}} u_{1} - i \sqrt{\frac{1}{3}} u_{0} \right\rangle
```

```math
= - i \sqrt{\frac{m \hbar \omega}{2}} ( - i \sqrt{\frac{2}{9}} - i \sqrt{\frac{2}{9}} )
```

```math
= - \sqrt{\frac{8}{9}} \sqrt{\frac{m \hbar \omega}{2}} = \frac{2}{3} \sqrt{m \hbar \omega}
```

2. Assuming $`u_{n}`$ represents the $`n^{th}`$ 1D harmonic oscillator energy eigenstate, calculate $`\langle u_{n} | p | u_{m} \rangle`$. **Answer**

```math
p = \sqrt{\frac{m \hbar \omega}{2}} \frac{A - A^{\dagger}}{i}
```

```math
\langle u n | p | u m \rangle = - i \sqrt{\frac{m \hbar \omega}{2}} \langle u n | A - A^{\dagger} | u m \rangle
```

```math
= - i \sqrt{\frac{m \hbar \omega}{2}} ( \sqrt{m} \delta_{n ( m - 1 )} - \sqrt{m + 1} \delta_{n ( m + 1 )} )
```

3. Evaluate the “uncertainty” in $`x`$ for the 1D HO ground state $`\sqrt{\langle u_{0} | ( x - \bar{x} )}`$<sup>2</sup>$`| u_{0} \rangle`$ where $`\bar{x} = \langle u_{0} | x | u_{0} \rangle`$. Similarly, evaluate the uncertainty in $`p`$ for the ground state. What is the product ∆$`p`$∆$`x`$?

**Answer**<br>
Its easy to see that $`\bar{x}`$ = 0 either from the integral or using operators. I’ll use operators to compute the rest.

```math
x = \sqrt{\frac{\hbar}{2 m\omega}} ( A + A^{\dagger} )
```

```math
\langle u_{0} | x^{2} | u_{0} \rangle = \frac{\hbar}{2 m\omega} \langle u_{0} | AA + AA^{\dagger} + A^{\dagger} A + A^{\dagger} A^{\dagger} ) | u_{0} \rangle
```

```math
= \frac{\hbar}{2 m\omega} \langle u_{0} | AA^{\dagger} | u_{0} \rangle = \frac{\hbar}{2 m\omega} 1 = \frac{\hbar}{2 m\omega}
```

```math
p = \frac{1}{i} \sqrt{\frac{m \hbar \omega}{2} ( A - A^{\dagger} )}
```

```math
\langle u_{0} | p^{2} | u_{0} \rangle = - \frac{m \hbar \omega}{2} \langle u_{0} |- AA^{\dagger} | u_{0} \rangle = \frac{m \hbar \omega}{2}
```

```math
\Delta x = \sqrt{\frac{\hbar}{2 m\omega}}
```

```math
\Delta p = \sqrt{\frac{m \hbar \omega}{2}}
```

```math
\Delta p \Delta x = \hbar
```

<div align="right"><a href="book.pdf#page=180" target="_blank" title="This page in the PDF"><sup>[p. 180]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p181" href="book.pdf#page=181" target="_blank" title="This page in the PDF"><sup>[p. 181]</sup></a>

4. Use the commutator relation between $`A`$ and $`A^{\dagger}`$ to derive $`[ H,A ]`$. Now show that $`A`$ is the

**lowering operator for the harmonic oscillator** energy.

5. At $`t`$ = 0, a **one dimensional harmonic oscillator** is in the state $`\psi ( t`$ = 0) = $`\sqrt{\frac{3}{4}} u_{0} + i \sqrt{\frac{1}{4}} u_{1}`$.

Calculate the expected value of $`p`$ as a function of time.

6. At $`t`$ = 0, a harmonic oscillator is in a linear combination of the $`n`$ = 1 and $`n`$ = 2 states.

```math
\psi = \sqrt{\frac{2}{3}} u_{1} - \sqrt{\frac{1}{3}} u_{2}
```

Find $`\langle x \rangle`$ and $`\langle x^{2} \rangle`$ as a function of time.

7. A 1D harmonic oscillator is in a linear combination of the energy eigenstates

```math
\psi = \sqrt{\frac{2}{3}} u_{0} + \sqrt{\frac{1}{3}} u_{2} .
```

```math
\text{ Find } \langle x^{2} \rangle .
```

```math
8 .
```

---

[← 9 One Dimensional Potentials](09-one-dimensional-potentials.md) · [11 More Fun with Operators →](11-more-fun-with-operators.md)
