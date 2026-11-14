[← 33 Quantum Theory of Radiation](33-quantum-theory-of-radiation.md) · [35 Electron Self Energy Corrections →](35-electron-self-energy-corrections.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p463" href="book.pdf#page=463" target="_blank" title="This page in the PDF"><sup>[p. 463]</sup></a>

# 34 Scattering of Photons

In the scattering of photons, for example from an atom, an **initial state photon** with wave-number<br>
$`\vec{k}`$ and polarization $`\hat{\epsilon}`$ is absorbed by the atom **and a final state photon** with wave-number $`\vec{k}^{\prime}`$ and polarization $`\hat{\epsilon}^{\prime}`$ is emitted. The atom may remain in the same state (elastic scattering) or it may<br>
change to another state (inelastic). Any calculation we will do will use the **matrix element of the** **interaction Hamiltonian between initial and final states**.

```math
H_{ni} = \langle n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} | H_{int} | i ; \vec{k} \hat{\epsilon}^{( \alpha )} \rangle
```

```math
H_{int} = - \frac{e}{mc} \vec{A} ( x ) \cdot \vec{p} + \frac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A}
```

The scattering process clearly requires terms in $`H_{int}`$ that **annihilate one photon and create**<br>
**another**. The order does not matter. The $`\tfrac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A}`$ is the square of the Fourier decomposition of<br>
the radiation field so it contains terms like $`a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} a_{k,\alpha}`$ and $`a_{k,\alpha} a_{k^{\prime} ,\alpha^{\prime}}^{\dagger}`$ which are just what we want.<br>
The $`- \tfrac{e}{mc} \vec{A} \cdot \vec{p}`$ term has both creation and annihilation operators in it but not products of them.<br>
It changes the number of photons by plus or minus one, not by zero as required for the scattering<br>
process. Nevertheless this part of the interaction could contribute in second order perturbation<br>
theory, by absorbing one photon in a transition from the initial atomic state to an intermediate<br>
state, then emitting another photon and making a transition to the final atomic state. While this<br>
is higher order in perturbation theory, it is the same order in the electromagnetic coupling constant<br>
$`e`$, which is what really counts when expanding in powers of $`\alpha`$. Therefore, we will need to **consider**<br>
**the** $`\frac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A}`$ **term in first order and the** $`- \tfrac{e}{mc} \vec{A} \cdot \vec{p}`$ **term in second order** perturbation theory<br>
to get an order $`\alpha`$ calculation of the matrix element.

Start with the first order perturbation theory term. All the terms in the sum that do not annihi-<br>
late the initial state photon and create the final state photon give zero. We will assume that the<br>
wavelength of the photon’s is long compared to the size of the atom so that $`e^{i \vec{k} \cdot \vec{r}} \approx 1`$.

```math
A_{\mu} ( x ) = \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \epsilon_{\mu}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\frac{e^{2}}{2 mc^{2}} \langle n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} | \vec{A} \cdot \vec{A} | i ; \vec{k} \hat{\epsilon}^{( \alpha )} \rangle = \frac{e^{2}}{2 mc^{2}} \frac{1}{V} \frac{\hbar c^{2}}{2 \sqrt{\omega^{\prime} \omega}} \epsilon_{\mu}^{( \alpha )} \epsilon_{\mu}^{( \alpha^{\prime} )} \langle n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} | \left( a_{k,\alpha} a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} + a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} a_{k,\alpha} \right) e^{i ( k_{\rho} - k_{\rho}^{\prime} ) x_{\rho}} | i ; \vec{k} \hat{\epsilon}^{( \alpha )} \rangle
```

```math
= \frac{e^{2}}{2 mc^{2}} \frac{1}{V} \frac{\hbar c^{2}}{2 \sqrt{\omega^{\prime} \omega}} \epsilon_{\mu}^{( \alpha )} \epsilon_{\mu}^{( \alpha^{\prime} )} e^{- i ( \omega - \omega^{\prime} ) t} \langle n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} | 2 | i ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} \rangle
```

```math
= \frac{e^{2}}{2 mc^{2}} \frac{1}{V} \frac{\hbar c^{2}}{2 \sqrt{\omega^{\prime} \omega}} \epsilon_{\mu}^{( \alpha )} \epsilon_{\mu}^{( \alpha^{\prime} )} e^{- i ( \omega - \omega^{\prime} ) t} 2 \langle n | i \rangle
```

```math
= \frac{e^{2}}{2 mc^{2}} \frac{1}{V} \frac{\hbar c^{2}}{\sqrt{\omega^{\prime} \omega}} \epsilon_{\mu}^{( \alpha )} \epsilon_{\mu}^{( \alpha^{\prime} )} e^{- i ( \omega - \omega^{\prime} ) t} \delta_{ni}
```

This is the matrix element $`H_{ni} ( t )`$. The **amplitude to be in the final state** $`| n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} \rangle`$ **is given**<br>
**by first order time dependent perturbation theory**.

```math
c_{n}^{( 1 )} ( t ) = \frac{1}{i \hbar} \int^{t} e^{i\omega_{ni} t^{\prime}} H_{ni} ( t^{\prime} ) dt^{\prime}
```

<div align="right"><a href="book.pdf#page=463" target="_blank" title="This page in the PDF"><sup>[p. 463]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p464" href="book.pdf#page=464" target="_blank" title="This page in the PDF"><sup>[p. 464]</sup></a>

```math
c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha \prime )}}^{( 1 )} ( t ) = \frac{1}{i \hbar} \frac{e^{2}}{2 mc^{2}} \frac{1}{V} \frac{\hbar c^{2}}{\sqrt{\omega^{\prime} \omega}} \epsilon_{\mu}^{( \alpha )} \epsilon_{\mu}^{( \alpha^{\prime} )} \delta_{ni} \int_{0}^{t} e^{i\omega_{ni} t^{\prime}} e^{- i ( \omega - \omega^{\prime} ) t} dt^{\prime}
```

```math
= \frac{e^{2}}{2 imV \sqrt{\omega^{\prime} \omega}} \hat{\epsilon}^{( \alpha )} \cdot \hat{\epsilon}^{( \alpha^{\prime} )} \delta_{ni} \int_{0}^{t} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t^{\prime}} dt^{\prime}
```

Recall that the absolute square of the time integral will turn into 2$`\pi t\delta ( \omega_{ni} + \omega^{\prime} - \omega )`$. We will carry along the integral for now, since we are not yet ready to square it.

Now we very carefully put the interaction term into the formula for **second order time dependent** **perturbation theory**, again using $`e^{i \vec{k} \cdot \vec{x}} \approx 1`$. Our notation is that the **intermediate state of** **atom and field is called** $`| I \rangle = | j,n_{\vec{k} ,\alpha} ,n_{\vec{k}^{\prime} ,\alpha^{\prime}} \rangle`$ where $`j`$ represents the state of the atom and we may have zero or two photons, as indicated in the diagram.

```math
\begin{aligned} \mathcal{V} &= - \frac{e}{mc} \vec{A} \cdot \vec{p} = - \frac{e}{mc} \frac{1}{\sqrt{V}} \sum_{\vec{k} \alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p} \left( a_{k,\alpha} e^{- i\omega t} + a_{k,\alpha}^{\dagger} e^{i\omega t} \right) \\ c_{n}^{( 2 )} ( t ) &= \frac{- 1}{\hbar^{2}} \sum \int^{t} dt_{2} \mathcal{V}_{nI} ( t_{2} ) e^{i\omega_{nj} t_{2}} \int^{t_{2}} dt_{1} e^{i\omega_{ji} t_{1}} \mathcal{V}_{Ii} ( t_{1} ) \\ c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha \prime )}}^{( 2 )} ( t ) &= \frac{- e^{2}}{m^{2} c^{2} \hbar^{2}} \sum_{I}^{j, \vec{k} ,\alpha 0} \frac{1}{V} \frac{\hbar c^{2}}{2 \sqrt{\omega^{\prime} \omega}} \int_{0}^{t} dt_{2}^{0} \langle n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha^{\prime} )} | ( \hat{\epsilon}^{( \alpha )} a_{k,\alpha} e^{- i\omega t_{2}} + \hat{\epsilon}^{( \alpha^{\prime} )} a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} e^{i\omega^{\prime} t_{2}} ) \cdot \vec{p} | I \rangle e^{i\omega_{nj} t_{2}} \end{aligned}
```

```math
\times \int_{0}^{t_{2}} dt_{1} e^{i\omega_{ji} t_{1}} \langle I | ( \hat{\epsilon}^{( \alpha )} a_{k,\alpha} e^{- i\omega t_{1}} + \hat{\epsilon}^{( \alpha^{\prime} )} a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} e^{i\omega^{\prime} t_{1}} ) \cdot \vec{p} | i ; \vec{k} \hat{\epsilon}^{( \alpha )} \rangle
```

We can understand this formula as a second order transition from state $`| i \rangle`$ to state $`| n \rangle`$ through all possible intermediate states. The transition from the initial state to the intermediate state takes place at time $`t_{1}`$. The transition from the intermediate state to the final state takes place at time $`t_{2}`$.

The **space-time diagram** below shows the three terms in $`c_{n} ( t )`$ Time is assumed to run upward in the diagrams.

Diagram (c) represents the $`A^{2}`$ term in which one photon is absorbed and one emitted at the same<span class="page" style="float:left;width:100%;margin:0.4em 0 0.6em"><span style="display:block;text-align:right"><a href="book.pdf#page=464" target="_blank" title="This page in the PDF"><sup>[p. 464]</sup></a></span><span style="display:block;height:2px;background:currentColor;opacity:0.25"></span><a id="p465" href="book.pdf#page=465" target="_blank" title="This page in the PDF"><sup>[p. 465]</sup></a></span> point. Diagrams (a) and (b) represent two second order terms. In diagram (a) the initial state photon is absorbed at time $`t_{1}`$, leaving the atom in an intermediate state which may or may not be the same as the initial (or final) atomic state. This intermediate state has no photons in the field. In diagram (b), the atom emits the final state photon at time $`t_{1}`$, leaving the atom in some intermediate state. The intermediate state $`| I \rangle`$ includes two photons in the field for this diagram. At time $`t_{2}`$ the atom absorbs the initial state photon.

Looking again at the formula for the second order scattering amplitude, note that we integrate over the times $`t_{1}`$ and $`t_{2}`$ and that $`t_{1} <t_{2}`$. For diagram (a), the annihilation operator $`a_{k,\alpha}`$ is active at time $`t_{1}`$ and the creation operator is active at time $`t_{2}`$. For diagram (b) its just the opposite. The second order formula above contains four terms as written. The $`a^{\dagger} a`$ and $`aa^{\dagger}`$ terms are the ones described by the diagram. The $`aa`$ and $`a^{\dagger} a^{\dagger}`$ terms will clearly give zero. Note that we are just picking the terms that will survive the calculation, not changing any formulas.

Now, reduce to the two nonzero terms. The operators just give a factor of 1 and make the photon states work out. If $`| j \rangle`$ is the intermediate atomic state, the **second order term reduces to**.

```math
\begin{aligned} c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{( \alpha \prime )}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 Vm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \int_{0}^{t} dt_{2} \int_{0}^{t_{2}} dt_{1} \left[ e^{i ( \omega^{\prime} + \omega_{nj} ) t_{2}} \langle n | \hat{\epsilon}^{( \alpha^{\prime} )} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{( \alpha )} \cdot \vec{p} | i \rangle e^{i ( \omega_{ji} - \omega ) t_{1}} \right. \\ & \quad \left. + \quad e^{i ( \omega_{nj} - \omega ) t_{2}} \langle n | \hat{\epsilon}^{( \alpha )} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{( \alpha )^{\prime}} \cdot \vec{p} | i \rangle e^{i ( \omega^{\prime} + \omega_{ji} ) t_{1}} \right] \\ c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 Vm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \int_{0}^{t} dt_{2} \left[ e^{i ( \omega^{\prime} + \omega_{nj} ) t_{2}} \langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle \left[ \frac{e^{i ( \omega_{ji} - \omega ) t_{1}}}{i ( \omega_{ji} - \omega )} \right] \frac{t_{2}}{0} \right. \\ & \quad \left. + \quad e^{i ( \omega_{nj} - \omega ) t_{2}} \langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle \left[ \frac{e^{i ( \omega^{\prime} + \omega_{ji} ) t_{1}}}{i ( \omega^{\prime} + \omega_{ji} )} \right] \frac{t_{2}}{0} \right] \\ c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 Vm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \int_{0}^{t} dt_{2} \left[ e^{i ( \omega^{\prime} + \omega_{nj} ) t_{2}} \langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle \left[ \frac{e^{i ( \omega_{ji} - \omega ) t_{2}} - 1}{i ( \omega_{ji} - \omega )} \right] \right. \\ & \quad \left. + \quad e^{i ( \omega_{nj} - \omega ) t_{2}} \langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle \left[ \frac{e^{i ( \omega^{\prime} + \omega_{ji} ) t_{2}} - 1}{i ( \omega^{\prime} + \omega_{ji} )} \right] \right] \end{aligned}
```

The $`- 1`$ terms coming from the integration over $`t_{1}`$ can be dropped. We can anticipate that the integral over $`t_{2}`$ will eventually give us a delta function of energy conservation, going to infinity when energy is conserved and going to zero when it is not. Those $`- 1`$ terms can never go to infinity and can therefore be neglected. When the energy conservation is satisfied, those terms are negligible and when it is not, the whole thing goes to zero.

```math
\begin{aligned} c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 Vm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \int_{0}^{t} dt_{2} \left[ e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}} \langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle \left[ \frac{1}{i ( \omega_{ji} - \omega )} \right] \right. \\ & \quad \left. + \quad e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}} \langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle \left[ \frac{1}{i ( \omega^{\prime} + \omega_{ji} )} \right] \right] \\ c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 iVm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \end{aligned}
```

<div align="right"><a href="book.pdf#page=465" target="_blank" title="This page in the PDF"><sup>[p. 465]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p466" href="book.pdf#page=466" target="_blank" title="This page in the PDF"><sup>[p. 466]</sup></a>

```math
\times \int_{0}^{t} dt_{2} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}}
```

We have calculated all the amplitudes. The **first order and second order amplitudes should** **be combined, then squared**.

```math
c_{n} ( t ) = c_{n}^{( 1 )} ( t ) + c_{n}^{( 2 )} ( t )_{t}
```

```math
\begin{aligned} c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 1 )} ( t ) &= \frac{e^{2}}{2 iVm \sqrt{\omega^{\prime} \omega}} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} \delta_{ni} \int_{0} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t^{\prime}} dt^{\prime} \\ c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}}^{( 2 )} ( t ) &= \frac{- e^{2}}{2 iVm^{2} \hbar \sqrt{\omega^{\prime} \omega}} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \int_{0}^{t} dt_{2} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}} \end{aligned}
```

```math
\begin{aligned} c_{n ; \vec{k}^{\prime} \hat{\epsilon}^{\prime}} ( t ) &= \left( \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right) \\ & \quad \times \quad \frac{e^{2}}{2 iVm \sqrt{\omega^{\prime} \omega}} \int_{0}^{t} dt_{2} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}} \end{aligned}
```

```math
\begin{aligned} | c ( t ) |^{2} &= \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2} \\ & \quad \times \quad \frac{e^{4}}{4 V^{2} m^{2} \omega^{\prime} \omega} \left| \int_{0}^{t} dt_{2} e^{i ( \omega_{ni} + \omega^{\prime} - \omega ) t_{2}} \right|^{2} \end{aligned}
```

```math
\begin{aligned} | c ( t ) |^{2} &= \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2} \\ & \quad \times \quad \frac{e^{4}}{4 V^{2} m^{2} \omega^{\prime} \omega} 2 \pi t\delta ( \omega_{ni} + \omega^{\prime} - \omega ) \end{aligned}
```

```math
\begin{aligned} \Gamma &= \int \frac{Vd^{3} k^{\prime}}{( 2 \pi )^{3}} \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2} \\ & \quad \times \quad \frac{e^{4}}{4 V^{2} m^{2} \omega^{\prime} \omega} 2 \pi \delta ( \omega_{ni} + \omega^{\prime} - \omega )_{2} \end{aligned}
```

```math
\begin{aligned} \Gamma &= \int \frac{V\omega^{\prime 2} d\omega^{\prime} d \Omega}{( 2 \pi c )^{3}} \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right| \\ & \quad \times \quad \frac{e^{4}}{4 V^{2} m^{2} \omega^{\prime} \omega} 2 \pi \delta ( \omega_{ni} + \omega^{\prime} - \omega ) \end{aligned}
```

<div align="right"><a href="book.pdf#page=466" target="_blank" title="This page in the PDF"><sup>[p. 466]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p467" href="book.pdf#page=467" target="_blank" title="This page in the PDF"><sup>[p. 467]</sup></a>

```math
\begin{aligned} \Gamma &= \int d \Omega \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2} \\ & \quad \times \quad \frac{V\omega^{\prime 2}}{( 2 \pi c )^{3}} \frac{e^{4}}{4 V^{2} m^{2} \omega^{\prime} \omega} 2 \pi \end{aligned}
```

```math
\frac{d \Gamma}{d \Omega} = \frac{e^{4} \omega^{\prime}}{( 4 \pi )^{2} Vm^{2} c^{3} \omega} \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2}
```

Note that the delta function has enforced energy conservation requiring that $`\omega^{\prime} = \omega - \omega_{ni}`$, but **we** **have left** $`\omega^{\prime}`$ **in the formula for convenience**.

The final step to a **differential cross section** is to divide the transition rate by the **incident flux** **of particles**. This is a surprisingly easy step because we are using plane waves of photons. The initial state is **one particle in the volume** $`V`$ **moving with a velocity of** $`c`$, so the flux is simply

```math
\tfrac{c}{V} . \quad \frac{d\sigma}{d \Omega} = \frac{e^{4} \omega^{\prime}}{( 4 \pi )^{2} m^{2} c^{4} \omega} \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega^{\prime} + \omega_{ji}} \right] \right|^{2}
```

The **classical radius of the electron** is defined to be $`r_{0} = \tfrac{e^{2}}{4 \pi mc^{2}}`$ in our units. We will factor the square of this out but leave the answer in terms of fundamental constants.

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{\omega^{\prime}}{\omega} \right) \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} + \omega^{\prime}} \right] \right|^{2}
```

This is called the **Kramers-Heisenberg Formula**. Even now, the three (space-time) **Feynman** **diagrams** are visible as separate terms in the formula.

```math
2
```

(They show up like $`\left| c + \sum ( a + b ) \right|`$ .) Note that, for the very short time that the system is in an

```math
j
```

intermediate state, **energy conservation is not strictly enforced**. The energy denominators in the formula suppress larger energy non-conservation. The formula can be applied to several physical situations as discussed below.

<div align="right"><a href="book.pdf#page=467" target="_blank" title="This page in the PDF"><sup>[p. 467]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p468" href="book.pdf#page=468" target="_blank" title="This page in the PDF"><sup>[p. 468]</sup></a>

Also note that the formula yields an infinite result if $`\omega = \pm \omega_{ji}`$. This is not a physical result. In fact the cross section will be large but not infinite when energy is conserved in the intermediate state. This condition is often refereed to as “the intermediate state being **on the mass shell**” because of the relation between energy and mass in four dimensions.

# <a id="s-34-1-resonant-scattering"></a>34.1 Resonant Scattering

The Kramers-Heisenberg photon scattering cross section, below, **has unphysical infinities if an** **intermediate state is on the mass shell**.

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{\omega^{\prime}}{\omega} \right) \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} + \omega^{\prime}} \right] \right|^{2}
```

