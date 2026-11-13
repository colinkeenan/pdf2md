[← The Dirac Theory](02-the-dirac-theory.md) · [Field Theory →](04-field-theory.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p22" href="book.pdf#page=28" target="_blank" title="This page in the PDF"><sup>[p. 22]</sup></a>

# Scattering Problems and Born Approximation

## <a id="s-general-discussion"></a>General Discussion

The problem of scattering of a Dirac particle by a potential can be treated exactly by finding the continuum solutions of the Dirac <a id="ix-66">equation</a>. This is a complicated business even for the simplest case of a Coulomb force. It was done by <a id="ix-216">Mott</a>, _Proc. Roy. Soc._ **A135** (1932) 429.

For most purposes in relativistic problems, and always when the scattering is produced by complicated effects involving radiation theory, one uses the <a id="ix-29"><em>Born approximation</em></a>. That is, we treat the scattering only to 1<sup>st</sup> order in the interaction, or only to some definite order in which we are interested.

The formula for scattering from an initial state $`A`$ to a final state $`B`$ lying in a continuum of <a id="ix-118">states</a> is, <a id="ix-245">transition</a> probability per unit time,

<a id="eq-102"></a>

```math
w = \frac{2 \pi}{\hbar} \rho_{E} | V_{BA} |^{2} \qquad \text{(102)}
```

This you ought to know. $`\rho_{E}`$ = density of final states per unit energy interval. $`V_{BA}`$ is the matrix element of the potential $`V`$ for the transition. Here $`V`$ may be anything, and may be itself a second-order or higher order effect obtained by using higher-order perturbation theory.

The difficulties in real calculations usually come from the factors 2 and $`\pi`$ and the correct _normalization_ _of states_. Always I shall normalize the continuum states not in the usual way (one particle per unit volume) which is non-invariant, but instead

<a id="eq-103"></a>

```math
\text{ One particle per volume } \frac{mc^{2}}{| E |} \qquad \text{(103)}
```

where $`| E |`$ is the energy of the particles. Then under a <a id="ix-181">Lorentz</a> transformation the volume of a fixed region transforms like 1$`/ | E |`$ and so the definition stays invariant.

Thus a continuum state given by the spinor $`\psi = u`$exp$`\{ ( i \boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x} - iEt ) / \hbar \}`$ is normalized so that

<a id="eq-104"></a>

```math
u^{*} u = \frac{| E |}{mc^{2}} \qquad \text{(104)}
```

Now if we multiply the Dirac equation for a free particle, (44), by $`\overline{u}`$ on the left, we get $`Eu^{*} \beta u = cu^{*} \beta \boldsymbol{\alpha} \boldsymbol{\cdot}`$ $`\boldsymbol{p} u + mc^{2} u^{*} u`$; its complex conjugate is $`Eu^{*} \beta u = - cu^{*} \beta \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} u + mc^{2} u^{*} u`$ since $`\beta \boldsymbol{\alpha}`$ is anti-<a id="ix-155">Hermitian</a>; then by adding we get

<a id="eq-105"></a>

```math
E \overline{u} u = mc^{2} u^{*} u \qquad \text{(105)}
```

Therefore the normalization becomes

<a id="eq-106"></a>

```math
\left. \begin{aligned} \overline{u} u &= + 1 \text{ for electron states } \\ &= - 1 \text{ for positron states } \end{aligned} \right\} = \epsilon ; \quad \text{ This is the definition of } \epsilon . \qquad \text{(106)}
```

With this normalization the density of states in momentum <a id="ix-214">space</a> is one per volume $`h^{3}`$ of phase space, that is to say

<a id="eq-107"></a>

```math
\rho = \frac{1}{h^{3}} \frac{mc^{2}}{| E |} dp_{1} dp_{2} dp_{3} \qquad \text{(107)}
```

per volume $`dp_{1} dp_{2} dp_{3}`$ of momentum space, for _each_ direction of spin and each sign of charge. Now we have again an invariant differential

<a id="eq-108"></a>

```math
\frac{dp_{1} dp_{2} dp_{3}}{| E |} \qquad \text{(108)}
```

<div align="right"><a href="book.pdf#page=28" target="_blank" title="This page in the PDF"><sup>[p. 22]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p23" href="book.pdf#page=29" target="_blank" title="This page in the PDF"><sup>[p. 23]</sup></a>

## <a id="s-projection-operators"></a>Projection Operators

Usually we are not interested in the spin either of an intermediate or of an initial or final state. Therefore we have to make sums over spin <a id="ix-119">states</a> which are of the form

<a id="eq-109"></a>

```math
S = \sum_{2} ( \overline{s} Ou ) ( \overline{u} Pr ) \qquad \text{(109)}
```

where $`O`$ and $`P`$ are some kind of operators, $`s`$ and $`r`$ some kind of spin states, and the sum is over the two spin states $`u`$ of an <a id="ix-120">electron</a> of <a id="ix-205">momentum</a> $`\boldsymbol{p}`$ and energy $`E`$.

Let us write

<a id="eq-110"></a>

```math
p / = \sum_{\mu} p_{\mu} \gamma_{\mu} , \quad p_{4} = iE/c \qquad \text{(110)}
```

The Dirac <a id="ix-67">equation</a> satisfied by $`u`$ is

<a id="eq-111"></a>

```math
\left( p / - imc \right) u = 0 \qquad \text{(111)}
```

The two spin states with momentum 4-vector $`- p`$ satisfy

<a id="eq-112"></a>

```math
\left( p / + imc \right) u = 0 \qquad \text{(112)}
```

As one can easily show from (48), these 4 states are all orthogonal in the sense that $`( \overline{u}^{\prime} u )`$ = 0 for each pair $`u^{\prime} u`$. Therefore the identity operator may be written in the form

<a id="eq-113"></a>

```math
\mathbb{I} = \sum_{4} ( u \overline{u} ) \epsilon \qquad \text{(113)}
```

summed over all 4 states with $`\epsilon`$ defined as earlier. Hence by (111) and (112) we can write (109) as

<a id="eq-114"></a>

```math
S = \sum_{4} \left( \overline{s} O \frac{p / + imc}{2 imc} \epsilon u \right) ( \overline{u} Pr ) = ( \overline{s} O \Lambda_{+} Pr ) \qquad \text{(114)}
```

by virtue of (113); here the operator

<a id="eq-115"></a>

```math
\Lambda_{+} = \frac{p / + imc}{2 imc} \qquad \text{(115)}
```

is a <a id="ix-250">projection operator</a> for _electrons_ of momentum $`p`$.

In the same way for a sum over the two positron states $`u`$ with momentum $`p`$ energy $`E`$

<a id="eq-116"></a>

```math
S = \sum_{2} ( \overline{s} Ou ) ( \overline{u} Pr ) = ( \overline{s} O \Lambda_{-} Pr ) \qquad \text{(116)}
```

with

<a id="eq-117"></a>

```math
\Lambda_{-} = \frac{p / - imc}{2 imc} \qquad \text{(117)}
```

and we have

<a id="eq-118"></a>

```math
\Lambda_{+} - \Lambda_{-} = \mathbb{I} \qquad \text{(118)}
```

These projection operators are covariant. In <a id="ix-152">Heitler</a> the business is done in a different way which makes them non-covariant and more difficult to handle.

Note that here charge-conjugate wave functions are _not_ used. The positrons of momentum $`p`$ are represented by the electron wave functions $`u`$ of momentum $`- p`$ energy $`- E`$.

<div align="right"><a href="book.pdf#page=29" target="_blank" title="This page in the PDF"><sup>[p. 23]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p24" href="book.pdf#page=30" target="_blank" title="This page in the PDF"><sup>[p. 24]</sup></a>

## <a id="s-calculation-of-spurs"></a>Calculation of Spurs

Suppose we have to calculate an expression such as

```math
\tfrac{1}{2} \sum_{I} \sum_{F} ( \overline{u}_{F} Ou_{I} ) ( \overline{u}_{I} Ou_{F} )
```

summed over electron states only. This gives

```math
\tfrac{1}{2} \sum ( \overline{u}_{F} O \Lambda_{+} O \Lambda_{+} u_{F} ) \epsilon
```

summed over all four spin states $`u_{F}`$. To calculate this, let us consider the general expression

```math
\sum_{u} \epsilon ( \overline{u} Qu )
```

summed over all 4 spin states, where $`Q`$ is any 4 $`\times 4`$ matrix.

Let $`Q`$ have the eigenvectors $`w_{1} ,w_{2} ,w_{3} ,w_{4}`$ with eigenvalues $`\lambda_{1} ,\lambda_{2} ,\lambda_{3} ,\lambda_{4}`$. Then

```math
Q = \sum_{k = 1}^{4} \lambda_{k} w_{k} w_{k}^{*}
```

and $`4 \quad 4`$

```math
\sum_{u} \epsilon ( \overline{u} Qu ) = \sum_{u} \epsilon \sum_{k = 1} \lambda_{k} ( \overline{u} w_{k} ) ( w^{*} u ) = \sum_{k = 1} \lambda_{k} w^{*} \left\{ \sum_{u} \epsilon ( u \overline{u} ) \right\} w_{k}
```

Hence by (113)

```math
\sum_{u} \epsilon ( \overline{u} Qu ) = \sum \lambda
```

Now $`\sum \lambda`$ = sum of diagonal elements of $`Q`$ = Spur $`Q`$. Thus<sup><a id="fnref-10" href="#fn-10">10</a></sup>

```math
\sum_{u} \epsilon ( \overline{u} Ou ) = \operatorname{Sp} Q
```

and this is always easy to calculate.

_Problem 3:_ Given a steady potential $`V`$ a function of position, and a beam of incident particles, electrons. Solve the Schrödinger <a id="ix-262">equation</a> in the <a id="ix-30">Born approximation</a>

a) By stationary perturbation theory<br>
b) By time-dependent perturbation theory.

