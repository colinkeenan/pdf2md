[← 34 Scattering of Photons](34-scattering-of-photons.md) · [36 Dirac Equation →](36-dirac-equation.md)

---

# 35 Electron Self Energy Corrections

If one calculates the **energy of a point charge using classical electromagnetism**, the result is infinite, yet as far as we know, the electron is point charge. One can calculate the energy needed to assemble an electron due, essentially, to the interaction of the electron with its own field. A uniform charge distribution with the **classical radius of an electron**, would have an energy of the order of $`m_{e} c^{2}`$. Experiments have probed the electron’s charge distribution and found that it is consistent with a point charge down to distances much smaller than the classical radius. Beyond classical calculations, the **self energy of the electron calculated in the quantum theory of** **Dirac is still infinite** but the divergences are less severe.

At this point we must take the unpleasant position that this (constant) **infinite energy should** **just be subtracted** when we consider the overall zero of energy (as we did for the field energy in the vacuum). Electrons exist and don’t carry infinite amount of energy baggage so we just subtract off the infinite constant. Nevertheless, we will find that the **electron’s self energy may** **change when it is a bound state** and that we should account for this change in our energy level calculations. This calculation will also give us the **opportunity to understand resonant** **behavior in scattering**.

We can **calculate the lowest order self energy corrections represented by the two Feyn-** **man diagrams** below.

In these, a photon is emitted then reabsorbed. As we now know, both of these amplitudes are of order $`e^{2}`$. The first one comes from the $`A^{2}`$ term in which the number of photons changes by zero or two and the second comes from the $`\vec{A} \cdot \vec{p}`$ term in second order time dependent perturbation theory. A calculation of the first diagram will give the same result for a free electron and a bound electron, while the second diagram will give different results because the intermediate states are different if

<div align="right"><a href="book.pdf#page=472" target="_blank" title="This page in the PDF"><sup>[p. 472]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p473" href="book.pdf#page=473" target="_blank" title="This page in the PDF"><sup>[p. 473]</sup></a>

an electron is bound than they are if it is free. We will therefore **compute the amplitude from** **the second diagram**.

```math
H_{int} = - \frac{e}{mc} \vec{A} \cdot \vec{p}
```

```math
\vec{A} = \frac{1}{\sqrt{V}} \sum_{\vec{k} ,\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \hat{\epsilon}^{( \alpha )} \left( a_{\vec{k} ,\alpha} e^{i ( \vec{k} \cdot \vec{x} - \omega t )} + a_{\vec{k} ,\alpha}^{\dagger} e^{- i ( \vec{k} \cdot \vec{x} - \omega t )} \right)
```

This contains a term causing absorption of a photon and another term causing emission. We separate the terms for absorption and emission and pull out the time dependence.

```math
H_{int} = \sum_{\vec{k} ,\alpha} \left( H_{\vec{k} ,\alpha}^{abs} e^{- i\omega t} + H_{\vec{k} ,\alpha}^{emit} e^{i\omega t} \right)
```

```math
H^{abs} = - \sqrt{2 m \hbar e^{2 2} \omega V} a_{\vec{k} ,\alpha} e^{i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )}
```

```math
H^{emit} = - \sqrt{2 m \hbar e^{2 2} \omega V} a_{\vec{k} ,\alpha}^{\dagger} e^{- i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )}
```

The initial and final state is the same $`| n \rangle`$, and second order perturbation theory will involve a **sum** **over intermediate atomic states,** $`| j \rangle`$ **and photon states**. We will use the matrix elements of the interaction Hamiltonian between those states.

```math
H_{jn} = \langle j | H_{\vec{k} ,\alpha}^{emit} | n \rangle
```

```math
H_{nj} = \langle n | H_{\vec{k} ,\alpha}^{abs} | j \rangle
```

```math
H_{nj} = H_{jn}^{*}
```

We have dropped the subscript on $`H_{jn}`$ specifying the photon emitted or absorbed leaving a reminder in the sum. Recall from earlier calculations that the creation and annihilation operators just give a factor of 1 when a photon is emitted or absorbed.

From time dependent perturbation theory, the rate of change of the amplitude to be in a state is given by

```math
i \hbar \frac{\partial c_{j} ( t )}{\partial t} = \sum_{k} H_{jk} ( t ) c_{k} ( t ) e^{i\omega_{jk} t}
```

In this case, we want to use the equations for the the state we are studying, $`\psi_{n}`$, and all intermediate states, $`\psi_{j}`$ plus a photon. Transitions can be made by emitting a photon from $`\psi_{n}`$ to an intermdiate state and transitions can be made back to the state $`\psi_{n}`$ from any intermediate state. We neglect transitions from one intermediate state to another as they are higher order. (The diagram is emit a photon from $`\psi_{n}`$ then reabsorb it.)

