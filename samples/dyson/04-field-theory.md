# Field Theory

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p32"><sup>[p. 32]</sup></a>

Before we can begin on the program of constructing our quantum theory of fields, we must make some remarks about Classical Field Theory.

## <a id="s-classical-relativistic-field-theory"></a>Classical Relativistic Field Theory

We take a field with components (vector, spinor etc.) labeled by a suffix $\alpha$. Let

<a id="eq-155"></a>

$$
\varphi_{\mu}^{\alpha} = \frac{\partial \varphi^{\alpha}}{\partial x_{\mu}} \tag{155}
$$

The theory is fully described by an invariant function of position called the <a id="ix-137">Lagrangian Density</a>,

<a id="eq-156"></a>

$$
L = L \left( \varphi^{\alpha} ( x ) ,\varphi_{\mu}^{\alpha} ( x ) \right) , \tag{156}
$$

a function of $\varphi^{\alpha}$ and its _first_ derivatives at the point $x$. The behaviour of the field is fixed by the <a id="ix-1"><em>Action</em></a> _Principle_. If Ω is any finite or infinite region of space-time, then

<a id="eq-157"></a>

$$
I ( \Omega ) = \frac{1}{c} \int_{\Omega} L d^{4} x \tag{157}
$$

is stationary for the physically possible fields $\varphi^{\alpha}$. Thus the variation $\phi^{\alpha} \to \varphi^{\alpha} + \delta \varphi^{\alpha}$ produces no change in $I$ to first order in $\delta \varphi^{\alpha}$, if $\delta \varphi^{\alpha}$ is an arbitrary variation equal to zero on the boundary of Ω.

It is always assumed that $L$ is at most quadratic in the $\varphi_{\mu}^{\alpha}$ and is in various other respects a well-behaved function.

Let Σ be the boundary of Ω, and $d\sigma$ an element of 3–dimensional volume on Σ, $n_{\mu}$ the outward unit vector normal to $d\sigma$, and

<a id="eq-158"></a>

$$
\begin{aligned} d\sigma_{\mu} &= n_{\mu} d\sigma , \sum_{\mu} n_{\mu}^{2} = - 1 \quad \mu = 1 , 2 , 3 , 4 \quad x_{0} = ct \\ d\sigma_{\mu} &= ( dx_{2} dx_{3} dx_{0} ,dx_{1} dx_{3} dx_{0} ,dx_{1} dx_{2} dx_{0} , - idx_{1} dx_{2} dx_{3} ) \end{aligned} \tag{158}
$$

Then

<a id="eq-159"></a>

$$
\begin{aligned} c\delta I ( \Omega ) &= \int_{\Omega} \sum_{\alpha} \left( \frac{\partial L}{\partial \varphi^{\alpha}} \delta \varphi^{\alpha} + \sum_{\mu} \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \delta \varphi_{\mu}^{\alpha} \right) d^{4} x \\ &= \int_{\Omega} \sum_{\alpha} \left\{ \frac{\partial L}{\partial \varphi^{\alpha}} - \sum_{\mu} \frac{\partial}{\partial x_{\mu}} \left( \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \right) \right\} \delta \varphi^{\alpha} d^{4} x + \int_{\Sigma \alpha ,\mu} \sum n_{\mu} \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \delta \varphi^{\alpha} d\sigma \end{aligned} \tag{159}
$$

So the principle of action gives the field equations

<a id="eq-160"></a>

$$
\frac{\partial L}{\partial \varphi^{\alpha}} - \sum_{\mu} \frac{\partial}{\partial x_{\mu}} \left( \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \right) = 0 \tag{160}
$$

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p33"><sup>[p. 33]</sup></a>

defining the motion of the fields.

The quantity

<a id="eq-161"></a>

$$
\pi_{\alpha} = \frac{1}{c} \sum_{\mu} n_{\mu} \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \tag{161}
$$

is the momentum conjugate to $\varphi^{\alpha}$, defined at $x$ and with respect to the surface Σ.

A more general type of variation is made by varying not only the $\varphi^{\alpha}$ but also the boundary of Ω, each point $x_{\mu}$ being moved to the position $( x_{\mu} + \delta x_{\mu} )$ where $\delta x_{\mu}$ is either constant or may vary over the surface. Writing ${}_{N} \varphi^{\alpha}$ for the new $\varphi^{\alpha}$ and ${}_{O} \varphi^{\alpha}$ for the old one, we have

<a id="eq-162"></a>

$$
\begin{aligned} \delta \varphi^{\alpha} ( x ) &= {}_{N} \varphi^{\alpha} ( x + \delta x ) - {}_{O} \varphi^{\alpha} ( x ) \\ {}_{O} \varphi^{\alpha} ( x + \delta x ) &= {}_{O} \varphi^{\alpha} ( x ) + \sum_{\mu} \delta x_{\mu O} \varphi_{\mu}^{\alpha} ( x ) \\ \Delta \varphi^{\alpha} ( x ) &= {}_{N} \varphi^{\alpha} ( x ) - {}_{O} \varphi^{\alpha} ( x ) \end{aligned} \tag{162}
$$

Therefore under the joint variation

$$
c\delta I ( \Omega ) = \int_{\Omega_{N}} L \left( {}_{N} \varphi^{\alpha} ( x ) , {}_{N} \varphi_{\mu}^{\alpha} ( x ) \right) d^{4} x - \int_{\Omega_{O}} L \left( {}_{O} \varphi^{\alpha} ( x ) , {}_{O} \varphi_{\mu}^{\alpha} ( x ) \right) d^{4} x
$$

$$
= \left\{ \int_{\Omega_{N}} - \int_{\Omega_{O}} \right\} L \left( {}_{N} \varphi^{\alpha} ( x ) , {}_{N} \varphi_{\mu}^{\alpha} ( x ) \right) d^{4} x - \int_{\Omega_{O}} \left\{ L \left( {}_{N} \varphi^{\alpha} ( x ) , {}_{N} \varphi_{\mu}^{\alpha} ( x ) \right) - L \left( {}_{O} \varphi^{\alpha} ( x ) , {}_{O} \varphi_{\mu}^{\alpha} ( x ) \right) \right\} d^{4} x
$$