Show that the results agree, with a <a id="ix-246">transition</a> probability per unit time given by $`w`$ = (2$`\pi / \hbar ) \rho_{E} | V_{BA} |^{2}`$. Evaluate the cross section in the case $`V = - Ze^{2} /r`$, averaging spin over initial state and summing over final state.

c) Repeat the calculation with particles obeying the <a id="ix-164">Klein-Gordon equation</a>, leaving out the $`V^{2}`$ term, by either method. Compare the <a id="ix-99">angular</a> <a id="ix-98">distribution</a> in the two cases.

_Problem 4:_ A nucleus (O<sup>16</sup>) has an even<sup><a id="fnref-11" href="#fn-11">11</a></sup> $`j`$ = 0 ground state and an even $`j`$ = 0 excited state at 6 MeV. Calculate the total rate of emission of pairs, and the angular and <a id="ix-101">momentum</a> distributions.

_Solution:_ Let ∆$`E`$ be the excitation energy, $`\rho_{N}`$ and $`\boldsymbol{j}_{N}`$ the charge and current density operators of the nucleus. Then for the transition we are interested in $`\rho_{N}`$ and $`\boldsymbol{j}_{N}`$ are functions of position $`r`$ with the time-variation of the single matrix element given by exp$`\{- i`$∆$`E/ \hbar \}`$. Also

<a id="eq-119"></a>

```math
\nabla \boldsymbol{\cdot} \boldsymbol{j}_{N} = - \frac{\partial \rho_{H}}{\partial t} = i \frac{\Delta E}{\hbar} \rho_{N} \qquad \text{(119)}
```

<div align="right"><a href="book.pdf#page=30" target="_blank" title="This page in the PDF"><sup>[p. 24]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p25" href="book.pdf#page=31" target="_blank" title="This page in the PDF"><sup>[p. 25]</sup></a>