The **differential equations for the amplitudes** are then.

```math
i \hbar \frac{dc_{j}}{dt} = \sum_{\vec{k} ,\alpha} H_{jn} e^{i\omega t} c_{n} e^{- i\omega_{nj} t}
```

<div align="right"><a href="book.pdf#page=473" target="_blank" title="This page in the PDF"><sup>[p. 473]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p474" href="book.pdf#page=474" target="_blank" title="This page in the PDF"><sup>[p. 474]</sup></a>

```math
i \hbar \frac{dc_{n}}{dt} = \sum_{\vec{k} ,\alpha} \sum_{j} H_{nj} e^{- i\omega t} c_{j} e^{i\omega_{nj} t}
```

In the equations for $`c_{n}`$, we **explicitly account for the fact that an intermediate state can** **make a transition back to the initial state**. Transitions through another intermediate state would be higher order and thus should be neglected. Note that the matrix elements for the transitions to and from the initial state are closely related. We also **include the effect that the initial state** **can become depleted** as intermediate states are populated by using $`c_{n}`$ (instead of 1) in the equation for $`c_{j}`$. Note also that all the photon states will make nonzero contributions to the sum.

Our task is to **solve these coupled equations**. Previously, we did this by integration, but needed the assumption that the amplitude to be in the initial state was 1.

Since we are attempting to **calculate an energy shift, let us make that assumption** and plug it into the equations to verify the solution.

```math
c_{n} = e^{\tfrac{- i \Delta Ent}{\hbar}}
```

∆$`E_{n}`$ will be a complex number, the **real part of which represents an energy shift**, and the **imaginary part of which represents the lifetime (and energy width)** of the state.

```math
i \hbar \frac{dc_{j}}{dt} = \sum_{\vec{k} ,\alpha} H_{jn} e^{i\omega t} c_{n} e^{- i\omega_{nj} t}
```

```math
c_{n} = e_{t}^{\tfrac{- i \Delta Ent}{\hbar}}
```

```math
c_{j} ( t ) = \frac{1}{i \hbar} \sum \int_{\vec{k} ,\alpha 0} dt^{\prime} H_{jn} e^{i\omega t^{\prime}} e^{\tfrac{- i \Delta Ent \prime}{\hbar}} e^{- i\omega_{nj} t^{\prime}}
```

```math
c_{j} ( t ) = \frac{1}{i \hbar} \sum \int_{\vec{k} ,\alpha 0}^{t} dt^{\prime} H_{jn} e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t^{\prime}}
```

```math
c_{j} ( t ) = \sum_{\vec{k} ,\alpha} H_{jn} \left[ \frac{e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t^{\prime}}}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )} \right] \frac{t}{0}
```

```math
c_{j} ( t ) = \sum_{\vec{k} ,\alpha} H_{jn} \frac{e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t} - 1}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )}
```

**Substitute** this back into the differential equation for $`c_{n}`$ to verify the solution and **to find out** **what** ∆$`E_{n}`$ **is**. Note that the double sum over photons reduces to a single sum because we must absorb the same type of photon that was emitted. (We have not explicitly carried along the photon state for economy.)

```math
i \hbar \frac{dc_{n}}{dt} = \sum_{\vec{k} ,\alpha} \sum_{j} H_{nj} e^{- i\omega t} c_{j} e^{i\omega_{nj} t}
```

```math
c_{j} ( t ) = \sum_{\vec{k} ,\alpha} H_{jn} \frac{e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t} - 1}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )}
```

<div align="right"><a href="book.pdf#page=474" target="_blank" title="This page in the PDF"><sup>[p. 474]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p475" href="book.pdf#page=475" target="_blank" title="This page in the PDF"><sup>[p. 475]</sup></a>

```math
i \hbar \frac{dc_{n}}{dt} = \Delta E_{n} e^{- i \Delta E_{n} t/ \hbar} = \sum_{\vec{k} ,\alpha} \sum_{j} H_{nj} H_{jn} e^{- i\omega t} e^{i\omega_{nj} t} \frac{e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t} - 1}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )}
```

```math
\Delta E_{n} = \sum_{\vec{k} ,\alpha} \sum_{j} | H_{nj} |^{2} e^{i ( \omega_{nj} + \Delta \omega_{n} - \omega ) t} \frac{e^{i ( - \omega_{nj} - \Delta \omega_{n} + \omega ) t} - 1}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )}
```

```math
\Delta E_{n} = \sum_{\vec{k} ,\alpha} \sum_{j} | H_{nj} |^{2} \frac{1 - e^{i ( \omega_{nj} + \Delta \omega_{n} - \omega ) t}}{\hbar ( \omega_{nj} + \Delta \omega_{n} - \omega )}
```

