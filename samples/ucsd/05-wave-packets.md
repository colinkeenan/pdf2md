[← 4 The Solution: Probability Amplitudes](04-the-solution-probability-amplitudes.md) · [6 Operators →](06-operators.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p100" href="book.pdf#page=100" target="_blank" title="This page in the PDF"><sup>[p. 100]</sup></a>

# 5 Wave Packets

## <a id="s-gasiorowicz-chapter-2"></a>Gasiorowicz Chapter 2

## <a id="s-rohlf-chapters-5"></a>Rohlf Chapters 5

## <a id="s-griffiths-chapter-2"></a>Griffiths Chapter 2

## <a id="s-cohen-tannoudji-et-al-chapter"></a>Cohen-Tannoudji et al. Chapter

# <a id="s-5-1-building-a-localized-single-particle-wave-packet"></a>5.1 Building a Localized Single-Particle Wave Packet

We now have a wave function for a **free particle with a definite momentum** $`p`$

```math
\psi ( x,t ) = e^{i ( px - Et ) / \hbar} = e^{i ( kx - \omega t )}
```

where the wave number $`k`$ is defined by $`p = \hbar k`$ and the angular frequency $`\omega`$ satisfies $`E = \hbar \omega`$. It is not localized since $`P ( x,t ) = | \psi ( x,t ) |^{2}`$ = 1 everywhere.

We would like a state which is localized and **normalized to one particle**.

```math
\int_{-\infty}^{\infty} \psi^{*} ( x,t ) \psi ( x,t ) dx = 1
```

To make a **wave packet which is localized in space**, we must add components of different wave number. Recall that we can use a Fourier Series (See section 5.6.1) to compose any function $`f ( x )`$ when we limit the range to $`- L<x<L`$. We do not want to limit our states in $`x`$, so we will take the limit that $`L \to \infty`$. In that limit, every wave number is allowed so the sum turns into an integral. The result is the very closely related Fourier Transform (See section 5.6.2)

```math
f ( x ) = \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{\infty} A ( k ) e^{ikx} dk
```

with coefficients which are computable,

```math
A ( k ) = \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{\infty} f ( x ) e^{- ikx} dx.
```

The normalizations of $`f ( x )`$ and $`A ( k )`$ are the same (with this **symmetric form**) and both can represent probability amplitudes.

```math
\int_{-\infty}^{\infty} f^{*} ( x ) f ( x ) dx = \int_{-\infty}^{\infty} A^{*} ( k ) A ( k ) dk
```

We understand $`f ( x )`$ as a **wave packet** made up of definite momentum terms $`e^{ikx}`$. The coefficient of each term is $`A ( k )`$. The probability for a particle to be found in a region $`dx`$ around some value of

<div align="right"><a href="book.pdf#page=100" target="_blank" title="This page in the PDF"><sup>[p. 100]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p101" href="book.pdf#page=101" target="_blank" title="This page in the PDF"><sup>[p. 101]</sup></a>

$`x`$ is $`| f ( x ) |^{2} dx`$. The probability for a particle to have wave number in region $`dk`$ around some value of $`k`$ is $`| A ( k ) |^{2} dk`$. (Remember that $`p = \hbar k`$ so the momentum distribution is very closely related. We work with $`k`$ for a while for economy of notation.)

# <a id="s-5-2-two-examples-of-localized-wave-packets"></a>5.2 Two Examples of Localized Wave Packets

Lets now try **two examples of a wave packet localized in** $`k`$ and properly normalized at $`t`$ = 0.

<div align="center">

1. A “square” packet: $`A ( k ) = \sqrt{\tfrac{1}{a}}`$ for $`k_{0} - \tfrac{a}{2} <k<k_{0} + \tfrac{a}{2}`$ and 0 elsewhere.

</div>

```math
2 . A \text{ Gaussian packet } : A ( k ) = \left( \tfrac{2 \alpha}{\pi} \right)^{1 / 4} e^{- \alpha ( k - k_{0} )^{2}} .
```

These are both localized in momentum about $`p = \hbar k_{0}`$.

Check the normalization of (1).

```math
\int_{-\infty}^{\infty} | A ( k ) |^{2} dk = \frac{1}{a} \int_{k_{0} - \tfrac{a}{2}}^{k_{0} + \tfrac{a}{2}} dk = \frac{1}{a} a = 1
```

Check the normalization of (2) using the result for a definite integral of a Gaussian (See section$`\infty`$

```math
5 . 6 . 3 ) \int_{-\infty} dxe^{- ax^{2}} = \sqrt{\frac{\pi}{a}} .
```

```math
\int_{-\infty}^{\infty} | A ( k ) |^{2} dk = \sqrt{\frac{2 \alpha}{\pi}} \int_{-\infty}^{\infty} e^{- 2 \alpha ( k - k_{0} )^{2}} dk = \sqrt{\frac{2 \alpha}{\pi}} \sqrt{\frac{\pi}{2 \alpha}} = 1
```

So now we take the Fourier Transform of (1) right here.

```math
f ( x ) = \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{\infty} A ( k ) e^{ikx} dk = \frac{1}{\sqrt{2 \pi}} \frac{1}{\sqrt{a}} \int_{k_{0} - {}^{a}}^{k_{0} + \tfrac{a}{2}} e^{ikx} dk
```

```math
f ( x ) = \frac{1}{\sqrt{2 \pi a}} \frac{1}{ix} \left[ e^{ikx} \right]_{k_{0} - \tfrac{\tfrac{a}{2}_{a}}{2}}^{k_{0} +} = \frac{1}{\sqrt{2 \pi a}} \frac{1}{ix} e^{ik_{0} x^{2}} \left[ e^{iax/ 2} - e^{- iax/ 2} \right]
```

```math
f ( x ) = \frac{1}{\sqrt{2 \pi a}} \frac{1}{ix} e^{ik_{0} x} \left[ 2 i \sin \left( \frac{ax}{2} \right) \right] = \sqrt{\frac{a}{2 \pi}} e^{ik_{0} x} \frac{2 \sin \left( \tfrac{ax}{2} \right)}{ax}
```

Note that $`\tfrac{2 \sin}{ax} ( \tfrac{ax}{2}`$ ) is equal to 1 at $`x`$ = 0 and that it decreases from there. If you square this, it should remind you of a single slit diffraction pattern! In fact, the single slit gives us a square localization in position space and the F.T. is this $`\frac{\sin ( x )}{x}`$ function.

The Fourier Transform of a Gaussian (See section 5.6.4) wave packet $`A ( k ) = \left( \tfrac{2 \alpha}{\pi} \right)^{1 / 4} e^{- \alpha ( k - k_{0} )^{2}}`$ is

```math
f ( x ) = \left( \frac{1}{2 \pi \alpha} \right)^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}
```

<div align="right"><a href="book.pdf#page=101" target="_blank" title="This page in the PDF"><sup>[p. 101]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p102" href="book.pdf#page=102" target="_blank" title="This page in the PDF"><sup>[p. 102]</sup></a>

also a Gaussian. We will show later that a Gaussian is the best one can do to localize a particle in position and momentum at the same time.

In both of these cases of $`f ( x )`$ (transformed from a normalized $`A ( k )`$ localized in momentum space) we see

$`•`$ A coefficient which **correctly normalizes** the state to 1,

$`• e^{ik_{0} x}`$ – a **wave corresponding to momentum** $`\hbar k_{0}`$,

$`•`$ and a packet function which is **localized** in $`x`$.

We have achieved our goal of finding **states that represent one free particle**. We see that we can have states which are localized both in position space and momentum space. We achieved this by making wave packets which are superpositions of states with definite momentum. The wave packets, while localized, have some width in $`x`$ and in $`p`$.

# <a id="s-5-3-the-heisenberg-uncertainty-principle"></a>5.3 The Heisenberg Uncertainty Principle

The wave packets we tried above satisfy an **uncertainty principle which is a property of waves**. That is ∆$`k`$∆$`x \geq \tfrac{1}{2}`$.

For the “square” packet the full width in $`k`$ is ∆$`k = a`$. The width in $`x`$ is a little hard to define, but, lets use the first node in the probability found at $`\tfrac{ax}{2} = \pi`$ or $`x =`$<sup>2</sup>$`a \pi`$. So the width is twice this or ∆$`x =`$<sup>4</sup>$`a \pi`$. This gives us

```math
\Delta k \Delta x = 4 \pi
```

which certainly satisfies the limit above. Note that if we change the width of $`A ( k )`$, the width of $`f ( x )`$ changes to keep the uncertainty product constant.

For the Gaussian wave packet, we can rigorously read the **RMS width of the probability distri-** **bution** as was done at the end of the section on the Fourier Transform of a Gaussian (See section 5.6.4).

```math
\sigma_{x} = \sqrt{\alpha}
```

```math
\sigma_{k} = \frac{1}{\sqrt{4 \alpha}}
```

We can again see that as we vary the width in k-space, the width in x-space varies to keep the product constant.

```math
\sigma_{x} \sigma_{k} = \frac{1}{2}
```

The **Gaussian wave packet gives the minimum uncertainty**. We will prove this later.

If we translate into momentum $`p = \hbar k`$, then

```math
\Delta p = \hbar \Delta k.
```

So the **Heisenberg Uncertainty Principle** states.

<div align="right"><a href="book.pdf#page=102" target="_blank" title="This page in the PDF"><sup>[p. 102]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p103" href="book.pdf#page=103" target="_blank" title="This page in the PDF"><sup>[p. 103]</sup></a>

```math
\Delta p \Delta x \geq \frac{\hbar}{2}
```

It says we cannot know the position of a particle and its momentum at the same time and tells us the limit of how well we can know them.

If we try to localize a particle to a very small region of space, its momentum becomes uncertain. If we try to make a particle with a definite momentum, its probability distribution spreads out over space.

# <a id="s-5-4-position-space-and-momentum-space"></a>5.4 Position Space and Momentum Space

We can represent a state with either $`\psi ( x )`$ or with $`\varphi ( p )`$. We can (Fourier) transform from one to the other.

We have the symmetric Fourier Transform.

```math
f ( x ) = \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{\infty} A ( k ) e^{ikx} dk
```

```math
A ( k ) = \frac{1}{\sqrt{2 \pi}} \int_{-\infty}^{\infty} f ( x ) e^{- ikx} dx
```

When we change variable from $`k`$ to $`p`$, we get the **Fourier Transforms in terms of** $`x`$ **and** $`p`$.

```math
\psi ( x ) = \frac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} \varphi ( p ) e^{ipx/ \hbar} dp
```

```math
\varphi ( p ) = \frac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} \psi ( x ) e^{- ipx/ \hbar} dx
```

These formulas are worth a little study. If we define $`u_{p} ( x )`$ to be the **state with definite momen-** **tum** $`p`$, (in position space) our formula for it is

```math
u_{p} ( x ) = \frac{1}{\sqrt{2 \pi \hbar}} e^{ipx/ \hbar} .
```

Similarly, the state (in momentum space) with **definite position** $`x`$ is

```math
v_{x} ( p ) = \frac{1}{\sqrt{2 \pi \hbar}} e^{- ipx/ \hbar}
```

These states cannot be normalized to 1 but they do have a normalization convention which is satisfied due to the constant shown.

<div align="right"><a href="book.pdf#page=103" target="_blank" title="This page in the PDF"><sup>[p. 103]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p104" href="book.pdf#page=104" target="_blank" title="This page in the PDF"><sup>[p. 104]</sup></a>

Our Fourier Transform can now be read to say that we **add up states of definite momentum** **to get** $`\psi ( x )`$

```math
\psi ( x ) = \int_{-\infty}^{\infty} \varphi ( p ) u_{p} ( x ) dp
```

and we add up states of definite position to get $`\varphi ( p )`$.

```math
\varphi ( p ) = \int_{-\infty}^{\infty} \psi ( x ) v_{x} ( p ) dx
```

There is a more abstract way to write these states. Using the notation of Dirac, the state with definite momentum $`p_{0} , u_{p 0} ( x ) = \sqrt{\tfrac{1}{2 \pi \hbar} e^{ip_{0} x/ \hbar}}`$ might be written as

```math
| p_{0} \rangle
```

and the state with definite position $`x_{1} , v_{x 1} ( p ) = \sqrt{\tfrac{1}{2 \pi \hbar} e^{- ipx_{1} / \hbar}}`$ might be written

```math
| x_{1} \rangle .
```

The arbitrary state represented by either $`\psi ( x )`$ or $`\varphi ( p )`$, might be written simple as

```math
| \psi \rangle .
```

The actual wave function $`\psi ( x )`$ would be written as

```math
\psi ( x ) = \langle x | \psi \rangle .
```

This gives us the amplitude to be at $`x`$ for any value of $`x`$.

We will find that there are other ways to represent Quantum states. This was a preview. We will spend more time on Dirac Bra-ket notation (See section 6.4) later.

# <a id="s-5-5-time-development-of-a-gaussian-wave-packet"></a>5.5 Time Development of a Gaussian Wave Packet \*

So far, we have performed our Fourier Transforms at $`t`$ = 0 and looked at the result only at $`t`$ = 0. We will now put time back into the wave function and look at the wave packet at later times. We will see that the behavior of photons and non-relativistic electrons is quite different.

Assume we **start with our Gaussian** (minimum uncertainty) wavepacket $`A ( k ) = e^{- \alpha ( k - k_{0} )^{2}}`$ at $`t`$ = 0. We can do the Fourier Transform to position space, including the time dependence.

```math
\psi ( x,t ) = \int_{-\infty}^{\infty} A ( k ) e^{i ( kx - \omega ( k ) t )} dk
```

We write explicitly that $`\omega`$ depends on $`k`$. For our free particle, this just means that the energy depends on the momentum. For a photon, $`E = pc`$, so $`\hbar \omega = \hbar kc`$, and hence $`\omega = kc`$. For an non-relativistic electron, $`E = \tfrac{p^{2}}{2 m}`$ , so $`\hbar \omega = \tfrac{\hbar^{2} k^{2}}{2 m}`$, and hence $`\omega = \tfrac{\hbar k^{2}}{2 m}`$.

<div align="right"><a href="book.pdf#page=104" target="_blank" title="This page in the PDF"><sup>[p. 104]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p105" href="book.pdf#page=105" target="_blank" title="This page in the PDF"><sup>[p. 105]</sup></a>

To cover the general case, lets expand $`\omega ( k )`$ around the center of the wave packet in k-space.

```math
\omega ( k ) = \omega ( k_{0} ) + \frac{d\omega}{dk} \left| {}_{k_{0}} ( k - k_{0} ) + \frac{1}{2} \frac{d^{2} \omega}{dk^{2}} \right|_{k_{0}} ( k - k_{0} )^{2}
```

We anticipate the outcome a bit and name the coefficients.

```math
\omega ( k ) = \omega_{0} + v_{g} ( k - k_{0} ) + \beta ( k - k_{0} )^{2}
```

For the photon, $`v_{g} = c`$ and $`\beta`$ = 0. For the NR electron, $`v_{g} = \tfrac{\hbar k_{0}}{m}`$ and $`\beta = \tfrac{\hbar}{2 m}`$.

Performing the Fourier Transform (See section 5.6.5), we get

```math
\psi ( x,t ) = \sqrt{\alpha + \pi i\beta t} e^{i ( k_{0} x - \omega_{0} t )} e^{\tfrac{- ( x - vgt ) 2}{4 ( \alpha + i\beta t )}}
```

```math
| \psi ( x,t ) |^{2} = \frac{\pi}{\sqrt{\alpha^{2} + \beta^{2} t^{2}}} e^{\tfrac{- \alpha ( x - vgt ) 2}{2 ( \alpha 2 + \beta 2 t 2 )}} .
```

We see that the photon will move with the velocity of light and that the wave packet will not disperse, because $`\beta`$ = 0.

For the NR electron, the wave packet moves with the correct **group velocity**, $`v_{g} = \tfrac{p}{m}`$, but the wave packet **spreads with time**. The RMS width is $`\sigma = \sqrt{\alpha^{2} + \left( \tfrac{\hbar t}{2 m} \right)^{2}}`$.

A wave packet naturally spreads because it contains waves of different momenta and hence different velocities. Wave packets that are very localized in space spread rapidly.

# <a id="s-5-6-derivations-and-computations"></a>5.6 Derivations and Computations

## <a id="s-5-6-1-fourier-series"></a>5.6.1 Fourier Series \*

Fourier series allow us to **expand any periodic function** on the range $`( - L,L )`$, in terms of sines and cosines also periodic on that interval.

```math
f ( x ) = \sum_{n = 0}^{\infty} A_{n} \cos \left( \frac{n\pi x}{L} \right) + \sum_{n = 1}^{\infty} B_{n} \sin \left( \frac{n\pi x}{L} \right)
```

Since the sines and cosines can be made from the complex exponentials, we can equally well use them for our basis for expansion. This has the nice simplification of having only one term in the sum, using negative $`n`$ to get the other term.

```math
f ( x ) = \sum_{n = -\infty}^{\infty} a_{n} e^{\tfrac{in\pi x}{L}}
```

The exponentials are orthogonal and normalized over the interval (as were the sines and cosines)

```math
\frac{1}{2 L} \int_{- L}^{L} e^{\tfrac{in\pi x}{L}} e^{\tfrac{- im\pi x}{L}} dx = \delta_{nm}
```

<div align="right"><a href="book.pdf#page=105" target="_blank" title="This page in the PDF"><sup>[p. 105]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p106" href="book.pdf#page=106" target="_blank" title="This page in the PDF"><sup>[p. 106]</sup></a>

so that we can easily compute the coefficients.

```math
a_{n} = \frac{1}{2 L} \int_{- L}^{L} f ( x ) e^{\tfrac{- in\pi x}{L}} dx
```

In summary, the Fourier series equations we will use are

```math
f ( x ) = \sum_{n = -\infty}^{\infty} a_{n} e^{\tfrac{in\pi x}{L}}
```

and

```math
a_{n} = \frac{1}{2 L} \int_{- L}^{L} f ( x ) e^{\tfrac{- in\pi x}{L}} dx.
```

We will expand the interval to infinity.

## <a id="s-5-6-2-fourier-transform"></a>5.6.2 Fourier Transform \*

To allow wave **functions to extend to infinity**, we will expand the interval used

```math
L \to \infty .
```

As we do this we will use the wave number

```math
k = \frac{n\pi}{L} .
```

As $`L \to \infty . , k`$ can take on any value, implying we will have a continuous distribution of $`k`$. Our sum over $`n`$ becomes an integral over $`k`$.

```math
dk = \frac{\pi}{L} dn.
```

If we define $`A ( k ) = \sqrt{\frac{2}{\pi}} La_{n}`$, we can make the transform come out with the constants we want.

```math
f ( x ) = {}_{n =} \sum^{\infty}_{-\infty} a_{n} e^{\tfrac{in\pi x}{L}} \quad \text{ Standard Fourier Series }
```

```math
a_{n} = \tfrac{1}{2 L} \int_{- L}^{L} f ( x ) e \tfrac{- in\pi x}{L} dx \quad \text{ Standard Fourier Series }
```

```math
A_{n} = \sqrt{\frac{2}{\pi}} L \tfrac{1}{2 L} \int_{- L}^{L} f ( x ) e^{-} \tfrac{in\pi x}{L} dx \quad \text{ redefine coefficient }
```

```math
A_{n} = \sqrt{\tfrac{1}{2 \pi} \int_{- L}^{L} f ( x ) e^{-} \tfrac{in\pi x}{L} dx}
```

```math
f ( x ) = \sqrt{\frac{\pi}{2}} \tfrac{1}{L}_{\infty n =} \sum_{-\infty}^{\infty} A_{n} e \tfrac{in\pi x}{L} \quad f \text{ stays the same }
```

<div align="center">

$`f ( x ) = \tfrac{\sqrt{\pi}}{\sqrt{2} L} \int A ( k ) e_{\overline{\pi}}^{ikxL} dk`$ but is rewritten in new A and dk

</div>

```math
f ( x ) = \tfrac{1}{2 \pi} \sqrt{\int_{-\infty}^{-\infty \infty} A ( k ) e^{ikx} dk \quad \text{ result }}
```

```math
A ( k ) = \tfrac{1}{2 \pi} \sqrt{\int_{-\infty}^{\infty} f ( x ) e^{- ikx} dx \quad \text{ result }}
```

<div align="right"><a href="book.pdf#page=106" target="_blank" title="This page in the PDF"><sup>[p. 106]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p107" href="book.pdf#page=107" target="_blank" title="This page in the PDF"><sup>[p. 107]</sup></a>

This is just the extension of the Fourier series to all $`x`$.

If $`f ( x )`$ is normalized, then $`A ( k )`$ will also be normalized with this (symmetric) form of the Fourier Transform. Thus, if $`f ( x )`$ is a probability amplitude in position-space, $`A ( k )`$ can be a probability amplitude (in k-space).

## <a id="s-5-6-3-integral-of-gaussian"></a>5.6.3 Integral of Gaussian

This is just a slick derivation of the definite integral of a Gaussian from minus infinity to infinity. With other limits, the integral cannot be done analytically but is tabulated. Functions are available in computer libraries to return this important integral.

The answer is $`\infty`$

```math
\int_{-\infty} dxe^{- ax^{2}} = \sqrt{\frac{\pi}{a} .}
```

Define $`\infty`$

```math
I = \int_{-\infty} dxe^{- ax^{2}} .
```

Integrate over both $`x`$ and $`y`$ so that

```math
I^{2} = \int_{-\infty}^{\infty} dxe^{- ax^{2}} \int_{-\infty}^{\infty} dye^{- ay^{2}} = \int_{-\infty -\infty}^{\infty} \int^{\infty} dxdye^{- a ( x^{2} + y^{2} )} .
```

Transform to polar coordinates.

```math
I^{2} = 2 \pi \int_{0}^{\infty} rdre^{- ar^{2}} = \pi \int_{0}^{\infty} d ( r^{2} ) e^{- ar^{2}} = \pi \left[ - \frac{1}{a} e^{- ar^{2}} \right] \frac{\infty}{0} = \frac{\pi}{a}
```

Now just take the square root to get the answer above.

```math
\int_{-\infty}^{\infty} dxe^{- ax^{2}} = \sqrt{\frac{\pi}{a} .}
```

Other forms can be obtained by differentiating with respect to $`a`$.

```math
\frac{\partial}{\partial a} \int_{-\infty}^{\infty} dxe^{- ax^{2}} = \frac{\partial}{\partial a} \sqrt{\frac{\pi}{a}}
```

```math
\int_{-\infty}^{\infty} dxx^{2} e^{- ax^{2}} = \frac{1}{2 a} \sqrt{\frac{\pi}{a}}
```

<div align="right"><a href="book.pdf#page=107" target="_blank" title="This page in the PDF"><sup>[p. 107]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p108" href="book.pdf#page=108" target="_blank" title="This page in the PDF"><sup>[p. 108]</sup></a>

## <a id="s-5-6-4-fourier-transform-of-gaussian"></a>5.6.4 Fourier Transform of Gaussian \*

We wish to **Fourier transform the Gaussian wave packet** in (momentum) k-space $`A ( k ) = \left( \tfrac{2 \alpha}{\pi} \right)^{1 / 4} e^{- \alpha ( k - k_{0} )^{2}}`$ to get $`f ( x )`$ in position space. The Fourier Transform formula is

```math
f ( x ) = \frac{1}{\sqrt{2 \pi}} \left( \frac{2 \alpha}{\pi} \right)^{1 / 4} \int_{-\infty}^{\infty} e^{- \alpha ( k - k_{0} )^{2}} e^{ikx} dk.
```

Now we will transform the integral a few times to get to the standard definite integral of a Gaussian for which we know the answer. First,

```math
k^{\prime} = k - k_{0}
```

which does nothing really since $`dk^{\prime} = dk`$.

```math
f ( x ) = \left( \frac{\alpha}{2 \pi^{3}} \right)^{1 / 4} e^{ik_{0} x} \int_{-\infty}^{\infty} e^{- \alpha ( k - k_{0} )^{2}} e^{i ( k - k_{0} ) x} dk
```

```math
f ( x ) = \left( \frac{\alpha}{2 \pi^{3}} \right)^{1 / 4} e^{ik_{0} x} \int_{-\infty}^{\infty} e^{- \alpha k^{\prime 2}} e^{ik^{\prime} x} dk^{\prime}
```

Now we want to **complete the square** in the exponent inside the integral. We plan a term like $`e^{- \alpha k^{\prime \prime 2}}`$ so we define

```math
k^{\prime \prime} = k^{\prime} - \frac{ix}{2 \alpha} .
```

Again $`dk^{\prime \prime} = dk^{\prime} = dk`$. Lets write out the planned exponent to see what we are missing.

```math
- \alpha \left( k^{\prime} - \frac{ix}{2 \alpha} \right)^{2} = - \alpha k^{\prime 2} + ik^{\prime} x + \frac{x^{2}}{4 \alpha}
```

We need to multiply by $`e^{- \tfrac{x 2}{4 \alpha}}`$ to cancel the extra term in the completed square.

```math
f ( x ) = \left( \frac{\alpha}{2 \pi^{3}} \right)^{1 / 4} e^{ik_{0} x} \int_{-\infty}^{\infty} e^{- \alpha ( k^{\prime} - \tfrac{ix}{2 \alpha} )^{2}} e^{- \tfrac{x 2}{4 \alpha}} dk^{\prime}
```

That term can be pulled outside the integral since it doesn’t depend on $`k`$.

```math
f ( x ) = \left( \frac{\alpha}{2 \pi^{3}} \right)^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}} \int_{-\infty}^{\infty} e^{- \alpha k^{\prime \prime 2}} dk^{\prime \prime}
```

So now we have the standard Gaussian integral which just gives us $`\sqrt{\frac{\pi}{\alpha}}`$.

```math
f ( x ) = \left( \frac{\alpha}{2 \pi^{3}} \right)^{1 / 4} \sqrt{\frac{\pi}{\alpha}} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}
```

```math
f ( x ) = \left( \frac{1}{2 \pi \alpha} \right)^{1 / 4} e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}
```

<div align="right"><a href="book.pdf#page=108" target="_blank" title="This page in the PDF"><sup>[p. 108]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p109" href="book.pdf#page=109" target="_blank" title="This page in the PDF"><sup>[p. 109]</sup></a>

Lets **check the normalization**.

```math
\int_{-\infty}^{\infty} | f ( x ) |^{2} dx = \sqrt{\frac{1}{2 \pi \alpha}} \int_{-\infty}^{\infty} e^{- \tfrac{x 2}{2 \alpha}} dx = \sqrt{\frac{1}{2 \pi \alpha}} \sqrt{2 \alpha \pi} = 1
```

Given a normalized $`A ( k )`$, we get a normalized $`f ( x )`$.

The RMS deviation, or standard deviation of a Gaussian can be read from the distribution.

```math
P ( x ) = \frac{1}{\sqrt{2 \pi \sigma^{2}}} e^{- \tfrac{( x - X ) 2}{2 \sigma 2}}
```

Squaring $`f ( x )`$, we get

```math
P ( x ) = \sqrt{\frac{1}{2 \pi \alpha}} e^{- \tfrac{x 2}{2 \alpha}} .
```

Reading from either the coefficient or the exponential we see that

```math
\sigma_{x} = \sqrt{\alpha}
```

For the width in k-space,

```math
P ( k ) = \sqrt{\frac{2 \alpha}{\pi}} e^{- 2 \alpha ( k - k_{0} )^{2}} .
```

Reading from the coefficient of the exponential, we get

```math
\sigma_{k} = \frac{1}{\sqrt{4 \alpha}} .
```

We can see that as we vary the width in k-space, the width in x-space varies to keep the product constant.

```math
\sigma_{x} \sigma_{k} = \frac{1}{2}
```

Translating this into momentum, we get the limit of the **Heisenberg Uncertainty Principle**.

```math
\sigma_{x} \sigma_{p} = \frac{\hbar}{2}
```

In fact the Uncertainty Principle states that

```math
\sigma_{x} \sigma_{p} \geq \frac{\hbar}{2}
```

so the Gaussian wave packets seem to saturate the bound!

## <a id="s-5-6-5-time-dependence-of-a-gaussian-wave-packet"></a>5.6.5 Time Dependence of a Gaussian Wave Packet \*

Assume we start with our Gaussian (minimum uncertainty) wavepacket $`A ( k ) = e^{- \alpha ( k - k_{0} )^{2}}`$ at $`t`$ = 0. We are not interested in careful normalization here so we will drop constants.

```math
\psi ( x,t ) = \int_{-\infty}^{\infty} A ( k ) e^{i ( kx - \omega ( k ) t )} dk
```

<div align="right"><a href="book.pdf#page=109" target="_blank" title="This page in the PDF"><sup>[p. 109]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p110" href="book.pdf#page=110" target="_blank" title="This page in the PDF"><sup>[p. 110]</sup></a>

We write explicitly that $`w`$ depends on $`k`$. For our free particle, this just means that the energy depends on the momentum. To cover the general case, lets expand $`\omega ( k )`$ around the center of the wave packet in k-space.

```math
\omega ( k ) = \omega ( k_{0} ) + \frac{d\omega}{dk} |_{k_{0}} ( k - k_{0} ) + \frac{1}{2} \frac{d^{2} \omega}{dk^{2}} |_{k_{0}} ( k - k_{0} )^{2}
```

We anticipate the outcome a bit and name the coefficients.

```math
\omega ( k ) = \omega_{0} + v_{g} ( k - k_{0} ) + \beta ( k - k_{0} )^{2}
```

We still need to do the integral as before. Make the substitution $`k^{\prime} = k - k_{0}`$ giving $`A ( k^{\prime} ) = e^{- \alpha k^{\prime 2}}`$. Factor out the constant exponential that has no $`k^{\prime}`$ dependence.

```math
\psi ( x,t ) = e^{i ( k_{0} x - w_{0} t )} \int_{-\infty}^{\infty} A ( k^{\prime} ) e^{i ( k^{\prime} x - v_{g} t )} e^{- ik^{\prime 2} \beta t} dk^{\prime}
```

```math
\psi ( x,t ) = e^{i ( k_{0} x - w_{0} t )} \int_{-\infty}^{\infty} e^{- \alpha k^{\prime 2}} e^{i ( k^{\prime} x - v_{g} t )} e^{- ik^{\prime 2} \beta t} dk^{\prime}
```

```math
\psi ( x,t ) = e^{i ( k_{0} x - w_{0} t )} \int_{-\infty}^{\infty} e^{- [ \alpha - i\beta t ] k^{\prime 2}} e^{i ( k^{\prime} x - v_{g} t )} dk^{\prime}
```

We now **compare this integral to the one we did earlier** (so we can avoid the work of completing the square again). Dropping the constants, we had

```math
f ( x ) = e^{ik_{0} x} \int_{-\infty}^{\infty} e^{- \alpha k^{\prime 2}} e^{ik^{\prime} x} dk^{\prime} = e^{ik_{0} x} e^{- \tfrac{x 2}{4 \alpha}}
```

Our new integral is the same with the **substitutions** $`k_{0} x \to k_{0} x - \omega_{0} t , k^{\prime} x \to k^{\prime} ( x - v_{g} t )`$, and $`\alpha \to ( \alpha + i\beta t )`$. We can then write down the answer

```math
\psi ( x,t ) = \sqrt{\alpha + \pi i\beta t} e^{i ( k_{0} x - \omega_{0} t )} e^{\tfrac{- ( x - vgt ) 2}{4 ( \alpha + i\beta t )}}
```

```math
| \psi ( x,t ) |^{2} = \sqrt{\alpha + \pi i\beta t} \sqrt{\alpha - \pi i\beta t} e^{\tfrac{- ( x - vgt ) 2}{4 ( \alpha + i\beta t )}} e^{\tfrac{- ( x - vgt ) 2}{4 ( \alpha - i\beta t )}}
```

```math
| \psi ( x,t ) |^{2} = \frac{\pi}{\sqrt{\alpha^{2} + \beta^{2} t^{2}}} e^{\tfrac{- \alpha ( x - vgt ) 2}{2 ( \alpha 2 + \beta 2 t 2 )}}
```

## <a id="s-5-6-6-numbers"></a>5.6.6 Numbers

The convenient unit of energy (mass and momentum too) is the electron volt.

```math
1 eV = 1 . 6 0 2 \times 1 0^{- 1 2} \text{ erg } = 1 . 6 0 2 \times 1 0^{- 1 9} \text{ Joule }
```

<div align="right"><a href="book.pdf#page=110" target="_blank" title="This page in the PDF"><sup>[p. 110]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p111" href="book.pdf#page=111" target="_blank" title="This page in the PDF"><sup>[p. 111]</sup></a>

<div align="center">

Use the fine structure constant to avoid CGS units which are used in the textbook.

</div>

```math
\alpha = \frac{e^{2}}{\hbar c} = 1 / 1 3 7
```

This combination saves a lot of work.

```math
\hbar c = 1 9 7 3 \text{ eV } ˚ A = 1 9 7 . 3 \text{ MeVF }
```

```math
1 ˚ A = 1 . 0 \times 1 0^{- 1 0} m
```

```math
1 \text{ Fermi } = 1 . 0 \times 1 0^{- 1 5} m
```

The Bohr radius gives the size of the Hydrogen atom.

```math
a_{0} = \frac{\hbar}{\alpha m_{e} c} = 0 . 5 2 9 \times 1 0^{- 1 0} m
```

```math
m_{p} = 9 3 8 . 3 \text{ MeV } / c^{2}
```

```math
m_{n} = 9 3 9 . 6 \text{ MeV } / c^{2}
```

```math
m_{e} = 0 . 5 1 1 \text{ MeV } / c^{2}
```

## <a id="s-5-6-7-the-dirac-delta-function"></a>5.6.7 The Dirac Delta Function

The **Dirac delta function is zero everywhere except at the point where its argument is** **zero**. At that point, it is just the right kind of infinity so that

```math
\int_{-\infty}^{\infty} dxf ( x ) \delta ( x ) = f ( 0 ) .
```

This is the definition of the delta function. It picks of the value of the function $`f ( x )`$ at the point where the argument of the delta function vanishes. A simple extension of the definition gives.

```math
\int_{-\infty}^{\infty} dxf ( x ) \delta ( x - a ) = f ( a )
```

The transformation of an integral allows us to compute

```math
\int_{-\infty}^{\infty} dxf ( x ) \delta ( g ( x ) ) = \left[ \frac{1}{| \tfrac{dg}{dx} |} f ( x ) \right]_{g ( x ) = 0}
```

the effect of the argument being a function.

If we make a wave packet in p-space using the delta function, and we transform to position space,

```math
\psi ( x ) = \frac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} \delta ( p - p_{0} ) e^{ipx/ \hbar} dp = \frac{1}{\sqrt{2 \pi \hbar}} e^{ip_{0} x/ \hbar}
```

we just get the state of definite $`p`$.

This is a state of definite momentum written in momentum space. $`\delta ( p - p_{0} )`$<br>
Its Fourier transform is $`\psi_{p} ( x,t ) = \tfrac{1}{\sqrt{2 \pi \hbar}} e^{i ( px - Et ) / \hbar}`$<br>
This is a state of definite position written in position space. $`\delta ( x - x_{0} )`$

<div align="right"><a href="book.pdf#page=111" target="_blank" title="This page in the PDF"><sup>[p. 111]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p112" href="book.pdf#page=112" target="_blank" title="This page in the PDF"><sup>[p. 112]</sup></a>

# <a id="s-5-7-examples"></a>5.7 Examples

## <a id="s-5-7-1-the-square-wave-packet"></a>5.7.1 The Square Wave Packet

Given the following one dimensional probability amplitude in the position variable x, compute the probability distribution in momentum space. Show that the uncertainty principle is roughly satisfied.

```math
\psi ( x ) = \frac{1}{\sqrt{2 a}}
```

for $`- a<x<a`$, otherwise $`\psi ( x )`$ = 0.

Its normalized.

```math
\int_{-\infty}^{\infty} \psi^{*} \psi dx = \int_{- a}^{a} \frac{1}{2 a} dx = 1
```

Take the Fourier Transform.

```math
\varphi ( k ) = \frac{1}{\sqrt{2 \pi}} \int_{- a}^{a} \frac{1}{\sqrt{2 a}} e^{- ikx} dx
```

```math
\varphi ( k ) = \frac{1}{\sqrt{4 \pi a}} \left[ \frac{- 1}{ik} e^{- ikx} \right] \frac{a}{- a}
```

```math
\varphi ( k ) = \frac{1}{ik \sqrt{4 \pi a}} \left[ e^{- ika} - e^{ika} \right]
```

```math
\varphi ( k ) = \frac{1}{ik \sqrt{4 \pi a}} [ - 2 i \sin ka ] = - \sqrt{\frac{1}{\pi a}} \frac{\sin ( ka )}{k}
```

Now estimate the width of the two probability distributions.

```math
\Delta x_{2} = 2 a
```

```math
| \varphi ( k ) |^{2} = \frac{1}{\pi a} \frac{\sin ( ka )}{k^{2}}
```

```math
\Delta k = \frac{2 \pi}{a}
```

```math
\Delta x \Delta k = 4 \pi
```

## <a id="s-5-7-2-the-gaussian-wave-packet"></a>5.7.2 The Gaussian Wave Packet \*

Given the following one dimensional probability amplitude in the position variable x, compute the probability distribution in momentum space. Show that the uncertainty principle is roughly satisfied. $`\psi ( x )`$ = ($`\tfrac{\alpha}{\pi} ) \tfrac{1}{4} e^{- \alpha x^{2} / 2}`$

<div align="right"><a href="book.pdf#page=112" target="_blank" title="This page in the PDF"><sup>[p. 112]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p113" href="book.pdf#page=113" target="_blank" title="This page in the PDF"><sup>[p. 113]</sup></a>

## <a id="s-5-7-3-the-dirac-delta-function-wave-packet"></a>5.7.3 The Dirac Delta Function Wave Packet \*

Given the following one dimensional probability amplitude in the position variable x, compute the probability distribution in momentum space. Show that the uncertainty principle is roughly satisfied. $`\psi ( x ) = \delta ( x - x_{0} )`$

## <a id="s-5-7-4-can-i-see-inside-an-atom"></a>5.7.4 Can I “See” inside an Atom

To see inside an atom, we must use light with a wavelength smaller than the size of the atom. With normal light, once a surface is polished down to the .25 micron level, it looks shiny. You can no longer see defects. So to see inside the atom, we would need light with $`\lambda = \tfrac{h}{p}`$ = 0$`. 1`$ A.$`( )`$˚

```math
p = \frac{2 \pi \hbar}{0 . 1}
```

```math
pc = \frac{2 \pi \hbar c}{0 . 1} = \frac{2 \pi 1 9 7 3}{0 . 1} = 1 2 0 0 0 0 \text{ eV }
```

This is more than enough kinetic energy to blow the atom apart. You can’t “see” inside.

A similar calculation can be made with the uncertainty principle.

```math
\Delta p \Delta x \geq \frac{\hbar}{2}
```

```math
\Delta ( pc ) \Delta x \geq \frac{\hbar c}{2}
```

```math
\Delta E_{\gamma} \geq \frac{\hbar c}{2 \Delta x}
```

```math
E_{\gamma} \geq \frac{\hbar c}{2 ( 0 . 1 ˚ A )} = 1 0 0 0 0 eV
```

The binding energy is 13 eV, so this will still blow it apart.

So we can’t “watch” the inside of an atom.

We can probe atoms with high energy photons (for example). These will blow the atoms apart, but we can use many atoms of the same kind. We learn about the internal structure of the atoms by scattering particles off them, blowing them apart.

## <a id="s-5-7-5-can-i-see-inside-a-nucleus"></a>5.7.5 Can I “See” inside a Nucleus

In a similar fashion to the previous section, $`E_{\gamma} \geq \tfrac{\hbar c}{2 ( 0 . 1 F )}`$ = 1000 MeV$`.`$

The binding energy per nucleon is a few MeV, so, we will also blow nuclei apart to look carefully inside them. We again can just use lots of nuclei to allow us to learn about internal nuclear structure.<span class="page" style="float:left;width:100%;margin:0.4em 0 0.6em"><span style="display:block;text-align:right"><a href="book.pdf#page=113" target="_blank" title="This page in the PDF"><sup>[p. 113]</sup></a></span><span style="display:block;height:2px;background:currentColor;opacity:0.25"></span><a id="p114" href="book.pdf#page=114" target="_blank" title="This page in the PDF"><sup>[p. 114]</sup></a></span> **5.7.6 Estimate the Hydrogen Ground State Energy**

The reason the Hydrogen atom (and other atoms) is so large is the essentially uncertainty principle. If the electron were confined to a smaller volume, ∆$`p`$ would increase, causing $`p`$ to increase on average. The energy would increase not decrease.

We can use the uncertainty principle to estimate the minimum energy for Hydrogen. This is not a perfect calculation but it is more correct than the Bohr model. The idea is that the radius must be larger than the spread in position, and the momentum must be larger than the spread in momentum.

```math
r \geq \Delta x
```

```math
p \geq \Delta p = \frac{\hbar}{2 r}
```

This is our formula for the potential energy in terms of the dimensionless fine structure constant $`\alpha`$.

```math
V ( r ) = - \frac{e^{2}}{r} = - \frac{\alpha \hbar c}{r}
```

## <a id="s-5-7-6-estimate-the-hydrogen-ground-state-energy"></a>Lets estimate the energy

```math
E = \frac{p^{2}}{2 m} - \frac{\alpha \hbar c}{r}
```

and put in the effect of the uncertainty principle.

```math
pr = \hbar
```

```math
E = \frac{p^{2}}{2 m} - \frac{\alpha \hbar cp}{\hbar}
```

```math
E = \frac{p^{2}}{2 m} - \alpha cp
```

Differentiate with respect to $`p`$ and set equal to zero to get the minimum.

```math
\frac{dE}{dp} = \frac{p}{m} - \alpha c = 0
```

```math
p = \alpha mc
```

```math
E = \frac{\alpha^{2} mc^{2}}{2} - \alpha^{2} mc^{2} = - \frac{\alpha^{2} mc^{2}}{2} = - 1 3 . 6 \text{ eV }
```

Note that the potential energy is just (-2) times the kinetic energy (as we expect from the Virial Theorem). The ground state energy formula is correct.

We can also estimate the radius.

```math
r = \frac{\hbar}{p} = \frac{\hbar}{\alpha mc} = \frac{\hbar c}{\alpha mc^{2}} = \frac{1 9 7 3 eV ˚ A ( 1 3 7 )}{5 1 1 0 0 0 eV} = 0 . 5 3 ˚ A
```

The ground state of Hydrogen has zero (orbital) angular momentum. It is not moving in a circular orbit as Bohr hypothesized. The electron just has a probability distribution that is spread out over about 1 ˚A. If it were not spread out, the energy would go up.

<div align="right"><a href="book.pdf#page=114" target="_blank" title="This page in the PDF"><sup>[p. 114]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p115" href="book.pdf#page=115" target="_blank" title="This page in the PDF"><sup>[p. 115]</sup></a>

# <a id="s-5-8-sample-test-problems"></a>5.8 Sample Test Problems

1. A nucleus has a radius of 4 Fermis. Use the uncertainty principle to estimate the kinetic energy for a neutron localized inside the nucleus. Do the same for an electron.

**Answer**

```math
\Delta p \Delta x \approx \hbar
```

```math
pr \approx \hbar
```

Try non-relativistic formula first and verify approximation when we have the energy.

```math
E = \frac{p^{2}}{2 m} = \frac{\hbar^{2}}{2 mr^{2}} = \frac{( \hbar c )^{2}}{2 mc^{2} r^{2}} = \frac{( 1 9 7 . 3 MeVF )^{2}}{2 ( 9 4 0 MeV ) ( 4 F )^{2}} \approx 1 . 3 MeV
```

This is much less than 940 MeV so the non-relativistic approximation is very good.<br>
The electron energy will be higher and its rest mass is only 0.51 MeV so it WILL be relativistic.

```math
\text{ This makes it easier } .
```

```math
pr \approx \hbar
```

```math
E = pc = \frac{\hbar c}{r} = \frac{1 9 7 . 3 MeVF}{4 F} \approx 5 0 MeV
```

2. **\*** Assume that the potential for a neutron near a heavy nucleus is given by $`V ( r )`$ = 0 for $`r> 5`$ Fermis and $`V ( r ) = - V_{0}`$ for $`r< 5`$ Fermis. Use the uncertainty principle to estimate the minimum value of $`V_{0}`$ needed for the neutron to be bound to the nucleus.

3. Use the uncertainty principle to estimate the ground state energy of Hydrogen.<br>
**Answer**

```math
\Delta p \Delta x \approx \hbar
```

```math
pr \approx \hbar
```

```math
E = \frac{p^{2}}{2 m} - \frac{e^{2}}{r} = \frac{p^{2}}{2 m} - \frac{e^{2}}{\hbar} p
```

(We could have replaced $`p`$ equally well.) Minimize.

```math
\frac{dE}{dp} = \frac{p}{m} - \frac{e^{2}}{\hbar} = 0
```

```math
p = \frac{me^{2}}{\hbar}
```

```math
E = \frac{p^{2}}{2 m} - \frac{e^{2}}{\hbar} p = \frac{m^{2} e^{4}}{2 m \hbar^{2}} - \frac{me^{4}}{\hbar^{2}} = \frac{me^{4}}{2 \hbar^{2}} - \frac{me^{4}}{\hbar^{2}} = - \frac{me^{4}}{2 \hbar^{2}}
```

```math
\alpha = \frac{e^{2}}{\hbar c}
```

```math
e^{2} = \alpha \hbar c
```

```math
E = - \frac{1}{2} \alpha^{2} mc^{2}
```

4. **\*** Given the following one dimensional probability amplitudes in the position variable x, compute the probability distribution in momentum space. Show that the uncertainty principle is roughly satisfied.

<div align="right"><a href="book.pdf#page=115" target="_blank" title="This page in the PDF"><sup>[p. 115]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p116" href="book.pdf#page=116" target="_blank" title="This page in the PDF"><sup>[p. 116]</sup></a>

a $`\psi ( x ) = \tfrac{1}{\sqrt{2 a}}`$ for $`- a<x<a`$, otherwise $`\psi ( x )`$ = 0.

```math
b \psi ( x ) = ( {}_{\pi}^{\alpha} ) 4^{1} e^{- \alpha x^{2} / 2}
```

```math
c \psi ( x ) = \delta ( x - x_{0} )
```

5. Use the Heisenberg uncertainty principle to estimate the ground state energy for a particle of

mass $`m`$ in the potential $`V ( x ) = \tfrac{1}{2} kx^{2}`$.

6. **\*** Find the one dimensional wave function in position space $`\psi ( x )`$ that corresponds to $`\varphi ( p ) =`$

```math
\delta ( p - p_{0} ) .
```

7. **\*** Find the one dimensional wave function in position space $`\psi ( x )`$ that corresponds to $`\varphi ( p ) =`$

$`\frac{1}{\sqrt{2 b}}`$ for $`- b<p<b`$, and $`\varphi ( p )`$ = 0 otherwise.

8. **\*** Assume that a particle is localized such that $`\psi ( x ) = \tfrac{1}{\sqrt{a}}`$ for $`0 <x<a`$ and that $`\psi ( x )`$ = 0

elsewhere. What is the probability for the particle to have a momentum between $`p`$ and $`p + dp`$?

9. A beam of photons of momentum $`p`$ is incident upon a slit of width $`a`$. The resulting diffraction pattern is viewed on screen which is a distance $`d`$ from the slit. Use the **uncertainty principle** to estimate the width of the central maximum of the diffraction pattern in terms of the variables given.

10. **\*** The wave-function of a particle in position space is given by $`\psi ( x ) = \delta ( x - a )`$. Find the

wave-function in **momentum space**. Is the state correctly normalized? Explain why.

11. **\*** A particle is in the state $`\psi ( x ) = Ae^{- \alpha x^{2} / 2}`$. What is the probability for the particle to have

a momentum between $`p`$ and $`p + dp`$?

12. A hydrogen atom has the potential $`V ( r ) = \tfrac{e^{2}}{r}^{-}`$. Use the uncertainty principle to estimate the

ground state energy.

13. **\*** Assume that $`\varphi ( p ) = \tfrac{1}{\sqrt{2 a}}`$ for $`| p | <a`$ and $`\varphi ( p )`$ = 0 elsewhere. What is $`\psi ( x )`$? What is the

probability to find the particle between $`x`$ and $`x + dx`$?

14. The hydrogen atom is made up of a proton and an electron bound together by the Coulomb potential, $`V ( r ) = \tfrac{- e^{2}}{r}`$. It is also possible to make a hydrogen-like atom from a proton and a muon. The force binding the muon to the proton is identical to that for the electron but the muon’s mass is 106 MeV/c<sup>2</sup>. Use the uncertainty principle to estimate the energy and the radius of the ground state of muonic hydrogen.

15. **\*** Given the following one dimensional probability amplitudes in the momentum representation, compute the probability amplitude in the position representation, $`\psi ( x )`$. Show that the uncertainty principle is satisfied.

(a) $`\bar{\psi} ( p ) = \sqrt{\tfrac{1}{2 a}}`$ for $`- a<p<a , \bar{\psi} ( p )`$ = 0 elsewhere.

```math
( b ) \bar{\psi} ( p ) = \delta ( p - p_{0} )
```

```math
( c ) \bar{\psi} ( p ) = ( {}_{\pi}^{\alpha} ) 4^{1} e^{- \alpha p^{2} / 2}
```

16. **\*** Assume that $`\varphi ( p ) = \delta ( p - p_{0} )`$. What is $`\psi ( x )`$? What is $`<p^{2} >`$? What is $`<x^{2} >`$?

17.

---

[← 4 The Solution: Probability Amplitudes](04-the-solution-probability-amplitudes.md) · [6 Operators →](06-operators.md)
