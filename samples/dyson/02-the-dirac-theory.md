# The Dirac Theory

<a id="p4"><sup>[p. 4]</sup></a>

## <a id="s-the-form-of-the-dirac-equation"></a>The Form of the <a id="ix-85">Dirac</a> <a id="ix-49">Equation</a>

Historically before the RQ theory came the one-particle theory of Dirac. This was so successful in dealing with the electron, that it was for many years the only respectable RQ theory in existence. And its difficulties are a lot less immediate than the difficulties of the one-particle KG theory.

Dirac said, suppose the particle can exist in several distinct states with the same <a id="ix-199">momentum</a> (different orientations of spin.) Then the wave-function $\psi$ satisfying (6) must have several _components_; it is not a scalar but a set of numbers each giving the prob. <a id="ix-11">amplitude</a> to find the particle at a given place and in a given substate. So we write for $\psi$ a column matrix

$$
\psi = \begin{bmatrix} \psi_{1} \\ \psi_{2} \\ \cdot \\ \cdot \\ \cdot \end{bmatrix} \quad \text{ for the components } \psi_{\alpha} ; \quad \alpha = 1 , 2 ,\dots
$$

Dirac assumed that the probability <a id="ix-184">density</a> at any point is still given by

$$
\rho = \sum_{\alpha} \psi_{\alpha}^{*} \psi_{\alpha} \tag{8}
$$

which we write

$$
\rho = \psi^{*} \psi
$$

as in the NR theory. Here $\psi^{*}$ is a _row_ matrix

$$
[ \psi_{1}^{*} ,\psi_{2}^{*} ,\dots ]
$$

We must have (3) still satisfied. So $\psi$ must satisfy a wave-equation of _First Order_ in $t$. But since the equations are relativistic, the equation has to be also of 1<sup>st</sup> order in $xyz$. Thus the most general possible wave-equation is

$$
\frac{1}{c} \frac{\partial \psi}{\partial t} + \sum_{1}^{3} \alpha^{k} \frac{\partial \psi}{\partial x_{k}} + i \frac{mc}{\hbar} \beta \psi = 0 \tag{9}
$$

where $x_{1} x_{2} x_{3}$ are written for $xyz$ and $\alpha^{1} \alpha^{2} \alpha^{3} \beta$ are square matrices whose elements are numbers. The <a id="ix-118">conjugate</a> of (9) gives

$$
\frac{1}{c} \frac{\partial \psi^{*}}{\partial t} + \sum_{1}^{3} \frac{\partial \psi^{*}}{\partial x_{k}} \alpha^{k *} - i \frac{mc}{\hbar} \psi^{*} \beta^{*} = 0 \tag{10}
$$

where $\alpha^{k *}$ and $\beta^{*}$ are <a id="ix-114">Hermitian</a> conjugates.

Now to get (3) out of (8), (9) and (10) we must have $\alpha^{k *} = \alpha^{k} , \beta^{*} = \beta$ so $\alpha^{k}$ and $\beta$ are _Hermitian_; and

$$
j_{k} = c \left( \psi^{*} \alpha^{k} \psi \right) \tag{11}
$$

<a id="p5"><sup>[p. 5]</sup></a>

Next what more do we want from equation (9)? Two things. (A) it must be consistent with the 2<sup>nd</sup> order equation (6) we started from; (B) the whole theory must be <a id="ix-157">Lorentz</a> invariant.

First consider (A). If (9) is consistent with (6) it must be possible to get exactly (6) by multiplying (9) by the operator

$$
\frac{1}{c} \frac{\partial}{\partial t} - \sum_{1}^{3} \alpha^{\ell} \frac{\partial}{\partial x_{\ell}} - i \frac{mc}{\hbar} \beta \tag{12}
$$

chosen so that the terms with mixed derivatives $\frac{\partial}{\partial t} \frac{\partial}{\partial x_{k}}$ and $\frac{\partial}{\partial t}$ cancel. This gives

$$
\begin{aligned} \frac{1}{c^{2}} \frac{\partial^{2} \psi}{\partial t^{2}} &= \sum \sum_{k \neq \ell} \frac{1}{2} \left( \alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} \right) \frac{\partial^{2} \psi}{\partial x_{k} \partial x_{\ell}} + \sum_{k} \alpha_{k}^{2} \frac{\partial^{2} \psi}{\partial x_{k}^{2}} \\ & \quad - \frac{m^{2} c^{2}}{\hbar^{2}} \beta^{2} \psi + i \frac{mc}{\hbar} \sum_{k} \left( \alpha^{k} \beta + \beta \alpha^{k} \right) \frac{\partial \psi}{\partial x_{k}} \end{aligned}
$$

This agrees with (6) if and only if

$$
\begin{aligned} \alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} &= 0 \quad k \neq \ell \\ \alpha^{k} \beta + \beta \alpha^{k} &= 0 \\ \alpha^{k 2} &= \beta^{2} = \mathbb{I} , ( \text{ identity matrix } ) \end{aligned} \tag{13}
$$

Thus we could not possibly _factorize_ the 2<sup>nd</sup> order equation into two first-order operators involving ordinary numbers. But we can do it with <a id="ix-69"><em>matrices</em></a>.

Consider the Pauli spin matrices

$$
\sigma_{1} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \sigma_{2} = \begin{pmatrix} 0 & - i \\ i & 0 \end{pmatrix} \quad \sigma_{3} = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix} \tag{14}
$$

you are familiar with. They satisfy

$$
\sigma_{k} \sigma_{\ell} + \sigma_{\ell} \sigma_{k} = 2 \delta_{\ell k}
$$

But we cannot make 4 matrices of this type all anti-commuting. They must be _at least_ 4 $\times$ 4.

One possible set of $\alpha^{k}$ and $\beta$ is

$$
\alpha^{k} = \begin{pmatrix} 0 & \sigma_{k} \\ \sigma_{k} & 0 \end{pmatrix} \quad \beta = \begin{pmatrix} \begin{matrix} 1 & 0 \\ 0 & 1 \end{matrix} & 0 \\ 0 & \begin{matrix} - 1 & 0 \\ 0 & - 1 \end{matrix} \end{pmatrix} \tag{15}
$$

In particular

$$
\alpha^{1} = \begin{pmatrix} 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0 \\ 0 & 1 & 0 & 0 \\ 1 & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{2} = \begin{pmatrix} 0 & 0 & 0 & - i \\ 0 & 0 & i & 0 \\ 0 & - i & 0 & 0 \\ i & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{3} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & - 1 \\ 1 & 0 & 0 & 0 \\ 0 & - 1 & 0 & 0 \end{pmatrix}
$$

These are hermitian as required. Of course if $\alpha^{k}$ and $\beta$ are any set satisfying (13) then $S\alpha^{k} S^{- 1}$ and $S\beta S^{- 1}$ are another set, where $S$ is any _unitary_ matrix $SS^{*}$ = 1. And conversely it can be proved that every possible 4 $\times$ 4 matrices $\alpha^{k}$ and $\beta$ are of this form with _some_ such matrix $S$. We do not prove this here.

The Dirac equation is thus a set of 4 simultaneous linear partial differential<sup><a id="fnref-4" href="#fn-4">4</a></sup> equations in the four functions $\psi_{\alpha}$.

<a id="p6"><sup>[p. 6]</sup></a>

## <a id="s-lorentz-invariance-of-the-dirac-equation"></a><a id="ix-165">Lorentz</a> Invariance of the Dirac Equation

What does this mean? Consider a general Lorentz transformation: If $x_{\mu}^{\prime}$ are the new coordinates:

$$
x_{\mu}^{\prime} = \sum_{\nu = 0}^{3} a_{\mu \nu} x_{\nu} \quad ( x_{o} = ct ) \tag{16}
$$

In the new coordinate <a id="ix-158">system</a> the wave-function will be $\psi^{\prime}$. Clearly we do not expect that $\psi^{\prime} = \psi$. Example: in the <a id="ix-173">Maxwell</a> theory which is relativistic, the magnetic field $H$ is no longer a pure magnetic field in a moving system. Instead it transforms like a tensor. So we have to find _some_ transformation law for the $\psi$ which will leave invariant the physical consequences of the equations.

We need in fact two things: (i) the interpretation of $\psi^{*} \psi$ as a probability <a id="ix-185">density</a> must be preserved, (ii) the validity of the Dirac equation must be preserved in the new system.

First consider (i). The quantity which can be directly observed and must be invariant is the quantity

$$
( \psi^{*} \psi ) \times V
$$

where $V$ is a volume. Now in going to a new Lorentz system with relative velocity $v$ the volume $V$ changes by <a id="ix-93">Fitzgerald contraction</a> to the value

$$
V^{\prime} = V \sqrt{1 - \frac{v^{2}}{c^{2}}}
$$

Therefore