Since this a **calculation to order** $`e^{2}`$ and the interaction Hamiltonian squared contains a factor of $`e^{2}`$ we should drop the ∆$`\omega_{n}`$ = ∆$`E_{n} / \hbar`$s from the right hand side of this equation.

```math
\Delta E_{n} = \sum_{\vec{k} ,\alpha} \sum_{j} | H_{nj} |^{2} \frac{1 - e^{i ( \omega_{nj} - \omega ) t}}{\hbar ( \omega_{nj} - \omega )}
```

**We have a solution to the coupled differential equations** to order $`e^{2}`$. We should let $`t \to \infty`$ since the self energy is not a time dependent thing, however, the result oscillates as a function of time. This has been the case for many of our important delta functions, like the dot product of states with definite momentum. Let us **analyze this self energy expression for large time**.

We have something of the form

```math
- i \int_{0}^{t} e^{ixt^{\prime}} dt^{\prime} = \frac{1 - e^{ixt}}{x}
```

If we think of $`x`$ as a complex number, our integral goes along the real axis. In the upper half plane, just above the real axis, $`x \to x + i\epsilon`$, the function goes to zero at infinity. In the lower half plane it blows up at infinity and on the axis, its not well defined. We will calculate our result in the upper half plane and take the limit as we approach the real axis.

```math
\lim_{t \to \infty} \frac{1 - e^{ixt}}{x} = - \lim_{\epsilon \to 0 +} i \int_{0}^{\infty} e^{ixt^{\prime}} dt^{\prime} = \lim_{\epsilon \to 0 +} \frac{1}{x + i\epsilon} = \lim_{\epsilon \to 0 +} \left[ \frac{x}{x^{2} + \epsilon^{2}} - \frac{i\epsilon}{x^{2} + \epsilon^{2}} \right]
```

This is well behaved everywhere except at $`x`$ = 0. The second term goes to $`-\infty`$ there. A little further analysis could show that the **second term is a delta function**.

```math
\lim_{t \to \infty} \frac{1 - e^{ixt}}{x} = \frac{1}{x} - i\pi \delta ( x )
```

Recalling that $`c_{n} e^{- iE_{n} t/ \hbar} = e^{\tfrac{- i \Delta Ent}{\hbar}} e^{- iE_{n} t/ \hbar} = e^{- i ( E_{n} + \Delta E_{n} ) t/ \hbar}`$, the real part of ∆$`E_{n}`$ corresponds to an **energy shift in the state** $`| n \rangle`$ and the **imaginary part corresponds to a width**.

```math
ℜ ( \Delta E_{n} ) = \sum_{\vec{k} ,\alpha} \sum_{j} \frac{| H_{nj} |^{2}}{\hbar ( \omega_{nj} - \omega )}
```

```math
ℑ ( \Delta E_{n} ) = - \pi \sum_{\vec{k} ,\alpha} \sum_{j} \frac{| H_{nj} |^{2}}{\hbar} \delta ( \omega_{nj} - \omega ) = - \pi \sum_{\vec{k} ,\alpha} \sum_{j} | H_{nj} |^{2} \delta ( E_{n} - E_{j} - \hbar \omega )
```

<div align="right"><a href="book.pdf#page=475" target="_blank" title="This page in the PDF"><sup>[p. 475]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p476" href="book.pdf#page=476" target="_blank" title="This page in the PDF"><sup>[p. 476]</sup></a>

**All photon energies contribute to the real part**. Only **photons that satisfy the delta** **function constraint contribute to the imaginary part**. Moreover, there will only be an imaginary part if there is a lower energy state into which the state in question can decay. We can relate this width to those we previously calculated.

```math
- \frac{2}{\hbar} ℑ ( \Delta E_{n} ) = \sum_{\vec{k} ,\alpha} \sum_{j} \frac{2 \pi | H_{nj} |^{2}}{\hbar} \delta ( E_{n} - E_{j} - \hbar \omega )
```

The right hand side of this equation is just what we previously derived for the decay rate of state $`n`$, summed over all final states.

```math
- \frac{2}{\hbar} ℑ ( \Delta E_{n} ) = \Gamma_{n}
```

The **time dependence of the wavefunction for the state** $`n`$ **is modified by the self energy** **correction**.

```math
\psi_{n} ( \vec{x} , t ) = \psi_{n} ( \vec{x} ) e^{- i ( E_{n} + ℜ ( \Delta E_{n} ) ) t/ \hbar} e^{{}^{-} \tfrac{\Gamma nt}{2}}
```

This also gives us the **exponential decay behavior** that we expect, **keeping resonant scattering** **cross sections from going to infinity**. So, the width just goes into the time dependence as expected and we don’t have to worry about it anymore. We can now **concentrate on the energy** **shift due to the real part** of ∆$`E_{n}`$.

