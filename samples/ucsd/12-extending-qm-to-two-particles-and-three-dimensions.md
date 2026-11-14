[← 11 More Fun with Operators](11-more-fun-with-operators.md) · [13 3D Problems Separable in Cartesian Coordinates →](13-3d-problems-separable-in-cartesian-coordinates.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p191" href="book.pdf#page=191" target="_blank" title="This page in the PDF"><sup>[p. 191]</sup></a>

# 12 Extending QM to Two Particles and Three Dimensions

# <a id="s-12-1-quantum-mechanics-for-two-particles"></a>12.1 Quantum Mechanics for Two Particles

We can know the state of two particles at the same time. The positions and momenta of particle 2 **commute** with the positions and momenta of particle 1.

```math
[ x_{1} ,x_{2} ] = [ p_{1} ,p_{2} ] = [ x_{1} ,p_{2} ] = [ x_{2} ,p_{1} ] = 0
```

The kinetic energy terms in the Hamiltonian are independent. There may be an interaction between the two particles in the potential. The **Hamiltonian for two particles** can be easily written.

```math
H = \frac{p_{1}^{2}}{2 m_{1}} + \frac{p_{2}^{2}}{2 m_{2}} + V ( x_{1} ,x_{2} )
```

Often, the potential will only depend on the **difference in the positions of the two particles**.

```math
V ( x_{1} ,x_{2} ) = V ( x_{1} - x_{2} )
```

This means that the overall **Hamiltonian has a translational symmetry**. Lets examine an infinitesimal translation in $`x`$. The original Schrödinger equation

```math
H\psi ( x_{1} ,x_{2} ) = E\psi ( x_{1} ,x_{2} )
```

transforms to

```math
H\psi ( x_{1} + dx,x_{2} + dx ) = E\psi ( x_{1} + dx,x_{2} + dx )
```

which can be Taylor expanded

```math
H \left( \psi ( x_{1} ,x_{2} ) + \frac{\partial \psi}{\partial x_{1}} dx + \frac{\partial \psi}{\partial x_{2}} dx \right) = E \left( \psi ( x_{1} ,x_{2} ) + \frac{\partial \psi}{\partial x_{1}} dx + \frac{\partial \psi}{\partial x_{2}} dx \right) .
```

We can write the derivatives in terms of the total momentum operator.

```math
p = p_{1} + p_{2} = \frac{\hbar}{i} \left( \frac{\partial}{\partial x_{1}} + \frac{\partial}{\partial x_{2}} \right)
```

```math
H\psi ( x_{1} ,x_{2} ) + \frac{i}{\hbar} Hp\psi ( x_{1} ,x_{2} ) dx = E\psi ( x_{1} ,x_{2} ) + \frac{i}{\hbar} Ep\psi ( x_{1} ,x_{2} ) dx
```

Subtract of the initial Schrödinger equation and commute $`E`$ through $`p`$.

```math
Hp\psi ( x_{1} ,x_{2} ) = Ep\psi ( x_{1} ,x_{2} ) = pH\psi ( x_{1} ,x_{2} )
```

We have proven that

```math
[ H,p ] = 0
```

if the Hamiltonian has translational symmetry. The **momentum is a constant of the motion**. Momentum is conserved. We can have **simultaneous eigenfunctions of the total momentum** **and of energy**.

<div align="right"><a href="book.pdf#page=191" target="_blank" title="This page in the PDF"><sup>[p. 191]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p192" href="book.pdf#page=192" target="_blank" title="This page in the PDF"><sup>[p. 192]</sup></a>

# <a id="s-12-2-quantum-mechanics-in-three-dimensions"></a>12.2 Quantum Mechanics in Three Dimensions

We have generalized Quantum Mechanics to include more than one particle. We now wish to include more than one dimension too.

Additional dimensions are essentially independent although they may be coupled through the potential. The coordinates and momenta from different dimensions commute. The fact that the commutators are zero can be calculated from the operators that we know. For example,

```math
[ x,p_{y} ] = [ x, \frac{\hbar}{i} \frac{\partial}{\partial y} ] = 0 .
```

The kinetic energy can simply be added and the potential now depends on 3 coordinates. The **Hamiltonian in 3D** is

```math
H = \frac{p_{x}^{2}}{2 m} + \frac{p_{y}^{2}}{2 m} + \frac{p_{z}^{2}}{2 m} + V ( \vec{r} ) = \frac{p^{2}}{2 m} + V ( \vec{r} ) = - \frac{\hbar^{2}}{2 m} \nabla^{2} + V ( \vec{r} ) .
```

This extension is really very simple.

# <a id="s-12-3-two-particles-in-three-dimensions"></a>12.3 Two Particles in Three Dimensions

The generalization of the Hamiltonian to three dimensions is simple.

```math
H = \frac{\vec{p}_{1}^{2}}{2 m} + \frac{\vec{p}_{2}^{2}}{2 m} + V ( \vec{r}_{1} - \vec{r}_{2} )
```

We define the vector **difference between the coordinates** of the particles.

```math
\vec{r} \equiv \vec{r}_{1} - \vec{r}_{2}
```

We also define the vector **position of the center of mass**.

```math
\vec{R} \equiv \frac{m_{1} \vec{r}_{1} + m_{2} \vec{r}_{2}}{m_{1} + m_{2}}
```

We will use the chain rule to transform our Hamiltonian. As a simple example, if we were working in one dimension we might use the chain rule like this.

```math
\frac{d}{dr_{1}} = \frac{\partial r}{\partial r_{1}} \frac{\partial}{\partial r} + \frac{\partial R}{\partial r_{1}} \frac{\partial}{\partial R}
```

In three dimensions we would have.

```math
\vec{\nabla}_{1} = \vec{\nabla}_{r} + \frac{m_{1}}{m_{1} + m_{2}} \vec{\nabla}_{R}
```

<div align="right"><a href="book.pdf#page=192" target="_blank" title="This page in the PDF"><sup>[p. 192]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p193" href="book.pdf#page=193" target="_blank" title="This page in the PDF"><sup>[p. 193]</sup></a>

```math
\vec{\nabla}_{2} = - \vec{\nabla}_{r} + \frac{m_{2}}{m_{1} + m_{2}} \vec{\nabla}_{R}
```

Putting this into the **Hamiltonian** we get

```math
\begin{aligned} H &= \frac{- \hbar^{2}}{2 m_{1}} \left[ \vec{\nabla}_{r}^{2} + \left( \frac{m_{1}}{m_{1} + m_{2}} \right)^{2} \vec{\nabla}_{R}^{2} + \frac{2 m_{1}}{m_{1} + m_{2}} \vec{\nabla}_{r} \cdot \vec{\nabla}_{R} \right] \\ & \quad + \frac{- \hbar^{2}}{2 m_{2}} \left[ \vec{\nabla}_{r}^{2} + \left( \frac{m_{2}}{m_{1} + m_{2}} \right)^{2} \vec{\nabla}_{R}^{2} - \frac{2 m_{2}}{m_{1} + m_{2}} \vec{\nabla} r \cdot \vec{\nabla} R \right] + V ( \vec{r} ) \end{aligned}
```

```math
H = - \hbar^{2} \left[ \left( \frac{1}{2 m_{1}} + \frac{1}{2 m_{2}} \right) \vec{\nabla}_{r}^{2} + \frac{m_{1} + m_{2}}{2 ( m_{1} + m_{2} )^{2}} \vec{\nabla}_{R}^{2} \right] + V ( \vec{r} ) .
```

Defining the **reduced mass** $`\mu`$

```math
\frac{1}{\mu} = \frac{1}{m_{1}} + \frac{1}{m_{2}}
```

and the total mass

```math
M = m_{1} + m_{2}
```

we get.

```math
H = - \frac{\hbar^{2}}{2 \mu} \vec{\nabla}_{r}^{2} - \frac{\hbar^{2}}{2 M} \vec{\nabla}_{R}^{2} + V ( \vec{r} )
```

The Hamiltonian actually **separates into two problems**: the motion of the **center of mass** as a free particle

```math
H = \frac{- \hbar^{2}}{2 M} \vec{\nabla}_{R}^{2}
```

and the **interaction between the two particles**.

```math
H = - \frac{\hbar^{2}}{2 \mu} \vec{\nabla}_{r}^{2} + V ( \vec{r} )
```

This is exactly the same separation that we would make in classical physics.

<div align="right"><a href="book.pdf#page=193" target="_blank" title="This page in the PDF"><sup>[p. 193]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p194" href="book.pdf#page=194" target="_blank" title="This page in the PDF"><sup>[p. 194]</sup></a>

# <a id="s-12-4-identical-particles"></a>12.4 Identical Particles

It is not possible to tell the difference between two electrons. They are identical in every way. Hence, there is a clear **symmetry in nature under the interchange of any two electrons**.

We define the **interchange operator** $`P_{1 2}`$. By our symmetry, this operator commutes with $`H`$ so we can have simultaneous eigenfunctions of energy and interchange.

If we interchange twice, we get back to the original state,

```math
P_{1 2} \psi ( x_{1} ,x_{2} ) = \psi ( x_{2} ,x_{1} )
```

```math
P_{1 2} P_{1 2} \psi ( x_{1} ,x_{2} ) = \psi ( x_{1} ,x_{2} )
```

so the **possible eigenvalues of the interchange operator are just +1 and -1**.

```math
P_{1 2} \psi_{\pm} = \pm \psi
```

It turns out that both possibilities exist in nature. Some particles like the electron, always have the -1 quantum number. The are spin one-half particles and are called **fermions**. The overall wavefunction changes sign whenever we interchange any pair of fermions. Some particles, like the photon, always have the +1 quantum number. They are integer spin particles, called **bosons**.

There is an important distinction between fermions and bosons which we can derive from the interchange symmetry. If any two fermions are in the same state, the wave function must be zero in order to be odd under interchange.

```math
\psi = u_{i} ( x_{1} ) u_{j} ( x_{2} ) \to u_{i} ( x_{1} ) u_{j} ( x_{2} ) - u_{j} ( x_{1} ) u_{i} ( x_{2} )
```

(Usually we write a state like $`u_{i} ( x_{1} ) u_{j} ( x_{2} )`$ when what we mean is the antisymmetrized version of that state $`u_{i} ( x_{1} ) u_{j} ( x_{2} ) - u_{j} ( x_{1} ) u_{i} ( x_{2} )`$.) Thus, no two fermions can be in the same state. This is often called the **Pauli exclusion principle**.

In fact, the interchange symmetry difference makes fermions behave like matter and bosons behave like energy. The fact that **no two fermions can be in the same state** means they take up space, unlike bosons. It is also related to the fact that fermions can only be created in conjunction with anti-fermions. They must be **made in pairs**. **Bosons can be made singly** and are their own anti-particle as can be seen from any light.

# <a id="s-12-5-sample-test-problems"></a>12.5 Sample Test Problems

1. **\*** Calculate the Fermi energy for $`N`$ particles of mass $`m`$ in a 3D cubic “box” of side $`L`$. Ignore spin for this problem.

**Answer**<br>
The energy levels are given in terms of three quantum numbers.

```math
E = \frac{\pi^{2} \hbar^{2}}{2 mL^{2}} ( n_{x}^{2} + n_{y}^{2} + n_{z}^{2} )
```

The number of states with inside some $`( n_{x}^{2} + n_{y}^{2} + n_{z}^{2} )_{max} ( \tfrac{1}{8}`$ of a a sphere in $`n`$ space) is