$$
= \int_{\Sigma \alpha ,\mu} \sum n_{\mu} \delta x_{\mu} L \left( {}_{N} \varphi^{\alpha} ( x ) , {}_{N} \varphi_{\mu}^{\alpha} ( x ) \right) d\sigma + c \int_{\Sigma} \sum_{\alpha} \pi_{\alpha} ( x ) \Delta \varphi^{\alpha} ( x ) d\sigma
$$

the latter being true by (159) if we assume (160).

Now since by (162)

$$
\delta \varphi^{\alpha} ( x ) = {}_{N} \varphi^{\alpha} ( x ) + \sum_{\mu} \delta x_{\mu N} \varphi_{\mu}^{\alpha} ( x ) - {}_{O} \varphi^{\alpha} ( x ) = \Delta \varphi^{\alpha} + \sum_{\mu} \delta x_{\mu N} \varphi_{\mu}^{\alpha} ( x )
$$

hence we get finally

<a id="eq-163"></a>

$$
\delta I ( \Omega ) = \int_{\Omega \alpha ,\mu} \sum \left\{ \pi_{\alpha} \delta \varphi^{\alpha} + \left( \frac{1}{c} n_{\mu} L - \varphi_{\mu}^{\alpha} \pi^{\alpha} \right) \delta x_{\mu} \right\} d\sigma \tag{163}
$$

with all the new quantities on the RHS.

In the case which is physically of importance, the actual motion is fixed uniquely by specifying the values of the $\varphi^{\alpha}$ everywhere on two space-time surfaces $\sigma_{2}$ and $\sigma_{1}$ which are the past and future boundaries of the volume Ω. A space-like surface is one on which every two points are outside each other’s light-cones, so that the fields can be fixed independently at every point.

Special case of non-relativistic theory, both $\sigma_{1}$ and $\sigma_{2}$ are just space at the time $t_{1}$ and $t_{2}$, and $\delta x_{\mu}$ is $ic$ times a displacement of the time by $\delta t_{1}$ and $\delta t_{2}$. Then we may write $n_{\mu}$ = (0$, 0 , 0 ,i ) , \pi_{\alpha} = \partial L /\partial \dot{\varphi}^{\alpha}$, and then for the <a id="ix-103">Hamiltonian</a>

<a id="eq-164"></a>

$$
H = \int d\tau \left( \sum_{\alpha} \pi_{\alpha} \dot{\varphi}^{\alpha} - L \right) \tag{164}
$$

and thus

<a id="eq-165"></a>

$$
\delta I ( \Omega ) = \int d\tau \sum_{\alpha} \{ ( \pi_{\alpha} \delta \varphi^{\alpha} ) ( t_{1} ) - ( \pi_{\alpha} \delta \varphi^{\alpha} ) ( t_{2} ) \}-\{ H ( t_{1} ) \delta t_{1} - H ( t_{2} ) \delta t_{2} \} \tag{165}
$$

The essential feature of this classical theory is that the <a id="ix-2">Action</a> Principle is stated only for variations vanishing on the boundary of Ω. From this one can _deduce_ as in (163) and (165) the effect on $I$(Ω) of variations not vanishing on the boundary. This is possible because each state of motion is defined by fixing _as many_ field quantities as can be fixed independently, (e.g. all the fields on two space-like surfaces or all

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p34"><sup>[p. 34]</sup></a>

the fields and their time-derivatives on one surface) and then the whole past and future of the motion is determined by the <a id="ix-62">field</a> equations.

Field equations can be written in the Hamiltonian form

$$
\dot{\varphi}^{\alpha} = \frac{\partial H}{\partial \pi_{\alpha}} , \quad \dot{\pi}_{\alpha} = - \frac{\partial H}{\partial \varphi^{\alpha}}
$$

## <a id="s-examples"></a>Examples:

1. <a id="ix-140">Klein-Gordon</a> Field, real

<a id="eq-166"></a>

$$
L_{K} = - \tfrac{1}{2} c^{2} \left\{ \sum_{\mu} \left( \frac{\partial \psi}{\partial x_{\mu}} \right)^{2} + \mu^{2} \psi^{2} \right\} \tag{166}
$$

2. Klein-Gordon Field, complex

<a id="eq-167"></a>

$$
L_{K}^{\prime} = - c^{2} \left\{ \sum_{\mu} \left( \frac{\partial \psi}{\partial x_{\mu}} \frac{\partial \psi^{*}}{\partial x_{\mu}} \right) + \mu^{2} \psi \psi^{*} \right\} \tag{167}
$$

where we consider $\psi$ and $\psi^{*}$ independent one-component fields.

3. <a id="ix-143">Maxwell</a> Field, four component $A_{\mu}$, <a id="ix-122">Fermi</a> form,

<a id="eq-168"></a>

$$
L_{M} = - \tfrac{1}{4} \sum_{\mu ,\nu} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right)^{2} - \tfrac{1}{2} \sum_{\mu} \left( \frac{\partial A_{\mu}}{\partial x_{\mu}} \right)^{2} \tag{168}
$$

4. <a id="ix-138">Dirac</a> Field

<a id="eq-169"></a>

$$
L_{D} = - \hbar c \overline{\psi} \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) \psi \quad \mu = \frac{mc}{\hbar} \tag{169}
$$

5. Dirac Field interacting with Maxwell Field

<a id="eq-170"></a>

$$
L_{Q} = L_{D} + L_{M} - \sum_{\lambda} ieA_{\lambda} \overline{\psi} \gamma_{\lambda} \psi \tag{170}
$$

here $Q$ stands for <a id="ix-215">quantum electrodynamics</a>.

_Problem 5._ Work out these examples: find the field equations, the momentum conjugate to each component of the field, and the Hamiltonian function, (the momenta and Hamiltonian defined for the case of a flat space $\sigma$ only). Verify that the Hamiltonian gives a correct canonical representation of the field equations as Hamiltonian equations of motion.