```math
\Delta E_{n} \equiv ℜ ( \Delta E_{n} ) = \sum_{\vec{k} ,\alpha} \sum_{j} \frac{| H_{nj} |^{2}}{\hbar ( \omega_{nj} - \omega )}
```

```math
H_{nj} = \langle n | H_{\vec{k} ,\alpha}^{abs} | j \rangle
```

```math
H^{abs} = - \sqrt{\frac{\hbar e^{2}}{2 m^{2} \omega V} e^{i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )}}
```

```math
\Delta E_{n} = \frac{\hbar e^{2}}{2 m^{2} V} \sum_{\vec{k} ,\alpha} \sum_{j} \frac{| \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )} | j \rangle |^{2}}{\hbar \omega ( \omega_{nj} - \omega )}
```

```math
= \frac{e^{2}}{2 m^{2} V} \int \frac{Vd^{3} k}{( 2 \pi )^{3}} \sum_{\alpha} \sum_{j} \frac{| \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )} | j \rangle |^{2}}{\omega ( \omega_{nj} - \omega )}
```

```math
= \frac{e^{2}}{( 2 \pi )^{3} 2 m^{2}} \sum_{\alpha} \sum_{j} \int d \Omega \frac{k^{2} dk}{\omega} \frac{| \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} \cdot \hat{\epsilon}^{( \alpha )} | j \rangle |^{2}}{( \omega_{nj} - \omega )}
```

```math
= \frac{e^{2}}{( 2 \pi )^{3} 2 m^{2} c^{3}} \sum_{j} \sum_{\alpha} \int d \Omega \int \frac{\omega | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle \cdot \hat{\epsilon}^{( \alpha )} |^{2}}{( \omega_{nj} - \omega )} d\omega
```

In our calculation of the total decay rate summed over polarization and integrated over photon direction (See section 29.7), we computed the cosine of the angle between each polarization vector and the (vector) matrix element. Summing these two and integrating over photon direction we got a factor of<sup>8</sup>$`3 \pi`$ and the polarization is eliminated from the matrix element. The **same calculation** **applies** here.

```math
\Delta E_{n} = \frac{e^{2}}{( 2 \pi )^{3} 2 m^{2} c^{3}} \sum_{j} \frac{8 \pi}{3} \int \frac{\omega | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2}}{( \omega_{nj} - \omega )} d\omega
```

<div align="right"><a href="book.pdf#page=476" target="_blank" title="This page in the PDF"><sup>[p. 476]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p477" href="book.pdf#page=477" target="_blank" title="This page in the PDF"><sup>[p. 477]</sup></a>

```math
= \frac{e^{2}}{6 \pi^{2} m^{2} c^{3}} \sum_{j} \int \frac{\omega | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2}}{( \omega_{nj} - \omega )} d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \int \frac{\omega | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2}}{( \omega_{nj} - \omega )} d\omega
```

Note that we wish to use the **electric dipole approximation which is not valid** for large $`k = \tfrac{\omega}{c}`$. It is valid up to about 2000 eV so we wish to cut off the calculation around there. While this calculation clearly diverges, things are less clear here because of the eventually rapid oscillation of the $`e^{i \vec{k} \cdot \vec{x}}`$ term in the integrand as the E1 approximation fails. Nevertheless, the largest differences in corrections between free electrons and bound electrons occur in the region in which the E1 approximation is valid. For now we will just use it and assume the cut-off is low enough.

It is the **difference between the bound electron’s self energy and that for a free electron** in which we are interested. Therefore, we will **start with the free electron with a definite** **momentum** $`\vec{p}`$. The normalized wave function for the free electron is $`\tfrac{1}{\sqrt{V}} e^{i \vec{p} \cdot \vec{x} / \hbar}`$.

```math
\Delta E_{free} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2} V^{2}} \sum_{\vec{p}^{\prime}} \int \frac{\omega}{( \omega_{nj} - \omega )} \left| \int e^{- i \vec{p} \cdot \vec{x} / \hbar} \vec{p} e^{i \vec{p}^{\prime} \cdot \vec{x} / \hbar} d^{3} x \right|^{2} d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2} V^{2}} | \vec{p} |^{2} \sum_{\vec{p}^{\prime}} \int \frac{\omega}{( \omega_{nj} - \omega )} \left| \int e^{i ( \vec{p}^{\prime} \cdot \vec{x} / \hbar - \vec{p} \cdot \vec{x} / \hbar )} d^{3} x \right|^{2} d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2} V^{2}} | \vec{p} |^{2} \sum_{\vec{p}^{\prime}} \int \frac{\omega}{( \omega_{nj} - \omega )} | V\delta_{\vec{p} ,}^{{}^{\prime} \vec{p}} |^{2} d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} \int_{0}^{\infty} \frac{\omega}{( \omega_{nj} - \omega )} d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} \int_{0}^{\infty} \frac{\omega}{( \omega_{nj} - \omega )} d\omega \to -\infty
```

