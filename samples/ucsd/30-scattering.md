[← 29 Radiation in Atoms](29-radiation-in-atoms.md) · [31 Classical Scalar Fields →](31-classical-scalar-fields.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p419" href="book.pdf#page=419" target="_blank" title="This page in the PDF"><sup>[p. 419]</sup></a>

# 30 Scattering

This material is covered in **Gasiorowicz Chapter 23**.

Scattering of one object from another is perhaps our best way of observing and learning about the microscopic world. Indeed it is the scattering of light from objects and the subsequent detection of the scattered light with our eyes that gives us the best information about the macroscopic world. We can learn the shapes of objects as well as some color properties simply by observing scattered light.

There is a limit to what we can learn with visible light. In Quantum Mechanics we know that we cannot discern details of microscopic systems (like atoms) that are smaller than the wavelength of the particle we are scattering. Since the **minimum wavelength of visible light is about** **0.25 microns**, we cannot see atoms or anything smaller even with the use of optical microscopes. The physics of atoms, nuclei, subatomic particles, and the fundamental particles and interactions in nature must be studied by scattering particles of higher energy than the photons of visible light.

Scattering is also something that we are familiar with from our every day experience. For example, billiard balls scatter from each other in a predictable way. We can fairly easily calculate how billiard balls would scatter if the collisions were elastic but with some energy loss and the possibility of transfer of energy to spin, the calculation becomes more difficult.

Let us take the macroscopic example of **BBs scattering from billiard balls** as an example to study. We will motivate some of the terminology used in scattering macroscopically. Assume we fire a BB at a billiard ball. If we miss the BB does not scatter. If we hit, the BB bounces off the ball and goes off in a direction different from the original direction. Assume our aim is bad and that the BB has a uniform probability distribution over the area around the billiard ball. The area of the projection of the billiard ball into two dimensions is just $`\pi R^{2}`$ if $`R`$ is the radius of the billiard ball. Assume the BB is much smaller so that its radius can be neglected for now.

We can then say something about the probability for a scattering to occur if we know the area of the projection of the billiard ball and number of BBs per unit area that we shot.

```math
N_{scat} = \frac{N}{A} \pi R^{2}
```

Where $`N`$ is the number of BBs we shot, $`A`$ is the area over which they are spread, and $`R`$ is the radius of the billiard ball.

In normal scattering experiments, we have a beam of particles and we know the number of particles per second. We measure the number of scatters per second so we just divide the above equation by the time period $`T`$ to get rates.

```math
Rate_{scat} = \frac{N}{AT} \pi R^{2} = ( Incident \quad Flux ) ( cross \quad section )
```

The **incident flux** is the number of particles per unit area per unit time in the beam. This is a well defined quantity in quantum mechanics, $`| \vec{j} |`$. The **cross section** $`\sigma`$ is the projected area of the billiard ball in this case. It may be more complicated in other cases. For example, if we do not neglect the radius $`r`$ of the BB, the cross section for scattering is

```math
\sigma = \pi ( R + r )^{2} .
```

<div align="right"><a href="book.pdf#page=419" target="_blank" title="This page in the PDF"><sup>[p. 419]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p420" href="book.pdf#page=420" target="_blank" title="This page in the PDF"><sup>[p. 420]</sup></a>

Clearly there is more information available from scattering than whether a particle scatters or not. For example, **Rutherford discovered that atomic nucleus** by seeing that high energy alpha particles sometimes backscatter from a foil containing atoms. The atomic model of the time did not allow this since the positive charge was spread over a large volume. We measure the probability to scatter into different directions. This will also happen in the case of the BB and the billiard ball. The polar angle of scattering will depend on the “impact parameter” of the incoming BB. We can measure the scattering into some small solid angle $`d`$Ω. The part of the cross section $`\sigma`$ that scatters into that solid angle can be called the **differential cross section** $`\frac{d\sigma}{d \Omega}`$. The integral over solid angle will give us back the total cross section.

```math
\int \frac{d\sigma}{d \Omega} d \Omega = \sigma
```

The idea of cross sections and incident fluxes translates well to the quantum mechanics we are using. If the **incoming beam is a plane wave**, that is a beam of particles of definite momentum or wave number, we can describe it simply in terms of the number or particles per unit area per second, the incident flux. The **scattered particle is also a plane wave** going in the direction defined by $`d`$Ω. What is left is the interaction between the target particle and the beam particle which causes the transition from the initial plane wave state to the final plane wave state.

We have already studied one approximation method for scattering called a partial wave analysis (See section 15.6) . It is good for scattering potentials of limited range and for low energy scattering. It divides the incoming plane wave in to partial waves with definite angular momentum. The high angular momentum components of the wave will not scatter (much) because they are at large distance from the scattering potential where that potential is very small. We may then deal with just the first few terms (or even just the $`\ell`$ = 0 term) in the expansion. We showed that the incoming partial wave and the outgoing wave can differ only by a phase shift for elastic scattering. If we calculate this phase shift $`\delta_{\ell}`$, we can then determine the differential scattering cross section.

Let’s review some of the equations. A plane wave can be decomposed into a sum of spherical waves with definite angular momenta which goes to a simple sum of incoming and outgoing spherical waves at large $`r`$.

```math
e^{ikz} = e^{ikr \cos \theta} = \sum_{\ell = 0}^{\infty} \sqrt{4 \pi ( 2 \ell + 1 )} i^{\ell} j_{\ell} ( kr ) Y_{\ell 0} \to - \sum_{\ell = 0}^{\infty} \sqrt{4 \pi ( 2 \ell + 1 )} i^{\ell} \frac{1}{2 ikr} \left( e^{- i ( kr - \ell \pi / 2 )} - e^{i ( kr - \ell \pi / 2 )} \right) Y_{\ell 0}
```

A potential causing elastic scattering will modify the phases of the outgoing spherical waves.

```math
\lim_{r \to \infty} \psi = - \sum_{\ell = 0}^{\infty} \sqrt{4 \pi ( 2 \ell + 1 )} i^{\ell} \frac{1}{2 ikr} \left( e^{- i ( kr - \ell \pi / 2 )} - e^{2 i\delta_{\ell} ( k )} e^{i ( kr - \ell \pi / 2 )} \right) Y_{\ell 0}
```

We can compute the differential cross section for elastic scattering.

```math
\frac{d\sigma}{d \Omega} = \frac{1}{k^{2}} \left| \sum_{\ell} ( 2 \ell + 1 ) e^{i\delta_{\ell} ( k )} \sin ( \delta_{\ell} ( k ) ) P_{\ell} ( \cos \theta ) \right|^{2}
```

It is useful to write this in terms of the amplitudes of the scattered waves.

```math
\frac{d\sigma}{d \Omega} = | f ( \theta ,\varphi ) |^{2}
```

```math
f ( \theta ,\varphi ) = \frac{1}{k} \sum_{\ell} ( 2 \ell + 1 ) e^{i\delta_{\ell} ( k )} \sin ( \delta_{\ell} ( k ) ) P_{\ell} ( \cos \theta ) = \sum_{\ell} f_{\ell} ( \theta ,\varphi )
```

<div align="right"><a href="book.pdf#page=420" target="_blank" title="This page in the PDF"><sup>[p. 420]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p421" href="book.pdf#page=421" target="_blank" title="This page in the PDF"><sup>[p. 421]</sup></a>

As an example, this has been used to compute the cross section for scattering from a spherical potential well (See section 15.7) assuming only the $`\ell`$ = 0 phase shift was significant. By matching the boundary conditions at the boundary of the spherical well, we determined the phase shift.

```math
\tan \delta_{0} = - \frac{C}{B} = \frac{k \cos ( ka ) \sin ( k^{\prime} a ) - k^{\prime} \cos ( k^{\prime} a ) \sin ( ka )}{k \sin ( ka ) \sin ( k^{\prime} a ) + k^{\prime} \cos ( k^{\prime} a ) \cos ( ka )}
```

The differential cross section is

```math
\frac{d\sigma}{d \Omega} \to \frac{\sin^{2} ( \delta_{0} )}{k^{2}}
```

which will have zeros if

```math
k^{\prime} \cot ( k^{\prime} a ) = k \cot ( ka ) .
```

We can compute the total scattering cross section using the relation $`\int d`$Ω$`P_{\ell}`$(cos $`\theta ) P_{\ell}^{\prime}`$(cos$`\theta ) =`$

```math
\sigma \tfrac{4 \pi}{2 \ell + 1} \delta_{\ell \ell^{\prime}} ._{tot} = \int d \Omega | f ( \theta ,\varphi ) |^{2}
```

```math
= \int d \Omega \left[ \frac{1}{k} \sum_{\ell} ( 2 \ell + 1 ) e^{i\delta_{\ell} ( k )} \sin ( \delta_{\ell} ( k ) ) P_{\ell} ( \cos \theta ) \right] \left[ \frac{1}{k} \sum_{\ell}^{\prime} ( 2 \ell^{\prime} + 1 ) e^{- i\delta_{\ell}^{\prime} ( k )} \sin ( \delta_{\ell}^{\prime} ( k ) ) P_{\ell}^{\prime} ( \cos \theta ) \right]
```

```math
= \frac{4 \pi}{k^{2}} \sum_{\ell} ( 2 \ell + 1 ) \sin ( \delta_{\ell} ( k ) )^{2}
```

It is interesting that we can relate the total cross section to the scattering amplitude at $`\theta`$ = 0, for which $`P_{\ell}`$(1) = 1.

```math
f ( \theta = 0 ,\varphi ) = \frac{1}{k} \sum_{\ell} ( 2 \ell + 1 ) e^{i\delta_{\ell} ( k )} \sin ( \delta_{\ell} ( k ) )
```

```math
Im [ f ( \theta = 0 ,\varphi ) ] = \frac{1}{k} \sum_{\ell} ( 2 \ell + 1 ) \sin^{2} ( \delta_{\ell} ( k ) )
```

```math
\sigma_{tot} = \frac{4 \pi}{k} Im [ f ( \theta = 0 ,\varphi ) ]
```

The total cross section is related to the imaginary part of the forward elastic scattering amplitude. This seemingly strange relation is known as the **Optical Theorem**. It can be understood in terms of removal of flux from the incoming plane wave. Remember we have an incoming plane wave plus scattered spherical waves. The total cross section corresponds to removal of flux from the plane wave. The only way to do this is destructive interference with the scattered waves. Since the plane wave is at $`\theta`$ = 0 it is only the scattered amplitude at $`\theta`$ = 0 that can interfere. It is therefore reasonable that a relation like the Optical Theorem is correct, even when elastic and inelastic processes are possible.

We have not treated inelastic scattering. **Inelastic scattering** can be a complex and interesting process. It was with high energy inelastic scattering of electrons from protons that the quark structure of the proton was “seen”. In fact, the electrons appeared to be scattering from essentially free quarks inside the proton. The proton was broken up into sometimes many particles in the process but the data could be simply analyzed using the scatter electron. In a phase shift analysis, inelastic scattering removes flux from the outgoing spherical waves.

```math
\lim_{r \to \infty} \psi = - \sum_{\ell = 0}^{\infty} \sqrt{4 \pi ( 2 \ell + 1 )} i^{\ell} \frac{1}{2 ikr} \left( e^{- i ( kr - \ell \pi / 2 )} - \eta_{\ell} ( k ) e^{2 i\delta_{\ell} ( k )} e^{i ( kr - \ell \pi / 2 )} \right) Y_{\ell 0}
```

<div align="right"><a href="book.pdf#page=421" target="_blank" title="This page in the PDF"><sup>[p. 421]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p422" href="book.pdf#page=422" target="_blank" title="This page in the PDF"><sup>[p. 422]</sup></a>

Here $`0 <\eta_{\ell} < 1`$, with 0 represent complete absorption of the partial wave and 1 representing purely elastic scattering. An interesting example of the effect of absorption (or inelastic production of another state) is the **black disk**. The disk has a definite radius $`a`$ and absorbs partial waves for $`\ell <ka`$. If one works out this problem, one finds that there is an inelastic scattering cross section of $`\sigma_{inel} = \pi a^{2}`$. Somewhat surprisingly the total elastic scattering cross section is $`\sigma_{elas} = \pi a^{2}`$. The disk absorbs part of the beam and there is also diffraction around the sharp edges. That is, the removal of the outgoing spherical partial waves modifies the plane wave to include scattered waves.

For high energies relative to the inverse range of the potential, a partial wave analysis is not helpful and it is far better to use perturbation theory. The **Born approximation** is valid for high energy and weak potentials. If the potential is weak, only one or two terms in the perturbation series need be calculated.

If we work in the usual center of mass system, we have a problem with one particle scattering in a potential. The incoming plane wave can be written as

```math
\psi_{i} ( \vec{r} ) = \frac{1}{\sqrt{V}} e^{i \vec{k}_{i} \cdot \vec{x}} .
```

The scattered plane wave is

```math
\psi_{f} ( \vec{r} ) = \frac{1}{\sqrt{V}} e^{i \vec{k}_{f} \cdot \vec{x}} .
```

We can use Fermi’s golden rule to calculate the transition rate to first order in perturbation theory.

```math
R_{i \to f} = \frac{2 \pi}{\hbar} \int \frac{Vd^{3} \vec{k}_{f}}{( 2 \pi )^{3}} | \langle \psi_{f} | V ( \vec{r} ) | \psi_{i} \rangle |^{2} \delta ( E_{f} - E_{i} )
```

The delta function expresses energy conservation for elastic scattering which we are assuming at this point. If inelastic scattering is to be calculated, the energy of the atomic state changes and that change should be included in the delta function and the change in the atomic state should be included in the matrix element.

The elastic scattering matrix element is

```math
\langle \psi_{f} | V ( \vec{r} ) | \psi_{i} \rangle = \frac{1}{V} \int d^{3} \vec{r} e^{- i \vec{k}_{f} \cdot \vec{x}} V ( \vec{r} ) e^{i \vec{k}_{i} \cdot \vec{x}} = \frac{1}{V} \int d^{3} \vec{r} e^{- i \vec{\Delta} \cdot \vec{x}} V ( \vec{r} ) = \frac{1}{V} \tilde{V} ( \vec{\Delta} )
```

where ∆ =$`\vec{\jmath} \quad \vec{k}_{f} - \vec{k}_{i}`$. We notice that this is just proportional to the Fourier Transform of the potential.

Assuming for now non-relativistic final state particles we calculate

```math
R_{i \to f} = \frac{2 \pi}{\hbar} \int \frac{Vd \Omega_{f} k_{f}^{2} dk_{f}}{( 2 \pi )^{3}} \frac{1}{V^{2}} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2} \delta \left( \frac{\hbar^{2} k_{f}^{2}}{2 \mu} - E_{i} \right)
```

```math
= \frac{2 \pi}{\hbar} \frac{1}{( 2 \pi )^{3} V} \int d \Omega_{f} k_{f}^{2} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2} \frac{\mu}{\hbar^{2} k_{f}}
```

```math
= \frac{1}{4 \pi^{2} \hbar^{3} V} \int d \Omega_{f} \mu k_{f} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2}
```

We now need to convert this transition rate to a cross section. Our wave functions are normalize to one particle per unit volume and we should modify that so that there is a flux of one particle

<div align="right"><a href="book.pdf#page=422" target="_blank" title="This page in the PDF"><sup>[p. 422]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p423" href="book.pdf#page=423" target="_blank" title="This page in the PDF"><sup>[p. 423]</sup></a>

per square centimeter per second to get a cross section. To do this we set the volume to be $`V`$ = (1 $`cm^{2} ) ( v_{rel} ) ( 1 \quad second )`$. The relative velocity is just the momentum divided by the reduced mass.

```math
\sigma = \frac{1}{4 \pi^{2} \hbar^{3} v_{rel}} \int d \Omega_{f} \mu k_{f} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2}
```

```math
= \frac{1}{4 \pi^{2} \hbar^{4}} \int d \Omega_{f} \mu^{2} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2}
```

```math
\frac{d\sigma}{d \Omega} = \frac{\mu^{2}}{4 \pi^{2} \hbar^{4}} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2}
```

This is a very useful formula for scattering from a weak potential or for scattering at high energy for problems in which the cross section gets small because the Fourier Transform of the potential diminishes for large values of $`k`$. It is not good for scattering due to the strong interaction since cross sections are large and do not typically decrease at high energy. Note that the matrix elements and hence the scattering amplitudes calculated in the Born approximation are real and therefore do not satisfy the Optical Theorem. This is a shortcoming of the approximation.

# <a id="s-30-1-scattering-from-a-screened-coulomb-potential"></a>30.1 Scattering from a Screened Coulomb Potential

A standard Born approximation example is Rutherford Scattering, that is, Coulomb scattering of a particle of charge $`Z_{1} e`$ in a screened Coulomb potential $`\varphi ( r ) = \tfrac{Z_{2} e}{r} e^{- r/a}`$. The exponential represents the screening of the nuclear charge by atomic electrons. Without screening, the total Coulomb scattering cross section is infinite because the range of the force in infinite.

The potential energy then is

```math
V ( r ) = \frac{Z_{1} Z_{2} e^{2}}{r} e^{- r/a}
```

We need to calculate its Fourier Transform.

```math
\tilde{V} ( \vec{\Delta} ) = Z_{1} Z_{2} e^{2} \int d^{3} re^{- i \vec{\Delta} \cdot \vec{r}} \frac{e^{- r/a}}{r}
```

Since the potential has spherical symmetry, we can choose ∆ to be in the $`z`$ direction and proceed with the integral.

```math
\tilde{V} ( \vec{\Delta} ) = Z_{1} Z_{2} e^{2} 2 \pi \int_{0}^{\infty} r^{2} dr \int_{- 1}^{1} d ( \cos \theta ) e^{- i \Delta r \cos \theta} \frac{e^{- r/a}}{r}
```

```math
= Z_{1} Z_{2} e^{2} 2 \pi \int_{0}^{\infty} r^{2} dr \left[ \frac{e^{- i \Delta rx}}{- i \Delta r} \right] \frac{x = 1}{x = - 1} \frac{e^{- r/a}}{r}
```

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{i \Delta} \int_{0}^{\infty} dr \left[ e^{- i \Delta r} - e^{i \Delta r} \right] e^{- r/a}
```

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{- i \Delta} \int_{0}^{\infty} dr \left[ e^{- ( \tfrac{1}{a} + i \Delta ) r} - e^{- ( \tfrac{1}{a} - i \Delta ) r} \right]
```

<div align="right"><a href="book.pdf#page=423" target="_blank" title="This page in the PDF"><sup>[p. 423]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p424" href="book.pdf#page=424" target="_blank" title="This page in the PDF"><sup>[p. 424]</sup></a>

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{- i \Delta} \left[ - \frac{e^{- ( \tfrac{1}{a} + i \Delta ) r}}{\tfrac{1}{a} + i \Delta} + \frac{e^{- ( \tfrac{1}{a} - i \Delta ) r}}{\tfrac{1}{a} - i \Delta} \right] \frac{\infty}{0}
```

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{- i \Delta} \left[ \frac{1}{\tfrac{1}{a} + i \Delta} - \frac{1}{\tfrac{1}{a} - i \Delta} \right]
```

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{- i \Delta} \left[ \frac{\tfrac{1}{a} - i \Delta - \tfrac{1}{a} - i \Delta}{\tfrac{1}{a^{2}} + \Delta^{2}} \right]
```

```math
= Z_{1} Z_{2} e^{2} \frac{2 \pi}{- i \Delta} \left[ \frac{- 2 i \Delta}{\tfrac{1}{a^{2}} + \Delta^{2}} \right]
```

```math
= \frac{4 \pi Z_{1} Z_{2} e^{2}}{\tfrac{1}{a^{2}} + \Delta^{2}}
```

Since ∆ =$`\vec{\jmath} \quad \vec{k}_{f} - \vec{k}_{i}`$, we have ∆2 $`= k_{f}^{2} + k_{i}^{2} - 2 k_{f} k_{i} \cos \theta`$. For elastic scattering, ∆2 = 2$`k^{2} ( 1 - \cos \theta )`$. The differential cross section is

```math
\frac{d\sigma}{d \Omega} = \frac{\mu^{2}}{4 \pi^{2} \hbar^{4}} \left| \tilde{V} ( \vec{\Delta} ) \right|^{2}
```

```math
= \frac{\mu^{2}}{4 \pi^{2} \hbar^{4}} \left| \frac{4 \pi Z_{1} Z_{2} e^{2}}{\tfrac{1}{a^{2}} + 2 k^{2} ( 1 - \cos \theta )} \right|^{2}
```

```math
= \left| \frac{\mu Z_{1} Z_{2} e^{2}}{\tfrac{\hbar^{2}}{2 a^{2}} + p^{2} ( 1 - \cos \theta )} \right|^{2}
```

```math
= \left| \frac{Z_{1} Z_{2} e^{2}}{\tfrac{\hbar^{2}}{2 \mu a^{2}} + 4 E \sin^{2} \tfrac{\theta}{2}} \right|^{2}
```

In the last step we have used the non-relativistic formula for energy and 1 $`- \cos \theta = \tfrac{1}{2}`$ sin<sup>2</sup> $`\frac{\theta}{2}`$.

The screened Coulomb potential gives a finite total cross section. It corresponds well with the experiment Rutherford did in which $`\alpha`$ particles were scattered from atoms in a foil. If we scatter from a bare charge where there is no screening, we can take the limit in which $`a \to \infty`$.

```math
\left| \frac{Z_{1} Z_{2} e^{2}}{4 E \sin^{2} \tfrac{\theta}{2}} \right|^{2}
```

The total cross section diverges in due to the region around zero scattering angle.

# <a id="s-30-2-scattering-from-a-hard-sphere"></a>30.2 Scattering from a Hard Sphere

Assume a low energy beam is incident upon a small, hard sphere of radius $`r_{0}`$. We will assume that $`\hbar kr_{0} < \hbar`$ so that only the $`\ell`$ = 0 partial wave is significantly affected by the sphere. As with the particle in a box, the boundary condition on a hard surface is that the wavefunction is zero. Outside the sphere, the potential is zero and the wavefunction solution will have reached its form for large $`r`$. So we set

```math
\left( e^{- i ( kr_{0} - \ell \pi / 2 )} - e^{2 i\delta_{\ell} ( k )} e^{i ( kr_{0} - \ell \pi / 2 )} \right) = \left( e^{- ikr_{0}} - e^{2 i\delta_{0} ( k )} e^{ikr_{0}} \right) = 0
```

```math
e^{2 i\delta_{0} ( k )} = e^{- 2 ikr_{0}}
```

<div align="right"><a href="book.pdf#page=424" target="_blank" title="This page in the PDF"><sup>[p. 424]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p425" href="book.pdf#page=425" target="_blank" title="This page in the PDF"><sup>[p. 425]</sup></a>

```math
\delta_{0} ( k ) = - kr_{0}
```

```math
\frac{d\sigma}{d \Omega} = \frac{1}{k^{2}} \left| e^{i\delta_{\ell} ( k )} \sin ( \delta_{\ell} ( k ) ) P_{0} ( \cos \theta ) \right|^{2}
```

```math
\frac{d\sigma}{d \Omega} = \frac{1}{k^{2}} \left| e^{- ikr_{0}} \sin ( kr_{0} ) \right|^{2}
```

```math
\frac{d\sigma}{d \Omega} = \frac{\sin^{2} ( kr_{0} )}{k^{2}}
```

For very low energy, $`kr_{0} << 1`$ and

```math
\frac{d\sigma}{d \Omega} \approx \frac{( kr_{0} )^{2}}{k^{2}} = r_{0}^{2}
```

The total cross section is then $`\sigma`$ = 4$`\pi r_{0}^{2}`$ which is 4 times the area of the hard sphere.

# <a id="s-30-3-homework-problems"></a>30.3 Homework Problems

1. Photons from the 3$`p \to 1 s`$ transition are observed coming from the sun. Quantitatively compare the natural line width to the widths from Doppler broadening and collision broadening expected for radiation from the sun’s surface.

# <a id="s-30-4-sample-test-problems"></a>30.4 Sample Test Problems

1. Calculate the differential cross section, $`\frac{d\sigma}{d \Omega}`$, for high energy scattering of particles of momentum p, from a spherical shell delta function

```math
V ( \boldsymbol{r} ) = \lambda \delta ( r - r_{0} )
```

Assume that the potential is weak so that perturbation theory can be used. Be sure to write your answer in terms of the scattering angles.

2. Assume that a heavy nucleus attracts $`K_{0}`$ mesons with a weak Yakawa potential $`V ( r ) = \tfrac{V_{0}}{r} e^{- \alpha r}`$. Calculate the differential cross section, $`\frac{d\sigma}{d \Omega}`$, for scattering high energy $`K_{0}`$ mesons (mass $`m_{K} )`$ from that nucleus. Give your answer in terms of the scattering angle $`\theta`$.

---

[← 29 Radiation in Atoms](29-radiation-in-atoms.md) · [31 Classical Scalar Fields →](31-classical-scalar-fields.md)