## <a id="s-quantum-relativistic-field-theory"></a>Quantum Relativistic Field Theory

The classical relativistic field theories were usually quantized by using the Hamiltonian form of the field equations and bringing in the commutation relations between coordinates and momenta taken from non-relativistic quantum mechanics. For this approach see <a id="ix-222">Wentzel</a>’s book. It is a very bad method, it is complicated; and it is not at all obvious or even easy to prove that the theory so made is relativistic, because the whole Hamiltonian approach is non-covariant.

Just recently we learnt a much better way of doing it, which I shall now expound in these lectures. It is due to <a id="ix-78">Feynman</a> and <a id="ix-243">Schwinger</a>.<sup><a id="fnref-16" href="#fn-16">16</a></sup>

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p35"><sup>[p. 35]</sup></a>

References: R. P. Feynman, _Rev. Mod. Phys._ **20** (1948) 367

_Phys. Rev._ **80** (1950) 440

J. Schwinger, _Phys. Rev._ **82** (1951) 914

It is relativistic all the way, and it is much simpler than the old methods. It is based directly on the Action Principle form of the classical theory which I have just given you, not the Hamiltonian form.

In the quantum theory the $\varphi^{\alpha}$ are operators defined at each point of space-time as before. They satisfy the same field equations as before, and this is ensured if we assume that the Action Principle

<a id="eq-171"></a>

$$
\begin{aligned} \delta I ( \Omega ) &= 0 \\ I ( \Omega ) &= \frac{1}{c} \int_{\Omega} L \left( \varphi^{\alpha} ,\varphi_{\mu}^{\alpha} \right) d^{4} x \end{aligned} \tag{171}
$$

holds for all variations $\delta \varphi^{\alpha}$ of the operators vanishing on the boundaries of Ω.

In the quantum theory, because of complementarity relations, it is not possible to give numerical values to all field operators throughout a physical motion. In fact the state of motion is specified by giving numerical values to the $\varphi^{\alpha}$ on _one_ space-like surface. The future of the state of motion cannot then be determined from the field equations, which are in general second-order differential equations. Therefore the action principle (171) which was enough for the classical theory is no longer enough. We must make some _additional_ statement about the behaviour of $\delta I$ for variations $\delta \varphi^{\alpha}$ which are not zero on the boundaries of Ω.

A state of motion is specified by specifying a space-time surface $\sigma$ and a set of numerical values $\varphi^{\prime \alpha}$ for the eigenvalue which the operators $\varphi^{\alpha}$ on $\sigma$ have in this state. The state is denoted by the Dirac <a id="ix-66">ket</a> vector $| \varphi^{\prime \alpha} ,\sigma \rangle$. This is a special kind of state in which the $\varphi^{\alpha}$ on $\sigma$ have eigenvalues: the general state is a linear combination of $| \varphi^{\prime \alpha} ,\sigma \rangle$ with various values of $\varphi^{\prime \alpha}$. The physically observable quantities are expressions such as the matrix element

<a id="eq-172"></a>

$$
\left\langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi^{\beta} ( x ) \varphi_{2}^{\prime \alpha} ,\sigma_{2} \right\rangle \tag{172}
$$

of the field operator $\varphi^{\beta} ( x )$ between the two states specified by $\varphi_{1}^{\prime \alpha}$ on $\sigma_{1}$ and by $\varphi_{2}^{\prime \alpha}$ on $\sigma_{2}$. In particular, the <a id="ix-19">transition</a> <a id="ix-169">probability</a> <a id="ix-13">amplitude</a> between the two states is

<a id="eq-173"></a>

$$
\langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle \tag{173}
$$

The squared modulus of this gives the probability of finding the values $\varphi_{1}^{\prime \alpha}$ for the fields on $\sigma_{1}$, in the motion which is defined by the fields being given the definite values $\varphi_{2}^{\prime \alpha}$ on $\sigma_{2}$.

## <a id="s-the-feynman-method-of-quantization"></a>The Feynman Method of <a id="ix-72">Quantization</a>

The Feynman method of quantizing the theory consists in writing down an explicit formula for the transition amplitude (173). Namely

<a id="eq-174"></a>

$$
\langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle = N \sum_{H} \exp \left\{ \frac{i}{\hbar} I_{H} ( \Omega ) \right\} \tag{174}
$$

Here $H$ represents a History of the fields between $\sigma_{2}$ and $\sigma_{1}$, i.e. any set of classical functions $\varphi^{\alpha} ( x )$ which are defined in the region Ω between $\sigma_{2}$ and $\sigma_{1}$ and which take the values $\varphi_{1}^{\prime \alpha}$ on $\sigma_{1}$ and $\varphi_{2}^{\prime \alpha}$ on $\sigma_{2} . I_{H}$(Ω) is the value of $I$(Ω) calculated with these particular functions. The sum $\sum H$ is taken over all possible histories, a continuously infinite sum whose exact mathematical definition is not easy to formulate. $N$ is a normalization factor independent of the particular states considered, chosen so as to make the sum of the squares of the amplitudes from a given state to all other states equal to 1. This formula is derived by Feynman from very general considerations, applying a Huyghens principle to the solution of wave-mechanics just as it is done in wave-optics. By this one formula the whole theory is quantized and the answer to any

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p36"><sup>[p. 36]</sup></a>

physical problem in principle given. The method applies not only to field theory but to ordinary NR quantum theory too. We do not try to derive or justify the Feynman formula here. We just show that it gives the same results as the usual QM. For a discussion of the difficulties in defining the sum $\sum H$, and a method of doing it in simple cases, see C. <a id="ix-133">Morette</a>, _Phys. Rev._ **81** (1951) 848.