The electrostatic potential $`V`$ of the nucleus has the matrix element given by

<a id="eq-120"></a>

```math
\nabla^{2} V = - 4 \pi \rho_{N} \qquad \text{(120)}
```

The states being spherically symmetric, $`\rho_{N}`$ is a function of $`r`$ only, and so the general solution of <a id="ix-223">Poisson’s equation</a> simplifies to<sup><a id="fnref-12" href="#fn-12">12</a></sup>

<a id="eq-121"></a>

```math
V ( r ) = - \frac{6 \pi}{r} \int_{0}^{r} r_{1}^{2} \rho_{N} ( r_{1} ) dr_{1} \qquad \text{(121)}
```

Outside the nucleus $`V ( r ) = Ze^{2} /r`$ is constant in time, and so the matrix element of $`V ( r )`$ for this transition is zero. In fact from (119) and (120) we get by integration

<a id="eq-122"></a>

```math
V ( r ) = \frac{\hbar}{i \Delta E} ( - 4 \pi ) ( - r ) j_{No} ( r ) = \frac{4 \pi r \hbar}{i \Delta E} j_{No} ( r ) \qquad \text{(122)}
```

where $`j_{No}`$ is the outward component of the current.

The interaction which creates pairs is then

<a id="eq-123"></a>

```math
I = \int \frac{4 \pi r \hbar}{i \Delta E} j_{No} ( r ) ( - e\psi^{*} \psi ( r ) ) d\tau \qquad \text{(123)}
```

As an approximation consider the <a id="ix-56">de Broglie wavelengths</a> of all pairs long compared with the nuclear size. Then

<a id="eq-124"></a>

```math
I = \psi^{*} \psi ( 0 ) \frac{4 \pi \hbar ei}{\Delta E} \int rj_{No} ( r ) d\tau \qquad \text{(124)}
```

The constant $`\int rj_{No} ( r ) d\tau`$ is not known exactly. Suppose as an order of magnitude estimate that the nucleus of charge $`Ze`$ is uniformly spread over a sphere of radius $`r_{o}`$ in the ground state, and also in the excited state. Since $`\rho_{N}`$ is roughly uniform inside the nucleus we have by integrating (119):

<a id="eq-125"></a>

```math
\begin{aligned} j_{N} &= \frac{i \Delta E}{3 \hbar} \overline{r} \rho_{N} , \quad \text{ and thus } \\ I &= \psi^{*} \psi ( 0 ) \left( \frac{- 4 \pi e}{3} \right) \int r^{2} \rho_{N} ( r ) d\tau = \psi^{*} \psi ( 0 ) \left( \frac{- 4 \pi e}{3} \right) Qe^{- i \Delta Et/ \hbar} \end{aligned} \qquad \text{(125)}
```

$`Q`$ is roughly a measure of the charge-moment of inertia of the nucleus, and is equal to

```math
\tfrac{3}{5} Zer_{o}^{2}
```

Thus

<a id="eq-126"></a>

```math
I = - \frac{4 \pi Ze^{2} r_{o}^{2}}{5} \{ \psi^{*} \psi ( 0 ) \} e^{- i \Delta Et/ \hbar} \qquad \text{(126)}
```

So the problem is just to compute the probabilities of <a id="ix-46">pair</a>-emission with this interaction. Note that real radiation is strictly forbidden in a 0–0 transition, and so these pairs are actually observed in the reaction

<a id="eq-127"></a>

```math
p + F_{1 9} \to O_{1 6}^{*} + \alpha \to O_{1 6} + e^{+} + e^{-} + \alpha \qquad \text{(127)}
```

Is it correct to take for the interaction just

```math
\int V ( r ) ( - e\psi^{*} \psi ) d\tau
```

taking the <a id="ix-39">Coulomb potential</a> of the nuclear charge and ignoring all electrodynamic effects? Yes. Because in general the interaction would be

<a id="eq-128"></a>

```math
\int \left\{ \phi ( - e\psi^{*} \psi ) - \sum_{k} A_{k} ( - e\psi^{*} \alpha_{k} \psi ) \right\} d\tau \qquad \text{(128)}
```

<div align="right"><a href="book.pdf#page=31" target="_blank" title="This page in the PDF"><sup>[p. 25]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p26" href="book.pdf#page=32" target="_blank" title="This page in the PDF"><sup>[p. 26]</sup></a>

where $`\phi , A_{k}`$ are the scalar and vector potentials satisfying the Maxwell <a id="ix-186">equations</a>

```math
\nabla^{2} \phi + \frac{1}{c} \nabla \boldsymbol{\cdot} \frac{\partial \boldsymbol{A}}{\partial t} = - 4 \pi \rho_{N}
```

```math
\nabla^{2} \boldsymbol{A} - \frac{1}{c^{2}} \frac{\partial^{2} \boldsymbol{A}}{\partial t^{2}} -\nabla \left\{ \nabla \boldsymbol{\cdot} \boldsymbol{A} + \frac{1}{c} \frac{\partial \phi}{\partial t} \right\} = - \frac{4 \pi}{c} \boldsymbol{j}_{N}
```

The matrix element of the interaction (128) is unchanged by any <a id="ix-143">gauge</a> <a id="ix-146">transformation</a> of the $`( \boldsymbol{A} ,\phi )`$. Therefore we may take the gauge in which

```math
\nabla \boldsymbol{\cdot} \boldsymbol{A} = 0
```

Incidentally, since $`\phi = V ( r )`$, the second Maxwell equation reduces to

```math
\nabla^{2} \boldsymbol{A} - \frac{1}{c^{2}} \frac{\partial^{2} \boldsymbol{A}}{\partial t^{2}} - = - \frac{4 \pi}{c} \boldsymbol{j}_{N}
```

