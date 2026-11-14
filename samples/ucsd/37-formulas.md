[← 36 Dirac Equation](36-dirac-equation.md)

---

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p546" href="book.pdf#page=546" target="_blank" title="This page in the PDF"><sup>[p. 546]</sup></a>

# 37 Formulas

$`\hbar`$ = 1$`. 0 5 \times 1 0^{- 2 7}`$ erg sec $`c`$ = 3$`. 0 0 \times 1 0`$<sup>10</sup> cm/sec $`e`$ = 1$`. 6 0 2 \times 1 0^{- 1 9}`$ coulomb

$`1 eV`$ = 1$`. 6 0 2 \times 1 0^{- 1 2}`$ erg $`\alpha = \tfrac{e^{2}}{\hbar c}`$ = 1$`/ 137`$ = $`\frac{e^{2}}{4 \pi \epsilon_{0} \hbar c}`$ (SI) $`\hbar c`$ = 1973 eV ˚A= 197.3 MeV F

1 ˚A= 1$`. 0 \times 1 0^{- 8}`$ cm 1 Fermi = 1$`. 0 \times 1 0^{- 1 3}`$ cm $`a_{0} = \tfrac{\hbar}{\alpha m_{e} c}`$ = 0$`. 5 2 9 \times 1 0^{- 8}`$ cm

$`m_{p}`$ = 938.3 MeV/$`c^{2} \quad m_{n}`$ = 939.6 MeV/$`c^{2} \quad m_{e}`$ = 9$`. 1 1 \times 1 0^{- 2 8}`$ g = 0.511 MeV/$`c^{2}`$

```math
k_{B} = 1 . 3 8 \times 1 0^{- 1 6} \text{ erg } /^{◦} K \quad g_{e} = 2 + \tfrac{\alpha}{\pi} \quad g_{p} = 5 . 6
```

```math
\mu_{Bohr} = \tfrac{e \hbar}{2 m_{e} c} = 0 . 5 7 9 \times 1 0^{- 8} \text{ eV } / \text{ gauss } \int_{-\infty}^{\infty} dxf ( x ) \delta ( g ( x ) ) = \left[ \tfrac{1}{|_{dx}^{dg} |} f ( x ) \right]_{g ( x ) = 0}
```

$`\int_{-\infty}^{\infty} dxf ( x ) \delta ( x - a ) = f ( a ) \int_{-\infty}^{\infty} dxe^{- ax^{2}} = \sqrt{\frac{\pi}{a}}`$ use $`\frac{\partial}{\partial a}`$ for other forms

```math
e^{A} = {}_{n} \sum_{= 0}^{\infty} \tfrac{A^{n}}{n !} \quad \sin \theta = {}_{n = 1} \sum^{\infty}_{, 3 , 5 \dots} \tfrac{\theta^{n}}{n !} ( - 1 )^{\tfrac{n - 1}{2}} \quad \cos \theta = {}_{n = 0} \sum^{\infty}_{, 2 , 4 \dots} \tfrac{\theta^{n}}{n !} ( - 1 )^{\tfrac{n}{2}}
```

```math
P ( x ) = \tfrac{1}{\sqrt{2 \pi \sigma^{2}}} e^{- x^{2} / 2 \sigma^{2}} \int_{0}^{\infty} drr^{n} e^{- ar} = \tfrac{n !}{a^{n + 1}} \quad E = \sqrt{m^{2} c^{4} + p^{2} c^{2}}
```

<div align="center">

# <a id="s-general-wave-mechanics"></a>GENERAL WAVE MECHANICS

</div>

```math
E = h\nu = \hbar \omega \quad \lambda = h/p \quad p = \hbar k
```

```math
\Delta p \Delta x \geq \tfrac{\hbar}{2} \quad \Delta A \Delta B \geq \langle \tfrac{i}{2} [ A,B ] \rangle \quad \Delta A = \sqrt{\langle A^{2} \rangle - \langle A \rangle^{2}}
```