From formula (174) we derive at once the most general _Correspondence Principle_ giving us back the classical theory in the limit as $\hbar \to$ 0. For suppose $\hbar \to$ 0 then the exponential factor in (174) becomes an extremely rapidly oscillating function of $H$ for all histories $H$ _except_ that one for which $I$(Ω) is stationary. Therefore in the limit the sum $\sum H$ reduces to the contribution from the classical motion leading from $\varphi_{2}^{\prime \alpha}$ on $\sigma_{2}$ to $\varphi_{1}^{\prime \alpha}$ on $\sigma_{1}$, all other contributions interfering destructively. The classical motion is defined by the condition that<sup><a id="fnref-17" href="#fn-17">17</a></sup> its $\delta I$(Ω) = 0 for all small variations of the $\varphi^{\alpha}$ between $\sigma_{2}$ and $\sigma_{1}$. This passage to the classical theory is precisely analogous to the passage from wave-optics to geometrical optics when the wave-length of light is allowed to tend to zero. The <a id="ix-223">WKB approximation</a> is gotten by taking $\hbar$ small but not quite zero.

To establish a connection between the Feynman method and the ordinary method of <a id="ix-73">quantization</a>, Feynman has to define what he means by an operator in his formulation. This he does as follows: Let $x$ be any space-time point inside Ω. Let $\mathcal{O} ( x )$ be any field operator defined at $x$, for example $\varphi^{\beta} ( x )$ or $\varphi_{\mu}^{\beta} ( x )$. Then $\mathcal{O} ( x )$ is given a meaning by defining its matrix element between the states $| \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle$ and $| \varphi_{1}^{\prime \alpha} ,\sigma_{1} \rangle$, where $\sigma_{2}$ and $\sigma_{1}$ are any two surfaces to the past and future of $x$. This matrix element is

<a id="eq-175"></a>

$$
\langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \mathcal{O} ( x ) \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle = N \sum_{H} \mathcal{O}_{H} ( x ) \exp \left\{ \frac{i}{\hbar} I_{H} ( \Omega ) \right\} \tag{175}
$$

The number $\mathcal{O} H$ is just the value which the expression $\mathcal{O}$ takes when the $\varphi^{\alpha}$ are given the values which they have in the history $H$. It is easily verified that the definitions (174) and (175) are physically reasonable and give the right formal properties of transition <a id="ix-14">amplitudes</a> and operator matrix elements.

The Feynman method has one fatal drawback: we cannot use it until we have some way of calculating or at least using the sums over histories, and so far nobody has suggested a practical way of doing this. But Schwinger has shown how to derive from the Feynman method an <a id="ix-3">Action</a> Principle formulation of the theory which avoids this difficulty.

## <a id="s-the-schwinger-action-principle"></a>The Schwinger Action Principle

Let the sets of eigenvalues $\varphi_{1}^{\prime \alpha}$ and $\varphi_{2}^{\prime \alpha}$ in (174) be held fixed. Let the numbers $\varphi_{H}^{\alpha} ( x )$ be varied so that $\varphi_{H}^{\alpha} ( x )$ is replaced by $\varphi_{H}^{\alpha} ( x ) + \delta \varphi^{\alpha} ( x )$ where $\delta \varphi^{\alpha} ( x )$ is an arbitrary infinitesimal c-number quantity. Let the surfaces $\sigma_{1}$ and $\sigma_{2}$ be varied so that the point $x_{\mu}$ moves to $x_{\mu} + \delta x_{\mu}$. And let the function $L$ also be varied so that it is replaced by $L + \delta L$ where $\delta L$ is any expression involving the $\varphi^{\alpha}$ and $\varphi_{\mu}^{\alpha}$. Under this triple variation (174) gives

<a id="eq-176"></a>

$$
\delta \langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle = N \sum_{H} \left\{ \frac{i}{\hbar} \delta I_{H} ( \Omega ) \exp \left( \frac{i}{\hbar} I_{H} ( \Omega ) \right) \right\} \tag{176}
$$

Using (175) this may be written

<a id="eq-177"></a>

$$
\delta \langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle = \frac{i}{\hbar} \langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \delta I ( \Omega ) \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle . \tag{177}
$$

Here $\delta I$(Ω) is the operator obtained by making the three variations on the operator $I$(Ω). Formally $\delta I$(Ω) is the same as the variation obtained in the classical theory,

<a id="eq-178"></a>

$$
\begin{aligned} \delta I ( \Omega ) &= \frac{1}{c} \int_{\Omega} \left\{ \delta L + \sum_{\alpha ,\mu} \left( \frac{\partial L}{\partial \varphi^{\alpha}} - \frac{\partial}{\partial x_{\mu}} \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} \right) \delta \varphi^{\alpha} \right\} d^{4} x \\ & \quad + \left\{ \int_{\sigma_{1}} - \int_{\sigma_{2}} \right\} \sum_{\alpha ,\mu} \left\{ \pi_{\alpha} \delta \varphi^{\alpha} + \left( \frac{1}{c} n_{\mu} L - \varphi_{\mu}^{\alpha} \pi_{\alpha} \right) \delta x_{\mu} \right\} d\sigma \end{aligned} \tag{178}
$$

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p37"><sup>[p. 37]</sup></a>

Only now everything on the RHS of (178) is an operator.

Now what is the meaning of this triple variation as applied to the left side of (174)? Since the $\varphi_{H}^{\alpha} ( x )$ are only variables of summation, the change from $\varphi_{H}^{\alpha} ( x )$ to $\varphi_{H}^{\alpha} ( x ) + \delta \varphi^{\alpha} ( x )$ only affects the left side in changing the boundary values which $\varphi_{H}^{\alpha} ( x )$ must take on $\sigma_{1}$ and $\sigma_{2}$. Thus instead of $\varphi_{H}^{\alpha} ( x ) = \varphi_{1}^{\alpha \prime \prime} ( x )$ on $\sigma_{1}$ we now have the new summation variable

$$
\varphi_{H}^{\alpha} ( x ) + \delta \varphi^{\alpha} = \varphi_{1}^{\alpha \prime \prime} ( x ) + \delta \varphi^{\alpha} \quad \text{ on } \sigma_{1}
$$

Therefore the change in $\varphi_{H}^{\alpha}$ is equivalent simply to changing