It easy to see that this will go to negative infinity if the limit on the integral is infinite. It is quite reasonable to **cut off the integral at some energy** beyond which the theory we are using is invalid. Since we are still using non-relativistic quantum mechanics, the cut-off should have $`\hbar \omega <<mc^{2}`$. For the E1 approximation, it should be $`\hbar \omega << 2 \pi \hbar c/ 1`$ = 10$`keV`$. We will approximate $`\tfrac{\omega}{( \omega_{nj} - \omega )} \approx - 1`$ since the integral is just giving us a number and we are not interested in high accuracy here. We will be more interested in accuracy in the next section when we compute the difference between free electron and bound electron self energy corrections.

```math
\Delta E_{free} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} \int_{0}^{E_{cut - off} / \hbar} \frac{\omega}{( \omega_{nj} - \omega )} d\omega
```

```math
= - \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} \int_{0}^{E_{cut - off} / \hbar} d\omega
```

```math
= - \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} E_{cut - off} / \hbar
```

<div align="right"><a href="book.pdf#page=477" target="_blank" title="This page in the PDF"><sup>[p. 477]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p478" href="book.pdf#page=478" target="_blank" title="This page in the PDF"><sup>[p. 478]</sup></a>

```math
= - \frac{2 \alpha}{3 \pi m^{2} c^{2}} | \vec{p} |^{2} E_{cut - off}
```

```math
= - C | \vec{p} |^{2}
```

If we were hoping for little dependence on the cut-off we should be disappointed. This self energy calculated is **linear in the cut-off**.

For a **non-relativistic free electron** the energy $`\frac{p^{2}}{2 m}`$ decreases as the mass of the electron increases, so the **negative sign corresponds to a positive shift in the electron’s mass**, and hence an increase in the real energy of the electron. Later, we will think of this as a **renormalization of** **the electron’s mass**. The electron starts off with some **bare mass**. The self-energy due to the interaction of the electron’s charge with its own radiation field **increases the mass to what is** **observed**.

Note that the correction to the energy is a **constant times** $`p^{2}`$**, like the non-relativistic formula** **for the kinetic energy**.

```math
C \equiv \frac{2 \alpha}{3 \pi m^{2} c^{2}} E_{cut - off}
```

```math
\frac{p^{2}}{2 m_{obs}} = \frac{p^{2}}{2 m_{bare}} - Cp^{2}
```

```math
\frac{1}{m_{obs}} = \frac{1}{m_{bare}} - 2 C
```

```math
m_{obs} = \frac{m_{bare}}{1 - 2 Cm_{bare}} \approx ( 1 + 2 Cm_{bare} ) m_{bare} \approx ( 1 + 2 Cm ) m_{bare}
```

```math
= ( 1 + \frac{4 \alpha E_{cut - off}}{3 \pi mc^{2}} ) m_{bare}
```

If we **cut off the integral at** $`m_{e} c^{2}`$**, the correction to the mass is only about 0.3%**, but if we don’t cut off, its infinite. It makes no sense to trust our non-relativistic calculation up to infinite energy, so we must proceed with the cut-off integral.

If we use the Dirac theory, then we will be justified to move the cut-off up to very high energy. It turns out that the relativistic correction diverges logarithmically (instead of linearly) and the difference between bound and free electrons is finite relativistically (while it diverges logarithmically for our non-relativistic calculation).

Note that the self-energy of the free electron depends on the momentum of the electron, so we cannot simply subtract it from our bound state calculation. (What $`p^{2}`$ would we choose?) Rather me **must account for the mass renormalization**. We **used the observed electron mass in** **the calculation** of the Hydrogen bound state energies. In so doing, we have already included some of the self energy correction and we must not double correct. This is the subtraction we must make.

Its hard to keep all the minus signs straight in this calculation, particularly if we consider the bound and continuum electron states separately. The free particle correction to the electron mass is positive. Because we ignore the rest energy of the electron in our non-relativistic calculations, This makes a negative energy correction to both the bound $`( E = - \tfrac{1}{2 n^{2}} \alpha^{2} mc^{2} )`$ and continuum ($`E \approx \tfrac{p^{2}}{2 m}`$ ). Bound states and continuum states have the same fractional change in the energy. We need to add back in a positive term in ∆$`E_{n}`$ to avoid double counting of the self-energy correction. Since the

<div align="right"><a href="book.pdf#page=478" target="_blank" title="This page in the PDF"><sup>[p. 478]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p479" href="book.pdf#page=479" target="_blank" title="This page in the PDF"><sup>[p. 479]</sup></a>