Now, since there is no free radiation present, also $`\nabla \times \boldsymbol{A}`$ = 0, and hence $`\boldsymbol{A}`$ = 0, in this gauge, and therefore we can indeed ignore all electrodynamic effects.

Let us calculate then the <a id="ix-241">probability</a> of <a id="ix-47">pair</a> emission with the interaction (126). A typical final state has an electron of momentum $`p_{1}`$ and a <a id="ix-226">positron</a> of momentum $`p_{2}`$, with energies $`E_{1} , E_{2}`$ and spins $`u_{1} , u_{2}`$ respectively. For the <a id="ix-44">creation</a> of this pair the matrix element of $`I`$ is

<a id="eq-128a"></a>

```math
I = - C \overline{u}_{1} \beta u_{2} , \quad C = \frac{4 \pi Ze^{2} r_{o}^{2}}{5} \qquad \text{(128a)}
```

The density of final states is by (107)

<a id="eq-129"></a>

```math
\frac{1}{( 2 \pi \hbar )^{6}} \frac{m^{2} c^{4}}{E_{1} E_{2}} p_{1}^{2} dp_{1} d\omega_{1} p_{2}^{2} dp_{2} d\omega_{2} \qquad \text{(129)}
```

where $`d\omega_{1}`$ and $`d\omega_{2}`$ are the <a id="ix-14">solid</a> angles for $`p_{1}`$ and $`p_{2}`$. The creation probability per unit time is thus by (102)

<a id="eq-130"></a>

```math
w = \frac{2 \pi}{\hbar} \frac{\rho_{E}}{dE} | I |^{2} = \frac{2 \pi}{\hbar} \frac{dp_{1} dp_{2}}{d ( E_{1} + E_{2} )} C^{2} \frac{1}{( 2 \pi \hbar )^{6}} \frac{m^{2} c^{4} p_{1}^{2} p_{2}^{2} d\omega_{1} d\omega_{2}}{E_{1} E_{2}} \sum_{u_{1} ,u_{2}} | u_{1} \beta u_{2} |^{2} \qquad \text{(130)}
```

Now fixing $`p_{1}`$,

```math
\frac{dp_{2}}{d ( E_{1} + E_{2} )} = \frac{dp_{2}}{dE_{2}} = \frac{E_{2}}{c^{2} p_{2}}
```

and

```math
\sum_{u_{1} ,u 2} | u_{1} \beta u_{2} |^{2} = \sum_{u_{1} ,u 2} ( \overline{u}_{1} \beta u_{2} ) ( \overline{u}_{2} \beta u_{1} ) = \operatorname{Spur} \left\{ \beta \frac{p /_{2} - imc}{2 imc} \beta \frac{p /_{1} + imc}{2 imc} \right\}
```

```math
= - 1 + \frac{\boldsymbol{p}_{1} \boldsymbol{\cdot} \boldsymbol{p}_{2}}{m^{2} c^{2}} + \frac{E_{1} E_{2}}{m^{2} c^{4}} = \frac{E_{1} E_{2} - m^{2} c^{4} + c^{2} p_{1} p_{2} \cos \theta}{m^{2} c^{4}}
```

where $`\theta`$ is the <a id="ix-10">angle</a> between the pair. Then writing in (130)

```math
dE_{1} = dp_{1} \frac{c^{2} p_{1}}{E_{1}} , \quad d\omega_{1} = 4 \pi , \quad d\omega_{2} = 2 \pi \sin \theta d\theta
```

we obtain the <a id="ix-239">differential</a> probability<sup><a id="fnref-13" href="#fn-13">13</a></sup> in $`E_{1}`$ and $`\theta`$

<a id="eq-131"></a>

```math
w_{o} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi c^{4} \hbar^{7}} p_{1} p_{2} dE_{1} \left( E_{1} E_{2} - m^{2} c^{4} + c^{2} p_{1} p_{2} \cos \theta \right) \sin \theta d\theta \qquad \text{(131)}
```

<div align="right"><a href="book.pdf#page=32" target="_blank" title="This page in the PDF"><sup>[p. 26]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p27" href="book.pdf#page=33" target="_blank" title="This page in the PDF"><sup>[p. 27]</sup></a>

Since

```math
\Delta E = 6 \text{ MeV } = 1 2 mc^{2}
```

we can to a good approximation treat all particles as extreme relativistic. Thus

<a id="eq-132"></a>

```math
w_{o} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi c^{6} \hbar^{7}} E_{1}^{2} E_{2}^{2} dE_{1} ( 1 + \cos \theta ) \sin \theta d\theta \qquad \text{(132)}
```

So the pairs have an <a id="ix-100">angular</a> distribution concentrated in the _same_ hemisphere, and predominantly _equal_ energies. Then, since

```math
\int_{0}^{\pi} ( 1 + \cos \theta ) \sin \theta d\theta = 2 , \quad \text{ and } \int_{0}^{\Delta E} E_{1}^{2} E_{2}^{2} dE_{1} = \int_{0}^{\Delta E} E_{1}^{2} ( E_{1} + \Delta E )^{2} dE_{1} = \frac{1}{1 5} ( \Delta E )^{5}
```

the total <a id="ix-45">creation</a> <a id="ix-232">probability</a> per unit time is

<a id="eq-133"></a>

```math
w_{T} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi \hbar^{7} c^{6}} \frac{1}{1 5} ( \Delta E )^{5} \qquad \text{(133)}
```

Numerically

```math
\frac{Ze^{2}}{\hbar c} \approx \frac{1}{1 7} \quad \text{ and } \quad \frac{\Delta Er_{o}}{\hbar c} \approx \frac{1}{1 0} \quad \text{ since } r_{o} = 4 \times 1 0^{- 1 3} \text{ cm }
```

Hence the lifetime will be

<a id="eq-134"></a>

