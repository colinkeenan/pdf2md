[← 31 Classical Scalar Fields](31-classical-scalar-fields.md) · [33 Quantum Theory of Radiation →](33-quantum-theory-of-radiation.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p436" href="book.pdf#page=436" target="_blank" title="This page in the PDF"><sup>[p. 436]</sup></a>

# 32 Classical Maxwell Fields

# <a id="s-32-1-rationalized-heaviside-lorentz-units"></a>32.1 Rationalized Heaviside-Lorentz Units

The **SI units** are based on a unit of length of the order of human size originally related to the size of the earth, a unit of time approximately equal to the time between heartbeats, and a unit of mass related to the length unit and the mass of water. None of these depend on any even nearly fundamental physical quantities. Therefore many important physical equations end up with extra (needless) constants in them like $`c`$. Even with the three basic units defined, we could have chosen the unit of charge correctly to make $`\epsilon_{0}`$ and $`\mu_{0}`$ unnecessary but instead a very arbitrary choice was made $`\mu_{0}`$ = 4$`\pi \times 1 0^{- 7}`$ and the Ampere is defined by the current in parallel wires at one meter distance from each other that gives a force of $`2 \times 1 0^{- 7}`$ Newtons per meter. The Coulomb is set so that the Ampere is one Coulomb per second. **With these choices SI units make Maxwell’s** **equations and our filed theory look very messy.**

Physicists have more often used **CGS units** in which the unit of charge and definition of the field units are set so that $`\epsilon_{0}`$ = 1 and $`\mu_{0}`$ = 1 so they need not show up in the equations. The CGS units are not perfect, however, and we will want to change them slightly to make our theory of the Maxwell Field simple. The mistake made in defining CGS units was in removing the 4$`\pi`$ that show up in Coulombs law. Coulombs law is not fundamental and the 4$`\pi`$ belonged there.

We will correct this little mistake and move to **Rationalized Heaviside-Lorentz Units** by making a minor modification to the unit of charge and the units of fields. With this modification, our field theory will have few constants to carry around. As the name of the system of units suggests, the problem with CGS has been with $`\pi`$. We don’t need to change the centimeter, gram or second to fix the problem.

In **Rationalized Heaviside-Lorentz units** we decrease the field strength by a factor of $`\sqrt{4 \pi}`$ and increase the charges by the same factor, leaving the force unchanged.

```math
\alpha = \frac{e^{2}}{\hbar c} \frac{\vec{B} \vec{E}}{e} \vec{A} \quad \to \to \frac{\to \to}{\to} \quad \frac{e^{2}}{4 \pi \hbar c} \frac{\vec{B}}{\sqrt{4 \pi}} \frac{\frac{\vec{E}}{\sqrt{4 \pi}}}{\frac{\vec{A}}{\frac{\surd}{e \sqrt{4 \pi}} 4 \pi}} \approx \frac{1}{1 3 7}
```

Its not a very big change but it would have been nice if Maxwell had started with this set of units. Of course the value of $`\alpha`$ cannot change, but, the formula for it does because we have redefined the charge $`e`$.

**Maxwell’s Equations** in CGS units are

```math
\vec{\nabla} \cdot \vec{B} = 0
```

```math
\vec{\nabla} \times \vec{E} + \frac{1}{c} \frac{\partial B}{\partial t} = 0
```

<div align="right"><a href="book.pdf#page=436" target="_blank" title="This page in the PDF"><sup>[p. 436]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p437" href="book.pdf#page=437" target="_blank" title="This page in the PDF"><sup>[p. 437]</sup></a>

```math
\vec{\nabla} \cdot \vec{E} = 4 \pi \rho
```

```math
\vec{\nabla} \times \vec{B} - \frac{1}{c} \frac{\partial E}{\partial t} = \frac{4 \pi}{c} \vec{j} .
```

The **Lorentz Force** is

```math
\vec{F} = - e ( \vec{E} + \frac{1}{c} \vec{v} \times \vec{B} ) .
```

When we change to **Rationalized Heaviside-Lorentz units**, the equations become

```math
\vec{\nabla} \cdot \vec{B} = 0
```

```math
\vec{\nabla} \times \vec{E} + \frac{1}{c} \frac{\partial B}{\partial t} = 0
```

```math
\vec{\nabla} \cdot \vec{E} = \rho
```

```math
\vec{\nabla} \times \vec{B} - \frac{1}{c} \frac{\partial E}{\partial t} = \frac{1}{c} \vec{j}
```

```math
\vec{F} = - e ( \vec{E} + \frac{1}{c} \vec{v} \times \vec{B} )
```

That is, the equations remain the same except the factors of 4$`\pi`$ in front of the source terms disappear. Of course, it would still be convenient to set $`c`$ = 1 since this has been confusing us about 4D geometry and $`c`$ is the last unnecessary constant in Maxwell’s equations. For our calculations, we can set $`c`$ = 1 any time we want unless we need answers in centimeters.

# <a id="s-32-2-the-electromagnetic-field-tensor"></a>32.2 The Electromagnetic Field Tensor

The transformation of electric and magnetic fields under a Lorentz boost we established even before Einstein developed the theory of relativity. We know that E-fields can transform into B-fields and vice versa. For example, a point charge at rest gives an Electric field. If we boost to a frame in which the charge is moving, there is an Electric and a Magnetic field. This means that the E-field cannot be a Lorentz vector. We need to put the Electric and Magnetic fields together into one (tensor) object to properly handle Lorentz transformations and to write our equations in a covariant way.

The simplest way and the correct way to do this is to make the Electric and Magnetic fields components of a **rank 2 (antisymmetric) tensor**.

```math
F_{\mu \nu} = \begin{pmatrix} 0 & B_{z} & - B_{y} & - iE_{x} \\ - B_{z} & 0 & B_{x} & - iE_{y} \\ B_{y} & - B_{x} & 0 & - iE_{z} \\ iE_{x} & iE_{y} & iE_{z} & 0 \end{pmatrix}
```

The fields can simply be written in terms of the **vector potential**, (which is a Lorentz vector) $`A_{\mu} = ( \vec{A} ,i\varphi`$ ).