In reality, the cross section becomes large but not infinite. These infinities come about because we have not properly accounted for the finite lifetime of the intermediate state when we derived the second order perturbation theory formula. If the energy width of the intermediate states is included in the calculation, as we will attempt below, the cross section is large but not infinite. The **resonance in the cross section** will exhibit the same shape and width as does the intermediate state.

These resonances in the cross section **can dominate scattering**. Again both resonant terms in the cross section, **occur if an intermediate state has the right energy** so that energy is conserved.

# <a id="s-34-2-elastic-scattering"></a>34.2 Elastic Scattering

In elastic scattering, the **initial and final atomic states are the same**, as are the initial and final photon energies.

```math
\frac{d\sigma_{elastic}}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left| \delta_{ii} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle i | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle i | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} + \omega} \right] \right|^{2}
```

With the help of some commutators, **the** $`\delta_{ii}`$ **term can be combined with the others**.

The commutator $`[ \vec{x} , \vec{p} ]`$ (with no dot products) can be very useful in calculations. When the two vectors are multiplied directly, we get something with two Cartesian indices.

```math
x_{i} p_{j} - p_{j} x_{i} = i \hbar \delta_{ij}
```

The commutator of the vectors is $`i \hbar`$ times the identity. This can be used to cast the first term above into something like the other two.