```math
\tau = 1 5 \times 2 5 \pi \times 1 0^{5} \times 1 7^{2} \times \frac{1}{4} \times \frac{r_{o}}{c} = 1 0^{1 0} \frac{r_{o}}{c} \approx 1 0^{- 1 3} \sec . \qquad \text{(134)}
```

## <a id="s-scattering-of-two-electrons-in-born-approximation-the-m-ller-formula"></a>Scattering of Two Electrons in <a id="ix-31">Born Approximation</a>.The <a id="ix-260">Møller</a> Formula.

We calculate now the transition scattering matrix element $`M`$ between an initial state $`A`$ consisting of 2 electrons with momenta $`p_{1} ,p_{2}`$ and spin states $`u_{1} ,u_{2}`$ and a final state $`B`$ consisting of two electrons with momenta $`p_{1}^{\prime} ,p_{2}^{\prime}`$ and spin states $`u_{1}^{\prime} ,u_{2}^{\prime}`$. Thus $`M`$ gives the probability amplitude for arriving in state $`B`$ after a long time when the system is known to be in state $`A`$ to begin with. Hence $`M`$ itself should be an _invariant_ relativistically.

We treat the interaction in Born approximation i.e. consider the particles to go directly from the free-particle state $`A`$ to the free-particle state $`B`$ by applying the interaction operator once to state $`A`$. For electrons at reasonably high or relativistic velocities this will be a very good approximation $`( e^{2} / \hbar v \ll`$ 1). Also we treat the electromagnetic interaction classically, just as in the O<sup>16</sup> problem, taking the field produced by particle 1 according to classical Maxwell <a id="ix-187">equations</a> to act directly on particle 2. This ignores the fact that the field consists of quanta. We shall see later, after we have developed the quantum field theories, that this introduces _no error_ so long as we are in the Born approximation.

For the field produced by particle 1 in a transition from the state $`p_{1} ,u_{1}`$ to $`p_{1}^{\prime} ,u_{1}^{\prime}`$ we have the matrix elements $`\phi_{( 1 )} , \boldsymbol{A}_{( 1 )}`$ say. We use now not the <a id="ix-144">gauge</a> in which $`\nabla \boldsymbol{\cdot} \boldsymbol{A}`$ = 0, but the covariant gauge in which<sup><a id="fnref-14" href="#fn-14">14</a></sup>

<a id="eq-135"></a>

```math
\sum_{\mu} \frac{\partial A_{\mu}}{\partial x_{\mu}} = 0 \quad A_{4} = i\phi \qquad \text{(135)}
```

So using covariant notations we have in this gauge

<a id="eq-136"></a>

```math
\sum_{\nu} \frac{\partial^{2}}{\partial x_{\nu}^{2}} A_{\mu ( 1 )} = + 4 \pi es_{\mu ( 1 )} \quad ( \text{ charge is } - e ) \qquad \text{(136)}
```

<a id="eq-137"></a>

```math
s_{\mu ( 1 )} = i ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \exp \left\{ \sum_{\nu} \frac{i}{\hbar} ( p_{1 \nu} - p_{1 \nu}^{\prime} ) x_{\nu} \right\} \qquad \text{(137)}
```

<div align="right"><a href="book.pdf#page=33" target="_blank" title="This page in the PDF"><sup>[p. 27]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p28" href="book.pdf#page=34" target="_blank" title="This page in the PDF"><sup>[p. 28]</sup></a>

whence

<a id="eq-138"></a>

```math
A_{\mu ( 1 )} = - 4 \pi ie \hbar^{2} \left[ \frac{( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \exp \left\{ \sum_{\nu} \frac{i}{\hbar} ( p_{1 \nu} - p_{1 \nu}^{\prime} ) x_{\nu} \right\}}{\sum_{\lambda} ( p_{1 \lambda} - p_{1 \lambda}^{\prime} )^{2}} \right] \qquad \text{(138)}
```

where

<a id="eq-139"></a>

```math
\sum_{\nu} ( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2} = | \boldsymbol{p}_{1} - \boldsymbol{p}_{1}^{\prime} |^{2} - \frac{1}{c^{2}} ( E_{1} - E_{1}^{\prime} )^{2} \qquad \text{(139)}
```

The effect of the field (138) on particle 2 is given by the interaction term in the Dirac <a id="ix-68">equation</a> for particle 2

<a id="eq-140"></a>

```math
- e\phi + e \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{A} = ie\beta \sum_{\mu} \gamma_{\mu} A_{\mu} \qquad \text{(140)}
```

This gives for particle 2 for the transition from state $`p_{2} ,u_{2}`$ to $`p_{2}^{\prime} ,u_{2}^{\prime}`$ a transition matrix element

<a id="eq-141"></a>

```math
\int d\tau \psi_{2}^{\prime *} \left( ie\beta \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} \qquad \text{(141)}
```

a 3-dimensional integral over space at the time $`t`$ say. For the total transition matrix element $`M`$ by first order perturbation method

<a id="eq-142"></a>

```math
M = - \frac{i}{\hbar} \int_{-\infty}^{\infty} dt \int d\tau \overline{\psi}_{2}^{\prime} \left( ie \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} = - \frac{i}{\hbar c} \int d^{4} x \overline{\psi}_{2}^{\prime} \left( ie \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} \qquad \text{(142)}
```

where the 4-fold integral is $`dx_{1} dx_{2} dx_{3} dx_{0} ,x_{0} = ct`$. Putting in the values of $`A_{\mu ( 1 )} ,\psi_{2}^{\prime}`$ and $`\psi_{2}`$, we get

<a id="eq-143"></a>

```math
\begin{aligned} M &= - \frac{4 \pi e^{2} \hbar i}{c} \sum_{\mu} ( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \frac{1}{\sum_{\nu} ( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} \int d^{4} x \exp \left\{ \sum_{\lambda} \frac{i}{\hbar} ( p_{1 \lambda} - p_{1 \lambda}^{\prime} + p_{2 \lambda} - p_{2 \lambda}^{\prime} ) x_{\lambda} \right\} \\ &= - \frac{4 \pi e^{2} \hbar i}{c} \sum_{\mu ,\nu} \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} )}{( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \end{aligned} \qquad \text{(143)}
```