```math
F_{\mu \nu} = \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}}
```

<div align="right"><a href="book.pdf#page=437" target="_blank" title="This page in the PDF"><sup>[p. 437]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p438" href="book.pdf#page=438" target="_blank" title="This page in the PDF"><sup>[p. 438]</sup></a>

Note that this is automatically antisymmetric under the interchange of the indices. As before, the **first two (sourceless) Maxwell equations are automatically satisfied** for fields derived from a vector potential. We may write the **other two Maxwell equations** in terms of the 4-vector $`j_{\mu} = ( \vec{j} , i c \rho )`$.

```math
\frac{\partial F_{\mu \nu}}{\partial x_{\nu}} = \frac{j_{\mu}}{c}
```

Which is why the T-shirt given to every MIT freshman when they take Electricity and Magnetism should say

“... and God said $`\tfrac{\partial}{\partial x_{\nu}} \left( \tfrac{\partial A_{\nu}}{\partial x_{\mu}} - \tfrac{\partial A_{\mu}}{\partial x_{\nu}} \right) = \tfrac{j_{\mu}}{c}`$ and there was light.”

Of course he or she hadn’t yet quantized the theory in that statement.

For some peace of mind, lets **verify a few terms in the equations**. Clearly all the diagonal terms in the field tensor are zero by antisymmetry. Lets take some example off-diagonal terms in the field tensor, checking the (old) definition of the fields in terms of the potential.

```math
\vec{B} = \vec{\nabla} \times \vec{A}
```

```math
\vec{E} = - \vec{\nabla} \varphi - \frac{1}{c} \frac{\partial \vec{A}}{\partial t}
```

```math
F_{1 2} = \frac{\partial A_{2}}{\partial x_{1}} - \frac{\partial A_{1}}{\partial x_{2}} = ( \vec{\nabla} \times \vec{A} )_{z} = B_{z}
```

```math
F_{1 3} = \frac{\partial A_{3}}{\partial x_{1}} - \frac{\partial A_{1}}{\partial x_{3}} = - ( \vec{\nabla} \times \vec{A} )_{y} = - B_{y}
```