```math
x_{i} p_{j} - p_{j} x_{i} = i \hbar \delta_{ij}
```

```math
\hat{\epsilon} \cdot \hat{\epsilon}^{\prime} = \hat{\epsilon}_{i} \hat{\epsilon}_{j}^{\prime} \delta_{ij}
```

```math
i \hbar \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} = \hat{\epsilon}_{i} \hat{\epsilon}_{j}^{\prime} ( x_{i} p_{j} - p_{j} x_{i} )
```

```math
= ( \hat{\epsilon} \cdot \vec{x} ) ( \hat{\epsilon}^{\prime} \cdot p ) - ( \hat{\epsilon}^{\prime} \cdot \vec{p} ) ( \hat{\epsilon} \cdot \vec{x} )
```

<div align="right"><a href="book.pdf#page=468" target="_blank" title="This page in the PDF"><sup>[p. 468]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p469" href="book.pdf#page=469" target="_blank" title="This page in the PDF"><sup>[p. 469]</sup></a>

Now we need to put the states in using an identity, then use the commutator with $`H`$ to change $`\vec{x}`$ to $`\vec{p}`$.

```math
1 = \sum_{j} \langle i | j \rangle \langle j | i \rangle
```

```math
i \hbar \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} = \sum_{j} [ ( \hat{\epsilon} \cdot \vec{x} )_{ij} ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ji} - ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ij} ( \hat{\epsilon} \cdot \vec{x} )_{ji} ]
```