```math
\psi ( x ) = \tfrac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} dp\varphi ( p ) e^{ipx/ \hbar} \quad \varphi ( p ) = \tfrac{1}{\sqrt{2 \pi \hbar}} \int_{-\infty}^{\infty} dx\psi ( x ) e^{- ipx/ \hbar}
```

```math
p_{op} = \tfrac{\hbar}{i} \tfrac{\partial}{\partial x} \quad E_{op} = i \hbar_{\partial t}^{\partial} \quad x_{op} = i \hbar \tfrac{\partial}{\partial p}
```

```math
Hu_{j} ( x ) = E_{j} u_{j} ( x ) \quad \psi_{j} ( x,t ) = u_{j} ( x ) e^{- iE_{j} t/ \hbar} \quad \tfrac{- \hbar^{2}}{2 m} \tfrac{\partial^{2} \psi}{\partial x^{2}} + V ( x ) \psi = i \hbar_{\partial t}^{\partial \psi}
```

$`\psi ( x )`$ continuous $`\frac{d\psi}{dx}`$ continous if $`V`$ finite

```math
\Delta \tfrac{d\psi}{dx} = \tfrac{2 m\lambda}{\hbar^{2}} \psi ( a ) \text{ for } V ( x ) = \lambda \delta ( x - a )
```

```math
\langle \varphi | \psi \rangle = \int_{-\infty}^{\infty} dx\varphi^{*} ( x ) \psi ( x ) \quad \langle u_{i} | u_{j} \rangle = \delta_{ij} \sum_{i} | u_{i} \rangle \langle u_{i} | = 1
```

```math
\varphi = \sum_{i} a_{i} u_{i} \quad a_{i} = \langle u_{i} | \varphi \rangle \quad \psi ( x ) = \langle x | \psi \rangle
```

```math
\langle \varphi | A | \psi \rangle = \langle \varphi | A\psi \rangle = \langle A^{\dagger} \varphi | \psi \rangle = \langle \psi | A_{\dagger} | \varphi \rangle^{*} \quad \varphi ( p ) = \langle p | \psi \rangle
```

```math
[ \tfrac{1}{2 m} ( \vec{p} + \tfrac{e}{c} \vec{A} )^{2} + V ( \vec{r} ) ] \psi ( \vec{r} ) = E\psi ( \vec{r} ) \quad H\psi = E\psi
```

```math
[ p_{x} ,x ] = \tfrac{\hbar}{i} \quad [ L_{x} ,L_{y} ] = i \hbar L_{z} \quad [ L^{2} ,L_{z} ] = 0
```

```math
\psi_{i} = \langle u_{i} | \psi \rangle \quad A_{ij} = \langle u_{i} | A | u_{j} \rangle \quad \tfrac{d \langle A \rangle}{dt} = \langle \tfrac{\partial A}{\partial t} \rangle + \tfrac{i}{\hbar} \langle [ H,A ] \rangle
```

<div align="right"><a href="book.pdf#page=546" target="_blank" title="This page in the PDF"><sup>[p. 546]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p547" href="book.pdf#page=547" target="_blank" title="This page in the PDF"><sup>[p. 547]</sup></a>

# <a id="s-harmonic-oscillator"></a>HARMONIC OSCILLATOR

```math
H = \tfrac{p^{2}}{2 m} + \tfrac{1}{2} m\omega^{2} x^{2} = \hbar \omega A^{\dagger} A + \tfrac{1}{2} \hbar \omega \quad 1 0 . 1 \quad E_{n} = ( n + \tfrac{1}{2} ) \hbar \omega \quad n = 0 , 1 , 2 \dots \quad 1 0 . 3
```

