[← 3 Diffraction](03-diffraction.md) · [5 Wave Packets →](05-wave-packets.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p97" href="book.pdf#page=97" target="_blank" title="This page in the PDF"><sup>[p. 97]</sup></a>

# 4 The Solution: Probability Amplitudes

For **EM waves**, the intensity, and hence the probability to find a photon, is proportional to the square of the fields. The fields obey the wave equation. The fields from two slits can add constructively or destructively giving interference patterns. The $`E`$ and $`B`$ fields are 90 degrees out of phase and both contribute to the intensity.

We will **use the same ideas for electrons**, although the details of the field will vary a bit because electrons and photons are somewhat different kinds of particles. For both particles the wavelength is given by

```math
\lambda = \frac{h}{p}
```

and the frequency by

```math
E = h\nu = \hbar \omega .
```

We will use a **complex probability amplitude** $`\psi ( x,t )`$ for the electron. The real and imaginary parts are out of phase like the EM fields. The traveling wave with momentum $`p`$ and energy $`E`$ then is

```math
\psi ( x,t ) \propto e^{i ( kx - \omega t )} = e^{i ( px - Et ) / \hbar}
```

The **probability to find an electron is equal to the absolute square** of the complex probability amplitude.

```math
P ( x,t ) = | \psi ( x,t ) |^{2}
```

(We will overcome the problem that this probability is 1 everywhere for our simple wavefunction.)

**We have just put in most of the physics of Quantum Mechanics. Much of what we do** **for the rest of the course will be deduced from the paragraph above. Our input came** **from deBroglie and Plank, with support from experiments.**

Lets summarize the physics input again.

$`•`$ Free particles are represented by complex wave functions with a relationship between their particle properties – energy and momentum, and their wave properties – frequency and wavelength given by Plank and deBroglie.

$`•`$ The absolute square of the wavefunction gives the probability distribution function. Quantum

Mechanics only tells us the probability.

$`•`$ We can make superpositions of our free particle wave functions to make states that do not have definite momentum. We will find that any state can be made from the superposition of free particle states with different momentum.

We now have a wave-particle duality for all the particles, however, **physics now only tells us** **the probability** for some quantum events to occur. We have lost the complete predictive power of classical physics.

**Gasiorowicz Chapter 1**

**Rohlf Chapter 5**

<div align="right"><a href="book.pdf#page=97" target="_blank" title="This page in the PDF"><sup>[p. 97]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p98" href="book.pdf#page=98" target="_blank" title="This page in the PDF"><sup>[p. 98]</sup></a>

## <a id="s-griffiths-1-2-1-3"></a>Griffiths 1.2, 1.3

## <a id="s-cohen-tannoudji-et-al-chapter"></a>Cohen-Tannoudji et al. Chapter

# <a id="s-4-1-derivations-and-computations"></a>4.1 Derivations and Computations

## <a id="s-4-1-1-review-of-complex-numbers"></a>4.1.1 Review of Complex Numbers

This is a simple review, but, you must make sure you use complex numbers correctly. One of the most common mistakes in test problems is to forget to take the complex conjugate when computing a probability.

A complex number $`c = a + ib`$ consists of a real part $`a`$ and an imaginary part $`ib`$. (We choose $`a`$ and $`b`$ to be real numbers.) $`i`$ is the square root of -1.

The complex conjugate of $`c`$ is $`c^{*} = a - ib`$. (Just change the sign of all the $`i . )`$

The absolute square of a complex number is calculated by multiplying it by its complex conjugate.

```math
| c |^{2} = c^{*} c = ( a - ib ) ( a + ib ) = a^{2} + iab - iab + b^{2} = a^{2} + b^{2}
```

This give the magnitude squared of the complex number. The absolute square is always real.

We will use complex exponentials all the time.

```math
e^{i\theta} = \cos \theta + i \sin \theta
```

```math
e^{- i\theta} = \cos \theta - i \sin \theta
```

You can verify that the absolute square of these exponentials is always 1. They are often called a phase factor.

We can write $`\sin \theta = \tfrac{e^{i\theta} \quad e^{- i\theta}}{2 i}^{-}`$ and $`\cos \theta = \tfrac{e^{i\theta} + e^{- i\theta}}{2}`$.

As with other exponentials, we can multiply them by adding the exponents.

```math
e^{ikx} e^{- i\omega t} = e^{i ( kx - \omega t )}
```

## <a id="s-4-1-2-review-of-traveling-waves"></a>4.1.2 Review of Traveling Waves

A normal traveling wave may be given by

```math
\cos ( kx - \omega t ) .
```

The phase of the wave goes through 2$`\pi`$ in one wavelength in $`x`$. So the wavelength $`\lambda`$ satisfies

```math
k\lambda = 2 \pi .
```

<div align="right"><a href="book.pdf#page=98" target="_blank" title="This page in the PDF"><sup>[p. 98]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p99" href="book.pdf#page=99" target="_blank" title="This page in the PDF"><sup>[p. 99]</sup></a>

Similarly the phase goes through 2$`\pi`$ in one period $`\tau`$ in time.

```math
\omega \tau = 2 \pi
```

$`\omega`$ is the angular frequency. It changes by 2$`\pi`$ every cycle. The frequency $`\nu`$ increases by 1 every cycle so

```math
\omega = 2 \pi \nu .
```

There is no reason to memorize these equations. They should be obvious.

Lets see how fast one of the peaks of the wave moves. This is called the phase velocity. At time $`t`$ = 0, there is a peak at $`x`$ = 0. This is the peak for which the argument of cosine is 0. At time $`t`$ = 1, the argument is zero when $`kx = \omega t`$ or at $`x = \tfrac{\omega}{k}`$. If we compute the phase velocity by taking $`\frac{\Delta x}{\Delta t}`$, we get

```math
v_{\text{ phase }} = \frac{\omega}{k} .
```

That is, one of the peaks of this wave travels with a velocity of $`\frac{\omega}{k}`$.

```math
v = \frac{\omega}{k} = \frac{2 \pi \nu}{\frac{2 \pi}{\lambda}} = \nu \lambda
```

In non-relativistic QM, we have $`\hbar k = p , E = \hbar \omega`$, and $`E = \tfrac{p^{2}}{2 m}`$ , so

```math
\omega ( k ) = \frac{E}{\hbar} = \frac{\hbar^{2} k^{2}}{2 m \hbar} = \frac{\hbar k^{2}}{2 m}
```

<div align="center">

You may remember that a pulse will move at the group velocity which is given by

</div>

```math
v_{g} = \left( \frac{d\omega}{dk} \right) = \frac{2 \hbar k}{2 m} = \frac{\hbar k}{m} = \frac{p}{m} .
```

(The phase velocity for the non-relativistic case is $`v_{p} = \tfrac{p}{2 m} . )`$

# <a id="s-4-2-sample-test-problems"></a>4.2 Sample Test Problems

1. Write down the two (unnormalized) free particle wave functions for a particle of kinetic energy E. Include the proper time dependence and expressions for other constants in terms of E.

```math
2 .
```

---

[← 3 Diffraction](03-diffraction.md) · [5 Wave Packets →](05-wave-packets.md)
