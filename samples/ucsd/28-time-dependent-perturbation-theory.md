[← 27 Molecular Physics](27-molecular-physics.md) · [29 Radiation in Atoms →](29-radiation-in-atoms.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p391" href="book.pdf#page=391" target="_blank" title="This page in the PDF"><sup>[p. 391]</sup></a>

# 28 Time Dependent Perturbation Theory

We have used time independent perturbation theory to find the energy shifts of states and to find the change in energy eigenstates in the presence of a small perturbation. We will now consider the case of a perturbation that is time dependent. Such a perturbation can cause transitions between energy eigenstates. We will calculate the rate of those transitions.

This material is covered in **Gasiorowicz Chapter 21,** in **Cohen-Tannoudji et al. Chapter** **XIII,** and briefly in Griffiths Chapter 9.

# <a id="s-28-1-general-time-dependent-perturbations"></a>28.1 General Time Dependent Perturbations

Assume that we solve the unperturbed energy eigenvalue problem exactly: $`H_{0} \varphi_{n} = E_{n} \varphi_{n}`$. Now we add a perturbation that depends on time, $`\mathcal{V} ( t )`$. Our problem is now inherently time dependent so we go back to the **time dependent Schrödinger equation**.

```math
( H_{0} + \mathcal{V} ( t ) ) \psi ( t ) = i \hbar \frac{\partial \psi ( t )}{\partial t}
```

We will **expand** $`\psi`$ in terms of the eigenfunctions: $`\psi ( t ) = \sum c_{k} ( t ) \varphi_{k} e^{- iE_{k} t/ \hbar}`$ with $`c_{k} ( t ) e^{- iE_{k} t/ \hbar} =`$

```math
k
```

$`\langle \varphi_{k} | \psi ( t ) \rangle`$. The time dependent Schrödinger equations is

```math
\sum_{k} ( H_{0} + \mathcal{V} ( t ) ) c_{k} ( t ) e^{- iE_{k} t/ \hbar} \varphi_{k} = i \hbar \sum_{k} \frac{\partial c_{k} ( t ) e^{- iE_{k} t/ \hbar}}{\partial t} \varphi_{k}
```

```math
\sum_{k} c_{k} ( t ) e^{- iE_{k} t/ \hbar} ( E_{k} + \mathcal{V} ( t ) ) \varphi_{k} = \sum_{k} \left( i \hbar \frac{\partial c_{k} ( t )}{\partial t} + E_{k} c_{k} ( t ) \right) e^{- iE_{k} t/ \hbar} \varphi_{k}
```

```math
\sum_{k} \mathcal{V} ( t ) c_{k} ( t ) e^{- iE_{k} t/ \hbar} \varphi_{k} = i \hbar \sum_{k} \frac{\partial c_{k} ( t )}{\partial t} e^{- iE_{k} t/ \hbar} \varphi_{k}
```

Now dot $`\langle \varphi_{n} |`$ into this equation to get the time dependence of one coefficient.

```math
\sum_{k} \mathcal{V}_{nk} ( t ) c_{k} ( t ) e^{- iE_{k} t/ \hbar} = i \hbar \frac{\partial c_{n} ( t )}{\partial t} e^{- iE_{n} t/ \hbar}
```

```math
\frac{\partial c_{n} ( t )}{\partial t} = \frac{1}{i \hbar} \sum_{k} \mathcal{V}_{nk} ( t ) c_{k} ( t ) e^{i ( E_{n} - E_{k} ) t/ \hbar}
```

Assume that at $`t`$ = 0, we are in an **initial state** $`\psi ( t`$ = 0) = $`\varphi_{i}`$ and hence all the other $`c_{k}`$ are equal to zero: $`c_{k} = \delta_{ki}`$.

```math
\frac{\partial c_{n} ( t )}{\partial t} = \frac{1}{i \hbar} \left( \mathcal{V}_{ni} ( t ) e^{i\omega_{ni} t} + \sum_{k \neq i} \mathcal{V}_{nk} ( t ) c_{k} ( t ) e^{i\omega_{nk} t} \right)
```