```math
F_{4 i} = \frac{\partial A_{i}}{\partial x_{4}} - \frac{\partial A_{4}}{\partial x_{i}} = \frac{1}{ic} \frac{\partial A_{i}}{\partial t} - \frac{\partial ( i\varphi )}{\partial x_{i}} = - i \left( \frac{1}{c} \frac{\partial A_{i}}{\partial t} + \frac{\partial \varphi}{\partial x_{i}} \right) = - i \left( \frac{\partial \varphi}{\partial x_{i}} + \frac{1}{c} \frac{\partial A_{i}}{\partial t} \right) = iE_{i}
```

Lets also **check what the Maxwell equation says** for the last row in the tensor.

```math
\frac{\partial F_{4 \nu}}{\partial x_{\nu}} = \frac{j_{4}}{c}
```

```math
\frac{\partial F_{4 i}}{\partial x_{i}} = \frac{ic\rho}{c}
```

```math
\frac{\partial ( iE_{i} )}{\partial x_{i}} = i\rho
```

```math
\frac{\partial E_{i}}{\partial x_{i}} = \rho
```

```math
\vec{\nabla} \cdot \vec{E} = \rho
```

We will not bother to check the Lorentz transformation of the fields here. Its right.

<div align="right"><a href="book.pdf#page=438" target="_blank" title="This page in the PDF"><sup>[p. 438]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p439" href="book.pdf#page=439" target="_blank" title="This page in the PDF"><sup>[p. 439]</sup></a>

# <a id="s-32-3-the-lagrangian-for-electromagnetic-fields"></a>32.3 The Lagrangian for Electromagnetic Fields

There are not many ways to make a **scalar Lagrangian from the field tensor**. We already know that

```math
\frac{\partial F_{\mu \nu}}{\partial x_{\nu}} = \frac{j_{\mu}}{c}
```

and we need to make our Lagrangian out of the fields, not just the current. Again, $`x_{\mu}`$ **cannot** **appear explicitly** because that violates symmetries of nature. Also we want a linear equation and so higher powers of the field should not occur. A term of the form $`mA_{\mu} A_{\mu}`$ is a mass term and would cause fields to fall off faster than $`\frac{1}{r}`$. So, the only reasonable choice is

```math
F_{\mu \nu} F_{\mu \nu} = 2 ( B^{2} - E^{2} ) .
```

One might consider

```math
e_{\mu \nu \lambda \sigma} F_{\mu \nu} F_{\lambda \sigma} = \vec{B} \cdot \vec{E}
```

but that is a pseudo-scalar, not a scalar. That is, it changes sign under a parity transformation. The EM interaction is known to conserve parity so this is not a real option. As with the scalar field, we need to **add an interaction with a source term**. Of course, we know electromagnetism well, so finding the right Lagrangian is not really guess work. The source of the field is the vector $`j_{\mu}`$, so the simple scalar we can write is $`j_{\mu} A_{\mu}`$.

The **Lagrangian for Classical Electricity and Magnetism** we will try is.

```math
\mathcal{L}_{\mathcal{E} \mathcal{M}} = - \frac{1}{4} F_{\mu \nu} F_{\mu \nu} + \frac{1}{c} j_{\mu} A_{\mu}
```

In working with this Lagrangian, we will **treat each component of** $`A`$ **as an independent field**.

The next step is to check what the Euler-Lagrange equation gives us.

```math
\frac{\partial}{\partial x_{\nu}} \left( \frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial x_{\nu} )} \right) - \frac{\partial \mathcal{L}}{\partial A_{\mu}} = 0
```

```math
\mathcal{L} = - \frac{1}{4} F_{\mu \nu} F_{\mu \nu} + \frac{1}{c} j_{\mu} A_{\mu} = - \frac{1}{4} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right) \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right) + \frac{1}{c} j_{\mu} A_{\mu}
```

```math
\frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial x_{\nu} )} = - \frac{1}{4} \frac{\partial}{\partial ( \partial A_{\mu} /\partial x_{\nu} )} \left( \frac{\partial A_{\sigma}}{\partial x_{\lambda}} - \frac{\partial A_{\lambda}}{\partial x_{\sigma}} \right) \left( \frac{\partial A_{\sigma}}{\partial x_{\lambda}} - \frac{\partial A_{\lambda}}{\partial x_{\sigma}} \right)
```