```math
[ H , \vec{x} ] = \frac{\hbar}{im} \vec{p}
```

```math
\frac{\hbar}{im} ( \hat{\epsilon} \cdot \vec{p} )_{ij} = ( \hat{\epsilon} \cdot [ H , \vec{x} ] )_{ij}
```

```math
\begin{aligned} &= \hbar \omega_{ij} ( \hat{\epsilon} \cdot \vec{x} )_{ij} \\ ( \hat{\epsilon} \cdot \vec{x} )_{ij} &= \frac{- i}{m\omega} ( \hat{\epsilon} \cdot \vec{p} )_{ij} \\ i \hbar \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} &= \sum^{ij} \left[ \frac{- i}{m\omega_{ij}} ( \hat{\epsilon} \cdot \vec{p} )_{ij} ( \hat{\epsilon}^{\prime} \cdot p )_{ji} - \frac{- i}{m\omega_{ji}} ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ij} ( \hat{\epsilon} \cdot \vec{p} )_{ji} \right] \\ &= \sum^{j} \left[ \frac{- i}{m\omega_{ij}} ( \hat{\epsilon} \cdot \vec{p} )_{ij} ( \hat{\epsilon}^{\prime} \cdot p )_{ji} + \frac{- i}{m\omega_{ij}} ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ij} ( \hat{\epsilon} \cdot \vec{p} )_{ji} \right] \\ &= \sum^{j} \frac{- i}{m\omega_{ij}} [ ( \hat{\epsilon} \cdot \vec{p} )_{ij} ( \hat{\epsilon}^{\prime} \cdot p )_{ji} + ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ij} ( \hat{\epsilon} \cdot \vec{p} )_{ji} ] \\ \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} &= \frac{- 1}{m \hbar}^{j} \sum_{j} \frac{1}{\omega_{ij}} [ ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ij} ( \hat{\epsilon} \cdot \vec{p} )_{ji} + ( \hat{\epsilon} \cdot \vec{p} )_{ij} ( \hat{\epsilon}^{\prime} \cdot \vec{p} )_{ji} ] \end{aligned}
```