$$
( \psi^{*\prime} \psi^{\prime} ) = \frac{\psi^{*} \psi}{\sqrt{1 - \frac{v^{2}}{c^{2}}}} \tag{17}
$$

and so $( \psi^{*} \psi ) = \rho$ transforms like an _energy_, i.e.like the fourth component of a vector. This shows incidentally that $\psi^{\prime} \neq \psi$. Since $\rho$ and $\hbar$ are related by the equation of continuity, the space-components of the 4-vector are

$$
( S_{1} ,S_{2} ,S_{3} ) = \psi^{*} \alpha^{k} \psi = \frac{1}{c} j_{k} \tag{18}
$$

So we require that the 4 quantities

$$
( S_{1} ,S_{2} ,S_{3} ,S_{0} ) = \left( \psi^{*} \alpha^{k} \psi ,\psi^{*} \psi \right) \tag{19}
$$

transform like a _4-vector_. This will be enough to preserve the interpretation of the theory.

Assume that

$$
\psi^{\prime} = S\psi \tag{20}
$$

where $S$ is a _linear_ operator. Then

$$
\psi^{\prime *} = \psi^{*} S^{*} \tag{21}
$$

So we require

$$
\begin{aligned} \psi^{*\prime} \alpha^{k} \psi^{\prime} &= \psi^{*} S^{*} \alpha^{k} S\psi = \sum_{\nu = 0}^{3} a_{k\nu} \psi^{*} \alpha^{\nu} \psi \\ \psi^{*\prime} \psi^{\prime} &= \psi^{*} S^{*} S\psi = \sum_{\nu = 0}^{3} a_{0 \nu} \psi^{*} \alpha^{\nu} \psi \end{aligned} \tag{22}
$$

writing $\alpha^{0} = \mathbb{I}$.

<a id="p7"><sup>[p. 7]</sup></a>

Thus we need $3$

$$
S^{*} \alpha^{\mu} S = \sum_{\nu = 0} a_{\mu \nu} \alpha^{\nu} , \quad \mu = 0 , 1 , 2 , 3 \tag{23}
$$

Next consider (ii). The Dirac equation for $\psi^{\prime}$ is

$$
\sum_{0}^{3} \alpha^{\nu} \frac{\partial}{\partial x_{\nu}^{\prime}} \psi^{\prime} + i \frac{mc}{\hbar} \beta \psi^{\prime} = 0 \tag{24}
$$

Now the original Dirac equation for $\psi$ expressed in terms of the new coordinates is

$$
\sum_{\mu = 0 \nu = 0}^{3} \sum^{3} \alpha^{\mu} \frac{\partial}{\partial x_{\nu}^{\prime}} a_{\nu \mu} S^{- 1} \psi^{\prime} + i \frac{mc}{\hbar} \beta S^{- 1} \psi^{\prime} = 0 \tag{25}
$$

The sets of equations (24) and (25) have to be equivalent, not identical. Thus (25) must be the same as (24) multiplied by $\beta S^{- 1} \beta$. The condition for this is

$$
\beta S^{- 1} \beta \alpha^{\nu} = \sum_{0}^{3} \alpha^{\lambda} a_{\nu \lambda} S^{- 1} \tag{26}
$$

But (23) and (26) are identical if

$$
\beta S^{- 1} \beta = S^{*} \quad \text{ which means } \quad S^{*} \beta S = \beta \tag{27}
$$

Thus $\beta$ transforms like a scalar, $\alpha^{\nu}$ like a 4-vector when multiplied by $S^{*} S$.

## <a id="s-to-find-the-s"></a>To find the S

Given two coordinate transformations in succession, with matrices already found, the combined transformation will correspond to the product of these matrices. Hence we have to consider only 3 simple types of transformation.

1) Pure rotations

$$
\begin{array}{lcl}
x_{0}^{\prime} & = & x_{0} \quad x_{3}^{\prime} = x_{3} \\
x_{1}^{\prime} & = & x_{1} \cos \theta + x_{2} \sin \theta \\
x_{2}^{\prime} & = & - x_{1} \sin \theta + x_{2} \cos \theta \\
\end{array}
$$

2) Pure Lorentz transformations

$$
\begin{array}{lcl}
x_{1}^{\prime} & = & x_{1} \quad x_{2}^{\prime} = x_{2} \\
x_{3}^{\prime} & = & x_{3} \cosh \theta + x_{0} \sinh \theta \\
x_{0}^{\prime} & = & x_{3} \sinh \theta + x_{0} \cosh \theta \\
\end{array}
$$

3) Pure reflections

$$
x_{1}^{\prime} = - x_{1} \quad x_{2}^{\prime} = - x_{2} \quad x_{3}^{\prime} = - x_{3} \quad x_{0}^{\prime} = x_{0}
$$

_Case 1_ Then

$$
S = \cos \tfrac{1}{2} \theta + i\sigma_{3} \sin \tfrac{1}{2} \theta \tag{28}
$$

<a id="p8"><sup>[p. 8]</sup></a>

Here

$$
\sigma_{3} = \begin{pmatrix} \sigma_{3} & 0 \\ 0 & \sigma_{3} \end{pmatrix}
$$

commutes with $\alpha_{3}$ and $\beta$.

$$
\sigma_{3} \alpha_{1} = i\alpha_{2} , \quad \sigma_{3} \alpha_{2} = - i\alpha_{1}
$$

$$
S^{*} = \cos \tfrac{1}{2} \theta - i\sigma_{3} \sin \tfrac{1}{2} \theta
$$

Then

$$
S^{*} \beta S = \beta
$$

$$
S^{*} \alpha^{0} S = \alpha^{0}
$$

$$
S^{*} \alpha^{3} S = \alpha^{3}
$$

as required.

$$
S^{*} \alpha^{1} S = \cos \theta \alpha^{1} + \sin \theta \alpha^{2}
$$

$$
S^{*} \alpha^{2} S = - \sin \theta \alpha^{1} + \cos \theta \alpha^{2}
$$

## <a id="s-case-2"></a>Case 2

$$
S = S^{*} = \cosh \tfrac{1}{2} \theta + \alpha_{3} \sinh \tfrac{1}{2} \theta \tag{29}
$$

Here

$$
S^{*} \beta S = \beta
$$

$$
S^{*} \alpha^{1} S = \alpha^{1}
$$

$$
S^{*} \alpha^{2} S = \alpha^{2}
$$

$$
S^{*} \alpha^{3} S = \cosh \theta \alpha^{3} + \sinh \theta \alpha^{0}
$$

$$
S^{*} \alpha^{0} S = \sinh \theta \alpha^{3} + \cos \theta \alpha^{0}
$$

## <a id="s-case-3"></a>Case 3

$$
S = S^{*} = \beta \tag{30}
$$

Note that in all cases S is ambiguous by a factor $\pm$1. So in case 1 a rotation though 360$◦$ gives $S = - 1 .$

_Problem 1_ Find the S corresponding to a general infinitesimal coordinate transformation. Compare and show that it agrees with the exact solutions given here.

The $\psi_{\alpha}$’s transforming with these $S$-transformations are called <a id="ix-213"><em>spinors</em></a>. They are a direct extension of the non-relativistic 2-component spin-functions. Mathematical theory of spinors is not very useful. In fact we find always in practice, calculations can be done most easily if one avoids any explicit representation of the spinors. _Use only formal algebra and commutation relations of the matrices._

## <a id="s-the-covariant-notation"></a>The covariant notation

In order to avoid distinction between covariant and contravariant vectors (which we have also unjustifiably ignored in the previous discussion) it is useful to use the imaginary 4<sup>th</sup> coordinate

$$
x_{4} = ix_{0} = ict \tag{31}
$$

In this coordinate system the four matrices<sup><a id="fnref-5" href="#fn-5">5</a></sup>

$$
\gamma_{1 , 2 , 3 , 4} = \left( - i\beta \alpha^{1 , 2 , 3} ,\beta \right) \quad i . e . \tag{32}
$$

<a id="p9"><sup>[p. 9]</sup></a>

$$
\gamma_{1} = \begin{pmatrix} 0 & \begin{matrix} 0 & - i \\ - i & 0 \end{matrix} \\ \begin{matrix} 0 & i \\ i & 0 \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{2} = \begin{pmatrix} 0 & \begin{matrix} 0 & - 1 \\ 1 & 0 \end{matrix} \\ \begin{matrix} 0 & 1 \\ - 1 & 0 \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{3} = \begin{pmatrix} 0 & \begin{matrix} - i & 0 \\ 0 & i \end{matrix} \\ \begin{matrix} i & 0 \\ 0 & - i \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{4} = \begin{pmatrix} \begin{matrix} 1 & 0 \\ 0 & 1 \end{matrix} & 0 \\ 0 & \begin{matrix} - 1 & 0 \\ 0 & - 1 \end{matrix} \end{pmatrix}
$$