bound state and continuum state terms have the same fractional change, it is convenient to just use $`\frac{p^{2}}{2 m}`$ for all the corrections.

```math
\frac{p^{2}}{2 m_{obs}} = \frac{p^{2}}{2 m_{bare}} - Cp^{2}
```

```math
\Delta E_{n}^{( obs )} = \Delta E_{n} + C \langle n | p^{2} | n \rangle = \Delta E_{n} + \frac{2 \alpha}{3 \pi m^{2} c^{2}} E_{cut - off} \langle n | p^{2} | n \rangle
```

Because we are correcting for the mass used to calculate the base energy of the state $`| n \rangle`$, our correction is written in terms of the electron’s momentum in that state.

# <a id="s-35-1-the-lamb-shift"></a>35.1 The Lamb Shift

In 1947, Willis E. Lamb and R. C. Retherford used microwave techniques to determine the **splitting** **between the** $`2 S 1_{2}`$ **and** $`2 P_{2} 1`$ **states in Hydrogen** to have a frequency of 1.06 GHz, (a wavelength of about 30 cm). (The shift is now accurately measured to be 1057.864 MHz.) This is about the same size as the hyperfine splitting of the ground state.

The technique used was quite interesting. They made a **beam of Hydrogen atoms in the** $`2 S 1`$ **state**, which has a very long lifetime because of selection rules. **Microwave radiation** with a<sup>2</sup> (fixed) frequency of 2395 MHz was used to cause transitions to the 2$`P 3_{2}`$ state and a **magnetic field** **was adjusted to shift the energy of the states** until the rate was largest. The **decay of the** $`2 P 3`$ **state to the ground state was observed** to determine the transition rate. From this, they were able to deduce the shift between the 2<sup>2</sup> $`S_{2} 1`$ and 2$`P 1_{2}`$ states.

**Hans Bethe used non-relativistic quantum mechanics** to calculate the self-energy correction to account for this observation.

<div align="right"><a href="book.pdf#page=479" target="_blank" title="This page in the PDF"><sup>[p. 479]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p480" href="book.pdf#page=480" target="_blank" title="This page in the PDF"><sup>[p. 480]</sup></a>

We now can compute the correction the same way he did.

```math
\Delta E_{n}^{( obs )} = \Delta E_{n} + C \langle n | p^{2} | n \rangle = \Delta E_{n} + \frac{2 \alpha}{3 \pi m^{2} c^{2}} E_{cut - off} \langle n | p^{2} | n \rangle
```

```math
\Delta E_{n} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \int \frac{\omega | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2}}{( \omega_{nj} - \omega )} d\omega
```

```math
\Delta E_{n}^{( obs )} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \int_{0}^{\omega_{cut - off}} \left( \sum_{j} \frac{\omega}{( \omega_{nj} - \omega )} | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2} + \langle n | p^{2} | n \rangle \right) d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \int_{0}^{\omega_{cut - off}} \sum_{j} \left( \frac{\omega}{( \omega_{nj} - \omega )} | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2} + \langle n | \vec{p} | j \rangle \langle j | \vec{p} | n \rangle \right) d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \int_{0}^{\omega_{cut - off}} \sum_{j} \left( \frac{\omega}{( \omega_{nj} - \omega )} | \langle n | e^{i \vec{k} \cdot \vec{x}} \vec{p} | j \rangle |^{2} + | \langle n | \vec{p} | j \rangle |^{2} \right) d\omega
```

It is now necessary to discuss **approximations needed** to complete this calculation. In particular, the electric dipole approximation will be of great help, however, it is certainly not warranted for large photon energies. For a good E1 approximation we need $`E_{\gamma} << 1973`$ eV. On the other hand, we want the cut-off for the calculation to be of order $`w_{cut - off} \approx mc^{2} / \hbar`$. We will use the E1 approximation and the high cut-off, as Bethe did, to get the right answer. At the end, the result from a relativistic calculation can be tacked on to show why it turns out to be the right answer. (We aren’t aiming for the worlds best calculation anyway.)

```math
\Delta E_{n}^{( obs )} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \int_{0}^{\omega_{cut - off}} \sum_{j} \left( \frac{\omega}{( \omega_{nj} - \omega )} | \langle n | \vec{p} | j \rangle |^{2} + | \langle n | \vec{p} | j \rangle |^{2} \right) d\omega
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \int^{\omega_{cut - off}} \sum_{j} \frac{\omega + ( \omega_{nj} - \omega )}{( \omega_{nj} - \omega )} | \langle n | \vec{p} | j \rangle |^{2} d\omega
```