<div align="right"><a href="book.pdf#page=391" target="_blank" title="This page in the PDF"><sup>[p. 391]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p392" href="book.pdf#page=392" target="_blank" title="This page in the PDF"><sup>[p. 392]</sup></a>

Now we want to calculate transition rates. To first order, all the $`c_{k} ( t )`$ are small compared to $`c_{i} ( t ) \approx`$= 1, so the sum can be neglected.

```math
\frac{\partial c_{n}^{( 1 )} ( t )}{\partial t} = \frac{1}{i \hbar} \mathcal{V}_{ni} ( t ) e^{i\omega_{ni} t}
```

```math
c_{n}^{( 1 )} ( t ) = \frac{1}{i \hbar} \int_{0}^{t} e^{i\omega_{ni} t^{\prime}} \mathcal{V}_{ni} ( t^{\prime} ) dt^{\prime}
```

This is the **equation to use to compute transition probabilities for a general time de-** **pendent perturbation**. We will also use it as a basis to compute transition rates for the specific problem of harmonic potentials. Again we are assuming $`t`$ is small enough that $`c_{i}`$ has not changed much. This is not a limitation. We can deal with the decrease of the population of the initial state later.

Note that, if there is a large energy difference between the initial and final states, a slowly varying perturbation can average to zero. We will find that the perturbation will need frequency components compatible with $`\omega_{ni}`$ to cause transitions.

If the first order term is zero or higher accuracy is required, the second order term can be computed. In second order, a transition can be made to an intermediate state $`\varphi_{k}`$, then a transition to $`\varphi_{n}`$. We just put the first order $`c_{k}^{( 1 )} ( t )`$ into the sum.

```math
\frac{\partial c_{n} ( t )}{\partial t} = \frac{1}{i \hbar} \left( \mathcal{V}_{ni} ( t ) e^{i\omega_{ni} t} + \sum_{k \neq i} \mathcal{V}_{nk} ( t ) c_{k}^{( 1 )} ( t ) e^{i\omega_{nk} t} \right)
```

```math
\begin{aligned} \frac{\partial c_{n} ( t )}{\partial t} &= \frac{1}{i \hbar} \left( \mathcal{V}_{ni} ( t ) e^{i\omega_{ni} t} + \sum_{k \neq i} \mathcal{V}_{nk} ( t ) \frac{1}{i \hbar} e^{i\omega_{nk} t} \int_{0}^{t} e^{i\omega_{ki} t^{\prime}} \mathcal{V}_{ki} ( t^{\prime} ) dt^{\prime} \right) \\ c_{n}^{( 2 )} ( t ) &= \frac{- 1}{\hbar^{2}} \sum \int_{k \neq i 0}^{t} dt^{\prime \prime} \mathcal{V}_{nk} ( t^{\prime \prime} ) e^{i\omega_{nk} t^{\prime \prime}} \int_{0}^{t^{\prime \prime}} dt^{\prime} e^{i\omega_{ki} t^{\prime}} \mathcal{V}_{ki} ( t^{\prime} ) \\ c_{n}^{( 2 )} ( t ) &= \frac{- 1}{\hbar^{2}} \sum \int_{k \neq i 0}^{t} dt^{\prime \prime} \mathcal{V}_{nk} ( t^{\prime \prime} ) e^{i\omega_{nk} t^{\prime \prime}} \int_{0}^{t^{\prime \prime}} dt^{\prime} e^{i\omega_{ki} t^{\prime}} \mathcal{V}_{ki} ( t^{\prime} ) \end{aligned}
```

\* See Example 28.3.1: Transitions of a 1D harmonic oscillator in a transient E field.\*

# <a id="s-28-2-sinusoidal-perturbations"></a>28.2 Sinusoidal Perturbations

An important case is a pure sinusoidal oscillating (harmonic) perturbation. We can **make up** **any time dependence from a linear combination** of sine and cosine waves. We define our

