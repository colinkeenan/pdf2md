[← 32 Classical Maxwell Fields](32-classical-maxwell-fields.md) · [34 Scattering of Photons →](34-scattering-of-photons.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p443" href="book.pdf#page=443" target="_blank" title="This page in the PDF"><sup>[p. 443]</sup></a>

# 33 Quantum Theory of Radiation

# <a id="s-33-1-transverse-and-longitudinal-fields"></a>33.1 Transverse and Longitudinal Fields

In non-relativistic Quantum Mechanics, the static Electric field is represented by a scalar potential, magnetic fields by the vector potential, and the radiation field also through the vector potential. It will be convenient to keep this separation between the large static atomic Electric field and the radiation fields, however, the equations we have contain the four-vector $`A_{\mu}`$ with all the fields mixed. When we quantize the field, all E and B fields as well as electromagnetic waves will be made up of photons. It is useful to be able to **separate the E fields due to fixed charges from the EM** **radiation from moving charges**. This separation is not Lorentz invariant, but it is still useful.

Enrico Fermi showed, in 1930, that $`A_{\|}`$ together with $`A_{0}`$ give rise to Coulomb interactions between particles, whereas $`A_{\perp}`$ gives rise to the EM radiation from moving charges. With this separation, we can maintain the form of our non-relativistic Hamiltonian,

```math
H = \sum_{j} \frac{1}{2 m_{j}} \left( \vec{p} - \frac{e}{c} \vec{A}_{\perp} ( \vec{x}_{j} ) \right)^{2} + \sum_{i>j} \frac{e_{i} e_{j}}{4 \pi | \vec{x}_{i} - \vec{x}_{j} |} + H_{rad}
```

where $`H_{rad}`$ is purely the Hamiltonian of the radiation (containing only $`\vec{A}_{\perp} )`$, and $`\vec{A}_{\perp}`$ is the part of the vector potential which satisfies $`\vec{\nabla} \cdot \vec{A}_{\perp}`$ = 0. Note that $`\vec{A}_{\|}`$ and $`A_{4}`$ appear nowhere in the Hamiltonian. Instead, we have the Coulomb potential. This separation allows us to continue with our standard Hydrogen solution and just add radiation. We will not derive this result.

In a **region in which there are no source terms**,

```math
j_{\mu} = 0
```

we can **make a gauge transformation which eliminates** $`A_{0}`$ by choosing Λ such that

```math
\frac{1}{c} \frac{\partial \Lambda}{\partial t} = A_{0} .
```

Since the fourth component of $`A_{\mu}`$ is now eliminated, the Lorentz condition now implies that

```math
\vec{\nabla} \cdot \vec{A} = 0 .
```

Again, making one component of a 4-vector zero is not a Lorentz invariant way of working. We have to redo the gauge transformation if we move to another frame.

If $`j_{\mu} \neq 0`$, then we cannot eliminate $`A_{0}`$, since ✷$`A_{0} = \tfrac{j_{0}}{c}`$ and we are only allowed to make gauge transformations for which ✷Λ = 0. In this case we must separate the vector potential into the transverse and longitudinal parts, with

```math
\vec{A} = \vec{A}_{\perp} + \vec{A}_{\|}
```

```math
\vec{\nabla} \cdot \vec{A}_{\perp} = 0
```

```math
\vec{\nabla} \times \vec{A}_{\|} = 0
```

<div align="right"><a href="book.pdf#page=443" target="_blank" title="This page in the PDF"><sup>[p. 443]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p444" href="book.pdf#page=444" target="_blank" title="This page in the PDF"><sup>[p. 444]</sup></a>

We will now study the radiation field in a region with no sources so that $`\vec{\nabla} \cdot \vec{A}`$ = 0. We will use the equations

```math
\vec{B} = \vec{\nabla} \times \vec{A}
```

```math
\vec{E} = - \frac{1}{c} \frac{\partial \vec{A}}{\partial t}
```

```math
\nabla^{2} \vec{A} - \frac{1}{c^{2}} \frac{\partial \vec{A}}{\partial t^{2}} = 0
```

# <a id="s-33-2-fourier-decomposition-of-radiation-oscillators"></a>33.2 Fourier Decomposition of Radiation Oscillators

Our goal is to write the Hamiltonian for the radiation field in terms of a sum of harmonic oscillator Hamiltonians. The first step is to write the radiation field in as simple a way as possible, as a sum of harmonic components. We will work in a cubic volume $`V = L^{3}`$ and apply **periodic boundary** **conditions** on our electromagnetic waves. We also assume for now that there are **no sources inside** **the region** so that we can make a gauge transformation to make $`A_{0}`$ = 0 and hence $`\vec{\nabla} \cdot \vec{A}`$ = 0. We **decompose the field into its Fourier components** at $`t`$ = 0

```math
\vec{A} ( \vec{x} , t = 0 ) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} \left( c_{k,\alpha} ( t = 0 ) e^{i \vec{k} \cdot \vec{x}} + c_{k,\alpha}^{*} ( t = 0 ) e^{- i \vec{k} \cdot \vec{x}} \right)
```

where $`\hat{\epsilon}^{( \alpha )}`$ are real unit vectors, and $`c_{k,\alpha}`$ is the coefficient of the wave with wave vector $`\vec{k}`$ and polarization vector $`\hat{\epsilon}^{( \alpha )}`$. Once the wave vector is chosen, the two polarization vectors must be picked so that $`\hat{\epsilon}^{( 1 )} , \hat{\epsilon}^{( 2 )}`$, and $`\vec{k}`$ **form a right handed orthogonal system**. The components of the wave vector must satisfy

```math
k_{i} = \frac{2 \pi n_{i}}{L}
```

due to the periodic boundary conditions. The factor out front is set to normalize the states nicely since

```math
\frac{1}{V} \int d^{3} xe^{i \vec{k} \cdot \vec{x}} e^{- i \vec{k}^{\prime} \cdot \vec{x}} = \delta_{\vec{k} \vec{k}^{\prime}}
```

and

```math
\hat{\epsilon}^{( \alpha )} \cdot \hat{\epsilon}^{( \alpha^{\prime} )} = \delta_{\alpha \alpha^{\prime}} .
```

We know the **time dependence of the waves** from Maxwell’s equation,

```math
c_{k,\alpha} ( t ) = c_{k,\alpha} ( 0 ) e^{- i\omega t}
```

where $`\omega = kc`$. We can now write the **vector potential as a function of position and time**.

```math
\vec{A} ( \vec{x} , t ) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} \left( c_{k,\alpha} ( t ) e^{i \vec{k} \cdot \vec{x}} + c_{k,\alpha}^{*} ( t ) e^{- i \vec{k} \cdot \vec{x}} \right)
```

We may write this solution in several different ways, and use the best one for the calculation being performed. One nice way to write this is in terms 4-vector $`k_{\mu}`$, the wave number,

```math
k_{\mu} = \frac{p_{\mu}}{\hbar} = ( k_{x} ,k_{y} ,k_{z} ,ik ) = ( k_{x} ,k_{y} ,k_{z} ,i \frac{\omega}{c} )
```

<div align="right"><a href="book.pdf#page=444" target="_blank" title="This page in the PDF"><sup>[p. 444]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p445" href="book.pdf#page=445" target="_blank" title="This page in the PDF"><sup>[p. 445]</sup></a>

so that

```math
k_{\rho} x_{\rho} = k \cdot x = \vec{k} \cdot \vec{x} - \omega t.
```

We can then write the radiation field in a more covariant way.

```math
\vec{A} ( \vec{x} , t ) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} \left( c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c_{k,\alpha}^{*} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

A convenient shorthand for calculations is possible by noticing that the second term is just the complex conjugate of the first.

```math
\vec{A} ( \vec{x} , t ) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} \left( c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c.c. \right)
```

```math
\vec{A} ( \vec{x} , t ) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c.c.
```

Note again that we have made this a transverse field by construction. The unit vectors $`\hat{\epsilon}^{( \alpha )}`$ are transverse to the direction of propagation. Also note that we are working in a gauge with $`A_{4}`$ = 0, so this can also represent the 4-vector form of the potential. The **Fourier decomposition of the** **radiation field** can be written very simply.

```math
A_{\mu} = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c.c.
```

This choice of gauge makes switching between 4-vector and 3-vector expressions for the potential trivial.

Let’s **verify that this decomposition of the radiation field satisfies the Maxwell equation**, just for some practice. Its most convenient to use the covariant form of the equation and field.

```math
✷ A_{\mu} = 0
```

```math
✷ \left( \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c.c. \right) = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} c_{k,\alpha} ( 0 ) ✷ e^{ik_{\rho} x_{\rho}} + c.c.
```

```math
= \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} c_{k,\alpha} ( 0 ) ( - k_{\nu} k_{\nu} ) e^{ik_{\rho} x_{\rho}} + c.c. = 0
```

The result is zero since $`k_{\nu} k_{\nu} = k^{2} - k^{2}`$ = 0.

Let’s also **verify that** $`\vec{\nabla} \cdot \vec{A}`$ = 0.

```math
\vec{\nabla} \cdot \left( \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \hat{\epsilon}^{( \alpha )} c_{k,\alpha} ( t ) e^{i \vec{k} \cdot \vec{x}} + c.c. \right) = \frac{1}{\sqrt{V}} \sum_{k} \sum_{\alpha = 1}^{2} c_{k,\alpha} ( t ) \hat{\epsilon}^{( \alpha )} \cdot \vec{\nabla} e^{i \vec{k} \cdot \vec{x}} + c.c.
```

```math
= \frac{1}{\sqrt{V}} \sum_{k} \sum_{\alpha = 1}^{2} c_{k,\alpha} ( t ) \hat{\epsilon}^{( \alpha )} \cdot \vec{k} e^{i \vec{k} \cdot \vec{x}} + c.c. = 0
```

The result here is zero because $`\hat{\epsilon}^{( \alpha )} \cdot \vec{k}`$ = 0.

<div align="right"><a href="book.pdf#page=445" target="_blank" title="This page in the PDF"><sup>[p. 445]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p446" href="book.pdf#page=446" target="_blank" title="This page in the PDF"><sup>[p. 446]</sup></a>

# <a id="s-33-3-the-hamiltonian-for-the-radiation-field"></a>33.3 The Hamiltonian for the Radiation Field

We now wish to **compute the Hamiltonian in terms of the coefficients** $`c_{k,\alpha} ( t )`$. This is an important calculation because we will **use the Hamiltonian formalism to do the quantization** **of the field**. We will do the calculation using the covariant notation (while Sakurai outlines an alternate calculation using 3-vectors). We have already calculated **the Hamiltonian density for** **a classical EM field**.

```math
\mathcal{H} = F_{\mu 4} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{4} F_{\mu \nu} F_{\mu \nu}
```

```math
\mathcal{H} = \left( \frac{\partial A_{4}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{4}} \right) \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{4} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right) \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right)
```

```math
\mathcal{H} = - \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{2} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\nu}}{\partial x_{\mu}} \frac{\partial A_{\mu}}{\partial x_{\nu}} \right)
```

Now lets **compute the basic element of the above formula** for our decomposed radiation field.

```math
A_{\mu} = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} \left( c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + c_{k,\alpha}^{*} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\frac{\partial A_{\mu}}{\partial x_{\nu}} = \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} \left( c_{k,\alpha} ( 0 ) ( ik_{\nu} ) e^{ik_{\rho} x_{\rho}} + c_{k,\alpha}^{*} ( 0 ) ( - ik_{\nu} ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\frac{\partial A_{\mu}}{\partial x_{\nu}} = i \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} k_{\nu} \left( c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} - c_{k,\alpha}^{*} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\frac{\partial A_{\mu}}{\partial x_{4}} = - \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} \frac{\omega}{c} \left( c_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} - c_{k,\alpha}^{*} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