are a 4-vector. They are all <a id="ix-115">Hermitian</a> and satisfy

$$
\gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu} = 2 \delta_{\mu \nu} \tag{33}
$$

The Dirac equation and its conjugate may now be written

$$
\begin{aligned} \sum_{1}^{4} \gamma_{\mu} \frac{\partial \psi}{\partial x_{\mu}} + \frac{mc}{\hbar} \psi &= 0 \\ \sum_{1}^{4} \frac{\partial \overline{\psi}}{\partial x_{\mu}} \gamma_{\mu} - \frac{mc}{\hbar} \overline{\psi} &= 0 \end{aligned} \tag{34}
$$

with

$$
\overline{\psi} = \psi^{*} \beta \quad \text{ and } \tag{35}
$$

$$
s_{\mu} = i \left( \overline{\psi} \gamma_{\mu} \psi \right) = \left( \frac{1}{c} \hbar ,i\rho \right) \tag{36}
$$

These notations are much the most convenient for calculations.

## <a id="s-conservation-laws-existence-of-spin"></a><a id="ix-64">Conservation Laws</a>. <a id="ix-211">Existence of Spin</a>.

The Hamiltonian in this theory is<sup><a id="fnref-6" href="#fn-6">6</a></sup>

$$
i \hbar \frac{\partial \psi}{\partial t} = H\psi \tag{37}
$$

$$
H = - i \hbar c \sum_{1}^{3} \alpha^{k} \frac{\partial}{\partial x_{k}} + mc^{2} \beta = - i \hbar c \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla + mc^{2} \beta \tag{38}
$$

This commutes with the <a id="ix-200">momentum</a> $\boldsymbol{p} = - i \hbar \nabla$. So the momentum $\boldsymbol{p}$ is a constant of motion.

However the <a id="ix-27">angular momentum</a> operator

$$
\boldsymbol{L} = \boldsymbol{r} \times \boldsymbol{p} = - i \hbar \boldsymbol{r} \times \nabla \tag{39}
$$

is not a constant. For

$$
[ H, \boldsymbol{L} ] = - \hbar^{2} c \boldsymbol{\alpha} \times \nabla \tag{40}
$$

But

$$
[ H, \boldsymbol{\sigma} ] = - i \hbar c \nabla \boldsymbol{\cdot} [ \boldsymbol{\alpha} , \boldsymbol{\sigma} ] \quad \text{ where } \quad \boldsymbol{\sigma} = ( \sigma_{1} ,\sigma_{2} ,\sigma_{3} )
$$

while

$$
\left[ \alpha^{1} ,\sigma_{1} \right] = 0 , \quad \left[ \alpha^{1} ,\sigma_{2} \right] = 2 i\alpha^{3} , \quad \left[ \alpha^{1} ,\sigma_{3} \right] = - 2 i\alpha^{2} , \quad \text{ etc } .
$$

So

$$
\begin{aligned} [ H,\sigma_{3} ] &= 2 \hbar c \left( \alpha^{1} \nabla_{2} - \alpha^{2} \nabla_{1} \right) \quad \text{ and thus } \\ [ H, \boldsymbol{\sigma} ] &= 2 \hbar c \boldsymbol{\alpha} \times \nabla \end{aligned} \tag{41}
$$

Thus

$$
\boldsymbol{L} + \tfrac{1}{2} \hbar \boldsymbol{\sigma} = \hbar \boldsymbol{J} \tag{42}
$$

<a id="p10"><sup>[p. 10]</sup></a>

is a constant, the total <a id="ix-30">angular momentum</a>, because by (40), (41) and (42)

$$
[ H, \boldsymbol{J} ] = 0
$$

$\boldsymbol{L}$ is the orbital a. m. and $\tfrac{1}{2} \hbar \boldsymbol{\sigma}$ the spin a. m. This agrees with the N. R. theory. But in that theory the spin and $L$ of a free particle were _separately_ constant. This is no longer the case.

When a central force potential $V ( r )$ is added to H, the operator $\boldsymbol{J}$ still is constant.

## <a id="s-elementary-solutions"></a>Elementary Solutions

For a particle with a particular momentum $\boldsymbol{p}$ and energy $E$, the wave function will be

$$
\psi ( x,t ) = u \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right) \tag{43}
$$

where $u$ is a constant spinor. The <a id="ix-86">Dirac</a> <a id="ix-50">equation</a> then becomes an equation for $u$ only

$$
Eu = \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) u \tag{44}
$$

We write now

$$
p_{+} = p_{1} + ip_{2} \quad p_{-} = p_{1} - ip_{2} \tag{45}
$$

Then (44) written out in full becomes

$$
\begin{aligned} \left( E - mc^{2} \right) u_{1} &= c ( p_{3} u_{3} + p_{-} u_{4} ) \\ \left( E - mc^{2} \right) u_{2} &= c ( p_{+} u_{3} - p_{3} u_{4} ) \\ \left( E + mc^{2} \right) u_{3} &= c ( p_{3} u_{1} + p_{-} u_{2} ) \\ \left( E + mc^{2} \right) u_{4} &= c ( p_{+} u_{1} - p_{3} u_{2} ) \end{aligned} \tag{46}
$$

These 4 equations determine $u_{3}$ and $u_{4}$ given $u_{1}$ and $u_{2}$, or vice-versa. And either $u_{1}$ and $u_{2}$, or $u_{3}$ and $u_{4}$, can be chosen arbitrarily provided that<sup><a id="fnref-7" href="#fn-7">7</a></sup>

$$
E^{2} = m^{2} c^{4} + c^{2} p^{2} \tag{47}
$$

Thus given $p$ and $E$ = +$\sqrt{m^{2} c^{4} + c^{2} p^{2}}$, there are two independent solutions of (46); these are, in nonnormalized form:

$$
\begin{pmatrix} 1 \\ 0 \\ \frac{cp_{3}}{E + mc^{2}} \\ \frac{cp_{+}}{E + mc^{2}} \end{pmatrix} \quad \begin{pmatrix} 0 \\ 1 \\ \frac{cp_{-}}{E + mc^{2}} \\ \frac{- cp_{3}}{E + mc^{2}} \end{pmatrix} \tag{48}
$$

This gives the two spin-<a id="ix-107">states</a> of an electron with given momentum, as required physically.

But there are also solutions with $E = - \sqrt{m^{2} c^{4} + c^{2} p^{2}}$. In fact again two independent solutions, making 4 altogether. These are the famous _negative energy states_. Why cannot we simply agree to ignore these states, say they are physically absent? Because when fields are present the theory gives transitions from positive to negative states. e.g.H atom should decay to negative state in 10$- 1 0$ secs. or less.

Certainly negative energy particles are not allowed physically. They can for example never be stopped by matter at rest, with every collision they move faster and faster. So Dirac was driven to

<a id="p11"><sup>[p. 11]</sup></a>

## <a id="s-the-hole-theory"></a>The <a id="ix-65">Hole Theory</a>

All <a id="ix-105">negative-energy</a> <a id="ix-161">states</a> are normally filled by one electron each. Because of the exclusion principle transitions of ordinary electrons to these states are forbidden. If sometimes a negative energy state of momentum $- p$ energy $- E$ is _empty_, this appears as a particle of momentum $p$ energy +$E$, and the opposite charge to an electron, i.e. an ordinary <a id="ix-152">positron</a>.

Thus we are led at once to a many-particle theory in order to get sensible results. With spin-0 particles, to get positive probabilities. With spin-$\frac{1}{2}$ particles, to get positive energies.

The Dirac theory in its one-particle form cannot describe properly the interaction between several particles. But so long as we are talking only about free particles, we can describe them with one-particle wave-functions.

## <a id="s-positron-states-charge-conjugation"></a>Positron States

So which wave-function will describe a positron with momentum $p$ and energy $E$? Clearly the wave function should be of the form

$$
\varphi ( x,t ) = v \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right) \tag{49}
$$

as always in quantum mechanics. But the negative-energy electron whose _absence is_ the positron has a wave-function

$$
\psi ( x,t ) = u \exp \left( - i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} + i \frac{Et}{\hbar} \right) \tag{50}
$$

since it has a momentum $- p$ energy $- E$.

Thus we must take

$$
\varphi = C\psi^{+} , \quad i . e . \quad v = Cu^{+} \tag{51}
$$

where $\psi^{+}$ is $\psi$ with complex conjugate elements but _not_ transposed, and $C$ is a suitable constant matrix;

$$
\psi^{+} ( x,t ) = u^{+} \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right)
$$

We know that $u$ is a solution of

$$
Eu = \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} - mc^{2} \beta \right) u \tag{52}
$$

We want the theory to make no distinction between electrons and positrons, and so $v$ must also satisfy the Dirac equation

$$
\begin{aligned} Ev &= \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) v \\ ECu^{+} &= \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) Cu^{+} \end{aligned} \tag{53}
$$