<div align="right"><a href="book.pdf#page=392" target="_blank" title="This page in the PDF"><sup>[p. 392]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p393" href="book.pdf#page=393" target="_blank" title="This page in the PDF"><sup>[p. 393]</sup></a>

perturbation carefully.

```math
\mathcal{V} ( \vec{r} , t ) = 2 V ( \vec{r} ) \cos ( \omega t ) \to 2 V \cos ( \omega t ) = V \left( e^{i\omega t} + e^{- i\omega t} \right)
```

We have introduced the factor of 2 for later convenience. With that factor, we have $`V`$ times a positive exponential plus a negative exponential. As before, $`V`$ depends on position but we don’t bother to write that for most of our calculations.

Putting this perturbation into the expression for $`c_{n} ( t )`$, we get

```math
c_{n} ( t ) = \frac{1}{i \hbar} \int_{0}^{t} e^{i\omega_{ni} t^{\prime}} \mathcal{V}_{ni} ( t^{\prime} ) dt^{\prime}
```

```math
= \frac{1}{i \hbar} V_{ni} \int_{0}^{t} dt^{\prime} e^{i\omega_{ni} t^{\prime}} \left( e^{i\omega t^{\prime}} + e^{- i\omega t^{\prime}} \right)
```

```math
= \frac{1}{i \hbar} V_{ni} \int_{0}^{t} dt^{\prime} \left( e^{i ( \omega_{ni} + \omega ) t^{\prime}} + e^{i ( \omega_{ni} - \omega ) t^{\prime}} \right)
```

Note that the terms in the time integral will **average to zero unless one of the exponents is** **nearly zero**. If one of the exponents is zero, the amplitude to be in the state $`\varphi_{n}`$ will increase with time. To make an exponent zero we must have one of two conditions satisfied.

```math
\omega = - \omega_{ni}
```

```math
\omega = - \frac{E_{n} - E_{i}}{\hbar}
```

```math
\hbar \omega = E_{i} - E_{n}
```

```math
E_{i} = E_{n} + \hbar \omega
```

This is energy conservation for the emission of a quantum of energy $`\hbar \omega`$.

```math
\omega = \omega_{ni}
```

```math
\omega = \frac{E_{n} - E_{i}}{\hbar}
```

```math
\hbar \omega = E_{n} - E_{i}
```

```math
E_{i} = E_{n} - \hbar \omega
```

This is energy conservation for the absorption of a quantum of energy $`\hbar \omega`$. We can see the possibility of absorption of radiation or of stimulated emission.

For $`t \to \infty`$, the time integral of the exponential gives (some kind of) **delta function of energy** **conservation**. We will expend some effort to determine exactly what delta function it is.

Lets take the **case of radiation of an energy quantum** $`\hbar \omega`$. If the initial and final states have energies such that this transition goes, the absorption term is completely negligible. (We can just use one of the exponentials at a time to make our formulas simpler.)

The amplitude to be in state $`\varphi_{n}`$ as a function of time is

```math
c_{n} ( t ) = \frac{1}{i \hbar} V_{ni} \int^{t} dt^{\prime} e^{i ( \omega_{ni} + \omega ) t^{\prime}}
```

<div align="right"><a href="book.pdf#page=393" target="_blank" title="This page in the PDF"><sup>[p. 393]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p394" href="book.pdf#page=394" target="_blank" title="This page in the PDF"><sup>[p. 394]</sup></a>

```math
= \frac{V_{ni}}{i \hbar} \left[ \frac{e^{i ( \omega_{ni} + \omega ) t^{\prime}}}{i ( \omega_{ni} + \omega )} \right] \frac{t^{\prime} = t}{t^{\prime} = 0}
```

```math
= \frac{V_{ni}}{i \hbar} \left[ \frac{e^{i ( \omega_{ni} + \omega ) t} - 1}{i ( \omega_{ni} + \omega )} \right]
```

```math
= \frac{V_{ni}}{i \hbar} e^{i ( \omega_{ni} + \omega ) t/ 2} \left[ \frac{e^{i ( \omega_{ni} + \omega ) t/ 2} - e^{- i ( \omega_{ni} + \omega ) t/ 2}}{i ( \omega_{ni} + \omega )} \right]
```