(Reminder: $`\omega_{ij} = \tfrac{E_{i} - E_{j}}{\hbar}`$ is just a number. ($`\hat{\epsilon} \cdot \vec{p} )_{ij} = \langle i | \hat{\epsilon} \cdot \vec{p} | j \rangle`$ is a matrix element between states.)

We may now **combine the terms** for elastic scattering.

```math
\begin{aligned} \frac{d\sigma_{elas}}{d \Omega} &= \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left| \delta_{ii} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle i | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle i | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} + \omega} \right] \right|^{2} \\ \delta_{ii} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} &= \frac{- 1}{m \hbar} \sum_{j} \left[ \frac{\langle i | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} ) | i \rangle}{\omega_{ij}} + \frac{\langle i | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ij}} \right] \end{aligned}
```

```math
\frac{1}{\omega_{ij}} + \frac{1}{\omega_{ji} \pm \omega} = \frac{\omega_{ji} \pm \omega + \omega_{ij}}{\omega_{ij} ( \omega_{ji} \pm \omega )} = \frac{\mp \omega}{\omega_{ji} ( \omega_{ji} \pm \omega )}
```

```math
\frac{d\sigma_{elas}}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{1}{m \hbar} \right)^{2} \left| \sum_{j} \left[ \frac{\omega \langle i | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} ( \omega_{ji} - \omega )} - \frac{\omega \langle i | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} ( \omega_{ji} + \omega )} \right] \right|^{2}
```

This is a **nice symmetric form for elastic scattering**. If computation of the matrix elements is planned, it useful to again use the commutator to change $`\vec{p}`$ into $`\vec{x}`$.

```math
\frac{d\sigma_{elas}}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{m\omega}{\hbar} \right)^{2} \left| \sum_{j} \omega_{ji} \left[ \frac{\langle i | \hat{\epsilon}^{\prime} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{x} | i \rangle}{\omega_{ji} - \omega} - \frac{\langle i | \hat{\epsilon} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{x} | i \rangle}{\omega_{ji} + \omega} \right] \right|^{2}
```

<div align="right"><a href="book.pdf#page=469" target="_blank" title="This page in the PDF"><sup>[p. 469]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p470" href="book.pdf#page=470" target="_blank" title="This page in the PDF"><sup>[p. 470]</sup></a>

# <a id="s-34-3-rayleigh-scattering"></a>34.3 Rayleigh Scattering

Lord Rayleigh calculated **low energy elastic scattering of light** from atoms using classical electromagnetism. If the energy of the scattered photon is much less than the energy needed to excite an atom, $`\omega <<\omega_{ji}`$, then the cross section may be approximated.

```math
\frac{\mp \omega_{ji}}{\omega_{ji} \pm \omega} = \frac{\mp \omega_{ji}}{\omega_{ji} ( 1 \pm \tfrac{\omega}{\omega_{ji}} )} = \mp ( 1 \mp \frac{\omega}{\omega_{ji}} ) = \mp 1 + \frac{\omega}{\omega_{ji}}
```

```math
\frac{d\sigma_{elas}}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{m\omega}{\hbar} \right)^{2} \left| \sum_{j} \left[ \frac{\omega_{ji} \langle i | \hat{\epsilon}^{\prime} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{x} | i \rangle}{\omega_{ji} - \omega} - \frac{\omega_{ji} \langle i | \hat{\epsilon} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{x} | i \rangle}{\omega_{ji} + \omega} \right] \right|^{2}
```

```math
\begin{aligned} &= \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{m\omega}{\hbar} \right)^{2} | \sum_{j} [ ( \langle i | \hat{\epsilon}^{\prime} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{x} | i \rangle - \langle i | \hat{\epsilon} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{x} | i \rangle ) \\ & \quad \left. + \frac{\omega}{\omega_{ji}} ( \langle i | \hat{\epsilon}^{\prime} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{x} | i \rangle + \langle i | \hat{\epsilon} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{x} | i \rangle ) \right] |_{2}^{2} \end{aligned}
```

```math
= \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{m}{\hbar} \right)^{2} \omega^{4} \left| \sum_{j} \left[ \frac{1}{\omega_{ji}} ( \langle i | \hat{\epsilon}^{\prime} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{x} | i \rangle + \langle i | \hat{\epsilon} \cdot \vec{x} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{x} | i \rangle ) \right] \right|
```

For the **colorless gasses** (like the ones in our atmosphere), the first excited state in the UV, so the scattering of visible light with be proportional to $`\omega^{4}`$, which explains why the sky is blue and sunsets are red. Atoms with intermediate states in the visible will appear to be colored due to the strong resonances in the scattering. **Rayleigh** got the same dependence from classical physics.

# <a id="s-34-4-thomson-scattering"></a>34.4 Thomson Scattering

If the energy of the scattered photon is much bigger than the binding energy of the atom, $`\omega >> 1`$ eV. then cross section approaches that for **scattering from a free electron, Thomson Scattering**. We still neglect the effect of electron recoil so we should also require that $`\hbar \omega <<m_{e} c^{2}`$. Start from the Kramers-Heisenberg formula.

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \left( \frac{\omega^{\prime}}{\omega} \right) \left| \delta_{ni} \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} - \frac{1}{m \hbar} \sum_{j} \left[ \frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{\omega_{ji} - \omega} + \frac{\langle n | \hat{\epsilon} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon}^{\prime} \cdot \vec{p} | i \rangle}{\omega_{ji} + \omega^{\prime}} \right] \right|^{2}
```

The $`\hbar \omega = \hbar \omega^{\prime}`$ denominators are much larger than $`\frac{\langle n | \hat{\epsilon}^{\prime} \cdot \vec{p} | j \rangle \langle j | \hat{\epsilon} \cdot \vec{p} | i \rangle}{m}`$ which is of the order of the electron’s kinetic energy, so we can ignore the second two terms. (Even if the intermediate and final states have unbound electrons, the initial state wave function will keep these terms small.)

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} | \hat{\epsilon} \cdot \hat{\epsilon}^{\prime} |^{2}
```