But from (52) we have for $u^{+}$ the equation

$$
Eu^{+} = \left( c \boldsymbol{\alpha}^{+} \boldsymbol{\cdot} \boldsymbol{p} - mc^{2} \beta^{+} \right) u^{+} \tag{54}
$$

In order that (53) and (54) be identical we should have

$$
C\alpha^{k +} = \alpha^{k} C, \quad C\beta^{+} = - \beta C \tag{55}
$$

Now in fact

$$
\alpha^{1 +} = \alpha^{1} \quad \alpha^{3 +} = \alpha^{3} \quad \alpha^{2 +} = - \alpha^{2} \quad \beta^{+} = \beta
$$

Therefore a suitable C will be

$$
C = - i\beta \alpha^{2} = \gamma_{2} = \begin{pmatrix} 0 & \begin{matrix} 0 & - 1 \\ 1 & 0 \end{matrix} \\ \begin{matrix} 0 & 1 \\ - 1 & 0 \end{matrix} & 0 \end{pmatrix} \tag{56}
$$

<a id="p12"><sup>[p. 12]</sup></a>

The relation between $\psi$ and $\varphi$ is symmetrical because

$$
C^{2} = \mathbb{I} \quad \text{ Hence } \quad \psi = C\varphi^{+} \tag{57}
$$

The $\varphi$ is called the _charge-conjugate_ wave-function corresponding to the <a id="ix-106">negative-energy</a> electron $\psi$. Clearly

$$
\varphi^{*} \varphi = \left( C\psi^{+} \right)^{*} \left( C\psi^{+} \right) = \psi^{T} C^{*} C\psi^{+} = \psi^{*} ( C^{*} C )^{T} \psi = \psi^{*} \psi \tag{58}
$$

And

$$
\varphi^{*} \alpha^{k} \varphi = \psi^{T} C^{*} \alpha^{k} C\psi^{+} = \psi^{*} C\alpha^{kT} C\psi = \psi^{*} \alpha^{k} \psi \tag{59}
$$

Thus the probability and flow <a id="ix-186">densities</a> are the same for a positron as for the conjugate negative electron.

For many purposes it is easier to represent positrons directly by the $\overline{\psi}$ wave-function, e.g. in computing cross-sections for <a id="ix-43">pair </a><a id="ix-8">creation</a> and so forth as we shall do later. But if you actually want to _see_ the positron, e.g. in describing the details of a positronium experiment, it is necessary to use the $\varphi$ wave-function to represent e.g. the way the spin is pointing.

This is all we shall say about free electrons and positrons.

## <a id="s-electromagnetic-properties-of-the-electron"></a>Electromagnetic Properties of the Electron

Given an external (c-number) electromagnetic field defined by the potentials

$$
A_{\mu} \quad \mu = 1 , 2 , 3 , 4 \quad A_{4} = i \Phi
$$

given functions of space and time. Then the motion of a particle in the field is found by substituting in the <a id="ix-136">free-particle Lagrangian</a>

$$
\begin{aligned} & \quad E + e \Phi \quad \text{ for } E \\ & \quad \boldsymbol{p} + \frac{e}{c} \boldsymbol{A} \quad \text{ for } \boldsymbol{p} \end{aligned} \tag{60}
$$

where $( - e )$ is the electron charge. We write the momentum-energy 4-vector

$$
p = ( p_{1} ,p_{2} ,p_{3} ,p_{4} = iE/c ) \tag{61}
$$

Then we have to substitute simply

$$
p_{\mu} + \frac{e}{c} A_{\mu} \quad \text{ for } p_{\mu} \tag{62}
$$

Now in the quantum theory

$$
p_{\mu} \to - i \hbar \frac{\partial}{\partial x_{\mu}} \tag{63}
$$

Therefore the Dirac equation with fields is

$$
\sum_{1}^{4} \gamma_{\mu} \left( \frac{\partial}{\partial x_{\mu}} + \frac{ie}{\hbar c} A_{\mu} \right) \psi + \frac{mc}{\hbar} \psi = 0 \tag{64}
$$

$$
\sum_{1}^{4} \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \overline{\psi} \gamma_{\mu} - \frac{mc}{\hbar} \overline{\psi} = 0 \tag{65}
$$

In the non-covariant notations this is

$$
i \hbar \frac{\partial \psi}{\partial t} = \left[ - e \Phi + \sum_{1}^{3} \left( - i \hbar c \frac{\partial}{\partial x_{k}} + eA_{k} \right) \alpha^{k} + mc^{2} \beta \right] \psi \tag{66}
$$

<a id="p13"><sup>[p. 13]</sup></a>

since by (57), we have $\overline{\psi} \gamma_{\mu} = \psi^{*} \beta \gamma_{\mu} = ( C\varphi^{+} )^{T} \beta \gamma_{\mu} = \varphi^{T} C^{T} \beta \gamma_{\mu}$; the wave function $\varphi = C\psi^{+}$of a positron satisfies by (65)

$$
\sum \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \gamma_{\mu}^{T} \beta C\varphi - \frac{mc}{\hbar} \beta C\varphi = 0 \tag{67}
$$

Multiplying by $C\beta$ this gives

$$
\sum \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \gamma_{\mu} \varphi + \frac{mc}{\hbar} \varphi = 0 \tag{68}
$$

This is exactly the Dirac <a id="ix-51">equation</a> for a particle of positive charge (+e). We have used

$$
C\beta \gamma_{\mu}^{T} \beta C = - \gamma_{\mu} , \tag{69}
$$

which follows from (15), (32), and (55).

## <a id="s-the-hydrogen-atom"></a>The <a id="ix-120">Hydrogen Atom</a>

This is the one problem which it is possible to treat very accurately using the one-electron Dirac theory. The problem is to find the eigenstates of the equation

$$
\begin{aligned} E\psi &= H\psi \\ H &= - i \hbar c \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla + mc^{2} \beta - \frac{e^{2}}{r} \end{aligned} \tag{70}
$$

As in the NR theory, we have as quantum numbers in addition to $E$ itself the quantities

$$
j_{z} = - i [ \boldsymbol{r} \times \nabla ]_{3} + \tfrac{1}{2} \sigma_{3} \tag{71}
$$

$$
j ( j + 1 ) = J^{2} = \left[ - i ( \boldsymbol{r} \times \nabla ) + \tfrac{1}{2} \boldsymbol{\sigma} \right]^{2} \tag{72}
$$

where $j_{z}$ and $j$ are now _half-odd integers_ by the ordinary theory of <a id="ix-28">angular momenta</a>. These quantum numbers are not enough to fix the state, because each value of $j$ may correspond to two NR states with $\ell$between states with$= j \pm \tfrac{1}{2}$. Therefore we need an additional operator which commutes with $H$, which will distinguish $\boldsymbol{\sigma}$ parallel or antiparallel to $\boldsymbol{J}$. The obvious choice is

$$
Q = \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J}
$$

But $[ H, \boldsymbol{\sigma} ]$ is non-zero and rather complicated. So it is better to try

$$
Q = \beta \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J} \tag{73}
$$

which is the same in the NR limit.

Then we have

$$
[ H,Q ] = [ H,\beta \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J} ] = [ H,\beta \boldsymbol{\sigma} ] \boldsymbol{\cdot} \boldsymbol{J} + \beta \boldsymbol{\sigma} \boldsymbol{\cdot} [ H, \boldsymbol{J} ]
$$

But $[ H, \boldsymbol{J} ]$ = 0; furthermore, since

$$
\alpha^{k} \beta \sigma_{\ell} = \beta \sigma_{\ell} \alpha^{k} \quad k \neq \ell \quad \text{ and } \quad \alpha^{k} \beta \sigma_{k} = - \beta \sigma_{k} \alpha^{k}
$$

we get

$$
[ H,\beta \boldsymbol{\sigma} ] = - i \hbar c \{ ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla ) \beta \boldsymbol{\sigma} - \beta \boldsymbol{\sigma} ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla ) \} = - 2 i \hbar c \sum_{k = 1}^{3} \alpha^{k} \sigma_{k} \beta \nabla_{k}
$$

Therefore

$$
[ H,\beta \boldsymbol{\sigma} ] \boldsymbol{\cdot} \boldsymbol{J} = - 2 \hbar c \sum_{k = 1}^{3} \alpha^{k} \sigma_{k} \beta \nabla_{k} ( \boldsymbol{r} \times \nabla )_{k} - i \hbar c ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla ) \beta = - i \hbar c ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla ) \beta = \left[ H, \tfrac{1}{2} \beta \right]
$$

<a id="p14"><sup>[p. 14]</sup></a>

because

$$
\nabla \boldsymbol{\cdot} \boldsymbol{r} \times \nabla = 0 \quad \text{ and } \quad \alpha^{k} \sigma_{k} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \text{ for all } k ’ s .
$$