```math
u_{n} ( x ) = \sum_{k = 0}^{\infty} a_{k} y^{k} e^{- y^{2} / 2} \quad a_{k + 2} = \tfrac{2 ( k - n )}{( k + 1 ) ( k + 2 )} a_{k} \quad y = \sqrt{\frac{m\omega}{\hbar}} x
```

```math
A = ( \sqrt{\frac{m\omega}{2 \hbar}} x + i \tfrac{p}{2 m \hbar \omega} \sqrt{) \quad A^{\dagger}} = ( \sqrt{\frac{m\omega}{2 \hbar}} x - i \tfrac{p}{2 m \hbar \omega} \sqrt{) \quad [ A,A^{\dagger}} ] = 1
```

```math
A^{\dagger} | n \rangle = \sqrt{( n + 1 )} | n + 1 \rangle \quad A | n \rangle = \sqrt{( n )} | n - 1 \rangle \quad u_{0} ( x ) = ( {}_{\hbar \pi}^{m\omega} ) 4^{1} e^{- m\omega x^{2} / 2 \hbar}
```

# <a id="s-angular-momentum"></a>ANGULAR MOMENTUM

```math
[ L_{i} ,L_{j} ] = i \hbar \epsilon_{ijk} L_{k} \quad [ L^{2} ,L_{i} ] = 0 \int Y_{\ell m}^{*} Y_{\ell^{\prime} m^{\prime}} d \Omega = \delta \ell \ell^{\prime} \delta_{mm^{\prime}}
```

```math
L^{2} Y_{\ell m} = \ell ( \ell + 1 ) \hbar^{2} Y_{\ell m} \quad L_{z} Y_{\ell m} = m \hbar Y_{\ell m} \quad - \ell \leq m \leq \ell
```

```math
L_{\pm} = L_{x} \pm iL_{y} \quad L_{\pm} Y_{\ell m} = \hbar \sqrt{\ell ( \ell + 1 ) - m ( m \pm 1 )} Y_{\ell ,m \pm 1}
```

```math
Y_{0 0} = \tfrac{1}{\sqrt{4 \pi}} \quad Y_{1 1} = - \sqrt{\tfrac{3}{8 \pi} e^{i\varphi} \sin \theta \quad Y_{1 0}} = \sqrt{\tfrac{3}{4 \pi} \cos \theta}
```

```math
Y_{2 2} = \sqrt{\tfrac{1 5}{3 2 \pi} e^{2 i\varphi} \sin^{2} \theta \quad Y_{2 1}} = - \sqrt{\tfrac{1 5}{8 \pi} e^{i\varphi} \sin \theta \cos \theta \quad Y_{2 0}} = \sqrt{\tfrac{5}{1 6 \pi} ( 3 \cos^{2} \theta - 1 )}
```

```math
Y_{\ell \ell} = e^{i\ell \varphi} \sin^{\ell} \theta \quad Y_{\ell ( - m )} = ( - 1 )^{m} Y_{\ell m}^{*} \quad Y_{\ell m} ( \pi - \theta ,\varphi + \pi ) = ( - 1 )^{\ell} Y_{\ell m} ( \theta ,\varphi )
```

```math
\tfrac{- \hbar^{2}}{2 \mu} \left[ \tfrac{\partial^{2}}{\partial r^{2}} + \tfrac{2}{r} \tfrac{\partial}{\partial r} \right] R_{n\ell} ( r ) + \left( V ( r ) + \tfrac{\ell ( \ell + 1 ) \hbar^{2}}{2 \mu r^{2}} \right) R_{n\ell} ( r ) = ER_{n\ell} ( r )
```

```math
j_{0} ( kr ) = \tfrac{\sin ( kr )}{kr} \quad n_{0} ( kr ) = - \tfrac{\cos ( kr )}{kr} \quad h_{\ell}^{( 1 )} ( kr ) = j_{\ell} ( kr ) + in_{\ell} ( kr )
```