```math
N = \frac{1}{8} \frac{4}{3} \pi ( n_{x}^{2} + n_{y}^{2} + n_{z}^{2} )_{max}^{\tfrac{3}{2}}
```

<div align="right"><a href="book.pdf#page=194" target="_blank" title="This page in the PDF"><sup>[p. 194]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p195" href="book.pdf#page=195" target="_blank" title="This page in the PDF"><sup>[p. 195]</sup></a>

So for $`N`$ particles filling the levels,

```math
( n_{x}^{2} + n_{y}^{2} + n_{z}^{2} )_{max}^{\tfrac{3}{2}} = \frac{6 N}{\pi} .
```

```math
( n_{x}^{2} + n_{y}^{2} + n_{z}^{2} )_{max} = \left( \frac{6 N}{\pi} \right)^{\tfrac{2}{3}}
```

The energy corresponding to this is the Fermi energy.

```math
E_{F} = \frac{\pi^{2} \hbar^{2}}{2 mL^{2}} \left( \frac{6 N}{\pi} \right)^{\tfrac{2}{3}}
```

2. **\*** We put $`N`$ fermions of mass $`m`$ into a (cold) **one dimensional box** of length $`L`$. The particles go into the lowest energy states possible. If the Fermi energy is defined as the energy of the highest energy particle, what is the Fermi energy of this system? You may assume that there are 2 spin states for these fermions.

---

[← 11 More Fun with Operators](11-more-fun-with-operators.md) · [13 3D Problems Separable in Cartesian Coordinates →](13-3d-problems-separable-in-cartesian-coordinates.md)