where $`\delta^{4} ( x ) = \prod_{k = 1}^{4} \delta ( x_{k} )`$.

There is also the exchange process in which the particle $`p_{1} ,u_{1}`$ goes to $`p_{2}^{\prime} ,u_{2}^{\prime}`$ and _vice-versa_. This gives a contribution to $`M`$ with a minus sign since the wave function ought to be taken antisymmetric between the two particles. Hence the final result is

<a id="eq-144"></a>

```math
M = - \frac{4 \pi e^{2} \hbar i}{c} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \sum_{\mu ,\nu} \left\{ \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} )}{( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} - \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{1} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{2} )}{( p_{1 \nu} - p_{2 \nu}^{\prime} )^{2}} \right\} \qquad \text{(144)}
```

This covariant formula is elegant and easy to arrive at. The question now is, how does one go from such a formula to a <a id="ix-49">cross-section</a>?

Generally, suppose in such a 2-particle collision process the transition matrix is

<a id="eq-145"></a>

```math
M = K ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \qquad \text{(145)}
```

Then what will be the cross-section in terms of $`K`$? We do this calculation once here so that later we can stop when we have found formulae for $`M`$ of the type (145) which come for example conveniently in this form out of radiation theory.

<div align="right"><a href="book.pdf#page=34" target="_blank" title="This page in the PDF"><sup>[p. 28]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p29" href="book.pdf#page=35" target="_blank" title="This page in the PDF"><sup>[p. 29]</sup></a>

## <a id="s-relation-of-cross-sections-to-transition-amplitudes"></a>Relation of Cross-sections to <a id="ix-247">Transition</a> <a id="ix-5">Amplitudes</a>

Let $`w`$ be the transition probability per unit volume and per unit time. This is related to the transition probability for a single final state, which is

<a id="eq-146"></a>

```math
w_{s} = c | K |^{2} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \qquad \text{(146)}
```