```math
H = H_{0} - \vec{\mu} \cdot \vec{B} \quad \vec{\mu} = \tfrac{e}{2 mc} \vec{L} \quad \vec{\mu} = \tfrac{ge}{2 mc} \vec{S}
```

```math
S_{i} = \tfrac{\hbar}{2} \sigma_{i} \quad [ \sigma_{i} ,\sigma_{j} ] = 2 i\epsilon_{ijk} \sigma_{k} \quad \{ \sigma_{i} ,\sigma_{j} \} = 0
```

```math
\sigma_{x} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \sigma_{y} = \begin{pmatrix} 0 & - i \\ i & 0 \end{pmatrix} \quad \sigma_{z} = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix}
```

```math
S_{x} = \hbar \begin{pmatrix} 0 & \tfrac{1}{\sqrt{2}} & 0 \\ \tfrac{1}{\sqrt{2}} & 0 & \tfrac{1}{\sqrt{2}} \\ 0 & \tfrac{1}{\sqrt{2}} & 0 \end{pmatrix} \quad S_{y} = \hbar \begin{pmatrix} 0 & \tfrac{- i}{2}^{\surd} & 0 \\ \tfrac{i}{\sqrt{2}} & 0 & \tfrac{i}{\sqrt{2}}^{-} \\ 0 & \tfrac{i}{2}^{\surd} & 0 \end{pmatrix} \quad S_{z} = \hbar \begin{pmatrix} 1 & 0 & 0 \\ 0 & 0 & 0 \\ 0 & 0 & - 1 \end{pmatrix} 1
```

# <a id="s-hydrogen-atom"></a>HYDROGEN ATOM

```math
H = \tfrac{p^{2}}{2 \mu} - \tfrac{Ze^{2}}{r} \quad \psi_{n\ell m} = R_{n\ell} ( r ) Y_{\ell m} ( \theta ,\varphi ) \quad E_{n} = - \tfrac{Z^{2} \alpha^{2} \mu c^{2}}{2 n^{2}} = - {}^{1 3} \tfrac{. 6}{n^{2}} \text{ eV }
```

```math
n = n_{r} + \ell + 1 \quad a_{0} = \tfrac{\hbar}{\alpha \mu c} \quad \ell = 0 , 1 ,\dots ,n - 1
```

```math
R_{n\ell} ( \rho ) = \rho^{\ell} \sum_{k = 0}^{\infty} a_{k} \rho^{k} e^{- \rho / 2} \quad a_{k + 1} = \tfrac{k + \ell + 1 n}{( k + 1 ) ( k + 2 \ell + 2 )}^{-} a_{k} \quad \rho = \surd^{-} \tfrac{8 \mu E}{\hbar^{2}} r = \tfrac{2 r}{na_{0}}
```

```math
R_{1 0} = 2 ( \tfrac{Z}{a_{0}} ) 2^{3} e^{\tfrac{- Zr}{a 0}} \quad R_{2 0} = 2 ( \tfrac{Z}{2 a_{0}} ) \tfrac{3}{2} ( 1 - \tfrac{Zr}{2 a_{0}} ) e^{\tfrac{- Zr}{2 a 0}} \quad R_{2 1} = \tfrac{1}{3} \sqrt{( \tfrac{Z}{2 a_{0}} ) 2^{3} ( \tfrac{Zr}{a_{0}} ) e^{\tfrac{- Zr}{2 a 0}}}
```

```math
R_{n,n - 1} \propto r^{n - 1} e^{- Zr/na_{0}} \quad \mu = \tfrac{m_{1} m_{2}}{m_{1} + m_{2}} \quad \langle \psi_{n\ell m} | \tfrac{e^{2}}{r} | \psi_{n\ell m} \rangle = \tfrac{Ze^{2}}{n^{2} a_{0}} = \tfrac{Z\alpha^{2} \mu c^{2}}{n^{2}}
```