We have all the elements to finish the calculation of the Hamiltonian. Before pulling this all together in a brute force way, its good to realize that **almost all the terms will give zero**. We see that the derivative of $`A_{\mu}`$ is proportional to a 4-vector, say $`k_{\nu}`$ and to a polarization vector, say $`\epsilon_{\mu}^{( \alpha )}`$. The dot products of the 4-vectors, either $`k`$ with itself or $`k`$ with $`\epsilon`$ are zero. Going back to our expression for the Hamiltonian density, we can eliminate some terms.

```math
\mathcal{H} = - \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{2} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\nu}}{\partial x_{\mu}} \frac{\partial A_{\mu}}{\partial x_{\nu}} \right)
```

```math
\mathcal{H} = - \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}} + \frac{1}{2} ( 0 - 0 )
```

```math
\mathcal{H} = - \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}}
```

The remaining term has a dot product between polarization vectors which will be nonzero if the polarization vectors are the same. (Note that this simplification is possible because we have assumed no sources in the region.)

<div align="right"><a href="book.pdf#page=446" target="_blank" title="This page in the PDF"><sup>[p. 446]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p447" href="book.pdf#page=447" target="_blank" title="This page in the PDF"><sup>[p. 447]</sup></a>

<div align="center">

The total Hamiltonian we are aiming at, is the integral of the Hamiltonian density.

</div>

```math
H = \int d^{3} x \mathcal{H}
```

When we integrate over the volume only products like $`e^{ik_{\rho} x_{\rho}} e^{- ik_{\rho} x_{\rho}}`$ will give a nonzero result. So when we multiply one sum over $`k`$ by another, only the terms with the same $`k`$ will contribute to the integral, basically because the waves with different wave number are orthogonal.

```math
\frac{1}{V} \int d^{3} xe^{ik_{\rho} x_{\rho}} e^{- ik_{\rho}^{\prime} x_{\rho}} = \delta_{kk^{\prime}}
```

```math
H = \int d^{3} x \mathcal{H}
```

```math
\mathcal{H} = - \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}}
```

```math
\frac{\partial A_{\mu}}{\partial x_{4}} = - \frac{1}{\sqrt{V}} \sum_{k \quad \alpha = 1} \sum^{2} \epsilon_{\mu}^{( \alpha )} \left( c_{k,\alpha} ( 0 ) \frac{\omega}{c} e^{ik_{\rho} x_{\rho}} - c_{k,\alpha}^{*} ( 0 ) \frac{\omega}{c} e^{- ik_{\rho} x_{\rho}} \right)
```

```math
H = - \int d^{3} x \frac{\partial A_{\mu}}{\partial x_{4}} \frac{\partial A_{\mu}}{\partial x_{4}}
```

```math
H = - \int d^{3} x \frac{1}{V} \sum_{k \quad \alpha = 1} \sum^{2} \left( c_{k,\alpha} ( 0 ) \frac{\omega}{c} e^{ik_{\rho} x_{\rho}} - c_{k,\alpha}^{*} ( 0 ) \frac{\omega}{c} e^{- ik_{\rho} x_{\rho}} \right)^{2}
```

```math
H = - \sum_{k \quad \alpha = 1} \sum^{2} \left( \frac{\omega}{c} \right)^{2} \left[ - c_{k,\alpha} ( t ) c_{k,\alpha}^{*} ( t ) - c_{k,\alpha}^{*} ( t ) c_{k,\alpha} ( t ) \right]
```

```math
H = \sum_{k \quad \alpha = 1} \sum^{2} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} ( t ) c_{k,\alpha}^{*} ( t ) + c_{k,\alpha}^{*} ( t ) c_{k,\alpha} ( t ) \right]
```

```math
H = \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} ( t ) c_{k,\alpha}^{*} ( t ) + c_{k,\alpha}^{*} ( t ) c_{k,\alpha} ( t ) \right]
```

This is the **result we will use to quantize the field**. We have been careful **not to commute** $`c`$ **and** $`c^{*}`$ here in anticipation of the fact that they do not commute.