$$
\varphi_{1}^{\alpha \prime \prime} \quad \text{ to } \quad \varphi_{1}^{\alpha \prime \prime} + \delta \varphi^{\alpha} \quad \text{ on } \sigma_{1}
$$

$$
\varphi_{2}^{\alpha \prime} \quad \text{ to } \quad \varphi_{2}^{\alpha \prime} + \delta \varphi^{\alpha} \quad \text{ on } \sigma_{2} .
$$

The change in $L$ and in the position of $\sigma$ produces a change in the left side of (174), by virtue of the change in the operators $\varphi^{\alpha}$ on $\sigma_{1}$ and $\sigma_{2}$ resulting from the variations $\delta L$ and $\delta x_{\mu}$ in consequence of the field equations.

Hence the net result of the triple variation on the left side of (174) is to give the change in the matrix element $\langle \varphi_{1}^{\alpha \prime \prime} ,\sigma_{1} \varphi_{2}^{\alpha \prime} ,\sigma_{2} \rangle$ if the $\varphi_{1}^{\alpha \prime \prime}$ and $\varphi_{2}^{\alpha \prime}$ are left fixed, the operators $\varphi^{\alpha} ( x )$ on $\sigma_{1}$ and $\sigma_{2}$ being modified in consequence of the variations $\delta L$ and $\delta x_{\mu}$ according to the field equations, and in addition $\varphi^{\alpha} ( x )$ on $\sigma_{1}$ and $\sigma_{2}$ being changed to $\varphi^{\alpha} ( x ) - \delta \varphi^{\alpha} ( x )$.

Schwinger takes equation (177) as the fundamental principle for setting up the quantum theory. Thus he gets rid of the unpleasant $\sum H$. Out of this <a id="ix-4">action</a> principle come very simply all the main features of a quantum field theory, A – G below.

### <a id="s-a-the-field-equations"></a>A. The Field Equations

If we take the special case of a variation $\delta \varphi^{\alpha}$ which vanishes on the boundary of Ω and $\delta L = \delta x_{\mu}$ = 0, then $\langle \varphi_{1}^{\alpha \prime} ,\sigma_{1} \varphi_{2}^{\alpha \prime} ,\sigma_{2} \rangle$ depends only on the operators $\varphi^{\alpha}$ on $\sigma_{1}$ and $\sigma_{2}$ and is unaffected by the variation. Therefore for all such variations

<a id="eq-171"></a>

$$
\delta I ( \Omega ) = 0 \tag{171}
$$

<a id="eq-179"></a>

$$
\frac{\partial L}{\partial \varphi^{\alpha}} - \sum_{\mu} \frac{\partial}{\partial x_{\mu}} \frac{\partial L}{\partial \varphi_{\mu}^{\alpha}} = 0 \tag{179}
$$

That is to say, the classical action principle and the classical field equations are valid for the quantum field operators.

We see that (177) is exactly the kind of generalization we want of the old variation principle (171). It includes the information, necessary for a quantum theory, concerning the effect on $I$(Ω) of variations not vanishing at the boundary of Ω.

### <a id="s-b-the-schr-dinger-equation-for-the-state-function"></a>B. The Schrödinger Equation for the State-function

Specialize $\sigma_{1}$ and $\sigma_{2}$ to be the whole space at the times $t_{1}$ and $t_{2}$. Then

$$
\langle \varphi_{1}^{\prime \alpha} ,\sigma_{1} \varphi_{2}^{\prime \alpha} ,\sigma_{2} \rangle = \langle \varphi_{1}^{\prime \alpha} ,t_{1} \varphi_{2}^{\prime \alpha} ,t_{2} \rangle = \Psi ( \varphi_{1}^{\prime \alpha} ,t_{1} )
$$

is a Schrödinger wave-function giving the probability amplitude for finding the system in the state $\varphi_{1}^{\prime \alpha}$ at the time $t_{1}$, given the initial conditions $\varphi_{2}^{\prime \alpha}$ at $t_{2}$. The development of Ψ $( \varphi_{1}^{\prime \alpha} ,t_{1} )$ with time $t_{1}$ is thus a description of the development of the state of the system with time in the Schrödinger representation.

Take in (177) a variation in which $\delta \varphi^{\alpha} = \delta L$ = 0, the surface $\sigma_{1}$ being just moved through the displacement $\delta t$ in the time direction. Then using (165) and (164)

$$
\delta \Psi ( \varphi_{1}^{\prime \alpha} ,t_{1} ) = - \frac{i}{\hbar} \langle \varphi_{1}^{\prime \alpha} ,t_{1} H ( t_{1} ) \varphi_{2}^{\prime \alpha} ,t_{2} \rangle \delta t_{1}
$$

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p38"><sup>[p. 38]</sup></a>

or

<a id="eq-180"></a>

$$
i \hbar \frac{d}{dt} \langle \varphi_{1}^{\prime \alpha} ,t_{1} \varphi_{2}^{\prime \alpha} ,t_{2} \rangle = \langle \varphi_{1}^{\prime \alpha} ,t_{1} H ( t_{1} ) \varphi_{2}^{\prime \alpha} ,t_{2} \rangle \tag{180}
$$

This is the ordinary Schrödinger <a id="ix-231">equation</a> in Dirac’s <a id="ix-75">notation</a>. It shows that the Schwinger <a id="ix-238">action principle</a> contains enough information for predicting the future behaviour of a system given initially in a known quantum state.

### <a id="s-c-operator-form-of-the-schwinger-principle"></a>C. Operator Form of the Schwinger Principle

Feynman defined operators by giving the formula (175) for their matrix elements between states specified on two different surfaces. The initial state had to be specified in the past, the final state in the future, the operator referring to some particular time which is taken as present.

The usual and generally more useful way of defining operators is to specify their matrix elements between states defined on the same surface. Thus we are interested in a matrix element

<a id="eq-181"></a>

$$
\langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle \tag{181}
$$

where $\varphi^{\prime \alpha}$ and $\varphi^{\prime \prime \alpha}$ are given sets of eigenvalues and $\sigma$ is a surface which may be past, present or future in relation to the field-points to which $\mathcal{O}$ refers.