```math
H_{1} = - {}_{8} \tfrac{p^{4}}{m^{3} c^{2}} \quad H_{2} = \tfrac{e^{2}}{2 m^{2} c^{2} r^{3}} \vec{S} \cdot \vec{L} \quad \Delta E_{1 2} = - {}_{2} \tfrac{1}{n^{3}} \alpha^{4} mc^{2} ( \tfrac{1}{j + \tfrac{1}{2}} - \tfrac{3}{4 n} )
```

```math
H_{3} = \tfrac{e^{2} g_{p}}{3 mM_{p} c^{2}} \vec{S} \cdot \vec{I} 4 \pi \delta^{3} ( \vec{r} ) \quad \Delta E_{3} = \tfrac{2 g_{p} m\alpha^{4} mc^{2}}{3 M_{p} n^{3}} ( f ( f + 1 ) - I ( I + 1 ) - \tfrac{3}{4} )
```

```math
H_{B} = \tfrac{eB}{2 mc} ( L_{z} + 2 S_{z} ) \quad \Delta E_{B} = \tfrac{e \hbar B}{2 mc} ( 1 \pm \tfrac{1}{2 \ell + 1} ) m_{j} \text{ for } j = \ell \pm \tfrac{1}{2}
```

<div align="right"><a href="book.pdf#page=547" target="_blank" title="This page in the PDF"><sup>[p. 547]</sup></a></div>
<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p548" href="book.pdf#page=548" target="_blank" title="This page in the PDF"><sup>[p. 548]</sup></a>

<div align="center">

# <a id="s-addition-of-angular-momentum"></a>ADDITION OF ANGULAR MOMENTUM

</div>

```math
\vec{J} = \vec{L} + \vec{S} \quad | \ell - s |\leq j \leq \ell + s \quad L \vec{\cdot} S = \tfrac{1}{2} ( J^{2} - L^{2} - S^{2} )
```

```math
\psi_{jm_{j} \ell s} = {}_{m} \sum_{\ell \quad s m} C ( jm_{j} ; \ell m_{\ell} sm_{s} ) Y_{\ell m_{\ell}} \chi_{sm_{s}} = {}_{m} \sum_{\ell \quad s m} \langle jm_{j} \ell s | \ell m_{\ell} sm_{s} \rangle Y_{\ell m_{\ell}} \chi_{sm_{s}}
```

```math
\psi_{j,m_{j}} = \psi_{\ell + \tfrac{1}{2} ,m + \tfrac{1}{2}} = \sqrt{\ell + 2 \ell + 1 m + 1} Y_{\ell m} \chi_{+} + \sqrt{\frac{\ell - m}{2 \ell + 1}} Y_{\ell ,m + 1} \chi_{-} \quad \text{ for } s = \tfrac{1}{2} \text{ and any } \ell
```

$`\psi_{j,m_{j}} = \psi_{\ell - \tfrac{1}{2} ,m + \tfrac{1}{2}} = \sqrt{\frac{\ell - m}{2 \ell + 1}} Y_{\ell m} \chi_{+} - \sqrt{\ell + 2 \ell + 1 m + 1} Y_{\ell ,m + 1} \chi_{-}`$ for $`s = \tfrac{1}{2}`$ and any $`\ell`$

<div align="center">

# <a id="s-perturbation-theory-and-radiative-decays"></a>PERTURBATION THEORY AND RADIATIVE DECAYS

</div>

```math
E_{n}^{( 1 )} = \langle \varphi_{n} | H_{1} | \varphi_{n} \rangle \quad E_{n}^{( 2 )} = {}_{k} \sum_{\not = n} \tfrac{| \langle \varphi_{k} | H_{1} | \varphi_{n} \rangle |^{2}}{E_{n}^{( 0 )} - E_{k}^{( 0 )}} \quad c_{nk}^{( 1 )} = \tfrac{\langle \varphi_{k} | H_{1} | \varphi_{n} \rangle}{E_{n}^{( 0 )} - E_{k}^{( 0 )}}
```