It should not be a surprise that the terms that made up the Lagrangian gave a zero contribution because $`\mathcal{L} = \tfrac{1}{2} ( E^{2} - B^{2} )`$ and we know that E and B have the same magnitude in a radiation field. (There is one wrinkle we have glossed over; terms with $`\vec{k}^{\prime} = - \vec{k} . )`$

# <a id="s-33-4-canonical-coordinates-and-momenta"></a>33.4 Canonical Coordinates and Momenta

We now have the **Hamiltonian for the radiation field**.

```math
H = \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} ( t ) c_{k,\alpha}^{*} ( t ) + c_{k,\alpha}^{*} ( t ) c_{k,\alpha} ( t ) \right]
```

<div align="right"><a href="book.pdf#page=447" target="_blank" title="This page in the PDF"><sup>[p. 447]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p448" href="book.pdf#page=448" target="_blank" title="This page in the PDF"><sup>[p. 448]</sup></a>

It was **with the Hamiltonian that we first quantized the non-relativistic motion of parti-** **cles**. The **position and momentum became operators which did not commute**. Lets define $`c_{k,\alpha}`$ to be the time dependent Fourier coefficient.

```math
\ddot{c}_{k,\alpha} = - \omega^{2} c_{k,\alpha}
```

We can then **simplify our notation a bit**.

```math
H = \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} c_{k,\alpha}^{*} + c_{k,\alpha}^{*} c_{k,\alpha} \right]
```

This now **clearly looks like the Hamiltonian for a collection of uncoupled oscillators**; one oscillator for each wave vector and polarization.

We wish to write the Hamiltonian in terms of a coordinate for each oscillator and the conjugate momenta. The coordinate should be real so it can be represented by a Hermitian operator and have a physical meaning. The simplest choice for a real coordinates is $`c + c^{*}`$. With a little effort we can **identify the coordinate**

```math
Q_{k,\alpha} = \frac{1}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} )
```

and its **conjugate momentum** for each oscillator,

```math
P_{k,\alpha} = - \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} ) .
```

The **Hamiltonian can be written in terms of these**.

```math
H = \frac{1}{2} \sum_{k,\alpha} \left[ P_{k,\alpha}^{2} + \omega^{2} Q_{k,\alpha}^{2} \right]
```

```math
= \frac{1}{2} \sum_{k,\alpha} \left[ - \left( \frac{\omega}{c} \right)^{2} ( c_{k,\alpha} - c_{k,\alpha}^{*} )^{2} + \left( \frac{\omega}{c} \right)^{2} ( c_{k,\alpha} + c_{k,\alpha}^{*} )^{2} \right]
```

```math
= \frac{1}{2} \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ - ( c_{k,\alpha} - c_{k,\alpha}^{*} )^{2} + ( c_{k,\alpha} + c_{k,\alpha}^{*} )^{2} \right]
```

```math
= \frac{1}{2} \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} 2 \left[ c_{k,\alpha} c_{k,\alpha}^{*} + c_{k,\alpha}^{*} c_{k,\alpha} \right]
```

```math
= \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} c_{k,\alpha}^{*} + c_{k,\alpha}^{*} c_{k,\alpha} \right]
```

This verifies that this choice gives the right Hamiltonian. We should also check that this choice of **coordinates and momenta satisfy Hamilton’s equations** to identify them as the canonical coordinates. The first equation is

```math
\frac{\partial H}{\partial Q_{k,\alpha}} = - \dot{P}_{k,\alpha}
```

```math
\omega^{2} Q_{k,\alpha} = \frac{i\omega}{c} ( \dot{c}_{k,\alpha} - \dot{c}_{k,\alpha}^{*} )
```

```math
\frac{\omega^{2}}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} ) = \frac{i\omega}{c} ( - i\omega c_{k,\alpha} - i\omega c_{k,\alpha}^{*} )
```

```math
\frac{\omega^{2}}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} ) = \frac{\omega^{2}}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} )
```

<div align="right"><a href="book.pdf#page=448" target="_blank" title="This page in the PDF"><sup>[p. 448]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p449" href="book.pdf#page=449" target="_blank" title="This page in the PDF"><sup>[p. 449]</sup></a>

This one checks out OK.

The **other equation of Hamilton** is

```math
\frac{\partial H}{\partial P_{k,\alpha}} = \dot{Q}_{k,\alpha}
```

```math
P_{k,\alpha} = \frac{1}{c} ( \dot{c}_{k,\alpha} + \dot{c}_{k,\alpha}^{*} )
```

```math
- \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} ) = \frac{1}{c} ( - i\omega c_{k,\alpha} + i\omega c_{k,\alpha}^{*} )
```

```math
- \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} ) = - \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} )
```

This also checks out, so **we have identified the canonical coordinates and momenta** of our oscillators.

We have a collection of uncoupled oscillators with identified canonical coordinate and momentum. The next step is to quantize the oscillators.

# <a id="s-33-5-quantization-of-the-oscillators"></a>33.5 Quantization of the Oscillators

To summarize the result of the calculations of the last section we have the **Hamiltonian for the** **radiation field**.

```math
H = \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} c_{k,\alpha}^{*} + c_{k,\alpha}^{*} c_{k,\alpha} \right]
```

```math
Q_{k,\alpha} = \frac{1}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} )
```

```math
P_{k,\alpha} = - \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} )
```

```math
H = \frac{1}{2} \sum_{k,\alpha} \left[ P_{k,\alpha}^{2} + \omega^{2} Q_{k,\alpha}^{2} \right]
```

Soon after the development of non-relativistic quantum mechanics, Dirac proposed that the canonical variables of the radiation oscillators be treated like $`p`$ and $`x`$ in the quantum mechanics we know. The place to start is with the commutators. **The coordinate and its corresponding momentum** **do not commute.** For example $`[ p_{x} ,x ] = \tfrac{\hbar}{i}`$. Coordinates and momenta that do not correspond, do commute. For example $`[ p_{y} ,x ]`$ = 0. Different coordinates commute with each other as do different momenta. We will impose the same rules here.

```math
[ Q_{k,\alpha} ,P_{k^{\prime} ,\alpha^{\prime}} ] = i \hbar \delta_{kk^{\prime}} \delta_{\alpha \alpha^{\prime}}
```

```math
[ Q_{k,\alpha} ,Q_{k^{\prime} ,\alpha^{\prime}} ] = 0
```

```math
[ P_{k,\alpha} ,P_{k^{\prime} ,\alpha^{\prime}} ] = 0
```

By now we know that if the $`Q`$ and $`P`$ do not commute, neither do the $`c`$ and $`c^{*}`$ so we should continue to avoid commuting them.

<div align="right"><a href="book.pdf#page=449" target="_blank" title="This page in the PDF"><sup>[p. 449]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p450" href="book.pdf#page=450" target="_blank" title="This page in the PDF"><sup>[p. 450]</sup></a>

Since we are dealing with harmonic oscillators, we want to find the analog of the **raising and** **lowering operators**. We developed the raising and lowering operators by trying to write the Hamiltonian as $`H = A^{\dagger} A \hbar \omega`$. Following the same idea, we get

```math
a_{k,\alpha} = \frac{1}{\sqrt{2 \hbar \omega}} ( \omega Q_{k,\alpha} + iP_{k,\alpha} )
```

```math
a_{k,\alpha}^{\dagger} = \frac{1}{\sqrt{2 \hbar \omega}} ( \omega Q_{k,\alpha} - iP_{k,\alpha} )
```

```math
a_{k,\alpha}^{\dagger} a_{k,\alpha} = \frac{1}{2 \hbar \omega} ( \omega Q_{k,\alpha} - iP_{k,\alpha} ) ( \omega Q_{k,\alpha} + iP_{k,\alpha} )
```

```math
= \frac{1}{2 \hbar \omega} ( \omega^{2} Q_{k,\alpha}^{2} + P_{k,\alpha}^{2} + i\omega Q_{k,\alpha} P_{k,\alpha} - i\omega P_{k,\alpha} Q_{k,\alpha} )
```

```math
= \frac{1}{2 \hbar \omega} ( \omega^{2} Q_{k,\alpha}^{2} + P_{k,\alpha}^{2} + i\omega Q_{k,\alpha} P_{k,\alpha} - i\omega ( Q_{k,\alpha} P_{k,\alpha} + \frac{\hbar}{i} ) )
```

```math
= \frac{1}{2 \hbar \omega} ( \omega^{2} Q_{k,\alpha}^{2} + P_{k,\alpha}^{2} - \hbar \omega )
```

```math
a_{k,\alpha}^{\dagger} a_{k,\alpha} + \frac{1}{2} = \frac{1}{2 \hbar \omega} ( \omega^{2} Q_{k,\alpha}^{2} + P_{k,\alpha}^{2} )
```

```math
\left( a_{k,\alpha}^{\dagger} a_{k,\alpha} + \frac{1}{2} \right) \hbar \omega = \frac{1}{2} ( \omega^{2} Q_{k,\alpha}^{2} + P_{k,\alpha}^{2} ) = H
```

```math
H = \left( a_{k,\alpha}^{\dagger} a_{k,\alpha} + \frac{1}{2} \right) \hbar \omega
```

This is just the **same as the Hamiltonian that we had for the one dimensional harmonic** **oscillator**. We therefore have the raising and lowering operators, as long as $`[ a_{k,\alpha} ,a_{k,\alpha}^{\dagger} ]`$ = 1, as we had for the 1D harmonic oscillator.

```math
\left[ a_{k,\alpha} ,a_{k,\alpha}^{\dagger} \right] = [ \frac{1}{\sqrt{2 \hbar \omega}} ( \omega Q_{k,\alpha} + iP_{k,\alpha} ) , \frac{1}{\sqrt{2 \hbar \omega}} ( \omega Q_{k,\alpha} - iP_{k,\alpha} ) ]
```

```math
= \frac{1}{2 \hbar \omega} [ \omega Q_{k,\alpha} + iP_{k,\alpha} ,\omega Q_{k,\alpha} - iP_{k,\alpha} ]
```

```math
= \frac{1}{2 \hbar \omega} ( - i\omega [ Q_{k,\alpha} ,P_{k,\alpha} ] + i\omega [ P_{k,\alpha} ,Q_{k,\alpha} ] )
```

```math
= \frac{1}{2 \hbar \omega} ( \hbar \omega + \hbar \omega )
```

```math
= 1
```

So these are definitely the raising and lowering operators. Of course the commutator would be zero if the operators were not for the same oscillator.

```math
[ a_{k,\alpha} ,a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} ] = \delta_{kk^{\prime}} \delta_{\alpha \alpha^{\prime}}
```

(Note that all of our commutators are assumed to be taken at equal time.) The Hamiltonian is written in terms $`a`$ and $`a^{\dagger}`$ in the same way as for the 1D harmonic oscillator. Therefore, everything we know about the raising and lowering operators (See section 10.3) applies here, including the commutator with the Hamiltonian, the raising and lowering of energy eigenstates, and even the

<div align="right"><a href="book.pdf#page=450" target="_blank" title="This page in the PDF"><sup>[p. 450]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p451" href="book.pdf#page=451" target="_blank" title="This page in the PDF"><sup>[p. 451]</sup></a>

constants.

```math
a_{k,\alpha} | n_{k,\alpha} \rangle = \sqrt{n_{k,\alpha}} | n_{k,\alpha} - 1 \rangle
```

```math
a_{k,\alpha}^{\dagger} | n_{k,\alpha} \rangle = \sqrt{n_{k,\alpha} + 1} | n_{k,\alpha} + 1 \rangle
```

The $`n_{k,\alpha}`$ can only take on **integer values** as with the harmonic oscillator we know.

As with the 1D harmonic oscillator, we also can define the **number operator**.

```math
H = \left( a_{k,\alpha}^{\dagger} a_{k,\alpha} + \frac{1}{2} \right) \hbar \omega = \left( N_{k,\alpha} + \frac{1}{2} \right) \hbar \omega
```

```math
N_{k,\alpha} = a_{k,\alpha}^{\dagger} a_{k,\alpha}
```

The last step is to **compute the raising and lowering operators in terms of the original** **coefficients**.

```math
a_{k,\alpha} = \frac{1}{\sqrt{2 \hbar \omega}} ( \omega Q_{k,\alpha} + iP_{k,\alpha} )
```

```math
Q_{k,\alpha} = \frac{1}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} )
```

```math
P_{k,\alpha} = - \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} )
```

```math
a_{k,\alpha} = \frac{1}{\sqrt{2 \hbar \omega}} ( \omega \frac{1}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} ) - i \frac{i\omega}{c} ( c_{k,\alpha} - c_{k,\alpha}^{*} ) )
```

```math
= \frac{1}{\sqrt{2 \hbar \omega}} \frac{\omega}{c} ( ( c_{k,\alpha} + c_{k,\alpha}^{*} ) + ( c_{k,\alpha} - c_{k,\alpha}^{*} ) )
```

```math
= \frac{1}{\sqrt{2 \hbar \omega}} \frac{\omega}{c} ( c_{k,\alpha} + c_{k,\alpha}^{*} + c_{k,\alpha} - c_{k,\alpha}^{*} )
```

```math
= \sqrt{\frac{\omega}{2 \hbar c^{2}}} ( 2 c_{k,\alpha} )
```

```math
= \sqrt{\frac{2 \omega}{\hbar c^{2}}} c_{k,\alpha}
```

```math
c_{k,\alpha} = \sqrt{\frac{\hbar c^{2}}{2 \omega}} a_{k,\alpha}
```

Similarly we can compute that

```math
c_{k,\alpha}^{*} = \sqrt{\frac{\hbar c^{2}}{2 \omega}} a_{k,\alpha}^{\dagger}
```

Since we now have the coefficients in our decomposition of the field equal to a constant times the raising or lowering operator, it is clear that **these coefficients have themselves become** **operators**.

<div align="right"><a href="book.pdf#page=451" target="_blank" title="This page in the PDF"><sup>[p. 451]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p452" href="book.pdf#page=452" target="_blank" title="This page in the PDF"><sup>[p. 452]</sup></a>

# <a id="s-33-6-photon-states"></a>33.6 Photon States

It is now obvious that the integer $`n_{k,\alpha}`$ is the **number of photons in the volume** with wave number $`\vec{k}`$ and polarization $`\hat{\epsilon}^{( \alpha )}`$. It is called the **occupation number** for the state designated by wave number $`\vec{k}`$ and polarization $`\hat{\epsilon}^{( \alpha )}`$. We can represent the state of the entire volume by giving the number of photons of each type (and some phases). The state vector for the volume is given by the direct product of the states for each type of photon.

```math
| n_{k_{1} ,\alpha_{1}} ,n_{k_{2} ,\alpha_{2}} ,\dots ,n_{k_{i} ,\alpha_{i}} ,\dots \rangle = | n_{k_{1} ,\alpha_{1}} \rangle | n_{k_{2} ,\alpha_{2}} \rangle \dots , | n_{k_{i} ,\alpha_{i}} \rangle \dots
```

The ground state for a particular oscillator cannot be lowered. The state in which **all the oscillators** **are in the ground state is called the vacuum state** and can be written simply as $`| 0 \rangle`$. We can generate any state we want by applying raising operators to the vacuum state.

```math
| n_{k_{1} ,\alpha_{1}} ,n_{k_{2} ,\alpha_{2}} ,\dots ,n_{k_{i} ,\alpha_{i}} ,\dots \rangle = \prod_{i} \frac{( a_{k_{i} ,\alpha_{i}}^{\dagger} )^{n_{ki,\alpha i}}}{\sqrt{n_{k_{i} ,\alpha_{i}} !}} | 0 \rangle
```

The factorial on the bottom cancels all the $`\sqrt{n}`$ + 1 we get from the raising operators.

Any multi-photon state we construct is **automatically symmetric under the interchange of** **pairs of photons**. For example if we want to raise two photons out of the vacuum, we apply two raising operators. Since $`[ a_{k,\alpha}^{\dagger} ,a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} ]`$ = 0, interchanging the photons gives the same state.

```math
a_{k,\alpha}^{\dagger} a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} | 0 \rangle = a_{k^{\prime} ,\alpha^{\prime}}^{\dagger} a_{k,\alpha}^{\dagger} | 0 \rangle
```

So the fact that the **creation operators commute dictates that photon states are symmetric** **under interchange**.

# <a id="s-33-7-fermion-operators"></a>33.7 Fermion Operators

At this point, we can hypothesize that the **operators that create fermion states do not com-** **mute**. In fact, if we assume that the **operators creating fermion states anti-commute** (as do the Pauli matrices), then we can show that fermion states are antisymmetric under interchange. Assume $`b_{r}^{\dagger}`$ and $`b_{r}`$ are the creation and annihilation operators for fermions and that they anti-commute.

```math
\{ b_{r}^{\dagger} ,b_{r^{\prime}}^{\dagger} \} = 0
```

The **states are then antisymmetric under interchange** of pairs of fermions.

```math
b_{r}^{\dagger} b_{r}^{\dagger^{\prime}} | 0 \rangle = - b_{r}^{\dagger^{\prime}} b_{r}^{\dagger} | 0 \rangle
```

Its not hard to show that the **occupation number for fermion states is either zero or one**.

<div align="right"><a href="book.pdf#page=452" target="_blank" title="This page in the PDF"><sup>[p. 452]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p453" href="book.pdf#page=453" target="_blank" title="This page in the PDF"><sup>[p. 453]</sup></a>

# <a id="s-33-8-quantized-radiation-field"></a>33.8 Quantized Radiation Field

The Fourier coefficients of the expansion of the classical radiation field should now be **replaced by** **operators**.

```math
c c_{k,\alpha k,\alpha}^{*} A_{\mu} \quad \to \to = \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\frac{\frac{\sqrt{\frac{\hbar c^{2}}{2 \omega}}}{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} a_{k,\alpha}}{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} a_{k,\alpha}^{\dagger}} \epsilon_{\mu}^{( \alpha )} \left( a_{k,\alpha} ( t ) e^{i \vec{k} \cdot \vec{x}} + a_{k,\alpha}^{\dagger} ( t ) e^{- i \vec{k} \cdot \vec{x}} \right)
```

$`A`$ is now an operator that acts on state vectors in occupation number space. The operator is parameterized in terms of $`\vec{x}`$ and $`t`$. This type of operator is called a **field operator** or a **quantized** **field**.

The Hamiltonian operator can also be written in terms of the creation and annihilation operators.

```math
H = \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \left[ c_{k,\alpha} c_{k,\alpha}^{*} + c_{k,\alpha}^{*} c_{k,\alpha} \right]
```

```math
= \sum_{k,\alpha} \left( \frac{\omega}{c} \right)^{2} \frac{\hbar c^{2}}{2 \omega} \left[ a_{k,\alpha} a_{k,\alpha}^{\dagger} + a_{k,\alpha}^{\dagger} a_{k,\alpha} \right]
```

```math
= \frac{1}{2} \sum_{k,\alpha} \hbar \omega \left[ a_{k,\alpha} a_{k,\alpha}^{\dagger} + a_{k,\alpha}^{\dagger} a_{k,\alpha} \right]
```

```math
H = \sum_{k,\alpha} \hbar \omega \left( N_{k,\alpha} + \frac{1}{2} \right)
```

For our purposes, we may remove the (infinite) constant energy due to the ground state energy of all the oscillators. It is simply the energy of the vacuum which we may define as zero. Note that the field fluctuations that cause this energy density, also cause the spontaneous decay of excited states of atoms. One thing that must be done is to **cut off** the sum at some maximum value of $`k`$. We do not expect electricity and magnetism to be completely valid up to infinite energy. Certainly by the gravitational or grand unified energy scale there must be important corrections to our formulas. The **energy density of the vacuum** is hard to define but plays an important role in cosmology. At this time, physicists have difficulty explaining how small the energy density in the vacuum is. Until recent experiments showed otherwise, most physicists thought it was actually zero due to some unknown symmetry. In any case we are not ready to consider this problem.

```math
H = \sum_{k,\alpha} \hbar \omega N_{k,\alpha}
```

<div align="center">

With this subtraction, the energy of the vacuum state has been defined to be zero.

</div>

```math
H | 0 \rangle = 0
```

<div align="right"><a href="book.pdf#page=453" target="_blank" title="This page in the PDF"><sup>[p. 453]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p454" href="book.pdf#page=454" target="_blank" title="This page in the PDF"><sup>[p. 454]</sup></a>

The total momentum in the (transverse) radiation field can also be computed (from the classical formula for the Poynting vector).

```math
\vec{P} = \frac{1}{c} \int \vec{E} \times \vec{B} d^{3} x = \sum_{k,\alpha} \hbar \vec{k} \left( N_{k,\alpha} + \frac{1}{2} \right)
```

This time the $`\frac{1}{2}`$ can really be dropped since the sum is over positive and negative $`\vec{k}`$, so it sums to zero.

```math
\vec{P} = \sum_{k,\alpha} \hbar \vec{k} N_{k,\alpha}
```

We can compute the energy and momentum of a single photon state by operating on that state with the Hamiltonian and with the total momentum operator. The state for a single photon with a given momentum and polarization can be written as $`a_{k,\alpha}^{\dagger} | 0 \rangle`$.

```math
Ha_{k,\alpha}^{\dagger} | 0 \rangle = \left( a_{k,\alpha}^{\dagger} H + [ H,a_{k,\alpha}^{\dagger} ] \right) | 0 \rangle = 0 + \hbar \omega a_{k,\alpha}^{\dagger} | 0 \rangle = \hbar \omega a_{k,\alpha}^{\dagger} | 0 \rangle
```

The energy of single photon state is $`\hbar \omega`$.

```math
Pa_{k,\alpha}^{\dagger} | 0 \rangle = \left( a_{k,\alpha}^{\dagger} P + [ P,a_{k,\alpha}^{\dagger} ] \right) | 0 \rangle = 0 + \hbar \vec{k} a_{k,\alpha}^{\dagger} | 0 \rangle = \hbar \vec{k} a_{k,\alpha}^{\dagger} | 0 \rangle
```

The momentum of the single photon state is $`\hbar \vec{k}`$. The mass of the photon can be computed.

```math
E^{2} = p^{2} c^{2} + ( mc^{2} )^{2}
```

```math
mc^{2} = \sqrt{( \hbar \omega )^{2} - ( \hbar k )^{2} c^{2}} = \hbar \sqrt{\omega^{2} - \omega^{2}} = 0
```

So the energy, momentum, and mass of a single photon state are as we would expect.

The vector potential has been given two transverse polarizations as expected from classical Electricity and Magnetism. The result is two possible transverse polarization vectors in our quantized field. The photon states are also labeled by one of two polarizations, that we have so far assumed were linear polarizations. The polarization vector, and therefore the vector potential, transform like a Lorentz vector. We know that the matrix element of vector operators (See section 29.9) is associated with an angular momentum of one. When a photon is emitted, selection rules indicate it is carrying away an angular momentum of one, so we deduce that the photon has spin one. We need not add anything to our theory though; the vector properties of the field are already included in our assumptions about polarization.

Of course we could equally well use circular polarizations which are related to the linear set we have been using by

```math
\hat{\epsilon}^{( \pm )} = \mp \frac{1}{\sqrt{2}} ( \hat{\epsilon}^{( 1 )} \pm i \hat{\epsilon}^{( 2 )} ) .
```

The polarization $`\hat{\epsilon}^{( \pm )}`$ is associated with the $`m = \pm 1`$ component of the photon’s spin. These are the transverse mode of the photon, $`\vec{k} \cdot \hat{\epsilon}^{( \pm )}`$ = 0. We have separated the field into transverse and longitudinal parts. The longitudinal part is partially responsible for static E and B fields, while the transverse part makes up radiation. The $`m`$ = 0 component of the photon is not present in radiation but is important in understanding static fields.

By assuming the canonical coordinates and momenta in the Hamiltonian have commutators like those of the position and momentum of a particle, led to an understanding that radiation is made up of spin-1 particles with mass zero. All fields correspond to a particle of definite mass and spin. We now have a pretty good idea how to quantize the field for any particle.

<div align="right"><a href="book.pdf#page=454" target="_blank" title="This page in the PDF"><sup>[p. 454]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p455" href="book.pdf#page=455" target="_blank" title="This page in the PDF"><sup>[p. 455]</sup></a>

# <a id="s-33-9-the-time-development-of-field-operators"></a>33.9 The Time Development of Field Operators

The creation and annihilation operators are related to the **time dependent coefficients** in our Fourier expansion of the radiation field.

```math
c_{k,\alpha} ( t ) = \sqrt{\frac{\hbar c^{2}}{2 \omega}} a_{k,\alpha}
```

```math
c_{k,\alpha}^{*} ( t ) = \sqrt{\frac{\hbar c^{2}}{2 \omega}} a_{k,\alpha}^{\dagger}
```

This means that the creation, annihilation, and other operators are time dependent operators as we have studied the Heisenberg representation (See section 11.6). In particular, we derived the canonical equation for the time dependence of an operator.

```math
\frac{d}{dt} B ( t ) = \frac{i}{\hbar} [ H,B ( t ) ]
```

```math
\dot{a}_{k,\alpha} = \frac{i}{\hbar} [ H,a_{k,\alpha} ( t ) ] = \frac{i}{\hbar} ( - \hbar \omega ) a_{k,\alpha} ( t ) = - i\omega a_{k,\alpha} ( t )
```

```math
\dot{a}_{k,\alpha}^{\dagger} = \frac{i}{\hbar} [ H,a_{k,\alpha}^{\dagger} ( t ) ] = i\omega a_{k,\alpha}^{\dagger} ( t )
```

So the operators have the **same time dependence as did the coefficients** in the Fourier expansion.

```math
a_{k,\alpha} ( t ) = a_{k,\alpha} ( 0 ) e^{- i\omega t}
```

```math
a_{k,\alpha}^{\dagger} ( t ) = a_{k,\alpha}^{\dagger} ( 0 ) e^{i\omega t}
```

<div align="center">

We can now write the quantized radiation field in terms of the operators at $`t`$ = 0.

</div>

```math
A_{\mu} = \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \epsilon_{\mu}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

Again, the 4-vector $`x_{\rho}`$ is a **parameter of this field**, not the location of a photon. The **field** **operator is Hermitian** and the field itself is real.

# <a id="s-33-10uncertainty-relations-and-rms-field-fluctuations"></a>33.10 Uncertainty Relations and RMS Field Fluctuations

Since the fields are a sum of creation and annihilation operators, they **do not commute with the** **occupation number operators**

```math
N_{k,\alpha} = a_{k,\alpha}^{\dagger} a_{k,\alpha} .
```

Observables corresponding to operators which do not commute have an uncertainty principle between them. So we **can’t fix the number of photons and know the fields** exactly. Fluctuations in the field take place even in the vacuum state, where we know there are no photons.

<div align="right"><a href="book.pdf#page=455" target="_blank" title="This page in the PDF"><sup>[p. 455]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p456" href="book.pdf#page=456" target="_blank" title="This page in the PDF"><sup>[p. 456]</sup></a>

Of course the average value of the Electric or Magnetic field vector is zero by symmetry. To get an idea about the size of field fluctuations, we should look at the **mean square value of the field**, for example **in the vacuum state**. We compute $`\langle 0 | \vec{E} \cdot \vec{E} | 0 \rangle`$.

```math
\vec{E} = - \frac{1}{c} \frac{\partial \vec{A}}{\partial t}
```

```math
A_{\mu} = \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \epsilon_{\mu}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\vec{A} = \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \hat{\epsilon}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\vec{E} = - i \frac{1}{c} \frac{1}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar c^{2}}{2 \omega}}} \hat{\epsilon}^{( \alpha )} \left( - \omega a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + \omega a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\vec{E} = \frac{i}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar \omega}{2}}} \hat{\epsilon}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} - a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right)
```

```math
\vec{E} | 0 \rangle = \frac{i}{\sqrt{V}} \sum_{k\alpha}^{\sqrt{\frac{\hbar \omega}{2}}} \hat{\epsilon}^{( \alpha )} \left( - a_{k,\alpha}^{\dagger} e^{- ik_{\rho} x_{\rho}} \right) | 0 \rangle
```

```math
\langle 0 | \vec{E} \cdot \vec{E} | 0 \rangle = \frac{1}{V} \sum_{k\alpha} \frac{\hbar \omega}{2} 1
```

```math
\langle 0 | \vec{E} \cdot \vec{E} | 0 \rangle = \frac{1}{V} \sum_{k} \hbar \omega \to \infty
```

(Notice that we are **basically taking the absolute square** of $`\vec{E} | 0 \rangle`$ and that the orthogonality of the states collapses the result down to a single sum.)

The calculation is illustrative even though the answer is infinite. Basically, a term proportional to $`aa^{\dagger}`$ **first creates one photon then absorbs it** giving a nonzero contribution for every oscillator mode. The terms sum to infinity but really its the infinitesimally short wavelengths that cause this. Again, some cut off in the maximum energy would make sense.

The **effect of these field fluctuations on particles** is mitigated by quantum mechanics. In reality, any quantum particle will be spread out over a finite volume and its the average field over the volume that might cause the particle to experience a force. So we could average the Electric field over a volume, then take the mean square of the average. If we average over a cubic volume ∆$`V`$ = ∆$`l^{3}`$, then we find that

```math
\langle 0 | \vec{E} \cdot \vec{E} | 0 \rangle \approx \frac{\hbar c}{\Delta l^{4}} .
```

Thus if we can probe short distances, the effective size of the fluctuations increases.

Even the **E and B fields do not commute**. It can be shown that

```math
[ E_{x} ( x ) ,B_{y} ( x^{\prime} ) ] = ic \hbar \delta ( ds = \sqrt{( x - x^{\prime} )_{\rho} ( x - x^{\prime} )_{\rho}} )
```

There is a nonzero commutator of the two spacetime points are connected by a light-like vector. Another way to say this is that the **commutator is non-zero if the coordinates are simultaneous**. This is a reasonable result considering causality.

<div align="right"><a href="book.pdf#page=456" target="_blank" title="This page in the PDF"><sup>[p. 456]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p457" href="book.pdf#page=457" target="_blank" title="This page in the PDF"><sup>[p. 457]</sup></a>

To make a narrow beam of light, one must adjust the phases of various components of the beam carefully. Another version of the uncertainty relation is that ∆$`N`$ ∆$`\varphi \geq 1`$, where $`phi`$ is the phase of a Fourier component and $`N`$ is the number of photons.

Of course the Electromagnetic **waves of classical physics usually have very large numbers** **of photons** and the quantum effects are not apparent. A good condition to identify the boundary between classical and quantum behavior is that for the classical E&M to be correct the **number of** **photons per cubic wavelength should be much greater than 1**.

# <a id="s-33-11emission-and-absorption-of-photons-by-atoms"></a>33.11 Emission and Absorption of Photons by Atoms

The interaction of an electron (See section 29.1) with the quantized field is already in the **standard** **Hamiltonian**.

```math
H = \frac{1}{2 m} \left( \vec{p} + \frac{e}{c} \vec{A} \right)^{2} + V ( r )
```

```math
H_{int} = - \frac{e}{2 mc} ( \vec{p} \cdot \vec{A} + \vec{A} \cdot \vec{p} ) + \frac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A}
```

```math
= - \frac{e}{mc} \vec{A} \cdot \vec{p} + \frac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A}
```

For completeness we should add the **interaction with the spin of the electron** $`H = - \vec{\mu} \cdot \vec{B}`$.

```math
H_{int} = - \frac{e}{mc} \vec{A} \cdot \vec{p} + \frac{e^{2}}{2 mc^{2}} \vec{A} \cdot \vec{A} - \frac{e \hbar}{2 mc} \vec{\sigma} \cdot \vec{\nabla} \times \vec{A}
```

For an atom with many electrons, we must **sum over all the electrons**. The field is evaluated at the coordinate $`x`$ which should be that of the electron.

This interaction Hamiltonian contains operators to create and annihilate photons with transitions between atomic states. From our previous study of time dependent perturbation theory (See section 28.1), we know that transitions between initial and final states are proportional to the **matrix** **element of the perturbing Hamiltonian** between the states, $`\langle n | H_{int} | i \rangle`$. The initial state $`| i \rangle`$ should include a **direct product of the atomic state and the photon state**. Lets concentrate on one type of photon for now. We then could write

```math
| i \rangle = | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle
```

with a similar expression for the final state.

We will first consider the **absorption of one photon from the field**. Assume there are $`n_{\vec{k} ,\alpha}`$ photons of this type in the initial state and that one photon is absorbed. We therefore will need a term in the interaction Hamiltonian that contains on annihilation operator (only). This will just come from the linear term in A.

```math
\langle n | H_{int} | i \rangle = \langle \psi_{n} ; n_{\vec{k} ,\alpha} - 1 |- \frac{e}{mc} \vec{A} \cdot \vec{p} | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle
```

```math
= - \frac{e}{mc} \langle \psi_{n} ; n_{\vec{k} ,\alpha} - 1 | \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar c^{2}}{2 \omega} \hat{\epsilon}^{( \alpha )} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} + a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} \right) \cdot \vec{p} | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle}
```

<div align="right"><a href="book.pdf#page=457" target="_blank" title="This page in the PDF"><sup>[p. 457]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p458" href="book.pdf#page=458" target="_blank" title="This page in the PDF"><sup>[p. 458]</sup></a>

```math
\langle n | H_{int}^{( abs )} | i \rangle = - \frac{e}{mc} \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar c^{2}}{2 \omega}} \langle \psi_{n} ; n_{\vec{k} ,\alpha} - 1 | \hat{\epsilon}^{( \alpha )} \cdot \vec{p} \left( a_{k,\alpha} ( 0 ) e^{ik_{\rho} x_{\rho}} \right) | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle
```

```math
= - \frac{e}{m} \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar}{2 \omega}} \langle \psi_{n} ; n_{\vec{k} ,\alpha} - 1 | \hat{\epsilon}^{( \alpha )} \cdot \vec{p} \sqrt{n_{\vec{k} ,\alpha}} e^{ik_{\rho} x_{\rho}} | \psi_{i} ; n_{\vec{k} ,\alpha} - 1 \rangle
```

```math
= - \frac{e}{m} \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar n_{\vec{k} ,\alpha}}{2 \omega} \langle \psi_{n} | e^{i \vec{k} \cdot \vec{r}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p} | \psi_{i} \rangle e^{- i\omega t}}
```

Similarly, for the **emission of a photon** the matrix element is.

```math
\langle n | H_{int} | i \rangle = \langle \psi_{n} ; n_{\vec{k} ,\alpha} + 1 |- \frac{e}{mc} \vec{A} \cdot \vec{p} | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle
```

```math
\langle n | H_{int}^{( emit )} | i \rangle = - \frac{e}{mc} \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar c^{2}}{2 \omega} \langle \psi_{n} ; n_{\vec{k} ,\alpha} + 1 | \hat{\epsilon}^{( \alpha )} \cdot \vec{p} a_{k,\alpha}^{\dagger} ( 0 ) e^{- ik_{\rho} x_{\rho}} | \psi_{i} ; n_{\vec{k} ,\alpha} \rangle}
```

```math
= - \frac{e}{m} \frac{1}{\sqrt{V}} \sqrt{\frac{\hbar ( n_{\vec{k} ,\alpha} + 1 )}{2 \omega} \langle \psi_{n} | e^{- i \vec{k} \cdot \vec{r}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p} | \psi_{i} \rangle e^{i\omega t}}
```

These give the same result as our earlier guess to put an $`n`$+ 1 in the emission operator (See Section 29.1).

# <a id="s-33-12review-of-radiation-of-photons"></a>33.12 Review of Radiation of Photons

In the previous section, we derived the same formulas for matrix elements (See Section 29.1) that we had earlier used to study decays of Hydrogen atom states with no applied EM field, that is **zero** **photons in the initial state**.

```math
\Gamma_{i \to n} = \frac{( 2 \pi )^{2} e^{2}}{m^{2} \omega V} | \langle \varphi_{n} | e^{- i \vec{k} \cdot \vec{r}} \hat{\epsilon} \cdot \vec{p} | \varphi_{i} \rangle |^{2} \delta ( E_{n} - E_{i} + \hbar \omega )
```

With the inclusion of the **phase space integral** over final states this became

```math
\Gamma_{tot} = \frac{e^{2} ( E_{i} - E_{n} )}{2 \pi \hbar^{2} m^{2} c^{3}} \sum_{\lambda} \int d \Omega_{p} | \langle \varphi_{n} | e^{- i \vec{k} \cdot \vec{r}} \hat{\epsilon}^{( \lambda )} \cdot \vec{p}_{e} | \varphi_{i} \rangle |^{2}
```

The quantity $`\vec{k} \cdot \vec{r}`$ is typically small for atomic transitions

```math
E_{\gamma} = pc = \hbar kc \approx \frac{1}{2} \alpha^{2} mc^{2}
```

```math
r \approx a_{0} = \frac{\hbar}{\alpha mc}
```

```math
kr \approx \frac{1}{2} \frac{\alpha^{2} mc}{\hbar} \frac{\hbar}{\alpha mc} = \frac{\alpha}{2}
```

Note that we have take the full binding energy as the energy difference between states so almost all transitions will have $`kr`$ smaller than this estimate. This makes $`\vec{k} \cdot \vec{r}`$ an excellent parameter in which to expand decay rate formulas.

<div align="right"><a href="book.pdf#page=458" target="_blank" title="This page in the PDF"><sup>[p. 458]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p459" href="book.pdf#page=459" target="_blank" title="This page in the PDF"><sup>[p. 459]</sup></a>

The approximation that $`e^{- i \vec{k} \cdot \vec{r}} \approx 1`$ is a very good one and is called the **electric dipole or E1** **approximation**. We previously derived the E1 selection rules (See Section 29.5).

```math
\Delta \ell = \pm 1 .
```

```math
\Delta m = 0 , \pm 1 .
```

```math
\Delta s = 0 .
```

The **general E1 decay result depends on photon direction and polarization**. If information about angular distributions or polarization is needed, it can be pried out of this formula.

```math
\Gamma_{tot} = \frac{e^{2} ( E_{i} - E_{n} )}{2 \pi \hbar^{2} m^{2} c^{3}} \sum_{\lambda} \int d \Omega_{\gamma} | \langle \varphi_{n} | e^{- i \vec{k} \cdot \vec{r}} \hat{\epsilon}^{( \lambda )} \cdot \vec{p}_{e} | \varphi_{i} \rangle |^{2}
```

```math
\approx \frac{\alpha \omega_{in}^{3}}{2 \pi c^{2}} \sum_{\lambda} \int d \Omega_{\gamma} \left| \int_{0}^{\sqrt{\frac{4 \pi}{3}}^{\infty}} r^{3} drR_{n_{n} \ell_{n}}^{*} R_{n_{i} \ell_{i}} \int d \Omega Y_{\ell_{n} m_{n}}^{*} \left( \epsilon_{z} Y_{1 0} + \frac{- \epsilon_{x} + i\epsilon_{y}}{\sqrt{2}} Y_{1 1} + \frac{\epsilon_{x} + i\epsilon_{y}}{\sqrt{2}} Y_{1 - 1} \right) Y_{\ell_{i} m_{i}} \right|^{2}
```

**Summing over polarization and integrating over photon direction**, we get a simpler formula that is quite useful to compute the decay rate from one initial atomic state to one final atomic state.

```math
\Gamma_{tot} = \frac{4 \alpha \omega_{in}^{3}}{3 c^{2}} | \vec{r}_{ni} |^{2}
```

Here $`\vec{r}_{ni}`$ is the matrix element of the coordinate vector between final and initial states.

For single electron atoms, we can **sum over the final states** with different $`m`$ and get a formula only requires us to do a radial integral.

```math
\Gamma_{tot} = \frac{4 \alpha \omega_{in}^{3}}{3 c^{2}} \begin{Bmatrix} \tfrac{\ell + 1}{2 \ell + 1} \\ \tfrac{\ell}{2 \ell + 1} \end{Bmatrix} \left| \int_{0}^{\infty} R_{n^{\prime} \ell^{\prime}}^{*} R_{n\ell} r^{3} dr \right|^{2} \quad f or \quad \ell^{\prime} = \{ \} \frac{\ell + 1}{\ell - 1}
```

The decay rate does not depend on the $`m`$ of the initial state.

## <a id="s-33-12-1-beyond-the-electric-dipole-approximation"></a>33.12.1 Beyond the Electric Dipole Approximation

Some atomic states have no lower energy state that satisfies the E1 selection rules to decay to. Then, higher order processes must be considered. The next order term in the expansion of $`e^{- i \vec{k} \cdot \vec{r}} = 1`$ $`- i \vec{k} \cdot \vec{r} + \dots`$ will allow other transitions to take place but at lower rates. We will attempt to understand the **selection rules** when we include the $`i \vec{k} \cdot \vec{r}`$ term.

The matrix element is proportional to $`- i \langle \varphi_{n} | ( \vec{k} \cdot \vec{r} )`$(ˆ$`\epsilon^{( \lambda )} \cdot \vec{p}_{e} ) | \varphi_{i} \rangle`$ which we will **split up into two** **terms**. You might ask why split it. The reason is that we will essentially be **computing matrix** **elements of at tensor** and dotting it into two vectors that do not depend on the atomic state.

```math
\vec{k} \cdot \langle \varphi_{n} | \vec{r} \vec{p}_{e} ) | \varphi_{i} \rangle \cdot \hat{\epsilon}^{( \lambda )}
```

<div align="right"><a href="book.pdf#page=459" target="_blank" title="This page in the PDF"><sup>[p. 459]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p460" href="book.pdf#page=460" target="_blank" title="This page in the PDF"><sup>[p. 460]</sup></a>

Putting these two vectors together is like adding to $`\ell`$ = 1 states. We can get total angular momentum quantum numbers 2, 1, and 0. Each vector has three components. The **direct product tensor** **has 9**. Its another case of

```math
3 \otimes 3 = 5_{S} \oplus 3_{A} \oplus 1_{S} .
```

The **tensor we make when we just multiply two vectors together can be reduced into** **three irreducible (spherical) tensors**. These are the ones for which we can use the Wigner- Eckart theorem to derive selection rules. Under rotations of the coordinate axes, the rotation matrix for the 9 component Cartesian tensor will be block diagonal. It can be reduced into three spherical tensors. Under rotations the 5 component (traceless) symmetric tensor will always rotate into another 5 component symmetric tensor. The 3 component anti symmetric tensor will rotate into another antisymmetric tensor and the part proportional to the identity will rotate into the identity.

```math
( \vec{k} \cdot \vec{r} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{p}_{e} ) = \frac{1}{2} [ ( \vec{k} \cdot \vec{r} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{p} ) + ( \vec{k} \cdot \vec{p} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{r} ) ] + \frac{1}{2} [ ( \vec{k} \cdot \vec{r} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{p} ) - ( \vec{k} \cdot \vec{p} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{r} ) ]
```

The **first term is symmetric and the second anti-symmetric** by construction.

The **first term can be rewritten**.

```math
\frac{1}{2} \langle \varphi_{n} | [ ( \vec{k} \cdot \vec{r} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{p} ) + ( \vec{k} \cdot \vec{p} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{r} ) ] | \varphi_{i} \rangle = \frac{1}{2} \vec{k} \cdot \langle \varphi_{n} | [ \vec{r} \vec{p} + \vec{p} \vec{r} ] | \varphi_{i} \rangle \cdot \hat{\epsilon}^{( \lambda )}
```

```math
= \frac{1}{2} \vec{k} \cdot \frac{im}{\hbar} \langle \varphi_{n} | [ H_{0} , \vec{r} \vec{r} ] | \varphi_{i} \rangle \cdot \hat{\epsilon}^{( \lambda )}
```

```math
= - \frac{im\omega}{2} \vec{k} \cdot \langle \varphi_{n} | \vec{r} \vec{r} | \varphi_{i} \rangle \cdot \hat{\epsilon}^{( \lambda )}
```

This makes the symmetry clear. Its normal to **remove the trace of the tensor**: $`\vec{r} \vec{r} \to \vec{r} \vec{r} - \tfrac{\delta_{ij}}{3} r^{2}`$. The term proportional to $`\delta_{ij}`$ gives zero because $`\vec{k} \cdot \hat{\epsilon}`$ = 0. The traceless symmetric tensor has 5 components like an $`\ell`$ = 2 operator; The anti-symmetric tensor has 3 components; and the trace term has one. This is the separation of the Cartesian tensor into irreducible spherical tensors. The **five** **components of the traceless symmetric tensor can be written as a linear combination** **of the** $`Y_{2 m}`$.

Similarly, the **second (anti-symmetric) term can be rewritten** slightly.

```math
\frac{1}{2} [ ( \vec{k} \cdot \vec{r} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{p} ) - ( \vec{k} \cdot \vec{p} ) ( \hat{\epsilon}^{( \lambda )} \cdot \vec{r} ) ] = ( \vec{k} \times \hat{\epsilon}^{( \lambda )} ) \cdot ( \vec{r} \times \vec{p} )
```

The **atomic state dependent part of this,** $`\vec{r} \times \vec{p}`$**, is an axial vector** and therefore has three components. (Remember and axial vector is the same thing as an anti-symmetric tensor.) So this is clearly an $`\ell`$ = 1 operator and can be **expanded in terms of the** $`Y_{1 m}`$. Note that it is actually a constant times the **orbital angular momentum operator** $`\vec{L}`$.

So the **first term is reasonably named the Electric Quadrupole** term because it depends on the quadrupole moment of the state. It **does not change parity** and gives us the selection rule.

```math
| \ell_{n} - \ell_{i} |\leq 2 \leq \ell_{n} + \ell_{i}
```

The second term dots the radiation magnetic field into the angular momentum of the atomic state, so it is reasonably called the **magnetic dipole interaction**. The interaction of the **electron spin** with the magnetic field is of the same order and should be included together with the E2 and M1 terms.

```math
\frac{e \hbar}{2 mc} ( \vec{k} \times \hat{\epsilon}^{( \lambda )} ) \cdot \vec{\sigma}
```

<div align="right"><a href="book.pdf#page=460" target="_blank" title="This page in the PDF"><sup>[p. 460]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p461" href="book.pdf#page=461" target="_blank" title="This page in the PDF"><sup>[p. 461]</sup></a>

Higher order terms can be computed but its not recommended.

Some atomic states, such as the **2s state of Hydrogen**, cannot decay by any of these terms basically because the 2s to 1s is a 0 to 0 transition and there is no way to conserve angular momentum and parity. This state can only decay by the emission of two photons.

While E1 transitions in hydrogen have lifetimes as small as $`1 0^{- 9}`$ seconds, the **E2 and M1 transi-** **tions have lifetimes of the order of** $`1 0^{- 3}`$ **seconds**, and the **2s state has a lifetime of about**

$`7 1`$ **of a second**.

# <a id="s-33-13black-body-radiation-spectrum"></a>33.13 Black Body Radiation Spectrum

We are in a position to fairly easily calculate the spectrum of Black Body radiation. Assume there is a **cavity with a radiation field on the inside** and that the field interacts with the atoms of the cavity. Assume **thermal equilibrium** is reached.

Let’s **take two atomic states that can make transitions to each other**: $`A \to B + \gamma`$ and $`B + \gamma \to A`$. **From statistical mechanics**, we have

```math
\frac{N_{B}}{N_{A}} = \frac{e^{- E_{b} /kT}}{e^{- E_{A} /kT}} = e^{\hbar \omega /kT}
```

and **for equilibrium** we must have

```math
N_{B} \Gamma_{absorb} = N_{A} \Gamma_{emit}
```

```math
\frac{N_{B}}{N_{A}} = \frac{\Gamma_{emit}}{\Gamma_{absorb}}
```

We have **previously calculated the emission and absorption rates**. We can calculate the ratio between the emission and absorption rates per atom:

```math
\frac{N_{B}}{N_{A}} = \frac{\Gamma_{emit}}{\Gamma_{absorb}} = \frac{( n_{\vec{k} ,\alpha} + 1 ) \left| \sum_{i} \langle B | e^{- i \vec{k} \cdot \vec{r}_{i}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p}_{i} | A \rangle \right|^{2}}{n_{\vec{k} ,\alpha} \left| \sum_{i} \langle A | e^{i \vec{k} \cdot \vec{r}_{i}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p}_{i} | B \rangle \right|^{2}}
```

where the sum is over atomic electrons. The matrix elements are closely related.

```math
\langle B | e^{- i \vec{k} \cdot \vec{r}_{i}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p}_{i} | A \rangle = \langle A | \vec{p}_{i} \cdot \hat{\epsilon}^{( \alpha )} e^{i \vec{k} \cdot \vec{r}_{i}} | B \rangle^{*} = \langle A | e^{i \vec{k} \cdot \vec{r}_{i}} \hat{\epsilon}^{( \alpha )} \cdot \vec{p}_{i} | B \rangle^{*}
```

We have used the fact that $`\vec{k} \cdot \epsilon`$ = 0. **The two matrix elements are simple complex conjugates** **of each other** so that when we take the absolute square, they are the same. Therefore, we may cancel them.

```math
\frac{N_{B}}{N_{A}} = \frac{( n_{\vec{k} ,\alpha} + 1 )}{n_{\vec{k} ,\alpha}} = e^{\hbar \omega /kT}
```

```math
1 = n_{\vec{k} ,\alpha} ( e^{\hbar \omega /kT} - 1 )
```

```math
n_{\vec{k} ,\alpha} = \frac{1}{e^{\hbar \omega /kT} - 1}
```

<div align="right"><a href="book.pdf#page=461" target="_blank" title="This page in the PDF"><sup>[p. 461]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p462" href="book.pdf#page=462" target="_blank" title="This page in the PDF"><sup>[p. 462]</sup></a>

Now suppose the **walls of the cavity are black** so that they emit and absorb photons at any energy. Then the result for the number of photons above is true for all the radiation modes of the cavity. The **energy in the frequency interval** $`( \omega ,\omega + d\omega )`$ **per unit volume can be calculated** by multiplying the number of photons by the energy per photon times the number of modes in that frequency interval and dividing by the volume of the cavity.

```math
U ( \omega ) d\omega = \frac{\hbar \omega}{e^{\hbar \omega /kT} - 1} 2 \left( \frac{L}{2 \pi} \right)^{3} 4 \pi k^{2} dk \frac{1}{L^{3}}
```

```math
U ( \omega ) = 8 \pi \frac{\hbar \omega}{e^{\hbar \omega /kT} - 1} \left( \frac{1}{2 \pi} \right)^{3} k^{2} \frac{dk}{d\omega}
```

```math
U ( \omega ) = \frac{8 \pi \hbar}{c^{3}} \left( \frac{\omega}{2 \pi} \right)^{3} \frac{1}{e^{\hbar \omega /kT} - 1}
```

```math
U ( \nu ) = U ( \omega ) \frac{d\omega}{d\nu} = \frac{8 \pi}{c^{3}} \frac{h\nu^{3}}{e^{\hbar \omega /kT} - 1}
```

**This was the formula Plank used to start the revolution.**

---

[← 32 Classical Maxwell Fields](32-classical-maxwell-fields.md) · [34 Scattering of Photons →](34-scattering-of-photons.md)