Hence the quantity which commutes with $H$ and is a constant of the motion is

$$
K = \beta \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J} - \tfrac{1}{2} \beta \tag{74}
$$

There must be a relation between $K$ and $J$. In fact

$$
K^{2} = \left( \frac{\boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{L}}{\hbar} + 1 \right)^{2} = \frac{L^{2}}{\hbar^{2}} + \frac{\boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{L}}{\hbar} + 1
$$

$$
J^{2} = \left( \frac{\boldsymbol{L}}{\hbar} + \tfrac{1}{2} \boldsymbol{\sigma} \right)^{2} = \frac{L^{2}}{\hbar^{2}} + \frac{\boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{L}}{\hbar} + \tfrac{3}{4}
$$

Hence

$$
K^{2} = J^{2} + \tfrac{1}{4} = \left( j + \tfrac{1}{2} \right)^{2} \tag{75}
$$

Therefore $K$ has _integer_ eigenvalues not zero,

$$
K = k = \pm \left( j + \tfrac{1}{2} \right) \tag{76}
$$

$$
j = | k |- \tfrac{1}{2} , \quad k = \pm 1 , \pm 2 , \pm 3 ,\dots \tag{77}
$$

Using the eigenvalue for $K$, we can simplify the <a id="ix-121">Hamiltonian</a>, which we could not do as in the NR theory with the eigenvalue of $L^{2}$ alone. First

$$
\boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{r\sigma} \boldsymbol{\cdot} ( \boldsymbol{r} \times \nabla ) = i \boldsymbol{\sigma} \boldsymbol{\cdot} ( \boldsymbol{r} \times ( \boldsymbol{r} \times \nabla ) ) = i ( \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{r} ) ( \boldsymbol{r} \boldsymbol{\cdot} \nabla ) - ir^{2} \boldsymbol{\sigma} \boldsymbol{\cdot} \nabla \tag{78}
$$

Let now

$$
\epsilon = - i\alpha^{1} \alpha^{2} \alpha^{3} \quad \sigma_{k} = \epsilon \alpha^{k} \tag{79}
$$

Then multiplying (78) by $\epsilon^{- 1}$ we get:

$$
- r^{2} i \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla = \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{r\sigma} \boldsymbol{\cdot} ( \boldsymbol{r} \times \nabla ) - i \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{r} \left( r \frac{\partial}{\partial r} \right)
$$

Let $\alpha_{r} = \frac{1}{r} \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{r}$, then by (39) and (42)

$$
- i \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla = \frac{1}{r} \alpha_{r} \left( i \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J} - \tfrac{3}{2} i \right) - i\alpha_{r} \frac{\partial}{\partial r} = \frac{1}{r} \alpha_{r} ( i\beta K - i ) - i\alpha_{r} \frac{\partial}{\partial r}
$$

Thus finally we can write (70) in the form

$$
H = mc^{2} \beta - \frac{e^{2}}{r} + i \hbar c\alpha_{r} \left( \frac{\beta K}{r} - \frac{1}{r} - \frac{\partial}{\partial r} \right) \tag{80}
$$

This gives the <a id="ix-87">Dirac</a> <a id="ix-52">equation</a> as an equation in the single variable $r$, having separated all angular variables.

For the solution of this equation, see – Dirac, _Quantum Mechanics_, Third Edition, Sec. 72, pp 268-271.

## <a id="s-solution-of-the-radial-equation"></a>Solution of Radial Equation

We may choose a two-component representation in which

$$
\beta = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix} , \quad \alpha_{r} = \begin{pmatrix} 0 & i \\ - i & 0 \end{pmatrix} , \quad \psi = \begin{pmatrix} u \\ v \end{pmatrix} \tag{81}
$$

<a id="p15"><sup>[p. 15]</sup></a>

Then

$$
\begin{aligned} \left( E - mc^{2} \right) u &= - \frac{e^{2}}{r} u + \hbar c \left( \frac{1 + K}{r} + \frac{\partial}{\partial r} \right) v \\ \left( E + mc^{2} \right) v &= - \frac{e^{2}}{r} v + \hbar c \left( - \frac{1 - K}{r} - \frac{\partial}{\partial r} \right) u \end{aligned} \tag{82}
$$

Let now

$$
a_{1} = \frac{- E + mc^{2}}{\hbar c} ; \quad a_{2} = \frac{E + mc^{2}}{\hbar c} ; \quad \alpha = \frac{e^{2}}{\hbar c} , \tag{83}
$$

the <a id="ix-89">fine structure constant</a>. Then

$$
\begin{aligned} \left( - a_{1} + \frac{\alpha}{\overline{r}} \right) u &= \left( \frac{1 + K}{r} + \frac{\partial}{\partial r} \right) v \\ \left( a_{2} + \frac{\alpha}{r} \right) v &= \left( \frac{- 1 + K}{r} - \frac{\partial}{\partial r} \right) u \end{aligned} \tag{84}
$$

Next put $a = \sqrt{a_{1} a_{2}} = \sqrt{m^{2} c^{4} - E^{2}} / \hbar c$ which is the magnitude of the imaginary <a id="ix-201">momentum</a> of a free electron of energy $E$. Then $\psi \sim e^{- ar}$ at infinity. Hence we write

$$
\begin{aligned} u &= \frac{e^{- ar}}{r} f \\ v &= \frac{e^{- ar}}{r} g \end{aligned} \tag{85}
$$

So

$$
\begin{aligned} \left( \frac{\alpha}{r} - a_{1} \right) f &= \left( \frac{\partial}{\partial r} - a + \frac{k}{r} \right) g \\ \left( \frac{\alpha}{r} + a_{2} \right) g &= \left( - \frac{\partial}{\partial r} + a + \frac{k}{r} \right) f \end{aligned} \tag{86}
$$

Now we try solutions in series

$$
f = \sum c_{s} r^{s} , \quad g = \sum d_{s} r^{s} \tag{87}
$$

This gives

$$
\begin{aligned} \alpha c_{s} - a_{1} c_{s - 1} &= - ad_{s - 1} + ( s + k ) d_{s} \\ \alpha d_{s} + a_{2} d_{s - 1} &= + ac_{s - 1} + ( - s + k ) c_{s} \end{aligned} \tag{88}
$$

Putting

$$
e_{s} = a_{1} c_{s - 1} - ad_{s - 1} \quad \text{ we have } \quad e_{s} = \alpha c_{s} - ( s + k ) d_{s} = \frac{a_{1}}{a} ( \alpha d_{s} + ( s - k ) c_{s} )
$$

$$
c_{s} = \frac{a_{1} \alpha + a ( s + k )}{a_{1} \alpha^{2} + a_{1} ( s^{2} - k^{2} )} e_{s} \quad d_{s} = \frac{a\alpha - a_{1} ( s - k )}{a_{1} \alpha^{2} + a_{1} ( s^{2} - k^{2} )} e_{s}
$$

$$
e_{s + 1} = \frac{\left( a_{1}^{2} - a^{2} \right) \alpha + 2 saa_{1}}{a_{1} \alpha^{2} + a_{1} ( s^{2} - k^{2} )} e_{s}
$$

<a id="p16"><sup>[p. 16]</sup></a>

Suppose the series do not terminate. Then for large $s$

$$
\frac{e_{s + 1}}{e_{s}} \approx \frac{c_{s + 1}}{c_{s}} \approx \frac{2 a}{s} \quad \text{ hence } \quad f \approx \exp ( 2 ar )
$$

This is permissible when $a$ is imaginary. Thus there is a continuum of states with

$$
E>mc^{2} \tag{89}
$$

For real $a$ the series must terminate at both ends in order not to blow up at infinity. Suppose then<sup><a id="fnref-8" href="#fn-8">8</a></sup> $e_{s}$ is non-zero for

$$
s = \epsilon + 1 ,\epsilon + 2 ,\dots \epsilon + n \quad n \geq 1 \tag{90}
$$

and otherwise zero. This gives

$$
\alpha^{2} + \epsilon^{2} - k^{2} = 0
$$

$$
\left( a_{1}^{2} - a^{2} \right) \alpha + 2 ( \epsilon + n ) aa_{1} = 0
$$

$c_{\epsilon}$ and $d_{\epsilon}$ are zero, thus the wave function $r^{- 1 + \epsilon}$ must be integrable at zero. This gives $\epsilon >$Now not both$- \tfrac{1}{2}$. But $\epsilon = \pm \sqrt{k^{2} - \alpha^{2}}$. Now $k^{2} \geq$ 1, hence $\sqrt{k^{2} - \alpha^{2}} > \tfrac{1}{2}$, and

$$
\epsilon = + \sqrt{k^{2} - \alpha^{2}} \tag{91}
$$

Also<sup><a id="fnref-9" href="#fn-9">9</a></sup>