```math
c_{n} ( t ) = \tfrac{1}{i \hbar} \int^{t}_{0} dt^{\prime} e^{i ( E_{n} - E_{i} ) t^{\prime} / \hbar} \langle \varphi_{n} | V ( t^{\prime} ) | \varphi_{i} \rangle
```

```math
\text{ Fermi } ’ s \text{ Golden Rule } : \quad \Gamma_{i \to f} = \tfrac{2 \pi}{\hbar} | \langle \psi_{f} | V | \psi_{i} \rangle |^{2} \rho_{f} ( E )
```

$`\Gamma_{i \to f} =`$<sup>2</sup>$`\tfrac{\pi}{\hbar} \int \prod ( \tfrac{Vd}{( 2} \tfrac{{}^{3} p_{k}}{\pi \hbar )^{3}} ) | M_{fi} |^{2} \delta^{3}`$(momentum conservation) $`\delta`$(Energy conservation)

```math
\Gamma_{m \to k}^{rad} = \tfrac{\alpha k}{2 \pi m^{2} c^{2}} \int d \Omega_{p} \omega_{km} | \langle \varphi_{m} | e^{- i \vec{k} \cdot \vec{r}} \hat{\epsilon} \cdot \vec{p} | \varphi_{k} \rangle |^{2}
```

```math
\Gamma_{m \to k}^{E 1} = \tfrac{\alpha}{2 \pi c^{2}} \int d \Omega_{p} \omega_{km}^{3} | \langle \varphi_{m} | \hat{\epsilon} \cdot \vec{r} | \varphi_{k} \rangle |^{2} \quad \Delta l = \pm 1 , \Delta s = 0
```

```math
\hat{\epsilon} \cdot \hat{r} = \sqrt{\tfrac{4 \pi}{3} ( \epsilon_{z} Y_{1 0} + \tfrac{- \epsilon_{x} + i\epsilon_{y}}{2}^{\surd} Y_{1 1} + \tfrac{\epsilon_{x} + i\epsilon_{y}}{2}^{\surd} Y_{1 - 1} ) \quad \hat{\epsilon} \cdot \vec{k}} = 0
```

```math
I ( \omega ) \propto \tfrac{\Gamma / 2}{( \omega - \omega_{0} )^{2} + ( \Gamma / 2 )^{2}} \quad \Gamma_{collision} = P\sigma \sqrt{\frac{3}{mkT}} \quad ( \tfrac{\Delta \omega}{\omega} )_{Dopler} = \sqrt{\frac{kT}{mc^{2}}}
```

```math
( \tfrac{d\sigma}{d \Omega} )_{BORN} = \tfrac{1}{4 \pi^{2} \hbar^{4}} \tfrac{p_{f}}{p_{i}} m_{f} m_{i} | \tilde{V} ( \vec{\Delta} ) |^{2} \quad \tilde{V} ( \vec{\Delta} ) = \int d^{3} \vec{r} e^{- i \vec{\Delta} \cdot \vec{r}} V ( \vec{r} ) \quad \vec{\Delta} = \tfrac{\vec{p}_{f} - \vec{p}_{i}}{\hbar}
```

<div align="center">

# <a id="s-atoms-and-molecules"></a>ATOMS AND MOLECULES

</div>

Hund: 1) max $`s`$ 2)max $`\ell`$ (allowed) 3) min $`j ( \leq \tfrac{1}{2}`$ shell) else max $`j`$

```math
E_{rot} = \tfrac{\ell ( \ell + 1 ) \hbar^{2}}{2 I} \approx \tfrac{1}{2 0 0 0} \text{ eV } \quad E_{vib} = ( n + \tfrac{1}{2} ) \hbar \omega \approx \tfrac{1}{5 0} \text{ eV }
```

---

[← 36 Dirac Equation](36-dirac-equation.md)