Suppose that a reference surface $\sigma_{o}$ is chosen in the remote past. Let the $\varphi^{\alpha} ,\sigma$ and $L$ be varied in such a way that everything on $\sigma_{o}$ remains fixed. For such a variation, (178) gives if we assume that (179) holds

<a id="eq-182"></a>

$$
\delta I ( \Omega ) = \frac{1}{c} \int_{\Omega} \delta L d^{4} x + \int_{\sigma \alpha ,\mu} \sum \left\{ \pi_{\alpha} \delta \varphi^{\alpha} + \left( \frac{1}{c} n_{\mu} L - \varphi_{\mu}^{\alpha} \pi_{\alpha} \right) \delta x_{\mu} \right\} d\sigma \tag{182}
$$

where Ω is the region bounded by $\sigma_{o}$ and $\sigma$. Let us now first calculate the variation of (181) arising from the change in the meaning of the states $| \varphi^{\prime \alpha} ,\sigma \rangle$ and $| \varphi^{\prime \prime \alpha} ,\sigma \rangle$. The operator $\mathcal{O}$ itself is at this point fixed and not affected by the variations in $\varphi^{\alpha} , \sigma$ and $L$ . Then

<a id="eq-183"></a>

$$
\langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle = \sum_{\varphi_{o}^{\prime}} \sum_{\varphi \prime \prime_{o}} \langle \varphi^{\prime \alpha} ,\sigma \varphi_{o}^{\prime \alpha} ,\sigma_{o} \rangle \langle \varphi_{o}^{\prime \alpha} ,\sigma_{o} \mathcal{O} \varphi_{o}^{\prime \prime \alpha} ,\sigma_{o} \rangle \langle \varphi_{o}^{\prime \prime \alpha} ,\sigma_{o} \varphi^{\prime \prime \alpha} ,\sigma \rangle \tag{183}
$$

therefore, denoting

$$
\langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle = \langle \sigma^{\prime} \mathcal{O} \sigma^{\prime \prime} \rangle \quad \text{ etc } . , \text{ we have }
$$

$$
\delta \langle \sigma^{\prime} \mathcal{O} \sigma^{\prime \prime} \rangle = \sum_{\prime} \sum_{\prime \prime} ( \delta \langle \sigma^{\prime} \sigma_{o}^{\prime} \rangle ) \langle \sigma_{o}^{\prime} \mathcal{O} \sigma_{o}^{\prime \prime} \rangle \langle \sigma_{o}^{\prime \prime} \sigma^{\prime \prime} \rangle + \sum_{\prime} \sum_{\prime \prime} \langle \sigma^{\prime} \sigma_{o}^{\prime} \rangle \langle \sigma_{o}^{\prime} \mathcal{O} \sigma_{o}^{\prime \prime} \rangle ( \delta \langle \sigma_{o}^{\prime \prime} \sigma^{\prime \prime} \rangle )
$$

because $| \varphi_{o}^{\prime \alpha} \rangle$ and $| \varphi_{o}^{\prime \prime \alpha} \rangle$ are not changed by the variation, and neither is $\mathcal{O}$. Therefore, using (177) we have

$$
\delta \langle \sigma^{\prime} \mathcal{O} \sigma^{\prime \prime} \rangle = \sum_{\prime} \sum_{\prime \prime} \frac{i}{\hbar} \langle \sigma^{\prime} \delta I_{\sigma - \sigma_{o}} \mathcal{O} \sigma^{\prime \prime} \rangle + \sum_{\prime} \sum_{\prime \prime} \frac{i}{\hbar} \langle \sigma^{\prime} \mathcal{O} \delta I_{\sigma_{o} - \sigma} \sigma^{\prime \prime} \rangle
$$

where the subscript $\sigma - \sigma_{o}$ refers to the surface integrals in (178). Since $\delta I_{\sigma - \sigma_{o}} = - \delta I_{\sigma_{o} - \sigma}$, we get finally

<a id="eq-184"></a>

$$
\delta \langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle = \frac{i}{\hbar} \langle \varphi^{\prime \alpha} ,\sigma \boldsymbol{[} \delta I ( \Omega ) , \mathcal{O} \boldsymbol{]} \varphi^{\prime \prime \alpha} ,\sigma \rangle \tag{184}
$$

where $\boldsymbol{[} P,R \boldsymbol{]} = PR - RP$. This applies for the case when $\mathcal{O}$ is fixed and the states vary.

Now we want to calculate the variation of $\langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle$ for the case when the states are fixed, and $\mathcal{O} = \mathcal{O} ( \varphi^{\alpha} ( \sigma )$) changes. This, however, will be the same as for the previous case, except with the opposite sign, because the variation of the matrix element<sup><a id="fnref-18" href="#fn-18">18</a></sup>

<a id="eq-185"></a>

$$
\langle \varphi^{\prime \alpha} ,\sigma \mathcal{O} \varphi^{\prime \prime \alpha} ,\sigma \rangle \tag{185}
$$

if _both_ the states and $\mathcal{O}$ change simultaneously is zero. Therefore, if we use a representation in which matrix elements of $\mathcal{O}$ are defined between states _not_ subject to variation we get<sup><a id="fnref-19" href="#fn-19">19</a></sup>

<a id="eq-186"></a>

$$
i \hbar \delta \mathcal{O} ( \sigma ) = \boldsymbol{[} \delta I ( \Omega ) , \mathcal{O} ( \sigma ) \boldsymbol{]} \tag{186}
$$

This is the Schwinger action principle in operator form. It is related to (177) exactly as the Heisenberg representation is to the Schrödinger representation in elementary quantum mechanics.

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p39"><sup>[p. 39]</sup></a>

### <a id="s-d-canonical-commutation-laws"></a>D. The Canonical Commutation Laws

Taking for $\sigma$ the space at time $t$, for $\mathcal{O} ( \sigma )$ the operator $\varphi^{\alpha} ( r,t )$ at the space-point $r$, and $\delta x_{\mu} = \delta L$ = 0 we have by (182) and (186) for an arbitrary variation $\delta \varphi^{\alpha}$