$$
( \epsilon + n )^{2} = \left( \frac{a_{1}^{2} - a^{2}}{2 aa_{1}} \right)^{2} \alpha^{2} = \left( \frac{\left( \left( mc^{2} - E \right)^{2} - \left( m^{2} c^{4} - E^{2} \right) \right)^{2}}{4 ( m^{2} c^{4} - E^{2} ) ( mc^{2} - E )^{2}} \right) \alpha^{2} = \frac{4 E^{2} \alpha^{2}}{4 ( m^{2} c^{4} - E^{2} )}
$$

$$
E^{2} = \frac{m^{2} c^{4}}{\left( 1 + \frac{\alpha^{2}}{( \epsilon + n )^{2}} \right)}
$$

Hence in this case

$$
E = \frac{mc^{2}}{\sqrt{1 + \frac{\alpha^{2}}{\left( n + \sqrt{k^{2} - \alpha^{2}} \right)^{2}}}} \tag{92}
$$

Given this positive $E , \left( a_{1}^{2} - a^{2} \right)$ is negative (see (83) and (84)) and so it is allowable to square $( \epsilon + n )$ to find these solutions, without introducing any difficulties. So for each

$$
\begin{aligned} k &= \pm 1 , \pm 2 , \pm 3 ,\dots \\ n &= 1 , 2 , 3 ,\dots \end{aligned} \tag{93}
$$

solutions exist, with $E$ given by (92).

The alternative possibility is that _all_ $e_{s}$ _are zero_. Suppose not both of $c_{\epsilon}$ and $d_{\epsilon}$ are zero. Then $\alpha^{2} + \epsilon^{2} - k^{2}$ = 0 as before and so $\epsilon = \sqrt{k^{2} - \alpha^{2}}$. But now

$$
a_{1} c_{\epsilon} - ad_{\epsilon} = 0
$$

$$
\alpha c_{\epsilon} - ( \epsilon + k ) d_{\epsilon} = 0
$$

Hence $a\alpha - a_{1} ( \epsilon + k )$ = 0 and $k$ must be positive to make $\epsilon + k = \sqrt{k^{2} - \alpha^{2}} + k>$ 0. After this the solution goes as before. So solutions (92) exist for

$$
n = 0 , \quad k = + 1 , + 2 , + 3 ,\dots \tag{94}
$$

<a id="p17"><sup>[p. 17]</sup></a>

The principal quantum number $N$ is

$$
N = n + | k |
$$

Expanding in powers of $\alpha$

$$
E = mc^{2} \left[ 1 - \underbrace{\frac{1}{2} \frac{\alpha^{2}}{N^{2}}}_{\text{ NR levels }} + \underbrace{\frac{\alpha^{4}}{N^{3}} \left( \frac{3}{8 N} - \frac{1}{2 | k |} \right)}_{\text{ fine structure }} \right] \tag{95}
$$

There is _exact_ degeneracy between the two states of a given $| k |$. Nonrelativistic states are given by

$$
j = \ell + \tfrac{1}{2} \to k = - ( \ell + 1 )
$$

$$
j = \ell - \tfrac{1}{2} \to k = + \ell
$$

So

$$
\left. \begin{aligned} {}^{2} P_{1 / 2} \quad \text{ is } \quad j &= \tfrac{1}{2} , \quad k = 1 \\ {}^{2} S_{1 / 2} \quad \text{ is } \quad j &= \tfrac{1}{2} , \quad k = - 1 \end{aligned} \right\} \to \text{ degenerate }
$$

$$
{}^{2} S_{3 / 2} \quad \text{ is } \quad j = \tfrac{1}{2} , \quad k = - 2
$$

## <a id="s-behavior-of-an-electron-in-non-relativistic-approximation"></a>Behaviour of an Electron in a Non-Relativistic Approximation

Multiplying the Dirac <a id="ix-53">equation</a> (64) by $\sum \gamma_{\nu} \left( \frac{\partial}{\partial x_{\nu}} + i \frac{e}{\hbar c} A_{\nu} \right) - \frac{mc}{\hbar}$ we have

$$
\sum_{\mu} \sum_{\nu} \gamma_{\mu} \gamma_{\nu} \left( \frac{\partial}{\partial x_{\mu}} + i \frac{e}{\hbar c} A_{\mu} \right)^{\nu} \left( \frac{\partial}{\partial x_{\nu}} + i \frac{e}{\hbar c} A_{\nu} \right) \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi = 0 \tag{96}
$$

Using $\gamma_{\mu}^{2}$ = 1$,\gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu}$ = 0 this gives

$$
\sum_{\mu} \left\{ \left( \frac{\partial}{\partial x_{\mu}} + \frac{ie}{\hbar c} A_{\mu} \right)^{2} \right\} \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi + \frac{ie}{2 \hbar c} \sum_{\mu} \sum_{\nu} \sigma_{\mu \nu} F_{\mu \nu} \psi = 0 \tag{97}
$$

Here

$$
\sigma_{\mu \nu} = \tfrac{1}{2} ( \gamma_{\mu} \gamma_{\nu} - \gamma_{\nu} \gamma_{\mu} ) \quad F_{\mu \nu} = \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}}
$$

Thus $F_{1 2} = H_{3}$ magnetic field component

$$
F_{1 4} = i \frac{\partial \Phi}{\partial x_{1}} + \frac{i}{c} \frac{\partial A_{1}}{\partial t} = - iE_{1} \quad \text{ electric field }
$$

$$
\sigma_{1 2} = i\sigma_{3} \quad \text{ spin component }
$$

$$
\sigma_{1 4} = i\alpha_{1} \quad \text{ velocity component }
$$

Thus (97) becomes

$$
\sum_{\mu} \left\{ \left( \frac{\partial}{\partial x_{\mu}} + \frac{ie}{\hbar c} A_{\mu} \right)^{2} \right\} \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi - \frac{e}{\hbar c} \{ \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{H} - i \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E} \} \psi = 0 \tag{98}
$$

This is still exact.

<a id="p18"><sup>[p. 18]</sup></a>

Now in the NR approximation

$$
i \hbar \frac{\partial}{\partial t} = mc^{2} + O ( 1 )
$$

$$
\left\{ \left( \frac{\partial}{\partial x_{4}} + \frac{ie}{\hbar c} A_{4} \right)^{2} \right\} - \frac{m^{2} c^{2}}{\hbar^{2}} = \frac{1}{\hbar^{2} c^{2}} \left\{ \left( - i \hbar \frac{\partial}{\partial t} - e \Phi \right)^{2} - m^{2} c^{4} \right\}
$$

$$
= \frac{1}{\hbar^{2} c^{2}} \left\{ \left( - i \hbar \frac{\partial}{\partial t} - e \Phi - mc^{2} \right) \left( - i \hbar \frac{\partial}{\partial t} - e \Phi + mc^{2} \right) \right\}
$$

$$
= \frac{1}{\hbar^{2} c^{2}} \left\{ - 2 mc^{2} + O ( 1 ) \right\} \left( - i \hbar \frac{\partial}{\partial t} - e \Phi + mc^{2} \right)
$$

Hence

$$
\left( - i \hbar \frac{\partial}{\partial t} - e \Phi + mc^{2} \right) \psi - \frac{h^{2}}{2 m} \sum_{k = 1}^{3} \left\{ \left( \frac{\partial}{\partial x_{k}} + \frac{ie}{\hbar c} A_{k} \right)^{2} \right\} \psi + \frac{e \hbar}{2 mc} [ \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{H} - i \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E} ] \psi + O \left( \frac{1}{mc^{2}} \right) = 0
$$

The NR approximation means dropping the terms $O \left( 1 /mc^{2} \right)$. Thus the NR Schrödinger <a id="ix-229">equation</a> is

$$
i \hbar \frac{\partial \psi}{\partial t} = \left\{ mc^{2} - e \Phi - \frac{h^{2}}{2 m} \sum_{k = 1}^{3} \left( \frac{\partial}{\partial x_{k}} + \frac{ie}{\hbar c} A_{k} \right)^{2} + \frac{e \hbar}{2 mc} ( \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{H} - i \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E} ) \right\} \psi \tag{99}
$$

The term $\boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E}$ is really relativistic, and should be dropped or treated more exactly. Then we have exactly the equation of motion of a NR particle with a spin <a id="ix-102">magnetic moment</a> equal to

$$
M = - \frac{e \hbar}{2 mc} \sigma \tag{100}
$$

This is one of the greatest triumphs of Dirac, that he got this magnetic moment right out of his general assumptions without any arbitrariness.

It is confirmed by measurements to about one part in 1000. Note that the most recent experiments show a definite discrepancy, and agree with the value

$$
M = - \frac{e \hbar}{2 mc} \sigma \left\{ 1 + \frac{e^{2}}{2 \pi \hbar c} \right\} \tag{101}
$$

calculated by <a id="ix-242">Schwinger</a> using the complete many-particle theory.