```math
= \frac{V_{ni}}{i \hbar} e^{i ( \omega_{ni} + \omega ) t/ 2} \frac{2 \sin ( ( \omega_{ni} + \omega ) t/ 2 )}{i ( \omega_{ni} + \omega )}
```

```math
P_{n} ( t ) = \frac{V_{ni}^{2}}{\hbar^{2}} \left[ \frac{4 \sin^{2} ( ( \omega_{ni} + \omega ) t/ 2 )}{( \omega_{ni} + \omega )^{2}} \right]
```

In the last line above we have squared the amplitude to get the probability to be in the final state. The last formula is appropriate to use, as is, for short times. For long times (compared to $`\frac{1}{\omega_{ni} + \omega}`$ which can be a VERY short time), the term in square brackets looks like some kind of delta function.

We will show (See section 28.4.1), that the quantity in square brackets in the last equation is $`2 \pi t\delta ( \omega_{ni} + \omega )`$. The probability to be in state $`\varphi_{n}`$ then is

```math
P_{n} ( t ) = \frac{V_{ni}^{2}}{\hbar^{2}} 2 \pi t\delta ( \omega_{ni} + \omega ) = \frac{2 \pi V_{ni}^{2}}{\hbar^{2}} \delta ( \omega_{ni} + \omega ) t = \frac{2 \pi V_{ni}^{2}}{\hbar} \delta ( E_{n} - E_{i} + \hbar \omega ) t
```

The probability to be in the final state $`\varphi_{n}`$ increases linearly with time. There is a delta function expressing energy conservation. The frequency of the harmonic perturbation must be set so that $`\hbar \omega`$ is the energy difference between initial and final states. This is true both for the (stimulated) emission of a quantum of energy and for the absorption of a quantum.

Since the probability to be in the final state increases linearly with time, it is reasonable to describe this in terms of a **transition rate**. The transition rate is then given by

```math
\Gamma_{i \to n} \equiv \frac{dP_{n}}{dt} = \frac{2 \pi V_{ni}^{2}}{\hbar} \delta ( E_{n} - E_{i} + \hbar \omega )
```

We would get a similar result for increasing $`E`$ (absorbing energy) from the other exponential.

```math
\Gamma_{i \to n} = \frac{2 \pi V_{ni}^{2}}{\hbar} \delta ( E_{n} - E_{i} - \hbar \omega )
```

It does not make a lot of sense to use this equation with a delta function to calculate the transition rate from a discrete state to a discrete state. If we tune the frequency just right we get infinity otherwise we get zero. This formula is what we need if either the initial or final state is a continuum state. If there is a free particle in the initial state or the final state, we have a continuum state. So, the absorption or emission of a particle, satisfies this condition.

The above results are very close to a transition rate formula known as **Fermi’s Golden Rule**. Imagine that instead of one final state $`\varphi_{n}`$ there are a **continuum of final states**. The total rate to that continuum would be obtained by integrating over final state energy, an integral done simply with the delta function. We then have

<div align="right"><a href="book.pdf#page=394" target="_blank" title="This page in the PDF"><sup>[p. 394]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p395" href="book.pdf#page=395" target="_blank" title="This page in the PDF"><sup>[p. 395]</sup></a>

```math
\Gamma_{i \to f} = \frac{2 \pi V_{ni}^{2}}{\hbar} \rho_{f} ( E )
```

where $`\rho_{f} ( E )`$ is the density of final states. When particles (like photons or electrons) are emitted, the final state will be a continuum due to the continuum of states available to a free particle. We will need to carefully compute the density of those states, often known as **phase space**.

# <a id="s-28-3-examples"></a>28.3 Examples

## <a id="s-28-3-1-harmonic-oscillator-in-a-transient-e-field"></a>28.3.1 Harmonic Oscillator in a Transient E Field

Assume we have an electron in a standard one dimensional harmonic oscillator of frequency $`\omega`$ in its ground state. An weak electric field is applied for a time interval $`T`$. Calculate the probability to make a transition to the first (and second) excited state.