This scattering **cross section is of the order of the classical radius of the electron squared**, and is **independent of the frequency** of the light.

The only dependence is on polarization. This is a good time to take a look at the meaning of the **polarization vectors** we’ve been carrying around in the calculation and at the lack of any

<div align="right"><a href="book.pdf#page=470" target="_blank" title="This page in the PDF"><sup>[p. 470]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p471" href="book.pdf#page=471" target="_blank" title="This page in the PDF"><sup>[p. 471]</sup></a>

wave-vectors for the initial and final state. A look back at the calculation shows that we calculated the transition rate from a state with one photon with wave-vector $`\vec{k}`$ and polarization $`\epsilon^{( \alpha )}`$ to a final state with polarization $`\epsilon^{( \alpha^{\prime} )}`$. We have **integrated over the final state wave vector magnitude,** **subject to the delta function** giving energy conservation, but, we have **not integrated over** **final state photon direction** yet, as indicated by the $`\frac{d\sigma}{d \Omega}`$. There is no explicit angular dependence but there is some **hidden in the dot product between initial and final polarization vectors,** **both of which must be transverse** to the direction of propagation. We are ready to compute four different differential cross sections corresponding to two initial polarizations times two final state photon polarizations. Alternatively, we average and/or sum, if we so choose.

In the high energy approximation we have made, there is no dependence on the state of the atoms, so we are **free to choose our coordinate system** any way we want. Set the **z-axis to be along** **the direction of the initial photon** and set the x-axis so that the **scattered photon is in the** **x-z plane** $`( \varphi`$ = 0). The scattered photon is at an angle $`\theta`$ to the initial photon direction and at $`\varphi`$ = 0. A reasonable set of **initial state polarization vectors** is