_Problem 2_ Calculate energy values and <a id="ix-80">wave functions</a> of a Dirac particle moving in a homogeneous infinite magnetic field. Can be done exactly. See F. <a id="ix-226">Sauter</a>, _Zeitschrift für Physik_ **69** (1931) 742.

_Solution_ Take the field $\boldsymbol{B}$ in the $z$ direction.

$$
A_{1} = - \tfrac{1}{2} By, \quad A_{2} = \tfrac{1}{2} Bx
$$

The second-order Dirac equation (98) gives for a stationary state of energy $\pm E$

$$
\left( \frac{E^{2}}{\hbar^{2} c^{2}} - \frac{m^{2} c^{2}}{\hbar^{2}} \right) \psi + \left( \frac{\partial}{\partial x} - \frac{1}{2} \frac{ieB}{\hbar c} y \right)^{2} \psi + \left( \frac{\partial}{\partial y} + \frac{1}{2} \frac{ieB}{\hbar c} x \right)^{2} \psi + \frac{\partial^{2}}{\partial z^{2}} \psi - \frac{eB}{\hbar c} \sigma_{z} \psi = 0
$$

Taking a representation with $\sigma_{z}$ diagonal, this splits at once into two states with $\sigma_{z} = \pm$1. Also

$$
L_{z} = - i \hbar \left\{ x \frac{\partial}{\partial y} - y \frac{\partial}{\partial x} \right\}
$$

<a id="p19"><sup>[p. 19]</sup></a>

is a constant of the motion, say $L_{z} = \ell \hbar$ where $\ell$ is an integer. And $- i \hbar \frac{\partial}{\partial z} = p_{z}$. Let $\lambda = | eB \hbar c |$. Then

$$
\left\{ E^{2} - m^{2} c^{4} - c^{2} p_{z}^{2} \pm ( \ell_{z} \pm 1 ) \lambda \right\} \psi = \hbar^{2} c^{2} \left\{ \tfrac{1}{4} \frac{\lambda^{2} r^{2}}{\hbar^{4} c^{4}} - \left( \frac{\partial^{2}}{\partial x^{2}} + \frac{\partial^{2}}{\partial y^{2}} \right) \right\} \psi
$$

This is an eigenvalue problem with eigenvalues of a two-dimensional harmonic oscillator.

Thus

$$
E^{2} = m^{2} c^{4} + c^{2} p_{z}^{2} + \lambda \{ n \pm ( \ell_{z} \pm 1 ) \}
$$

$$
\text{ where } \ell_{z} = 0 , \pm 1 , \pm 2 ,\dots , \pm ( n - 1 ) .
$$

So the eigenvalues are

$$
E = \sqrt{m^{2} c^{4} + c^{2} p_{z}^{2} + M | eB \hbar c |} \quad \text{ with } M = 0 , 1 , 2 ,\dots
$$

The lowest state has energy exactly $mc^{2}$.

<a id="p20"><sup>[p. 20]</sup></a>

<div align="center">

## <a id="s-summary-of-matrices-in-the-dirac-theory-in-our-notation"></a>Summary of Matrices in the Dirac Theory in our notation

</div>

$$
\alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} = 2 \delta_{k\ell} \mathbb{I} \quad \alpha^{k} \beta + \beta \alpha^{k} = 0 \quad \beta^{2} = \mathbb{I} \quad \sigma_{k} \sigma_{\ell} + \sigma_{\ell} \sigma_{k} = 2 \delta_{k\ell} \mathbb{I}
$$

$$
\gamma_{k} = - i\beta \alpha^{k} \quad \alpha^{k} = i\beta \gamma_{k} \quad \gamma_{4} = \beta \quad \gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu} = 2 \delta_{\mu \nu} \mathbb{I} \quad ( \gamma_{k} )^{*} = \gamma_{k}
$$

$$
\alpha^{k} \gamma_{\ell} - \gamma_{\ell} \alpha^{k} = 2 i\delta_{\ell k} \beta \quad \gamma_{5} = \gamma_{1} \gamma_{2} \gamma_{3} \gamma_{4} \quad \gamma_{\mu} \gamma_{5} + \gamma_{5} \gamma_{\mu} = 0 \quad \alpha^{k} \gamma_{5} - \gamma_{5} \alpha^{k} = 0 \quad \gamma_{5}^{2} = \mathbb{I}
$$

We use the following representation:

$$
\sigma_{1} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \sigma_{2} = \begin{pmatrix} 0 & - i \\ i & 0 \end{pmatrix} \quad \sigma_{3} = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix}
$$

$$
\alpha^{k} = \begin{pmatrix} \mathbb{O} & \sigma_{k} \\ \sigma_{k} & \mathbb{O} \end{pmatrix} \quad i . e . \quad \alpha^{1} = \begin{pmatrix} 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0 \\ 0 & 1 & 0 & 0 \\ 1 & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{2} = \begin{pmatrix} 0 & 0 & 0 & - i \\ 0 & 0 & i & 0 \\ 0 & - i & 0 & 0 \\ i & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{3} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & - 1 \\ 1 & 0 & 0 & 0 \\ 0 & - 1 & 0 & 0 \end{pmatrix}
$$

$$
\beta = \begin{pmatrix} \mathbb{I} & \mathbb{O} \\ \mathbb{O} & - \mathbb{I} \end{pmatrix} \quad i . e . \quad \beta = \gamma_{4} = \begin{pmatrix} 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & - 1 & 0 \\ 0 & 0 & 0 & - 1 \end{pmatrix} \quad \gamma_{5} = \begin{pmatrix} \mathbb{O} & - \mathbb{I} \\ - \mathbb{I} & \mathbb{O} \end{pmatrix} = \begin{pmatrix} 0 & 0 & - 1 & 0 \\ 0 & 0 & 0 & - 1 \\ - 1 & 0 & 0 & 0 \\ 0 & - 1 & 0 & 0 \end{pmatrix}
$$

$$
\gamma_{k} = \begin{pmatrix} \mathbb{O} & - i\sigma_{k} \\ i\sigma_{k} & \mathbb{O} \end{pmatrix} \quad i . e . \quad \gamma_{1} = \begin{pmatrix} 0 & 0 & 0 & - i \\ 0 & 0 & - i & 0 \\ 0 & i & 0 & 0 \\ i & 0 & 0 & 0 \end{pmatrix} \quad \gamma_{2} = \begin{pmatrix} 0 & 0 & 0 & - 1 \\ 0 & 0 & 1 & 0 \\ 0 & 1 & 0 & 0 \\ - 1 & 0 & 0 & 0 \end{pmatrix} \quad \gamma_{3} = \begin{pmatrix} 0 & 0 & - i & 0 \\ 0 & 0 & 0 & i \\ i & 0 & 0 & 0 \\ 0 & - i & 0 & 0 \end{pmatrix}
$$

$$
\sigma_{k} = \epsilon \alpha^{k} \quad \alpha^{k} = \epsilon \sigma_{k} \quad \eta = i\epsilon \beta \quad \epsilon = - i\alpha^{1} \alpha^{2} \alpha^{3} \quad \epsilon^{2} = \eta^{2} = \mathbb{I} \quad \gamma_{5} = - \epsilon
$$

$$
\sigma_{k} = \eta \gamma_{k} \quad \gamma_{k} = \eta \sigma_{k} \quad \epsilon = - i\eta \beta \quad \eta = - \alpha^{1} \alpha^{2} \alpha^{3}
$$

$$
\epsilon = \begin{pmatrix} \mathbb{O} & \mathbb{I} \\ \mathbb{I} & \mathbb{O} \end{pmatrix} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \\ 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \end{pmatrix} \quad \eta = \begin{pmatrix} \mathbb{O} & - i \mathbb{I} \\ i \mathbb{I} & \mathbb{O} \end{pmatrix} = \begin{pmatrix} 0 & 0 & - i & 0 \\ 0 & 0 & 0 & - i \\ i & 0 & 0 & 0 \\ 0 & i & 0 & 0 \end{pmatrix}
$$

$$
\alpha^{k} \sigma_{\ell} + \sigma_{\ell} \alpha^{k} = 2 \delta_{\ell k} \epsilon \quad \gamma_{k} \sigma_{\ell} + \sigma_{\ell} \gamma_{k} = 2 \delta_{\ell k} \eta \quad \beta \sigma_{k} - \sigma_{k} \beta = 0
$$

$$
\sigma_{k} \sigma_{\ell} = \alpha_{k} \alpha_{\ell} = \gamma_{k} \gamma_{\ell} = i\sigma_{m} \quad k,\ell ,m = ( 1 , 2 , 3 ) \text{ cyclicly permuted }
$$

$$
\alpha^{k} \epsilon - \epsilon \alpha^{k} = \gamma_{\mu} \epsilon + \epsilon \gamma_{\mu} = \sigma_{k} \epsilon - \epsilon \sigma_{k} = 0
$$