The perturbation is $`eEx`$ for $`0 <t<T`$ and zero for other times. We can write this in terms of the raising an lowering operators.

```math
\mathcal{V} = eE \sqrt{\frac{\hbar}{2 m\omega}} ( A + A^{\dagger} )
```

We now use our time dependent perturbation result to compute the transition probability to the first excited state.

```math
c_{n} ( t ) = \frac{1}{i \hbar} \int_{0}^{t} e^{i\omega_{ni} t^{\prime}} \mathcal{V}_{ni} ( t^{\prime} ) dt^{\prime}
```

```math
c_{1} = \frac{1}{i \hbar} eE \int_{0}^{\sqrt{\frac{\hbar}{2 m\omega}}^{T}} e^{i\omega t^{\prime}} \langle 1 | A + A^{\dagger} | 0 \rangle dt^{\prime}
```

```math
= \frac{eE}{i \hbar} \int_{0}^{\sqrt{\frac{\hbar}{2 m\omega}}^{T}} e^{i\omega t^{\prime}} dt^{\prime}
```

```math
= \frac{eE}{i \hbar} \sqrt{\frac{\hbar}{2 m\omega}} \left[ \frac{e^{i\omega t^{\prime}}}{i\omega} \right] \frac{T}{0}
```

```math
= - \frac{eE}{\hbar \omega} \sqrt{\frac{\hbar}{2 m\omega}} \left[ e^{i\omega T} - 1 \right]
```

```math
= - \frac{eE}{\hbar \omega} \sqrt{\frac{\hbar}{2 m\omega}} e^{i\omega T/ 2} \left[ e^{i\omega T/ 2} - e^{- i\omega T/ 2} \right]
```

```math
= - \frac{eE}{\hbar \omega} \sqrt{\frac{\hbar}{2 m\omega}} e^{i\omega T/ 2} 2 i \sin ( \omega T/ 2 )
```

```math
P_{1} = \frac{e^{2} E^{2}}{\hbar^{2} \omega^{2}} \frac{\hbar}{2 m\omega} 4 \sin^{2} ( \omega T/ 2 )
```

```math
P_{1} = \frac{2 e^{2} E^{2}}{m \hbar \omega^{3}} \sin^{2} ( \omega T/ 2 )
```

<div align="right"><a href="book.pdf#page=395" target="_blank" title="This page in the PDF"><sup>[p. 395]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p396" href="book.pdf#page=396" target="_blank" title="This page in the PDF"><sup>[p. 396]</sup></a>

As long as the E field is weak, the initial state will not be significantly depleted and the assumption we have made concerning that is valid. We do see that the transition probability oscillates with the time during which the E field is applied. We would get a (much) larger transition probability if we applied an oscillating E field tuned to have the right frequency to drive the transition.

Clearly the probability to make a transition to the second excited state is zero in first order. If we really want to compute this, we can use our first order result for $`c_{1}`$ and calculate the transition probability to the $`n`$ = 2 state from that. This is a second order calculation. Its not too bad to do since there is only one intermediate state.

# <a id="s-28-4-derivations-and-computations"></a>28.4 Derivations and Computations

## <a id="s-28-4-1-the-delta-function-of-energy-conservation"></a>28.4.1 The Delta Function of Energy Conservation

For harmonic perturbations, we have derived a probability to be in the final state $`\varphi_{n}`$ proportional to the following.

```math
P_{n} \propto \left[ \frac{4 \sin^{2} ( ( \omega_{ni} + \omega ) t/ 2 )}{( \omega_{ni} + \omega )^{2}} \right]
```

For simplicity of analysis lets consider the characteristics of the function

```math
g ( \Delta \equiv \omega_{ni} + \omega ) = \left[ \frac{4 \sin^{2} ( ( \omega_{ni} + \omega ) t/ 2 )}{( \omega_{ni} + \omega )^{2} t^{2}} \right] \equiv \frac{4 \sin^{2} ( \Delta t/ 2 )}{\Delta^{2} t^{2}}
```