```math
\hat{\epsilon}^{( 1 )} = \hat{x}
```

```math
\hat{\epsilon}^{( 2 )} = \hat{y}
```

Pick $`\hat{\epsilon}^{( 1 )^{\prime}}`$ to be in the **scattering plane** (x-z) defined as the plane containing both $`\vec{k}`$ and $`\vec{k}^{\prime}`$ and $`\hat{\epsilon}^{( 2 )^{\prime}}`$ to be perpendicular to the scattering plane. $`\hat{\epsilon}^{( 1 )^{\prime}}`$ is then at an angle $`\theta`$ to the x-axis. $`\hat{\epsilon}^{( 2 )^{\prime}}`$ is along the y-axis. We can compute all the dot products.

```math
\hat{\epsilon}^{( 1 )} \cdot \hat{\epsilon}^{( 1 )^{\prime}} = \cos \theta
```

```math
\hat{\epsilon}^{( 1 )} \cdot \hat{\epsilon}^{( 2 )^{\prime}} = 0
```

```math
\hat{\epsilon}^{( 2 )} \cdot \hat{\epsilon}^{( 1 )^{\prime}} = 0
```

```math
\hat{\epsilon}^{( 2 )} \cdot \hat{\epsilon}^{( 2 )^{\prime}} = 1
```