$$
\alpha^{k} \eta + \eta \alpha^{k} = \gamma_{k} \eta - \eta \gamma_{k} = \sigma_{k} \eta - \eta \sigma_{k} = \beta \eta + \eta \beta = 0
$$

$$
\left. \begin{aligned} \alpha_{k} \sigma_{\ell} &= i\alpha_{m} \\ \sigma_{k} \gamma_{\ell} &= i\gamma_{m} \\ \gamma_{k} \alpha_{\ell} &= \beta \sigma_{m} \end{aligned} \right\} \quad k,\ell ,m = ( 1 , 2 , 3 ) \text{ cyclicly permuted }
$$

Comparison with the Dirac notation: $\rho_{1} = \epsilon \quad \rho_{2} = \eta \quad \rho_{3} = \beta$.

Latin indices: 1, 2, 3. Greek indices: 1, 2, 3, 4.

<a id="p21"><sup>[p. 21]</sup></a>

<div align="center">

## <a id="s-summary-of-matrices-in-the-dirac-theory-in-the-feynman-notation"></a>Summary of Matrices in the Dirac Theory in the Feynman notation

</div>

$$
\alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} = 2 \delta_{k\ell} \mathbb{I} \quad \alpha^{k} \beta + \beta \alpha^{k} = 0 \quad g_{0 0} = + 1 \quad g_{kk} = - 1 \quad g_{\mu \nu} = 0 ,\mu \neq \nu
$$

$$
\sigma_{k} \sigma_{\ell} + \sigma_{\ell} \sigma_{k} = 2 \delta_{k\ell} \mathbb{I} \quad \beta^{2} = \mathbb{I}
$$

$$
\gamma_{k} = \beta \alpha^{k} \quad \alpha^{k} = \beta \gamma_{k} \quad \gamma_{0} = \beta \quad \gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu} = 2 g_{\mu \nu} \mathbb{I} \quad ( \gamma_{k} )^{*} = - \gamma_{k}
$$

$$
\alpha^{k} \gamma_{\ell} - \gamma_{\ell} \alpha^{k} = - 2 \delta_{\ell k} \beta \quad \gamma_{5} = i\gamma_{0} \gamma_{1} \gamma_{2} \gamma_{3} \quad \gamma_{\mu} \gamma_{5} + \gamma_{5} \gamma_{\mu} = 0 \quad \alpha^{k} \gamma_{5} - \gamma_{5} \alpha^{k} = 0 \quad \gamma_{5}^{2} = - \mathbb{I}
$$

Representation:

$$
\sigma_{1} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \sigma_{2} = \begin{pmatrix} 0 & - i \\ i & 0 \end{pmatrix} \quad \sigma_{3} = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix}
$$

$$
\alpha^{k} = \begin{pmatrix} \mathbb{O} & \sigma_{k} \\ \sigma_{k} & \mathbb{O} \end{pmatrix} \quad i . e . \quad \alpha^{1} = \begin{pmatrix} 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0 \\ 0 & 1 & 0 & 0 \\ 1 & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{2} = \begin{pmatrix} 0 & 0 & 0 & - i \\ 0 & 0 & i & 0 \\ 0 & - i & 0 & 0 \\ i & 0 & 0 & 0 \end{pmatrix} \quad \alpha^{3} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & - 1 \\ 1 & 0 & 0 & 0 \\ 0 & - 1 & 0 & 0 \end{pmatrix}
$$

$$
\beta = \begin{pmatrix} \mathbb{I} & \mathbb{O} \\ \mathbb{O} & - \mathbb{I} \end{pmatrix} \quad i . e . \quad \beta = \gamma_{0} = \begin{pmatrix} 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \\ 0 & 0 & - 1 & 0 \\ 0 & 0 & 0 & - 1 \end{pmatrix}
$$

$$
\gamma_{k} = \begin{pmatrix} \mathbb{O} & \sigma_{k} \\ - \sigma_{k} & \mathbb{O} \end{pmatrix} \quad i . e . \quad \gamma_{1} = \begin{pmatrix} 0 & 0 & 0 & 1 \\ 0 & 0 & 1 & 0 \\ 0 & - 1 & 0 & 0 \\ - 1 & 0 & 0 & 0 \end{pmatrix} \quad \gamma_{2} = \begin{pmatrix} 0 & 0 & 0 & - i \\ 0 & 0 & i & 0 \\ 0 & - i & 0 & 0 \\ i & 0 & 0 & 0 \end{pmatrix} \quad \gamma_{3} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & - 1 \\ - 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \end{pmatrix}
$$

$$
\rho_{1} = \begin{pmatrix} \mathbb{O} & \mathbb{I} \\ \mathbb{I} & \mathbb{O} \end{pmatrix} = \begin{pmatrix} 0 & 0 & 1 & 0 \\ 0 & 0 & 0 & 1 \\ 1 & 0 & 0 & 0 \\ 0 & 1 & 0 & 0 \end{pmatrix} = \gamma_{5} \quad \rho_{2} = \begin{pmatrix} \mathbb{O} & - i \mathbb{I} \\ i \mathbb{I} & \mathbb{O} \end{pmatrix} = \begin{pmatrix} 0 & 0 & - i & 0 \\ 0 & 0 & 0 & - i \\ i & 0 & 0 & 0 \\ 0 & i & 0 & 0 \end{pmatrix}
$$

$$
\sigma_{k} = \rho_{1} \alpha^{k} \quad \alpha^{k} = \rho_{1} \sigma_{k} \quad \rho_{2} = i\rho_{1} \beta \quad \rho_{1} = - i\alpha^{1} \alpha^{2} \alpha^{3} \quad \rho_{1}^{2} = \rho_{2}^{2} = \mathbb{I}
$$

$$
\sigma_{k} = - i\rho_{2} \gamma_{k} \quad \gamma_{k} = i\rho_{2} \sigma_{k} \quad \rho_{1} = - i\rho_{2} \beta \quad \rho_{2} = - \alpha^{1} \alpha^{2} \alpha^{3} \beta
$$

$$
\alpha^{k} \sigma_{\ell} + \sigma_{\ell} \alpha^{k} = 2 \delta_{\ell k} \rho_{1} \quad \gamma_{k} \sigma_{\ell} + \sigma_{\ell} \gamma_{k} = - 2 \delta_{\ell k} \rho_{2} \quad \beta \sigma_{k} - \sigma_{k} \beta = 0
$$

$$
\sigma_{k} \sigma_{\ell} = \alpha_{k} \alpha_{\ell} = - \gamma_{k} \gamma_{\ell} = i\sigma_{m} \quad k,\ell ,m = ( 1 , 2 , 3 ) \text{ cyclicly permuted }
$$

$$
\alpha^{k} \rho_{1} - \rho_{1} \alpha^{k} = \gamma_{\mu} \rho_{1} + \rho_{1} \gamma_{\mu} = \sigma_{k} \rho_{1} - \rho_{1} \sigma_{k} = 0
$$

$$
\alpha^{k} \rho_{2} + \rho_{2} \alpha^{k} = \gamma_{k} \rho_{2} - \rho_{2} \gamma_{k} = \sigma_{k} \rho_{2} - \rho_{2} \sigma_{k} = \beta \rho_{2} + \rho_{2} \beta = 0
$$

$$
\left. \begin{aligned} \alpha_{k} \sigma_{\ell} &= i\alpha_{m} \\ \sigma_{k} \gamma_{\ell} &= i\gamma_{m} \\ \gamma_{k} \alpha_{\ell} &= i\beta \sigma_{m} \end{aligned} \right\} \quad k,\ell ,m = ( 1 , 2 , 3 ) \text{ cyclicly permuted }
$$

Latin indices: 1, 2, 3. Greek indices: 0, 1, 2, 3.

---

## Notes

<a id="fn-4"></a>**4.** “differential” replaces “diff.” [↩](#fnref-4)

<a id="fn-5"></a>**5.** In the second edition, this equation lacked a label. [↩](#fnref-5)

<a id="fn-6"></a>**6.** In the first edition, the coefficient of $\boldsymbol{\alpha} \boldsymbol{\cdot} \nabla$ is +$i \hbar c$ [↩](#fnref-6)

<a id="fn-7"></a>**7.** $E$ lacked the exponent 2 in Eq. (47). [↩](#fnref-7)

<a id="fn-8"></a>**8.** $e_{s}$ lacked a subscript $s$ [↩](#fnref-8)

<a id="fn-9"></a>**9.** The quantity $\left( \left( mc^{2} - E \right)^{2} - \left( m^{2} c^{4} - E^{2} \right) \right)$ lacked an exponent 2. [↩](#fnref-9)

---

[← Introduction](01-introduction.md) · [Scattering Problems and Born Approximation →](03-scattering-problems-and-born-approximation.md)