for values of $`t>> \tfrac{1}{\Delta}`$. (Note that we have divided our function to be investigated by $`t^{2}`$. For ∆ = 0, $`g`$(∆) = 1 while for all other values for ∆, $`g`$(∆) approaches zero for large $`t`$. This is clearly some form of a delta function.

To find out exactly what delta function it is, we need to integrate over ∆.

```math
\int_{-\infty}^{\infty} d \Delta f ( \Delta ) g ( \Delta ) = f ( \Delta = 0 ) \int_{-\infty}^{\infty} d \Delta g ( \Delta )
```

```math
= f ( \Delta = 0 ) \int_{-\infty}^{\infty} d \Delta \frac{4 \sin^{2} ( \Delta t/ 2 )}{\Delta^{2} t^{2}}
```

```math
= f ( \Delta = 0 ) \int_{-\infty}^{\infty} d \Delta \frac{4 \sin^{2} ( y )}{4 y^{2}}
```

```math
= f ( \Delta = 0 ) \frac{2}{t} \int_{-\infty}^{\infty} dy \frac{\sin^{2} ( y )}{y^{2}}
```

```math
= f ( \Delta = 0 ) \frac{2}{t} \int_{-\infty}^{\infty} dy \frac{\sin^{2} ( y )}{y^{2}}
```

We have made the substitution that $`y = \tfrac{\Delta t}{2}`$. The definite integral over $`y`$ just gives $`\pi`$ (consult your<span class="page" style="float:left;width:100%;margin:0.4em 0 0.6em"><span style="display:block;text-align:right"><a href="book.pdf#page=396" target="_blank" title="This page in the PDF"><sup>[p. 396]</sup></a></span><span style="display:block;height:2px;background:currentColor;opacity:0.25"></span><a id="p397" href="book.pdf#page=397" target="_blank" title="This page in the PDF"><sup>[p. 397]</sup></a></span> table of integrals), so the result is simple.

```math
\int_{-\infty}^{\infty} d \Delta f ( \Delta ) g ( \Delta ) = f ( \Delta = 0 ) \frac{2 \pi}{t}
```

```math
g ( \Delta ) = \frac{2 \pi}{t} \delta ( \Delta )
```

```math
\left[ \frac{4 \sin^{2} ( ( \omega_{ni} + \omega ) t/ 2 )}{( \omega_{ni} + \omega )^{2}} \right] = 2 \pi t\delta ( \omega_{ni} + \omega )
```

Q.E.D.

# <a id="s-28-5-homework-problems"></a>28.5 Homework Problems

1. A hydrogen atom is placed in an electric field which is uniform in space and turns on at $`t`$ = 0 then decays exponentially. That is, $`\vec{E} ( t )`$ = 0 for $`t< 0`$ and $`\vec{E} ( t ) = \vec{E}_{0} e^{- \gamma t}`$ for $`t> 0`$. What is the probability that, as $`t \to \infty`$, the hydrogen atom has made a transition to the 2$`p`$ state?

2. A one dimensional harmonic oscillator is in its ground state. It is subjected to the additional potential $`W = - e\xi x`$ for a a time interval $`\tau`$. Calculate the probability to make a transition to the first excited state (in first order). Now calculate the probability to make a transition to the second excited state. You will need to calculate to second order.

# <a id="s-28-6-sample-test-problems"></a>28.6 Sample Test Problems

1. A hydrogen atom is in a uniform electric field in the $`z`$ direction which turns on abruptly at $`t`$ = 0 and decays exponentially as a function of time, $`E ( t ) = E_{0} e^{- t/\tau}`$. The atom is initially in its ground state. Find the probability for the atom to have made a transition to the 2$`P`$ state as $`t \to \infty`$. You need not evaluate the radial part of the integral. What $`z`$ components of orbital angular momentum are allowed in the 2$`P`$ states generated by this transition?

---

[← 27 Molecular Physics](27-molecular-physics.md) · [29 Radiation in Atoms →](29-radiation-in-atoms.md)