<div align="right"><a href="book.pdf#page=480" target="_blank" title="This page in the PDF"><sup>[p. 480]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p481" href="book.pdf#page=481" target="_blank" title="This page in the PDF"><sup>[p. 481]</sup></a>

```math
= - \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \int_{0}^{\omega_{cut - off}} \frac{\omega_{nj}}{\omega - \omega_{nj}} | \langle n | \vec{p} | j \rangle |^{2} d\omega
```

```math
= - \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \omega_{nj} [ \log ( \omega - \omega_{nj} ) ]_{0}^{\omega_{cut - off}} | \langle n | \vec{p} | j \rangle |^{2}
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \omega_{nj} [ \log ( | \omega_{nj} | ) - \log ( \omega_{cut - off} - \omega_{nj} ) ] | \langle n | \vec{p} | j \rangle |^{2}
```

```math
\approx \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \omega_{nj} [ \log ( | \omega_{nj} | ) - \log ( \omega_{cut - off} ) ] | \langle n | \vec{p} | j \rangle |^{2}
```

```math
= \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \sum_{j} \omega_{nj} \log \left( \frac{| \omega_{nj} |}{\omega_{cut - off}} \right) | \langle n | \vec{p} | j \rangle |^{2}
```

The log term varies more slowly than does the rest of the terms in the sum. We can **approximate** **it by an average**. Bethe used numerical calculations to determine that the effective average of $`\hbar \omega_{nj}`$ is 8$`. 9 \alpha^{2} mc^{2}`$. We will do the same and pull the log term out as a constant.

```math
\Delta E_{n}^{( obs )} = \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \log \left( \frac{| \bar{\omega}_{nj} |}{\omega_{cut - off}} \right) \sum_{j} \omega_{nj} | \langle n | \vec{p} | j \rangle |^{2}
```

This sum can now be reduced further to a simple expression proportional to the $`| \psi_{n} ( 0 ) |^{2}`$ **using a** **typical clever quantum mechanics calculation**. The basic Hamiltonian for the Hydrogen atom is $`H_{0} = \tfrac{p^{2}}{2 m} + V ( r )`$.

```math
[ \vec{p} ,H_{0} ] = [ \vec{p} ,V ] = \frac{\hbar}{i} \vec{\nabla} V
```

```math
\langle j | [ \vec{p} ,H_{0} ] | n \rangle = \frac{\hbar}{i} \langle j | \vec{\nabla} V | n \rangle
```

```math
\sum_{j} \langle n | \vec{p} | j \rangle \langle j | [ \vec{p} ,H_{0} ] | n \rangle = \frac{\hbar}{i} \sum_{j} \langle n | \vec{p} | j \rangle \cdot \langle j | \vec{\nabla} V | n \rangle
```

```math
\sum_{j} ( E_{i} - E_{n} ) \langle n | \vec{p} | j \rangle \langle j | \vec{p} | n \rangle = \frac{\hbar}{i} \sum_{j} \langle n | \vec{p} | j \rangle \langle j | \vec{\nabla} V | n \rangle
```

This **must be a real number so we may use its complex conjugate**.

```math
\left( \sum_{j} ( E_{i} - E_{n} ) \langle n | \vec{p} | j \rangle \langle j | \vec{p} | n \rangle \right)^{*} = \sum_{j} ( E_{i} - E_{n} ) \langle n | \vec{p} | j \rangle \langle j | \vec{p} | n \rangle
```

```math
= - \frac{\hbar}{i} \sum_{j} \langle n | \vec{\nabla} V | j \rangle \langle j | \vec{p} | n \rangle
```

```math
\sum_{j} ( E_{i} - E_{n} ) \langle n | \vec{p} | j \rangle \langle j | \vec{p} | n \rangle = \frac{\hbar}{2 i} \left[ \sum_{j} \langle n | \vec{p} | j \rangle \langle j | \vec{\nabla} V | n \rangle - \langle n | \vec{\nabla} V | j \rangle \langle j | \vec{p} | n \rangle \right]
```

```math
= \frac{\hbar}{2 i} \langle n | [ \vec{p} , \vec{\nabla} V ] | n \rangle
```

```math
= - \hbar^{2} \langle n |\nabla^{2} V | n \rangle
```

<div align="right"><a href="book.pdf#page=481" target="_blank" title="This page in the PDF"><sup>[p. 481]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p482" href="book.pdf#page=482" target="_blank" title="This page in the PDF"><sup>[p. 482]</sup></a>

```math
= - \frac{\hbar^{2}}{2} \langle n | e^{2} \delta^{3} ( \vec{x} ) | n \rangle
```

```math
= - \frac{e^{2} \hbar^{2}}{2} | \psi_{n} ( 0 ) |^{2}
```