From these, we can compute any cross section we want. For example, **averaging over initial state** **polarization and summing over final is just half the sum of the squares of the above**.

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \frac{1}{2} ( 1 + \cos^{2} \theta )
```

Even if the initial state is unpolarized, the **final state can be polarized**. For example, for $`\theta = \tfrac{\pi}{2}`$, all of the above dot products are zero except $`\hat{\epsilon}^{( 2 )} \cdot \hat{\epsilon}^{( 2 )^{\prime}}`$ = 1. That means only the initial photons polarized along the y direction will scatter and that the scattered photon is **100% polarized** **transverse to the scattering plane** (really just the same polarization as the initial state). The angular distribution could also be used to deduce the polarization of the initial state if a large ensemble of initial state photons were available.

For a **definite initial state polarization** (at an angle $`\varphi`$ to the scattering plane, the component along $`\hat{\epsilon}^{( 1 )}`$ is $`\cos \varphi`$ and along $`\hat{\epsilon}^{( 2 )}`$ is $`\sin \varphi`$. If we don’t observe final state polarization we sum (cos$`\theta \cos \varphi )^{2}`$ + (sin $`\varphi )^{2}`$ and have

```math
\frac{d\sigma}{d \Omega} = \left( \frac{e^{2}}{4 \pi mc^{2}} \right)^{2} \frac{1}{2} ( \cos^{2} \theta \cos^{2} \varphi + \sin^{2} \varphi )
```

For **atoms with more than one electron**, this cross section will grow as $`Z^{4}`$.

<div align="right"><a href="book.pdf#page=471" target="_blank" title="This page in the PDF"><sup>[p. 471]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p472" href="book.pdf#page=472" target="_blank" title="This page in the PDF"><sup>[p. 472]</sup></a>

# <a id="s-34-5-raman-effect"></a>34.5 Raman Effect

The Kramers-Heisenberg formula clearly allows for the **initial and final state to be different**. The atom changes state and the scattered photon energy is not equal to the initial photon energy. This is called the **Raman effect**. Of course, total energy is still conserved. A given initial photon frequency will produce scattered photons with definite frequencies, or lines in the spectrum.

---

[← 33 Quantum Theory of Radiation](33-quantum-theory-of-radiation.md) · [35 Electron Self Energy Corrections →](35-electron-self-energy-corrections.md)