```math
= - \frac{1}{4} \frac{\partial}{\partial ( \partial A_{\mu} /\partial x_{\nu} )} \left( 2 \frac{\partial A_{\sigma}}{\partial x_{\lambda}} \frac{\partial A_{\sigma}}{\partial x_{\lambda}} - 2 \frac{\partial A_{\sigma}}{\partial x_{\lambda}} \frac{\partial A_{\lambda}}{\partial x_{\sigma}} \right)
```

```math
= - \frac{1}{4} 4 \left( \frac{\partial A_{\mu}}{\partial x_{\nu}} - \frac{\partial A_{\nu}}{\partial x_{\mu}} \right)
```

```math
= - F_{\nu \mu} = F_{\mu \nu}
```

```math
\frac{\partial}{\partial x_{\nu}} F_{\mu \nu} - \frac{\partial \mathcal{L}}{\partial A_{\mu}} = 0
```

```math
\frac{\partial}{\partial x_{\nu}} F_{\mu \nu} - \frac{j_{\mu}}{c} = 0
```

```math
\frac{\partial}{\partial x_{\nu}} F_{\mu \nu} = \frac{j_{\mu}}{c}
```

<div align="right"><a href="book.pdf#page=439" target="_blank" title="This page in the PDF"><sup>[p. 439]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p440" href="book.pdf#page=440" target="_blank" title="This page in the PDF"><sup>[p. 440]</sup></a>

Note that, since we have four independent components of $`A_{\mu}`$ as independent fields, we have four equations; or one 4-vector equation. The **Euler-Lagrange equation gets us back Maxwell’s** **equation** with this choice of the Lagrangian. This clearly justifies the choice of $`\mathcal{L}`$.

It is important to emphasize that we have a Lagrangian based, formal classical field theory for electricity and magnetism which has **the four components of the 4-vector potential as the** **independent fields.** We could not treat each component of $`F_{\mu \nu}`$ as independent since they are clearly correlated. We could have tried using the six independent components of the antisymmetric tensor but it would not have given the right answer. Using the 4-vector potentials as the fields does give the right answer. **Electricity and Magnetism is a theory of a 4-vector field** $`A_{\mu}`$**.**

We can also calculate the **free field Hamiltonian density**, that is, the Hamiltonian density in regions with no source term. We use the standard definition of the Hamiltonian in terms of the Lagrangian.

```math
\mathcal{H} = \left( \frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial dt )} \right) \frac{\partial A_{\mu}}{\partial dt} - \mathcal{L} = \left( \frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial x_{4} )} \right) \frac{\partial A_{\mu}}{\partial x_{4}} - \mathcal{L}
```

We just calculated above that

```math
\frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial x_{\nu} )} = F_{\mu \nu}
```

which we can use to get

```math
\frac{\partial \mathcal{L}}{\partial ( \partial A_{\mu} /\partial x_{4} )} = F_{\mu 4}
```

```math
\mathcal{H} = ( F_{\mu 4} ) \frac{\partial A_{\mu}}{\partial x_{4}} - \mathcal{L}
```

```math
= F_{\mu 4} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

```math
\mathcal{H} = F_{\mu 4} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

We will use this once we have written the radiation field in a convenient form. In the meantime, we can check what this gives us in general in a region with no sources.

```math
\mathcal{H} = F_{\mu 4} \left( F_{4 \mu} + \frac{\partial A_{4}}{\partial x_{\mu}} \right) + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

```math
= - F_{4 \mu} \left( F_{4 \mu} + \frac{\partial A_{4}}{\partial x_{\mu}} \right) + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