since in $`| M |^{2}`$ one of the two (2$`\pi \hbar )`$<sup>4</sup>$`\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) /c`$ factors represents merely the volume of space-time in which the interaction can occur. The number of final states is by (107)

<a id="eq-147"></a>

```math
\frac{1}{( 2 \pi \hbar )^{6}} \frac{mc^{2}}{| E_{1}^{\prime} |} \frac{mc^{2}}{| E_{2}^{\prime} |} dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} dp_{2 1}^{\prime} dp_{2 2}^{\prime} dp_{2 3}^{\prime} \qquad \text{(147)}
```

Multiplying (146) by (147) gives the total transition probability

<a id="eq-148"></a>

```math
w = | K |^{2} \frac{1}{( 2 \pi \hbar )^{2}} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} c\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} dp_{2 1}^{\prime} dp_{2 2}^{\prime} dp_{2 3}^{\prime} \qquad \text{(148)}
```

As $`\delta ( ax ) = \frac{1}{a} \delta ( x )`$, we have

```math
\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) = \delta^{3} ( \boldsymbol{p}_{1} + \boldsymbol{p}_{2} - \boldsymbol{p}_{1}^{\prime} - \boldsymbol{p}_{2}^{\prime} ) c\delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} )
```

and the integration over $`dp_{2}`$ gives then by the momentum <a id="ix-206">conservation</a>

<a id="eq-148a"></a>

```math
w = | K |^{2} \frac{c^{2}}{( 2 \pi \hbar )^{2}} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} \delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} ) dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} \qquad \text{(148a)}
```

Furthermore,

```math
\text{ if } f ( a ) = 0 , \text{ we have } f ( x ) = f ( a ) + f^{\prime} ( a ) ( x - a ) = f^{\prime} ( a ) ( x - a )
```

and thus

```math
\delta ( f ( x ) ) = \delta \{ f^{\prime} ( a ) ( x - a ) \} = \frac{\delta ( x - a )}{f^{\prime} ( a )}
```

Applying this to (148a) with $`f ( x ) = f ( p_{1 3}^{\prime} ) = E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime}`$ and $`a = ( p_{1 3}^{\prime} )_{c}`$ = the value of $`p_{1 3}^{\prime}`$ giving momentum and energy conservation, we get

```math
\delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} ) = \frac{1}{d ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} )} dp_{1 3}^{\prime} \delta \{ p_{1 3}^{\prime} - ( p_{1 3}^{\prime} )_{c} \}
```

Hence we finally obtain

```math
w = | K |^{2} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} \frac{c^{2}}{( 2 \pi \hbar )^{2}} \frac{dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime}}{d ( E_{1}^{\prime} + E_{2}^{\prime} )}
```

Choose a Lorentz-<a id="ix-177">system</a> in which $`p_{1}`$ and $`p_{2}`$ are both along the $`x_{3}`$-direction and take $`p_{1 1}^{\prime}`$ and $`p_{1 2}^{\prime}`$ as the variables over which the transition probability is taken. This is necessary for relativistic invariance. Then $`p_{1 1}^{\prime}`$ and $`p_{1 2}^{\prime}`$ being fixed and having from the momentum conservation $`p_{1 3}^{\prime} = - p_{2 3}^{\prime}`$ we get

<a id="eq-149"></a>

```math
\frac{d ( E_{1}^{\prime} + E_{1}^{\prime} )}{dp_{1 3}^{\prime}} = \left| \frac{dE_{1}^{\prime}}{dp_{1 3}^{\prime}} - \frac{dE_{2}^{\prime}}{dp_{2 3}^{\prime}} \right| = c^{2} \frac{| E_{2}^{\prime} p_{1 3}^{\prime} - E_{1}^{\prime} p_{2 3}^{\prime} |}{E_{1}^{\prime} E_{2}^{\prime}} \qquad \text{(149)}
```

Then the cross-section $`\sigma`$ is defined in this system by

<a id="eq-150"></a>

```math
\sigma = \frac{wV_{1} V_{2}}{| \boldsymbol{v}_{1} - \boldsymbol{v}_{2} |} \qquad \text{(150)}
```

<div align="right"><a href="book.pdf#page=35" target="_blank" title="This page in the PDF"><sup>[p. 29]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p30" href="book.pdf#page=36" target="_blank" title="This page in the PDF"><sup>[p. 30]</sup></a>

where $`V_{1}`$ is the normalization volume for particle 1, and $`v_{1}`$ its velocity. In fact by (103)

<a id="eq-151"></a>

```math
V_{1} = \frac{mc^{2}}{E_{1}} \quad V_{2} = \frac{mc^{2}}{E_{2}} \quad ( \boldsymbol{v}_{1} - \boldsymbol{v}_{2} ) = \frac{c^{2} \boldsymbol{p}_{1}}{E_{1}} - \frac{c^{2} \boldsymbol{p}_{2}}{E_{2}} \qquad \text{(151)}
```

Hence the cross-section becomes

<a id="eq-152"></a>

```math
\sigma = \frac{w \left( mc^{2} \right)^{2}}{c^{2} | \boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1} |} = | K |^{2} \frac{\left( mc^{2} \right)^{4}}{c^{2} | E_{2} p_{1 3} - E_{1} p_{2 3} || E_{2}^{\prime} p_{1 3}^{\prime} - E_{1}^{\prime} p_{2 3}^{\prime} |} \frac{1}{( 2 \pi \hbar )^{2}} dp_{1 1}^{\prime} dp_{1 2}^{\prime} \qquad \text{(152)}
```

It is worth noting that the factor $`\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}`$ is invariant under Lorentz <a id="ix-182">transformations</a> leaving the $`x_{1}`$ and $`x_{2}`$ components unchanged (e.g.boosts parallel to the $`x_{3}`$ axis.)<sup><a id="fnref-15" href="#fn-15">15</a></sup> To prove this, we have to show that $`p_{1 3} E_{2} - p_{2 3} E_{1} = \tilde{p}_{1 3} \tilde{E}_{2} - \tilde{p}_{2 3} \tilde{E}_{1}`$ (where ˜ denotes the quantities after the Lorentz transformation) because we have chosen a Lorentz system in which the direction of the momentum vector is the $`x_{3}`$ axis. Then

```math
\tilde{E} = E \cosh \theta - cp \sinh \theta
```

```math
\tilde{p} = p \cosh \theta - \frac{E}{c} \sinh \theta
```

Since $`E^{2} = p^{2} c^{2} + m^{2} c^{4}`$, we can write

```math
E = mc^{2} \cosh \varphi \quad pc = mc^{2} \sinh \varphi , \quad \text{ which makes }
```

```math
\tilde{E} = mc^{2} \cosh ( \varphi - \theta ) \quad \tilde{p} c = mc^{2} \sinh ( \varphi - \theta ) \quad \text{ and thus }
```

```math
\tilde{E}_{2} \tilde{p}_{1 3} - \tilde{E}_{1} \tilde{p}_{2 3} = m^{2} c^{3} \{ \cosh ( \varphi_{2} - \theta ) \sinh ( \varphi_{1} - \theta ) - \cosh ( \varphi_{1} - \theta ) \sinh ( \varphi_{2} - \theta ) \}
```

```math
= m^{2} c^{3} \sinh ( \varphi_{1} - \varphi_{2} )
```

independently of $`\theta`$. Hence we see that $`\sigma`$ is invariant under Lorentz transformations parallel to the $`x_{3}`$ axis.

## <a id="s-results-for-m-ller-scattering"></a>Results for <a id="ix-201">Møller </a><a id="ix-13">Scattering</a>

One electron initially at rest, the other initially with energy $`E = \gamma mc^{2}`$;

```math
\gamma = \frac{1}{\sqrt{1 - ( v/c )^{2}}}
```

scattering angle = $`\theta`$ in the lab system

$`= \theta^{*}`$ in the center-of-mass system

Then the <a id="ix-48">differential</a> cross-section is (<a id="ix-217">Mott</a> and <a id="ix-183">Massey</a>, _Theory of Atomic Collisions_, 2<sup>nd</sup> ed., p. 368)

<a id="eq-153"></a>

```math
2 \pi \sigma ( \theta ) d\theta = 4 \pi \left( \frac{e^{2}}{mv^{2}} \right)^{2} \left( \frac{\gamma + 1}{\gamma^{2}} \right) dx \left\{ \frac{4}{( 1 - x^{2} )^{2}} - \frac{3}{1 - x^{2}} + \left( \frac{\gamma - 1}{2 \gamma} \right)^{2} \left( 1 + \frac{4}{1 - x^{2}} \right) \right\} \qquad \text{(153)}
```

with

```math
x = \cos \theta^{*} = \frac{2 - ( \gamma + 3 ) \sin^{2} \theta}{2 + ( \gamma - 1 ) \sin^{2} \theta}
```

Without spin you get simply

```math
4 \pi \left( \frac{e^{2}}{mv^{2}} \right)^{2} \left( \frac{\gamma + 1}{\gamma^{2}} \right) dx \left\{ \frac{4}{( 1 - x^{2} )^{2}} - \frac{3}{1 - x^{2}} \right\}
```

Effect of spin is a measurable _increase_ of scattering over the Mott formula. Effect of exchange is roughly the 1 $`- 3 x^{2}`$ term. Positron-electron scattering is very similar. Only the exchange effect is different because of <a id="ix-20">annihilation</a> possibility.

<div align="right"><a href="book.pdf#page=36" target="_blank" title="This page in the PDF"><sup>[p. 30]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p31" href="book.pdf#page=37" target="_blank" title="This page in the PDF"><sup>[p. 31]</sup></a>

## <a id="s-note-on-the-treatment-of-exchange-effects"></a>Note on the Treatment of Exchange Effects

The correctly normalized initial and final states in this problem are

<a id="eq-154"></a>

```math
\frac{1}{\sqrt{2}} \frac{1}{\sqrt{2}} \{ \psi_{1} ( 1 ) \psi_{2} ( 2 ) - \psi_{1} ( 2 ) \psi_{2} ( 1 ) \} \{ \psi_{1}^{\prime} ( 1 ) \psi_{2}^{\prime} ( 2 ) - \psi_{1}^{\prime} ( 2 ) \psi_{2}^{\prime} ( 1 ) \} \qquad \text{(154)}
```

where $`\psi_{2}`$(1) means the particle 2 in the state 1, and so on. With these states the matrix element $`M`$ is exactly as we have calculated it including the exchange term.

The number of possible final states is only one half of the number of states of two distinguishable particles. But this does not bring a factor $`\frac{1}{2}`$ into the differential cross-section, because the density of antisymmetrical states, in which one of the two particles has a momentum in a particular range $`dp_{1} dp_{2} dp_{3}`$ is exactly the same as the density of states of 2 distinguishable particles in which the particle labelled 1 lies in the given range. Hence the general rule: the differential cross-section does _not_ have a factor $`\frac{1}{2}`$, the total cross section _does_ because each final state may only be counted once in integrating over the <a id="ix-11">angles</a>.

## <a id="s-relativistic-treatment-of-several-particles"></a>Relativistic Treatment of Several Particles

The Møller treatment of the interaction of two electrons succeeds because the <a id="ix-188">field</a> of particle 1 is calculated for all time without taking any account of the effect of particle 2 on particle 1. How can one do a better calculation taking such reactions into account? Clearly we must construct an <a id="ix-69">equation</a> of motion which follows the motions of both particles continuously in time and keeps them in step with each other. So we must have a <a id="ix-97">Dirac</a> equation for 2 electrons, taking exact account of their interaction by including in the equation the behaviour of the Maxwell field too.

This kind of 2-particle Dirac equation is no longer relativistically invariant, if we give each particle a separate position in space but all the same _time_. To avoid this Dirac constructed the _many-time_ theory in which each electron has its own private time coordinate, and satisfies its private Dirac equation. This theory is all right in principle. But it becomes hopelessly complicated when pairs are created and you have equations with new time-coordinates suddenly appearing and disappearing. In fact the whole program of quantizing the electron theory as a theory of discrete particles each with its private time becomes nonsense when you are dealing with an infinite “sea” or an indefinite number of particles. So we have come to the end of what we can do with the relativistic quantum theory of particles.

Where did the theory go wrong? Obviously a lot of the troubles of arose from the fact that a particle was always described by an operator $`r`$ representing its position at the time $`t , t`$ being a number and not an operator. This made the interpretation of the formalism _essentially_ non-relativistic even when the equations were formally invariant. In equations like the K. G. and the Dirac equation, the space and time coordinates appear symmetrically. Thus we are led to the following new view-point.

Relativistic quantum theory is the study of quantities $`\psi`$ which are functions of four coordinates $`x_{1} , x_{2}`$, $`x_{3} , x_{0}`$, all the coordinates being c-numbers and only the expressions containing $`\psi`$ being operators describing the dynamical system.

The dynamical system is specified by the quantity $`\psi`$ existing at all points of space-time, and so consists of a system of fields. Relativistic quantum theory is necessarily a field theory.

The process of reinterpreting a one-particle wave-function like the Dirac $`\psi`$ as a quantized field operator is called <a id="ix-275"><em>Second Quantization</em></a>.

---

## Notes

<a id="fn-10"></a>**10.** The _spur_ of a matrix $`A`$ is the sum of its diagonal elements, and denoted Spur $`A`$ or Sp $`A`$. This term is also, and more commonly, known as the _trace_ of a matrix, denoted Tr $`A`$. [↩](#fnref-10)

<a id="fn-11"></a>**11.** “ever” was written for “even” [↩](#fnref-11)

<a id="fn-12"></a>**12.** This equation differs from the form given in the first edition by a factor of $`- \tfrac{6}{\pi}`$, but this form also seems incorrect; perhaps the correct relation is [↩](#fnref-12)

```math
V ( r ) = - 4 \pi \int_{0}^{r} \frac{1}{r_{1}^{2}} dr_{1} \int_{0}^{r_{1}} r_{2}^{2} \rho ( r_{2} ) dr_{2}
```

<a id="fn-13"></a>**13.** In the second edition, the factor “$`dE_{1}`$” was missing [↩](#fnref-13)

<a id="fn-14"></a>**14.** In the literature, the gauge condition $`\nabla \boldsymbol{\cdot} \boldsymbol{A}`$ = 0 is now called “Coulomb gauge”; the choice of the gauge condition $`\partial_{\mu} A^{\mu}`$ = 0 (using the Einstein summation convention) is called “Lorentz gauge”. (See also Eq. (588).) In the first edition, Dyson uses Einstein’s convention; in the second edition, Moravcsik does not. See also the parenthetical remark following Eq. (234a). [↩](#fnref-14)

<a id="fn-15"></a>**15.** Rewritten. In v.1, Dyson writes “The factor $`\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}`$ is invariant for a Lorentz transformation parallel to the 3 axis.” In v.2, Moravcsik writes “It is worth noting that the factor $`\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}`$ is invariant under Lorentz transformations leaving the $`x_{1}`$ and $`x_{2}`$ components unchanged.” [↩](#fnref-15)

---

[← The Dirac Theory](02-the-dirac-theory.md) · [Field Theory →](04-field-theory.md)