<a id="eq-187"></a>

$$
- i \hbar \delta \varphi^{\alpha} ( r,t ) = \sum_{\beta} \int \boldsymbol{[} \pi_{\beta} ( r^{\prime} ,t ) \delta \varphi^{\beta} ( r^{\prime} ,t ) ,\varphi^{\alpha} ( r,t ) \boldsymbol{]} d^{3} \boldsymbol{r}^{\prime} \tag{187}
$$

because $d\sigma = - n_{\mu} d\sigma_{\mu} = - i ( - idx_{1}^{\prime} dx_{2}^{\prime} dx_{3}^{\prime} ) = - d^{3} r^{\prime}$ by (158); the unit vector in the _increasing_ time direction is $i$, and this is the outward direction since we choose $\sigma_{o}$ in the past. Hence for every $r,r^{\prime}$

<a id="eq-188"></a>

$$
\boldsymbol{[} \varphi^{\alpha} ( r,t ) ,\pi_{\beta} ( r^{\prime} ,t ) \boldsymbol{]} = i \hbar \delta_{\alpha \beta} \delta^{3} ( \boldsymbol{r} - \boldsymbol{r}^{\prime} ) \tag{188}
$$

Also since the $\varphi^{\alpha} ( r )$ on $\sigma$ are assumed independent variables,

<a id="eq-189"></a>

$$
\boldsymbol{[} \varphi^{\alpha} ( r,t ) ,\varphi^{\beta} ( r^{\prime} ,t ) \boldsymbol{]} = 0 \tag{189}
$$

So this method gives automatically the correct canonical commutation laws for the fields. There is no need to prove that the commutation rules are consistent with the field equations, as was necessary in the older methods.

### <a id="s-e-the-heisenberg-equation-of-motion-for-the-operators"></a>E. The Heisenberg Equation of Motion for the Operators

Suppose that $\sigma$ is a flat surface at time $t$, and that a variation is made by moving the surface through the small time $\delta t$ as in B above. But now let $\mathcal{O} ( t ) = \mathcal{O} ( \sigma )$ be an operator built up out of the field-operators $\varphi^{\alpha}$ on $\sigma$. Then by (165) and (186) the change in $\mathcal{O} ( t )$ produced by the variation is given by

$$
i \hbar \delta \mathcal{O} ( t ) = \boldsymbol{[} - H ( t ) \delta t, \mathcal{O} ( t ) \boldsymbol{]}
$$

That is to say, $\mathcal{O} ( t )$ satisfies the Heisenberg equation of motion

<a id="eq-190"></a>

$$
i \hbar \frac{d \mathcal{O} ( t )}{dt} = \boldsymbol{[} \mathcal{O} ( t ) ,H ( t ) \boldsymbol{]} \tag{190}
$$

where $H ( t )$ is the total Hamiltonian operator.

### <a id="s-f-general-covariant-commutation-laws"></a>F. General Covariant Commutation Laws