Only the **s states will have a non-vanishing probability to be at the origin** with $`| \psi_{n 0 0} ( 0 ) |^{2} = \frac{1}{\pi n^{3} a_{0}^{3}}`$ and $`a_{0} = \tfrac{\hbar}{\alpha mc}`$ . Therefore, only the s states will shift in energy appreciably. The shift will be.

```math
\Delta E_{n}^{( obs )} = - \frac{2 \alpha \hbar}{3 \pi m^{2} c^{2}} \log \left( \frac{| \bar{\omega}_{nj} |}{\omega_{cut - off}} \right) \frac{e^{2} \hbar}{2} \frac{1}{\pi n^{3}} \left( \frac{\alpha mc}{\hbar} \right)^{3}
```

```math
= \frac{\alpha^{4} e^{2} mc}{3 \pi^{2} \hbar n^{3}} \log \left( \frac{\omega_{cut - off}}{| \bar{\omega}_{nj} |} \right)
```

```math
= \frac{4 \alpha^{5} mc^{2}}{3 \pi n^{3}} \log \left( \frac{\omega_{cut - off}}{| \bar{\omega}_{nj} |} \right)
```

```math
\Delta E_{2 s}^{( obs )} = \frac{\alpha^{5} mc^{2}}{6 \pi} \log \left( \frac{mc^{2}}{8 . 9 \alpha^{2} mc^{2}} \right)
```

```math
\nu = \frac{\Delta E_{2 s}^{( obs )}}{2 \pi \hbar} = \frac{\alpha^{5} mc^{2} c}{1 2 \pi^{2} \hbar c} \log \left( \frac{1}{8 . 9 \alpha^{2}} \right) = 1 . 0 4 1 GHz
```

This **agrees far too well** with the measurement, considering the approximations made and the dependence on the cut-off. There is, however, justification in the relativistic calculation. Typically, the full calculation was made by using this non-relativistic approach up to some energy of the order of $`\alpha mc^{2}`$, and using the relativistic calculation above that. The **relativistic free electron** **self-energy correction diverges only logarithmically and a very high cutoff can be used** without a problem. The mass of the electron is renormalized as above. The **Lamb shift does not** **depend on the cutoff** and hence it is well calculated. We only need the non-relativistic part of the calculation up to photon energies for which the E1 approximation is OK. The **relativistic part** of the calculation down to $`\omega_{min}`$ yields.

```math
\Delta E_{n} = \frac{4 \alpha^{5}}{3 \pi n^{3}} \left( \log \left( \frac{mc^{2}}{2 \hbar \omega_{min}} \right) + \frac{1 1}{2 4} - \frac{1}{5} \right) mc^{2}
```

The **non-relativistic calculation** gave.

```math
\Delta E_{n} = \frac{4 \alpha^{5}}{3 \pi n^{3}} \log \left( \frac{\omega_{min}}{| \bar{\omega}_{nj} |} \right) mc^{2}
```

So the **sum of the two gives**.

```math
\Delta E_{n}^{( obs )} = \frac{4 \alpha^{5}}{3 \pi n^{3}} \left( \log \left( \frac{mc^{2}}{2 \hbar \bar{\omega}_{nj}} \right) + \frac{1 1}{2 4} - \frac{1}{5} \right) mc^{2}
```

The **dependence on** $`\omega_{min}`$ **cancels**. In this calculation, the $`mc^{2}`$ in the log is the outcome of the relativistic calculation, not the cutoff. The electric dipole approximation is even pretty good since we did not need to go up to large photon energies non-relativistically and no E1 approximation is needed for the relativistic part. That’s how we (and Bethe) got about the right answer.

The **Lamb shift splits the** $`2 S 1`$ **and** $`2 P 1`$ **states which are otherwise degenerate**. Its origin is **purely from field theory**. The experimental<sup>2</sup> $`2`$ **measurement of the Lamb shift stimulated**

<div align="right"><a href="book.pdf#page=482" target="_blank" title="This page in the PDF"><sup>[p. 482]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p483" href="book.pdf#page=483" target="_blank" title="This page in the PDF"><sup>[p. 483]</sup></a>

**theorists to develop Quantum ElectroDynamics**. The correction increases the energy of s states. One may think of the physical origin as the electron becoming less pointlike as virtual photons are emitted and reabsorbed. Spreading the electron out a bit decreases the effect of being in the deepest part of the potential, right at the origin. Based on the energy shift, I estimate that the **electron in the 2s state is spread out by about 0.005 Angstroms**, much more than the size of the nucleus.

The **anomalous magnetic moment of the electron**, $`g - 2`$, which can also be calculated in field theory, makes a small contribution to the Lamb shift.

---

[← 34 Scattering of Photons](34-scattering-of-photons.md) · [36 Dirac Equation →](36-dirac-equation.md)