```math
= - F_{4 \mu} F_{4 \mu} - F_{4 \mu} \frac{\partial A_{4}}{\partial x_{\mu}} + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

```math
= E^{2} - F_{4 i} \frac{\partial A_{4}}{\partial x_{i}} + \frac{1}{2} ( B^{2} - E^{2} )
```

```math
= \frac{1}{2} ( E^{2} + B^{2} ) - iE_{i} \frac{\partial ( i\varphi )}{\partial x_{i}}
```

```math
= 1 ( E^{2} + B^{2} ) + E_{i} \frac{\partial \varphi}{\partial x_{i}}
```

<div align="right"><a href="book.pdf#page=440" target="_blank" title="This page in the PDF"><sup>[p. 440]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p441" href="book.pdf#page=441" target="_blank" title="This page in the PDF"><sup>[p. 441]</sup></a>

If we integrate the last term by parts, (and the fields fall to zero at infinity), then that term contains a $`\vec{\nabla} \cdot \vec{E}`$ which is zero with no sources in the region. We can therefore drop it and are left with

```math
\mathcal{H} = \frac{1}{2} ( E^{2} + B^{2} ) .
```

This is the result we expected, the energy density and an EM field. (Remember the fields have been decreased by a factor of $`\sqrt{4 \pi}`$ compared to CGS units.)

We will study the interaction between electrons and the electromagnetic field with the Dirac equation. Until then, the Hamiltonian used for non-relativistic quantum mechanics will be sufficient. We have derived the Lorentz force law from that Hamiltonian.

```math
H = \frac{1}{2 m} \left( \vec{p} + \frac{e}{c} \vec{A} \right)^{2} + eA_{0}
```

# <a id="s-32-4-gauge-invariance-can-simplify-equations"></a>32.4 Gauge Invariance can Simplify Equations

We have already studied many aspects of gauge invariance (See Section 20.3). in electromagnetism and the corresponding invariance under a phase transformation in Quantum Mechanics. One point to note is that, with our choice to “treat each component of $`A_{\mu}`$ as an independent field”, we are making a **theory for the vector field** $`A_{\mu}`$ **with a gauge symmetry**, not really a theory for the field $`F_{\mu \nu}`$.

Recall that the gauge symmetry of Electricity and Magnetism and the phase symmetry of electron wavefunctions are really one and the same. Neither the phase of the wavefunction nor the vector potential are directly observable, but the symmetry is.

We will not go over the consequences of gauge invariance again here, but, we do want to use gauge invariance to simplify our equations.

Maxwell’s equation is

```math
\frac{\partial F_{\mu \nu}}{\partial x_{\nu}} = \frac{j_{\mu}}{c}
```

```math
\frac{\partial}{\partial x_{\nu}} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right) = \frac{j_{\mu}}{c}
```

```math
\frac{\partial}{\partial x_{\nu}} \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial^{2} A_{\mu}}{\partial x_{\nu}^{2}} = \frac{j_{\mu}}{c}
```

```math
\frac{\partial^{2} A_{\mu}}{\partial x_{\nu}^{2}} - \frac{\partial}{\partial x_{\mu}} \frac{\partial A_{\nu}}{\partial x_{\nu}} = - \frac{j_{\mu}}{c}
```

We can simplify this basic equation by setting the gauge according to the **Lorentz condition**.

```math
\frac{\partial A_{\nu}}{\partial x_{\nu}} = 0
```

The gauge transformation needed is

```math
A_{\mu} \to A_{\mu} + \frac{\partial \chi}{\partial x_{\mu}}
```

<div align="right"><a href="book.pdf#page=441" target="_blank" title="This page in the PDF"><sup>[p. 441]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p442" href="book.pdf#page=442" target="_blank" title="This page in the PDF"><sup>[p. 442]</sup></a>

```math
✷ \chi = - \left[ \frac{\partial A_{\nu}}{\partial x_{\nu}} \right]_{old}
```

The **Maxwell equation with the Lorentz condition** now reads

```math
✷ A_{\mu} = - \frac{j_{\mu}}{c} .
```

There is **still substantial gauge freedom** possible. The second derivative of Λ is set by the Lorentz condition but there is still freedom in the first derivative which will modify $`A`$. Gauge transformations can be made as shown below.

```math
A_{\mu} \to A_{\mu} + \frac{\partial \Lambda}{\partial x_{\mu}}
```

✷Λ = 0

**This transformation will not disturb the Lorentz condition** which simplifies our equation. We will use a further gauge condition in the next chapter to work with transverse fields.

---

[← 31 Classical Scalar Fields](31-classical-scalar-fields.md) · [33 Quantum Theory of Radiation →](33-quantum-theory-of-radiation.md)