From (186) we derive at once the general covariant form of the commutation laws discovered by Peierls in 1950 <a id="refback-13-1"></a>[[13]](#ref-13). This covariant form is not easy to reach in the Hamiltonian formalism.

Let two field points $z$ and $y$ be given, and two operators $\mathcal{R} ( z )$ and $\mathcal{Q} ( y )$ depending on the field quantities $\varphi^{\alpha}$ at $z$ and $y$. Let a reference surface $\sigma_{o}$ be fixed, past of both $z$ and $y$. Suppose the quantity,

<a id="eq-191"></a>

$$
\delta_{\mathcal{R}} ( L ) = \epsilon \delta^{4} ( x - z ) \mathcal{R} ( z ) \tag{191}
$$

is added to the Lagrangian density $L ( x )$, where $\epsilon$ is an infinitesimal c-number This will make at most a certain infinitesimal change $\epsilon \delta_{\mathcal{R}} \varphi^{\alpha} ( x )$ in the solutions $\varphi^{\alpha} ( x )$ of the field equations. Supposing the new $\varphi^{\alpha} ( x )$ to be identical with the old one on $\sigma_{o}$ then $\delta_{\mathcal{R}} \varphi^{\alpha} ( x )$ is different from zero only in the future light-cone of $z$.

Similarly adding

<a id="eq-192"></a>

$$
\delta_{\mathcal{Q}} ( L ) = \epsilon \delta^{4} ( x - y ) \mathcal{Q} ( y ) \tag{192}
$$

to $L ( x )$ produces at most a change $\epsilon \delta_{\mathcal{Q}} \varphi^{\alpha} ( x )$ in the $\varphi^{\alpha} ( x )$. Let $\epsilon \delta_{\mathcal{R}} \mathcal{Q} ( y )$ be the change in $\mathcal{Q} ( y )$ produced by the addition (191), while $\epsilon \delta_{\mathcal{Q}} \mathcal{R} ( z )$ be the change in $\mathcal{R} ( z )$ produced by (192). Suppose $y$ lies on a surface

<hr class="page" style="height:6px;border:0;background:currentColor;opacity:0.25">
<a id="p40"><sup>[p. 40]</sup></a>

$\sigma$ lying in the future of $z$. Then we take $\mathcal{Q} ( y )$ for $\mathcal{O} ( \sigma )$ in (186), and $\delta L$ given by (191). The $\delta I$(Ω) given by (182) reduces then simply to

$$
\delta I ( \Omega ) = \frac{1}{c} \epsilon \mathcal{R} ( z )
$$

It is assumed that there is no intrinsic change $\delta \varphi^{\alpha}$ of $\varphi^{\alpha}$ or $\delta x_{\mu}$ of $\sigma$ apart from the change whose effect is already included in the $\delta L$ term. Thus (186) gives

<a id="eq-193"></a>

$$
\begin{aligned} \boldsymbol{[} \mathcal{R} ( z ) , \mathcal{Q} ( y ) \boldsymbol{]} &= i \hbar c\delta_{\mathcal{R}} \mathcal{Q} ( y ) ( y_{0} >z_{0} ) \\ \boldsymbol{[} \mathcal{R} ( z ) , \mathcal{Q} ( y ) \boldsymbol{]} &= - i \hbar c\delta_{\mathcal{Q}} \mathcal{R} ( z ) ( z_{0} >y_{0} ) \end{aligned} \tag{193}
$$

When $y$ and $z$ are separated by a space-like interval, the commutator is zero, because the disturbance $\mathcal{R} ( z )$ propagates with a velocity at most $c$ and therefore can affect things only in the future lightcone of $z$; this means $\delta_{\mathcal{R}} \mathcal{Q} ( y )$ = 0 in this case.

<a id="ix-148">Peierls</a>’ <a id="ix-145">formula</a>, valid for any pair of field operators, is

<a id="eq-194"></a>

$$
\boldsymbol{[} \mathcal{R} ( z ) , \mathcal{Q} ( y ) \boldsymbol{]} = i \hbar c \{ \delta_{\mathcal{R}} \mathcal{Q} ( y ) - \delta_{\mathcal{Q}} \mathcal{R} ( z ) \} \tag{194}
$$

This is a useful formula for calculating commutators in a covariant way.

### <a id="s-g-anticommuting-fields"></a>G. Anticommuting Fields

There is one type of field theory which can be constructed easily by Schwinger’s <a id="ix-239">action principle</a>, but which does not come out of Feynman’s picture. Suppose a classical field theory in which a group of field operators $\psi^{\alpha}$ always occurs in the <a id="ix-68">Lagrangian</a> in bilinear combinations like $\overline{\psi}^{\beta} \psi^{\alpha}$ with the group of field operators $\overline{\psi}$. Examples, the Dirac $L_{D}$ and the quantum electrodynamics $L_{Q}$.

Then instead of taking every $\varphi^{\alpha}$ on a given surface $\sigma$ to commute as in (189), we may take every pair of $\psi^{\alpha}$ to <a id="ix-38">anticommute</a>, thus<sup><a id="fnref-20" href="#fn-20">20</a></sup>

<a id="eq-195"></a>

$$
\{ \psi^{\alpha} ( r,t ) ,\psi^{\beta} ( r^{\prime} ,t ) \} = 0 \quad \{ \mathcal{P} , \mathcal{R} \} = \mathcal{P} \mathcal{R} + \mathcal{R} \mathcal{P} \tag{195}
$$

The bilinear combination will still commute, like the $\varphi^{\alpha}$’s did before. The $\psi^{\alpha}$ commute as before with any field quantities on $\sigma$ other than the $\psi$ and $\overline{\psi}$. Schwinger then assumes (177) to hold precisely as before, except that in calculating $\delta I$(Ω) according to (178), the variation $\delta \psi^{\alpha}$ _anticommutes_ with all operators $\psi^{\alpha}$ and $\overline{\psi}^{\beta}$. In these theories it turns out that the momentum $\pi_{\alpha}$ conjugate to $\psi^{\alpha}$ is just a linear combination of $\overline{\psi}$, because the Lagrangian is only linear in the derivatives of $\psi$. With the anticommuting fields the field equations (179) are deduced as before, also the Schrödinger equation (180), the commutation rules being given by (186) and (187). But now in order to make (187) valid, since $\delta \psi^{\beta}$ anticommutes with the $\psi$ and $\pi$ operators,<sup><a id="fnref-21" href="#fn-21">21</a></sup> the canonical commutation law must be written

<a id="eq-196"></a>

$$
\{ \psi^{\alpha} ( r,t ) ,\pi_{\beta} ( r^{\prime} ,t ) \} = - i \hbar \delta_{\alpha \beta} \delta^{3} ( \boldsymbol{r} - \boldsymbol{r}^{\prime} ) \tag{196}
$$

The general commutation rule (194) is still valid provided that $\mathcal{Q}$ and $\mathcal{R}$ are also expressions bilinear in the $\overline{\psi}$ and $\psi$.

The interpretation of the operators, and the justification for the Schwinger principle, in the case of anticommuting fields, is not clear. But it is clear that the Schwinger principle in this case gives a consistent and simple formulation of a relativistic quantum field theory. And we may as well take advantage of the method, even if we do not quite understand its conceptual basis. The resulting theory is mathematically unambiguous, and gives results in agreement with experiment; that should be good enough.

---

## Notes

<a id="fn-16"></a>**16.** These three articles may be found in Schwinger, _Selected Papers on Quantum Electrodynamics_. [↩](#fnref-16)

<a id="fn-17"></a>**17.** Deleted “for”; the original statement read “condition that for its $\delta I$(Ω) = 0” [↩](#fnref-17)

<a id="fn-18"></a>**18.** Eq. (185) lacked a label in v.2. The discussion beginning at Eq. (182) and continuing to Eq. (186) is unusually different between the editions. What is here follows Moravcsik’s v.2 with the addition of the phrase “the matrix element” at Eq. (185). [↩](#fnref-18)

<a id="fn-19"></a>**19.** A $\delta$ was missing: the equation read $i \hbar \mathcal{O} = \boldsymbol{[} \delta I$(Ω)$, \mathcal{O} ( \sigma ) \boldsymbol{]}$. [↩](#fnref-19)

<a id="fn-20"></a>**20.** The notation originally used for anticommutators was $AB + BA = [ A,B ]_{+}$.The more familiar $\{ A,B \}$ has been used instead. [↩](#fnref-20)

<a id="fn-21"></a>**21.** Here, $\psi$ was substituted for the original $\varphi$ (the variable in Eq. (187)) for clarity. [↩](#fnref-21)

---

## References

<a id="ref-13"></a>**[13]** R. E. Peierls, “The commutation laws of relativistic field theory”, _Proc. Roy. Soc. A_ **214** (1952) 143. Note the year of publication is 1952. [↩](#refback-13-1)

---

[← Scattering Problems and Born Approximation](03-scattering-problems-and-born-approximation.md) · [Examples of Quantized Field Theories →](05-examples-of-quantized-field-theories.md)
