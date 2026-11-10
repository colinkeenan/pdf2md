# Advanced Quantum Mechanics

<a id="00-front-matter"></a>

# Front Matter

<div align="center">

**F. J. Dyson**

## <a id="s-advanced"></a>ADVANCED

## <a id="s-quantum-mechanics"></a>QUANTUM MECHANICS

**Second Edition**

<a id="pi"><sup>[p. i]</sup></a>

_ADVANCED QUANTUM MECHANICS_

Lecture notes by Professor F. J. Dyson for a course in Relativistic Quantum Mechanics<br>
given at Cornell University in the Fall of 1951.

Second Edition

The first edition of these notes was written by Professor Dyson. The second edition was prepared by<br>
Michael J. Moravcsik; he is responsible for the changes made in the process of re-editing.

_Generally used notation:_

</div>

$$
\begin{array}{lcl}
A^{*} & : & \text{complex conjugate transposed (Hermitian conjugate)} \\
A^{+} & : & \text{complex conjugate (not transposed)} \\
\overline{A} & : & A^{*} \beta = A^{*} \gamma_{4} = \text{ adjoint } \\
A^{- 1} & = & \text{inverse} \\
A^{T} & = & \text{transposed} \\
\mathbb{I} & = & \text{identity matrix or operator} \\
\end{array}
$$

<a id="pii"><sup>[p. ii]</sup></a>

## <a id="s-table-of-contents"></a>Table of Contents

- [Introduction](#01-introduction) — [pp. 1–3](#p1)
  - [Subject Matter](#s-subject-matter) — [p. 1](#p1)
  - [Detailed Program](#s-detailed-program) — [p. 2](#p2)
  - [One Particle Theories](#s-one-particle-theories) — [p. 2](#p2)
- [The Dirac Theory](#02-the-dirac-theory) — [pp. 4–21](#p4)
  - [The Form of the Dirac Equation](#s-the-form-of-the-dirac-equation) — [p. 4](#p4)
  - [Lorentz Invariance of the Dirac Equation](#s-lorentz-invariance-of-the-dirac-equation) — [p. 6](#p6)
  - [To Find the S](#s-to-find-the-s) — [p. 7](#p7)
  - [The Covariant Notation](#s-the-covariant-notation) — [p. 8](#p8)
  - [Conservation Laws – Existence of Spin](#s-conservation-laws-existence-of-spin) — [p. 9](#p9)
  - [Elementary Solutions](#s-elementary-solutions) — [p. 10](#p10)
  - [The Hole Theory](#s-the-hole-theory) — [p. 11](#p11)
  - [Positron States – charge conjugation](#s-positron-states-charge-conjugation) — [p. 11](#p11)
  - [Electromagnetic Properties of the Electron](#s-electromagnetic-properties-of-the-electron) — [p. 12](#p12)
  - [The Hydrogen Atom](#s-the-hydrogen-atom) — [p. 13](#p13)
  - [Solution of the radial equation](#s-solution-of-the-radial-equation) — [p. 14](#p14)
  - [Behavior of an Electron in non-relativistic approximation](#s-behavior-of-an-electron-in-non-relativistic-approximation) — [p. 17](#p17)
  - [Summary of Matrices in the Dirac Theory in our notation](#s-summary-of-matrices-in-the-dirac-theory-in-our-notation) — [p. 20](#p20)
  - [Summary of Matrices in the Dirac Theory in the Feynman notation](#s-summary-of-matrices-in-the-dirac-theory-in-the-feynman-notation) — [p. 21](#p21)
- [Scattering Problems and Born Approximation](#03-scattering-problems-and-born-approximation) — [pp. 22–31](#p22)
  - [General Discussion](#s-general-discussion) — [p. 22](#p22)
  - [Projection Operators](#s-projection-operators) — [p. 23](#p23)
  - [Calculation of Spurs](#s-calculation-of-spurs) — [p. 24](#p24)
  - [Scattering of two electrons in Born Approximation – The Møller Formula](#s-scattering-of-two-electrons-in-born-approximation-the-m-ller-formula) — [p. 27](#p27)
  - [Relation of Cross Sections to Transition Amplitudes](#s-relation-of-cross-sections-to-transition-amplitudes) — [p. 29](#p29)
  - [Results for Møller Scattering](#s-results-for-m-ller-scattering) — [p. 30](#p30)
  - [Note on the Treatment of Exchange Effects](#s-note-on-the-treatment-of-exchange-effects) — [p. 31](#p31)
  - [Relativistic Treatment of Several Particles](#s-relativistic-treatment-of-several-particles) — [p. 31](#p31)
- [Field Theory](#04-field-theory) — [pp. 32–40](#p32)
  - [Classical Relativistic Field Theory](#s-classical-relativistic-field-theory) — [p. 32](#p32)
  - [Quantum Relativistic Field Theory](#s-quantum-relativistic-field-theory) — [p. 34](#p34)
  - [The Feynman Method of Quantization](#s-the-feynman-method-of-quantization) — [p. 35](#p35)
  - [The Schwinger Action Principle](#s-the-schwinger-action-principle) — [p. 36](#p36)
    - [A. The Field Equations](#s-a-the-field-equations) — [p. 37](#p37)
    - [B. The Schrödinger Equation for the State-function](#s-b-the-schr-dinger-equation-for-the-state-function) — [p. 37](#p37)
    - [C. Operator Form of the Schwinger Principle](#s-c-operator-form-of-the-schwinger-principle) — [p. 38](#p38)
    - [D. Canonical Commutation Laws](#s-d-canonical-commutation-laws) — [p. 39](#p39)
    - [E. The Heisenberg Equation of Motion for the Operators](#s-e-the-heisenberg-equation-of-motion-for-the-operators) — [p. 39](#p39)
    - [F. General Covariant Commutation Laws](#s-f-general-covariant-commutation-laws) — [p. 39](#p39)
    - [G. Anticommuting Fields](#s-g-anticommuting-fields) — [p. 40](#p40)
- [Examples of Quantized Field Theories](#05-examples-of-quantized-field-theories) — [pp. 41–80](#p41)
  - [I. The Maxwell Field](#s-i-the-maxwell-field) — [p. 41](#p41)
    - [Momentum Representations](#s-momentum-representations) — [p. 42](#p42)
    - [Fourier Analysis of Operators](#s-fourier-analysis-of-operators) — [p. 43](#p43)
    - [Emission and Absorption Operators](#s-emission-and-absorption-operators) — [p. 44](#p44)
    - [Gauge Invariance of the Theory](#s-gauge-invariance-of-the-theory) — [p. 45](#p45)
    - [The Vacuum State](#s-the-vacuum-state) — [p. 45](#p45)
    - [The Gupta-Bleuler Method](#s-the-gupta-bleuler-method) — [p. 47](#p47)
    - [Example: Spontaneous Emission of Radiation](#s-example-spontaneous-emission-of-radiation) — [p. 47](#p47)
    - [The Hamiltonian Operator](#s-the-hamiltonian-operator) — [p. 49](#p49)
    - [Fluctuations of the Fields](#s-fluctuations-of-the-fields) — [p. 50](#p50)
    - [The Lamb Shift – Fluctuation of Position of Electron](#s-the-lamb-shift-fluctuation-of-position-of-electron) — [p. 51](#p51)
  - [Ia. Theory of Line Shift and Line Width](#s-ia-theory-of-line-shift-and-line-width) — [p. 52](#p52)
    - [The Interaction Representation](#s-the-interaction-representation) — [p. 53](#p53)
    - [Application of Interaction Representation to Line Shift and Line Width](#s-application-of-interaction-representation-to-line-shift-and-line-width) — [p. 54](#p54)
    - [Calculation of Line Shift – Non-Relativistic Theory](#s-calculation-of-line-shift-non-relativistic-theory) — [p. 57](#p57)
    - [The Idea of Mass Renormalization](#s-the-idea-of-mass-renormalization) — [p. 57](#p57)
  - [II. Field Theory of the Dirac Electron – Without Interaction](#s-ii-field-theory-of-the-dirac-electron-without-interaction) — [p. 59](#p59)
    - [Covariant Commutation Rules](#s-covariant-commutation-rules) — [p. 60](#p60)
    - [Momentum Representations](#s-momentum-representations-2) — [p. 61](#p61)
    - [Fourier Analysis of Operators](#s-fourier-analysis-of-operators-2) — [p. 62](#p62)
    - [Emission and Absorption Operators](#s-emission-and-absorption-operators-2) — [p. 62](#p62)
    - [Charge Symmetrical Representation](#s-charge-symmetrical-representation) — [p. 63](#p63)
    - [The Hamiltonian](#s-the-hamiltonian) — [p. 64](#p64)
    - [Failure of Theory with Commuting Fields](#s-failure-of-theory-with-commuting-fields) — [p. 64](#p64)
    - [The Exclusion Principle](#s-the-exclusion-principle) — [p. 65](#p65)
    - [The Vacuum State](#s-the-vacuum-state-2) — [p. 65](#p65)
  - [III. Field Theory of Dirac Electron in External Field](#s-iii-field-theory-of-dirac-electron-in-external-field) — [p. 66](#p66)
    - [Covariant Commutation Rules](#s-covariant-commutation-rules-2) — [p. 66](#p66)
    - [The Hamiltonian](#s-the-hamiltonian-2) — [p. 68](#p68)
    - [Antisymmetry of the States](#s-antisymmetry-of-the-states) — [p. 69](#p69)
    - [Polarization of the Vacuum](#s-polarization-of-the-vacuum) — [p. 70](#p70)
    - [Calculation of Momentum Integrals](#s-calculation-of-momentum-integrals) — [p. 73](#p73)
    - [Physical Meaning of the Vacuum Polarization](#s-physical-meaning-of-the-vacuum-polarization) — [p. 75](#p75)
    - [Slowly Varying Weak Fields. The Uehling Effect](#s-slowly-varying-weak-fields-the-uehling-effect) — [p. 78](#p78)
  - [IV. Field Theory Of Dirac And Maxwell Fields In Interaction](#s-iv-field-theory-of-dirac-and-maxwell-fields-in-interaction) — [p. 78](#p78)
    - [The Complete Relativistic Quantum Electrodynamics](#s-the-complete-relativistic-quantum-electrodynamics) — [p. 78](#p78)
    - [Free Interaction Representation](#s-free-interaction-representation) — [p. 80](#p80)
- [Free Particle Scattering Problems](#06-free-particle-scattering-problems) — [pp. 81–93](#p81)
  - [A. Møller Scattering of Two Electrons](#s-a-m-ller-scattering-of-two-electrons) — [p. 81](#p81)
    - [Properties of the $D_{F}$ Function](#s-properties-of-the-d-f-function) — [p. 82](#p82)
    - [The Møller Formula – Conclusion](#s-the-m-ller-formula-conclusion) — [p. 83](#p83)
    - [Electron-Positron Scattering](#s-electron-positron-scattering) — [p. 84](#p84)
  - [B. Scattering of a Photon by an Electron – The Compton Effect – The Klein-Nishina Formula](#s-b-scattering-of-a-photon-by-an-electron-the-compton-effect-the-klein-nishina-formula) — [p. 84](#p84)
    - [Calculation of Cross Section](#s-calculation-of-cross-section) — [p. 86](#p86)
    - [Sum Over Spins](#s-sum-over-spins) — [p. 87](#p87)
  - [C. Two-Quantum Pair Annihilation](#s-c-two-quantum-pair-annihilation) — [p. 90](#p90)
  - [D. Bremsstrahlung and Pair Creation in the Coulomb Field of an Atom](#s-d-bremsstrahlung-and-pair-creation-in-the-coulomb-field-of-an-atom) — [p. 92](#p92)
- [General Theory of Free Particle Scattering](#07-general-theory-of-free-particle-scattering) — [pp. 94–117](#p94)
  - [Reduction of an Operator to Normal Form](#s-reduction-of-an-operator-to-normal-form) — [p. 96](#p96)
  - [Feynman Graphs](#s-feynman-graphs) — [p. 98](#p98)
  - [Feynman Rules of Calculation](#s-feynman-rules-of-calculation) — [p. 100](#p100)
  - [The Self-Energy of the Electron](#s-the-self-energy-of-the-electron) — [p. 102](#p102)
  - [Second-Order Radiative Corrections to Scattering](#s-second-order-radiative-corrections-to-scattering) — [p. 104](#p104)
  - [Treatment of Low-Frequency Photons – The Infra-Red Catastrophe](#s-treatment-of-low-frequency-photons-the-infra-red-catastrophe) — [p. 116](#p116)
- [Scattering By A Static Potential](#08-scattering-by-a-static-potential-comparison-with-experimental-results) — [pp. 118–131](#p118)
  - [A. The Magnetic Moment of the Electron](#s-a-the-magnetic-moment-of-the-electron) — [p. 121](#p121)
  - [B. Relativistic Calculation of the Lamb Shift](#s-b-relativistic-calculation-of-the-lamb-shift) — [p. 123](#p123)
    - [Covariant Part of the Calculation](#s-covariant-part-of-the-calculation) — [p. 124](#p124)
    - [Discussion of the Nature of the Φ-Representation](#s-discussion-of-the-nature-of-the-representation) — [p. 126](#p126)
    - [Concluding Non-Covariant Part of the Calculation](#s-concluding-non-covariant-part-of-the-calculation) — [p. 127](#p127)
    - [Accuracy of the Lamb Shift Calculation](#s-accuracy-of-the-lamb-shift-calculation) — [p. 130](#p130)
- [Typist’s Afterword](#09-typist-s-afterword) — [pp. 132–133](#p132)
- [Index](#10-index) — [pp. 140–143](#p140)

<a id="piii"><sup>[p. iii]</sup></a>

<a id="piv"><sup>[p. iv]</sup></a>

<div align="center">

## <a id="s-lecture-course-491-advanced-quantum-theory-f-j-dyson-fall-semester-1951"></a>Lecture Course 491 — Advanced Quantum Theory F. J. Dyson — Fall Semester 1951

</div>

---

<a id="01-introduction"></a>

# Introduction

<a id="p1"><sup>[p. 1]</sup></a>

## <a id="s-books"></a>Books

W. <a id="ix-142">Pauli</a>, “Die Allgemeinen Principien der Wellenmechanik”; _Handbuch der Physik_, 2 ed., Vol. 24,

Part 1; Edwards reprint, Ann Arbor 1947. (In German) <a id="refback-1-1"></a>[[1]](#ref-1)

W. <a id="ix-112">Heitler</a>, _Quantum Theory of Radiation_, 2nd Edition, Oxford. 3rd edition just published. <a id="refback-2-1"></a>[[2]](#ref-2)

G. <a id="ix-221">Wentzel</a>, _Introduction to the Quantum Theory of Wave-Fields_, Interscience, N.Y. 1949 <a id="refback-3-1"></a>[[3]](#ref-3)

I shall not expect you to have read any of these, but I shall refer to them as we go along. The later part of the course will be new stuff, taken from papers of <a id="ix-76">Feynman</a> and <a id="ix-240">Schwinger</a> mainly. <a id="01-introduction--refback-4-1"></a>[[4]](#01-introduction--ref-4), <a id="refback-5-1"></a>[[5]](#ref-5), <a id="01-introduction--refback-6-1"></a>[[6]](#01-introduction--ref-6), <a id="01-introduction--refback-7-1"></a>[[7]](#01-introduction--ref-7), <a id="01-introduction--refback-8-1"></a>[[8]](#01-introduction--ref-8)

## <a id="s-subject-matter"></a>Subject Matter

You have had a complete course in non-relativistic quantum theory. I assume this known. All the general principles of the NR theory are valid and true under all circumstances, in particular also when the system happens to be relativistic. What you have learned is therefore still good.

You have had a course in classical mechanics and electrodynamics including special relativity. You know what is meant by a system being relativistic; the equations of motion are formally invariant under <a id="ix-156">Lorentz</a> <a id="ix-164">transformations</a>. General relativity we shall not touch.

This course will be concerned with the development of a _Lorentz–invariant Quantum theory_. That is not a general dynamical method like the NR quantum theory, applicable to all systems. We cannot yet devise a general method of that kind, and it is probably impossible. Instead we have to find out what are the possible systems, the particular equations of motion, which can be handled by the NR quantum dynamics and which are at the same time Lorentz–invariant.

In the NR theory it was found that almost any classical system could be handled, i.e.quantized. Now on the contrary we find there are very few possibilities for a relativistic quantized system. This is a most important fact. It means that starting only from the principles of relativity and quantization, it is mathematically possible only for very special types of objects to exist. So one can _predict_ mathematically some important things about the real world. The most striking examples of this are:

(i) <a id="ix-83">Dirac</a> from a study of the electron predicted the <a id="ix-151">positron</a>, which was later discovered <a id="refback-9-1"></a>[[9]](#ref-9).

(ii) <a id="ix-225">Yukawa</a> from a study of nuclear forces predicted the <a id="ix-196">meson</a>, which was later discovered <a id="refback-10-1"></a>[[10]](#ref-10).

These two examples are special cases of the general principle, which is the basic success of the relativistic quantum theory, that _A Relativistic Quantum Theory of a Finite Number of Particles is Impossible._ A RQ theory necessarily contains these features: an indefinite number of particles of one or more types, particles of each type being identical and indistinguishable from each other, possibility of <a id="ix-6">creation</a> and <a id="ix-32">annihilation</a> of particles. Thus the two principles of relativity and quantum theory when combined lead to a world built up out of various types of elementary particles, and so make us feel quite confident that we are on the right way to an understanding of the real world. In addition, various detailed properties of the observed particles <a id="p2"><sup>[p. 2]</sup></a> are necessary consequences of the general theory. These are for example:

(i) <a id="ix-101">Magnetic moment</a> of Electron (<a id="ix-84">Dirac</a>) <a id="refback-9-2"></a>[[9]](#ref-9).

(ii) Relation between spin and statistics (<a id="ix-141">Pauli</a>) <a id="refback-11-1"></a>[[11]](#ref-11).

## <a id="s-detailed-program"></a>Detailed Program

We shall not develop straightaway a correct theory including many particles. Instead we follow the historical development. We try to make a relativistic quantum theory of _one_ particle, find out how far we can go and where we get into trouble. Then we shall see how to change the theory and get over the trouble by introducing many particles. Incidentally, the one–particle theories are quite useful, being correct to a good approximation in many situations where <a id="ix-7">creation</a> of new particles does not occur, and where something better than a NR approximation is needed. An example is the Dirac theory of the H atom.<sup><a id="fnref-1" href="#fn-1">1</a></sup>

The NR theory gave levels correctly but no fine-structure. (Accuracy of one part in 10,000). The Dirac one-particle theory gives all the main features of the fine-structure correctly, number of components and separations good to 10% but not better. (Accuracy one part in 100,000).

The Dirac many-particle theory gives the fine-structure separations (Lamb <a id="ix-149">experiment</a>) correctly to about one part in 10,000. (Overall accuracy 1 in 10<sup>8</sup>.)

Probably to get accuracy better than 1 in 10<sup>8</sup> even the DMP theory is not enough and one will need to take all kinds of <a id="ix-197">meson</a> effects into account which are not yet treated properly. Experiments are so far only good to about 1 in 10<sup>8</sup>.

In this course I will go through the one-particle theories first in detail. Then I will talk about their breaking down. At that point I will make a fresh start and discuss how one can make a relativistic quantum theory in general, using the new methods of <a id="ix-77">Feynman</a> and <a id="ix-241">Schwinger</a>. From this we shall be led to the many-particle theories. I will talk about the general features of these theories. Then I will take the special example of <a id="ix-214">quantum electrodynamics</a> and get as far as I can with it before the end of the course.

## <a id="s-one-particle-theories"></a>One-Particle Theories

Take the simplest case, one particle with no forces. Then the NR wave-mechanics tells you to take the equation $E = \frac{1}{2 m} p^{2}$ of classical mechanics, and write

<a id="eq-1"></a>

$$
E \to i \hbar \frac{\partial}{\partial t} \quad p_{x} \to - i \hbar \frac{\partial}{\partial x} \tag{1}
$$

to get the wave-equation<sup><a id="fnref-2" href="#fn-2">2</a></sup>

<a id="eq-2"></a>

$$
i \hbar \frac{\partial}{\partial t} \psi = - \frac{\hbar^{2}}{2 m} \left( \frac{\partial^{2}}{\partial x^{2}} + \frac{\partial^{2}}{\partial y^{2}} + \frac{\partial^{2}}{\partial z^{2}} \right) \psi = - \frac{\hbar^{2}}{2 m} \nabla^{2} \psi \tag{2}
$$

satisfied by the wave-function $\psi$.

To give a physical meaning to $\psi$, we state that $\rho = \psi^{*} \psi$ is the probability of finding the particle at the point $xyz$ at time $t$. And the probability is <a id="ix-176">conserved</a> because<sup><a id="fnref-3" href="#fn-3">3</a></sup>

<a id="eq-3"></a>

$$
\frac{\partial \rho}{\partial t} + \nabla \boldsymbol{\cdot} \hbar = 0 \tag{3}
$$

where

<a id="eq-4"></a>

$$
\hbar = \frac{\hbar}{2 mi} ( \psi^{*} \nabla \psi - \psi \nabla \psi^{*} ) \tag{4}
$$

where $\psi^{*}$ is the complex conjugate of $\psi$.

Now do this relativistically. We have classically

<a id="eq-5"></a>

$$
E^{2} = m^{2} c^{4} + c^{2} p^{2} \tag{5}
$$

<a id="p3"><sup>[p. 3]</sup></a>

which gives the wave equation

<a id="eq-6"></a>

$$
\frac{1}{c^{2}} \frac{\partial^{2}}{\partial t^{2}} \psi = \nabla^{2} \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi \tag{6}
$$

This is an historic equation, the <a id="ix-130">Klein-Gordon equation</a>. <a id="ix-237">Schrödinger</a> already in 1926 tried to make a RQ theory out of it. But he failed, and many other people too, until Pauli and <a id="ix-219">Weisskopf</a> gave the many-particle theory in 1934 <a id="refback-12-1"></a>[[12]](#ref-12). Why?

Because in order to interpret the wave-function as a probability we must have a <a id="ix-172">continuity</a> equation. This can only be got out of the wave-equation if we take $\hbar$ as before, and

<a id="eq-7"></a>

$$
\rho = \frac{i \hbar}{2 mc^{2}} \left( \psi^{*} \frac{\partial \psi}{\partial t} - \frac{\partial \psi^{*}}{\partial t} \psi \right) \tag{7}
$$

But now since the equation is 2<sup>nd</sup> order, $\psi$ and $\frac{\partial \psi}{\partial t}$ are arbitrary. Hence $\rho$ need not be positive. We have _Negative Probabilities_. This defeated all attempts to make a sensible one-particle theory.

The theory can be carried through quite easily, if we make $\psi$ describe an assembly of particles of both positive and negative charge, and $\rho$ is the _net_ charge density at any point. This is what Pauli and Weisskopf did, and the theory you get is correct for $\pi$-mesons, the mesons which are made in the synchrotron downstairs. I will talk about it later.

---

## Notes

<a id="fn-1"></a>**1.** “is” substituted for “in” [↩](#fnref-1)

<a id="fn-2"></a>**2.** Both the first and the second editions use ∆ for the Laplacian differential operator; as there are many quantities with ∆ in them to denote a change or shift, the use of $\nabla^{2}$ for the Laplacian seems a safer choice. [↩](#fnref-2)

<a id="fn-3"></a>**3.** The text symbols “div”, “grad” and “curl” have been replaced throughout by “$\nabla \boldsymbol{\cdot}$”, “$\nabla$”, and “$\nabla \times$”, respectively. For example, the equation (3) originally read [↩](#fnref-3)

$$
\frac{\partial \rho}{\partial t} + \text{ div } j = 0
$$

---

## References

<a id="ref-1"></a>**[1]** Wolfgang Pauli, _General Principles of Quantum Mechanics_, trans. P. Achuthan and K. Venkatesan, Springer-Verlag, Berlin, 1980. This is an English translation of “Principien der Quantentheorie I” in _Handbuch der Physik_, v. 5, 1958, which is a revised edition of the original 1933 work reprinted by Edwards in 1947. The 1933 chapter on quantum electrodynamics is reprinted as Chapter X in the revised English edition. [↩](#refback-1-1)

<a id="ref-2"></a>**[2]** W. Heitler, _The Quantum Theory of Radiation_, 3<sup>rd</sup> ed., Oxford U. P., Oxford, 1954. Reissued in 1984 by Dover Publications. [↩](#refback-2-1)

<a id="ref-3"></a>**[3]** G. Wentzel, _Introduction to the Quantum Theory of Wave Fields_, Interscience, NY, 1949. Reissued in 2003 by Dover Publications as _Quantum Theory of Fields_. [↩](#refback-3-1)

<a id="01-introduction--ref-4"></a>**[4]** J. Schwinger, ed., _Selected Papers on Quantum Electrodynamics_, Dover Publications, New York, 1958. Many of the most important Feynman, Schwinger and Dyson papers, together with those of other authors, are gathered in this anthology edited by Schwinger. [↩](#01-introduction--refback-4-1)

<a id="ref-5"></a>**[5]** Arthur I. Miller, _Early Quantum Electrodynamics: a source book_, Cambridge U. P., Cambridge UK, 1994. Miller’s book includes a valuable historical essay and English translations of three articles cited by Dyson: Heisenberg’s on the Dirac theory of the positron (_Zeits. f. Phys._ **90** (1934) 209), Kramers’s suggestion of mass renormalization (_Nuovo Cim._ NS **15** (1938) 108), and the Pauli-Weisskopf discussion of the relativistic many-particle (scalar) theory (_Helv. Phys. Acta_ **7** (1934) 709). [↩](#refback-5-1)

<a id="01-introduction--ref-6"></a>**[6]** Silvan S. Schweber, _An Introduction to Relativistic Quantum Field Theory_, Row, Peterson & Co., Evanston, IL, 1961. This magisterial textbook has been reissued by Dover Publications (2005) in paperback. Contains a very complete set of references to the QED work done from 1926-1960. [↩](#01-introduction--refback-6-1)

<a id="01-introduction--ref-7"></a>**[7]** Silvan S. Schweber, _QED and the Men Who Made It: Dyson, Feynman, Schwinger and Tomonaga_, Princeton U. P., Princeton NJ, 1994. A very readable, technical history of QED. [↩](#01-introduction--refback-7-1)

<a id="01-introduction--ref-8"></a>**[8]** David Kaiser, _Drawing Theories Apart: the dispersion of Feynman diagrams in postwar physics_, U of Chicago Press, Chicago, 2005. The sociology of the transmission of Feynman’s graphical techniques. [↩](#01-introduction--refback-8-1)

<a id="ref-9"></a>**[9]** P. A. M. Dirac, “The quantum theory of the electron”, _Proc. Roy. Soc. A_ **117** (1928) 610. [↩1](#refback-9-1) [↩2](#refback-9-2)

<a id="ref-10"></a>**[10]** H. Yukawa, “On the interaction of elementary particles”, _Prog. Theo. Phys._ **17** (1935) 48. In Henry A. Boorse and Lloyd Motz, _The World of the Atom_, vol. II, Basic Books, Inc., New York, 1966, pp 1419–1422. [↩](#refback-10-1)

<a id="ref-11"></a>**[11]** W. Pauli, “The Connection Between Spin and Statistics”, _Phys. Rev._ **58** (1940) 716. In Schwinger, _Selected Papers in Quantum Electrodynamics_, pp 372–378. [↩](#refback-11-1)

<a id="ref-12"></a>**[12]** W. Pauli and V. Weisskopf, “The quantization of the scalar relativistic wave equation”, _Helv. Phys._ _Acta_ **7** (1934) 709. In Miller, _Early Quantum Electrodynamics_, pp 188–205 (English). [↩](#refback-12-1)

---

<a id="02-the-dirac-theory"></a>

# The Dirac Theory

<a id="p4"><sup>[p. 4]</sup></a>

## <a id="s-the-form-of-the-dirac-equation"></a>The Form of the <a id="ix-85">Dirac</a> <a id="ix-49">Equation</a>

Historically before the RQ theory came the one-particle theory of Dirac. This was so successful in dealing with the electron, that it was for many years the only respectable RQ theory in existence. And its difficulties are a lot less immediate than the difficulties of the one-particle KG theory.

Dirac said, suppose the particle can exist in several distinct states with the same <a id="ix-199">momentum</a> (different orientations of spin.) Then the wave-function $\psi$ satisfying (6) must have several _components_; it is not a scalar but a set of numbers each giving the prob. <a id="ix-11">amplitude</a> to find the particle at a given place and in a given substate. So we write for $\psi$ a column matrix

$$
\psi = \begin{bmatrix} \psi_{1} \\ \psi_{2} \\ \cdot \\ \cdot \\ \cdot \end{bmatrix} \quad \text{ for the components } \psi_{\alpha} ; \quad \alpha = 1 , 2 ,\dots
$$

Dirac assumed that the probability <a id="ix-184">density</a> at any point is still given by

<a id="eq-8"></a>

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

<a id="eq-9"></a>

$$
\frac{1}{c} \frac{\partial \psi}{\partial t} + \sum_{1}^{3} \alpha^{k} \frac{\partial \psi}{\partial x_{k}} + i \frac{mc}{\hbar} \beta \psi = 0 \tag{9}
$$

where $x_{1} x_{2} x_{3}$ are written for $xyz$ and $\alpha^{1} \alpha^{2} \alpha^{3} \beta$ are square matrices whose elements are numbers. The <a id="ix-118">conjugate</a> of (9) gives

<a id="eq-10"></a>

$$
\frac{1}{c} \frac{\partial \psi^{*}}{\partial t} + \sum_{1}^{3} \frac{\partial \psi^{*}}{\partial x_{k}} \alpha^{k *} - i \frac{mc}{\hbar} \psi^{*} \beta^{*} = 0 \tag{10}
$$

where $\alpha^{k *}$ and $\beta^{*}$ are <a id="ix-114">Hermitian</a> conjugates.

Now to get (3) out of (8), (9) and (10) we must have $\alpha^{k *} = \alpha^{k} , \beta^{*} = \beta$ so $\alpha^{k}$ and $\beta$ are _Hermitian_; and

<a id="eq-11"></a>

$$
j_{k} = c \left( \psi^{*} \alpha^{k} \psi \right) \tag{11}
$$

<a id="p5"><sup>[p. 5]</sup></a>

Next what more do we want from equation (9)? Two things. (A) it must be consistent with the 2<sup>nd</sup> order equation (6) we started from; (B) the whole theory must be <a id="ix-157">Lorentz</a> invariant.

First consider (A). If (9) is consistent with (6) it must be possible to get exactly (6) by multiplying (9) by the operator

<a id="eq-12"></a>

$$
\frac{1}{c} \frac{\partial}{\partial t} - \sum_{1}^{3} \alpha^{\ell} \frac{\partial}{\partial x_{\ell}} - i \frac{mc}{\hbar} \beta \tag{12}
$$

chosen so that the terms with mixed derivatives $\frac{\partial}{\partial t} \frac{\partial}{\partial x_{k}}$ and $\frac{\partial}{\partial t}$ cancel. This gives

$$
\begin{aligned} \frac{1}{c^{2}} \frac{\partial^{2} \psi}{\partial t^{2}} &= \sum \sum_{k \neq \ell} \frac{1}{2} \left( \alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} \right) \frac{\partial^{2} \psi}{\partial x_{k} \partial x_{\ell}} + \sum_{k} \alpha_{k}^{2} \frac{\partial^{2} \psi}{\partial x_{k}^{2}} \\ & \quad - \frac{m^{2} c^{2}}{\hbar^{2}} \beta^{2} \psi + i \frac{mc}{\hbar} \sum_{k} \left( \alpha^{k} \beta + \beta \alpha^{k} \right) \frac{\partial \psi}{\partial x_{k}} \end{aligned}
$$

This agrees with (6) if and only if

<a id="eq-13"></a>

$$
\begin{aligned} \alpha^{k} \alpha^{\ell} + \alpha^{\ell} \alpha^{k} &= 0 \quad k \neq \ell \\ \alpha^{k} \beta + \beta \alpha^{k} &= 0 \\ \alpha^{k 2} &= \beta^{2} = \mathbb{I} , ( \text{ identity matrix } ) \end{aligned} \tag{13}
$$

Thus we could not possibly _factorize_ the 2<sup>nd</sup> order equation into two first-order operators involving ordinary numbers. But we can do it with <a id="ix-69"><em>matrices</em></a>.

Consider the Pauli spin matrices

<a id="eq-14"></a>

$$
\sigma_{1} = \begin{pmatrix} 0 & 1 \\ 1 & 0 \end{pmatrix} \quad \sigma_{2} = \begin{pmatrix} 0 & - i \\ i & 0 \end{pmatrix} \quad \sigma_{3} = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix} \tag{14}
$$

you are familiar with. They satisfy

$$
\sigma_{k} \sigma_{\ell} + \sigma_{\ell} \sigma_{k} = 2 \delta_{\ell k}
$$

But we cannot make 4 matrices of this type all anti-commuting. They must be _at least_ 4 $\times$ 4.

One possible set of $\alpha^{k}$ and $\beta$ is

<a id="eq-15"></a>

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

<a id="eq-16"></a>

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

<a id="eq-17"></a>

$$
( \psi^{*\prime} \psi^{\prime} ) = \frac{\psi^{*} \psi}{\sqrt{1 - \frac{v^{2}}{c^{2}}}} \tag{17}
$$

and so $( \psi^{*} \psi ) = \rho$ transforms like an _energy_, i.e.like the fourth component of a vector. This shows incidentally that $\psi^{\prime} \neq \psi$. Since $\rho$ and $\hbar$ are related by the equation of continuity, the space-components of the 4-vector are

<a id="eq-18"></a>

$$
( S_{1} ,S_{2} ,S_{3} ) = \psi^{*} \alpha^{k} \psi = \frac{1}{c} j_{k} \tag{18}
$$

So we require that the 4 quantities

<a id="eq-19"></a>

$$
( S_{1} ,S_{2} ,S_{3} ,S_{0} ) = \left( \psi^{*} \alpha^{k} \psi ,\psi^{*} \psi \right) \tag{19}
$$

transform like a _4-vector_. This will be enough to preserve the interpretation of the theory.

Assume that

<a id="eq-20"></a>

$$
\psi^{\prime} = S\psi \tag{20}
$$

where $S$ is a _linear_ operator. Then

<a id="eq-21"></a>

$$
\psi^{\prime *} = \psi^{*} S^{*} \tag{21}
$$

So we require

<a id="eq-22"></a>

$$
\begin{aligned} \psi^{*\prime} \alpha^{k} \psi^{\prime} &= \psi^{*} S^{*} \alpha^{k} S\psi = \sum_{\nu = 0}^{3} a_{k\nu} \psi^{*} \alpha^{\nu} \psi \\ \psi^{*\prime} \psi^{\prime} &= \psi^{*} S^{*} S\psi = \sum_{\nu = 0}^{3} a_{0 \nu} \psi^{*} \alpha^{\nu} \psi \end{aligned} \tag{22}
$$

writing $\alpha^{0} = \mathbb{I}$.

<a id="p7"><sup>[p. 7]</sup></a>

Thus we need $3$

<a id="eq-23"></a>

$$
S^{*} \alpha^{\mu} S = \sum_{\nu = 0} a_{\mu \nu} \alpha^{\nu} , \quad \mu = 0 , 1 , 2 , 3 \tag{23}
$$

Next consider (ii). The Dirac equation for $\psi^{\prime}$ is

<a id="eq-24"></a>

$$
\sum_{0}^{3} \alpha^{\nu} \frac{\partial}{\partial x_{\nu}^{\prime}} \psi^{\prime} + i \frac{mc}{\hbar} \beta \psi^{\prime} = 0 \tag{24}
$$

Now the original Dirac equation for $\psi$ expressed in terms of the new coordinates is

<a id="eq-25"></a>

$$
\sum_{\mu = 0 \nu = 0}^{3} \sum^{3} \alpha^{\mu} \frac{\partial}{\partial x_{\nu}^{\prime}} a_{\nu \mu} S^{- 1} \psi^{\prime} + i \frac{mc}{\hbar} \beta S^{- 1} \psi^{\prime} = 0 \tag{25}
$$

The sets of equations (24) and (25) have to be equivalent, not identical. Thus (25) must be the same as (24) multiplied by $\beta S^{- 1} \beta$. The condition for this is

<a id="eq-26"></a>

$$
\beta S^{- 1} \beta \alpha^{\nu} = \sum_{0}^{3} \alpha^{\lambda} a_{\nu \lambda} S^{- 1} \tag{26}
$$

But (23) and (26) are identical if

<a id="eq-27"></a>

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

<a id="eq-28"></a>

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

<a id="eq-29"></a>

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

<a id="eq-30"></a>

$$
S = S^{*} = \beta \tag{30}
$$

Note that in all cases S is ambiguous by a factor $\pm$1. So in case 1 a rotation though 360$◦$ gives $S = - 1 .$

_Problem 1_ Find the S corresponding to a general infinitesimal coordinate transformation. Compare and show that it agrees with the exact solutions given here.

The $\psi_{\alpha}$’s transforming with these $S$-transformations are called <a id="ix-213"><em>spinors</em></a>. They are a direct extension of the non-relativistic 2-component spin-functions. Mathematical theory of spinors is not very useful. In fact we find always in practice, calculations can be done most easily if one avoids any explicit representation of the spinors. _Use only formal algebra and commutation relations of the matrices._

## <a id="s-the-covariant-notation"></a>The covariant notation

In order to avoid distinction between covariant and contravariant vectors (which we have also unjustifiably ignored in the previous discussion) it is useful to use the imaginary 4<sup>th</sup> coordinate

<a id="eq-31"></a>

$$
x_{4} = ix_{0} = ict \tag{31}
$$

In this coordinate system the four matrices<sup><a id="fnref-5" href="#fn-5">5</a></sup>

<a id="eq-32"></a>

$$
\gamma_{1 , 2 , 3 , 4} = \left( - i\beta \alpha^{1 , 2 , 3} ,\beta \right) \quad i . e . \tag{32}
$$

<a id="p9"><sup>[p. 9]</sup></a>

$$
\gamma_{1} = \begin{pmatrix} 0 & \begin{matrix} 0 & - i \\ - i & 0 \end{matrix} \\ \begin{matrix} 0 & i \\ i & 0 \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{2} = \begin{pmatrix} 0 & \begin{matrix} 0 & - 1 \\ 1 & 0 \end{matrix} \\ \begin{matrix} 0 & 1 \\ - 1 & 0 \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{3} = \begin{pmatrix} 0 & \begin{matrix} - i & 0 \\ 0 & i \end{matrix} \\ \begin{matrix} i & 0 \\ 0 & - i \end{matrix} & 0 \end{pmatrix} ; \quad \gamma_{4} = \begin{pmatrix} \begin{matrix} 1 & 0 \\ 0 & 1 \end{matrix} & 0 \\ 0 & \begin{matrix} - 1 & 0 \\ 0 & - 1 \end{matrix} \end{pmatrix}
$$

are a 4-vector. They are all <a id="ix-115">Hermitian</a> and satisfy

<a id="eq-33"></a>

$$
\gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu} = 2 \delta_{\mu \nu} \tag{33}
$$

The Dirac equation and its conjugate may now be written

<a id="eq-34"></a>

$$
\begin{aligned} \sum_{1}^{4} \gamma_{\mu} \frac{\partial \psi}{\partial x_{\mu}} + \frac{mc}{\hbar} \psi &= 0 \\ \sum_{1}^{4} \frac{\partial \overline{\psi}}{\partial x_{\mu}} \gamma_{\mu} - \frac{mc}{\hbar} \overline{\psi} &= 0 \end{aligned} \tag{34}
$$

with

<a id="eq-35"></a>

$$
\overline{\psi} = \psi^{*} \beta \quad \text{ and } \tag{35}
$$

<a id="eq-36"></a>

$$
s_{\mu} = i \left( \overline{\psi} \gamma_{\mu} \psi \right) = \left( \frac{1}{c} \hbar ,i\rho \right) \tag{36}
$$

These notations are much the most convenient for calculations.

## <a id="s-conservation-laws-existence-of-spin"></a><a id="ix-64">Conservation Laws</a>. <a id="ix-211">Existence of Spin</a>.

The Hamiltonian in this theory is<sup><a id="fnref-6" href="#fn-6">6</a></sup>

<a id="eq-37"></a>

$$
i \hbar \frac{\partial \psi}{\partial t} = H\psi \tag{37}
$$

<a id="eq-38"></a>

$$
H = - i \hbar c \sum_{1}^{3} \alpha^{k} \frac{\partial}{\partial x_{k}} + mc^{2} \beta = - i \hbar c \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla + mc^{2} \beta \tag{38}
$$

This commutes with the <a id="ix-200">momentum</a> $\boldsymbol{p} = - i \hbar \nabla$. So the momentum $\boldsymbol{p}$ is a constant of motion.

However the <a id="ix-27">angular momentum</a> operator

<a id="eq-39"></a>

$$
\boldsymbol{L} = \boldsymbol{r} \times \boldsymbol{p} = - i \hbar \boldsymbol{r} \times \nabla \tag{39}
$$

is not a constant. For

<a id="eq-40"></a>

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

<a id="eq-41"></a>

$$
\begin{aligned} [ H,\sigma_{3} ] &= 2 \hbar c \left( \alpha^{1} \nabla_{2} - \alpha^{2} \nabla_{1} \right) \quad \text{ and thus } \\ [ H, \boldsymbol{\sigma} ] &= 2 \hbar c \boldsymbol{\alpha} \times \nabla \end{aligned} \tag{41}
$$

Thus

<a id="eq-42"></a>

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

<a id="eq-43"></a>

$$
\psi ( x,t ) = u \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right) \tag{43}
$$

where $u$ is a constant spinor. The <a id="ix-86">Dirac</a> <a id="ix-50">equation</a> then becomes an equation for $u$ only

<a id="eq-44"></a>

$$
Eu = \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) u \tag{44}
$$

We write now

<a id="eq-45"></a>

$$
p_{+} = p_{1} + ip_{2} \quad p_{-} = p_{1} - ip_{2} \tag{45}
$$

Then (44) written out in full becomes

<a id="eq-46"></a>

$$
\begin{aligned} \left( E - mc^{2} \right) u_{1} &= c ( p_{3} u_{3} + p_{-} u_{4} ) \\ \left( E - mc^{2} \right) u_{2} &= c ( p_{+} u_{3} - p_{3} u_{4} ) \\ \left( E + mc^{2} \right) u_{3} &= c ( p_{3} u_{1} + p_{-} u_{2} ) \\ \left( E + mc^{2} \right) u_{4} &= c ( p_{+} u_{1} - p_{3} u_{2} ) \end{aligned} \tag{46}
$$

These 4 equations determine $u_{3}$ and $u_{4}$ given $u_{1}$ and $u_{2}$, or vice-versa. And either $u_{1}$ and $u_{2}$, or $u_{3}$ and $u_{4}$, can be chosen arbitrarily provided that<sup><a id="fnref-7" href="#fn-7">7</a></sup>

<a id="eq-47"></a>

$$
E^{2} = m^{2} c^{4} + c^{2} p^{2} \tag{47}
$$

Thus given $p$ and $E$ = +$\sqrt{m^{2} c^{4} + c^{2} p^{2}}$, there are two independent solutions of (46); these are, in nonnormalized form:

<a id="eq-48"></a>

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

<a id="eq-49"></a>

$$
\varphi ( x,t ) = v \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right) \tag{49}
$$

as always in quantum mechanics. But the negative-energy electron whose _absence is_ the positron has a wave-function

<a id="eq-50"></a>

$$
\psi ( x,t ) = u \exp \left( - i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} + i \frac{Et}{\hbar} \right) \tag{50}
$$

since it has a momentum $- p$ energy $- E$.

Thus we must take

<a id="eq-51"></a>

$$
\varphi = C\psi^{+} , \quad i . e . \quad v = Cu^{+} \tag{51}
$$

where $\psi^{+}$ is $\psi$ with complex conjugate elements but _not_ transposed, and $C$ is a suitable constant matrix;

$$
\psi^{+} ( x,t ) = u^{+} \exp \left( i \frac{\boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x}}{\hbar} - i \frac{Et}{\hbar} \right)
$$

We know that $u$ is a solution of

<a id="eq-52"></a>

$$
Eu = \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} - mc^{2} \beta \right) u \tag{52}
$$

We want the theory to make no distinction between electrons and positrons, and so $v$ must also satisfy the Dirac equation

<a id="eq-53"></a>

$$
\begin{aligned} Ev &= \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) v \\ ECu^{+} &= \left( c \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} + mc^{2} \beta \right) Cu^{+} \end{aligned} \tag{53}
$$

But from (52) we have for $u^{+}$ the equation

<a id="eq-54"></a>

$$
Eu^{+} = \left( c \boldsymbol{\alpha}^{+} \boldsymbol{\cdot} \boldsymbol{p} - mc^{2} \beta^{+} \right) u^{+} \tag{54}
$$

In order that (53) and (54) be identical we should have

<a id="eq-55"></a>

$$
C\alpha^{k +} = \alpha^{k} C, \quad C\beta^{+} = - \beta C \tag{55}
$$

Now in fact

$$
\alpha^{1 +} = \alpha^{1} \quad \alpha^{3 +} = \alpha^{3} \quad \alpha^{2 +} = - \alpha^{2} \quad \beta^{+} = \beta
$$

Therefore a suitable C will be

<a id="eq-56"></a>

$$
C = - i\beta \alpha^{2} = \gamma_{2} = \begin{pmatrix} 0 & \begin{matrix} 0 & - 1 \\ 1 & 0 \end{matrix} \\ \begin{matrix} 0 & 1 \\ - 1 & 0 \end{matrix} & 0 \end{pmatrix} \tag{56}
$$

<a id="p12"><sup>[p. 12]</sup></a>

The relation between $\psi$ and $\varphi$ is symmetrical because

<a id="eq-57"></a>

$$
C^{2} = \mathbb{I} \quad \text{ Hence } \quad \psi = C\varphi^{+} \tag{57}
$$

The $\varphi$ is called the _charge-conjugate_ wave-function corresponding to the <a id="ix-106">negative-energy</a> electron $\psi$. Clearly

<a id="eq-58"></a>

$$
\varphi^{*} \varphi = \left( C\psi^{+} \right)^{*} \left( C\psi^{+} \right) = \psi^{T} C^{*} C\psi^{+} = \psi^{*} ( C^{*} C )^{T} \psi = \psi^{*} \psi \tag{58}
$$

And

<a id="eq-59"></a>

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

<a id="eq-60"></a>

$$
\begin{aligned} & \quad E + e \Phi \quad \text{ for } E \\ & \quad \boldsymbol{p} + \frac{e}{c} \boldsymbol{A} \quad \text{ for } \boldsymbol{p} \end{aligned} \tag{60}
$$

where $( - e )$ is the electron charge. We write the momentum-energy 4-vector

<a id="eq-61"></a>

$$
p = ( p_{1} ,p_{2} ,p_{3} ,p_{4} = iE/c ) \tag{61}
$$

Then we have to substitute simply

<a id="eq-62"></a>

$$
p_{\mu} + \frac{e}{c} A_{\mu} \quad \text{ for } p_{\mu} \tag{62}
$$

Now in the quantum theory

<a id="eq-63"></a>

$$
p_{\mu} \to - i \hbar \frac{\partial}{\partial x_{\mu}} \tag{63}
$$

Therefore the Dirac equation with fields is

<a id="eq-64"></a>

$$
\sum_{1}^{4} \gamma_{\mu} \left( \frac{\partial}{\partial x_{\mu}} + \frac{ie}{\hbar c} A_{\mu} \right) \psi + \frac{mc}{\hbar} \psi = 0 \tag{64}
$$

<a id="eq-65"></a>

$$
\sum_{1}^{4} \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \overline{\psi} \gamma_{\mu} - \frac{mc}{\hbar} \overline{\psi} = 0 \tag{65}
$$

In the non-covariant notations this is

<a id="eq-66"></a>

$$
i \hbar \frac{\partial \psi}{\partial t} = \left[ - e \Phi + \sum_{1}^{3} \left( - i \hbar c \frac{\partial}{\partial x_{k}} + eA_{k} \right) \alpha^{k} + mc^{2} \beta \right] \psi \tag{66}
$$

<a id="p13"><sup>[p. 13]</sup></a>

since by (57), we have $\overline{\psi} \gamma_{\mu} = \psi^{*} \beta \gamma_{\mu} = ( C\varphi^{+} )^{T} \beta \gamma_{\mu} = \varphi^{T} C^{T} \beta \gamma_{\mu}$; the wave function $\varphi = C\psi^{+}$of a positron satisfies by (65)

<a id="eq-67"></a>

$$
\sum \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \gamma_{\mu}^{T} \beta C\varphi - \frac{mc}{\hbar} \beta C\varphi = 0 \tag{67}
$$

Multiplying by $C\beta$ this gives

<a id="eq-68"></a>

$$
\sum \left( \frac{\partial}{\partial x_{\mu}} - \frac{ie}{\hbar c} A_{\mu} \right) \gamma_{\mu} \varphi + \frac{mc}{\hbar} \varphi = 0 \tag{68}
$$

This is exactly the Dirac <a id="ix-51">equation</a> for a particle of positive charge (+e). We have used

<a id="eq-69"></a>

$$
C\beta \gamma_{\mu}^{T} \beta C = - \gamma_{\mu} , \tag{69}
$$

which follows from (15), (32), and (55).

## <a id="s-the-hydrogen-atom"></a>The <a id="ix-120">Hydrogen Atom</a>

This is the one problem which it is possible to treat very accurately using the one-electron Dirac theory. The problem is to find the eigenstates of the equation

<a id="eq-70"></a>

$$
\begin{aligned} E\psi &= H\psi \\ H &= - i \hbar c \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla + mc^{2} \beta - \frac{e^{2}}{r} \end{aligned} \tag{70}
$$

As in the NR theory, we have as quantum numbers in addition to $E$ itself the quantities

<a id="eq-71"></a>

$$
j_{z} = - i [ \boldsymbol{r} \times \nabla ]_{3} + \tfrac{1}{2} \sigma_{3} \tag{71}
$$

<a id="eq-72"></a>

$$
j ( j + 1 ) = J^{2} = \left[ - i ( \boldsymbol{r} \times \nabla ) + \tfrac{1}{2} \boldsymbol{\sigma} \right]^{2} \tag{72}
$$

where $j_{z}$ and $j$ are now _half-odd integers_ by the ordinary theory of <a id="ix-28">angular momenta</a>. These quantum numbers are not enough to fix the state, because each value of $j$ may correspond to two NR states with $\ell$between states with$= j \pm \tfrac{1}{2}$. Therefore we need an additional operator which commutes with $H$, which will distinguish $\boldsymbol{\sigma}$ parallel or antiparallel to $\boldsymbol{J}$. The obvious choice is

$$
Q = \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{J}
$$

But $[ H, \boldsymbol{\sigma} ]$ is non-zero and rather complicated. So it is better to try

<a id="eq-73"></a>

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

<a id="eq-74"></a>

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

<a id="eq-75"></a>

$$
K^{2} = J^{2} + \tfrac{1}{4} = \left( j + \tfrac{1}{2} \right)^{2} \tag{75}
$$

Therefore $K$ has _integer_ eigenvalues not zero,

<a id="eq-76"></a>

$$
K = k = \pm \left( j + \tfrac{1}{2} \right) \tag{76}
$$

<a id="eq-77"></a>

$$
j = | k |- \tfrac{1}{2} , \quad k = \pm 1 , \pm 2 , \pm 3 ,\dots \tag{77}
$$

Using the eigenvalue for $K$, we can simplify the <a id="ix-121">Hamiltonian</a>, which we could not do as in the NR theory with the eigenvalue of $L^{2}$ alone. First

<a id="eq-78"></a>

$$
\boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{r\sigma} \boldsymbol{\cdot} ( \boldsymbol{r} \times \nabla ) = i \boldsymbol{\sigma} \boldsymbol{\cdot} ( \boldsymbol{r} \times ( \boldsymbol{r} \times \nabla ) ) = i ( \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{r} ) ( \boldsymbol{r} \boldsymbol{\cdot} \nabla ) - ir^{2} \boldsymbol{\sigma} \boldsymbol{\cdot} \nabla \tag{78}
$$

Let now

<a id="eq-79"></a>

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

<a id="eq-80"></a>

$$
H = mc^{2} \beta - \frac{e^{2}}{r} + i \hbar c\alpha_{r} \left( \frac{\beta K}{r} - \frac{1}{r} - \frac{\partial}{\partial r} \right) \tag{80}
$$

This gives the <a id="ix-87">Dirac</a> <a id="ix-52">equation</a> as an equation in the single variable $r$, having separated all angular variables.

For the solution of this equation, see – Dirac, _Quantum Mechanics_, Third Edition, Sec. 72, pp 268-271.

## <a id="s-solution-of-the-radial-equation"></a>Solution of Radial Equation

We may choose a two-component representation in which

<a id="eq-81"></a>

$$
\beta = \begin{pmatrix} 1 & 0 \\ 0 & - 1 \end{pmatrix} , \quad \alpha_{r} = \begin{pmatrix} 0 & i \\ - i & 0 \end{pmatrix} , \quad \psi = \begin{pmatrix} u \\ v \end{pmatrix} \tag{81}
$$

<a id="p15"><sup>[p. 15]</sup></a>

Then

<a id="eq-82"></a>

$$
\begin{aligned} \left( E - mc^{2} \right) u &= - \frac{e^{2}}{r} u + \hbar c \left( \frac{1 + K}{r} + \frac{\partial}{\partial r} \right) v \\ \left( E + mc^{2} \right) v &= - \frac{e^{2}}{r} v + \hbar c \left( - \frac{1 - K}{r} - \frac{\partial}{\partial r} \right) u \end{aligned} \tag{82}
$$

Let now

<a id="eq-83"></a>

$$
a_{1} = \frac{- E + mc^{2}}{\hbar c} ; \quad a_{2} = \frac{E + mc^{2}}{\hbar c} ; \quad \alpha = \frac{e^{2}}{\hbar c} , \tag{83}
$$

the <a id="ix-89">fine structure constant</a>. Then

<a id="eq-84"></a>

$$
\begin{aligned} \left( - a_{1} + \frac{\alpha}{\overline{r}} \right) u &= \left( \frac{1 + K}{r} + \frac{\partial}{\partial r} \right) v \\ \left( a_{2} + \frac{\alpha}{r} \right) v &= \left( \frac{- 1 + K}{r} - \frac{\partial}{\partial r} \right) u \end{aligned} \tag{84}
$$

Next put $a = \sqrt{a_{1} a_{2}} = \sqrt{m^{2} c^{4} - E^{2}} / \hbar c$ which is the magnitude of the imaginary <a id="ix-201">momentum</a> of a free electron of energy $E$. Then $\psi \sim e^{- ar}$ at infinity. Hence we write

<a id="eq-85"></a>

$$
\begin{aligned} u &= \frac{e^{- ar}}{r} f \\ v &= \frac{e^{- ar}}{r} g \end{aligned} \tag{85}
$$

So

<a id="eq-86"></a>

$$
\begin{aligned} \left( \frac{\alpha}{r} - a_{1} \right) f &= \left( \frac{\partial}{\partial r} - a + \frac{k}{r} \right) g \\ \left( \frac{\alpha}{r} + a_{2} \right) g &= \left( - \frac{\partial}{\partial r} + a + \frac{k}{r} \right) f \end{aligned} \tag{86}
$$

Now we try solutions in series

<a id="eq-87"></a>

$$
f = \sum c_{s} r^{s} , \quad g = \sum d_{s} r^{s} \tag{87}
$$

This gives

<a id="eq-88"></a>

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

<a id="eq-89"></a>

$$
E>mc^{2} \tag{89}
$$

For real $a$ the series must terminate at both ends in order not to blow up at infinity. Suppose then<sup><a id="fnref-8" href="#fn-8">8</a></sup> $e_{s}$ is non-zero for

<a id="eq-90"></a>

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

<a id="eq-91"></a>

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

<a id="eq-92"></a>

$$
E = \frac{mc^{2}}{\sqrt{1 + \frac{\alpha^{2}}{\left( n + \sqrt{k^{2} - \alpha^{2}} \right)^{2}}}} \tag{92}
$$

Given this positive $E , \left( a_{1}^{2} - a^{2} \right)$ is negative (see (83) and (84)) and so it is allowable to square $( \epsilon + n )$ to find these solutions, without introducing any difficulties. So for each

<a id="eq-93"></a>

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

<a id="eq-94"></a>

$$
n = 0 , \quad k = + 1 , + 2 , + 3 ,\dots \tag{94}
$$

<a id="p17"><sup>[p. 17]</sup></a>

The principal quantum number $N$ is

$$
N = n + | k |
$$

Expanding in powers of $\alpha$

<a id="eq-95"></a>

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

<a id="eq-96"></a>

$$
\sum_{\mu} \sum_{\nu} \gamma_{\mu} \gamma_{\nu} \left( \frac{\partial}{\partial x_{\mu}} + i \frac{e}{\hbar c} A_{\mu} \right)^{\nu} \left( \frac{\partial}{\partial x_{\nu}} + i \frac{e}{\hbar c} A_{\nu} \right) \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi = 0 \tag{96}
$$

Using $\gamma_{\mu}^{2}$ = 1$,\gamma_{\mu} \gamma_{\nu} + \gamma_{\nu} \gamma_{\mu}$ = 0 this gives

<a id="eq-97"></a>

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

<a id="eq-98"></a>

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

<a id="eq-99"></a>

$$
i \hbar \frac{\partial \psi}{\partial t} = \left\{ mc^{2} - e \Phi - \frac{h^{2}}{2 m} \sum_{k = 1}^{3} \left( \frac{\partial}{\partial x_{k}} + \frac{ie}{\hbar c} A_{k} \right)^{2} + \frac{e \hbar}{2 mc} ( \boldsymbol{\sigma} \boldsymbol{\cdot} \boldsymbol{H} - i \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E} ) \right\} \psi \tag{99}
$$

The term $\boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{E}$ is really relativistic, and should be dropped or treated more exactly. Then we have exactly the equation of motion of a NR particle with a spin <a id="ix-102">magnetic moment</a> equal to

<a id="eq-100"></a>

$$
M = - \frac{e \hbar}{2 mc} \sigma \tag{100}
$$

This is one of the greatest triumphs of Dirac, that he got this magnetic moment right out of his general assumptions without any arbitrariness.

It is confirmed by measurements to about one part in 1000. Note that the most recent experiments show a definite discrepancy, and agree with the value

<a id="eq-101"></a>

$$
M = - \frac{e \hbar}{2 mc} \sigma \left\{ 1 + \frac{e^{2}}{2 \pi \hbar c} \right\} \tag{101}
$$

calculated by <a id="ix-242">Schwinger</a> using the complete many-particle theory.

_Problem 2_ Calculate energy values and <a id="ix-80">wave functions</a> of a Dirac particle moving in a homogeneous infinite magnetic field. Can be done exactly. See F. <a id="ix-226">Sauter</a>, _Zeitschrift für Physik_ **69** (1931) 742.

_Solution_<br>
Take the field $\boldsymbol{B}$ in the $z$ direction.

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

<a id="03-scattering-problems-and-born-approximation"></a>

# Scattering Problems and Born Approximation

<a id="p22"><sup>[p. 22]</sup></a>

## <a id="s-general-discussion"></a>General Discussion

The problem of scattering of a Dirac particle by a potential can be treated exactly by finding the continuum solutions of the Dirac <a id="ix-54">equation</a>. This is a complicated business even for the simplest case of a Coulomb force. It was done by <a id="ix-134">Mott</a>, _Proc. Roy. Soc._ **A135** (1932) 429.

For most purposes in relativistic problems, and always when the scattering is produced by complicated effects involving radiation theory, one uses the <a id="ix-45"><em>Born approximation</em></a>. That is, we treat the scattering only to 1<sup>st</sup> order in the interaction, or only to some definite order in which we are interested.

The formula for scattering from an initial state $A$ to a final state $B$ lying in a continuum of <a id="ix-108">states</a> is, <a id="ix-205">transition</a> probability per unit time,

<a id="eq-102"></a>

$$
w = \frac{2 \pi}{\hbar} \rho_{E} | V_{BA} |^{2} \tag{102}
$$

This you ought to know. $\rho_{E}$ = density of final states per unit energy interval. $V_{BA}$ is the matrix element of the potential $V$ for the transition. Here $V$ may be anything, and may be itself a second-order or higher order effect obtained by using higher-order perturbation theory.

The difficulties in real calculations usually come from the factors 2 and $\pi$ and the correct _normalization_ _of states_. Always I shall normalize the continuum states not in the usual way (one particle per unit volume) which is non-invariant, but instead

<a id="eq-103"></a>

$$
\text{ One particle per volume } \frac{mc^{2}}{| E |} \tag{103}
$$

where $| E |$ is the energy of the particles. Then under a <a id="ix-166">Lorentz</a> transformation the volume of a fixed region transforms like 1$/ | E |$ and so the definition stays invariant.

Thus a continuum state given by the spinor $\psi = u$exp$\{ ( i \boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{x} - iEt ) / \hbar \}$ is normalized so that

<a id="eq-104"></a>

$$
u^{*} u = \frac{| E |}{mc^{2}} \tag{104}
$$

Now if we multiply the Dirac equation for a free particle, (44), by $\overline{u}$ on the left, we get $Eu^{*} \beta u = cu^{*} \beta \boldsymbol{\alpha} \boldsymbol{\cdot}$ $\boldsymbol{p} u + mc^{2} u^{*} u$; its complex conjugate is $Eu^{*} \beta u = - cu^{*} \beta \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{p} u + mc^{2} u^{*} u$ since $\beta \boldsymbol{\alpha}$ is anti-<a id="ix-116">Hermitian</a>; then by adding we get

<a id="eq-105"></a>

$$
E \overline{u} u = mc^{2} u^{*} u \tag{105}
$$

Therefore the normalization becomes

<a id="eq-106"></a>

$$
\left. \begin{aligned} \overline{u} u &= + 1 \text{ for electron states } \\ &= - 1 \text{ for positron states } \end{aligned} \right\} = \epsilon ; \quad \text{ This is the definition of } \epsilon . \tag{106}
$$

With this normalization the density of states in momentum <a id="ix-132">space</a> is one per volume $h^{3}$ of phase space, that is to say

<a id="eq-107"></a>

$$
\rho = \frac{1}{h^{3}} \frac{mc^{2}}{| E |} dp_{1} dp_{2} dp_{3} \tag{107}
$$

per volume $dp_{1} dp_{2} dp_{3}$ of momentum space, for _each_ direction of spin and each sign of charge. Now we have again an invariant differential

<a id="eq-108"></a>

$$
\frac{dp_{1} dp_{2} dp_{3}}{| E |} \tag{108}
$$

<a id="p23"><sup>[p. 23]</sup></a>

## <a id="s-projection-operators"></a>Projection Operators

Usually we are not interested in the spin either of an intermediate or of an initial or final state. Therefore we have to make sums over spin <a id="ix-162">states</a> which are of the form

<a id="eq-109"></a>

$$
S = \sum_{2} ( \overline{s} Ou ) ( \overline{u} Pr ) \tag{109}
$$

where $O$ and $P$ are some kind of operators, $s$ and $r$ some kind of spin states, and the sum is over the two spin states $u$ of an electron of <a id="ix-202">momentum</a> $\boldsymbol{p}$ and energy $E$.

Let us write

<a id="eq-110"></a>

$$
p / = \sum_{\mu} p_{\mu} \gamma_{\mu} , \quad p_{4} = iE/c \tag{110}
$$

The Dirac <a id="ix-55">equation</a> satisfied by $u$ is

<a id="eq-111"></a>

$$
\left( p / - imc \right) u = 0 \tag{111}
$$

The two spin states with momentum 4-vector $- p$ satisfy

<a id="eq-112"></a>

$$
\left( p / + imc \right) u = 0 \tag{112}
$$

As one can easily show from (48), these 4 states are all orthogonal in the sense that $( \overline{u}^{\prime} u )$ = 0 for each pair $u^{\prime} u$. Therefore the identity operator may be written in the form

<a id="eq-113"></a>

$$
\mathbb{I} = \sum_{4} ( u \overline{u} ) \epsilon \tag{113}
$$

summed over all 4 states with $\epsilon$ defined as earlier. Hence by (111) and (112) we can write (109) as

<a id="eq-114"></a>

$$
S = \sum_{4} \left( \overline{s} O \frac{p / + imc}{2 imc} \epsilon u \right) ( \overline{u} Pr ) = ( \overline{s} O \Lambda_{+} Pr ) \tag{114}
$$

by virtue of (113); here the operator

<a id="eq-115"></a>

$$
\Lambda_{+} = \frac{p / + imc}{2 imc} \tag{115}
$$

is a <a id="ix-212">projection operator</a> for _electrons_ of momentum $p$.

In the same way for a sum over the two positron states $u$ with momentum $p$ energy $E$

<a id="eq-116"></a>

$$
S = \sum_{2} ( \overline{s} Ou ) ( \overline{u} Pr ) = ( \overline{s} O \Lambda_{-} Pr ) \tag{116}
$$

with

<a id="eq-117"></a>

$$
\Lambda_{-} = \frac{p / - imc}{2 imc} \tag{117}
$$

and we have

<a id="eq-118"></a>

$$
\Lambda_{+} - \Lambda_{-} = \mathbb{I} \tag{118}
$$

These projection operators are covariant. In <a id="ix-113">Heitler</a> the business is done in a different way which makes them non-covariant and more difficult to handle.

Note that here charge-conjugate <a id="ix-109">wave functions</a> are _not_ used. The positrons of momentum $p$ are represented by the electron wave functions $u$ of momentum $- p$ energy $- E$.

<a id="p24"><sup>[p. 24]</sup></a>

## <a id="s-calculation-of-spurs"></a>Calculation of Spurs

Suppose we have to calculate an expression such as

$$
\tfrac{1}{2} \sum_{I} \sum_{F} ( \overline{u}_{F} Ou_{I} ) ( \overline{u}_{I} Ou_{F} )
$$

summed over <a id="ix-210">electron</a> states only. This gives

$$
\tfrac{1}{2} \sum ( \overline{u}_{F} O \Lambda_{+} O \Lambda_{+} u_{F} ) \epsilon
$$

summed over all four spin states $u_{F}$. To calculate this, let us consider the general expression

$$
\sum_{u} \epsilon ( \overline{u} Qu )
$$

summed over all 4 spin states, where $Q$ is any 4 $\times$4 matrix.

Let $Q$ have the eigenvectors $w_{1} ,w_{2} ,w_{3} ,w_{4}$ with eigenvalues $\lambda_{1} ,\lambda_{2} ,\lambda_{3} ,\lambda_{4}$. Then

$$
Q = \sum_{k = 1}^{4} \lambda_{k} w_{k} w_{k}^{*}
$$

and $4 \quad 4$

$$
\sum_{u} \epsilon ( \overline{u} Qu ) = \sum_{u} \epsilon \sum_{k = 1} \lambda_{k} ( \overline{u} w_{k} ) ( w^{*} u ) = \sum_{k = 1} \lambda_{k} w^{*} \left\{ \sum_{u} \epsilon ( u \overline{u} ) \right\} w_{k}
$$

Hence by (113)

$$
\sum_{u} \epsilon ( \overline{u} Qu ) = \sum \lambda
$$

Now $\sum \lambda$ = sum of diagonal elements of $Q$ = Spur $Q$. Thus<sup><a id="fnref-10" href="#fn-10">10</a></sup>

$$
\sum_{u} \epsilon ( \overline{u} Ou ) = \operatorname{Sp} Q
$$

and this is always easy to calculate.

_Problem 3:_ Given a steady potential $V$ a function of position, and a beam of incident particles, electrons. Solve the Schrödinger <a id="ix-230">equation</a> in the <a id="ix-46">Born approximation</a>

a) By stationary perturbation theory<br>
b) By time-dependent perturbation theory.

Show that the results agree, with a <a id="ix-206">transition</a> probability per unit time given by $w$ = (2$\pi / \hbar ) \rho_{E} | V_{BA} |^{2}$. Evaluate the cross section in the case $V = - Ze^{2} /r$, averaging spin over initial state and summing over final state.

c) Repeat the calculation with particles obeying the <a id="ix-131">Klein-Gordon equation</a>, leaving out the $V^{2}$ term, by either method. Compare the <a id="ix-91">angular</a> <a id="ix-90">distribution</a> in the two cases.

_Problem 4:_ A nucleus (O<sup>16</sup>) has an even<sup><a id="fnref-11" href="#fn-11">11</a></sup> $j$ = 0 ground state and an even $j$ = 0 excited state at 6 MeV. Calculate the total rate of emission of pairs, and the angular and <a id="ix-94">momentum</a> distributions.

_Solution:_ Let ∆$E$ be the excitation energy, $\rho_{N}$ and $\boldsymbol{j}_{N}$ the charge and current density operators of the nucleus. Then for the transition we are interested in $\rho_{N}$ and $\boldsymbol{j}_{N}$ are functions of position $r$ with the time-variation of the single matrix element given by exp$\{- i$∆$E/ \hbar \}$. Also

<a id="eq-119"></a>

$$
\nabla \boldsymbol{\cdot} \boldsymbol{j}_{N} = - \frac{\partial \rho_{H}}{\partial t} = i \frac{\Delta E}{\hbar} \rho_{N} \tag{119}
$$

<a id="p25"><sup>[p. 25]</sup></a>

The electrostatic potential $V$ of the nucleus has the matrix element given by

<a id="eq-120"></a>

$$
\nabla^{2} V = - 4 \pi \rho_{N} \tag{120}
$$

The states being spherically symmetric, $\rho_{N}$ is a function of $r$ only, and so the general solution of <a id="ix-150">Poisson’s equation</a> simplifies to<sup><a id="fnref-12" href="#fn-12">12</a></sup>

<a id="eq-121"></a>

$$
V ( r ) = - \frac{6 \pi}{r} \int_{0}^{r} r_{1}^{2} \rho_{N} ( r_{1} ) dr_{1} \tag{121}
$$

Outside the nucleus $V ( r ) = Ze^{2} /r$ is constant in time, and so the matrix element of $V ( r )$ for this transition is zero. In fact from (119) and (120) we get by integration

<a id="eq-122"></a>

$$
V ( r ) = \frac{\hbar}{i \Delta E} ( - 4 \pi ) ( - r ) j_{No} ( r ) = \frac{4 \pi r \hbar}{i \Delta E} j_{No} ( r ) \tag{122}
$$

where $j_{No}$ is the outward component of the current.

The interaction which creates pairs is then

<a id="eq-123"></a>

$$
I = \int \frac{4 \pi r \hbar}{i \Delta E} j_{No} ( r ) ( - e\psi^{*} \psi ( r ) ) d\tau \tag{123}
$$

As an approximation consider the <a id="ix-44">de Broglie wavelengths</a> of all pairs long compared with the nuclear size. Then

<a id="eq-124"></a>

$$
I = \psi^{*} \psi ( 0 ) \frac{4 \pi \hbar ei}{\Delta E} \int rj_{No} ( r ) d\tau \tag{124}
$$

The constant $\int rj_{No} ( r ) d\tau$ is not known exactly. Suppose as an order of magnitude estimate that the nucleus of charge $Ze$ is uniformly spread over a sphere of radius $r_{o}$ in the ground state, and also in the excited state. Since $\rho_{N}$ is roughly uniform inside the nucleus we have by integrating (119):

<a id="eq-125"></a>

$$
\begin{aligned} j_{N} &= \frac{i \Delta E}{3 \hbar} \overline{r} \rho_{N} , \quad \text{ and thus } \\ I &= \psi^{*} \psi ( 0 ) \left( \frac{- 4 \pi e}{3} \right) \int r^{2} \rho_{N} ( r ) d\tau = \psi^{*} \psi ( 0 ) \left( \frac{- 4 \pi e}{3} \right) Qe^{- i \Delta Et/ \hbar} \end{aligned} \tag{125}
$$

$Q$ is roughly a measure of the charge-moment of inertia of the nucleus, and is equal to

$$
\tfrac{3}{5} Zer_{o}^{2}
$$

Thus

<a id="eq-126"></a>

$$
I = - \frac{4 \pi Ze^{2} r_{o}^{2}}{5} \{ \psi^{*} \psi ( 0 ) \} e^{- i \Delta Et/ \hbar} \tag{126}
$$

So the problem is just to compute the probabilities of <a id="ix-16">pair</a>-emission with this interaction. Note that real radiation is strictly forbidden in a 0–0 transition, and so these pairs are actually observed in the reaction

<a id="eq-127"></a>

$$
p + F_{1 9} \to O_{1 6}^{*} + \alpha \to O_{1 6} + e^{+} + e^{-} + \alpha \tag{127}
$$

Is it correct to take for the interaction just

$$
\int V ( r ) ( - e\psi^{*} \psi ) d\tau
$$

taking the <a id="ix-0">Coulomb potential</a> of the nuclear charge and ignoring all electrodynamic effects? Yes. Because in general the interaction would be

<a id="eq-128"></a>

$$
\int \left\{ \phi ( - e\psi^{*} \psi ) - \sum_{k} A_{k} ( - e\psi^{*} \alpha_{k} \psi ) \right\} d\tau \tag{128}
$$

<a id="p26"><sup>[p. 26]</sup></a>

where $\phi , A_{k}$ are the scalar and vector potentials satisfying the <a id="ix-190">Maxwell</a> <a id="ix-174">equations</a>

$$
\nabla^{2} \phi + \frac{1}{c} \nabla \boldsymbol{\cdot} \frac{\partial \boldsymbol{A}}{\partial t} = - 4 \pi \rho_{N}
$$

$$
\nabla^{2} \boldsymbol{A} - \frac{1}{c^{2}} \frac{\partial^{2} \boldsymbol{A}}{\partial t^{2}} -\nabla \left\{ \nabla \boldsymbol{\cdot} \boldsymbol{A} + \frac{1}{c} \frac{\partial \phi}{\partial t} \right\} = - \frac{4 \pi}{c} \boldsymbol{j}_{N}
$$

The matrix element of the interaction (128) is unchanged by any gauge <a id="ix-100">transformation</a> of the $( \boldsymbol{A} ,\phi )$. Therefore we may take the gauge in which

$$
\nabla \boldsymbol{\cdot} \boldsymbol{A} = 0
$$

Incidentally, since $\phi = V ( r )$, the second Maxwell equation reduces to

$$
\nabla^{2} \boldsymbol{A} - \frac{1}{c^{2}} \frac{\partial^{2} \boldsymbol{A}}{\partial t^{2}} - = - \frac{4 \pi}{c} \boldsymbol{j}_{N}
$$

Now, since there is no free radiation present, also $\nabla \times \boldsymbol{A}$ = 0, and hence $\boldsymbol{A}$ = 0, in this gauge, and therefore we can indeed ignore all electrodynamic effects.

Let us calculate then the probability of <a id="ix-17">pair</a> emission with the interaction (126). A typical final state has an electron of momentum $p_{1}$ and a <a id="ix-153">positron</a> of momentum $p_{2}$, with energies $E_{1} , E_{2}$ and spins $u_{1} , u_{2}$ respectively. For the <a id="ix-9">creation</a> of this pair the matrix element of $I$ is

<a id="eq-128a"></a>

$$
I = - C \overline{u}_{1} \beta u_{2} , \quad C = \frac{4 \pi Ze^{2} r_{o}^{2}}{5} \tag{128a}
$$

The density of final states is by (107)

<a id="eq-129"></a>

$$
\frac{1}{( 2 \pi \hbar )^{6}} \frac{m^{2} c^{4}}{E_{1} E_{2}} p_{1}^{2} dp_{1} d\omega_{1} p_{2}^{2} dp_{2} d\omega_{2} \tag{129}
$$

where $d\omega_{1}$ and $d\omega_{2}$ are the <a id="ix-25">solid</a> angles for $p_{1}$ and $p_{2}$. The creation probability per unit time is thus by (102)

<a id="eq-130"></a>

$$
w = \frac{2 \pi}{\hbar} \frac{\rho_{E}}{dE} | I |^{2} = \frac{2 \pi}{\hbar} \frac{dp_{1} dp_{2}}{d ( E_{1} + E_{2} )} C^{2} \frac{1}{( 2 \pi \hbar )^{6}} \frac{m^{2} c^{4} p_{1}^{2} p_{2}^{2} d\omega_{1} d\omega_{2}}{E_{1} E_{2}} \sum_{u_{1} ,u_{2}} | u_{1} \beta u_{2} |^{2} \tag{130}
$$

Now fixing $p_{1}$,

$$
\frac{dp_{2}}{d ( E_{1} + E_{2} )} = \frac{dp_{2}}{dE_{2}} = \frac{E_{2}}{c^{2} p_{2}}
$$

and

$$
\sum_{u_{1} ,u 2} | u_{1} \beta u_{2} |^{2} = \sum_{u_{1} ,u 2} ( \overline{u}_{1} \beta u_{2} ) ( \overline{u}_{2} \beta u_{1} ) = \operatorname{Spur} \left\{ \beta \frac{p /_{2} - imc}{2 imc} \beta \frac{p /_{1} + imc}{2 imc} \right\}
$$

$$
= - 1 + \frac{\boldsymbol{p}_{1} \boldsymbol{\cdot} \boldsymbol{p}_{2}}{m^{2} c^{2}} + \frac{E_{1} E_{2}}{m^{2} c^{4}} = \frac{E_{1} E_{2} - m^{2} c^{4} + c^{2} p_{1} p_{2} \cos \theta}{m^{2} c^{4}}
$$

where $\theta$ is the <a id="ix-20">angle</a> between the pair. Then writing in (130)

$$
dE_{1} = dp_{1} \frac{c^{2} p_{1}}{E_{1}} , \quad d\omega_{1} = 4 \pi , \quad d\omega_{2} = 2 \pi \sin \theta d\theta
$$

we obtain the <a id="ix-187">differential</a> probability<sup><a id="fnref-13" href="#fn-13">13</a></sup> in $E_{1}$ and $\theta$

<a id="eq-131"></a>

$$
w_{o} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi c^{4} \hbar^{7}} p_{1} p_{2} dE_{1} \left( E_{1} E_{2} - m^{2} c^{4} + c^{2} p_{1} p_{2} \cos \theta \right) \sin \theta d\theta \tag{131}
$$

<a id="p27"><sup>[p. 27]</sup></a>

Since

$$
\Delta E = 6 \text{ MeV } = 1 2 mc^{2}
$$

we can to a good approximation treat all particles as extreme relativistic. Thus

<a id="eq-132"></a>

$$
w_{o} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi c^{6} \hbar^{7}} E_{1}^{2} E_{2}^{2} dE_{1} ( 1 + \cos \theta ) \sin \theta d\theta \tag{132}
$$

So the pairs have an <a id="ix-92">angular</a> distribution concentrated in the _same_ hemisphere, and predominantly _equal_ energies. Then, since

$$
\int_{0}^{\pi} ( 1 + \cos \theta ) \sin \theta d\theta = 2 , \quad \text{ and } \int_{0}^{\Delta E} E_{1}^{2} E_{2}^{2} dE_{1} = \int_{0}^{\Delta E} E_{1}^{2} ( E_{1} + \Delta E )^{2} dE_{1} = \frac{1}{1 5} ( \Delta E )^{5}
$$

the total <a id="ix-10">creation</a> <a id="ix-168">probability</a> per unit time is

<a id="eq-133"></a>

$$
w_{T} = \frac{4 Z^{2} e^{4} r_{o}^{4}}{2 5 \pi \hbar^{7} c^{6}} \frac{1}{1 5} ( \Delta E )^{5} \tag{133}
$$

Numerically

$$
\frac{Ze^{2}}{\hbar c} \approx \frac{1}{1 7} \quad \text{ and } \quad \frac{\Delta Er_{o}}{\hbar c} \approx \frac{1}{1 0} \quad \text{ since } r_{o} = 4 \times 1 0^{- 1 3} \text{ cm }
$$

Hence the lifetime will be

<a id="eq-134"></a>

$$
\tau = 1 5 \times 2 5 \pi \times 1 0^{5} \times 1 7^{2} \times \frac{1}{4} \times \frac{r_{o}}{c} = 1 0^{1 0} \frac{r_{o}}{c} \approx 1 0^{- 1 3} \sec . \tag{134}
$$

## <a id="s-scattering-of-two-electrons-in-born-approximation-the-m-ller-formula"></a>Scattering of Two Electrons in <a id="ix-47">Born Approximation</a>.The <a id="ix-227">Møller</a> Formula.

We calculate now the transition scattering matrix element $M$ between an initial state $A$ consisting of 2 electrons with <a id="ix-204">momenta</a> $p_{1} ,p_{2}$ and spin states $u_{1} ,u_{2}$ and a final state $B$ consisting of two electrons with momenta $p_{1}^{\prime} ,p_{2}^{\prime}$ and spin states $u_{1}^{\prime} ,u_{2}^{\prime}$. Thus $M$ gives the probability amplitude for arriving in state $B$ after a long time when the system is known to be in state $A$ to begin with. Hence $M$ itself should be an _invariant_ relativistically.

We treat the interaction in Born approximation i.e. consider the particles to go directly from the free-particle state $A$ to the free-particle state $B$ by applying the interaction operator once to state $A$. For electrons at reasonably high or relativistic velocities this will be a very good approximation $( e^{2} / \hbar v \ll$ 1). Also we treat the electromagnetic interaction classically, just as in the O<sup>16</sup> problem, taking the field produced by particle 1 according to classical Maxwell <a id="ix-175">equations</a> to act directly on particle 2. This ignores the fact that the field consists of quanta. We shall see later, after we have developed the quantum field theories, that this introduces _no error_ so long as we are in the Born approximation.

For the field produced by particle 1 in a transition from the state $p_{1} ,u_{1}$ to $p_{1}^{\prime} ,u_{1}^{\prime}$ we have the matrix elements $\phi_{( 1 )} , \boldsymbol{A}_{( 1 )}$ say. We use now not the gauge in which $\nabla \boldsymbol{\cdot} \boldsymbol{A}$ = 0, but the covariant gauge in which<sup><a id="fnref-14" href="#fn-14">14</a></sup>

<a id="eq-135"></a>

$$
\sum_{\mu} \frac{\partial A_{\mu}}{\partial x_{\mu}} = 0 \quad A_{4} = i\phi \tag{135}
$$

So using covariant notations we have in this gauge

<a id="eq-136"></a>

$$
\sum_{\nu} \frac{\partial^{2}}{\partial x_{\nu}^{2}} A_{\mu ( 1 )} = + 4 \pi es_{\mu ( 1 )} \quad ( \text{ charge is } - e ) \tag{136}
$$

<a id="eq-137"></a>

$$
s_{\mu ( 1 )} = i ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \exp \left\{ \sum_{\nu} \frac{i}{\hbar} ( p_{1 \nu} - p_{1 \nu}^{\prime} ) x_{\nu} \right\} \tag{137}
$$

<a id="p28"><sup>[p. 28]</sup></a>

whence

<a id="eq-138"></a>

$$
A_{\mu ( 1 )} = - 4 \pi ie \hbar^{2} \left[ \frac{( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \exp \left\{ \sum_{\nu} \frac{i}{\hbar} ( p_{1 \nu} - p_{1 \nu}^{\prime} ) x_{\nu} \right\}}{\sum_{\lambda} ( p_{1 \lambda} - p_{1 \lambda}^{\prime} )^{2}} \right] \tag{138}
$$

where

<a id="eq-139"></a>

$$
\sum_{\nu} ( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2} = | \boldsymbol{p}_{1} - \boldsymbol{p}_{1}^{\prime} |^{2} - \frac{1}{c^{2}} ( E_{1} - E_{1}^{\prime} )^{2} \tag{139}
$$

The effect of the field (138) on particle 2 is given by the interaction term in the Dirac <a id="ix-56">equation</a> for particle 2

<a id="eq-140"></a>

$$
- e\phi + e \boldsymbol{\alpha} \boldsymbol{\cdot} \boldsymbol{A} = ie\beta \sum_{\mu} \gamma_{\mu} A_{\mu} \tag{140}
$$

This gives for particle 2 for the transition from state $p_{2} ,u_{2}$ to $p_{2}^{\prime} ,u_{2}^{\prime}$ a transition matrix element

<a id="eq-141"></a>

$$
\int d\tau \psi_{2}^{\prime *} \left( ie\beta \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} \tag{141}
$$

a 3-dimensional integral over space at the time $t$ say. For the total transition matrix element $M$ by first order perturbation method

<a id="eq-142"></a>

$$
M = - \frac{i}{\hbar} \int_{-\infty}^{\infty} dt \int d\tau \overline{\psi}_{2}^{\prime} \left( ie \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} = - \frac{i}{\hbar c} \int d^{4} x \overline{\psi}_{2}^{\prime} \left( ie \sum_{\mu} \gamma_{\mu} A_{\mu ( 1 )} \right) \psi_{2} \tag{142}
$$

where the 4-fold integral is $dx_{1} dx_{2} dx_{3} dx_{0} ,x_{0} = ct$. Putting in the values of $A_{\mu ( 1 )} ,\psi_{2}^{\prime}$ and $\psi_{2}$, we get

<a id="eq-143"></a>

$$
\begin{aligned} M &= - \frac{4 \pi e^{2} \hbar i}{c} \sum_{\mu} ( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} ) \frac{1}{\sum_{\nu} ( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} \int d^{4} x \exp \left\{ \sum_{\lambda} \frac{i}{\hbar} ( p_{1 \lambda} - p_{1 \lambda}^{\prime} + p_{2 \lambda} - p_{2 \lambda}^{\prime} ) x_{\lambda} \right\} \\ &= - \frac{4 \pi e^{2} \hbar i}{c} \sum_{\mu ,\nu} \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} )}{( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \end{aligned} \tag{143}
$$

where $\delta^{4} ( x ) = \prod_{k = 1}^{4} \delta ( x_{k} )$.

There is also the exchange process in which the particle $p_{1} ,u_{1}$ goes to $p_{2}^{\prime} ,u_{2}^{\prime}$ and _vice-versa_. This gives a contribution to $M$ with a minus sign since the wave function ought to be taken antisymmetric between the two particles. Hence the final result is

<a id="eq-144"></a>

$$
M = - \frac{4 \pi e^{2} \hbar i}{c} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \sum_{\mu ,\nu} \left\{ \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{1} )}{( p_{1 \nu} - p_{1 \nu}^{\prime} )^{2}} - \frac{( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{1} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{2} )}{( p_{1 \nu} - p_{2 \nu}^{\prime} )^{2}} \right\} \tag{144}
$$

This covariant formula is elegant and easy to arrive at. The question now is, how does one go from such a formula to a cross-section?

Generally, suppose in such a 2-particle collision process the transition matrix is

<a id="eq-145"></a>

$$
M = K ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \tag{145}
$$

Then what will be the cross-section in terms of $K$? We do this calculation once here so that later we can stop when we have found formulae for $M$ of the type (145) which come for example conveniently in this form out of radiation theory.

<a id="p29"><sup>[p. 29]</sup></a>

## <a id="s-relation-of-cross-sections-to-transition-amplitudes"></a>Relation of Cross-sections to <a id="ix-207">Transition</a> <a id="ix-12">Amplitudes</a>

Let $w$ be the transition probability per unit volume and per unit time. This is related to the transition probability for a single final state, which is

<a id="eq-146"></a>

$$
w_{s} = c | K |^{2} ( 2 \pi \hbar )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \tag{146}
$$

since in $| M |^{2}$ one of the two (2$\pi \hbar )$<sup>4</sup>$\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) /c$ factors represents merely the volume of space-time in which the interaction can occur. The number of final states is by (107)

<a id="eq-147"></a>

$$
\frac{1}{( 2 \pi \hbar )^{6}} \frac{mc^{2}}{| E_{1}^{\prime} |} \frac{mc^{2}}{| E_{2}^{\prime} |} dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} dp_{2 1}^{\prime} dp_{2 2}^{\prime} dp_{2 3}^{\prime} \tag{147}
$$

Multiplying (146) by (147) gives the total transition probability

<a id="eq-148"></a>

$$
w = | K |^{2} \frac{1}{( 2 \pi \hbar )^{2}} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} c\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} dp_{2 1}^{\prime} dp_{2 2}^{\prime} dp_{2 3}^{\prime} \tag{148}
$$

As $\delta ( ax ) = \frac{1}{a} \delta ( x )$, we have

$$
\delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) = \delta^{3} ( \boldsymbol{p}_{1} + \boldsymbol{p}_{2} - \boldsymbol{p}_{1}^{\prime} - \boldsymbol{p}_{2}^{\prime} ) c\delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} )
$$

and the integration over $dp_{2}$ gives then by the momentum <a id="ix-203">conservation</a>

<a id="eq-148a"></a>

$$
w = | K |^{2} \frac{c^{2}}{( 2 \pi \hbar )^{2}} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} \delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} ) dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime} \tag{148a}
$$

Furthermore,

$$
\text{ if } f ( a ) = 0 , \text{ we have } f ( x ) = f ( a ) + f^{\prime} ( a ) ( x - a ) = f^{\prime} ( a ) ( x - a )
$$

and thus

$$
\delta ( f ( x ) ) = \delta \{ f^{\prime} ( a ) ( x - a ) \} = \frac{\delta ( x - a )}{f^{\prime} ( a )}
$$

Applying this to (148a) with $f ( x ) = f ( p_{1 3}^{\prime} ) = E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime}$ and $a = ( p_{1 3}^{\prime} )_{c}$ = the value of $p_{1 3}^{\prime}$ giving momentum and energy conservation, we get

$$
\delta ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} ) = \frac{1}{d ( E_{1} + E_{2} - E_{1}^{\prime} - E_{2}^{\prime} )} dp_{1 3}^{\prime} \delta \{ p_{1 3}^{\prime} - ( p_{1 3}^{\prime} )_{c} \}
$$

Hence we finally obtain

$$
w = | K |^{2} \frac{m^{2} c^{4}}{E_{1}^{\prime} E_{2}^{\prime}} \frac{c^{2}}{( 2 \pi \hbar )^{2}} \frac{dp_{1 1}^{\prime} dp_{1 2}^{\prime} dp_{1 3}^{\prime}}{d ( E_{1}^{\prime} + E_{2}^{\prime} )}
$$

Choose a Lorentz-<a id="ix-159">system</a> in which $p_{1}$ and $p_{2}$ are both along the $x_{3}$-direction and take $p_{1 1}^{\prime}$ and $p_{1 2}^{\prime}$ as the variables over which the transition probability is taken. This is necessary for relativistic invariance. Then $p_{1 1}^{\prime}$ and $p_{1 2}^{\prime}$ being fixed and having from the momentum conservation $p_{1 3}^{\prime} = - p_{2 3}^{\prime}$ we get

<a id="eq-149"></a>

$$
\frac{d ( E_{1}^{\prime} + E_{1}^{\prime} )}{dp_{1 3}^{\prime}} = \left| \frac{dE_{1}^{\prime}}{dp_{1 3}^{\prime}} - \frac{dE_{2}^{\prime}}{dp_{2 3}^{\prime}} \right| = c^{2} \frac{| E_{2}^{\prime} p_{1 3}^{\prime} - E_{1}^{\prime} p_{2 3}^{\prime} |}{E_{1}^{\prime} E_{2}^{\prime}} \tag{149}
$$

Then the cross-section $\sigma$ is defined in this system by

<a id="eq-150"></a>

$$
\sigma = \frac{wV_{1} V_{2}}{| \boldsymbol{v}_{1} - \boldsymbol{v}_{2} |} \tag{150}
$$

<a id="p30"><sup>[p. 30]</sup></a>

where $V_{1}$ is the normalization volume for particle 1, and $v_{1}$ its velocity. In fact by (103)

<a id="eq-151"></a>

$$
V_{1} = \frac{mc^{2}}{E_{1}} \quad V_{2} = \frac{mc^{2}}{E_{2}} \quad ( \boldsymbol{v}_{1} - \boldsymbol{v}_{2} ) = \frac{c^{2} \boldsymbol{p}_{1}}{E_{1}} - \frac{c^{2} \boldsymbol{p}_{2}}{E_{2}} \tag{151}
$$

Hence the cross-section becomes

<a id="eq-152"></a>

$$
\sigma = \frac{w \left( mc^{2} \right)^{2}}{c^{2} | \boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1} |} = | K |^{2} \frac{\left( mc^{2} \right)^{4}}{c^{2} | E_{2} p_{1 3} - E_{1} p_{2 3} || E_{2}^{\prime} p_{1 3}^{\prime} - E_{1}^{\prime} p_{2 3}^{\prime} |} \frac{1}{( 2 \pi \hbar )^{2}} dp_{1 1}^{\prime} dp_{1 2}^{\prime} \tag{152}
$$

It is worth noting that the factor $\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}$ is invariant under Lorentz <a id="ix-167">transformations</a> leaving the $x_{1}$ and $x_{2}$ components unchanged (e.g.boosts parallel to the $x_{3}$ axis.)<sup><a id="fnref-15" href="#fn-15">15</a></sup> To prove this, we have to show that $p_{1 3} E_{2} - p_{2 3} E_{1} = \tilde{p}_{1 3} \tilde{E}_{2} - \tilde{p}_{2 3} \tilde{E}_{1}$ (where ˜ denotes the quantities after the Lorentz transformation) because we have chosen a Lorentz system in which the direction of the momentum vector is the $x_{3}$ axis. Then

$$
\tilde{E} = E \cosh \theta - cp \sinh \theta
$$

$$
\tilde{p} = p \cosh \theta - \frac{E}{c} \sinh \theta
$$

Since $E^{2} = p^{2} c^{2} + m^{2} c^{4}$, we can write

$$
E = mc^{2} \cosh \varphi \quad pc = mc^{2} \sinh \varphi , \quad \text{ which makes }
$$

$$
\tilde{E} = mc^{2} \cosh ( \varphi - \theta ) \quad \tilde{p} c = mc^{2} \sinh ( \varphi - \theta ) \quad \text{ and thus }
$$

$$
\tilde{E}_{2} \tilde{p}_{1 3} - \tilde{E}_{1} \tilde{p}_{2 3} = m^{2} c^{3} \{ \cosh ( \varphi_{2} - \theta ) \sinh ( \varphi_{1} - \theta ) - \cosh ( \varphi_{1} - \theta ) \sinh ( \varphi_{2} - \theta ) \}
$$

$$
= m^{2} c^{3} \sinh ( \varphi_{1} - \varphi_{2} )
$$

independently of $\theta$. Hence we see that $\sigma$ is invariant under Lorentz transformations parallel to the $x_{3}$ axis.

## <a id="s-results-for-m-ller-scattering"></a>Results for <a id="ix-198">Møller </a><a id="ix-23">Scattering</a>

One electron initially at rest, the other initially with energy $E = \gamma mc^{2}$;

$$
\gamma = \frac{1}{\sqrt{1 - ( v/c )^{2}}}
$$

scattering angle = $\theta$ in the lab system

$= \theta^{*}$ in the center-of-mass system

Then the <a id="ix-29">differential</a> cross-section is (<a id="ix-135">Mott</a> and <a id="ix-170">Massey</a>, _Theory of Atomic Collisions_, 2<sup>nd</sup> ed., p. 368)

<a id="eq-153"></a>

$$
2 \pi \sigma ( \theta ) d\theta = 4 \pi \left( \frac{e^{2}}{mv^{2}} \right)^{2} \left( \frac{\gamma + 1}{\gamma^{2}} \right) dx \left\{ \frac{4}{( 1 - x^{2} )^{2}} - \frac{3}{1 - x^{2}} + \left( \frac{\gamma - 1}{2 \gamma} \right)^{2} \left( 1 + \frac{4}{1 - x^{2}} \right) \right\} \tag{153}
$$

with

$$
x = \cos \theta^{*} = \frac{2 - ( \gamma + 3 ) \sin^{2} \theta}{2 + ( \gamma - 1 ) \sin^{2} \theta}
$$

Without spin you get simply

$$
4 \pi \left( \frac{e^{2}}{mv^{2}} \right)^{2} \left( \frac{\gamma + 1}{\gamma^{2}} \right) dx \left\{ \frac{4}{( 1 - x^{2} )^{2}} - \frac{3}{1 - x^{2}} \right\}
$$

Effect of spin is a measurable _increase_ of scattering over the Mott formula. Effect of exchange is roughly the 1 $- 3 x^{2}$ term. Positron-electron scattering is very similar. Only the exchange effect is different because of <a id="ix-33">annihilation</a> possibility.

<a id="p31"><sup>[p. 31]</sup></a>

## <a id="s-note-on-the-treatment-of-exchange-effects"></a>Note on the Treatment of Exchange Effects

The correctly normalized initial and final states in this problem are

<a id="eq-154"></a>

$$
\frac{1}{\sqrt{2}} \frac{1}{\sqrt{2}} \{ \psi_{1} ( 1 ) \psi_{2} ( 2 ) - \psi_{1} ( 2 ) \psi_{2} ( 1 ) \} \{ \psi_{1}^{\prime} ( 1 ) \psi_{2}^{\prime} ( 2 ) - \psi_{1}^{\prime} ( 2 ) \psi_{2}^{\prime} ( 1 ) \} \tag{154}
$$

where $\psi_{2}$(1) means the particle 2 in the state 1, and so on. With these states the matrix element $M$ is exactly as we have calculated it including the exchange term.

The number of possible final states is only one half of the number of states of two distinguishable particles. But this does not bring a factor $\frac{1}{2}$ into the differential cross-section, because the density of antisymmetrical states, in which one of the two particles has a momentum in a particular range $dp_{1} dp_{2} dp_{3}$ is exactly the same as the density of states of 2 distinguishable particles in which the particle labelled 1 lies in the given range. Hence the general rule: the differential cross-section does _not_ have a factor $\frac{1}{2}$, the total cross section _does_ because each final state may only be counted once in integrating over the <a id="ix-21">angles</a>.

## <a id="s-relativistic-treatment-of-several-particles"></a>Relativistic Treatment of Several Particles

The Møller treatment of the interaction of two electrons succeeds because the <a id="ix-177">field</a> of particle 1 is calculated for all time without taking any account of the effect of particle 2 on particle 1. How can one do a better calculation taking such reactions into account? Clearly we must construct an <a id="ix-57">equation</a> of motion which follows the motions of both particles continuously in time and keeps them in step with each other. So we must have a <a id="ix-88">Dirac</a> equation for 2 electrons, taking exact account of their interaction by including in the equation the behaviour of the Maxwell field too.

This kind of 2-particle Dirac equation is no longer relativistically invariant, if we give each particle a separate position in space but all the same _time_. To avoid this Dirac constructed the _many-time_ theory in which each electron has its own private time coordinate, and satisfies its private Dirac equation. This theory is all right in principle. But it becomes hopelessly complicated when pairs are created and you have equations with new time-coordinates suddenly appearing and disappearing. In fact the whole program of quantizing the electron theory as a theory of discrete particles each with its private time becomes nonsense when you are dealing with an infinite “sea” or an indefinite number of particles. So we have come to the end of what we can do with the relativistic quantum theory of particles.

Where did the theory go wrong? Obviously a lot of the troubles of arose from the fact that a particle was always described by an operator $r$ representing its position at the time $t , t$ being a number and not an operator. This made the interpretation of the formalism _essentially_ non-relativistic even when the equations were formally invariant. In equations like the K. G. and the Dirac equation, the space and time coordinates appear symmetrically. Thus we are led to the following new view-point.

Relativistic quantum theory is the study of quantities $\psi$ which are functions of four coordinates $x_{1} , x_{2}$, $x_{3} , x_{0}$, all the coordinates being c-numbers and only the expressions containing $\psi$ being operators describing the dynamical system.

The dynamical system is specified by the quantity $\psi$ existing at all points of space-time, and so consists of a system of fields. Relativistic quantum theory is necessarily a field theory.

The process of reinterpreting a one-particle wave-function like the Dirac $\psi$ as a quantized field operator is called <a id="ix-208"><em>Second Quantization</em></a>.

---

## Notes

<a id="fn-10"></a>**10.** The _spur_ of a matrix $A$ is the sum of its diagonal elements, and denoted Spur $A$ or Sp $A$. This term is also, and more commonly, known as the _trace_ of a matrix, denoted Tr $A$. [↩](#fnref-10)

<a id="fn-11"></a>**11.** “ever” was written for “even” [↩](#fnref-11)

<a id="fn-12"></a>**12.** This equation differs from the form given in the first edition by a factor of $- \tfrac{6}{\pi}$, but this form also seems incorrect; perhaps the correct relation is [↩](#fnref-12)

$$
V ( r ) = - 4 \pi \int_{0}^{r} \frac{1}{r_{1}^{2}} dr_{1} \int_{0}^{r_{1}} r_{2}^{2} \rho ( r_{2} ) dr_{2}
$$

<a id="fn-13"></a>**13.** In the second edition, the factor “$dE_{1}$” was missing [↩](#fnref-13)

<a id="fn-14"></a>**14.** In the literature, the gauge condition $\nabla \boldsymbol{\cdot} \boldsymbol{A}$ = 0 is now called “Coulomb gauge”; the choice of the gauge condition $\partial_{\mu} A^{\mu}$ = 0 (using the Einstein summation convention) is called “Lorentz gauge”. (See also Eq. (588).) In the first edition, Dyson uses Einstein’s convention; in the second edition, Moravcsik does not. See also the parenthetical remark following Eq. (234a). [↩](#fnref-14)

<a id="fn-15"></a>**15.** Rewritten. In v.1, Dyson writes “The factor $\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}$ is invariant for a Lorentz transformation parallel to the 3 axis.” In v.2, Moravcsik writes “It is worth noting that the factor $\boldsymbol{p}_{1} E_{2} - \boldsymbol{p}_{2} E_{1}$ is invariant under Lorentz transformations leaving the $x_{1}$ and $x_{2}$ components unchanged.” [↩](#fnref-15)

---

<a id="04-field-theory"></a>

# Field Theory

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

The essential feature of this classical theory is that the <a id="ix-2">Action</a> Principle is stated only for variations vanishing on the boundary of Ω. From this one can _deduce_ as in (163) and (165) the effect on $I$(Ω) of variations not vanishing on the boundary. This is possible because each state of motion is defined by fixing _as many_ field quantities as can be fixed independently, (e.g. all the fields on two space-like surfaces or all <a id="p34"><sup>[p. 34]</sup></a> the fields and their time-derivatives on one surface) and then the whole past and future of the motion is determined by the <a id="ix-62">field</a> equations.

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

<a id="04-field-theory--eq-168"></a>

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

Here $H$ represents a History of the fields between $\sigma_{2}$ and $\sigma_{1}$, i.e. any set of classical functions $\varphi^{\alpha} ( x )$ which are defined in the region Ω between $\sigma_{2}$ and $\sigma_{1}$ and which take the values $\varphi_{1}^{\prime \alpha}$ on $\sigma_{1}$ and $\varphi_{2}^{\prime \alpha}$ on $\sigma_{2} . I_{H}$(Ω) is the value of $I$(Ω) calculated with these particular functions. The sum $\sum H$ is taken over all possible histories, a continuously infinite sum whose exact mathematical definition is not easy to formulate. $N$ is a normalization factor independent of the particular states considered, chosen so as to make the sum of the squares of the amplitudes from a given state to all other states equal to 1. This formula is derived by Feynman from very general considerations, applying a Huyghens principle to the solution of wave-mechanics just as it is done in wave-optics. By this one formula the whole theory is quantized and the answer to any <a id="p36"><sup>[p. 36]</sup></a> physical problem in principle given. The method applies not only to field theory but to ordinary NR quantum theory too. We do not try to derive or justify the Feynman formula here. We just show that it gives the same results as the usual QM. For a discussion of the difficulties in defining the sum $\sum H$, and a method of doing it in simple cases, see C. <a id="ix-133">Morette</a>, _Phys. Rev._ **81** (1951) 848.

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

to $L ( x )$ produces at most a change $\epsilon \delta_{\mathcal{Q}} \varphi^{\alpha} ( x )$ in the $\varphi^{\alpha} ( x )$. Let $\epsilon \delta_{\mathcal{R}} \mathcal{Q} ( y )$ be the change in $\mathcal{Q} ( y )$ produced by the addition (191), while $\epsilon \delta_{\mathcal{Q}} \mathcal{R} ( z )$ be the change in $\mathcal{R} ( z )$ produced by (192). Suppose $y$ lies on a surface <a id="p40"><sup>[p. 40]</sup></a> $\sigma$ lying in the future of $z$. Then we take $\mathcal{Q} ( y )$ for $\mathcal{O} ( \sigma )$ in (186), and $\delta L$ given by (191). The $\delta I$(Ω) given by (182) reduces then simply to

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

<a id="05-examples-of-quantized-field-theories"></a>

# Examples of Quantized Field Theories

<a id="p41"><sup>[p. 41]</sup></a>

## <a id="s-i-the-maxwell-field"></a>I. The Maxwell Field

Lagrangian

<a id="05-examples-of-quantized-field-theories--eq-168"></a>

$$
L_{M} = - \tfrac{1}{4} \sum_{\mu ,\nu} \left( \frac{\partial A_{\nu}}{\partial x_{\mu}} - \frac{\partial A_{\mu}}{\partial x_{\nu}} \right)^{2} - \tfrac{1}{2} \sum_{\mu} \left( \frac{\partial A_{\mu}}{\partial x_{\mu}} \right)^{2} \tag{168}
$$

Field equations

<a id="eq-197"></a>

$$
\sum_{\mu} \frac{\partial^{2}}{\partial x_{\mu}^{2}} A_{\lambda} = \Box^{2} A_{\lambda} = 0 \tag{197}
$$

Commutation rules for the $A_{\lambda}$: To find these we uses the Peierls method. Take two points $y$ and $z$ with $z_{0} >y_{0}$. Let $\mathcal{Q} ( y ) = A_{\lambda} ( y ) , \mathcal{R} ( z ) = A_{\mu} ( z )$. Note: in this section $x,y,z,k$ etc. are meant to have components 1, 2, 3 and 0, while in $x_{\mu} ,y_{\mu} ,z_{\mu} ,k_{\mu}$, etc. we mean $\mu$ = 1$, 2 ,$ 3 and 4. When $\delta_{\mathcal{Q}} ( L ) = \epsilon \delta^{4} ( x - y ) A_{\lambda} ( y )$ is added to $L_{M}$, the field <a id="ix-232">equation</a> for $A_{\mu}$ becomes

<a id="eq-198"></a>

$$
\Box^{2} A_{\mu} + \delta_{\lambda \mu} \epsilon \delta^{4} ( x - y ) = 0 \tag{198}
$$

This equation is satisfied by $A_{\mu} + \delta_{\mathcal{Q}} A_{\mu} ( z )$ (by definition), and hence also by $\delta_{\mathcal{Q}} A_{\mu} ( z )$ because of (197). Therefore $\delta_{\mathcal{Q}} A_{\mu} ( z )$ is defined by the conditions

<a id="eq-199"></a>

$$
\begin{aligned} \Box^{2} ( \delta_{\mathcal{Q}} A_{\mu} ( z ) ) &= - \delta_{\lambda \mu} \delta^{4} ( z - y ) \\ \delta_{\mathcal{Q}} A_{\mu} ( z ) &= 0 \text{ for } z_{0} <y_{0} . \end{aligned} \tag{199}
$$

That is to say, $\delta_{\mathcal{Q}} A_{\mu} ( z )$ is a c-number and is the retarded potential created by a point source acting instantaneously at the space-time point $y$.

<a id="eq-200"></a>

$$
\begin{aligned} \delta_{\mathcal{Q}} A_{\mu} ( z ) &= \delta_{\lambda \mu} D_{R} ( z - y ) \\ \Box^{2} D_{R} ( z - y ) &= - \delta^{4} ( z - y ) \end{aligned} \tag{200}
$$

If $x$ is any 4-vector, using

$$
\delta ( x^{2} - a^{2} ) = \frac{1}{2 a} \{ \delta ( x - a ) + \delta ( x + a ) \} , \quad a> 0
$$

we get

<a id="eq-201"></a>

$$
D_{R} ( x ) = \frac{1}{2 \pi} \Theta ( x ) \delta ( x^{2} ) = \frac{1}{4 \pi | \boldsymbol{r} |} \delta ( x_{0} -| \boldsymbol{r} | ) \tag{201}
$$

Here

$$
| \boldsymbol{r} | = \sqrt{x_{1}^{2} + x_{2}^{2} + x_{3}^{2} ;} \quad x_{o} = ct ; \quad x^{2} = r^{2} - x_{o}^{2} \quad \Theta ( x ) = \begin{cases} + 1 & \text{ for } x> 0 \\ 0 & \text{ for } x< 0 \end{cases}
$$

In the same way

<a id="eq-202"></a>

$$
\delta_{\mathcal{R}} A_{\lambda} ( y ) = \delta_{\lambda \mu} D_{A} ( z - y ) = \delta_{\lambda \mu} D_{R} ( y - z ) \tag{202}
$$

<a id="p42"><sup>[p. 42]</sup></a>

where $D_{A}$ is the advanced potential of the same source,

$$
D_{A} ( x ) = \frac{1}{4 \pi | \boldsymbol{r} |} \delta ( x_{0} + | \boldsymbol{r} | )
$$

Hence we have the the commutation rule (194)

<a id="eq-203"></a>

$$
\begin{aligned} \boldsymbol{[} A_{\mu} ( z ) ,A_{\lambda} ( y ) \boldsymbol{]} &= i \hbar c\delta_{\lambda \mu} [ D_{A} ( z - y ) - D_{R} ( z - y ) ] \\ &= i \hbar c\delta_{\lambda \mu} D ( z - y ) \quad ( \text{ definition of } D ) \end{aligned} \tag{203}
$$

This invariant $D$-function satisfies by (200)

<a id="eq-204"></a>

$$
\Box^{2} D ( x ) = - \delta^{4} ( x ) - ( - \delta^{4} ( x ) ) = 0 \tag{204}
$$

as it must. Also

<a id="eq-205"></a>

$$
\begin{aligned} D ( x ) &= \frac{1}{4 \pi | \boldsymbol{r} |} [ \delta ( x_{0} + | \boldsymbol{r} | ) - \delta ( x_{0} -| \boldsymbol{r} | ) ] \\ &= - \frac{1}{2 \pi} \epsilon ( x ) \delta ( x^{2} ) \quad \epsilon ( x ) = \text{ sign } ( x_{0} ) \end{aligned} \tag{205}
$$

### <a id="s-momentum-representations"></a>Momentum Representations

We have

<a id="eq-206"></a>

$$
\delta^{4} ( x ) = \frac{1}{( 2 \pi )^{4}} \int \exp ( ik \cdot x ) d^{4} k \tag{206}
$$

where the integral is fourfold, over $dk_{1} dk_{2} dk_{3} dk_{4}$. Therefore

<a id="eq-207"></a>

$$
D_{R} ( x ) = \frac{1}{( 2 \pi )^{4}} \int_{+} \exp ( ik \cdot x ) \frac{1}{k^{2}} d^{4} k \tag{207}
$$

where $k^{2} = | \boldsymbol{k} |^{2} - k_{0}^{2}$. The integration with respect to $k_{1} k_{2} k_{3}$ is an ordinary real integral. That with respect to $k_{0}$ is a contour integral going along the real axis and _above_ the two poles at $k_{0} = \pm | \boldsymbol{k} |$.

![Figure 1 (p. 42)](images/fig-p048-1.png)

_Figure 1 (p. 42)_

For detailed calculations, see the Appendix below. This gives the correct behaviour of $D_{R}$ being zero for $x_{0} <$ 0. Similarly

<a id="eq-208"></a>

$$
D_{A} ( x ) = \frac{1}{( 2 \pi )^{4}} \int_{-} \exp ( ik \cdot x ) \frac{1}{k^{2}} d^{4} k \tag{208}
$$

with a contour going _below_ both the poles. Therefore

<a id="eq-209"></a>

$$
D ( x ) = \frac{1}{( 2 \pi )^{4}} \int_{s} \exp ( ik \cdot x ) \frac{1}{k^{2}} d^{4} k \tag{209}
$$

with a contour $s$ as shown.

![Figure 2 (p. 42)](images/fig-p048-2.png)

_Figure 2 (p. 42)_

<a id="p43"><sup>[p. 43]</sup></a>

Evaluating the residues we have

<a id="eq-210"></a>

$$
D ( x ) = - \frac{i}{( 2 \pi )^{3}} \int \exp ( ik \cdot x ) \delta ( k^{2} ) \epsilon ( k ) d^{4} k \tag{210}
$$

this being now an ordinary real integral.

## <a id="s-appendix"></a>Appendix

![Figure 3 (p. 43)](images/fig-p049-1.png)

_Figure 3 (p. 43)_

Let us verify, e.g. (207). For $x_{0} <$ 0, we have to take the top path shown,<br>
otherwise the integrand blows up; this gives obviously 0.

For $x_{0} >$ 0, we have to take the bottom path; then

$$
D_{R} ( x ) = \frac{1}{( 2 \pi )^{4}} \int_{+} \frac{e^{i \boldsymbol{k} \cdot \boldsymbol{x}} e^{- ik_{0} x_{0}}}{\boldsymbol{k}^{2} - k_{0}^{2}} d^{3} kdk_{0}
$$

where $\boldsymbol{k}$ and $\boldsymbol{x}$ are 3 dimensional vectors.

Now because of the clockwise direction

$$
\int_{+} \frac{e^{- ik_{0} x_{0}}}{\boldsymbol{k}^{2} - k_{0}^{2}} dk_{0} = - \int_{+} \frac{e^{- ik_{0} x_{0}}}{( k_{0} -| \boldsymbol{k} | ) ( k_{0} + | \boldsymbol{k} | )} dk_{0}
$$

$$
= 2 \pi i ( \text{ Residue at } k_{0} = | \boldsymbol{k} | + \text{ Residue at } k_{0} = -| \boldsymbol{k} | ) = 2 \pi i \left( \frac{e^{- i | \boldsymbol{k} | x_{0}}}{2 | \boldsymbol{k} |} - \frac{e^{+ i | \boldsymbol{k} | x_{0}}}{2 | \boldsymbol{k} |} \right)
$$

Hence

$$
D_{R} ( x ) = \frac{i}{( 2 \pi )^{3}} \int \frac{1}{2 | \boldsymbol{k} |} e^{i \boldsymbol{k} \cdot \boldsymbol{x}} \left\{ e^{i | \boldsymbol{k} | x_{0}} - e^{- i | \boldsymbol{k} | x_{0}} \right\} d^{3} k
$$

$$
= \frac{i}{( 2 \pi )^{3}} 2 \pi \int \int \frac{1}{2 | \boldsymbol{k} |} e^{i | \boldsymbol{k} || \boldsymbol{x} | \cos \theta} \left\{ e^{i | \boldsymbol{k} | x_{0}} - e^{- i | \boldsymbol{k} | x_{0}} \right\} | \boldsymbol{k} |^{2} d | \boldsymbol{k} | \sin \theta d\theta
$$

$$
= - \frac{i}{( 2 \pi )^{2}} \int_{0}^{\infty} \frac{1}{2 | \boldsymbol{k} |} \frac{1}{i | \boldsymbol{k} || \boldsymbol{x} |} \left\{ e^{i | \boldsymbol{k} | x_{0}} - e^{- i | \boldsymbol{k} | x_{0}} \right\} | \boldsymbol{k} |^{2} \left( e^{i | \boldsymbol{k} || \boldsymbol{x} | \alpha} \begin{vmatrix} \alpha = + 1 \\ \alpha = - 1 \end{vmatrix} d | \boldsymbol{k} | \right.
$$

$$
= - \frac{1}{4 \pi^{2}} \frac{1}{2 | \boldsymbol{x} |} \int_{0}^{\infty} \left\{ e^{i | \boldsymbol{k} | ( x_{0} + | \boldsymbol{x} | )} - e^{i | \boldsymbol{k} | ( x_{0} -| \boldsymbol{x} | )} - e^{- i | \boldsymbol{k} | ( x_{0} -| \boldsymbol{x} | )} + e^{- i | \boldsymbol{k} | ( x_{0} + | \boldsymbol{x} | )} \right\} d | \boldsymbol{k} |
$$

$$
= - \frac{1}{4 \pi | \boldsymbol{x} |} \frac{1}{2 \pi} \int_{-\infty}^{+ \infty} \left\{ e^{i | \boldsymbol{k} | ( x_{0} + | \boldsymbol{x} | )} - e^{i | \boldsymbol{k} | ( x_{0} -| \boldsymbol{x} | )} \right\} d | \boldsymbol{k} |
$$

$$
= - \frac{1}{4 \pi | \boldsymbol{x} |} \delta ( x_{0} -| \boldsymbol{x} | ) \quad \text{ for } x_{0} > 0 .
$$

### <a id="s-fourier-analysis-of-operators"></a>Fourier Analysis of Operators

Let us analyze the potential $A_{\mu}$ into Fourier components

<a id="eq-211"></a>

$$
A_{\mu} ( x ) = B \int d^{3} k | \boldsymbol{k} |^{- 1 / 2} \{ a_{k\mu} \exp ( ik \cdot x ) + \tilde{a}_{k\mu} \exp ( - ik \cdot x ) \} \tag{211}
$$

where the factor $| \boldsymbol{k} |^{- 1 / 2}$ appears only as a matter of convenience; the actual Fourier coefficients are then $| \boldsymbol{k} |^{- 1 / 2} a_{k\mu}$ and $| \boldsymbol{k} |^{- 1 / 2} \tilde{a}_{k\mu}$. The integration is over all 4-vectors $( k )$ with $k_{0}$ = +$| \boldsymbol{k} | . B$ is a normalization factor to be determined later. The $a_{k\mu}$ and $\tilde{a}_{k\mu}$ are operators independent of $x$.

Since $A_{1} ,A_{2} ,A_{3}$ and $A_{0}$ are Hermitian,

<div align="center">

$\tilde{a}_{k\mu} = a_{k\mu}^{*}$ = Hermitian conjugate of $a_{k\mu} , \quad \mu$ = 1$, 2 , 3 ,$ 0 and therefore

</div>

<a id="eq-212"></a>

$$
\tilde{a}_{k 4} = - a_{k 4}^{*} = - \text{ Hermitian conjugate of } a_{k 4} \tag{212}
$$

<a id="p44"><sup>[p. 44]</sup></a>

Computing the commutator $\boldsymbol{[} A_{\mu} ( z ) ,A_{\lambda} ( y ) \boldsymbol{]}$ from (211) and comparing the result with (203) in the momentum representation (210), we have first, since the result is a function of $( z - y )$ only<sup><a id="fnref-22" href="#fn-22">22</a></sup>

<a id="eq-213"></a>

$$
\begin{aligned} \boldsymbol{[} a_{k\mu} ,a_{k^{\prime} \lambda} \boldsymbol{]} &= 0 \\ \boldsymbol{[} \tilde{a}_{k\mu} , \tilde{a}_{k^{\prime} \lambda} \boldsymbol{]} &= 0 \\ \boldsymbol{[} a_{k\mu} , \tilde{a}_{k^{\prime} \lambda} \boldsymbol{]} &= \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{\mu \lambda} \end{aligned} \tag{213}
$$

And the two results for the commutator agree then precisely if we take

<a id="eq-214"></a>

$$
B = \sqrt{\frac{\hbar c}{1 6 \pi^{3}}} \tag{214}
$$

### <a id="s-emission-and-absorption-operators"></a>Emission and Absorption Operators

The operators $A_{\mu} ( x )$ obey the Heisenberg equations of motion for operators (190)

<a id="eq-190a"></a>

$$
i \hbar \frac{\partial A_{\mu}}{\partial t} = \boldsymbol{[} A_{\mu} ,H \boldsymbol{]} \tag{190a}
$$

Therefore the operator $a_{k\mu}$ exp$( ik \cdot x )$ has matrix elements between an initial state of energy $E_{1}$ and a final state of energy $E_{2}$, only if

<a id="eq-215"></a>

$$
i \hbar ( - ick_{0} ) = E_{1} - E_{2} = \hbar c | \boldsymbol{k} | \tag{215}
$$

because by (211) and (190a) we have

<a id="eq-216"></a>

$$
\psi_{1} i \hbar ( - ick_{0} ) a_{k\mu} \psi_{2} = \psi_{1} \hbar c | \boldsymbol{k} | a_{k\mu} \psi_{2} = \psi_{1} \boldsymbol{[} a_{\mu k} ,H \boldsymbol{]} \psi_{2} = ( E_{1} - E_{2} ) \psi_{1} a_{\mu k} \psi_{2} \tag{216}
$$

Now $\hbar c | \boldsymbol{k} |$ is a constant energy, characteristic of the frequency $\omega = ck$ characteristic of the particular Fourier <a id="ix-95">components</a> of the field. The operator $a_{k\mu}$ can only operate so as to _reduce_ the energy of a system by a lump of energy of this size. In the same way, $\tilde{a}_{k\mu}$ will only operate when

$$
E_{1} - E_{2} = - \hbar c | \boldsymbol{k} |
$$

to _increase_ the energy by the same amount.

This is the fundamental property of the quantized field operators, that they change the energy of a system not continuously but in jumps. This shows that our formalism includes correctly the experimentally known quantum behaviour of radiation.

We call $a_{k\mu}$ the _absorption_ operator for the field oscillator with propagation vector $k$ and polarization direction $\mu$. Likewise $\tilde{a}_{k\mu}$ the _emission_ operator.

We have thus 4 directions of polarization for a photon of given momentum. There are not all observed in electromagnetic radiation. Free radiation can only consist of transverse waves, and has only 2 possible polarizations. This is because the physically allowable states Ψ are restricted by a supplementary condition

<a id="eq-217"></a>

$$
\sum_{\mu} \frac{\partial A_{\mu}^{( + )}}{\partial x_{\mu}} \Psi = 0 \tag{217}
$$

where $A_{\mu}^{( + )}$ is the positive frequency part of $A_{\mu}$, i.e. the part containing the absorption operators. In the classical theory we have

$$
\sum_{\mu} \frac{\partial A_{\mu}}{\partial x_{\mu}} = 0
$$

the condition imposed in order to simplify the Maxwell equations to the simple form $\Box^{2} A_{\mu}$ = 0. In the quantum theory it was usual to take

$$
\sum_{\mu} \frac{\partial A_{\mu}}{\partial x_{\mu}} \Psi = 0
$$

<a id="p45"><sup>[p. 45]</sup></a>

but this means that photons of a certain kind cannot be _emitted_ in a physical state which is physically difficult to understand and brings mathematical inconsistencies into the theory. So we assume only (216) which only says these photons are _not present_ and cannot be absorbed from a physical state, which makes good sense. Also in the classical limit $\sum_{\mu} \partial A_{\mu} /\partial x_{\mu}$ is a real quantity, and so $\sum_{\mu} \partial A_{\mu} /\partial x_{\mu}$ = 0 follows correctly from (216) alone.

The method of using (216) as supplementary condition is due to Gupta and Bleuler<sup><a id="fnref-23" href="#fn-23">23</a></sup>;

S. N. Gupta, _Proc. Roy. Soc. A_ **63** (1950) 681.

K. Bleuler, _Helv. Phys. Acta_ **23** (1950) 567.

The older treatment is unnecessary and difficult, so we will not bother about it.

By (211), (216) is equivalent to assuming

<a id="eq-216a"></a>

$$
\sum_{\mu} ( k_{\mu} a_{k\mu} ) \Psi = 0 \tag{216a}
$$

for each momentum vector $k$ of a photon.

As a result of this work of Gupta and Bleuler, the supplementary conditions do not come into the practical use of the theory at all. We use the theory and get correct results, forgetting about the supplementary conditions.

### <a id="s-gauge-invariance-of-the-theory"></a>Gauge-Invariance of the Theory

The theory is gauge-invariant. That is to say, adding a gradient<sup><a id="fnref-24" href="#fn-24">24</a></sup> Λ${}_{\mu} = \partial$Λ$/\partial x_{\mu}$ to the potentials does not change the fields by anything which is physically observable. Therefore all states which differ only by such an addition to the potentials are physically identical.

If Ψ is any state then

$$
\Psi^{\prime} = \left( 1 + \lambda \sum_{\mu} k_{\mu} \tilde{a}_{k\mu} \right) \Psi
$$

is a state obtained from Ψ by emitting a pseudo-photon with potentials proportional to $\partial$Λ$/\partial x_{\mu}$. Hence Ψ$\prime$ should be indistinguishable from Ψ.

Now if Ψ$2$ is any state whatever satisfying the supplementary condition (216a) the matrix element

<a id="eq-216b"></a>

$$
\begin{aligned} ( \Psi^{\prime *} , \Psi_{1} ) &= \left( \Psi^{*} \left( 1 + \lambda \sum_{\mu} k_{\mu}^{*} \tilde{a}_{k\mu}^{*} \right) , \Psi_{2} \right) \\ &= \left( \Psi^{*} , \left( 1 + \lambda \sum_{\mu} k_{\mu} a_{k\mu}^{*} \right) \Psi_{2} \right) = ( \Psi^{*} , \Psi_{2} ) \end{aligned} \tag{216b}
$$

Hence the matrix elements of Ψ$\prime$ and Ψ to any physical state Ψ$2$ whatever are equal, and so the results of the theory are all independent of whether the state Ψ is represented by the vector Ψ or by Ψ$\prime$. This is enough to show that the theory is properly gauge-invariant, in spite of the fact that states are specified by the potentials which are not themselves gauge-invariant.

### <a id="s-the-vacuum-state"></a>The Vacuum State

The vacuum state is by definition the state of lowest energy, so that all absorption operators operating on it give zero:

<a id="eq-217a"></a>

$$
a_{k\mu} \Psi_{o} = 0 \tag{217a}
$$

and therefore by (212)

<a id="eq-217b"></a>

$$
( a_{k\mu} \Psi_{o} )^{*} = ( \Psi_{o}^{*} a_{k\mu}^{*} ) = \pm ( \Psi_{o}^{*} \tilde{a}_{k\mu} ) = 0 \tag{217b}
$$

Given any operator $\mathcal{Q}$, we are interested in the “vacuum expectation value” of $\mathcal{Q}$ defined as

<a id="eq-218"></a>

$$
\langle Q \rangle_{o} = ( \Psi_{o}^{*} , \mathcal{Q} \Psi_{o} ) \tag{218}
$$

<a id="p46"><sup>[p. 46]</sup></a>

Then we have at once

<a id="eq-219a"></a>

$$
\langle a_{k\mu} a_{k^{\prime} \lambda} \rangle_{o} = 0 \quad \text{ by } ( 2 1 7 a ) \tag{219a}
$$

<a id="eq-219b"></a>

$$
\langle \tilde{a}_{k\mu} \tilde{a}_{k^{\prime} \lambda} \rangle_{o} = 0 \quad \text{ by } ( 2 1 7 b ) \tag{219b}
$$

<a id="eq-219c"></a>

$$
\langle \tilde{a}_{k\mu} a_{k^{\prime} \lambda} \rangle_{o} = 0 \quad \text{ by } ( 2 1 7 a , b ) \tag{219c}
$$

And by the commutation laws (213) and (219c) we have

<a id="eq-220"></a>

$$
\langle a_{k\mu} \tilde{a}_{k^{\prime} \lambda} \rangle_{o} = \langle \boldsymbol{[} a_{k\mu} , \tilde{a}_{k^{\prime} \lambda} \boldsymbol{]} \rangle_{o} = \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{\mu \lambda} \tag{220}
$$

The vacuum expectation value $\langle A_{\mu} ( z ) A_{\lambda} ( y ) \rangle_{o}$ is thus just the part of the commutator $\boldsymbol{[} A_{\mu} ( z ) ,A_{\lambda} ( y ) \boldsymbol{]}$ which contains positive frequencies exp$\{ ik \cdot ( z - y ) \}$ with $k_{0} >$ 0, as one can see using (211), (219) and (220). Thus<sup><a id="fnref-25" href="#fn-25">25</a></sup>

<a id="eq-221"></a>

$$
\langle A_{\mu} ( z ) A_{\lambda} ( y ) \rangle_{o} = i \hbar c\delta_{\mu \lambda} D^{+} ( z - y ) \tag{221}
$$

<a id="eq-222"></a>

$$
D^{+} ( x ) = - \frac{i}{( 2 \pi )^{3}} \int_{k_{0} > 0} \exp ( ik \cdot x ) \delta ( k^{2} ) \Theta ( k ) d^{4} k \tag{222}
$$

We write

<a id="eq-223"></a>

$$
D ( x ) = D^{+} ( x ) + D^{-} ( x ) \tag{223}
$$

<a id="eq-224"></a>

$$
D^{+} = \tfrac{1}{2} \left( D - iD^{( 1 )} \right) \quad D^{-} = \tfrac{1}{2} \left( D + iD^{( 1 )} \right) \tag{224}
$$

The even function $D^{( 1 )}$ is then defined by

<a id="eq-225"></a>

$$
\langle A_{\mu} ( z ) A_{\lambda} ( y ) + A_{\lambda} ( y ) A_{\mu} ( z ) \rangle_{o} = \hbar c\delta_{\mu \lambda} D^{( 1 )} ( z - y ) \tag{225}
$$

<a id="eq-226"></a>

$$
D^{( 1 )} ( x ) = \frac{1}{( 2 \pi )^{3}} \int \exp ( ik \cdot x ) \delta ( k^{2} ) d^{4} k \tag{226}
$$

It is then not hard to prove (see the Appendix below) that

<a id="eq-227"></a>

$$
D^{( 1 )} ( x ) = \frac{1}{2 \pi^{2} x^{2}} \tag{227}
$$

The functions $D$ and $D^{( 1 )}$ are the two independent solutions of $\Box^{2} D$ = 0, one odd and the other even. Then we define the function

<a id="eq-228"></a>

$$
\begin{aligned} \overline{D} ( x ) &= - \tfrac{1}{2} \epsilon ( x ) D ( x ) = \tfrac{1}{2} ( D_{R} ( x ) + D_{A} ( x ) ) = \frac{1}{4 \pi} \delta ( x^{2} ) \\ &= \frac{1}{( 2 \pi )^{4}} \int \exp ( ik \cdot x ) \frac{1}{k^{2}} d^{4} k \end{aligned} \tag{228}
$$

the last being a real principal value integral: This is the even solution of the point-source equation

<a id="eq-229"></a>

$$
\Box^{2} \overline{D} ( x ) = - \delta^{4} ( x ) \tag{229}
$$

## <a id="s-appendix-2"></a>Appendix

$$
D^{( 1 )} ( x ) = \frac{1}{( 2 \pi )^{3}} \int e^{ik \cdot x} \delta ( k^{2} ) d^{4} k
$$

$$
= - \frac{1}{( 2 \pi )^{2}} \int_{- 1}^{+ 1} d\mu \int_{-\infty}^{\infty} dk_{0} \int_{0}^{\infty} d | \boldsymbol{k} | e^{i | \boldsymbol{k} || \boldsymbol{x} | \mu} e^{- ik_{0} x_{0}} \frac{\{ \delta ( k_{0} -| \boldsymbol{k} | ) + \delta ( k_{0} + | \boldsymbol{k} | ) \}| \boldsymbol{k} |^{2}}{2 | \boldsymbol{k} |}
$$

$$
= \frac{1}{( 2 \pi )^{2}} \int_{0}^{\infty} d | \boldsymbol{k} | \frac{1}{i | \boldsymbol{k} || \boldsymbol{x} |} \left\{ e^{i | \boldsymbol{k} || \boldsymbol{x} |} - e^{- i | \boldsymbol{k} || \boldsymbol{x} |} \right\} \frac{1}{2 | \boldsymbol{k} |} \left\{ e^{- i | \boldsymbol{k} | x_{0}} + e^{i | \boldsymbol{k} | x_{0}} \right\} | \boldsymbol{k} |^{2}
$$

$$
= \frac{1}{2 \pi^{2}} \frac{1}{| \boldsymbol{x} |} \int_{0}^{\infty} \sin ( | \boldsymbol{k} || \boldsymbol{x} | ) \cos ( | \boldsymbol{k} | x_{0} ) d | \boldsymbol{k} | = \frac{1}{2 \pi^{2}} \frac{1}{2 | \boldsymbol{x} |} \int_{0}^{\infty} d | \boldsymbol{k} |\{ \sin ( ( | \boldsymbol{x} | + x_{0} ) | \boldsymbol{k} | ) + \sin ( ( | \boldsymbol{x} |- x_{0} ) | \boldsymbol{k} | ) \}
$$

<a id="p47"><sup>[p. 47]</sup></a>

Taking the integral in the Abelian sense

$$
\lim \int_{\epsilon \to 0 \quad 0}^{\infty} e^{- \epsilon x} \sin axdx = \lim_{\epsilon \to 0} \frac{a}{\epsilon^{2} + a^{2}} = \frac{1}{a}
$$

hence in our case<sup><a id="fnref-26" href="#fn-26">26</a></sup>

$$
D^{( 1 )} ( x ) = \frac{1}{2 \pi^{2}} \frac{1}{2 | \boldsymbol{x} |} \left\{ \frac{1}{| \boldsymbol{x} | + x_{0}} + \frac{1}{| \boldsymbol{x} |- x_{0}} \right\} = \frac{1}{2 \pi^{2} x^{2}} .
$$

### <a id="s-the-gupta-bleuler-method"></a>The Gupta-Bleuler Method

There is one difficulty in the preceding theory. We assume according to (220)

<a id="eq-220a"></a>

$$
\langle a_{k\mu} a_{k^{\prime} \lambda}^{*} \rangle_{o} = \pm \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{\mu \lambda} \tag{220a}
$$

Here the plus sign holds for $\mu$ = 1$, 2 ,$3; the minus sign for $\mu$ = 4. Now if the operators $a_{k\mu} ,a_{k\mu}^{*}$ are represented in the usual way by matrices, as it is done in the elementary theory of the harmonic oscillator$( )$ (see Wentzel p. 33, Eq. (6.16)), the vacuum expectation values of a product $( a_{k\mu} a_{k\mu}^{*} )$ will always be _positive_, i.e. the plus sign should hold in (220a) for $\mu$ = 4 also. In fact $( a_{k\mu} a_{k\mu}^{*} )$ will have a positive expectation value in any state whatever, if the photon oscillators are treated as ordinary$( )$ elementary oscillators.

Therefore we must distinguish between the scalar product (Ψ$\frac{*}{1} ,$ Ψ${}_{2} )$ as we have defined it by our covariant theory, and the scalar product (Ψ$\frac{*}{1} ,$ Ψ${}_{2} )_{E}$ which one would calculate using the explicit matrix representation of the operators. The product (Ψ$\frac{*}{1} ,$ Ψ${}_{2} )_{E}$ has no physical significance, because the matrix representations of the $a_{k 4}$ refer to states with photons polarized purely in the time dimension, which cannot occur physically. However, it is convenient also to be able to use the matrix representations in practice.

To use the matrix representations, we have only to define an operator $\eta$ by the condition

<a id="eq-220b"></a>

$$
\eta \Phi = ( - 1 ) \Phi \tag{220b}
$$

where Φ is any state in which there is a definite number $N$ of photons polarized in the 4-direction. Then the physical scalar product is given in terms of the explicit matrix representations by

<a id="eq-220c"></a>

$$
( \Psi_{1}^{*} , \Psi_{2} ) = ( \Psi_{1}^{*} ,\eta \Psi_{2} )_{E} \tag{220c}
$$

The definition (220c), introduced by Gupta, makes the matrix representations consistent with all the requirements of the covariant theory, in particular also it gives (220) correctly. The physical scalar product is thus an _indefinite metric_ regarded from the point of view of the matrix representations. However, we have seen in (216b) that for any _physical_ states the scalar product (Ψ$\frac{*}{1} ,$ Ψ${}_{2} )$ is equal to (Ψ$\frac{*}{1 T} ,$ Ψ${}_{1 T} )$ where Ψ$1 T$ is a state involving transverse photons only, and hence is positive. Thus for physical states the metric is definite and this is all that we require of it.

### <a id="s-example-spontaneous-emission-of-radiation"></a>Example: Spontaneous Emission of Radiation

This is a purely quantum-mechanical effect. A classical treatment, considering the reaction of the atom to a classical applied Maxwell field, gives a correct account of the absorption of radiation and of stimulated emission, but fails to give the spontaneous emission.

Let an atom have 2 states, the ground state 1 and an excited state 2 with energy $\hbar cq$. For the transition 2 $\to$ 1 let the charge-current density of the atom have the unintegrated matrix elements

$$
j_{\mu A} ( x ) = j_{\mu A} ( r,t ) \quad \text{ at the point } x = ( r,t )
$$

The interaction with the Maxwell field has matrix element<sup><a id="fnref-27" href="#fn-27">27</a></sup>

<a id="eq-230"></a>

$$
I = - \frac{1}{c} \int \sum_{\mu} j_{\mu A} ( r,t ) \langle A_{\mu} ( r,t ) \rangle_{\text{ emit }} d^{3} \boldsymbol{r} \tag{230}
$$

<a id="p48"><sup>[p. 48]</sup></a>

for making a transition with emission of a photon. The total emission probability per unit time is obtained using time dependent perturbation theory:<sup><a id="fnref-28" href="#fn-28">28</a></sup>

<a id="eq-231"></a>

$$
\begin{aligned} w &= \frac{1}{T} \sum | a_{1} ( T ) |^{2} = \frac{1}{T} \sum_{\text{ photon }} \left| \frac{1}{i \hbar} \int \left\{ - \frac{1}{c} \int \sum_{\mu} j_{\mu A} ( r,t ) \langle A_{\mu} ( r,t ) \rangle_{\text{ emit }} d^{3} \boldsymbol{r} \right\} dt \right|^{2} \\ &= \frac{1}{Tc^{4} \hbar^{2}} \int \int \sum_{\lambda ,\mu} j_{\lambda A}^{*} ( x^{\text{ states } \prime} ) j_{\mu A} ( x ) \langle A_{\lambda}^{*} ( x^{\prime} ) A_{\mu} ( x ) \rangle_{o} d^{4} xd^{4} x^{\prime} \end{aligned} \tag{231}
$$

the integral being over all space for a long time $T$, the sum over the physical photon states only. It is _not_ correct to take in (231) the photon states to be the 4 states with polarization in the directions $\mu$ = 1$, 2 , 3 , 4 ,$ because these are not physical states.

Using a sum-rule to sum over the states

$$
w = \frac{1}{Tc^{4} \hbar^{2}} \int \int \sum_{\lambda ,\mu} j_{\lambda A}^{*} ( x^{\prime} ) j_{\mu A} ( x ) \langle A_{\lambda}^{*} ( x^{\prime} ) A_{\mu} ( x ) \rangle_{o} d^{4} xd^{4} x^{\prime}
$$

Write $\tilde{j}_{\lambda A} ( x^{\prime} )$ for the matrix element<sup><a id="fnref-29" href="#fn-29">29</a></sup> of $j_{\lambda A} ( x^{\prime} )$ in the reverse transition 1 $\to$ 2. Then

$$
j_{\lambda A}^{*} ( x^{\prime} ) = \eta_{\lambda} \tilde{j}_{\lambda A} ( x^{\prime} ) , \quad A_{\lambda}^{*} ( x^{\prime} ) = \eta_{\lambda} A_{\lambda} ( x^{\prime} )
$$

with $\eta_{\lambda}$ = +1$,\lambda$ = 1$, 2 , 3 ; \eta_{4} = - 1 .$

Hence

<a id="eq-232"></a>

$$
\begin{aligned} w &= \frac{1}{Tc^{4} \hbar^{2}} \int \int \sum_{\lambda ,\mu = 1}^{4} j_{\lambda A}^{*} ( x^{\prime} ) j_{\mu A} ( x ) \langle A_{\lambda}^{*} ( x^{\prime} ) A_{\mu} ( x ) \rangle_{o} d^{4} xd^{4} x^{\prime} \\ &= \frac{\hbar c}{( 2 \pi )^{3}} \int d^{4} k\delta ( k^{2} ) \Theta ( k_{0} ) \sum_{\mu = 1}^{4} j_{\mu A} ( k ) \tilde{j}_{\mu A} ( k ) \frac{1}{Tc^{4} \hbar^{2}} \int_{0}^{cT} \int_{0}^{cT} dx_{0} dx_{0}^{\prime} \exp \{ i ( x - x_{0} ) ( q - k_{o} ) \} \\ &= \frac{1}{( 2 \pi )^{2} \hbar c^{2}} \sum_{\mu = 1}^{4} \int d^{3} \boldsymbol{k} \delta ( | \boldsymbol{k} |^{2} - q^{2} ) j_{\mu A} ( \boldsymbol{k} ) \tilde{j}_{\mu A} ( \boldsymbol{k} ) \end{aligned} \tag{232}
$$

since

$$
\frac{1}{cT} \int_{0}^{cT cT} \int_{0} dx_{0} dx_{0}^{\prime} \exp \{ i ( x - x_{0} ) ( q - k_{o} ) \} = cT \frac{\sin^{2} \tfrac{cT}{2} ( q - k_{0} )}{\left( \tfrac{cT}{2} ( q - k_{0} ) \right)^{2}} \to \pi \delta ( q - k_{0} ) \text{ if } cT \to \infty
$$

where<sup><a id="fnref-30" href="#fn-30">30</a></sup>

$$
j_{\mu A} ( \boldsymbol{k} ) = \int j_{\mu A} ( r ) e^{- i \boldsymbol{k} \cdot \boldsymbol{r}} d^{3} \boldsymbol{r} \quad \tilde{j}_{\mu A} ( \boldsymbol{k} ) = \int \tilde{j}_{\mu A} ( r ) e^{i \boldsymbol{k} \cdot \boldsymbol{r}} d^{3} \boldsymbol{r}
$$

By the charge conservation law

$$
\sum_{\mu} k_{\mu} j_{\mu A} ( k ) = 0 \sum_{\mu} k_{\mu} \tilde{j}_{\mu A} ( k ) = 0
$$

and so

<a id="eq-232a"></a>

$$
\begin{aligned} \sum_{\mu} j_{\mu A} ( k ) \tilde{j}_{\mu A} ( k ) &= \frac{1}{q^{2}} qj_{4 A} q \tilde{j}_{4 A} + \sum_{i}^{3} | j_{iA} ( k ) |^{2} = - \frac{1}{q^{2}} \sum_{i,\ell = 1}^{3} \left\{ k_{i} j_{iA} ( k ) k_{\ell} \tilde{j}_{\ell A} ( k ) \right\} + \sum_{i}^{3} | j_{iA} ( k ) |^{2} \\ &= - \frac{1}{q^{2}} | \boldsymbol{k} |^{2} | \boldsymbol{j}_{A} |^{2} \cos^{2} \theta + | \boldsymbol{j}_{A} |^{2} = | \boldsymbol{j}_{A} |^{2} ( 1 - \cos^{2} \theta ) = | j_{1 A} |^{2} + | j_{2 A} |^{2} \end{aligned} \tag{232a}
$$

<a id="p49"><sup>[p. 49]</sup></a>

where 1 and 2 are the two directions of transverse polarization. This shows how the third and fourth polarization directions do not appear in real <a id="ix-189">emission</a> problems. The same result would be obtained if we used the indefinite metric explicitly, i.e.take the sum in (231) over the 4 polarization states $\mu$ = 1$, 2 , 3 , 4 ,$ with the $\mu$ = 4 given a minus sign arising from the $\eta$ in (220c). But it is always simpler to work directly with the covariant formalism, than to bother with the non-physical photon states and then have to use $\eta$ to get the right answers.

Finally, the emission probability in direction of polarization 1 and in direction of propagation given by the solid angle $d$Ω is, using (232), (232a), and $\delta ( | \boldsymbol{k} |^{2} - q^{2} ) = \frac{1}{2 q} \delta ( | \boldsymbol{k} |- q )$ for $q>$ 0

<a id="eq-233"></a>

$$
w = \frac{qd \Omega}{8 \pi^{2} \hbar c^{2}} | j_{1 A} ( x ) |^{2} \tag{233}
$$

For dipole radiation by a one-electron atom with coordinates $( x,y,z )$

$$
j_{1} = e \dot{x} = iecqx \quad \boldsymbol{k} \boldsymbol{\cdot} \boldsymbol{r} \ll 1
$$

and

<a id="eq-234"></a>

$$
w = \frac{e^{2} q^{3} d \Omega}{8 \pi^{2} \hbar} | \langle x \rangle_{1 2} |^{2} \tag{234}
$$

This checks with Bethe’s _Handbuch_ article.<sup><a id="fnref-31" href="#fn-31">31</a></sup>

The example shows how covariant methods will work, even for problems of this elementary sort for which they are not particularly suited. The covariant method avoids the necessity of having to think about the normalization of the photon states, the factors of 2 and $\pi$ etc. being given automatically when one uses (221).

### <a id="s-the-hamiltonian-operator"></a>The Hamiltonian Operator

From the equation

$$
i \hbar \frac{\partial A_{\mu}}{\partial t} = \boldsymbol{[} A_{\mu} ,H \boldsymbol{]}
$$

we find

$$
\boldsymbol{[} a_{k\mu} ,H \boldsymbol{]} = \hbar c | \boldsymbol{k} | a_{k\mu}
$$

$$
\boldsymbol{[} \tilde{a}_{k\mu} ,H \boldsymbol{]} = - \hbar c | \boldsymbol{k} | \tilde{a}_{k\mu}
$$

Using the commutation rules (213) we can find an operator $H$ which satisfies all these conditions simultaneously. Namely

<a id="eq-234a"></a>

$$
H = \int d^{3} \boldsymbol{k} \hbar c | \boldsymbol{k} | \sum_{1}^{4} \tilde{a}_{k\lambda} a_{k\lambda} \tag{234a}
$$

This operator is in fact unique apart from an arbitrary additive constant. To fix the constant we require<sup><a id="fnref-32" href="#fn-32">32</a></sup> $\langle H \rangle_{o}$ = 0 which leads to the result (234a) precisely, as one can see at once from (219). Hence (234a) is the Hamiltonian of this theory, which is very simple in this momentum representation.

To derive $H$ from the Lagrangian is also possible but much more tedious. From (234a) we see that

$$
N_{k\lambda} = \tilde{a}_{k\lambda} a_{k\lambda} \quad ( \text{ not summed } )
$$

is an operator just representing the number of quanta in the frequency $k$ and polarization $\lambda$. It follows at once from the commutation rules (213), from the singular $\delta$-function factor which comes from the continuous spectrum, $N_{k\lambda}$ being in fact the number of quanta per unit frequency range, that $\int N_{k\lambda} d^{3} \boldsymbol{k}$ integrated over _any_ region of momentum-space has the integer eigenvalues 0, 1, 2, $\dots$ . This is so, because the state with <a id="p50"><sup>[p. 50]</sup></a> $n_{i}$ particles with momentum $k_{i}$ is Ψ = $\prod_{i = 1}^{\ell} ( \tilde{a}_{k_{i} \lambda} )^{n_{i}}$Ψ$o$. Then taking $\int_{\Omega} N_{k\lambda} d^{3} \boldsymbol{k}$ over Ω including the momenta $k_{1} ,k_{2} ,\dots ,k_{j}$ we get

$$
\int_{\Omega} N_{k\lambda} d^{3} \boldsymbol{k} \Psi = \int_{\Omega} \tilde{a}_{k\lambda} a_{k\lambda} \prod_{i = 1}^{\ell} ( \tilde{a}_{k_{i} \lambda} )^{n_{i}} \Psi_{o} d^{3} \boldsymbol{k}
$$

$$
= \int_{\Omega} \tilde{a}_{k\lambda} \left\{ \sum_{i = 1}^{\ell} n_{i} \prod_{i = 1}^{\ell} \tilde{a}_{k_{i} \lambda}^{( n_{i} - 1 )} \boldsymbol{[} a_{k\lambda} , \tilde{a}_{k_{i} \lambda} \boldsymbol{]} + \prod_{i = 1}^{\ell} ( a_{k_{i} \lambda} )^{n_{i}} a_{k\lambda} \right\} \Psi_{o} d^{3} \boldsymbol{k} \quad \text{ by } ( 2 1 3 ) \text{ and } ( 2 1 7 a )
$$

$$
= \int_{\Omega} \tilde{a}_{k\lambda} \sum_{i = 1}^{\ell} n_{i} \prod_{i = 1}^{\ell} \tilde{a}_{k_{i} \lambda}^{( n_{i} - 1 )} \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}_{i} ) \Psi_{o} d^{3} \boldsymbol{k} = \sum_{i = 1}^{j} n_{i} \prod_{i = 1}^{\ell} ( \tilde{a}_{k_{i} \lambda} )^{n_{i}} \Psi_{o} = \sum_{i = 1}^{j} n_{i} \Psi
$$

### <a id="s-fluctuations-of-the-fields"></a>Fluctuations of the Fields

Since the electromagnetic fields $\boldsymbol{E}$ and $\boldsymbol{H}$ are quantum-mechanical variables, they do not have well-determined values in any state in which energy and momentum are well-defined, for example the vacuum state. A state of the fields can be specified _either_ by fixing the values of $\boldsymbol{E}$ and $\boldsymbol{H}$ _or_ by specifying the numbers of quanta present with various momenta and energies. The two descriptions are complementary and are both possible only in the classical limit of large numbers of quanta and very strong fields.

An educational discussion of these questions, considering in detail the example of a cavity resonator with one mode of oscillation, has been given by L. P. Smith, _Phys. Rev._ **69** (1946) 195. This is worth reading. Essential is the fact that you cannot fix the time-dependence of the field (phase) with a fixed number of quanta (energy).

We consider a more general problem. What is the mean-square fluctuation in the vacuum state of a field-quantity? We define

<a id="eq-235"></a>

$$
\begin{aligned} E_{1} ( VT ) &= \frac{1}{VT} \int_{VT} E_{1} ( x ) d\tau dt \\ H_{1} ( VT ) &= \frac{1}{VT} \int_{VT} H_{1} ( x ) d\tau dt \end{aligned} \tag{235}
$$

averaged over some finite space-volume $V$ and also over a time $T$. Let $V ( \boldsymbol{k} ) = \int_{V} e^{- i \boldsymbol{k} \cdot \boldsymbol{r}} d\tau$. Then since $\boldsymbol{H} = \nabla \times \boldsymbol{A}$, we have

<a id="eq-236"></a>

$$
\begin{aligned} \left\langle \{ H_{1} ( VT ) \}^{2} \right\rangle_{o} &= \frac{1}{V^{2} T^{2}} \int \int d\tau d\tau^{\prime} dtdt^{\prime} \left\langle \left( \frac{\partial}{\partial x_{2}} A_{3} - \frac{\partial}{\partial x_{3}} A_{2} \right) \left( \frac{\partial}{\partial x_{2}^{\prime}} A_{3}^{\prime} - \frac{\partial}{\partial x_{3}^{\prime}} A_{2}^{\prime} \right) \right\rangle_{o} \\ &= \frac{1}{V^{2} T^{2}} \frac{\hbar c}{2} \int \int d\tau d\tau^{\prime} dtdt^{\prime} \left( \frac{\partial}{\partial x_{2}} \frac{\partial}{\partial x_{2}^{\prime}} + \frac{\partial}{\partial x_{3}} \frac{\partial}{\partial x_{3}^{\prime}} \right) D^{( 1 )} ( x - x^{\prime} ) \quad \text{ using } ( 2 2 5 ) \\ &= \frac{1}{V^{2} T^{2}} \frac{\hbar c}{1 6 \pi^{3}} \int \int d\tau d\tau^{\prime} dtdt^{\prime} \left( \frac{\partial}{\partial x_{2}} \frac{\partial}{\partial x_{2}^{\prime}} + \frac{\partial}{\partial x_{3}} \frac{\partial}{\partial x_{3}^{\prime}} \right) \int d^{4} ke^{ik \cdot ( x - x^{\prime} )} \delta ( k^{2} ) \text{ using } ( 2 2 6 ) \\ &= \frac{\hbar c}{1 6 \pi^{3} V^{2} T^{2}} \int_{0}^{T} \int_{0}^{T} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} ( k_{2}^{2} + k_{3}^{2} ) | V ( k ) |^{2} e^{i | \boldsymbol{k} | x_{0}} e^{- i | \boldsymbol{k} | x_{0}^{\prime}} dtdt^{\prime} \\ &= \frac{\hbar c}{1 6 \pi^{3} V^{2} T^{2}} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} ( k_{2}^{2} + k_{3}^{2} ) | V ( k ) |^{2} \frac{4 \sin^{2} ( \tfrac{1}{2} c | \boldsymbol{k} T )}{c^{2} | \boldsymbol{k} |^{2}} \\ \left\langle \{ E_{1} ( VT ) \}^{2} \right\rangle_{o} &= \frac{1}{V^{2} T^{2}} \frac{\hbar c}{2} \int \int d\tau d\tau^{\prime} dtdt^{\prime} \left( - \frac{\partial}{\partial x_{4}} \frac{\partial}{\partial x_{4}^{\prime}} - \frac{\partial}{\partial x_{1}} \frac{\partial}{\partial x_{1}^{\prime}} \right) D^{( 1 )} ( x - x^{\prime} ) \\ &= \frac{\hbar c}{1 6 \pi^{3} V^{2} T^{2}} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} ( | \boldsymbol{k} |^{2} - k_{1}^{2} ) | V ( k ) |^{2} \frac{4 \sin^{2} ( \tfrac{1}{2} c | \boldsymbol{k} T )}{c^{2} | \boldsymbol{k} |^{2}} = \left\langle \{ H_{1} ( VT ) \}^{2} \right\rangle_{o} \end{aligned} \tag{236}
$$

<a id="p51"><sup>[p. 51]</sup></a>

Taking for $V$ any finite volume and $T$ a finite time, this mean-square fluctuation is finite. Example: a sphere of radius R gives

<a id="eq-237"></a>

$$
V ( K ) = \frac{4 \pi}{| \boldsymbol{k} |^{3}} ( \sin R | \boldsymbol{k} |- R | \boldsymbol{k} | \cos R | \boldsymbol{k} | ) \tag{237}
$$

But if either $R$ or $T$ tends to zero, the fluctuations tend to $\infty$ and in the limit actually diverge.<sup><a id="fnref-33" href="#fn-33">33</a></sup> That is to say, only measurements of field-quantities averaged both in space and in time can have any physical reality.

### <a id="s-the-lamb-shift-fluctuation-of-position-of-electron"></a>Fluctuation of Position of an Electron in a Quantized Electromagnetic Field. The Lamb Shift.

Consider an electron represented by an extended spherical charge of radius R, lying in a stationary state in the potential $\varphi ( r )$ of a hydrogen atom. It has a certain wave-function $\psi ( r )$. We consider everything non-relativistically except for the quantized radiation field with which the electron interacts. The effect of this fluctuating field is to produce a rapid fluctuation in position of the electron. In fact we have for rapid fluctuations

$$
m \ddot{\boldsymbol{r}} = - e \boldsymbol{E}
$$

Thus a fluctuating component of $\boldsymbol{E}$ with frequency $c | \boldsymbol{K} |$ produces the same fluctuation in $\boldsymbol{r}$ with amplitude multiplied by a factor $\frac{e}{m} \frac{1}{c^{2} | \boldsymbol{K} |^{2}}$ . The slow fluctuations of $\boldsymbol{E}$ cannot be followed by the electron if the frequency is less than the atomic frequency $cK_{H}$. Hence we find from (236), making $T \to$ 0

$$
\left\langle r_{1}^{2} \right\rangle_{o} = \frac{e^{2}}{m^{2}} \frac{\hbar c}{1 6 \pi^{3} V^{2}} \int_{K_{H}}^{\infty} \frac{d^{3} \boldsymbol{K}}{| \boldsymbol{K} |} ( K_{2}^{2} + K_{3}^{2} ) | V ( K ) |^{2} \frac{1}{c^{4} | \boldsymbol{K} |^{4}}
$$

because lim sin$x {}^{2} x^{2}$ = 1. The integral now converges at $\infty$, because of the finite size of the electron. Since

$$
x \to 0
$$

$R$ is very small we may approximate (237) by

$$
V ( K ) = \begin{cases} \tfrac{4}{3} \pi R^{3} = V & \text{ for } | \boldsymbol{K} | R< 1 \\ 0 & \text{ for } | \boldsymbol{K} | R> 1 \end{cases}
$$

Then, since $( K_{2}^{2} + K_{3}^{2} ) = | \boldsymbol{K} |^{2} ( 1 -$ cos<sup>2</sup> $\theta )$ and $\int 0 \pi$ sin<sup>2</sup> $\theta$ sin$\theta d\theta = \tfrac{4}{3}$, we have<sup><a id="fnref-34" href="#fn-34">34</a></sup>

<a id="eq-238"></a>

$$
\left\langle r_{1}^{2} \right\rangle_{o} = \frac{e^{2} \hbar}{6 m^{2} c^{3} \pi^{2}} \int_{K_{H}}^{1 /R} \frac{d | \boldsymbol{K} |}{| \boldsymbol{K} |} = \frac{e^{2} \hbar}{6 m^{2} c^{3} \pi^{2}} \log \left( \frac{1}{RK_{H}} \right) \tag{238}
$$

This fluctuation in position produces a change in the effective potential acting on the electron. Thus

$$
\langle V ( r + \delta r ) \rangle = V ( r ) + \langle \delta \boldsymbol{r} \cdot \nabla V ( r ) \rangle_{o} + \tfrac{1}{2} \left\langle ( \delta \boldsymbol{r} )^{2} \right\rangle_{o} \frac{\partial^{2} V}{\partial r^{2}} + \dots
$$

$$
= V ( r ) + \tfrac{1}{2} \left\langle r_{1}^{2} \right\rangle_{o} \nabla^{2} V
$$

because $\langle \delta \boldsymbol{r} \cdot \nabla V ( r ) \rangle_{o}$ = 0, being odd. Now in a hydrogen atom, $\nabla^{2} V = e^{2} \delta^{3} ( \boldsymbol{r} )$ (Heaviside units!) Hence the change in the energy of the electron due to the fluctuations is<sup><a id="fnref-35" href="#fn-35">35</a></sup> $( a_{o}$ = Bohr radius)

<a id="eq-239"></a>

$$
\begin{aligned} \Delta E &= \int \psi^{*} \delta V\psi d\tau = \tfrac{1}{2} \left\langle r_{1}^{2} \right\rangle_{o} e^{2} | \psi ( 0 ) |^{2} \\ &= \begin{cases} \frac{e^{4} \hbar}{1 2 \pi^{2} m^{2} c^{3}} \log \left( \frac{1}{RK_{H}} \right) \frac{1}{\pi n^{3} a_{o}^{3}} & \text{ for s } - \text{ states } \\ 0 & \text{ for all others } \end{cases} \end{aligned} \tag{239}
$$

<a id="p52"><sup>[p. 52]</sup></a>

because for the hydrogen atom<sup><a id="fnref-36" href="#fn-36">36</a></sup> $( \rho = r \sqrt{- 8 m_{r} E_{n}} / \hbar )$

$$
\psi_{n\ell m} ( r,\theta ,\phi ) = - \frac{1}{\sqrt{2 \pi}} e^{im\phi} \left\{ \frac{( 2 \ell + 1 ) ( \ell -| m | ) !}{2 ( \ell + | m | ) !} \right\}^{1 / 2} P_{\ell}^{| m |} ( \cos \theta ) \left[ \left( \frac{2}{na_{o}} \right)^{3} \frac{( n - \ell - 1 ) !}{2 n [ ( n + \ell ) ! ]^{3}} \right]^{1 / 2} e^{- \rho / 2} \rho^{\ell} L_{n + \ell}^{2 \ell + 1} ( \rho )
$$

and

$$
\psi_{n 0 0} ( 0 ,\theta ,\phi ) = \frac{1}{\sqrt{2 \pi}} \frac{\sqrt{2}}{2} \left\{ - \left[ \left( \frac{2}{na_{o}} \right)^{3} \frac{( n - 1 ) !}{2 n ( n ! )^{3}} \right]^{1 / 2} \right\} \left( - \frac{( n ! )^{2}}{( n - 1 ) !} \right) = \frac{1}{\pi^{1 / 2} a_{o}^{3 / 2} n^{3 / 2}}
$$

There will also be a (much bigger) addition to kinetic energy, arising from the fluctuations. We ignore this on the grounds that it will be the same for all atomic states and so will not give any relativistic displacement. Of course this is not a good argument.

Hence we find the first approximation to the <a id="ix-123">Lamb shift</a>; the 2$s$ state is shifted relative to the 2$p$ states by

$$
\Delta E = + \frac{e^{4} \hbar}{9 6 \pi^{3} m^{2} c^{3} a_{o}^{3}} \log \frac{1}{RK_{H}}
$$

Now<sup><a id="fnref-37" href="#fn-37">37</a></sup>

$$
a_{o} = \frac{4 \pi \hbar^{2}}{me^{2}} = \frac{1}{\alpha} \frac{\hbar}{mc} \quad ( \text{ Bohr radius } )
$$

<div align="center">

Ry = $\frac{e^{4} m}{3 2 \pi^{2} \hbar^{4}}$ (Rydberg energy unit)

</div>

$$
K_{H} = \frac{\text{ Ry }}{4 \hbar c}
$$

We take $R = ( \hbar /mc )$, the electron Compton wave-length since it is at this frequency that the NR treatment becomes completely wrong. Then

<a id="eq-240"></a>

$$
\begin{aligned} RK_{H} &= \frac{\text{ Ry }}{4 mc^{2}} = \frac{1}{8} \alpha^{2} \\ \Delta E &= + \frac{\alpha^{3}}{3 \pi} \log ( 8 \times 1 3 7^{2} ) \text{ Ry } \end{aligned} \tag{240}
$$

Actually $\frac{\alpha^{3}}{3 \pi}$ Ry = 136 Mc in frequency units. This gives an effect of the right sign and order of magnitude. Method due to Welton <a id="refback-14-1"></a>[[14]](#ref-14).

The size of the log is wrong because the low-frequency cut-off was badly done. We find ∆$E \sim$ 1600 Mc instead of the correct value of 1060 Mc. But physically the origin of the shift is correctly described in this way.

## <a id="s-ia-theory-of-line-shift-and-line-width"></a>Ia. Theory of Line Shift and Line Width

To make a better treatment of the effect of the radiation interaction on energy-levels, we must try to solve more exactly the equation of motion for the system atom plus radiation field. The effect of the field is shown not only in a shift of energy levels, but also in a finite _width_ of the levels due to real radiation. Roughly, if the state has a life-time $T$ for decay by radiation, the width Γ of the level, or the mean variation in energy of the emitted photons, is given by the uncertainty principle Γ $\approx \hbar /T$. The line-shift and line-width are effects of the same kind and cannot be correctly treated except in combination.

So we make a theory now which treats the atom non-relativistically, but takes account properly of the radiation interaction. This means, we repeat the calculation of spontaneous emission by an atom, but now including the reaction of the radiation on the atom instead of taking the atom as given by a fixed charge-current oscillator.

For this sort of calculation it is always convenient to work in a special representation called the _Inter-_ _action Representation_.

<a id="p53"><sup>[p. 53]</sup></a>

### <a id="s-the-interaction-representation"></a>The Interaction Representation

In the Schrödinger representation the wave-function Ψ satisfies the equation of motion

<a id="eq-241"></a>

$$
i \hbar \frac{\partial}{\partial t} \Psi = H \Psi \tag{241}
$$

where $H$ is the Hamiltonian. In the case of an atom interacting with the radiation field, we have

<a id="eq-242"></a>

$$
H = H_{A} + H_{M} + H_{I}^{S} \tag{242}
$$

where $H_{A}$ is the Hamiltonian operator for the atom, and $H_{M}$ that for the Maxwell field without interaction. $H_{M}$ is given by (234a), and in quantum electrodynamics according to (170)

<a id="eq-243"></a>

$$
H_{I}^{S} = - \frac{1}{c} \int \sum_{\mu} j_{\mu}^{S} ( r ) A_{\mu}^{S} ( r ) d^{3} \boldsymbol{r} \tag{243}
$$

because $j_{\mu}^{S} ( r ) = ie \overline{\psi} \gamma_{\mu} \psi$, and in this case $H = \sum_{\mu} \pi_{\mu} A_{\mu} - L = - L$ because $\pi_{\mu} = \partial L /\partial \dot{A}_{\mu}$, and $L_{I}$ does not contain $\dot{A}_{\mu}$. All the operators in (242), (243) are time-independent<sup><a id="fnref-38" href="#fn-38">38</a></sup>, Schrödinger representation operators, and so are the given the label $S$.

Now we choose a new wave function Φ$( t )$ which is given in terms of Ψ by

<a id="eq-244"></a>

$$
\Psi ( t ) = \exp \left\{ - \frac{i}{\hbar} ( H_{A} + H_{M} ) t \right\} \Phi ( t ) \tag{244}
$$

This Φ$( t )$ will be a constant for any state representing the atom and the Maxwell field without interaction. Thus the time-variation of Φ$( t )$ in an actual state describes _just_ the effect of the interaction in perturbing the atomic states. From (241) and (244), the time-variation of Φ is given by the Schrödinger equation

<a id="eq-245"></a>

$$
i \hbar \frac{\partial \Phi}{\partial t} = H_{I} ( t ) \Phi \tag{245}
$$

with

<a id="eq-246"></a>

$$
H_{I} ( t ) = \exp \left\{ \frac{i}{\hbar} ( H_{A} + H_{M} ) t \right\} H_{I}^{S} \exp \left\{ - \frac{i}{\hbar} ( H_{A} + H_{M} ) t \right\} \tag{246}
$$

Thus

<a id="eq-247"></a>

$$
H_{I} = - \frac{1}{c} \int \sum_{\mu} j_{\mu} ( r,t ) A_{\mu} ( r,t ) d^{3} \boldsymbol{r} \tag{247}
$$

with

<a id="eq-248"></a>

$$
j_{\mu} ( r,t ) = \exp \left\{ \frac{i}{\hbar} H_{A} t \right\} j_{\mu}^{S} ( \boldsymbol{r} ) \exp \left\{ - \frac{i}{\hbar} H_{A} t \right\} \tag{248}
$$

<a id="eq-249"></a>

$$
A_{\mu} ( r,t ) = \exp \left\{ \frac{i}{\hbar} H_{M} t \right\} A_{\mu}^{S} ( r ) \exp \left\{ - \frac{i}{\hbar} H_{M} t \right\} \tag{249}
$$

These operators $j_{\mu} ( r,t )$ and $A_{\mu} ( r,t )$ have precisely the time-dependence of the field-operators in the Heisenberg representation, for the two systems, atom and radiation field, taken separately without interaction. Thus in the Interaction Representation the time-dependence of the Schrödinger wave-function is split into two parts, the operators taking the time-dependence of the non-interaction systems, the wave-function taking a time-dependence showing the effects of the interaction only. The operators $A_{\mu} ( r,t )$ satisfy the wave-equation $\Box^{2} A_{\mu}$ = 0 and the covariant commutation laws (203), because we see from (249) that

$$
\frac{\partial A_{\mu} ( r,t )}{\partial t} = i \hbar \boldsymbol{[} H_{M} ,A_{\mu} ( r,t ) \boldsymbol{]}
$$

i. e., the time variation of $A_{\mu} ( r,t )$ is the same as that of $A_{\mu} ( x )$ in the Heisenberg representation without interaction (see (190a)), which in turn leads to the field equations (197). Matrix elements of IR operators given by (246), (248) or (249) between IR wave-functions given by (244) are of course the same as the matrix elements which would be obtained in any other representation.

<a id="p54"><sup>[p. 54]</sup></a>

### <a id="s-application-of-interaction-representation-to-line-shift-and-line-width"></a>The Application of the Interaction <a id="ix-235">Representation</a> to the Theory of Line-Shift and Line-Width

Consider the solution of Eq. (245) in which the atom is given initially in a stationary unperturbed state $O$ with energy $E_{o}$, the <a id="ix-188">Maxwell</a> <a id="ix-178">field</a> being in the vacuum state, with no photons present. Let Φ$o$ be the interaction representation <a id="ix-236">wave-function</a> representing the atom in state $O$ and the Maxwell field in the vacuum state, without interaction. Φ$o$ is independent of time.

The initial condition Φ$( t )$ = Φ$o$ at time $t = t_{o}$ is a physically unreal one. It would mean putting the atom into existence at time 0 without any radiation field being excited at this time. This we cannot do physically. In fact the initial condition for an atom in an excited state will depend on how it got into the excited state. This cannot be formulated in a simple way: one needs a complicated model in order to describe the initial excitation of the atom.

We are interested in calculating the variation with time of (Φ$o *$Φ$( t )$), the probability amplitude for finding the atom still in the unperturbed state Φ$o$ at time $t$. By (245) we have

<a id="eq-250"></a>

$$
\frac{d}{dt} ( \Phi_{o}^{*} \Phi ( t ) ) = - \frac{i}{\hbar} ( \Phi_{o}^{*} H_{I} ( t ) \Phi ( t ) ) \tag{250}
$$

Suppose we take the physically unreal initial condition

$$
\Phi ( t ) = \Phi_{o} \quad \text{ at } t = t_{o}
$$

Then (250) will give

<a id="eq-251"></a>

$$
\frac{d}{dt} ( \Phi_{o}^{*} \Phi ( t ) )_{t = t_{o}} = 0 \tag{251}
$$

from the $H_{I}$ given by (247) has zero expectation value in the Maxwell field vacuum, since $A_{\mu} ( r,t )$ also has a zero expectation value in vacuum as one can see from (211) and (217). Thus (Φ$o *$Φ$( t )$) is momentarily stationary at $t = t_{o}$. This is however not interesting since the conditions at $t = t_{o}$ are entirely unphysical.

The physically meaningful quantity is the value of (250) at a time $t$ a long time after $t_{o}$. Then the atom will have “settled down” to a quasi-stationary state of radiative decay, and we may expect that the value we find for (250) is independent of the particular initial condition chosen and will be correct for an atom which has been excited by any reasonable method to the state Φ$o$.

We make the calculation so as to include effects of the radiation $H_{I}$ up to second order. This means we include effects of emitting and absorbing one photon only. In fact we know physically that effects from two or more photons are very small, so the approximation is a good one.

Suppose that $( t - t_{o} )$ is long compared with all the atomic frequencies. Then a solution of (245), valid to first order in $H_{I}$, is

<a id="eq-252"></a>

$$
\Phi_{1} ( t ) = \left[ 1 - \frac{i}{\hbar} \int_{-\infty}^{t} H_{I} ( t^{\prime} ) dt^{\prime} \right] a ( t ) \Phi_{o} \tag{252}
$$

+ terms involving other atomic states Φ$n$ with two or more photons present.

Here $a ( t )$ = (Φ$o *$Φ$( t )$) is a slowly varying amplitude, constant to first order in $H_{I}$, representing the slow decay of the atom. Note that our treatment is not only a perturbation theory correct to second order in $H_{I}$, but it also must account _exactly_ for the big effects produced by the radiative decay over long periods of time. Therefore we do not put $a ( t )$ = 1 in (252) although this would be correct to first order in $H_{I}$.

If we put $a ( t )$ = 1 in (252) we should have just the solution of the radiation emission problem, neglecting all effects of radiation reaction on the atom, which we obtained before from Eg. (230).

The value of $\frac{d}{dt} a ( t ) = \frac{d}{dt}$ (Φ$o *$Φ$( t )$) is obtained correct to second order in $H_{I}$, and including the radiative reaction effects, by substituting (252) into (250).

Hence we have

<a id="eq-253"></a>

$$
\frac{1}{a ( t )} \frac{d}{dt} a ( t ) = - \frac{1}{\hbar} \int_{-\infty}^{t} dt^{\prime} \{ \Phi_{o}^{*} H_{I} ( t ) H_{I} ( t^{\prime} ) \Phi_{o} \} \tag{253}
$$

<a id="p55"><sup>[p. 55]</sup></a>

Using (247), (221) this gives<sup><a id="fnref-39" href="#fn-39">39</a></sup>

<a id="eq-254"></a>

$$
\begin{aligned} \frac{1}{a} \frac{da}{dt} &= - \frac{i}{\hbar c} \int_{-\infty}^{t} dt^{\prime} \int \int d^{3} \boldsymbol{r} d^{3} \boldsymbol{r}^{\prime} D^{+} ( r - r^{\prime} ,t - t^{\prime} ) \sum_{\mu} \langle j_{\mu} ( r,t ) j_{\mu} ( r^{\prime} ,t^{\prime} ) \rangle_{oo} \\ &= - \frac{1}{( 2 \pi )^{3} \hbar c} \int \frac{d^{3} \boldsymbol{k}}{2 | \boldsymbol{k} |} \int_{-\infty}^{t} dt^{\prime} \int \int d^{3} \boldsymbol{r} d^{3} \boldsymbol{r}^{\prime} \exp \{ i \boldsymbol{k} \cdot ( \boldsymbol{r} - \boldsymbol{r}^{\prime} ) - ic | \boldsymbol{k} | ( t - t^{\prime} ) \} \sum_{\mu} \langle j_{\mu} ( r,t ) j_{\mu} ( r^{\prime} ,t^{\prime} ) \rangle_{oo} \end{aligned} \tag{254}
$$

Let the atomic states be labelled by $n$, the state $n$ having energy $E_{n}$. Let

<a id="eq-255"></a>

$$
j_{\mu}^{k} ( n,m ) \tag{255}
$$

be the matrix element of the operator

<a id="eq-256"></a>

$$
\int j_{\mu}^{S} ( r ) e^{- i \boldsymbol{k} \cdot \boldsymbol{r}} d^{3} \boldsymbol{r} \tag{256}
$$

in the transition $m \to n$. Then using a matrix product to evaluate $\langle j_{\mu} ( r,t ) j_{\mu} ( r^{\prime} ,t^{\prime} ) \rangle_{oo}$,

<a id="eq-257"></a>

$$
\frac{1}{a} \frac{da}{dt} = - \frac{1}{1 6 \pi^{3} \hbar c} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} \int_{-\infty}^{t} dt^{\prime} \sum_{n} \exp \left\{ \frac{i}{\hbar} ( t - t^{\prime} ) ( E_{o} - E_{n} - \hbar c | \boldsymbol{k} | ) \right\} \sum_{\mu} | j_{\mu}^{k} ( n, 0 ) |^{2} \tag{257}
$$

where we made use of (248).

As before, the sum extends only over the two transverse polarizations $\mu$, the other two cancelling each other exactly. Now we have to calculate

<a id="eq-258"></a>

$$
\int_{-\infty}^{0} e^{iax} dx = \pi \delta ( a ) + \frac{1}{ia} = 2 \pi \delta_{+} ( a ) \tag{258}
$$

this being the definition of the $\delta_{+}$ function. Thus

<a id="eq-259"></a>

$$
\frac{1}{a} \frac{da}{dt} = - \frac{1}{8 \pi^{2} c} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} \sum_{n,\mu} | j_{\mu}^{k} ( n, 0 ) |^{2} \delta_{+} ( E_{n} - E_{0} + \hbar c | \boldsymbol{k} | ) \tag{259}
$$

We write

<a id="eq-260"></a>

$$
\frac{1}{a} \frac{da}{dt} = - \frac{1}{2} \Gamma - \frac{i}{\hbar} \Delta E \tag{260}
$$

Then ∆$E$ and Γ are real constants given by

<a id="eq-261"></a>

$$
\Delta E = - \frac{\hbar}{1 6 \pi^{3} c} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} \sum_{n,\mu} \frac{| j_{\mu}^{k} ( n, 0 ) |^{2}}{E_{n} - E_{0} + \hbar c | \boldsymbol{k} |} \tag{261}
$$

<a id="eq-262"></a>

$$
\Gamma = \frac{1}{8 \pi^{2} c} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} \sum_{n,\mu} | j_{\mu}^{k} ( n, 0 ) |^{2} \delta ( E_{n} - E_{0} + \hbar c | \boldsymbol{k} | ) \tag{262}
$$

These are independent of $t$. Therefore the <a id="ix-15">amplitude</a> of the state Φ$o$ in the wave-function Φ$( t )$ is given for all $t \gg t_{o}$ by

<a id="eq-263"></a>

$$
a ( t ) = ( \Phi_{o}^{*} \Phi ( t ) ) = \exp \left\{ - \frac{i}{\hbar} \Delta E ( t - t_{o} ) - \frac{1}{2} \Gamma ( t - t_{o} ) \right\} \tag{263}
$$

The state Φ$o$, as a result of the perturbation by the radiation field, has its energy shifted by ∆$E$, and decays exponentially like

$$
| a ( t ) |^{2} = e^{- \Gamma ( t - t_{o} )} \quad ( 2 6 3 / a )
$$

<a id="p56"><sup>[p. 56]</sup></a>

Comparing (232) and (262), we see that Γ is exactly the total probability per unit time of radiation from the state $o$ to all other states $n$, calculated neglecting radiation reaction. This gives the physical interpretation for the decay law (263/a). When the denominators in (261) have zeroes, the integration over $| \boldsymbol{k} |$ is to be taken as a Cauchy principal value. The energy shift ∆$E$ is exactly what would be obtained from elementary second-order perturbation theory, if the difficulties arising from vanishing denominators were just ignored.

We calculate now the spectrum of radiation emitted in the transition from the level $o$ to the level $n$, including effects of the level shifts ∆$E_{o}$ and ∆$E_{n}$ and the widths Γ$o$ and Γ$n$. Let $b_{nk}$ be the amplitude at time $t$, of the state in which the atom is in state $n$ and the photon is present with propagation vector $k$. The equation of motion for $b_{nk}$, including effects of radiation out of the state $n$, is

<a id="eq-264"></a>

$$
\frac{db_{nk}}{dt} = \left\{ - \frac{1}{2} \Gamma_{n} - \frac{i}{\hbar} \Delta E_{n} \right\} b_{nk} - Q \exp \left\{ \frac{i}{\hbar} ( E_{n} - E_{o} + \hbar c | \boldsymbol{k} | ) t \right\} a ( t ) \tag{264}
$$

with $a ( t )$ given by (263).<sup><a id="fnref-40" href="#fn-40">40</a></sup> Here the last term represents the effects of transitions $o \to n$, and $Q$ is the space part of a matrix element of $H_{I}^{S}$ which is independent of $t$ and varies only slowly with $k$ so that we can regard $Q$ as a constant for all values of $k$ within the line-width. The exponential is the time part of the matrix element, the exponent being proportional to the energy difference between the atom in state $n$ plus the photon, and the atom in the zero state. The solution of (264) is, taking for convenience $t_{o}$ = 0,

<a id="eq-265"></a>

$$
b_{nk} = A \{ \exp ( - \beta t ) - \exp ( - \gamma t ) \} \tag{265}
$$

using the initial condition $b_{nk}$ = 0 at $t$ = 0. Here

<a id="eq-266"></a>

$$
\begin{aligned} \beta &= \frac{1}{2} \Gamma_{o} + \frac{i}{\hbar} ( E_{o} + \Delta E_{o} - E_{n} - \hbar c | \boldsymbol{k} | ) \\ \gamma &= \frac{1}{2} \Gamma_{n} + \frac{i}{\hbar} \Delta E_{n} \end{aligned} \tag{266}
$$

and $A = Q/ ( \beta - \gamma )$.

The probability that the atom leaves the state $n$ by a second radiative transition at time $t$, so that a quantum $k$ remains, is

$$
\Gamma_{n} | b_{nk} ( t ) |^{2}
$$

The quantum $k$ remains from the first transition $o \to n$. After the atom makes the second transition to a continuum of possible states, the final states will no longer be coherent, and so the quanta left behind at different times $t$ will not interfere with each other. The total probability for the emission of a quantum of frequency $k$ in the first transition is thus

<a id="eq-267"></a>

$$
P ( k ) = \Gamma_{n} | Q |^{2} \frac{1}{| \beta - \gamma |^{2}} \int_{0}^{\infty} | e^{- \beta t} - e^{- \gamma t} |^{2} dt \tag{267}
$$

Now

<a id="eq-268"></a>

$$
\begin{aligned} \frac{1}{| \beta - \gamma |^{2}} \int_{0}^{\infty} | e^{- \beta t} - e^{- \gamma t} |^{2} dt &= \frac{1}{( \beta - \gamma ) ( \beta^{*} - \gamma^{*} )} \left\{ \frac{1}{\beta + \beta^{*}} + \frac{1}{\gamma + \gamma^{*}} - \frac{1}{\beta + \gamma^{*}} - \frac{1}{\beta^{*} + \gamma} \right\} \\ &= \frac{1}{\beta - \gamma} \left\{ \frac{1}{( \gamma + \gamma^{*} ) ( \beta^{*} + \gamma )} - \frac{1}{( \beta + \beta^{*} ) ( \beta + \gamma^{*} )} \right\} \\ &= \frac{\beta + \beta^{*} + \gamma + \gamma^{*}}{( \beta + \beta^{*} ) ( \gamma + \gamma^{*} ) ( \beta + \gamma^{*} ) ( \gamma + \beta^{*} )} = \frac{1}{2} \frac{\operatorname{Re} ( \beta + \gamma )}{\operatorname{Re} ( \beta ) \operatorname{Re} ( \gamma ) | \beta + \gamma^{*} |^{2}} \end{aligned} \tag{268}
$$

Hence

<a id="eq-269"></a>

$$
P ( k ) = | Q |^{2} \frac{\Gamma_{o} + \Gamma_{n}}{\Gamma_{o}} \frac{\hbar^{2}}{( E_{o} + \Delta E_{o} - E_{n} - \Delta E_{n} - \hbar c | \boldsymbol{k} | )^{2} + \tfrac{1}{4} \hbar^{2} ( \Gamma_{o} + \Gamma_{n} )^{2}} \tag{269}
$$

This formula for P(k) gives the natural shape of a spectral line. The maximum intensity occurs at

<a id="eq-270"></a>

$$
\hbar c | \boldsymbol{k} | = ( E_{o} + \Delta E_{o} ) - ( E_{n} + \Delta E_{n} ) \tag{270}
$$

<a id="p57"><sup>[p. 57]</sup></a>

i.e. at the difference between the energies of the two levels including the radiative level shifts. The width at half-maximum is

<a id="eq-271"></a>

$$
\hbar ( \Gamma_{o} + \Gamma_{n} ) \tag{271}
$$

just the sum of the two widths of the levels given by their reciprocal life-times.

These formulae (270) and (271) are important in interpreting the modern radio-frequency spectroscopic experiments with their very accurate measurements of line shapes and positions.

### <a id="s-calculation-of-line-shift-non-relativistic-theory"></a>Calculation of Line-Shift, Non-Relativistic Theory

In all atomic systems, the line widths are finite and easily calculated from the known transition amplitudes. For this, non-relativistic theory is accurate enough for all purposes. The line-shift (261) is much more difficult, and non-relativistic theory is not accurate enough to handle it properly. Still we shall calculate (261) using the NR theory, to see what it gives. It turns out to give quite a lot that is interesting.

First, in a NR calculation we use the dipole approximation which we also used to derive (234). Supposing a one-electron atom, the electron having mass $m$ and charge $- e$, we put

<a id="eq-272"></a>

$$
j_{1}^{k} ( n 0 ) = - \frac{e}{m} ( p_{1} )_{n 0} = - \frac{e}{m} \left\{ \int \psi_{n}^{*} \left( - i \hbar \frac{\partial}{\partial x} \right) \psi_{0} d^{3} \boldsymbol{r} \right\} \tag{272}
$$

The line shift (261) becomes

$$
\Delta E = - \frac{e^{2} \hbar}{1 6 \pi^{3} m^{2} c} \int \frac{d^{3} \boldsymbol{k}}{| \boldsymbol{k} |} \sum_{n} \frac{| ( p_{1} )_{n 0} |^{2} + | ( p_{2} )_{n 0} |^{2}}{E_{n} - E_{o} + \hbar c | \boldsymbol{k} |}
$$

and integrating over the direction of $\boldsymbol{k}$, (compare with (238)),

<a id="eq-273"></a>

$$
\Delta E = - \frac{e^{2} \hbar}{6 \pi^{2} m^{2} c} \int_{0}^{\infty} | \boldsymbol{k} | d | \boldsymbol{k} | \sum_{n} \frac{| \boldsymbol{p}_{n 0} |^{2}}{E_{n} - E_{o} + \hbar c | \boldsymbol{k} |} \tag{273}
$$

The integral over $| \boldsymbol{k} |$ is now obviously divergent, even before summing over $n$. Therefore the line-shift is _infinite_. When a complete relativistic theory with positrons is used, the divergence becomes only logarithmic instead of linear, but it still definitely diverges. This was for many years a disaster which destroyed all faith in the theory, and no way of avoiding the difficulty was found until 1947.

### <a id="s-the-idea-of-mass-renormalization"></a>The Idea of Mass Renormalization

The line shift (273) is also infinite for a _free_ electron of momentum $\boldsymbol{p}$. In this case $\boldsymbol{p}$ is a diagonal operator and the sum over $n$ reduces to the term $n$ = 0. Therefore

<a id="eq-274"></a>

$$
\Delta E_{F} = - \frac{1}{6 \pi^{2}} \frac{e^{2}}{m^{2} c^{2}} \int \left( \tfrac{\infty}{0} d | \boldsymbol{k} | \right) \boldsymbol{p}^{2} \tag{274}
$$

The effect of the radiation interaction is just to give a free electron an additional energy proportional to its kinetic energy $( \boldsymbol{p}^{2} / 2 m )$. If the integral in (274) is cut off at an upper limit $K \sim ( mc/ \hbar )$ in order to allow for the fact that the theory anyway is wrong in the relativistic region; then

$$
\Delta E_{F} \approx - \frac{1}{6 \pi^{2}} \frac{e^{2}}{\hbar c} \frac{\boldsymbol{p}^{2}}{m}
$$

is a small correction to the kinetic energy, which would be produced by an increase in the rest-mass of the electron from $m$ to $( m + \delta m )$,

<a id="eq-275"></a>

$$
\delta m = \frac{1}{3 \pi^{2}} \frac{e^{2}}{c^{2}} \int_{0}^{\infty} d | \boldsymbol{k} | \tag{275}
$$

<a id="p58"><sup>[p. 58]</sup></a>

We must now take into account that the observed rest-mass of _any_ electron, bound or free, is not $m$ but $m + \delta m$. Therefore in (273) a part

<a id="eq-276"></a>

$$
- \frac{1}{6 \pi^{2}} \frac{e^{2}}{c^{2}} \int \left( \tfrac{\infty}{0} d | \boldsymbol{k} | \right) \left\langle \boldsymbol{p}^{2} \right\rangle_{oo} \tag{276}
$$

expresses only the effect of the mass-change $\delta m$ on the kinetic energy of the bound electron; this part is already included in the kinetic energy of the electron, when the observed mass $( m + \delta m )$ is taken for the mass in the formula $( \boldsymbol{p}^{2} / 2 m )$. Therefore the part (276) has to be subtracted from (273), to give the observable <a id="ix-154">line-shift</a>. The subtraction just cancels out the error that was made in identifying the mass $m$ of a “bare” electron without electromagnetic interaction with the observed electronic mass.

The idea of this <a id="ix-171">mass-renormalization</a> is that, although the “bare” mass $m$ appears in the original description of the atom without radiation field, all the final results of the theory should depend only on the physically observable $m + \delta m$. The idea is originally due to Kramers <a id="refback-16-1"></a>[[16]](#ref-16), developed by Bethe (_Phys. Rev._ **72** (1947) 339.)

Subtracting (276) from (273) gives the physically observable line-shift

<a id="eq-277"></a>

$$
\Delta E = \frac{e^{2}}{6 \pi^{2} m^{2} c^{2}} \int_{0}^{\infty} d | \boldsymbol{k} | \sum_{n} \frac{( E_{n} - E_{o} ) | \boldsymbol{p}_{no} |^{2}}{E_{n} - E_{o} + \hbar c | \boldsymbol{k} |} \tag{277}
$$

The divergence at high $| \boldsymbol{k} |$ is now only logarithmic. Taking an upper limit cut-off for the integral at the point

$$
\hbar c | \boldsymbol{k} | = K
$$

where $K$ is an energy on the order of magnitude of $mc^{2}$, we have

<a id="eq-278"></a>

$$
\Delta E = \frac{e^{2}}{6 \pi^{2} m^{2} c^{3} \hbar} \sum_{n} ( E_{n} - E_{o} ) | \boldsymbol{p}_{no} |^{2} \log \frac{K}{E_{n} - E_{o}} \tag{278}
$$

remembering that the integration over $| \boldsymbol{k} |$ in (277) is to be taken as a Cauchy principal value when $( E_{n} - E_{0} )$ is negative.

From this formula (278) the line-shift for hydrogen states can be calculated numerically, as was done by Bethe, Brown and Stehn (_Phys. Rev._ **77** (1950) 370.)

Since the log in (278) will be quite large $( \sim$ 7) for states $n$ which are in the non-relativistic range, it is convenient to write

<a id="eq-279"></a>

$$
\sum_{n} ( E_{n} - E_{o} ) | \boldsymbol{p}_{no} |^{2} \log | E_{n} - E_{o} | = \left\{ \sum_{n} ( E_{n} - E_{o} ) | \boldsymbol{p}_{no} |^{2} \right\} \log ( E - E_{o} )_{\text{ av }} \tag{279}
$$

this being the definition of $( E - E_{o} )_{\text{ av }}$. Then $( E - E_{o} )_{\text{ av }}$ is a non-relativistic energy. Exact calculation gives for the 2s state in hydrogen

<a id="eq-280"></a>

$$
( E - E_{o} )_{\text{ av }} = 1 6 . 6 \text{ Ry } \tag{280}
$$

Thus the important transitions are to states which although non-relativistic are continuum states with very high excitation. This is surprising.

Note that in (278) the terms are all positive if $E_{o}$ is the ground state. For higher states there will be both positive and negative contributions. In particular, we shall see that for a Coulomb potential the positive and negative terms cancel almost exactly, for all except $s$ states<sup><a id="fnref-41" href="#fn-41">41</a></sup>. The cancellation is more or less accidental and seems to have no deeper meaning.

Now using a sum rule

<a id="eq-281"></a>

$$
\sum_{n} ( E_{n} - E_{o} ) | \boldsymbol{p}_{no} |^{2} = \langle \boldsymbol{p} \boldsymbol{\cdot} \boldsymbol{[} H, \boldsymbol{p} \boldsymbol{]} \rangle_{oo} \tag{281}
$$

where $H$ is the Hamiltonian for the atom<sup><a id="fnref-42" href="#fn-42">42</a></sup>

<a id="eq-282"></a>

$$
H = \frac{1}{2 m} \boldsymbol{p}^{2} + V \quad V = - \frac{1}{4 \pi} \frac{e^{2}}{r} , \quad \boldsymbol{[} H, \boldsymbol{p} \boldsymbol{]} = i \hbar ( \nabla V ) \tag{282}
$$

<a id="p59"><sup>[p. 59]</sup></a>

<a id="eq-283"></a>

$$
\begin{aligned} \langle \boldsymbol{p} \cdot \boldsymbol{[} H, \boldsymbol{p} \boldsymbol{]} \rangle_{oo} &= \hbar^{2} \int \psi_{o}^{*} \nabla \cdot ( \psi_{o} \nabla V ) d\tau \\ &= \frac{\hbar^{2}}{2} \left\{ \int \psi_{o}^{*} \nabla \cdot ( \psi_{o} \nabla V ) d\tau + \int \psi_{o} \nabla \cdot ( \psi_{o}^{*} \nabla V ) d\tau \right\} \\ &= \frac{\hbar^{2}}{2} \left\{ 2 \int \psi_{o}^{*} \psi_{o} \nabla^{2} Vd\tau + \int \nabla ( \psi_{o}^{*} \psi_{o} ) \cdot \nabla Vd\tau \right\} \\ &= \frac{\hbar^{2}}{2} \int \psi_{o}^{*} \psi_{o} \nabla^{2} Vd\tau = \tfrac{1}{2} e^{2} \hbar^{2} | \psi_{o} ( 0 ) |^{2} \end{aligned} \tag{283}
$$

where we used Green’s vector theorem, the fact that $\langle \boldsymbol{p} \cdot \boldsymbol{[} H, \boldsymbol{p} \boldsymbol{]} \rangle_{oo}$ is real, and the result that $\nabla^{2} V$ = $e^{2} \delta^{3} ( \boldsymbol{r} )$ in Heaviside units.

Hence<sup><a id="fnref-43" href="#fn-43">43</a></sup>

<a id="eq-284"></a>

$$
\begin{aligned} \Delta E &= \frac{e^{4} \hbar}{1 2 \pi^{2} m^{2} c^{3}} | \psi_{o} ( 0 ) |^{2} \log \frac{K}{( E - E_{o} )_{\text{ av }}} \\ &= \frac{e^{4} \hbar}{1 2 \pi^{2} m^{2} c^{3}} \log \frac{K}{( E - E_{o} )_{\text{ av }}} \times \begin{cases} 1 / ( \pi n^{3} a_{o}^{3} ) & \text{ for } s \text{ states } \\ 0 & \text{ for others } \end{cases} \end{aligned} \tag{284}
$$

Compare this with (239). It differs only in having the log$( K/ ( E - E_{o} )_{\text{ av }} )$ replacing log(1$/RK_{H} )$. The low frequency photons have now been treated properly instead of being estimated. Only the high-frequency end is still inaccurate because of the vagueness of the cut-off $K$. Taking $K = mc^{2}$, (284) gives for the Lamb shift 2$s - 2 p$ the value _1040 Megacycles_. Remarkably close to the experimental value of 1062.

The success of this calculation of the line shift shows that the correct treatment of the interaction between an electron and the Maxwell field, with the help of the idea of mass renormalization, will give sensible results in agreement with experiment. This calculation could be done non-relativistically because the line shift is mainly a low-frequency and non-relativistic effect.

There are other effects of the radiation interaction, especially the anomalous increase in the observed magnetic moment of the electron by a factor $( )$1 + $\frac{\alpha}{2 \pi} ( )$ over the value given by the Dirac theory, which are essentially relativistic in character. For studying these effects, and for calculating the Lamb shift accurately without an arbitrary cut-off, we need to use a complete relativistic quantum electrodynamics, in which both electrons and the Maxwell field are handled relativistically.

Therefore we must go back to where we left off the theory of the Dirac electron on page 31, and start to construct a relativistic field theory of electrons and positrons, similar to the quantized Maxwell field theory.

## <a id="s-ii-field-theory-of-the-dirac-electron-without-interaction"></a>II. Field Theory of the Dirac Electron, Without Interaction

We apply to the Dirac equation the method of field quantization for anti-commuting fields. The reason why we must do this, and not use commuting fields, we will see later. Write

$$
\mu = ( mc/ \hbar ) , \quad m = \text{ electron mass }
$$

Lagrangian

<a id="eq-285"></a>

$$
L_{o} = - \hbar c \overline{\psi} \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) \psi \tag{285}
$$

Note the factor $\hbar$ here. This means that the theory has no classical limit in the sense of the Correspondence Principle. In the classical limit, only charges and currents composed of many particles have any meaning; the $\psi$ field disappears entirely from view. The $\hbar$ has to be put into (285) to make the dimensions right, since $\left( \overline{\psi} \psi \right)$ has dimensions (1/Volume) just as in the 1-particle Dirac theory of which this is an extension. <a id="p60"><sup>[p. 60]</sup></a> <a id="ix-179">Field</a> equations

<a id="eq-286"></a>

$$
\begin{aligned} \sum_{\lambda} \gamma_{\lambda} \frac{\partial \psi}{\partial x_{\lambda}} + \mu \psi &= 0 \\ \sum_{\lambda} \frac{\partial \overline{\psi}}{\partial x_{\lambda}} \gamma_{\lambda} - \mu \overline{\psi} &= 0 \end{aligned} \tag{286}
$$

The charge-conjugate field $\varphi$ can be defined by

$$
\varphi = C\psi^{+}
$$

according to (51), and it also satisfies

<a id="eq-287"></a>

$$
\left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) \varphi = 0 \tag{287}
$$

### <a id="s-covariant-commutation-rules"></a>Covariant Commutation Rules

We proceed as for the <a id="ix-192">Maxwell</a> field. Take two points $z$ and $y$ with $z_{0} >y_{0}$. Let

<a id="eq-288"></a>

$$
\begin{aligned} \mathcal{Q} ( y ) &= \overline{\psi} ( y ) u \\ \mathcal{R} ( z ) &= \overline{v} \psi ( z ) \text{ or } \quad \overline{\psi} ( z ) v \end{aligned} \tag{288}
$$

Here $u$ and $v$ are spinor operators, not depending on $y$ or $z$ and anticommuting with all the $\psi$ and $\overline{\psi}$ operators in our equations, as we assumed at the beginning of this section. For example take $u = \psi ( w )$ where $w$ is a point far away outside the light-cones of both $y$ and $z$. We make a change in the Lagrangian by

<a id="eq-289"></a>

$$
\delta_{\mathcal{Q}} L = \epsilon \delta^{4} ( x - y ) \overline{\psi} ( y ) u \tag{289}
$$

The factor $u$ must be put in to make $\delta_{\mathcal{Q}} L$ a bilinear expression, which is necessary for the Peierls method to be applicable. In fact only bilinear expressions have a physically observable meaning, and it is never meaningful under any circumstances to add together a term linear and a term bilinear in the field operators.

The changed field equations for $\psi$ and $\overline{\psi}$ are

For $\overline{\psi}$ : No change

<a id="eq-290"></a>

$$
\text{ For } \psi : \quad \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) \psi - \frac{\epsilon}{\hbar c} \delta^{4} ( x - y ) u = 0 \tag{290}
$$

Thus $\delta_{\mathcal{Q}} \psi ( z )$ = 0 and $\epsilon \delta_{\mathcal{Q}} \psi ( z )$ satisfies (290). (Compare with (198).)<br>
Hence $\delta_{\mathcal{Q}} \psi ( z )$ is defined by the conditions

<a id="eq-291"></a>

$$
\begin{aligned} \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) \delta_{\mathcal{Q}} \psi ( z ) &= \frac{1}{\hbar c} \delta^{4} ( z - y ) u \\ \delta_{\mathcal{Q}} \psi ( z ) &= 0 \quad \text{ for } z_{0} <y_{0} \end{aligned} \tag{291}
$$

From (291), $\delta_{\mathcal{Q}} \psi ( z )$ is a c-number spinor. We write

<a id="eq-292"></a>

$$
\delta_{\mathcal{Q}} \psi ( z ) = - \frac{1}{\hbar c} S_{R} ( z - y ) u \tag{292}
$$

Then $S_{R} ( x )$ is a c-number Dirac matrix function of x, satisfying

<a id="eq-293"></a>

$$
\begin{aligned} \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) S_{R} ( x ) &= - \delta^{4} ( x ) \\ S_{R} ( x ) &= 0 \text{ for } x_{0} < 0 \end{aligned} \tag{293}
$$

<a id="p61"><sup>[p. 61]</sup></a>

and the 4 $\times$ 4 unit matrix $\mathbb{I}$ is understood on the right of (293).

If we have $\mathcal{R} = \overline{\psi} ( z ) v$ then $\delta \overline{\psi}$ = 0, as before. If $\mathcal{R} = \overline{v} \psi ( z )$ then

<a id="eq-294"></a>

$$
\begin{aligned} \delta_{\mathcal{R}} \psi ( y ) &= - \frac{1}{\hbar c} \overline{v} S_{A} ( z - y ) \\ \delta_{\mathcal{R}} \psi ( y ) &= 0 \end{aligned} \tag{294}
$$

where $S_{A} ( x )$ is the <a id="ix-139">Dirac</a> matrix satisfying

<a id="eq-295"></a>

$$
\begin{aligned} \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) S_{A} ( x ) &= - \delta^{4} ( x ) \\ S_{A} ( x ) &= 0 \text{ for } x_{0} > 0 \end{aligned} \tag{295}
$$

Finally, if we had chosen $\mathcal{Q} = \overline{u} \psi ( y )$ we should have had $\delta_{\mathcal{Q}} \psi ( z )$ = 0 in the same way.

Hence using the Peierls commutation law (194) with (292) and (294) we find

<a id="eq-296"></a>

$$
\begin{aligned} \boldsymbol{[} \overline{v} \psi ( z ) , \overline{u} \psi ( y ) \boldsymbol{]} &= \boldsymbol{[} \overline{\psi} ( z ) v, \overline{\psi} ( y ) u \boldsymbol{]} = 0 \\ \boldsymbol{[} \overline{v} \psi ( z ) , \overline{\psi} ( y ) u \boldsymbol{]} &= - i \overline{v} [ S_{A} ( z - y ) - S_{R} ( z - y ) ] u \end{aligned} \tag{296}
$$

These hold for every choice of $u$ and $v$ if we now choose $u$ and $v$ anti-commuting with all the $\psi$ and $\overline{\psi}$ operators. Therefore writing

<a id="eq-297"></a>

$$
S ( x ) = S_{A} ( x ) - S_{R} ( x ) \tag{297}
$$

we can write down the commutation rules for the operator components:

<a id="eq-298"></a>

$$
\{ \psi_{\alpha} ( z ) ,\psi_{\beta} ( y ) \} = \left\{ \psi_{\alpha} ( z ) , \overline{\psi}_{\beta} ( y ) \right\} = 0 \tag{298}
$$

<a id="eq-299"></a>

$$
\left\{ \psi_{\alpha} ( z ) , \overline{\psi}_{\beta} ( y ) \right\} = - iS_{\alpha \beta} ( z - y ) \tag{299}
$$

The invariant $S -$function satisfies by (293) and (295)

<a id="eq-300"></a>

$$
\left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} + \mu \right) S ( x ) = 0 \tag{300}
$$

There are no simple formulae for the $S$-functions like (261) and (265) for the $D$-functions, in coordinate space. However in momentum representations the $S$-functions are equally simple.

### <a id="s-momentum-representations-2"></a>Momentum Representations

Write

<a id="eq-301"></a>

$$
\begin{aligned} S_{R} ( x ) &= \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} - \mu \right) \Delta_{R} ( x ) , \quad S_{A} ( x ) = \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} - \mu \right) \Delta_{A} ( x ) \\ S ( x ) &= \left( \sum_{\lambda} \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} - \mu \right) \Delta ( x ) \end{aligned} \tag{301}
$$

Then we have

<a id="eq-302"></a>

$$
\begin{aligned} \left( \Box^{2} - \mu^{2} \right) \Delta_{R} ( x ) &= \left( \Box^{2} - \mu^{2} \right) \Delta_{A} ( x ) = - \delta^{4} ( x ) \\ \left( \Box^{2} - \mu^{2} \right) \Delta ( x ) &= 0 \quad \Delta ( x ) = ( \Delta_{A} - \Delta_{R} ) ( x ) \end{aligned} \tag{302}
$$

with the boundary conditions as before. And the ∆-functions are exact analogues of the $D$-functions, the $D$-functions being the special case $\mu$ = 0. Instead of (207) we have by making the formal substitution

<a id="eq-303"></a>

$$
\begin{aligned} & \quad k^{2} \to k^{2} + \mu^{2} \int \\ \Delta_{R} ( x ) &= \frac{1}{( 2 \pi )^{4}}_{+} e^{ik \cdot x} \frac{1}{k^{2} + \mu^{2}} d^{4} k \end{aligned} \tag{303}
$$

<a id="p62"><sup>[p. 62]</sup></a>

where the contour in the $k_{0}$-plane goes above the two poles at $k_{0} = \pm \sqrt{| \boldsymbol{k} |^{2} + \mu^{2}}$. Similarly for (208). And instead of (210)

<a id="eq-304"></a>

$$
\Delta ( x ) = - \frac{i}{( 2 \pi )^{3}} \int e^{ik \cdot x} \delta ( k^{2} + \mu^{2} ) \epsilon ( k ) d^{4} k \tag{304}
$$

Hence using (301) and the notation (110)

<a id="eq-305"></a>

$$
S ( x ) = \frac{1}{( 2 \pi )^{3}} \int e^{ik \cdot x} ( k / + i\mu ) \delta ( k^{2} + \mu^{2} ) \epsilon ( k ) d^{4} k \tag{305}
$$

Note the projection operator Λ$+$ appearing here, as defined by (115) with the momentum $p = \hbar k$. Thus the $S$-function distinguishes automatically between the electron states $k_{0}$ = +$\sqrt{| \boldsymbol{k} |^{2} + \mu^{2}}$ and the positron states $k_{0} = - \sqrt{| \boldsymbol{k} |^{2} + \mu^{2}}$

### <a id="s-fourier-analysis-of-operators-2"></a>Fourier Analysis of Operators

We analyze $\psi_{\alpha}$ into Fourier components, written in a quite general form:

<a id="eq-306"></a>

$$
\psi_{\alpha} ( x ) = Q \int d^{3} \boldsymbol{k} \left( \frac{\mu^{2}}{| \boldsymbol{k} |^{2} + \mu^{2}} \right)^{1 / 4} \left\{ \sum_{u^{+}} u_{\alpha} e^{i \boldsymbol{k} \cdot \boldsymbol{x}} b_{ku} + \sum_{u^{-}} u_{\alpha} e^{- i \boldsymbol{k} \cdot \boldsymbol{x}} b_{ku} \right\} \tag{306}
$$

where, as in (211), the factor $\left( \mu^{2} / ( | \boldsymbol{k} |^{2} + \mu^{2} ) \right)^{1 / 4}$ only makes the notation simpler. The integration is over all 4-vectors $k$ with $k_{0}$ = +$\sqrt{| \boldsymbol{k} |^{2} + \mu^{2}}$. For each $k$, the sum $\sum u^{+}$ is over the two spin-states $u$ satisfying, by (111), the equation

<a id="eq-307"></a>

$$
( / k - i\mu ) u = 0 \tag{307}
$$

and the sum $\sum u -$ is over the two spin-states $u$ satisfying by (112)

<a id="eq-308"></a>

$$
( / k + i\mu ) u = 0 \tag{308}
$$

the normalization being given by (106) and (113). The $b_{ku}$ are operators independent of $x$ and $\alpha$, whose properties are to be determined.

Taking the adjoint to (306) we have

<a id="eq-309"></a>

$$
\overline{\psi}_{\alpha} ( x ) = Q \int d^{3} \boldsymbol{k} \left( \frac{\mu^{2}}{| \boldsymbol{k} |^{2} + \mu^{2}} \right)^{1 / 4} \left\{ \sum_{u^{+}} b_{ku}^{*} \overline{u}_{\alpha} e^{- i \boldsymbol{k} \cdot \boldsymbol{x}} + \sum_{u -} b_{ku}^{*} \overline{u}_{\alpha} e^{i \boldsymbol{k} \cdot \boldsymbol{x}} \right\} \tag{309}
$$

Here the $b_{ku}^{*}$ are ordinary Hermitian conjugates of $b_{ku}$.

Computing the anti-commutators (298), (299) from (306) and (309), and comparing the results with the momentum integral (305), using (115) and the properties of Λ$+$ we have

<a id="eq-310"></a>

$$
\{ b_{ku} ,b_{k^{\prime} v} \} = \{ b_{ku}^{*} ,b_{k^{\prime} v}^{*} \} = 0 \tag{310}
$$

<a id="eq-311"></a>

$$
\{ b_{ku} ,b_{k^{\prime} v}^{*} \} = \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{uv} \tag{311}
$$

and we find that the constant $Q$ in (306) and (309) is given by<sup><a id="fnref-44" href="#fn-44">44</a></sup>

<a id="eq-312"></a>

$$
Q = ( 2 \pi )^{- 3 / 2} \tag{312}
$$

### <a id="s-emission-and-absorption-operators-2"></a>Emission and Absorption Operators

Let

<a id="eq-313"></a>

$$
E_{k} = \hbar c \sqrt{| \boldsymbol{k} |^{2} + \mu^{2}} \tag{313}
$$

<a id="p63"><sup>[p. 63]</sup></a>

be the energy of an electron or a positron with momentum $\hbar k$. We apply the same argument which led to (215) for the Maxwell field. From this it follows that

<div align="center">

$b_{ku}$ for electron <a id="ix-163">states</a> $u$<br>
$b_{ku}^{*}$ for positron states $u$

</div>

have matrix elements only for transitions from an initial state of energy $E_{1}$ to a final state of energy $E_{2}$, where

<a id="eq-314"></a>

$$
E_{1} - E_{2} = E_{k} \tag{314}
$$

## <a id="s-and"></a>And

<div align="center">

$b_{ku}$ for positron states $u$<br>
$b_{ku}^{*}$ for electron states $u$

</div>

have matrix elements which are non-zero only when

<a id="eq-315"></a>

$$
E_{2} - E_{1} = E_{k} \tag{315}
$$

Thus we see as before that the field has the properties we require of a quantized field. It can carry energy only in discrete lumps of magnitude $E_{k}$ for each frequency $k$. And the energy can be carried by two kinds of excitation, which we have called electrons and positrons anticipating the later results of the theory. We see already that these two excitation states have particle properties, and that there are two kinds of particles.

The absorption operators are

<div align="center">

$b_{ku}$ for electrons

</div>

$$
b_{ku}^{*} \text{ for positrons }
$$

the emission operators are

<div align="center">

$b_{ku}$ for positrons

</div>

$$
b_{ku}^{*} \text{ for electrons }
$$

### <a id="s-charge-symmetrical-representation"></a>Charge-Symmetrical Representation

We use the charge-<a id="ix-119">conjugate</a> field $\varphi$ defined by (51), in order to put the whole theory into a form where there is complete symmetry between electrons and positrons. This symmetry is known as the charge-symmetry of the theory.

Let $k$ be given, and a spinor $u$ satisfying (308), representing a positron state. We represent the positron state alternatively by the charge conjugate spinor

<a id="eq-316"></a>

$$
v = Cu^{+} \tag{316}
$$

which will satisfy (307) as $u$ does for electron states. We denote by

$$
b_{kv}^{C} = b_{ku}^{*}
$$

the absorption operator for the positron state $v$. Then instead of (306), (309) we may write the pair of equations

<a id="eq-317"></a>

$$
\psi_{\alpha} ( x ) = Q \int d^{3} \boldsymbol{k} \left( \frac{\mu^{2}}{| \boldsymbol{k} |^{2} + \mu^{2}} \right)^{1 / 4} \left\{ \sum_{u^{+}} u_{\alpha} e^{i \boldsymbol{k} \cdot \boldsymbol{x}} b_{ku} + \sum_{v^{+}} \left\{ Cv^{+} \right\}_{\alpha} e^{- i \boldsymbol{k} \cdot \boldsymbol{x}} b_{kv}^{* C} \right\} \tag{317}
$$

<a id="eq-318"></a>

$$
\varphi_{\alpha} ( x ) = Q \int d^{3} \boldsymbol{k} \left( \frac{\mu^{2}}{| \boldsymbol{k} |^{2} + \mu^{2}} \right)^{1 / 4} \left\{ \sum_{v^{+}} v_{\alpha} e^{i \boldsymbol{k} \cdot \boldsymbol{x}} b_{kv}^{C} + \sum_{u^{+}} \left\{ Cu^{+} \right\}_{\alpha} e^{- i \boldsymbol{k} \cdot \boldsymbol{x}} b_{ku}^{*} \right\} \tag{318}
$$

<a id="p64"><sup>[p. 64]</sup></a>

The $\psi$ and $\varphi$ fields are thus entirely symmetrical between positrons and electrons; $\varphi$ could be taken as the starting-point and $\psi$ derived from it, just as easily as vice versa.

The commutation rules (311) become

<a id="eq-318a"></a>

$$
\begin{aligned} \{ b_{ku} ,b_{k^{\prime} u^{\prime}}^{*} \} &= \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{uu^{\prime}} \\ \left\{ b_{kv}^{C} ,b_{k^{\prime} v^{\prime}}^{* C} \right\} &= \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{vv^{\prime}} \\ \left\{ b_{kv^{\prime}}^{C} ,b_{ku}^{*} \right\} &= 0 , \quad \text{ etc } . \end{aligned} \tag{318a}
$$

These are also symmetrical between electron and positron.

### <a id="s-the-hamiltonian"></a>The Hamiltonian

The Hamiltonian $H$ has commutation rules with the emission and absorption operators as for the Maxwell <a id="ix-180">field</a>. These rules come straight from the Heisenberg equations of motion for $\psi$ and $\varphi$. For any electron state $u$ or positron $v$

<a id="eq-319"></a>

$$
\begin{aligned} \boldsymbol{[} b_{ku} ,H \boldsymbol{]} &= E_{k} b_{ku} \quad \boldsymbol{[} b_{kv}^{C} ,H \boldsymbol{]} = E_{k} b_{kv}^{C} \\ \boldsymbol{[} b_{ku}^{*} ,H \boldsymbol{]} &= - E_{k} b_{ku}^{*} \quad \boldsymbol{[} b_{kv}^{* C} ,H \boldsymbol{]} = - E_{k} b_{kv}^{* C} \end{aligned} \tag{319}
$$

Hence the Hamiltonian of the theory is

<a id="eq-320"></a>

$$
H = \int d^{3} \boldsymbol{k} E_{k} \left\{ \sum_{u^{+}} b_{ku}^{*} b_{ku} + \sum_{v^{+}} b_{kv}^{* C} b_{kv}^{C} \right\} \tag{320}
$$

as one can verify at once by substituting into (319).

The additive constant is again chosen so that $\langle H \rangle_{o}$, the expectation value of $H$ in the vacuum state, is zero. This eliminates a possible arbitrary additive constant from $H$.

In (317), (318) and (320) there is complete symmetry between electrons and positrons. The theory could just have well been built up from the positron as the fundamental particle, instead of from the electron.

But for practical calculations we shall generally not use (317), (318), (320). It is generally easier to work with the unsymmetrical form of the theory, with the fields $\psi$ and $\overline{\psi}$.

### <a id="s-failure-of-theory-with-commuting-fields"></a>Failure of Theory with Commuting Fields

Suppose we had taken the theory up to this point, only assuming $\psi$ and $\overline{\psi}$ to be ordinary commuting fields. Then $u$ and $v$ would be taken to be quantities commuting with all $\psi$ and $\overline{\psi}$, in the relations (296). Thus (298) and (299) would still be true only with commutators everywhere instead of anticommutators. Likewise (310) and (311). However in this symmetric representation, instead of (318a) we should have

<a id="eq-321"></a>

$$
\begin{aligned} \boldsymbol{[} b_{ku} ,b_{k^{\prime} u^{\prime}}^{*} \boldsymbol{]} &= \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{uu^{\prime}} \\ \boldsymbol{[} b_{kv}^{C} ,b_{k^{\prime} v^{\prime}}^{* C} \boldsymbol{]} &= - \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{vv^{\prime}} \end{aligned} \tag{321}
$$

And then the Hamiltonian instead of being given by (320) would have to be

<a id="eq-322"></a>

$$
H = \int d^{3} \boldsymbol{k} E_{k} \left\{ \sum_{u^{+}} b_{ku}^{*} b_{ku} - \sum_{v^{+}} b_{kv}^{* C} b_{kv}^{C} \right\} \tag{322}
$$

So positrons would actually be particles of negative energy, like the negative energy electrons of the one-electron theory. This is physically inadmissible.

Thus the use of anticommuting fields is the only thing which gives us a proper positive energy for the positrons. This is reasonable, because the intuitive Dirac hole theory can only work by virtue of the Pauli exclusion principle, and the exclusion principle is a feature of anticommuting fields.

<a id="p65"><sup>[p. 65]</sup></a>

### <a id="s-the-exclusion-principle"></a>The Exclusion Principle

Take any creation operator $b_{ku}^{*}$. As a special case of (310) we have identically

<a id="eq-323"></a>

$$
b_{ku}^{*} b_{ku}^{*} = 0 \tag{323}
$$

Given any state Ψ, the result of creating _two_ electrons with frequency $k$ and spin $u$ in that state is $b_{ku}^{*} b_{ku}^{*}$Ψ = 0. Thus there are no states in which two electrons have the same momentum and spin. So we have the Pauli exclusion principle valid for both electrons and positrons. Also, an electron and a positron do not exclude each other.

It is a very great success of the general <a id="ix-181">field</a> theory, that it has given us the Pauli principle automatically, not by special hypothesis as in the old particle theory of electrons.<sup><a id="fnref-45" href="#fn-45">45</a></sup>

The most general state of the fields is described by specifying for each electron and positron state the number of particles occupying it. This number in each case can take just the two values 0 and 1.

### <a id="s-the-vacuum-state-2"></a>The Vacuum State

The vacuum state Ψ$o$ is defined by

<div align="center">

$b_{ku}$Ψ$o$ = 0 and thus Ψ$\frac{*}{o} b_{ku}^{*}$ = 0 for electron states $u$

</div>

<a id="eq-323a"></a>

$$
b_{ku}^{*} \Psi_{o} = 0 \text{ and thus } \quad \Psi_{o}^{*} b_{ku} = 0 \text{ for positron states } u \tag{323a}
$$

So the vacuum expectation value of products of emission and absorption operators are given by (311). We find using (323a)

<a id="eq-324"></a>

$$
\begin{aligned} \langle b_{ku} b_{k^{\prime} v} \rangle_{o} &= \langle b_{ku}^{*} b_{k^{\prime} v}^{*} \rangle_{o} = 0 \\ \langle b_{ku} b_{k^{\prime} v}^{*} \rangle_{o} &= \Theta_{u} \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{uv} \\ \langle b_{ku}^{*} b_{k^{\prime} v} \rangle_{o} &= ( 1 - \Theta_{u} ) \delta^{3} ( \boldsymbol{k} - \boldsymbol{k}^{\prime} ) \delta_{uv} \end{aligned} \tag{324}
$$

where

$$
\Theta_{u} = \begin{cases} = 1 & \text{ for electron states } u , \\ = 0 & \text{ for positron states } . \end{cases}
$$

Hence by (306) and (309) the expectation value $\left\langle \psi_{\alpha} ( z ) \overline{\psi}_{\beta} ( y ) \right\rangle_{o}$ is just the part of the anti-commutator $\{ \psi_{\alpha} ( z ) , \overline{\psi}_{\beta} ( y ) \}$ which contains positive frequencies exp$[ ik \cdot ( z - y )$], with $k_{o} >$ 0. Thus, similarly to (221)

<a id="eq-325"></a>

$$
\left\langle \psi_{\alpha} ( z ) \overline{\psi}_{\beta} ( y ) \right\rangle_{o} = - iS_{\alpha \beta}^{+} ( z - y ) \tag{325}
$$

<a id="eq-326"></a>

$$
S^{+} ( x ) = \frac{1}{( 2 \pi )^{3}} \int e^{ik \cdot x} ( k / + i\mu ) \delta ( k^{2} + \mu^{2} ) \Theta ( k ) d^{4} k \tag{326}
$$

where

$$
\Theta ( x ) = \begin{cases} = + 1 & \text{ for } x_{0} > 0 \\ = 0 & \text{ for } x_{0} < 0 \end{cases}
$$

writing as before

<a id="eq-327"></a>

$$
S^{+} = \tfrac{1}{2} \left( S - iS^{( 1 )} \right) \quad S^{-} = \tfrac{1}{2} \left( S + iS^{( 1 )} \right) \tag{327}
$$

<a id="eq-328"></a>

$$
\left\langle \psi_{\beta} ( y ) \psi_{\alpha} ( z ) \right\rangle_{o} = - iS_{\alpha \beta}^{-} ( z - y ) \tag{328}
$$

<a id="eq-329"></a>

$$
\left\langle \boldsymbol{[} \psi_{\alpha} ( z ) , \overline{\psi}_{\beta} ( y ) \boldsymbol{]} \right\rangle_{o} = - S_{\alpha \beta}^{( 1 )} ( z - y ) \tag{329}
$$

<a id="eq-330"></a>

$$
S^{( 1 )} ( x ) = \frac{i}{( 2 \pi )^{3}} \int e^{ik \cdot x} ( k / + i\mu ) \delta ( k^{2} + \mu^{2} ) d^{4} k \tag{330}
$$

<a id="p66"><sup>[p. 66]</sup></a>

<a id="eq-330a"></a>

$$
S^{-} ( x ) = - \frac{1}{( 2 \pi )^{3}} \int e^{ik \cdot x} ( k / + i\mu ) \delta ( k^{2} + \mu^{2} ) \Theta ( - k ) d^{4} k \tag{330a}
$$

These results for the Dirac theory without electromagnetic interaction will be used a lot when we come to the complete quantum electrodynamics with both Dirac and Maxwell fields quantized. Meanwhile, we should say a little about the theory of quantized Dirac particles in a given c-number Maxwell field.

## <a id="s-iii-field-theory-of-dirac-electron-in-external-field"></a>III. Field Theory of Dirac Electron in External Field

Lagrangian

<a id="eq-331"></a>

$$
L = L_{D} - ie \overline{\psi} / A^{e} \psi \tag{331}
$$

Field equations

<a id="eq-332"></a>

$$
\left\{ \sum_{\lambda} \gamma_{\lambda} \left( \frac{\partial}{\partial x_{\lambda}} + \frac{ie}{\hbar c} A_{\lambda}^{e} \right) + \mu \right\} \psi = 0 \tag{332}
$$

<a id="eq-333"></a>

$$
\sum_{\lambda} \left( \frac{\partial \overline{\psi}}{\partial x_{\lambda}} - \frac{ie}{\hbar c} A_{\lambda}^{e} \overline{\psi} \right) \gamma_{\lambda} - \mu \overline{\psi} = 0 \tag{333}
$$

These equations are still _linear_, the $A_{\mu}^{e}$ being given functions of position. This makes the theory still simple.

### <a id="s-covariant-commutation-rules-2"></a>Covariant Commutation Rules

Because of the linearity, a change $\overline{v} \psi ( z )$ in $L$ makes no difference to the field equation for $\psi ( y )$. Therefore for every two space-time points $y$ and $z$ we still have, as in (298),

<a id="eq-334"></a>

$$
\{ \psi_{\alpha} ( z ) ,\psi_{\beta} ( y ) \} = \{ \psi_{\alpha} ( z ) , \overline{\psi}_{\beta} ( y ) \} = 0 \tag{334}
$$

Beyond this point not much can be done with the theory for general time-dependent potentials $A_{\mu}^{e}$. In practice when we have time-dependent $A_{\mu}^{e}$ we always use perturbation theory starting from free field formalism, assuming the $A_{\mu}^{e}$ to be small, or else use special tricks for particular problems.

The important practical cases in which the $A_{\mu}^{e}$ are not small are always those in which the $A_{\mu}^{e}$ are _time-independent_ in a particular <a id="ix-155">Lorentz</a> system. Examples: electrons bound in atoms by static Coulomb forces, electrons moving in constant macroscopic electric and magnetic fields.

So we assume $A_{\mu}^{e} = A_{\mu}^{e} ( r )$ time-independent. We also assume the $A_{\mu}^{e}$ to be physically well-behaved so that the stationary eigenvalue equation

<a id="eq-335"></a>

$$
E_{n} \psi_{n} = \left\{ - e \Phi + \sum_{j = 1}^{3} \left( - i \hbar c \frac{\partial}{\partial x_{j}} + eA_{j}^{e} \right) \alpha^{j} + mc^{2} \beta \right\} \psi_{n} \tag{335}
$$

regarded as an equation for the c-number Dirac wave-function $\psi_{n} ( r )$ has a complete set of eigenfunctions $\psi_{n}$ with eigenvalues $E_{n}$; the spectrum may be either discrete or continuous or mixed. Equation (335) is derived from (332) by substituting into it the particular function

<a id="eq-336"></a>

$$
\psi = \psi_{n} ( r ) \exp \left\{ - i \frac{E_{n}}{\hbar} t \right\} \tag{336}
$$

We assume further the potentials to be such that the eigenfunctions $\psi_{n}$ separate clearly into two classes, the $\psi_{n +}$ with positive $E_{n}$ and the $\psi_{n -}$ with negative $E_{n}$. This is again true for all physically occurring potentials, though it would fail for the Coulomb field of a point nucleus with charge $Z>$ 137.

The time-independent potentials make the problem essentially non-covariant, and so we shall use the non-covariant notations in developing the theory. We write the equations as if all the levels $n$ are _discrete_, thus $\sum n$ means a sum over discrete levels plus an integral over continuous levels suitably normalized. We are now chiefly interested in discrete levels, and so we need not bother to write the formulae for normalizing <a id="p67"><sup>[p. 67]</sup></a> the continuous levels explicitly. This makes the picture apparently simpler than the free-particle theory, where the normalization of continuous levels was done at every stage accurately; the simplicity is only apparent, because we shall just be ignoring the complications arising from continuous levels.

The general solution of the <a id="ix-182">field</a> equations (332) is

<a id="eq-337"></a>

$$
\psi ( r,t ) = \sum_{n} b_{n} \psi_{n} ( r ) \exp \left\{ - i \frac{E_{n}}{\hbar} t \right\} \tag{337}
$$

where the $b_{n}$ are operators independent of $r$ and $t$, and $E_{n}$ can be both positive and negative.

By (334) we have

$$
\{ b_{m} ,b_{n} \} = 0 \quad \{ b_{m}^{*} ,b_{n}^{*} \} = 0
$$

where $b_{m}^{*}$ is the Hermitian conjugate of $b_{m}$. We take the $\psi_{n}$ normalized so that

<a id="eq-338"></a>

$$
\int \psi_{m}^{*} ( r ) \psi_{n} ( r ) d^{3} \boldsymbol{r} = \delta_{nm} \tag{338}
$$

i.e.in the usual non-covariant way. Note especially in (339) that we are treating all levels as if they were discrete; this can be done for example by enclosing our whole system in a finite box.

We want still to find the continuous rules between $\psi$ and $\psi^{*}$, or between $b_{n}$ and $b_{m}^{*}$.<sup><a id="fnref-46" href="#fn-46">46</a></sup> Suppose we add to the Lagrangian (331)

<a id="eq-339"></a>

$$
\delta L ( r,t ) = \epsilon \delta ( t - t_{o} ) \psi^{*} ( r,t_{o} ) \psi_{n} ( r ) u \tag{339}
$$

where $u$ is an operator anticommuting with $\psi$ and $\psi^{*}$ as in (288). This produces a change in the field equation for $\psi$ which now becomes

<a id="eq-340"></a>

$$
\left\{ \sum_{\lambda} \gamma_{\lambda} \left( \frac{\partial}{\partial x_{\lambda}} + \frac{ie}{\hbar c} A_{\lambda}^{e} \right) + \mu \right\} \psi = \frac{\epsilon}{\hbar c} \delta ( t - t_{o} ) \beta \psi_{n} ( r ) u \tag{340}
$$

So the change $\delta \psi$ produced in $\psi$ by the addition of $\delta L$ satisfies (341) with the initial condition $\delta \psi ( r,t )$ = 0 for $t<t_{o}$. (Compare with (198) and (290).) Now the solution of (341) will obviously be of the form

<a id="eq-341"></a>

$$
\delta \psi = a ( t ) \psi_{n} ( r ) \tag{341}
$$

where $a ( t )$ is a function of $t$ only, because the right side of the linear equation also has this form. Substituting (342) into (341) and using (335) we have

<a id="eq-342"></a>

$$
\left( i \hbar \frac{\partial}{\partial t} - E_{n} \right) a ( t ) = - \epsilon \delta ( t - t_{o} ) u \tag{342}
$$

and hence

<a id="eq-343"></a>

$$
\delta \psi = \frac{i\epsilon}{\hbar} \Theta ( t - t_{o} ) \psi_{n} ( r ) \exp \left\{ - i \frac{E_{n}}{\hbar} ( t - t_{o} ) \right\} u \tag{343}
$$

as we can verify using

$$
\frac{d}{dt} \Theta ( t - t_{o} ) = \delta ( t - t_{o} )
$$

Integrating (340) over space-time gives by (339)

<a id="eq-344"></a>

$$
c \int \int \delta L ( r,t ) d^{3} \boldsymbol{r} dt = \epsilon cb_{n}^{*} \exp \left\{ i \frac{E_{n}}{\hbar} t_{o} \right\} u \tag{344}
$$

For $t>t_{o}$, (343) is the change produced in $\psi ( r,t )$ by adding (344) to the action integral. Hence by the Peierls commutation rule, using (193), (343) and (344),

$$
\boldsymbol{[} b_{n}^{*} u,\psi ( r,t ) \boldsymbol{]} = - \psi_{n} ( r ) \exp \left\{ - i \frac{E_{n}}{\hbar} t \right\}
$$

<a id="p68"><sup>[p. 68]</sup></a>

and therefore

<a id="eq-345"></a>

$$
\{ b_{n}^{*} ,\psi ( r,t ) \} = \psi_{n} ( r ) \exp \left\{ - i \frac{E_{n}}{\hbar} t \right\} \tag{345}
$$

because by assumption the $u$’s <a id="ix-39">anticommute</a> with the $\psi$’s. The time $t_{o}$ no longer appears in (345), checking the consistency of the method.

Multiplying (345) by $\psi_{n}^{*} ( r^{\prime} )$ exp$\{- iE_{n} t^{\prime} / \hbar \}$ and summing over $n$, we have<sup><a id="fnref-47" href="#fn-47">47</a></sup>

<a id="eq-346"></a>

$$
\{ \psi_{\alpha} ( r,t ) ,\psi_{\beta}^{*} ( r^{\prime} ,t^{\prime} ) \} = \sum_{n} \psi_{n\alpha} ( r ) \psi_{n\beta}^{*} ( r^{\prime} ) \exp \left\{ - i \frac{E_{n}}{\hbar} ( t - t^{\prime} ) \right\} \tag{346}
$$

This is the general commutation rule which reduces to (299) in the special case of free particles.

Multiplying (345) by $\psi_{m}^{*} ( r )$ and integrating over $r$, we have

<a id="eq-347"></a>

$$
\{ b_{m} ,b_{n}^{*} \} = \delta_{nm} \tag{347}
$$

which is identical to (311) in the case of free particles when normalizations are properly treated.

### <a id="s-the-hamiltonian-2"></a>The Hamiltonian

As before, the $b_{n +}$ are absorption operators for electrons and the $b_{n -}^{*}$ absorption operators for positrons, only the electrons and positrons being defined by the bound <a id="ix-81">wave-functions</a>. The vacuum state Ψ$o$ is given by

<a id="eq-348"></a>

$$
b_{n +} \Psi_{0} = 0 \quad b_{n -}^{*} \Psi_{0} = 0 \tag{348}
$$

And the total Hamiltonian of the <a id="ix-160">system</a>, in order to give the correct commutators with the $b_{n}$ and the $b_{n}^{*}$ and also to have zero expectation value in the vacuum, is

<a id="eq-349"></a>

$$
H = \sum_{n +} E_{n} b_{n}^{*} b_{n} - \sum_{n -} E_{n} b_{n} b_{n}^{*} \tag{349}
$$

<a id="eq-350"></a>

$$
= \sum_{n +} E_{n} b_{n}^{*} b_{n} + \sum_{n -} | E_{n} | b_{n} b_{n}^{*} \tag{350}
$$

From this Hamiltonian it is clear that the system is just a superposition of non-interacting particle-states. In each particle-state independently there is a number of particles given by

<div align="center">

$N_{n} = b_{n}^{*} b_{n}$ for electron states<br>
$N_{n} = b_{n} b_{n}^{*}$ for positron states

</div>

From the commutation rules (338) and (347)

<a id="eq-351"></a>

$$
N_{n}^{2} = N_{n} \tag{351}
$$

so each $N_{n}$ has just the two eigenvalues 0, 1. This describes exactly the physical situation in a many-electron atom where each atomic level may be full or empty independent of the others.

If each $N$ is represented by the (2 $\times$ 2) diagonal matrix

<a id="eq-352"></a>

$$
N_{n} = \begin{pmatrix} 0 & 0 \\ 0 & 1 \end{pmatrix} \tag{352}
$$

then we have

<a id="eq-353"></a>

$$
\begin{aligned} b_{n +} &= \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix} \quad b_{n -}^{*} = \begin{pmatrix} 0 & 0 \\ 1 & 0 \end{pmatrix} \\ b_{n -} &= \begin{pmatrix} 0 & 0 \\ 1 & 0 \end{pmatrix} \quad b_{n -}^{*} = \begin{pmatrix} 0 & 1 \\ 0 & 0 \end{pmatrix} \end{aligned} \tag{353}
$$

<a id="p69"><sup>[p. 69]</sup></a>

This gives an explicit matrix representation for the operators. Each of the states $n$ has its own two-valued row and column index. Thus for an atom with $M$ levels altogether the operators would be represented by $( 2^{M} \times 2^{M} )$ matrices.

Once having got the Hamiltonian (350) and the stationary states $\psi_{n}$ the theory of many-electron systems is completely straightforward. We see that the levels of the hydrogen atom given by the one-electron Dirac theory are still exactly valid in this many-electron theory. Only now the Hamiltonian (350) having positive eigenvalues, the negative energy states give us no disasters. The positrons appear with positive energy, so that all the results of the Dirac theory are given to us simply and automatically.

### <a id="s-antisymmetry-of-the-states"></a>Antisymmetry of the States

We know that in elementary quantum theory of many-electron systems we have to represent the system wave-functions by determinants of one-particle wave-functions, so as to make the system wave-functions always antisymmetric in the particle coordinates. We shall no longer need to make any such arbitrary choice of wave-functions in the field theory, all the results of the antisymmetry being given automatically by the theory.

For example, consider an atom with 2 electrons in states $\psi_{1}$ and $\psi_{2}$, all other states being empty. Then the state of the system is given by

<a id="eq-354"></a>

$$
\Psi = b_{1}^{*} b_{2}^{*} \Psi_{o} \tag{354}
$$

where Ψ$o$ is the vacuum state. In (354) there is no arbitrariness; an interchange of the indices 1 and 2 will only change Ψ into $-$Ψ which means no physical change. Now consider a 2-particle interaction operator

<a id="eq-355"></a>

$$
V = \tfrac{1}{2} \int \int d^{3} \boldsymbol{r}_{1} d^{3} \boldsymbol{r}_{2} \{ \psi^{*} ( r_{1} ) \psi ( r_{1} ) \} V ( r_{1} - r_{2} ) \{ \psi^{*} ( r_{2} ) \psi ( r_{2} ) \} \tag{355}
$$

For example, $V$ may be the Coulomb potential between two electrons, which is not included in the Lagrangian (331). The $\frac{1}{2}$ is put in so as to count each pair of points $r_{1} , r_{2}$ only once. We calculate the matrix element of $V$ for a transition from Ψ to a state

$$
\Psi^{\prime} = b_{3}^{*} b_{4}^{*} \Psi_{o}
$$

where the 2 electrons are in two other states $\psi_{3}$ and $\psi_{4}$. This matrix element is

<a id="eq-356"></a>

$$
M = ( \Psi_{o}^{*} ,b_{4} b_{3} Vb_{1}^{*} b_{2}^{*} \Psi_{o} ) \tag{356}
$$

Expanding $V$ by (337) into a sum of products of $b_{n}$ and $b_{n}^{*}$, a contribution to (356) will come only from the 4 terms in $V$ proportional to $b_{1} b_{2} b_{3}^{*} b_{4}^{*}$. Using the <a id="ix-40">anticommutation</a> rules, we have

<a id="eq-357"></a>

$$
\begin{aligned} ( \Psi^{\prime *} ,b_{1} b_{2} Vb_{3}^{*} b_{4}^{*} \Psi ) &= - 1 \\ ( \Psi^{\prime *} ,b_{1} b_{2} Vb_{4}^{*} b_{3}^{*} \Psi ) &= 1 , \quad \text{ etc } . , \end{aligned} \tag{357}
$$

Hence adding up the 4 terms

<a id="eq-358"></a>

$$
\begin{aligned} M &= \int \int d^{3} \boldsymbol{r}_{1} d^{3} \boldsymbol{r}_{2} V ( r_{1} - r_{2} ) \{\{ \psi_{3}^{*} ( r_{1} ) \psi_{1} ( r_{1} ) \}\{ \psi_{4}^{*} ( r_{2} ) \psi_{2} ( r_{2} ) \} \\ & \quad -\{ \psi_{3}^{*} ( r_{1} ) \psi_{2} ( r_{1} ) \}\{ \psi_{4}^{*} ( r_{2} ) \psi_{2} ( r_{2} ) \}\} \end{aligned} \tag{358}
$$

This is exactly the result, direct minus exchange interaction, which would be given by using antisymmetrized wave-functions.

The field theory thus gives the full force of the Fermi statistics for electrons. And we could have shown in the same way that it gives the Bose statistics for photons.

<a id="p70"><sup>[p. 70]</sup></a>

### <a id="s-polarization-of-the-vacuum"></a>Polarization of the Vacuum

Because of the possibility of exciting the vacuum by creating a positron-electron pair, the vacuum behaves like a dielectric, just as a solid has dielectric properties in virtue of the possibility of its atoms being excited to excited states by Maxwell radiation. This effect does not depend on the quantizing of the Maxwell field, so we calculate it using classical fields.

Like a real solid dielectric, the vacuum is both non-linear and dispersive, i.e. the dielectric constant depends on the field intensity and on the frequency. And for sufficiently high frequencies and field intensities it has a complex dielectric constant, meaning it can absorb energy from the Maxwell field by real creation of pairs.

We calculate the dielectric constant only in the linear region, i.e.assuming weak fields. The critical field for this problem is

<a id="eq-359"></a>

$$
E_{c} = \frac{m^{2} c^{3}}{e \hbar} \approx 1 0^{1 6} \frac{\text{ Volts }}{\text{ cm }} \quad ( \text{ from } eE \cdot \frac{\hbar}{mc} \approx mc^{2} ) \tag{359}
$$

and in fact the linear theory is good enough for almost all problems. The important case where it is _not_ good enough is the propagation of photons through the intense Coulomb field round a heavy nucleus like lead. Then the non-linearity produces a scattering of photons which is small but has been detected experimentally by Wilson. <a id="refback-15-1"></a>[[15]](#ref-15)

We calculate the dispersive effects exactly, i.e. with no restriction on the frequency. Since the treatment is linear, the imposed Maxwell field may be supposed to be given by the potentials of a plane wave whose amplitude increases slowly with time

<a id="eq-360"></a>

$$
A_{\mu}^{e} ( x ) = e_{\mu} \exp \{ iq \cdot x + \delta_{o} x_{o} \} \tag{360}
$$

Here $e$ and $q$ are given vectors, $\delta_{o}$ is a small positive number. This exponentially increasing amplitude is put in so that the potential $A_{\mu}^{e}$ effectively acts only for a finite time before any given time at which observations may be made. This enables us to fix the initial conditions of the problem unambiguously. At the end of the calculation we shall go to the limit $\delta_{o}$ = 0.

The vacuum polarization is the effect of the fluctuations of the quantized electron-positron field on a given Maxwell field. The Lamb shift is the effect of the fluctuations of the quantized Maxwell field on a given electron. The two effects are just opposites of each other, the roles of the two fields being interchanged. Thus we can treat the vacuum polarization now conveniently with the theory of the quantized electron field alone. The treatment will be relativistic and so more correct than the treatment given for the Lamb shift. Later, to have a complete theory of both effects, we shall quantize both fields together and consider the reaction of each on the other.

Historically the electron self-energy (Lamb shift) and the vacuum polarization were the two problems on which the theory broke down because of divergences. Schwinger showed that the vacuum polarization could be calculated and was finite, if one used the same kind of renormalization idea that made the Lamb shift also finite.

The electron field operator $\psi_{H}$ in the field (360) satisfies (332). Here $\psi_{H}$ is the operator in the Heisenberg representation. Now a solution of (332) correct to first order in $A_{\mu}^{e}$ is

<a id="eq-361"></a>

$$
\psi_{H} ( x ) = \psi ( x ) + \frac{ie}{\hbar c} \int dx^{\prime} S_{R} ( x - x^{\prime} ) A /^{e} ( x^{\prime} ) \psi ( x^{\prime} ) \tag{361}
$$

Here $S_{R}$ is given by (293), (301), and (303), and $\psi ( x )$ is a solution of the free-field equation (286). In fact $\psi ( x )$ is the field operator of the interaction representation, when the effects of $A_{\mu}$ are represented in the wave-function instead of in the operators. Using the retarded potential in (361) means that the unperturbed states are specified in the past, as the initial states upon which $A_{\mu}^{e}$ later produces effects. Thus the vacuum state defined by (323a) is the state in which initially no electrons or positrons are present. This is the state which we wish to study, and we call it Ψ$o$.

Using the interaction representation operators $\psi ( x )$, Ψ$o$ is the vacuum state and remains so for all time; the physical state is initially Ψ$o$ but does not remain Ψ$o$. Using the Heisenberg operators $\psi_{H} ( x )$, Ψ$o$ is the <a id="p71"><sup>[p. 71]</sup></a> physical state for all time; it is initially the vacuum state but does not remain so. In the remote past as $x_{0} \to -\infty$, because $S_{R}$ is a retarded potential $\psi_{H} ( x )$ and $\psi ( x )$ become identical.

The expression (361) is useful because we know how to calculate matrix elements of $\psi ( x )$ from the state Ψ$o$, whereas the matrix elements of $\psi_{H}$ do not have any simple form. We also need the adjoint equation

<a id="eq-362"></a>

$$
\overline{\psi}_{H} ( x ) = \overline{\psi} ( x ) + \frac{ie}{\hbar c} \int dx^{\prime} \overline{\psi} ( x^{\prime} ) A /^{e} ( x^{\prime} ) S_{A} ( x^{\prime} - x ) \tag{362}
$$

where $S_{A} ( x )$ is given by (295).

The total current operator to first order in $A_{\mu}$ is

<a id="eq-363"></a>

$$
\begin{aligned} j_{\mu H} ( x ) &= - iec \overline{\psi}_{H} ( x ) \gamma_{\mu} \psi_{H} ( x ) \\ &= j_{\mu} ( x ) + \frac{e^{2}}{\hbar} \int d^{4} x^{\prime} \left\{ \psi ( x ) \gamma_{\mu} S_{R} ( x - x^{\prime} ) A /^{e} ( x^{\prime} ) \psi ( x^{\prime} ) + \overline{\psi} ( x^{\prime} ) A /^{e} ( x^{\prime} ) S_{A} ( x^{\prime} - x ) \gamma_{\mu} \psi ( x ) \right\} \end{aligned} \tag{363}
$$

Here

<a id="eq-364"></a>

$$
j_{\mu} ( x ) = - iec \overline{\psi} ( x ) \gamma_{\mu} \psi ( x ) \tag{364}
$$

is the current operator of the interaction representation. The vacuum expectation value

<a id="eq-365"></a>

$$
\begin{aligned} ( \Psi_{o}^{*} j_{\mu} ( x ) \Psi_{o} ) &= \langle j_{\mu} ( x ) \rangle_{o} \\ &= - iec \left\langle \sum_{\alpha ,\beta} \overline{\psi}_{\beta} ( x ) ( \gamma_{\mu} )_{\beta \alpha} \psi_{\alpha} ( x ) \right\rangle_{o} \\ &= - iec \sum_{\alpha ,\beta} ( \gamma_{\mu} )_{\beta \alpha} \left\langle \psi_{\beta} ( x ) \psi_{\alpha} ( x ) \right\rangle_{o} \end{aligned} \tag{365}
$$

is given by (328)

<a id="eq-366"></a>

$$
\begin{aligned} \langle j_{\mu} ( x ) \rangle_{o} &= - ec \operatorname{Spur} \int \left\{ \gamma_{\mu} S^{-} ( 0 ) \right\} \\ &= \frac{ec}{( 2 \pi )^{3}} \quad d^{3} \boldsymbol{k} \delta ( k^{2} + \mu^{2} ) \Theta ( - k ) \operatorname{Spur} \{ \gamma_{\mu} [ / k - i\mu ] \} \\ &= \frac{4 ec}{( 2 \pi )^{3}} \int d^{3} \boldsymbol{k} \delta ( k^{2} + \mu^{2} ) \Theta ( - k ) k_{\mu} \end{aligned} \tag{366}
$$

This is a highly divergent integral and is mathematically meaningless. This is one of the difficulties of the theory about which one can argue for a long time.

However there is no doubt that correct physical results are obtained by putting $\langle j_{\mu} ( x ) \rangle_{o}$ = 0 simply. There are two good reasons for doing this.

(1) Physical. $\langle j_{\mu} ( x ) \rangle_{o}$, being the expectation value of the charge-current in the vacuum in the absence of all external fields, is known to be zero experimentally. Therefore if we calculated $\langle j_{\mu} ( x ) \rangle_{o}$ and found it were not zero we should simply define the current operator to be $j_{\mu} - \langle j_{\mu} \rangle_{o}$. With this definition the expectation value would become zero automatically.

(2) Mathematical. $\langle j_{\mu} ( x ) \rangle_{o}$ as we have calculated it is a vector, each component of which is a number quite independent of the coordinate system. There does not exist such a vector invariant under Lorentz transformations, except for this zero vector. Therefore $\langle j_{\mu} ( x ) \rangle_{o}$ = 0 is the only assumption we _can_ make which would keep the theory invariant.

This is a simple example of a method which has often to be used in quantum electrodynamics. When a calculation leads to a divergent integral or a mathematically indeterminate expression, we use physical arguments or arguments of Lorentz invariance to find a definite value for the quantity we are unable to calculate. This is the reason for the great success of the covariant formulation of electrodynamics introduced by Schwinger.

So using this principle we have by (328)

<a id="eq-367"></a>

$$
\langle j_{\mu H} ( x ) \rangle_{o} = - \frac{ie^{2}}{\hbar} \int d^{4} x^{\prime} \operatorname{Spur} \{ A /^{e} ( x^{\prime} ) S^{-} ( x^{\prime} - x ) \gamma_{\mu} S_{R} ( x - x^{\prime} ) + A /^{e} ( x^{\prime} ) S_{A} ( x^{\prime} - x ) \gamma_{\mu} S^{-} ( x - x^{\prime} ) \} \tag{367}
$$

<a id="p72"><sup>[p. 72]</sup></a>

We use the momentum representation (303) for $S_{R}$. But instead of taking the contour along the real axis for $k_{0}$ we can use a path of integration running along a straight line parallel to the real axis a distance $\delta_{o}$ above it. That gives the momentum representation

<a id="eq-368"></a>

$$
e^{- \delta_{0}} S_{R} ( x ) = \frac{i}{( 2 \pi )^{4}} \int e^{ik \cdot x} \frac{/ k + i/\delta + i\mu}{( k + i\delta )^{2} + \mu^{2}} d^{4} k \tag{368}
$$

where $\delta_{0}$ is any positive real number, $\delta$ is the vector with components (0$, 0 , 0 ,\delta_{0} )$ and the integral is along the real axis. The poles of (368) in the $k_{0}$ plane are displaced away from the real axis and so the integrand is free from singularities on the path of integration. Similarly

<a id="eq-369"></a>

$$
e^{+ \delta_{0}} S_{A} ( x ) = \frac{i}{( 2 \pi )^{4}} \int e^{ik \cdot x} \frac{/ k - i/\delta + i\mu}{( k - i\delta )^{2} + \mu^{2}} d^{4} k \tag{369}
$$

When using (368) and (369) we shall usually make $\delta_{0} \to$ 0 after carrying out the integrations, so that the convergence factors $e^{\pm \delta_{0} x_{0}}$ will tend to 1 for every finite $x$.

Thus the momentum representation of (367) will be

$$
\begin{aligned} \langle j_{\mu H} ( x ) \rangle_{o} &= - \frac{e^{2}}{\hbar} \frac{1}{( 2 \pi )^{7}} \int d^{4} x^{\prime} \int \int d^{4} k_{1} d^{4} k_{2} \exp \{ iq \cdot x^{\prime} + i ( k_{1} - k_{2} ) \cdot ( x^{\prime} - x ) + \delta_{0} x_{0} \}\times \\ & \quad \times \left\{ \operatorname{Spur} \{ /e ( k /_{1} + i\mu ) \gamma_{\mu} ( / k_{2} + i/\delta + i\mu ) \} \frac{\delta ( k_{1}^{2} + \mu^{2} ) \Theta ( - k_{1} )}{( k_{2} + i\delta )^{2} + \mu^{2}} \right. \\ & \quad \left. + \operatorname{Spur} \{ /e ( / k_{1} - i/\delta + i\mu ) \gamma_{\mu} ( / k_{2} + i\mu ) \} \frac{\delta ( k_{2}^{2} + \mu^{2} ) \Theta ( - k_{2} )}{( k_{1} - i\delta )^{2} + \mu^{2}} \right\} \end{aligned}
$$

The integration of $x^{\prime}$ is immediate and gives (2$\pi )$<sup>4</sup>$\delta^{4} ( k_{1} - k_{2} + q )$. Hence

<a id="eq-370"></a>

$$
\begin{aligned} \langle j_{\mu H} ( x ) \rangle_{o} &= - \frac{e^{2}}{( 2 \pi )^{3} \hbar} e^{iq \cdot x + \delta_{0} x_{0}} \times \\ & \quad \times \int d^{4} k \left\{ \operatorname{Spur} \{ /e ( / k + i\mu ) \gamma_{\mu} ( / k + q / + i/\delta + i\mu ) \} \frac{\delta ( k^{2} + \mu^{2} ) \Theta ( - k )}{( k + q + i\delta )^{2} + \mu^{2}} \right. \\ & \quad \left. + \operatorname{Spur} \{ /e ( k / - i/\delta + i\mu ) \gamma_{\mu} ( k / + q / + i\mu ) \} \frac{\delta \{ ( k + q )^{2} + \mu^{2} \} \Theta ( - k - q )}{( k - i\delta )^{2} + \mu^{2}} \right\} \end{aligned} \tag{370}
$$

Now consider the function<sup><a id="fnref-48" href="#fn-48">48</a></sup>

<a id="eq-371"></a>

$$
F_{\nu} ( k ) = \operatorname{Spur} \{ /e ( / k + i\mu ) \gamma_{\nu} ( / k + /q + i/\delta + i\mu ) \} \frac{1}{( k^{2} + \mu^{2} ) [ ( k + q + i\delta )^{2} + \mu^{2} ]} \tag{371}
$$

This has poles at 4 points in the $k_{0}$ plane

<a id="eq-372"></a>

$$
k_{0} = \pm \sqrt{| \boldsymbol{k} |^{2} + \mu^{2}} \quad k_{0} = - q_{0} - i\delta_{0} \pm \sqrt{| \boldsymbol{k} + \boldsymbol{q} |^{2} + \mu^{2}} \tag{372}
$$

The integral in the expression (370) is just the sum of the residues at the two points<sup><a id="fnref-49" href="#fn-49">49</a></sup>

<a id="eq-373"></a>

$$
k_{0} = - \sqrt{| \boldsymbol{k} |^{2} + \mu^{2}} \quad k_{0} = - q_{0} - i\delta_{0} - \sqrt{| \boldsymbol{k} + \boldsymbol{q} |^{2} + \mu^{2}} \tag{373}
$$

![Figure 4 (p. 72)](images/fig-p078-1.png)

_Figure 4 (p. 72)_

![Figure 5 (p. 72)](images/fig-p078-2.png)

_Figure 5 (p. 72)_

<a id="p73"><sup>[p. 73]</sup></a>

Hence

<a id="eq-374"></a>

$$
\langle j_{\nu H} ( x ) \rangle_{o} = \frac{ie^{2}}{( 2 \pi )^{4} \hbar} e^{iq \cdot x + \delta_{0} x_{0}} \int_{C} F_{\nu} ( k ) dk \tag{374}
$$

where $\int C$ means a contour integral in the $k_{0}$ plane drawn as shown in the diagram, going from $-\infty$ to +$\infty$ below the two poles (373) and above the other two poles, and including the upper semicircle of the $k_{0}$ plane at infinity. So long as $\delta_{0} >$ 0 the poles are always clearly separated from each other. Now the calculation really starts, with the evaluation of this integral (374). This will be typical of all the calculations which have to be done in <a id="ix-216">quantum electrodynamics</a> using the modern methods.

### <a id="s-calculation-of-momentum-integrals"></a>Calculation of Momentum Integrals

Write $J_{\nu} = \int_{C} F_{\nu} ( k ) dk$. Then $J_{\nu}$ is a vector function of the variables $\mu$ (the mass of the electron) and

<a id="eq-375"></a>

$$
Q = q + i\delta \tag{375}
$$

$J_{\nu}$ is certainly an analytic function of $\mu$ if $\delta >$ 0, and it is an analytic function of $Q$ if $\mu$ is large enough so that the poles (373) always lie to the left of the imaginary axis and the other two poles (372) to the right. Therefore we can calculate $J_{\nu}$ for large values of $\mu$ and $\delta$ = 0 so that $Q = q$.

To simplify (371) we take $\delta$ = 0 and use Feynman’s formula (_Phys. Rev._ **76** (1949) 785)

<a id="eq-376"></a>

$$
\frac{1}{ab} = \int_{0}^{1} dz \frac{1}{[ az + b ( 1 - z ) ]^{2}} \tag{376}
$$

which we can get at once simply from the definite integral of the RHS.<sup><a id="fnref-50" href="#fn-50">50</a></sup> Hence

<a id="eq-377"></a>

$$
\begin{aligned} J_{\nu} &= \int_{0}^{1} dz \int_{C} dk \operatorname{Spur} \{ /e ( / k + i\mu ) \gamma_{\nu} ( k / + /q + i\mu ) \} \frac{1}{\{ k^{2} + \mu^{2} + z ( 2 k \cdot q + q^{2} ) \}^{2}} \\ &= \int_{0}^{1} dz \int_{C} dk \operatorname{Spur} \{ /e ( / k - z/q + i\mu ) \gamma_{\nu} ( / k + ( 1 - z ) /q + i\mu ) \} \frac{1}{\{ k^{2} + \mu^{2} + ( z - z^{2} ) q^{2} \}^{2}} \end{aligned} \tag{377}
$$

In the last step a shift of origin was made in the $k$–integration replacing $k$ by $( k - zq )$. Again in (377) the poles in the $k_{0}$–plane for every $z$ are well separated by the imaginary axis, provided that $\mu$ is large. Evaluating the spur and dropping terms are odd in $k$, and using (33),<sup><a id="fnref-51" href="#fn-51">51</a></sup> Spur $\gamma_{\nu}$ = 0, Spur $\gamma_{\mu} \gamma_{\nu}$ = 4$\delta_{\mu \nu}$, we get

<a id="eq-378"></a>

$$
J_{\nu} = 4 \int_{0}^{1} dz \int_{C} dk \frac{e_{\nu} ( - k^{2} - \mu^{2} + ( z - z^{2} ) q^{2} ) + 2 ( e \cdot k ) k_{\nu} - 2 ( z - z^{2} ) ( e \cdot q ) q_{\nu}}{\{ k^{2} + \mu^{2} + ( z - z^{2} ) q^{2} \}^{2}} \tag{378}
$$

the odd terms going out because we can now take the $k_{0}$–integration straight up the imaginary axis from $- i \infty$ to $i \infty$ if we like. For the same reasons of symmetry we may replace

$$
e \cdot kk_{\nu} \quad \text{ by } \quad \tfrac{1}{4} k^{2} e_{\nu} \quad \text{ because } \quad e \cdot kk_{\nu} = \sum_{\lambda} e_{\lambda} k_{\lambda} k_{\nu} \to e_{\nu} k_{\nu} k_{\nu} \to \tfrac{1}{4} e_{\nu} k^{2}
$$

in the numerator, and obtain finally

<a id="eq-379"></a>

$$
J_{\nu} = 4 \int_{0}^{1} dz \int_{C} dk \frac{e_{\nu} \{- \tfrac{1}{2} k^{2} - \mu^{2} + ( z - z^{2} ) q^{2} \}- 2 ( z - z^{2} ) ( e \cdot q ) q_{\nu}}{\{ k^{2} + \mu^{2} + ( z - z^{2} ) q^{2} \}^{2}} \tag{379}
$$

This integral is still badly divergent. So we again use a physical argument to give a definite value to the most divergent part of it. The current operator both in interaction and in Heisenberg <a id="ix-128">representation</a> must satisfy

<a id="eq-380"></a>

$$
\sum_{\nu} \frac{\partial j_{\nu} ( x )}{\partial x_{\nu}} = 0 \tag{380}
$$

<a id="p74"><sup>[p. 74]</sup></a>

identically. Therefore (374) gives (since we now take $\delta$ = 0)

<a id="eq-381"></a>

$$
\sum_{\nu} q_{\nu} J_{\nu} = 0 \tag{381}
$$

which gives the relation

<a id="eq-382"></a>

$$
\int_{0}^{1} dz \int_{C} dk \frac{- \tfrac{1}{2} k^{2} - \mu^{2} - ( z - z^{2} ) q^{2}}{\{ k^{2} + \mu^{2} + ( z - z^{2} ) q^{2} \}^{2}} \equiv 0 \tag{382}
$$

This equation (382) is a guiding equation telling us that a certain divergent expression appearing in (379) is to be given the value zero in order to make physical sense. We are left with

<a id="eq-383"></a>

$$
J_{\nu} = 8 ( q^{2} e_{\nu} - e \cdot qq_{\nu} ) \int_{0}^{1} dz ( z - z^{2} ) \int_{C} \frac{dk}{\{ k^{2} + \mu^{2} + ( z - z^{2} ) q^{2} \}^{2}} \tag{383}
$$

For any positive Λ the integral

<a id="eq-384"></a>

$$
I_{\Lambda} = \int_{C} \frac{dk}{( k^{2} + \Lambda )^{3}} \tag{384}
$$

is convergent and can be evaluated by integrating for $k_{0}$ up the imaginary axis from $- i \infty$ to +$i \infty$. This gives (see the Appendix below)

<a id="eq-385"></a>

$$
I_{\Lambda} = i \int \int \int \int \frac{dk_{1} dk_{2} dk_{3} dk_{0}}{( k_{1}^{2} + k_{2}^{2} + k_{3}^{2} + k_{0}^{2} + \Lambda )^{3}} = 2 \pi^{2} i \int_{0}^{\infty} \frac{k^{3} dk}{( k^{2} + \Lambda )^{3}} = \pi^{2} i \int_{0}^{\infty} \frac{xdx}{( x + \Lambda )^{3}} = \frac{\pi^{2} i}{2 \Lambda} \tag{385}
$$

Hence integrating with respect to Λ

<a id="eq-386"></a>

$$
\int_{C} dk \left\{ \frac{1}{( k^{2} + \Lambda )^{2}} - \frac{1}{( k^{2} + \mu^{2} )^{2}} \right\} = \pi^{2} i \log \left( \frac{\mu^{2}}{\Lambda} \right) \tag{386}
$$

this integral being also convergent. However

$$
\int_{C} \frac{dk}{( k^{2} + \mu^{2} )^{2}}
$$

is logarithmically divergent for large $k$. Its value is

<a id="eq-387"></a>

$$
2 i\pi^{2} \log \left( \frac{k_{\max}}{\mu} \right) = 2 i\pi^{2} R \tag{387}
$$

where $R$ is the logarithmic factor and is independent of $q$.

Using (386) and (387) in (383), with Λ = $\mu^{2} + ( z - z^{2} ) q^{2}$, we get

<a id="eq-388"></a>

$$
J_{\nu} = 8 \pi^{2} i ( q^{2} e_{\nu} - e \cdot qq_{\nu} ) \left\{ \tfrac{1}{3} R - \int_{0}^{1} dz ( z - z^{2} ) \log \left[ 1 + \frac{( z - z^{2} ) q^{2}}{\mu^{2}} \right] \right\} \tag{388}
$$

This is the analytic formula for $J_{\nu}$ valid for large $\mu$ in which case the logarithm is real. We make the analytic continuation to small values of $\mu$ by writing $( q + i\delta )$ for $q$ in (388), treating $\delta_{0}$ as small and positive. Then $q^{2}$ becomes $q^{2} - 2 i\delta q_{0}$ and the logarithm becomes

$$
\log \left| 1 + \frac{( z - z^{2} ) q^{2}}{\mu^{2}} \right| + \begin{cases} 0 & \text{ for } \frac{( z - z^{2} ) q^{2}}{\mu^{2}} > - 1 \\ - i\pi \epsilon ( q_{0} ) & \text{ for } \frac{( z - z^{2} ) q^{2}}{\mu^{2}} < - 1 \end{cases}
$$

<a id="p75"><sup>[p. 75]</sup></a>

Writing now $z$ for 4$( z - z^{2} )$ and using (374), we go to the limit $\delta_{0}$ = 0 and find<sup><a id="fnref-52" href="#fn-52">52</a></sup>

<a id="eq-389"></a>

$$
\begin{aligned} \langle j_{\nu H} ( x ) \rangle_{o} &= - \frac{e^{2}}{2 \pi^{2} \hbar} \left( q^{2} e_{\nu} - e \cdot qq_{\nu} \right) e^{iq \cdot x} \times \\ & \quad \times \left\{ \tfrac{1}{3} R - \frac{1}{8} \int_{0}^{1} \frac{zdz}{\sqrt{1 - z}} \log \left| 1 + \frac{zq^{2}}{4 \mu^{2}} \right| + \frac{i\pi}{8} \epsilon ( q_{0} ) \int_{0}^{- 4 \mu^{2} /q^{2}} \frac{zdz}{\sqrt{1 - z}} \right\} \end{aligned} \tag{389}
$$

the last term being zero except when

<a id="eq-390"></a>

$$
q^{2} < - 4 \mu^{2} \tag{390}
$$

Now the external potential $A_{\nu}^{e} ( x )$ is associated with a classical external charge-current density

<a id="eq-391"></a>

$$
\begin{aligned} j_{\nu E} ( x ) &= - c \sum_{\lambda} \frac{\partial}{\partial x_{\lambda}} F_{\lambda \nu E} ( x ) \\ &= - c \sum_{\lambda} \left\{ \frac{\partial^{2}}{\partial x_{\lambda}^{2}} A_{\nu}^{e} ( x ) - \frac{\partial^{2}}{\partial x_{\nu} \partial x_{\lambda}} A_{\lambda}^{e} ( x ) \right\} \\ &= c \left\{ q^{2} e_{\nu} - e \cdot qq_{\nu} \right\} e^{iq \cdot x} \end{aligned} \tag{391}
$$

Hence (389) gives the final result, with $\alpha =$ 1371 $= \frac{e^{2}}{4 \pi \hbar c}$, (Heaviside units)

<a id="eq-392"></a>

$$
\langle j_{\nu H} ( x ) \rangle_{o} = - \alpha j_{\nu E} ( x ) \left\{ \frac{2}{3 \pi} R - \frac{1}{4 \pi} \int_{0}^{1} \frac{zdz}{\sqrt{1 - z}} \log \left| 1 + \frac{zq^{2}}{4 \mu^{2}} \right| + \frac{i}{4} \epsilon ( q_{0} ) \int_{0}^{- 4 \mu^{2} /q^{2}} \frac{zdz}{\sqrt{1 - z}} \right\} \tag{392}
$$

## <a id="s-appendix-3"></a>Appendix

The four dimensional volume element is (see (385)) $d\xi_{1} d\xi_{2} d\xi_{3} d\xi_{4}$ = 2$\pi^{2} r^{3} dr$ in four dimensional polar coordinates. To show this, we denote the surface of a $p$ dimensional unit sphere by $\omega$. Then the surface of a $p$ dimensional sphere of radius $R$ is $R^{p - 1} \omega$, and hence the volume element in polar coordinates is $\omega R^{p - 1} dR$.

To calculate the value of $\omega$, we compute the $p$ dimensional Laplace <a id="ix-126">integral</a> in Cartesian and polar coordinates. We have

$$
J = \int \int \dots \int \exp \left\{ - \sum_{i = 1}^{p} \xi_{i}^{2} \right\} d\xi_{1} d\xi_{2} \dots d\xi_{p} = \left( \sqrt{\pi} \right)^{p}
$$

On the other hand

$$
J = \omega \int_{0}^{\infty} e^{- \rho^{2}} \rho^{p - 1} d\rho = \omega \frac{\Gamma ( p/ 2 )}{2}
$$

and thus by comparison

$$
\omega = \frac{2 \pi^{p/ 2}}{\Gamma ( p/ 2 )}
$$

For $p$ = 4, we have Γ(2) = 1, $\omega$ = 2$\pi^{2}$. For $p$ = 3, we have Γ$( \tfrac{3}{2} ) = \frac{\pi}{2} \sqrt{, \omega} = \frac{2 \pi^{3 / 2}}{\sqrt{\pi /} 2}$ = 4$\pi$, etc.

### <a id="s-physical-meaning-of-the-vacuum-polarization"></a>Physical Meaning of the Vacuum Polarization

We now discuss the various physical effects arising from the calculation of

$$
\langle j_{\nu H} ( x ) \rangle_{o}
$$

1) The result is completely gauge-invariant. This may be seen at once from (391): for if a gradient $\partial$Λ$/\partial x_{\nu}$ is added to $A_{\nu}^{e}$, there is no change in $j_{\nu E}$.

<a id="p76"><sup>[p. 76]</sup></a>

2) If the relation (382) had not been used in order to simplify (379), we should have had an addition to $\langle j_{\nu H} ( x ) \rangle_{o}$ of the form $K^{\prime} e_{\nu} = KA_{\nu}^{e} ( x )$ (see (360)) where $K$ is an indeterminate numerical factor containing the divergent integral standing on the left of (382). That is to say, an induced current proportional to the inducing potential. This would have destroyed the gauge-invariance of the result if $K \neq$ 0. Therefore we can also use the physical requirement that the results must be gauge-invariant in order to give the value zero to the indeterminate $K$.

3) The energy density of the vacuum arising from the polarization by the potentials $A_{\nu}^{e} ( x )$ is

<a id="eq-393"></a>

$$
d ( x ) = - \frac{1}{2 c} \sum_{\nu} A_{\nu}^{e} ( x ) \langle j_{\nu E} ( x ) \rangle_{o} \tag{393}
$$

Thus the term $KA_{\nu}^{e} ( x )$ would give an energy density

<a id="eq-394"></a>

$$
- \frac{K}{2 c} \sum_{\nu} A_{\nu}^{e} ( x ) A_{\nu}^{e} ( x ) \tag{394}
$$

associated with the electromagnetic potentials. This would give to the photon a finite rest-mass, and for this reason $K$ is often referred to as the “self-energy of the photon”. There is a lot of discussion in the literature of this photon self-energy. But since physical arguments lead us definitely to give $K$ the value zero, we have no choice but to say that the photon self-energy also is zero. This is of course the result that any consistent theory of electrodynamics must give.

4) The logarithmic divergence $R$ is a real divergence and cannot be given the value 0 from physical arguments. However, it gives only an induced charge exactly proportional to the inducing external charge. It is never possible experimentally to separate the external charge from the proportional induced charge. Therefore in all measurements of the external charge, the measured charge will be not $j_{\nu E} ( x )$ but

<a id="eq-395"></a>

$$
j_{\nu R} ( x ) = \left( 1 - \frac{2 \alpha}{3 \pi} R \right) j_{\nu E} ( x ) \tag{395}
$$

$j_{\nu R}$ here meaning “renormalized charge”. Thus the effect of the term $R$ in (392) is only change the unit in which the external charge is measured. We write the results in terms of the observed external charge $j_{\nu R}$ instead of the unobservable $j_{\nu E}$, this change of units being called “charge renormalization”. Note the similarity between charge and mass renormalization. In both cases a divergent effect is recognized as producing no observable phenomenon, because it only changes the value of one of the fundamental constants, in the one case the electron mass $m$ and in the other the unit of charge $e$. Since $m$ and $e$ are things which are observed directly, the divergent effects disappear completely when the results are written in terms of the observed $m$ and $e$. Thus (392) becomes

<a id="eq-396"></a>

$$
\langle j_{\nu H} ( x ) \rangle_{o} = \alpha j_{\nu R} ( x ) \left\{ \frac{1}{4 \pi} \int_{0}^{1} \frac{zdz}{\sqrt{1 - z}} \log \left| 1 + \frac{zq^{2}}{4 \mu^{2}} \right| - \frac{i}{4} \epsilon ( q_{0} ) \int_{0}^{- 4 \mu^{2} /q^{2}} \frac{zdz}{\sqrt{1 - z}} \right\} \tag{396}
$$

Everything here is now finite and observable.

5) When $A_{\nu}^{e} ( x )$ is the potential of a pure radiation field without sources, $j_{\nu R}$ = 0 and so there is no polarization. Thus for every photon or freely travelling wave, the vacuum behaves like a true vacuum; there are no dielectric effects of any kind. This agrees with the common-sense idea of the vacuum. The result is however only true so long as the polarization can be treated as linear; if non-linear effects were included then two beams of light crossing the same region would produce a polarization current, giving rise to a “scattering of light by light”. The light-by-light scattering has been calculated, it is not zero but is much too small to be observable.

6) The induced current (396) consists of two components, the first in phase with the potential $A_{\nu}^{e} ( x )$ and the second out of phase by $\pi /$2. Speaking of the vacuum as a circuit being driven by the potential $A_{\nu}^{e} ( x )$, the first is an inductive effect, the second a resistive one. So only the second term gives an absorption of energy by the vacuum from the driving potentials.

<a id="p77"><sup>[p. 77]</sup></a>

We study the energy balance, remembering that the classical potentials $A_{\nu}^{e} ( x )$ must always be real quantities like

<a id="eq-397"></a>

$$
A_{\nu}^{e} ( x ) = e_{\nu} \cos ( q \cdot x ) \tag{397}
$$

and we take without loss of generality $q_{0}$ positive. Then (396) gives, taking $e \cdot q$ = 0,

<a id="eq-398"></a>

$$
\langle j_{\nu H} ( x ) \rangle_{o} = e_{\nu} \{ A \cos ( q \cdot x ) + B \sin ( q \cdot x ) \} \tag{398}
$$

where $A$ and $B$ are real and $2 \quad 2$

<a id="eq-399"></a>

$$
B = \tfrac{1}{4} \alpha cq^{2} \int_{0}^{- 4 \mu /q} \frac{zdz}{\sqrt{1 - z}} \tag{399}
$$

The energy supplied by the potentials to the vacuum per unit volume and per unit time is

<a id="eq-400"></a>

$$
\begin{aligned} E &= - \frac{1}{c} \sum_{\nu} \langle j_{\nu H} ( x ) \rangle_{o} \frac{\partial A_{\nu}^{e} ( x )}{\partial t} \\ &= - q_{0} \sum_{\nu} e_{\nu}^{2} \left[ A \sin ( q \cdot x ) \cos ( q \cdot x ) + B \sin^{2} ( q \cdot x ) \right] \end{aligned} \tag{400}
$$

From (400) one sees that the in-phase current gives no net absorption of energy, while the out-of-phase current gives a mean energy supply per unit time

<a id="eq-401"></a>

$$
\overline{E} = - \tfrac{1}{2} q_{0} e^{2} B = - \frac{\alpha ce^{2} q^{2} q_{0}}{8} \int_{0}^{- 4 \mu^{2} /q^{2}} \frac{zdz}{\sqrt{1 - z}} \tag{401}
$$

If $q$ does not satisfy (390), i.e.if

<a id="eq-402"></a>

$$
q_{0} < \sqrt{4 \mu^{2} + | \boldsymbol{q} |^{2}} \tag{402}
$$

then $B$ = 0 and $E$ = 0, there is not enough energy in the vibrations of the field to create a real positron-electron pair whose rest-mass alone requires 2$mc^{2}$, given that the field supplies in the interaction a momentum $\hbar k$ together with the energy $\hbar cq_{0}$.

However if (390) is satisfied, then there is enough energy for creation of real pairs, each real pair carrying energy $\hbar cq_{0}$. Since $q$ is time-like and $e \cdot q$ = 0, $e$ is space-like and $( e^{2} ) >$ 0. This can be seen as follows:

$$
e \cdot q = 0 = \boldsymbol{e} \cdot \boldsymbol{q} - e_{0} q_{0}
$$

Since $q$ is timelike, we can use a Lorentz transformation which makes $\boldsymbol{q}$ = 0; then, of course, $q_{0} \neq$ 0. But then we must have $e_{0}$ = 0, which means that $e$ is spacelike. Therefore $E>$ 0, verifying that the potentials can never extract energy from the vacuum. And we have the probability per unit volume and per unit time that the potential (397) will create a real pair

<a id="eq-403"></a>

$$
w = \frac{\overline{E}}{\hbar cq_{0}} = - \frac{\alpha ( e^{2} ) ( q^{2} )}{8 \hbar} \int_{0}^{- 4 \mu^{2} /q^{2}} \frac{zdz}{\sqrt{1 - z}} \tag{403}
$$

The result could of course have been found more easily by elementary methods. The thing I want to stress here is that the elementary real pair-creation processes are necessarily tied up with the less elementary <a id="ix-218">vacuum-polarization</a> effect given by the $A$ term in (398), and the $A$ term will exist whether or not real pair production is possible. The situation is quite the same as the tying up of the elementary line-width effect in atomic spectra with the less elementary line-shifts; we discussed these effects in detail earlier. Therefore we have quite as much reason to take seriously the vacuum-polarization effect given by the in-phase current in (396), as we have to take seriously the Lamb shift itself. Because physicists were unwilling to take these two effects seriously, physics was held up for quite a number of years.

<a id="p78"><sup>[p. 78]</sup></a>

### <a id="s-slowly-varying-weak-fields-the-uehling-effect"></a>Vacuum Polarization For Slowly Varying Weak Fields. The Uehling Effect.

Let now the external potential $A_{\nu}^{e} ( x )$ be not only weak but also slowly varying in both space and time, i.e. let it be a superposition of Fourier components (360) with

<a id="eq-404"></a>

$$
| q^{2} |\ll \mu^{2} \tag{404}
$$

Then by (390) the second term in (396) is zero<sup><a id="fnref-53" href="#fn-53">53</a></sup> and the logarithm may be expanded in terms of $( q^{2} /\mu^{2} )$. Keeping only the term of order $q^{2}$

$$
\langle j_{\nu H} ( x ) \rangle_{o} = \alpha \frac{q^{2}}{1 6 \pi \mu^{2}} j_{\nu R} ( x ) \int_{0}^{1} \frac{z^{2} dz}{\sqrt{1 - z}} = \frac{\alpha q^{2}}{1 5 \pi \mu^{2}} j_{\nu R} ( x )
$$

But in each Fourier component of $j_{\nu R} ( x )$, operating with the D’Alembertian $\Box^{2}$ gives a factor $( - q^{2} )$. Hence the result, independent of the Fourier decomposition, valid for slowly varying fields

<a id="eq-405"></a>

$$
\langle j_{\nu H} ( x ) \rangle_{o} = - \frac{\alpha}{1 5 \pi \mu^{2}} \left\{ \Box^{2} j_{\nu R} ( x ) \right\} \tag{405}
$$

Consider the effect of (405) in the case of the hydrogen atom. The proton<sup><a id="fnref-54" href="#fn-54">54</a></sup> is represented by the static <a id="ix-220">charge</a>-density $\rho_{P} ( r )$, and this induces a charge in the vacuum whose density is

<a id="eq-406"></a>

$$
\rho_{\text{ IN }} ( r ) = - \frac{\alpha}{1 5 \pi \mu^{2}} \nabla^{2} \rho_{P} ( r ) \tag{406}
$$

The electrostatic potential of the proton is thus $V ( r ) + V_{\text{ IN }} ( r )$, where

$$
\nabla^{2} V ( r ) = - \rho_{P} ( r )
$$

$$
\nabla^{2} V_{\text{ IN }} ( r ) = - \rho_{\text{ IN }} ( r ) = \frac{\alpha}{1 5 \pi \mu^{2}} \nabla^{2} \rho_{P} ( r )
$$

and hence

<a id="eq-407"></a>

$$
V_{\text{ IN }} ( r ) = + \frac{\alpha}{1 5 \pi \mu^{2}} \rho_{P} ( r ) \tag{407}
$$

Thus for a point proton, the potential added to the Coulomb potential by vacuum-polarization is

<a id="eq-408"></a>

$$
V_{\text{ IN }} ( r ) = + \frac{\alpha e}{1 5 \pi \mu^{2}} \delta^{3} ( \boldsymbol{r} ) \tag{408}
$$

And the change in the energy of a state of the hydrogen atom with wave-function $\psi ( r )$ is

<a id="eq-409"></a>

$$
\Delta E_{P} = - \frac{\alpha e^{2}}{1 5 \pi \mu^{2}} | \psi ( 0 ) |^{2} = - \tfrac{1}{5} \left\{ \frac{e^{4} \hbar}{1 2 \pi^{2} m^{2} c^{3}} | \psi ( 0 ) |^{2} \right\} \tag{409}
$$

This is just the same as the formula for the Lamb shift, (284), with $( - 1 /$5) instead of the logarithm. Thus it is a factor 40 smaller than the Lamb shift, and in the opposite direction, $-$27 megacycles in the total of 1062. Still the experiments are good enough to show that the effect is there all right.

The result (409) was calculated many years ago by Uehling <a id="refback-17-1"></a>[[17]](#ref-17) using old-fashioned methods.

## <a id="s-iv-field-theory-of-dirac-and-maxwell-fields-in-interaction"></a>IV. Field Theory Of Dirac And Maxwell Fields In Interaction

### <a id="s-the-complete-relativistic-quantum-electrodynamics"></a>The Complete Relativistic Quantum Electrodynamics

We now take the combined system of Dirac and Maxwell fields in interaction, and make out of it a relativistic quantum theory using the methods we have already developed. This will then be the complete theory of quantum electrodynamics, applicable to all problems in which electrons, positrons and photons are <a id="p79"><sup>[p. 79]</sup></a> concerned. We also include in the theory a classical <a id="ix-191">Maxwell</a> field, which acts on the electrons and positrons and represents the effects of external charges such as protons which may happen to be present.

Lagrangian

<a id="eq-410"></a>

$$
L = L_{D} + L_{M} - ie \overline{\psi} / A\psi - ie \overline{\psi} / A^{e} \psi \tag{410}
$$

Here we use $A_{\nu} ( x )$ for the Maxwell potential operators and $A_{\nu}^{e} ( x )$ for the potentials of the classical external field.

Field equations

<a id="eq-411"></a>

$$
\left\{ \sum_{\lambda} \gamma_{\lambda} \left\{ \frac{\partial}{\partial x_{\lambda}} + \frac{ie}{\hbar c} ( A_{\lambda} + A_{\lambda}^{e} ) \right\} + \mu \right\} \psi = 0 \tag{411}
$$

<a id="eq-412"></a>

$$
\sum_{\lambda} \left\{ \frac{\partial}{\partial x_{\lambda}} - \frac{ie}{\hbar c} ( A_{\lambda} + A_{\lambda}^{e} ) \right\} \overline{\psi} \gamma_{\lambda} - \mu \overline{\psi} = 0 \tag{412}
$$

<a id="eq-413"></a>

$$
\Box^{2} A_{\nu} = ie \overline{\psi} \gamma_{\nu} \psi \tag{413}
$$

(See (384).)<sup><a id="fnref-55" href="#fn-55">55</a></sup>

These equations are _non-linear_. And so there is no possibility of finding the general commutation rules of the field operators in closed form. We cannot find any solutions of the field equations, except for the solutions which are obtained as formal power series expansions in the coefficient $e$ which multiplies the non-linear interaction terms. It is thus a basic limitation of the theory, that it is in its nature a perturbation theory stating from the non-interacting fields as an unperturbed system. Even to write down the general commutation laws of the fields, it is necessary to use a perturbation theory of this kind.

Since the perturbation theory treatment is forced on us from the beginning, it is convenient not to set up the theory in the Heisenberg representation but to use the interaction representation. The IR is just designed for a perturbation theory in which the radiation interaction is treated as small. In the IR the commutation rules can be obtained simply in closed form, and so the theory can be set up with a minimum of trouble.

There are two different interaction representations which we can use. The first may be called the _Bound_ _Interaction Representation_. It is exactly the representation we used in discussing the radiation from an atom in the non-relativistic theory. We take all field operators to have the time-dependence of the Heisenberg operators in the theory of the free Maxwell field and the electron field interacting with external potential, only the interaction between the two fields being omitted. Thus the field equations in the BIR are (332), (333), and

<a id="eq-414"></a>

$$
\Box^{2} A_{\nu} = 0 \tag{414}
$$

The wave-function Φ$( t )$ in the BIR satisfies the Schrödinger equation

<a id="eq-415"></a>

$$
i \hbar \frac{\partial \Phi}{\partial t} = H_{R} ( t ) \Phi \tag{415}
$$

<a id="eq-416"></a>

$$
H_{R} ( t ) = ie \int \overline{\psi} ( r,t ) A / ( r,t ) \psi ( r,t ) d^{3} \boldsymbol{r} \tag{416}
$$

This $H_{R} ( t )$ is just the difference between the Hamiltonians of the theories with and without the radiation interaction. Because no derivatives of field operators occur in $H_{R}$, the difference is just minus the difference between the corresponding Lagrangians, and so has the simple form given by (416). (Compare with (243).)

Using the BIR we can discuss the radiation of light by an atom, as it was done before, but now treating the atom relativistically. In fact this representation must be used, as soon as we wish to calculate effects accurately enough to require exact Dirac wave-functions for the unperturbed atomic states. However, the BIR is not convenient to use, because the commutation rules for the electron field are given by (346) and are still too complicated for all but the simplest problems. Therefore we shall use the BIR only when we are compelled to, and then usually only in the final stages of a problem. Generally we can do the main part of the work, with the major calculations, in the second type of interaction representation.

<a id="p80"><sup>[p. 80]</sup></a>

### <a id="s-free-interaction-representation"></a>Free Interaction Representation

Here we take all field operators $\overline{\psi} , \psi$ and $A_{\mu}$ to satisfy the free field equations (286) and (414). The commutation rules are then also given by the free-field formulae (203) and (298), (299). The wave-function satisfies the Schrödinger equation

<a id="eq-417"></a>

$$
i \hbar \frac{\partial \Phi}{\partial t} = \{ H^{e} ( t ) + H_{R} ( t ) \} \Phi \tag{417}
$$

<a id="eq-418"></a>

$$
H^{e} ( t ) = ie \int \overline{\psi} ( \boldsymbol{r} ,t ) A /^{e} ( \boldsymbol{r} ,t ) \psi ( r,t ) d^{3} \boldsymbol{r} \tag{418}
$$

with $H_{R}$ formally again given by (416). But here $H_{R}$ is not the same operator as in (415) because of the time-dependence of the $\overline{\psi}$ and $\psi$ in the two cases.

This FIR is the interaction representation normally used in <a id="ix-144">quantum electrodynamics</a>, and will be called henceforth the Interaction Representation or IR simply. It is very well suited to relativistic calculations because it makes the field-commutators and expectation values invariant functions. Thus the calculations can be explicitly and formally invariant, even when the <a id="ix-193">potentials</a> $A_{\nu}^{e}$ are given in a special Lorentz frame as in the hydrogen atom.

It was Schwinger and Feynman who first discovered the importance of making calculations formally invariant when using a relativistic theory. They made this discovery in completely different ways, characteristically. Feynman simply found that the calculations become much easier and simpler when done in a way which did not conceal the invariance of the theory. This is still true, in fact the main reason why we can tackle now more difficult problems than we could 10 years ago is just that the calculations with the new methods are so much shorter. But the greater and essential advantage of the covariant calculations, pointed out by Schwinger, is that they enable the separation of finite observable effects from infinite renormalization terms to be made in a clear and unambiguous way. We had an example of this in the treatment we gave of vacuum polarization, where the covariant type of calculation was used. The divergent term (382) could be separated clearly from (379), because of the way in which (379) depends formally on the vectors $e_{\nu}$ and $q_{\nu}$. If the calculation had been done in a non-covariant way we could not have used (381) as we did.

So now we shall apply the covariant methods, working in the IR, to solve a number of the standard problems of electrodynamics, in increasing order of difficulty.

---

## Notes

<a id="fn-22"></a>**22.** In the last commutator, the operator $\tilde{a}_{k^{\prime} \lambda}$ lacked the tilde. [↩](#fnref-22)

<a id="fn-23"></a>**23.** “Bleuler” was written “Bleuber”. [↩](#fnref-23)

<a id="fn-24"></a>**24.** Originally, the phrase read $A_{\mu} = \partial$Λ$/\partial x_{\mu}$; this seemed confusing as the original potential is itself $A_{\mu}$. [↩](#fnref-24)

<a id="fn-25"></a>**25.** Eq. (222) lacked the lower limit $k_{0} >$ 0 on the integral. [↩](#fnref-25)

<a id="fn-26"></a>**26.** $D^{( 1 )} ( x )$ was added on the left-hand side for clarity [↩](#fnref-26)

<a id="fn-27"></a>**27.** Eq. (230) lacked the sum over $\mu$; _cf._ Eq. (170). [↩](#fnref-27)

<a id="fn-28"></a>**28.** “obtained” was inserted; statement read “per unit time is using . . . ” [↩](#fnref-28)

<a id="fn-29"></a>**29.** The subscript $A$ was missing in the term $j_{\lambda A} ( x^{\prime} )$. [↩](#fnref-29)

<a id="fn-30"></a>**30.** The exponential in the first integral lacked $i$; it had the argument $- \boldsymbol{k} \cdot \boldsymbol{r}$. [↩](#fnref-30)

<a id="fn-31"></a>**31.** Bethe & Salpeter, ref. <a id="05-examples-of-quantized-field-theories--refback-20-1"></a>[[20]](#05-examples-of-quantized-field-theories--ref-20), p. 249, Eq. 59.7 [↩](#fnref-31)

<a id="fn-32"></a>**32.** There was no subscript “o” on the vacuum expectation value. [↩](#fnref-32)

<a id="fn-33"></a>**33.** “if” substuted for “it”; and “gives” added to the previous sentence. [↩](#fnref-33)

<a id="fn-34"></a>**34.** The limits on the integral were $-$1 and +1, and the value of the integral was given as $\frac{2}{3}$. [↩](#fnref-34)

<a id="fn-35"></a>**35.** The expression for $a_{o}$ is not here in the original, but it appears before Eq. (240). [↩](#fnref-35)

<a id="fn-36"></a>**36.** The expression for $\rho$ is not in the original. [↩](#fnref-36)

<a id="fn-37"></a>**37.** $a_{o}$ = 0$. 5 2 9 1 7 7 \times 1 0^{- 8}$ cm; Ry = 13.6056 eV. [↩](#fnref-37)

<a id="fn-38"></a>**38.** “time-independent” substituted for “time-dependent”, which describes the Heisenberg representation. [↩](#fnref-38)

<a id="fn-39"></a>**39.** Sum over $\mu$ inserted. [↩](#fnref-39)

<a id="fn-40"></a>**40.** Following “given by (263)”, the second edition has the phrase “with $o$ suffices”. The first edition lacks this phrase. As the sentence makes more sense without it, it has been deleted. [↩](#fnref-40)

<a id="fn-41"></a>**41.** “$s$ states” replaces “$x$ states”. [↩](#fnref-41)

<a id="fn-42"></a>**42.** The coefficient of the first integral in Eq. (283) was $\hbar$; it has been replaced by $\hbar^{2}$. Also, Eq. (283) lacked a label in the second edition. [↩](#fnref-42)

<a id="fn-43"></a>**43.** Here, the Bohr radius was denoted $a$; it seemed reasonable to use $a_{o}$ instead. [↩](#fnref-43)

<a id="fn-44"></a>**44.** “is” replaces “being” [↩](#fnref-44)

<a id="fn-45"></a>**45.** “by” replaces “be” [↩](#fnref-45)

<a id="fn-46"></a>**46.** In v.2, the Hermitian conjugate $b_{m}^{*}$ lacked the asterisk. [↩](#fnref-46)

<a id="fn-47"></a>**47.** The argument of the exp function originally had the factor $( t - t_{o} )$; this has been replaced by the factor [↩](#fnref-47)

$$
( t - t^{\prime} ) .
$$

<a id="fn-48"></a>**48.** The function $F_{\nu} ( k )$ was written as a scalar, $F ( k )$. This is misleading; the right-hand side is a vector function, because it is linear in $\gamma_{\nu}$. So $F ( k )$ was promoted to $F_{\nu} ( k )$. [↩](#fnref-48)

<a id="fn-49"></a>**49.** The phrase “integral in the” was inserted. [↩](#fnref-49)

<a id="fn-50"></a>**50.** “definite” replaces “indefinite” [↩](#fnref-50)

<a id="fn-51"></a>**51.** In v.2, this reads “Spur $\sum_{\mu} \gamma_{\mu} \gamma_{\nu}$ = 4” [↩](#fnref-51)

<a id="fn-52"></a>**52.** The bottom limit of 0 was added to the last integral sign. Note that the change of variable is easier to follow by first observing [↩](#fnref-52)

$$
\int_{0}^{1} dz ( z - z^{2} ) f ( z - z^{2} ) = 2 \int_{0}^{1 / 2} dz ( z - z^{2} ) f ( z - z^{2} )
$$

because the expression $( z - z^{2} )$ is symmetric about $z = \tfrac{1}{2}$.

<a id="fn-53"></a>**53.** The original read “Then by (390) in (396) the second term is zero”. [↩](#fnref-53)

<a id="fn-54"></a>**54.** “proton” replaces “photon” [↩](#fnref-54)

<a id="fn-55"></a>**55.** The potentials had a subscript $\nu$ and the gamma matrices a subscript $\lambda$. [↩](#fnref-55)

---

## References

<a id="ref-14"></a>**[14]** Theodore A. Welton, “Some Observable Effects of the Quantum-Mechanical Fluctuations of the Electromagnetic Field”, _Phys. Rev._ **74** (1948) 1157. Ted Welton was a friend and undergraduate classmate of Feynman’s at MIT. See Schweber, _QED and the Men Who Made It_, pp 375–378. [↩](#refback-14-1)

<a id="ref-16"></a>**[16]** H. A. Kramers, “The interaction between charged particles and the radiation field”, _Nuovo Cim._ NS **15** (1938) 108. English translation in Miller, _Early Quantum Electrodynamics_, pp 254–258. [↩](#refback-16-1)

<a id="ref-15"></a>**[15]** R. R. Wilson, “Scattering of 1$. 3$<sup>3</sup> MeV Gamma Rays by an Electric Field”, _Phys. Rev._ **90** (1953) 720. Wilson was at Cornell at the time; also he notes “The measurements here reported were all made in 1951. Publication has been held up until now in the hope that the Rayleigh scattering could be calculated more accurately.” [↩](#refback-15-1)

<a id="ref-17"></a>**[17]** E. A. Uehling, “Polarization Effects in the Positron Theory”, _Phys. Rev._ **48** (1935) 55. [↩](#refback-17-1)

<a id="05-examples-of-quantized-field-theories--ref-20"></a>**[20]** Hans A. Bethe & Edwin E. Salpeter, _Quantum Mechanics of One- and Two-Electron Atoms_, Springer- Verlag, Berlin, 1957. Reissued by Plenum Publishing Co., New York, 1977, (paperback edition). This is a revised and updated version of the article Dyson cites, H. A. Bethe, “Quantenmechanik der Einund Zwei-Elektronenprobleme”, _Handbuch der Physik_, Bd. 24/1, Springer, Berlin, 1933. The relevant formula for Eq. (710) is to be found in the new work with exactly the same label, (3.26), on p. 17. Note that Bethe & Salpeter use Hartree’s “atomic units”, so that distances are measured in terms of $a_{o}$. [↩](#05-examples-of-quantized-field-theories--refback-20-1)

---

<a id="06-free-particle-scattering-problems"></a>

# Free Particle Scattering Problems

<a id="p81"><sup>[p. 81]</sup></a>

In this extensive class of problems we are interested in calculating the over-all transition matrix element $M$ between an initial state $A$ and a final state $B , A$ and $B$ being specified by assigning the spins and momenta of the free particles which are present in these states. The scattering process is supposed to occur as follows: The free particles which are specified by state $A$ in the remote past, converge and interact, and other free particles emerge or are created in the interaction and finally constitute the state $B$ in the remote future. We wish to calculate the matrix element $M$ for this process, without studying the equations of motion or investigating the behavior of the system at intermediate times while the interaction is in progress.

The unperturbed states $A$ and $B$ are supposed to be states of free particles without interaction and are therefore represented by constant state-vectors Φ$A$ and Φ$B$ in the <a id="ix-125">interaction representation</a>. The actual initial and final states in a scattering problem will consist of particles each having a self-<a id="ix-183">field</a> with which it continues to interact even in the remote future and past, hence Φ$A$ and Φ$B$ do not accurately represent the initial and final states. However, so long as we are using perturbation theory and not including the higher order effects arising form the self-fields of the particles, it is consistent to use constant Φ$A$ and Φ$B$ representing bare particles <a id="ix-111">without radiation</a> interaction. Even when self-field effects are considered, it turns out that the bare-particle Φ$A$ and Φ$B$ can still be used, although in this case some careful justification for it is needed.

The matrix element $M$ is

<a id="eq-419"></a>

$$
M = ( \Phi_{B}^{*} U \Phi_{A} ) \tag{419}
$$

Here $U$Φ$A$ is the state which is obtained at $t$ = +$\infty$ by solving the <a id="ix-233">equation</a> of motion (417) with the initial condition Φ = Φ$A$ at $t = -\infty . U$ can easily be written down as a perturbation expansion in the operators $H^{e}$ and $H_{R}$,

<a id="eq-420"></a>

$$
\begin{aligned} U &= 1 + \left( - \frac{i}{\hbar} \right) \int_{-\infty}^{\infty} dt_{1} \{ H^{e} ( t_{1} ) + H_{R} ( t_{1} ) \} \\ & \quad + \left( - \frac{i}{\hbar} \right)^{2} \int_{-\infty}^{\infty} dt_{1} \int_{-\infty}^{\infty} dt_{2} \{ H^{e} ( t_{1} ) + H_{R} ( t_{1} ) \}\{ H^{e} ( t_{2} ) + H_{R} ( t_{2} ) \} + \dots \end{aligned} \tag{420}
$$

<a id="eq-421"></a>

$$
= \sum_{n = 0}^{\infty} \left( - \frac{i}{\hbar} \right)^{n} \frac{1}{n !} \int_{-\infty}^{\infty} dt_{1} \dots \int_{-\infty}^{\infty} dt_{n} P \{\{ H^{e} ( t_{1} ) + H_{R} ( t_{1} ) \} \dots \{ H^{e} ( t_{n} ) + H_{R} ( t_{n} ) \}\} \tag{421}
$$

Here the $P$ signifies a chronological product, the factors in it being multiplied not in the order in which they are written but in the order of the times $t_{1} ,t_{2} ,\dots ,t_{n}$, the factors with later times standing to the left of those with earlier times. This accounts for the factor 1$/n$! after we change all the limits to cover the whole range from $-\infty$ to +$\infty$. The operator $U$ is generally referred to as “the S-matrix”.

Before discussing the general analysis of the series expansion (421), we shall use it to solve some standard problems.

## <a id="s-a-m-ller-scattering-of-two-electrons"></a>A. Møller Scattering of Two Electrons

We have in the initial state $A$ two electrons in states $( p_{1} u_{1} ) ( p_{2} u_{2} )$ and in the final state $B$ two electrons $( p_{1}^{\prime} u_{1}^{\prime} ) ( p_{2}^{\prime} u_{2}^{\prime} )$. The electron ($p_{1} u_{1} )$ is given by the one-particle <a id="ix-82">wave-function</a>

<a id="eq-422"></a>

$$
u_{1} e^{ip_{1} \cdot x} \tag{422}
$$

<a id="p82"><sup>[p. 82]</sup></a>

normalized by $( \overline{u}_{1} u_{1} )$ = 1. With the wave function normalized in this way, (422) is just the matrix element of the operator $\psi ( x )$ between the vacuum state and the state containing electron 1. We can see this from

$$
\psi ( x ) = \sum_{p,u} b_{pu} ue^{ip \cdot x}
$$

$$
\text{ where } \{ b_{pu} ,b_{p^{\prime} u^{\prime}}^{*} \} = \delta_{pp^{\prime}} \delta_{uu^{\prime}} . \text{ Then } ( \Phi_{o}^{*} ,\psi ( x ) \Phi_{pu} ) = ( \Phi_{o}^{*} ,b_{pu} \Phi_{pu} ) ue^{ip \cdot x} = ( \Phi_{o}^{*} , \Phi_{o} ) ue^{ip \cdot x} = ue^{ip \cdot x} .
$$

Thus we consider the states 1, 2 and 1${}^{\prime} , 2^{\prime}$ as if they were discrete states, the $\psi$ operator being given by the expansion (337). It would also be possible to use the continuous-state expansion (306) for $\psi$, but then the normalization of the initial and final states would have to be considered afresh. Since we fixed the normalization (472) when we previously derived the Møller formula (144), we shall now stick to it.

We shall calculate in the Born approximation as before, thus keeping only the term $n$ = 2 in (421) which gives the matrix element $M$ proportional to $e^{2}$. In this problem the external potential $A^{e}$ is zero. The term $n$ = 2 in (421) is

<a id="eq-423"></a>

$$
U_{2} = \frac{+ e^{2}}{2 \hbar^{2} c^{2}} \int \int dx_{1} dx_{2} P \left\{ \psi ( x_{1} ) A / ( x_{1} ) \psi ( x_{1} ) , \overline{\psi} ( x_{2} ) A / ( x_{2} ) \psi ( x_{2} ) \right\} \tag{423}
$$

the integration going over all space-time. To obtain the matrix element $M$ = (Φ$\frac{*}{B} U_{2}$Φ${}_{A} )$ we only have to replace according to (377)

<a id="eq-424"></a>

$$
\psi ( x_{i} ) = u_{1} e^{ip_{1} \cdot x_{i}} b_{1} + u_{2} e^{ip_{2} \cdot x_{i}} b_{2} + u^{\prime} e^{ip_{1}^{\prime} \cdot x_{i}} b_{1}^{\prime} + u_{2}^{\prime} e^{ip_{2}^{\prime} \cdot x_{i}} b_{2}^{\prime} \quad \text{ by } \quad u_{1} e^{ip_{1} \cdot x_{i}} b_{1} + u_{2} e^{ip_{2} \cdot x_{i}} b_{2} \tag{424}
$$

and

$$
\overline{\psi} ( x_{i} ) = \overline{u}_{1} e^{- ip_{1} \cdot x_{i}} b_{1}^{*} + \overline{u}_{2} e^{- ip_{2} \cdot x_{i}} b_{2}^{*} + \overline{u}_{1}^{\prime} e^{- ip_{1}^{\prime} \cdot x_{i}} b_{1}^{*\prime} + \overline{u}_{2}^{\prime} e^{- ip_{2}^{\prime} \cdot x_{i}} b_{2}^{*\prime} \quad \text{ by } \quad \overline{u}_{1}^{\prime} e^{- ip_{1}^{\prime} \cdot x_{i}} b_{1}^{*\prime} + \overline{u}_{2}^{\prime} e^{- ip_{2}^{\prime} \cdot x_{i}} b_{2}^{*\prime}
$$

because we only absorb 1 and 2 and only create 1$\prime$ and 2$\prime$. Then we want to pick out the coefficient of

<a id="eq-425"></a>

$$
( b_{1}^{*\prime} b_{1} ) ( b_{2}^{*\prime} b_{2} ) \tag{425}
$$

in the resulting expansion. There are no photons in the initial and final states and so the vacuum expectation value is taken for the Maxwell potential operators. We thus find, taking account of the fact that the $b$ and $\overline{b}$ anticommute with each other as in the derivation of (358), the result

<a id="eq-426"></a>

$$
\begin{aligned} M &= \sum_{\mu ,\lambda} \frac{e^{2}}{\hbar^{2} c^{2}} \int \int dx_{1} dx_{2} \{ \exp [ i ( p_{1} - p_{1}^{\prime} ) \cdot x_{1} + i ( p_{2} - p_{2}^{\prime} ) \cdot x_{2} ] ( \overline{u}_{1}^{\prime} \gamma_{\lambda} u_{1} ) ( \overline{u}_{2}^{\prime} \gamma_{\mu} u_{2} ) \\ & \quad - \exp [ i ( p_{1} - p_{2}^{\prime} \cdot x_{1} + i ( p_{2} - p_{1}^{\prime} ) \cdot x_{2} ] ( \overline{u}_{2}^{\prime} \gamma_{\lambda} u_{1} ) ( \overline{u}_{1}^{\prime} \gamma_{\mu} u_{2} ) \} \langle P \{ A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) \} \rangle_{o} \end{aligned} \tag{426}
$$

The expectation-value of the chronological product brings in an important new function,

<a id="eq-427"></a>

$$
\langle P \{ A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) \} \rangle_{o} = \tfrac{1}{2} \hbar c\delta_{\lambda \mu} D_{F} ( x_{1} - x_{2} ) \tag{427}
$$

where $F$ stands for <a id="ix-79">Feynman</a>. Also called by Stueckelberg $D^{c} , c$ for causality. <a id="refback-18-1"></a>[[18]](#ref-18)

### <a id="s-properties-of-the-d-f-function"></a>Properties of the $D_{F}$ Function

Since

<a id="eq-428"></a>

$$
P \{ A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) \} = \tfrac{1}{2} \{ A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) \} + \tfrac{1}{2} \epsilon ( x_{1} - x_{2} ) \boldsymbol{[} A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) \boldsymbol{]} \tag{428}
$$

we have by (203) and (205)

<a id="eq-429"></a>

$$
\begin{aligned} D_{F} ( x ) &= D^{( 1 )} ( x ) + i\epsilon ( x ) D ( x ) = \frac{1}{2 \pi^{2}} \left[ \frac{1}{x^{2}} - i\pi \delta ( x^{2} ) \right] \\ &= D^{( 1 )} ( x ) - i \{ D_{A} ( x ) + D_{R} ( x ) \} \\ &= D^{( 1 )} ( x ) - 2 i \overline{D} ( x ) \end{aligned} \tag{429}
$$

<a id="p83"><sup>[p. 83]</sup></a>

according to (228). Obviously $D_{F}$ is an even function. It has the property that asymptotically as $x_{0} \to \infty$ in the future, $D_{F}$ = 2$iD^{+}$ contains only positive frequencies, while as $x_{0} \to -\infty$ in the past, $D_{F} = - 2 iD^{-}$ contains only negative frequencies. See Fierz, _Helv. Phys. Acta_ **23** (1950) 731 for a full discussion.

Thus $D_{F}$ is the potential arising from a point source disturbance at the origin, when all the potential travelling out in the future represents particles <a id="ix-5">created</a>, and all the potential travelling in from<sup><a id="fnref-56" href="#fn-56">56</a></sup> the past represents particles to be absorbed, all particles having positive energy. It is thus the potential which maintains the correct causal time-sequence of events, and in this way it was discovered by Stueckelberg. But the definition (427) is easier to understand and use.

The momentum representation of $D_{F}$ is

<a id="eq-430"></a>

$$
D_{F} ( x ) = \frac{- 2 i}{( 2 \pi )^{4}} \int_{F} e^{ik \cdot x} \frac{d^{4} k}{k^{2}} \tag{430}
$$

The contour of integration here is along the real axis, below the pole at $k_{0} = -| \boldsymbol{k} |$ and above the pole at $k_{0}$ = +$| \boldsymbol{k} |$, in the $k_{0}$ plane:

![Figure 6 (p. 83)](images/fig-p089-1.png)

_Figure 6 (p. 83)_

We can see this using (429), (207), (208), and comparing (210), (226) and (209) one also knows that

![Figure 7 (p. 83)](images/fig-p089-2.png)

_Figure 7 (p. 83)_

(430) is always known as a “Feynman integral”. We can also write

<a id="eq-431"></a>

$$
D_{F} ( x ) = \frac{- 2 i}{( 2 \pi )^{4}} \int e^{ik \cdot x} \frac{d^{4} k}{k^{2} - i\epsilon} \tag{431}
$$

Here the integration is along the real axis for all four components of $k . \epsilon$ is a small positive real number and the limit $\epsilon \to$ 0 is supposed to be taken in (431) after the integration is done. Before going to the limit, the $\epsilon$ term just shifts the poles off the real axis, the pole $| \boldsymbol{k} |$ going done and the pole $-| \boldsymbol{k} |$ going up, and so the integral is well-defined and non-singular.

### <a id="s-the-m-ller-formula-conclusion"></a>The Møller Formula, conclusion

Using (427) and (431) in (426), the integrations over $x_{1}$ and $x_{2}$ can be done at once, giving a $\delta$-function involving $k$, and the $k$ integration can then also be done immediately. The result is

<a id="eq-432"></a>

$$
M_{2} = \sum_{\lambda} \frac{- ie^{2}}{\hbar c} ( 2 \pi )^{4} \delta^{4} ( p_{1} + p_{2} - p_{1}^{\prime} - p_{2}^{\prime} ) \left\{ \frac{( \overline{u}_{1}^{\prime} \gamma_{\lambda} u_{1} ) ( \overline{u}_{2}^{\prime} \gamma_{\lambda} u_{2} )}{( p_{1} - p_{1}^{\prime} )^{2} - i\epsilon} - \frac{( \overline{u}_{2}^{\prime} \gamma_{\lambda} u_{1} ) ( \overline{u}_{1}^{\prime} \gamma_{\lambda} u_{2} )}{( p_{1} - p_{2}^{\prime} )^{2} - i\epsilon} \right\} \tag{432}
$$

Now $p_{1}$ and $p_{1}^{\prime}$ are both electron momentum 4-vectors and hence $( p_{1} - p_{1}^{\prime} )$ is a space-like vector and its square cannot be zero. Hence we can go directly to the limit by putting $\epsilon$ = 0 in (432). This gives us the Møller formula (144) allowing for a difference in units in $p$ and $e$.

It is clear that the formula comes very directly out of (423), once we know the momentum expansion (431) of the $D_{F}$ function. And we shall find that things are equally simple in other free-particle scattering problems.

<a id="p84"><sup>[p. 84]</sup></a>

### <a id="s-electron-positron-scattering"></a>Electron-Positron <a id="ix-228">Scattering</a>

Exactly the same formula (432) also gives the matrix element of scattering of an electron by a positron. We must assume that the electron is initially in state 1 given by

<a id="eq-433"></a>

$$
u_{1} e^{ip_{1} \cdot x} \tag{433}
$$

and finally in state 1$\prime$ by

<a id="eq-434"></a>

$$
u_{1}^{\prime} e^{ip_{1}^{\prime} \cdot x} \tag{434}
$$

But now the initial state of the positron is given by the wave-function

<a id="eq-435"></a>

$$
\overline{u}_{2}^{\prime} e^{- ip_{2}^{\prime} \cdot x} \tag{435}
$$

and the final state by

<a id="eq-436"></a>

$$
\overline{u}_{2} e^{- ip_{2} \cdot x} \tag{436}
$$

using the negative-energy electron wave-functions and _not_ the charge-conjugate functions to represent the positron. The correctness of (435) and (436) is clear since $b_{2}$ is the _emission_ operator and $b_{2}^{\prime}$ the _absorption_ operator for this positron.

The second term in (432) now represents not an ordinary exchange effect but a special short-range scattering due to a virtual annihilation of the positron and electron. This term has been observed experimentally by measuring the fine-structure constant of positronium. (M. Deutsch and E. Dulit, _Phys. Rev._ **84** (1951) 601, (Nov. 1, 1951).)

## <a id="s-b-scattering-of-a-photon-by-an-electron-the-compton-effect-the-klein-nishina-formula"></a>B. Scattering of a Photon by an Electron. The Compton Effect. Klein-Nishina Formula.

Again we use the same operator $U_{2}$ given by (423). We only have to calculate its matrix element $M_{2}$ between an initial state $A$ and a final state $B$, where $A$ consists of an electron with wave-function

<a id="eq-437"></a>

$$
ue^{ip \cdot x} \tag{437}
$$

and a photon with <a id="ix-194">potentials</a> given by

<a id="eq-438"></a>

$$
A_{\mu} = e_{\mu} e^{ik \cdot x} \tag{438}
$$

and $B$ consists of the electron in state

<a id="eq-439"></a>

$$
u^{\prime} e^{ip^{\prime} \cdot x} \tag{439}
$$

and the photon with potentials

<a id="eq-440"></a>

$$
A_{\mu} = e_{\mu}^{\prime} e^{ik^{\prime} \cdot x} \tag{440}
$$

The operator $A_{\lambda} ( x_{1} )$ appearing in (423) contains both photon emission and absorption components according to (211), and so does $A_{\mu} ( x_{2} )$. Hence the matrix element $M_{2}$ is a sum of contributions; we can either take $e_{\lambda} e^{ik \cdot x_{1}}$ from $A_{\lambda} ( x_{1} )$ and<sup><a id="fnref-57" href="#fn-57">57</a></sup> $e_{\mu}^{\prime} e^{ik^{\prime} \cdot x_{2}}$ from $A_{\mu} ( x_{2} )$ or vice versa. Likewise the electron can be absorbed by $\psi ( x_{2} )$ or emitted again by $\overline{\psi} ( x_{1} )$, or vice versa. Thus altogether we find for $M_{2}$, after taking into account that the whole expression is symmetrical in $x_{1}$ and $x_{2}$,

<a id="eq-441"></a>

$$
\begin{aligned} M_{2} &= \frac{e^{2}}{\hbar^{2} c^{2}} \sum_{\lambda ,\mu ,\alpha ,\beta} \int \int dx_{1} dx_{2} \left\{ \exp ( ip \cdot x_{2} - ip^{\prime} \cdot x_{1} ) ( \overline{u}_{1}^{\prime} \gamma_{\lambda} )_{\alpha} \left\langle \epsilon ( x_{1} - x_{2} ) P \left\{ \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \right\} \right\rangle_{o} ( \gamma_{\mu} u )_{\beta} \right\} \times \\ & \quad \left\{ e_{\lambda} e_{\mu}^{\prime} \exp ( ik \cdot x_{1} - ik^{\prime} \cdot x_{2} ) + e_{\mu} e_{\lambda}^{\prime} \exp ( ik \cdot x_{2} - ik^{\prime} \cdot x_{1} ) \right\} \end{aligned} \tag{441}
$$

Note that with <a id="ix-41">anticommuting</a> fields the expression $\epsilon ( x_{1} - x_{2} ) P \left\{ \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \right\}$ is a relativistic invariant whereas the $P$-product by itself is not. Thus in analogy with (427) we write

<a id="eq-442"></a>

$$
\left\langle \epsilon ( x_{1} - x_{2} ) P \left\{ \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \right\} \right\rangle_{o} = - \tfrac{1}{2} S_{F\alpha \beta} ( x_{1} - x_{2} ) \tag{442}
$$

<a id="p85"><sup>[p. 85]</sup></a>

where $S_{F}$ is a new invariant function. Since

<a id="eq-443"></a>

$$
\epsilon ( x_{2} - x_{1} ) P \left\{ \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \right\} = \tfrac{1}{2} \boldsymbol{[} \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \boldsymbol{]} + \tfrac{1}{2} \epsilon ( x_{2} - x_{1} ) \left\{ \psi_{\alpha} ( x_{1} ) , \overline{\psi}_{\beta} ( x_{2} ) \right\} \tag{443}
$$

we have by (299) and (329)

<a id="eq-444"></a>

$$
S_{F} ( x ) = S^{( 1 )} + i\epsilon ( x ) S ( x ) \tag{444}
$$

in exact analogy to (429). We write also

<a id="eq-445"></a>

$$
S_{F} ( x ) = \sum_{\lambda} \left( \gamma_{\lambda} \frac{\partial}{\partial x_{\lambda}} - \mu \right) \Delta_{F} ( x ) \tag{445}
$$

and we find from (444) the momentum <a id="ix-129">representations</a>

<a id="eq-446"></a>

$$
\Delta_{F} ( x ) = \frac{- 2 i}{( 2 \pi )^{4}} \int e^{ik \cdot x} \frac{d^{4} k}{k^{2} + \mu^{2} - i\epsilon} \tag{446}
$$

<a id="eq-447"></a>

$$
S_{F} ( x ) = \frac{2}{( 2 \pi )^{4}} \int e^{ik \cdot x} \left( \frac{/ k + i\mu}{k^{2} + \mu^{2} - i\epsilon} \right) d^{4} k \tag{447}
$$

We can also write (447) conveniently as

<a id="eq-448"></a>

$$
S_{F} ( x ) = \frac{2}{( 2 \pi )^{4}} \int_{F} e^{ik \cdot x} \frac{1}{k / - i\mu} d^{4} k \tag{448}
$$

Here the Dirac matrix in the denominator means that we have to multiply above and below by $( / k + i\mu )$ in order to evaluate the integral. Thus (448) is not a real simplification of (447), only it saves writing. The Feynman integration in (448) is defined as a <a id="ix-67">contour integral</a> exactly as in (430).

Now substituting from (448) into (441), just like for (432)

<a id="eq-449"></a>

$$
M_{2} = \frac{- e^{2}}{\hbar^{2} c^{2}} ( 2 \pi )^{4} \delta^{4} ( p + k - p^{\prime} - k^{\prime} ) \overline{u}^{\prime} \left[ /e \frac{1}{p / - k /^{\prime} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{p / + / k - i\mu} /e \right] u \tag{449}
$$

Again the denominators $( p - k )$<sup>2</sup> $+ \mu^{2}$ can never vanish so the $\epsilon$ can be put equal to zero in (447). In fact if the electron is initially at rest

$$
( p - k^{\prime} )^{2} + \mu^{2} = ( p^{2} + \mu^{2} ) + k^{\prime 2} - 2 p \cdot k^{\prime} = 2 p_{0} k_{0}^{\prime} = 2 \mu k_{0}^{\prime}
$$

and similarly

<a id="eq-450"></a>

$$
( p + k )^{2} + \mu^{2} = - 2 \mu k_{0} \tag{450}
$$

because $k^{\prime 2}$ = 0 and $p^{2} = - \mu^{2} ; \boldsymbol{p}$ = 0 because the electron is at rest.

<a id="eq-451"></a>

$$
M_{2} = \frac{- e^{2}}{2 \hbar^{2} c^{2} \mu} ( 2 \pi )^{4} \delta^{4} ( p + k - p^{\prime} - k^{\prime} ) \overline{u}^{\prime} \left[ \frac{1}{k_{0}^{\prime}} /e ( p / - / k^{\prime} + i\mu ) /e^{\prime} - \frac{1}{k_{0}} /e^{\prime} ( p / + / k + i\mu ) /e \right] u \tag{451}
$$

Now we can simplify (451) further. Since the photon is not polarized in time, $e_{4} = e_{4}^{\prime}$ = 0; since the electron is at rest, $\boldsymbol{p}$ = 0. Hence $e \cdot p$ = 0, and thus $/e/p = - p/e /$ + 2$\mathbb{I} e \cdot p = - p/e /$ , i.e. $p /$ and $/e$ anticommute. This plus the fact that $u$ is a spin state of momentum $\hbar p$, i.e.

<a id="eq-452"></a>

$$
( p / - i\mu ) u = 0 \tag{452}
$$

mean that the term $p / + i\mu$ in (451) can be omitted. Thus we get

<a id="eq-453"></a>

$$
M_{2} = \frac{e^{2}}{2 \hbar^{2} c^{2} \mu} ( 2 \pi )^{4} \delta^{4} ( p + k - p^{\prime} - k^{\prime} ) \overline{u}^{\prime} \left[ /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{/ k}{k_{0}} /e \right] u \tag{453}
$$

<a id="p86"><sup>[p. 86]</sup></a>

### <a id="s-calculation-of-cross-section"></a>Calculation of the Cross-Section

We write as in (145)

<a id="eq-454"></a>

$$
M_{2} = K ( 2 \pi )^{4} \delta^{4} ( p + k - p^{\prime} - k^{\prime} ) \tag{454}
$$

Then the <a id="ix-117">scattering</a> probability per unit volume and per unit time for the single final state is as before

<a id="eq-455"></a>

$$
w_{\delta} = c | K |^{2} ( 2 \pi )^{4} \delta^{4} ( p + k - p^{\prime} - k^{\prime} ) \tag{455}
$$

The number of final states for the electron is

<a id="eq-456"></a>

$$
\frac{1}{( 2 \pi )^{3}} \left( \frac{mc^{2}}{E^{\prime}} \right) dp_{1}^{\prime} dp_{2}^{\prime} dp_{3}^{\prime} \tag{456}
$$

The photon with potentials (440), when $e_{\mu}^{\prime}$ is a space-like vector having $( e_{\mu}^{\prime} )$<sup>2</sup> = 1 is normalized to one particle per volume $\hbar c/ 2 k_{0}^{\prime}$. This can be seen at once, comparing (440) with (211) and (214), and taking into account the difference of (2$\pi )$<sup>3</sup> between the continuous and discrete normalizations. Hence the number of final states for the photon is

<a id="eq-457"></a>

$$
\frac{1}{( 2 \pi )^{3}} \left( \frac{\hbar c}{2 k_{0}^{\prime}} \right) dk_{1}^{\prime} dk_{2}^{\prime} dk_{3}^{\prime} \tag{457}
$$

The total transition probability is thus

<a id="eq-458"></a>

$$
w = c | K |^{2} \frac{1}{( 2 \pi )^{2}} \left( \frac{mc^{2}}{E^{\prime}} \frac{\hbar c}{2 k_{0}^{\prime}} \right) \frac{dk_{1}^{\prime} dk_{2}^{\prime} dk_{3}^{\prime}}{d ( p_{0}^{\prime} + k_{0}^{\prime} )} \tag{458}
$$

We write this as a probability for scattering the _photon_ with frequency $k_{0}^{\prime}$ into a unit solid angle $d$Ω. Then using momentum conservation, we have

<a id="eq-459"></a>

$$
\begin{aligned} \frac{dp_{0}^{\prime}}{dk_{0}^{\prime}} &= \frac{p_{1}^{\prime} dp_{1}^{\prime} + \dots}{p_{0}^{\prime} dk_{0}^{\prime}} = - \frac{p_{1}^{\prime} dk_{1}^{\prime} + \dots}{p_{0}^{\prime} dk_{0}^{\prime}} = - \frac{p_{1}^{\prime} k_{1}^{\prime} + \cdot \cdot \cdot + p_{3}^{\prime} k_{3}^{\prime}}{p_{0}^{\prime} k_{0}^{\prime}} \\ \frac{dk_{0}^{\prime}}{d ( p_{0}^{\prime} + k_{0}^{\prime} )} &= \frac{p_{0}^{\prime} k_{0}^{\prime}}{- ( p^{\prime} \cdot k^{\prime} )} = \frac{p_{0}^{\prime} k_{0}^{\prime}}{- p \cdot k} = \frac{p_{0}^{\prime} k_{0}^{\prime}}{p_{0} k_{0}} \end{aligned} \tag{459}
$$

Hence finally

<a id="eq-460"></a>

$$
w = c | K |^{2} \frac{1}{( 2 \pi )^{2}} \frac{\hbar c}{2 k_{0}} k_{0}^{\prime 2} d \Omega \tag{460}
$$

The differential cross-section for scattering the photon of frequency $k_{0}$ into solid angle $d$Ω is then

$$
\sigma = \frac{wV_{1} V_{2}}{c}
$$

where $V_{1}$ is according to (149) the electron normalization volume $\frac{mc^{2}}{E}$ = 1 and $V_{2}$ is the photon volume,

$V_{2} = \frac{\hbar c}{2 k_{0}}$ . Thus

<a id="eq-461"></a>

$$
\sigma = \left( \frac{\hbar c}{4 \pi k_{0}} \right)^{2} | K |^{2} k_{0}^{\prime 2} d \Omega \tag{461}
$$

<a id="eq-462"></a>

$$
K = \frac{e^{2}}{2 \hbar^{2} c^{2} \mu} \overline{u}^{\prime} \left[ /e \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{k /}{k_{0}} /e \right] u \tag{462}
$$

This gives the cross-section for a known electron spin in the initial and final states.

<a id="p87"><sup>[p. 87]</sup></a>

### <a id="s-sum-over-spins"></a>Sum over Spins

Experimentally we do not observe the electron spins. Hence we observe only the cross-section $\overline{\sigma}$ obtained by averaging $\sigma$ over the two spin states $u$ and summing over the two spin states $u^{\prime}$. The summing and averaging we do by the method of projection operators according to (109), (114).

Here are some rules for spurs and dagger operators:

<div align="center">

$\xi$ is a Dirac matrix in general.

</div>

1) Spur $\left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( 2 k - 1 )} \right)$ = 0, i.e.the spur of an odd number of factors is 0.

2) Spur $\left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( 2 k )} \right)$ = Spur $\left\{ P \left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( 2 k )} \right) \right\}$ where $P$ is any cyclic permutation.

This is clear, because any cyclic permutation consists of steps of the form

$$
\left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( s )} \right) \xi^{( m )} \to \xi^{( m )} \left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( s )} \right)
$$

and for two square matrices<sup><a id="fnref-58" href="#fn-58">58</a></sup> $A , B$

$$
\operatorname{Spur} AB = \sum_{i,j} a_{ij} b_{ji} = \operatorname{Spur} BA
$$

$$
3 ) \operatorname{Spur} \left( \xi^{( 1 )} \xi^{( 2 )} \dots \xi^{( 2 k - 1 )} \xi^{( 2 k )} \right) = \operatorname{Spur} \left( \xi^{( 2 k )} \xi^{( 2 k - 1 )} \dots \xi^{( 2 )} \xi^{( 1 )} \right)
$$

To show this, it is enough to assume that all $\xi^{( i )}$ are different; by the commutation rules of the $\gamma$’s we can always reduce the product to this form. Then, since each inversion (of neighbors) brings in a minus sign, and since there are an even number of inversions, we get our rule immediately.

$$
4 ) a/b / = - /b/a + 2 \mathbb{I} ( a \cdot b )
$$

In particular

$$
/e/e = \mathbb{I} ( e \cdot e ) \quad /e/ k = - / k/e \quad /e^{\prime} k /^{\prime} = - k /^{\prime} /e^{\prime}
$$

5) By 2), one can cyclically permute a product of dagger operators without changing its spur.

Now we can proceed to evaluate the sum over spins. We have

<a id="eq-463"></a>

$$
\begin{aligned} \tfrac{1}{2} \sum_{u} \sum_{u^{\prime}} | K |^{2} &= - \frac{e^{4}}{8 \hbar^{4} c^{4} \mu^{2}} \sum_{u} \sum_{u^{\prime}} \left\{ \overline{u}^{\prime} \left( /e \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{/ k}{k_{0}} /e \right) u \right\} \left\{ \overline{u} \left( /e \frac{k /}{k_{0}} /e^{\prime} + /e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e \right) u^{\prime} \right\} \\ &= \frac{e^{4}}{3 2 \hbar^{4} c^{4} \mu^{4}} \operatorname{Spur} \left\{ ( p / + i\mu ) \left( /e \frac{/ k}{k_{0}} /e^{\prime} + /e^{\prime} \frac{k /^{\prime}}{k_{0}^{\prime}} /e \right) ( p /^{\prime} + i\mu ) \left( /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{k /}{k_{0}} /e \right) \right\} \\ \text{ Now } \frac{/ k}{k_{0}} &= i\beta + \frac{k_{1} \gamma_{1} + k_{2} \gamma_{2} + k_{3} \gamma_{3}}{k_{0}} = i\beta + \gamma_{k} , \text{ say } . \end{aligned} \tag{463}
$$

Similarly $\frac{k /^{\prime}}{k_{0}^{\prime}} = i\beta + \gamma_{k}^{\prime}$.

Since $/p$ <a id="ix-42">anticommutes</a> with $/e , /e^{\prime} , \gamma_{k}$ and $\gamma_{k}^{\prime}$, (compare with remark after (451)) we may write (463) in the form

$$
\frac{e^{4}}{3 2 \hbar^{4} c^{4} \mu^{4}} \operatorname{Spur} \left[ \left\{ \left( /e \frac{k /}{k_{0}} /e^{\prime} + /e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e \right) ( i\mu - p / ) + 4 \mu ( e \cdot e^{\prime} ) \right\} ( p /^{\prime} + i\mu ) \left\{ /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{/ k}{k_{0}} /e \right\} \right]
$$

<a id="p88"><sup>[p. 88]</sup></a>

because, using 4), and again compare with remark after (451),<sup><a id="fnref-59" href="#fn-59">59</a></sup>

$$
p/e / \frac{k /}{k_{0}} /e^{\prime} = - /e/p \frac{k /}{k_{0}} /e^{\prime} = + /e \frac{/ k}{k_{0}} p/e /^{\prime} - 2 /e \frac{k \cdot p}{k_{0}} /e^{\prime} = - /e \frac{k /}{k_{0}} /e^{\prime} p / + 2 \mu /e/e^{\prime}
$$

and similarly<sup><a id="fnref-60" href="#fn-60">60</a></sup>

$$
p/e /^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e = - /e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e/p + 2 \mu /e^{\prime} /e
$$

so adding the terms gives

$$
- \left( /e \frac{/ k}{k_{0}} /e^{\prime} + /e^{\prime} \frac{k /^{\prime}}{k_{0}^{\prime}} /e \right) p / + 2 \mu \{ /e,/e^{\prime} \} = - \left( /e \frac{/ k}{k_{0}} /e^{\prime} + /e^{\prime} \frac{k /^{\prime}}{k_{0}^{\prime}} /e \right) p / + 4 \mu ( e \cdot e^{\prime} )
$$

Now since $k/ / k = / k^{\prime} k /^{\prime} = p/p / + \mu^{2}$ = 0 (for the photons because they are on the lightcone, for the electron because $p^{2} = - \mu^{2} )$, we get for $| K |^{2}$

<a id="eq-464"></a>

$$
\frac{e^{4}}{3 2 \hbar^{4} c^{4} \mu^{4}} \operatorname{Spur} \left[ 4 \mu ( e \cdot e^{\prime} ) ( p /^{\prime} + i\mu ) ( /e \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e^{\prime} \frac{/ k}{k_{0}} /e ) + ( /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} /e \frac{/ k}{k_{0}} /e^{\prime} + /e^{\prime} \frac{/ k}{k_{0}} /e/e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e ) ( i\mu - p / ) ( / k - k /^{\prime} ) \right] \tag{464}
$$

because

$$
( i\mu - p / ) ( p /^{\prime} + i\mu ) = i\mu ( p /^{\prime} - p / ) - \mu^{2} - p/p /^{\prime} = i\mu ( / k - k /^{\prime} ) - \mu^{2} - p / ( p / - / k^{\prime} + / k ) = ( i\mu - p / ) ( / k - / k^{\prime} )
$$

We consider the second part of (464) first:

$$
\operatorname{Spur} \left[ /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} /e \frac{k /}{k_{0}} /e^{\prime} \left\{ - p / ( k / - / k^{\prime} ) - ( k / - / k^{\prime} ) p / \right\} \right] = 2 p_{0} ( k_{0} - k_{0}^{\prime} ) \operatorname{Spur} \left[ /e/e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} \frac{/ k}{k_{0}} /e/e^{\prime} \right]
$$

$$
= 2 \mu ( k_{0} - k_{0}^{\prime} ) \operatorname{Spur} \left[ - 2 ( e \cdot e^{\prime} ) \left( \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} \frac{/ k}{k_{0}} /e \right) - \frac{/ k^{\prime}}{k_{0}^{\prime}} \frac{k /}{k_{0}} \right]
$$

$$
= - 8 \mu \frac{k_{0} - k_{0}^{\prime}}{k_{0} k_{0}^{\prime}} ( k \cdot k^{\prime} ) + 4 \mu ( e \cdot e^{\prime} ) \operatorname{Spur} \left[ - / k/e \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + / k^{\prime} /e^{\prime} \frac{/ k}{k_{0}} /e \right]
$$

since

$$
\operatorname{Sp} \left[ /e/e^{\prime} / k^{\prime} / k/e/e^{\prime} \right] = \operatorname{Sp} \left[ /e/e^{\prime} /e/e^{\prime} / k^{\prime} k / \right] = \operatorname{Sp} \left[ - /e/e/e^{\prime} /e^{\prime} k /^{\prime} / k + 2 \mathbb{I} e/e /^{\prime} / k^{\prime} k / ( e \cdot e^{\prime} ) \right] = \operatorname{Sp} \left[ - / k^{\prime} k / - 2 ( e \cdot e^{\prime} ) / k^{\prime} /e^{\prime} / k/e \right]
$$

Hence altogether (464) becomes

$$
\text{ But } / k/e/ k = k /^{\prime} \frac{e^{4}}{3 2 \hbar^{4} c^{4} \mu^{4}} /e^{\prime} / k^{\prime} = 0 \text{ and } \left\{ - 8 \mu \frac{k_{0} - k_{0}^{\prime}}{k_{0} k_{0}^{\prime}} ( k \cdot k^{\prime} ) + 4 \mu ( e \cdot e^{\prime} ) \operatorname{Spur} \left[ ( p / - k /^{\prime} ) /e \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} + ( p / + / k ) /e^{\prime} \frac{k /}{k_{0}} /e \right] \right\}
$$

$$
( p^{\prime} - p )^{2} = ( k - k^{\prime} )^{2} = k^{2} + k^{\prime 2} - 2 k \cdot k^{\prime} = - 2 k \cdot k^{\prime}
$$

$$
( p^{\prime} - p )^{2} = p^{\prime 2} + p^{2} - 2 p^{\prime} \cdot p = 2 \mu^{2} + 2 \mu p_{0}^{\prime} = 2 \mu^{2} + 2 \mu ( - \mu + k_{0} - k_{0}^{\prime} ) = 2 \mu ( k_{0} - k_{0}^{\prime} )
$$

Hence $k \cdot k^{\prime} = - \mu ( k_{0} - k_{0}^{\prime} )$ and then

$$
\operatorname{Sp} \left[ ( /p - / k^{\prime} ) /e \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + ( p / + / k ) /e^{\prime} \frac{k /}{k_{0}} /e \right] = \operatorname{Sp} \left[ p/e / \frac{/ k^{\prime}}{k_{0}^{\prime}} /e^{\prime} - / k^{\prime} /e^{\prime} \frac{/ k^{\prime}}{k_{0}^{\prime}} /e + p/e /^{\prime} \frac{/ k}{k_{0}} /e + / k/e \frac{k /}{k_{0}} /e^{\prime} \right]
$$

$$
= \operatorname{Sp} \left[ p/e / \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + /e \frac{/ k}{k_{0}} /e^{\prime} p / \right] = \operatorname{Sp} \left[ p/e / \frac{k /^{\prime}}{k_{0}^{\prime}} /e^{\prime} + p/e / \frac{/ k}{k_{0}} /e^{\prime} \right]
$$

<a id="p89"><sup>[p. 89]</sup></a>

and so (464) is

<a id="eq-465"></a>

$$
\frac{e^{4}}{8 \hbar^{4} c^{4} \mu^{4}} \left\{ \frac{2 \mu^{2} ( k_{0} - k_{0}^{\prime} )^{2}}{k_{0} k_{0}^{\prime}} + 2 \mu ( e \cdot e^{\prime} ) \operatorname{Spur} \left[ p/ei\beta /e /^{\prime} \right] \right\} = \frac{e^{4}}{4 \hbar^{4} c^{4} \mu^{2}} \left\{ \frac{( k_{0} - k_{0}^{\prime} )^{2}}{k_{0} k_{0}^{\prime}} + 4 ( e \cdot e^{\prime} )^{2} \right\} \tag{465}
$$

Hence by (461) the cross-section averaged over electron spins is

$$
\overline{\sigma} = \frac{e^{4} k_{0}^{\prime 2} d \Omega}{6 4 \pi^{2} \hbar^{2} c^{2} \mu^{2} k_{0}^{2}} \left\{ \frac{( k_{0} - k_{0}^{\prime} )^{2}}{k_{0} k_{0}^{\prime}} + 4 ( e \cdot e^{\prime} )^{2} \right\}
$$

The classical electron radius is

$$
r_{o} = \frac{e^{2}}{4 \pi mc^{2}} = \frac{e^{2}}{4 \pi \hbar c\mu}
$$

Hence

<a id="eq-466"></a>

$$
\overline{\sigma} = \tfrac{1}{4} r_{o}^{2} d \Omega \left( \frac{k_{0}^{\prime}}{k_{0}} \right)^{2} \left\{ \frac{( k_{0} - k_{0}^{\prime} )^{2}}{k_{0} k_{0}^{\prime}} + 4 \cos^{2} \varphi \right\} \tag{466}
$$

where $\varphi$ is the angle between the polarizations of the incident quantum $k_{0}$ and the emitted quantum $k_{0}^{\prime}$ . This is the famous _Klein-Nishina formula_.

To put $\overline{\sigma}$ explicitly as a function of the scattering angle $\theta$, we must use the equations<sup><a id="fnref-61" href="#fn-61">61</a></sup>

$$
k \cdot k^{\prime} = - \mu ( k_{0} - k_{0}^{\prime} )
$$

$$
k \cdot k^{\prime} = | \boldsymbol{k} || \boldsymbol{k}^{\prime} | \cos \theta - k_{0} k_{0}^{\prime} = k_{0} k_{0}^{\prime} ( \cos \theta - 1 )
$$

$$
k_{0} k_{0}^{\prime} ( 1 - \cos \theta ) = \mu ( k_{0} - k_{0}^{\prime} )
$$

$$
\frac{k_{0}}{k_{0}^{\prime}} = 1 + ( 1 - \cos \theta ) \frac{k_{0}}{\mu}
$$

Put

$$
\epsilon = \frac{k_{0}}{\mu} = \left( \frac{\text{ Photon energy }}{mc^{2}} \right)
$$

Then

<a id="eq-467"></a>

$$
\overline{\sigma} = \tfrac{1}{4} r_{o}^{2} d \Omega \frac{\left( \frac{( 1 - \cos \theta )^{2} \epsilon^{2}}{1 + \epsilon ( 1 - \cos \theta )} + 4 \cos^{2} \varphi \right)}{[ 1 + \epsilon ( 1 - \cos \theta ) ]^{2}} \tag{467}
$$

Thus for large $\epsilon$ the scattered photons are mainly unpolarized and concentrated in the forward direction. For small $\epsilon$ (non-relativistic problem) we have simply<sup><a id="fnref-62" href="#fn-62">62</a></sup>

<a id="eq-468"></a>

$$
\overline{\sigma} = r_{o}^{2} \cos^{2} \varphi d \Omega \tag{468}
$$

the classical result. Summing over the two polarizations of the photon $k^{\prime}$ and averaging over all polarizations of $k$, this gives the cross-section for all polarizations

<a id="eq-469"></a>

$$
\overline{\sigma} = \tfrac{1}{2} r_{o}^{2} ( 1 + \cos^{2} \theta ) d \Omega \tag{469}
$$

We get this by evaluating $\tfrac{1}{2} \sum_{e} \sum_{e^{\prime}} ( e \cdot e^{\prime} )$<sup>2</sup>. First we have to sum over the two polarization directions of photon $k^{\prime}$. This summation for three directions would give

$$
\sum ( e \cdot e^{\prime} )^{2} = e^{2} = 1
$$

Hence for the two directions perpendicular to $k^{\prime}$ we can write

$$
\sum_{e^{\prime}} ( e \cdot e^{\prime} )^{2} = 1 - ( \boldsymbol{e} \cdot \hat{\boldsymbol{k}}^{\prime} )^{2}
$$

<a id="p90"><sup>[p. 90]</sup></a>

Now we perform the other summation over the two polarization directions of photon $k$, using the same argument. This gives

$$
\sum_{e} \sum_{e^{\prime}} ( e \cdot e^{\prime} )^{2} = \sum_{e} \left[ 1 - ( \boldsymbol{e} \cdot \hat{\boldsymbol{k}}^{\prime} )^{2} \right] = 2 - \left[ \hat{\boldsymbol{k}}^{\prime 2} - ( \hat{\boldsymbol{k}} \cdot \hat{\boldsymbol{k}}^{\prime} )^{2} \right] = 1 + \cos^{2} \theta
$$

This with the averaging factor of $\frac{1}{2}$ gives (469).

The total cross-section then is

<a id="eq-470"></a>

$$
\sigma = \tfrac{8}{3} \pi r_{o}^{2} \tag{470}
$$

This non-relativistic scattering given by (468) – (470) is called _Thomson scattering_.<sup><a id="fnref-63" href="#fn-63">63</a></sup>

## <a id="s-c-two-quantum-pair-annihilation"></a>C. Two Quantum Pair Annihilation

Consider a process by which an electron in the state $( p,u )$ and a positron associated with the wave-function (439) are both annihilated, with the emission of two photons given by the potentials (438) and (440). The positron momentum-energy 4-vector is then $( - \hbar p^{\prime} )$ so we write $p_{+} = - p^{\prime}$. The positron spinor in the charge-conjugate representation is $v = Cu^{\prime +}$.

This annihilation process will again be effected by the operator $U_{2}$ given by (423). And the matrix element for the transition is exactly as before given by an expression identical with (449) except that $k$ is now replaced by $- k$, namely

<a id="eq-471"></a>

$$
\begin{aligned} M_{2} &= - \frac{e^{2} ( 2 \pi )^{4}}{\hbar^{2} c^{2}} \delta^{4} ( p + p_{+} - k - k^{\prime} ) \overline{u}^{\prime} \left\{ /e \frac{1}{p / - / k^{\prime} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{p / - k / - i\mu} /e \right\} u \\ &= K ( 2 \pi )^{4} \delta^{4} ( p + p_{+} - k - k^{\prime} ) \end{aligned} \tag{471}
$$

We consider the probability for this process for an electron and positron _both at rest_. The result will then apply to the decay of a positronium atom, where the velocities are only of the order of $\alpha c$ and can be treated as zero to a good approximation.

Then

<a id="eq-472"></a>

$$
\begin{aligned} p &= p_{+} = ( 0 , 0 , 0 ,i\mu ) \\ k_{0} &= k_{0}^{\prime} = \mu \end{aligned} \tag{472}
$$

As in (453) we have

<a id="eq-473"></a>

$$
K = \frac{e^{2}}{2 \hbar^{2} c^{2} \mu^{2}} \overline{u}^{\prime} ( /e/ k^{\prime} /e^{\prime} + /e^{\prime} / k/e ) u \tag{473}
$$

The decay probability per unit volume per unit time into a solid angle $d$Ω for _one_ of the photons is

$$
w = c | K |^{2} \frac{1}{( 2 \pi )^{2}} \left( \frac{\hbar c}{2 \mu} \right)^{2} \frac{dk_{1} dk_{2} dk_{3}}{d ( k_{0} + k_{0}^{\prime} )}
$$

(because $k_{0} = \mu$ here) in analogy to (458). But now $d ( k_{0} + k_{0}^{\prime} )$ = 2$dk_{0}^{\prime}$ and so

<a id="eq-474"></a>

$$
w = c | K |^{2} \frac{1}{( 2 \pi )^{2}} \frac{1}{8} \hbar^{2} c^{2} d \Omega \tag{474}
$$

For _parallel_ polarizations, $e = e^{\prime}$, and

$$
( /e/ k^{\prime} /e^{\prime} + /e^{\prime} k/e / ) = - ( / k^{\prime} + / k ) = - 2 i\mu \beta
$$

But $\beta$ has zero matrix element between the spin-states $u$ and $u^{\prime}$, which are positive and negative-frequency states both of zero momentum. Hence for parallel polarizations

<a id="eq-475"></a>

$$
w = 0 \tag{475}
$$

<a id="p91"><sup>[p. 91]</sup></a>

For _perpendicular_ polarizations, take coordinate axes 1 along $e$, 2 along $e^{\prime}$, and 3 along $k$. Then

<a id="eq-476"></a>

$$
( /e/ k^{\prime} /e^{\prime} + /e^{\prime} / k/e ) = \mu \{ \gamma_{1} ( - \gamma_{3} + i\beta ) \gamma_{2} + \gamma_{2} ( \gamma_{3} + i\beta ) \gamma_{1} \} = 2 \mu \gamma_{1} \gamma_{2} \gamma_{3} \tag{476}
$$

Hence for perpendicular polarizations<sup><a id="fnref-64" href="#fn-64">64</a></sup>

$$
\overline{u} [ /e/ k^{\prime} /e^{\prime} + /e^{\prime} / k/e ] u = 2 \mu v^{T} C\gamma_{4} \gamma_{1} \gamma_{2} \gamma_{3} u = - 2 \mu iv^{T} \gamma_{4} \sigma_{2} u
$$

$$
= 2 \mu v^{T} \begin{bmatrix} 0 & - 1 & 0 & 0 \\ 1 & 0 & 0 & 0 \\ 0 & 0 & 0 & 1 \\ 0 & 0 & - 1 & 0 \end{bmatrix} u
$$

0 when spins $u$ and $v$ are parallel

$$
= \{ \quad \sqrt{\}}
$$

$2 \mu$ 2 when spins $u$ and $v$ are antiparallel

We get this latter result by observing that for antiparallel spins the initial wave function is

$$
\psi = \frac{1}{\sqrt{2}} \begin{pmatrix} 1 \\ 0 \end{pmatrix} \begin{pmatrix} 0 \\ 1 \end{pmatrix} - \frac{1}{\sqrt{2}} \begin{pmatrix} 0 \\ 1 \end{pmatrix} \begin{pmatrix} 1 \\ 0 \end{pmatrix}
$$

(neglecting the “small components”) and therefore

$$
v^{T} \begin{bmatrix} 0 & - 1 \\ 1 & 0 \end{bmatrix} u = \frac{1}{\sqrt{2}} ( 0 1 ) \begin{bmatrix} 0 & - 1 \\ 1 & 0 \end{bmatrix} \begin{pmatrix} 1 \\ 0 \end{pmatrix} - \frac{1}{\sqrt{2}} ( 1 0 ) \begin{bmatrix} 0 & - 1 \\ 1 & 0 \end{bmatrix} \begin{pmatrix} 0 \\ 1 \end{pmatrix}
$$

$$
= \frac{1}{\sqrt{2}} \left[ ( 1 0 ) \begin{pmatrix} 1 \\ 0 \end{pmatrix} - ( 0 - 1 ) \begin{pmatrix} 0 \\ 1 \end{pmatrix} \right] = \frac{2}{\sqrt{2}} = \sqrt{2}
$$

This is one place where charge-conjugate spinors are useful and necessary!

Summarizing, we find for electron and positron with spins parallel, in triplet state, the 2-photon decay is forbidden. This selection rule is in fact an exact one for positronium in the ground state 1$s$ triplet. Only 3-photon decay can occur and this makes the lifetime $\sim$ 1100 times longer. For electron and positron in singlet state, the 2-photon decay occurs with the photons always polarized perpendicularly to each other. The probability for the decay, integrating (474) over a solid <a id="ix-22">angle</a> 2$\pi$ since the photons are indistinguishable, is

<a id="eq-477"></a>

$$
w = \frac{\hbar^{2} c^{3}}{1 6 \pi} 2 | K |^{2} = \frac{2 e^{4}}{\hbar^{2} c\mu^{2} 8 \pi} = 4 \pi cr_{o}^{2} \tag{477}
$$

Formula (477) is for electron and positron normalized to one particle per unit volume. If the density of the electron probability relative to the position of the positron is $\rho$, then the mean <a id="ix-35">annihilation</a> life-time will be<sup><a id="fnref-65" href="#fn-65">65</a></sup> (the “classical electron radius” $r_{o} = e^{2} / ( 4 \pi mc^{2} )$ in Heaviside units)

<a id="eq-478"></a>

$$
\tau = \frac{1}{4 \pi cr_{o}^{2} \rho} \tag{478}
$$

For the positronium 1$s$ singlet state

<a id="eq-479"></a>

$$
\begin{aligned} \rho &= \frac{1}{8 \pi a_{o}^{3}} \quad a_{o} = \text{ Bohr radius } = 1 3 7^{2} r_{o} \\ \tau &= 2 \times 1 3 7^{4} \times \frac{a_{o}}{c} = 2 \times 1 3 7^{5} \times \frac{\hbar}{mc^{2}} \approx 1 . 2 \times 1 0^{- 1 0} \sec . \end{aligned} \tag{479}
$$

For slowly-moving electrons and positrons with relative velocity $v$, the annihilation cross-section according to (477) will be

<a id="eq-480"></a>

$$
4 \pi r_{o}^{2} \left( \frac{c}{v} \right) \quad \text{ singlet state } , \tag{480}
$$

proportional to 1 just like neutron cross-sections at low (thermal) energies.

<a id="p92"><sup>[p. 92]</sup></a>

## <a id="s-d-bremsstrahlung-and-pair-creation-in-the-coulomb-field-of-an-atom"></a>D. Bremsstrahlung and <a id="ix-36">Pair</a> Creation in the Coulomb Field of an Atom

We consider these two important processes together. Given an external potential $A_{\mu}^{e}$ representing the Coulomb field, the processes are:

_Bremsstrahlung_

Electron $( pu ) \to$ Electron $( p^{\prime} u^{\prime} )$ + Photon ($k^{\prime} e^{\prime} )$

_Pair-creation_

Photon $( k^{\prime} e^{\prime} ) \to$ Electron $( pu )$ + Positron ($p_{+}^{\prime} u^{\prime} )$

We treat not only the photon $( ke )$ but also the potential $A^{e}$ in _Born approximation._ This is valid so long as

<div align="center">

Potential energy $\times$ time of transit $\ll \hbar$

</div>

<a id="eq-481"></a>

$$
\text{ or } \text{ or } \quad \frac{Ze^{2}}{4 \pi \hbar v} \frac{Ze^{2}}{4 \pi r} \times \frac{r}{v} = \frac{Z}{1 3 7} \frac{c}{v} \ll \hbar \ll 1 \tag{481}
$$

The treatment will only be good for _relativistic velocities_ $v \sim c$, and for _lighter atoms_ $Z \ll$ 137. In fact for heavy atoms $( Z$ = 82 for lead) and $v \sim c$ the error from the Born approximation is about 10%.

The processes arise in the Born approximation just from the term linear in $A_{\mu}$ and linear in $A_{\mu}^{e}$ in (421). This term is

<a id="eq-482"></a>

$$
U_{1} = \frac{e^{2}}{\hbar^{2} c^{2}} \int \int dx_{1} dx_{2} P \left\{ \psi ( x_{1} ) A / ( x_{1} ) \psi ( x_{1} ) , \overline{\psi} ( x_{2} ) A /^{e} ( x_{2} ) \psi ( x_{2} ) \right\} \tag{482}
$$

The factor $\frac{1}{2}$ in (423) is now missing, otherwise everything is the same as before. We suppose $A_{\mu}^{e} ( x_{2} )$ is a superposition of Fourier components

<a id="eq-483"></a>

$$
A_{\mu}^{e} ( x_{2} ) = \frac{1}{( 2 \pi )^{4}} \int dkf ( k ) e_{\mu} e^{ik \cdot x_{2}} \tag{483}
$$

where $f ( k )$ is a known function of $k$. For a static Coulomb field all the vectors $k$ appearing in (483) have zero 4<sup>th</sup> component, and $e_{\mu}$ is the constant vector (0$, 0 , 0 ,i )$. We calculate the matrix element $M_{1}$ for bremsstrahlung or pair creation with $A_{\mu}^{e}$ given by the pure Fourier component (438) ; the results are then to be afterwards superposed to give the actual potential according to (483).

For bremsstrahlung the formula for $M_{1}$ is (449), identically the same as for the Compton effect, or integrating over the frequency $k$

<a id="eq-484"></a>

$$
M_{1} = - \frac{e^{2}}{\hbar^{2} c^{2}} f ( p^{\prime} + k^{\prime} - p ) \overline{u}^{\prime} \left\{ /e \frac{1}{p / - / k^{\prime} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{p /^{\prime} + / k^{\prime} - i\mu} /e \right\} u \tag{484}
$$

The factor 2 difference between (482) and (423) is just compensated by the fact that the photon $k^{\prime} e^{\prime}$ can be emitted by two operators $A_{\mu}$ in (423) and by only one in (482). The bremsstrahlung cross section is then calculated by squaring (484) and integrating over $k^{\prime}$ and $p^{\prime}$ with appropriate normalization factors. For the details see Heitler’s book $§ 1 7 .$<sup><a id="fnref-66" href="#fn-66">66</a></sup>

For pair creation the same formula (449) gives the matrix element $M_{1}$, allowing for the fact that the roles of the particles are now changed around so that electron $( pu )$ instead of ($p^{\prime} u^{\prime} )$ is created etc. Thus

<a id="eq-485"></a>

$$
M_{1} = - \frac{e^{2} ( 2 \pi )^{4} )}{\hbar^{2} c^{2}} \delta^{4} ( k + k^{\prime} - p - p_{+} ) \overline{u} \left\{ /e \frac{1}{/ k^{\prime} - p /_{+} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{/ k - p /_{+} - i\mu} /e \right\} u^{\prime} \tag{485}
$$

<a id="p93"><sup>[p. 93]</sup></a>

and integrating over the components of the potential

<a id="eq-486"></a>

$$
M_{1} = - \frac{e^{2}}{\hbar^{2} c^{2}} f ( p + p_{+} - k^{\prime} ) \overline{u} \left\{ /e \frac{1}{/ k^{\prime} - p /_{+} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{p / - / k^{\prime} - i\mu} /e \right\} u^{\prime} \tag{486}
$$

For the <a id="ix-34">cross-section</a> calculation see again Heitler, $§ 2 0 .$<sup><a id="fnref-67" href="#fn-67">67</a></sup>

---

## Notes

<a id="fn-56"></a>**56.** “from” replaces a second “in” [↩](#fnref-56)

<a id="fn-57"></a>**57.** The factor $x_{2}$ had a superfluous superscript “$\prime$”. [↩](#fnref-57)

<a id="fn-58"></a>**58.** $A , B$ added for clarity [↩](#fnref-58)

<a id="fn-59"></a>**59.** The words after “because”, “using 4) . . . (451)” were added. [↩](#fnref-59)

<a id="fn-60"></a>**60.** In the second edition, the intermediate calculation was wrong; however, the conclusion was correct. It was rewritten up to Eq. (464). See the trace theorems Eq. (585) _et seq._ Also, note the identity [↩](#fnref-60)

$$
\operatorname{Sp} ( / a_{1} / a_{2} a /_{3} / a_{4} ) = ( a_{1} \cdot a_{2} ) ( a_{3} \cdot a_{4} ) - ( a_{1} \cdot a_{3} ) ( a_{2} \cdot a_{4} ) + ( a_{1} \cdot a_{4} ) ( a_{2} \cdot a_{3} )
$$

<a id="fn-61"></a>**61.** The third equation lacked a subscript “0” on the variable $k_{0}^{\prime}$ ; the fourth equation lacked a superscript “$\prime$” on the variable $k_{0}^{\prime}$ . [↩](#fnref-61)

<a id="fn-62"></a>**62.** Eq. (468) lacked a label. The word “simply” was inserted. [↩](#fnref-62)

<a id="fn-63"></a>**63.** “Thomson” replaces “Thompson”. [↩](#fnref-63)

<a id="fn-64"></a>**64.** The first spinor $u$ lacked a bar; $\overline{u}$ replaces $u$. [↩](#fnref-64)

<a id="fn-65"></a>**65.** The expression for $r_{o}$ was added. [↩](#fnref-65)

<a id="fn-66"></a>**66.** In the 3<sup>rd</sup> edition of Heitler’s book, see $§ 2 5 .$ [↩](#fnref-66)

<a id="fn-67"></a>**67.** In the 3<sup>rd</sup> edition of Heitler’s book, see $§ 2 6 .$ [↩](#fnref-67)

---

## References

<a id="ref-18"></a>**[18]** E. C. G. Stueckelberg, “Une propriété de l’operateur $S$ en mécanique asymptotique”, _Helv. Phys. Acta_ **19** (1946) 242. See also D. Rivier & E. C. G. Stuecklberg (_sic_), “A convergent expression for the magnetic moment of the muon”, _Phys. Rev._ **74** (1948) 218. [↩](#refback-18-1)

---

<a id="07-general-theory-of-free-particle-scattering"></a>

# General Theory of Free Particle Scattering

<a id="p94"><sup>[p. 94]</sup></a>

We have shown how (421) leads to matrix elements for standard scattering processes, from which cross-sections can be calculated. In each case we used only the term $n$ = 2 in (421), which happened to be the lowest term giving a contribution to these processes. The higher terms $n$ = 4$, 6 ,\dots$ will also give contributions to the matrix elements for these processes, contributions which are collectively called “radiative corrections”. It turns out that the results without radiative corrections agree with the experimental scattering cross-sections in all cases. The experiments are never accurate to better than a few percent, and the radiative corrections are always smaller than the lowest-order terms by at least one power of $( e^{2} / 4 \pi \hbar c )$ = (1$/$137). Thus the study of the radiative corrections for scattering processes will not lead to any directly observable effects.

Nevertheless we shall work out a method of calculating the high-order terms of (421). This method turns out to be simplest and easiest to explain when we are discussing scattering problems. Incidentally we shall see what the radiative corrections to scattering look like, and we shall learn something about the nature of radiative corrections in general. Finally at the end we shall be able to use the method of calculation in order to find the radiative corrections to the motion of an electron in a hydrogen atom, which is the case in which these small effects can be accurately observed, but where the pure scattering theory is not directly applicable.

To avoid unnecessary complications we suppose there is no external field $A^{e}$. Problems in which there is an external field, so long as it can be treated in the <a id="ix-48">Born approximation</a>, can always be simply related to problems without external field, just as the <a id="ix-24">bremsstrahlung</a> matrix element (484) is related to the Compton <a id="ix-59">effect</a> (449). When there is no external field the matrix element for any scattering process is

<a id="eq-487"></a>

$$
M = ( \Phi_{B}^{*} S \Phi_{A} ) \tag{487}
$$

<a id="eq-488"></a>

$$
S = \sum_{n = 0}^{\infty} \left( \frac{e}{\hbar c} \right)^{n} \frac{1}{n !} \int \dots \int dx_{1} \dots dx_{n} P \left\{ \psi / A\psi ( x_{1} ) ,\dots , \psi / A\psi ( x_{n} ) \right\} \tag{488}
$$

The operators in (488) are field-operators of the interaction representation, the integrations over the points $x_{1} ,\dots x_{n}$ extend over all space-time, and the initial and final states $A$ and $B$ are entirely arbitrary.

We wish to calculate the matrix element $M$ of $S$ for a particular scattering process, in which the states $A$ and $B$ are specified by enumerating the particles present in the two states. We must now take properly into account the fact that the particles in states $A$ and $B$, although well separated and no longer interacting with one another, are real particles interacting with their self-fields and with the vacuum-fluctuations of the fields in their neighborhood. Thus $A$ and $B$ are really time-dependent states in the interaction representation and will not be given by time-independent vectors Φ$A$ and Φ$B$, except in the lowest-order approximation. (See page 81.) Let Ψ${}_{B} ( t )$ be the actual time-dependent state-vector of the state $B$ in the IR. We are not interested in the dependence of Φ${}_{B} ( t )$ on $t$. In an actual scattering experiment the particles in state B are observed in counters or photographic plates or cloud-chambers and the time of their arrival is not measured precisely. Therefore it is convenient to use for $B$ not the state-function Ψ${}_{B} ( t )$ but a state-function Φ$B$ which is _by definition_ the state-function describing a set of bare particles without radiation interaction, the bare particles having the same momenta and spins as the real particles in the state $B$. In the IR the state-function Φ$B$ is time-independent. The question is only, what is the connection between Ψ${}_{B} ( t )$ and Φ$B$?

<a id="p95"><sup>[p. 95]</sup></a>

Suppose $t_{B}$ to be a time so long in the future after the scattering process is over, that from $t_{B}$ to +$\infty$ the state $B$ consists of separated outward-traveling particles. Then the relation between Ψ${}_{B} ( t )$ and Φ$B$ is simple. We imagine a fictitious world in which the charge $e$ occurring in the radiation interaction decreases infinitely slowly (adiabatically) from its actual value at time $t_{B}$ to zero at time (+$\infty )$. In the fictitious world, the state Ψ${}_{B} ( t_{B} )$ at time $t_{B}$ will grow into the bare-particle state Φ$B$ at time +$\infty$. Thus

<a id="eq-489"></a>

$$
\Phi_{B} = \Omega_{2} ( t_{B} ) \Psi_{B} ( t_{B} ) \tag{489}
$$

where<sup><a id="fnref-68" href="#fn-68">68</a></sup>

<a id="eq-490"></a>

$$
\Omega_{2} ( t_{B} ) = \sum_{n = 0}^{\infty} \left( \frac{e}{\hbar c} \right)^{n} \frac{1}{n !} \int_{t_{B}}^{\infty} \dots \int_{t_{B}}^{\infty} dx_{1} \dots dx_{n} P \left\{ \psi / A\psi ( x_{1} ) ,\dots , \overline{\psi} A\psi / ( x_{n} ) \right\} g_{B} ( t_{1} ) \dots g_{B} ( t_{n} ) \tag{490}
$$

and $g_{B} ( t )$ is a function decreasing adiabatically from the value 1 at $t = t_{B}$ to zero at $t = \infty$. Similarly, when $t_{A}$ is a time so far in the past that the state $A$ consists of separated converging particles from $t = -\infty$ to $t = t_{A}$ we have

<a id="eq-491"></a>

$$
\Psi_{A} ( t_{A} ) = \Omega_{1} ( t_{A} ) \Phi_{A} \tag{491}
$$

<a id="eq-492"></a>

$$
\Omega_{1} ( t_{A} ) = \sum_{n = 0}^{\infty} \left( \frac{e}{\hbar c} \right)^{n} \frac{1}{n !} \int_{-\infty}^{t_{A}} \dots \int_{-\infty}^{t_{A}} dx_{1} \dots dx_{n} P \left\{ \psi / A\psi ( x_{1} ) ,\dots , \overline{\psi} A\psi / ( x_{n} ) \right\} g_{A} ( t_{1} ) \dots g_{A} ( t_{n} ) \tag{492}
$$

where $g_{A} ( t )$ is a function increasing adiabatically from $t = -\infty$ to $t = t_{A}$.

The scattering matrix element between states $A$ and $B$ is given exactly by

<a id="eq-493"></a>

$$
M = \left( \Psi_{B}^{*} ( t_{B} ) S_{t_{A}}^{t_{B}} \Psi_{A} ( t_{A} ) \right) \tag{493}
$$

<a id="eq-494"></a>

$$
S_{t_{A}}^{t_{B}} = \sum_{n = 0}^{\infty} \left( \frac{e}{\hbar c} \right)^{n} \frac{1}{n !} \int_{t_{A}}^{t_{B}} \dots \int_{t_{A}}^{t_{B}} P \left\{ \psi / A\psi ( x_{1} ) ,\dots , \overline{\psi} A\psi / ( x_{n} ) \right\} dx_{1} \dots dx_{n} \tag{494}
$$

Of course (493) is independent of the times $t_{A}$ and $t_{B}$. When $t_{A}$ and $t_{B}$ are chosen so far in the past that (489) and (491) are satisfied, then (493) may be written in the form (487), where now

<a id="eq-495"></a>

$$
\begin{aligned} S &= \Omega_{2} ( t_{B} ) S_{t_{A}}^{t_{B}} \Omega_{1} ( t_{A} ) \\ &= \sum_{n = 0}^{\infty} \left( \frac{e}{\hbar c} \right)^{n} \frac{1}{n !} \int_{-\infty}^{\infty} \dots \int_{-\infty}^{\infty} dx_{1} \dots dx_{n} P \left\{ \psi / A\psi ( x_{1} ) ,\dots , \overline{\psi} A\psi / ( x_{n} ) \right\} g ( t_{1} ) \dots g ( t_{n} ) \end{aligned} \tag{495}
$$

and $g ( t )$ is a function increasing adiabatically from 0 to 1 for $-\infty <t<t_{A}$, equal to 1 for $t_{A} \leq t \leq t_{B}$, and decreasing adiabatically from 1 to 0 for $t_{A} <t< \infty$. Thus we come to the important conclusion that formula (487) for the matrix element is correct, using the bare particle state-functions Φ$A$ and Φ$B$, provided that formula (488) for $S$ is interpreted by putting in the slowly varying cut-off function $g ( t_{i} )$ to make the integrals converge at $t_{i} = \pm \infty$. The cut-off functions are to be put in as they appear in (495), and then $S$ is defined as the limit to which (495) tends as the rate of variation of $g ( t )$ is made infinitely slow.

The main practical effect of this limiting process in the definition of $S$ is to justify us in throwing away all terms in the integrals which oscillate finitely at $t_{i} = \pm \infty$. There are however certain cases in which the integral (488) is in a more serious way ambiguous due to bad convergence at $t_{i} = \pm \infty$. In these cases the cut-off functions have to be kept explicitly until a late stage of the calculations before going to the limit $g ( t )$ = 1. In all cases, if the limiting process is done in this way, the matrix element $M$ is obtained correctly and unambiguously.

The use of bare-particle wave-functions Φ$A$ and Φ$B$ in (487) is thus justified. This makes the calculation of $M$ in principle simple. It is only necessary to pick out from (488) the terms which contain the right combination of elementary emission and absorption operators to annihilate the particles in $A$ and to create those in $B$. We shall next describe a general method of systematically picking out those terms, which is due originally to Feynman. It was first explained in published form by G. C Ẇick, _Phys. Rev._ **80** (1950) 268. Feynman and Wick have applied the method only to chronologically ordered products such as appear in (488). However the method applies in the same way to all products whether chronological or not, and we shall describe it in full generality.

<a id="p96"><sup>[p. 96]</sup></a>

## <a id="s-reduction-of-an-operator-to-normal-form"></a>The Reduction of an Operator to Normal Form

Given any operator $\mathcal{O}$ which is a product of field operators, for example

<a id="eq-496"></a>

$$
\mathcal{O} = \overline{\psi} ( x_{1} ) A / ( x_{1} ) \psi ( x_{1} ) \overline{\psi} ( x_{2} ) A / ( x_{2} ) \psi ( x_{2} ) \tag{496}
$$

we want to pick out the matrix element of $\mathcal{O}$ for a transition between states $A$ and $B$ in which there is a known distribution of bare particles. For example $A$ may be a state with only one electron in state 1, and $B$ a state with one electron in state 2. Then we wish to pick out from (496) terms in which there appear the operators $b_{1}$ and $b_{2}^{*}$. In order to pick out all such terms systematically, we make a complete analysis of $\mathcal{O}$ into a sum of terms $\mathcal{O}_{n}$, each $\mathcal{O}_{n}$ being a sum of products of emission and absorption operators in which all emission operators stand to the left of all absorption operators.<sup><a id="fnref-69" href="#fn-69">69</a></sup>. Any operator in which the emission and absorption operators are arranged in this special way is called “Normal”<sup><a id="fnref-70" href="#fn-70">70</a></sup> <a id="refback-19-1"></a>[[19]](#ref-19). The $\mathcal{O}_{n}$ will be called the “Normal constituents” of $\mathcal{O}$. Once $\mathcal{O}$ has been analyzed in this way, then we find the matrix element simply by taking the coefficient of $b_{2}^{*} b_{1}$ in the expansion $\sum \mathcal{O}_{n}$. No other term in the expansion can give any contribution to the matrix element. In $\mathcal{O}$ itself there might appear a term such as

<a id="eq-497"></a>

$$
b_{2}^{*} b_{3} b_{3}^{*} b_{1} \tag{497}
$$

which would give a contribution to the matrix element, since the operator $b_{3}^{*}$ could create a particle in an intermediate state 3 which the operator $b_{3}$ would then <a id="ix-31">annihilate</a>. The expansion of $\mathcal{O}$ into normal constituents eliminates all terms such as (497) and replaces them by sums of normal products with numerical coefficients. Thus using the anticommutation rule for $b_{3}$ and $b_{3}^{*}$, (497) becomes replaced by

<a id="eq-498"></a>

$$
Ab_{2}^{*} b_{1} - b_{2}^{*} b_{3}^{*} b_{3} b_{1} \tag{498}
$$

where $A$ is a numerical coefficient. The second term in (498) gives no contribution to the matrix element.

It is clear that by using the commutation rules of the operators in this way, every $\mathcal{O}$ can be written as a sum of normal products, and that the analysis leads to a unique expansion of $\mathcal{O}$. But we do not need to go through the tedious algebra of using the commutation rules, instead we can write down the normal constituents $\mathcal{O}_{n}$ directly by following simple rules.

First, we define the notation $N ( \mathcal{Q} )$, where $\mathcal{Q}$ is any product of emission and absorption operators, to be the product obtained by simply rearranging the factors of $\mathcal{Q}$ in a normal order, irrespective of the commutation rules, with a factor $( -$1) if the rearrangement involves an odd permutation of the electron-positron operators. Similarly if $\mathcal{Q}$ is any sum of products, $N ( \mathcal{Q} )$ is defined by rearranging factors in each term of the sum in the same way. Thus we have for example (see (211))

<a id="eq-499"></a>

$$
N ( A_{\lambda} ( x ) A_{\mu} ( y ) ) = A_{\lambda}^{+} ( x ) A_{\mu}^{+} ( y ) + A_{\lambda}^{-} ( x ) A_{\mu}^{-} ( y ) + A_{\lambda}^{-} ( x ) A_{\mu}^{+} ( y ) + A_{\mu}^{+} ( y ) A_{\lambda}^{+} ( x ) \tag{499}
$$

where $A_{\mu}^{+} ( x )$ is the positive-frequency part of $A_{\mu} ( x )$, i.e. the part containing absorption operators. Observe that the order of factors in the first two products in (499) is immaterial, only the third and fourth products have their order fixed by the condition of being normal. Similarly (see (306) and (309))

<a id="eq-500"></a>

$$
N \left( \psi_{\alpha} ( x ) \overline{\psi}_{\beta} ( y ) \right) = \psi_{\alpha}^{+} ( x ) \overline{\psi}_{\beta}^{+} ( y ) + \psi_{\alpha}^{-} ( x ) \overline{\psi}_{\beta}^{-} ( y ) + \psi_{\alpha}^{-} ( x ) \overline{\psi}_{\beta}^{+} ( y ) - \psi_{\beta}^{-} ( y ) \psi_{\alpha}^{+} ( x ) \tag{500}
$$

With this notation, every product of _two_ field operators can immediately be written as a sum of normal constituents. Using the commutation rules (213) and the vacuum expectation values given by (219), (220),

<a id="eq-501"></a>

$$
A_{\lambda} ( x ) A_{\mu} ( y ) = \langle A_{\lambda} ( x ) A_{\mu} ( y ) \rangle_{o} + N ( A_{\lambda} ( x ) A_{\mu} ( y ) ) \tag{501}
$$

Similarly, using (310), (311), (324),

<a id="eq-502"></a>

$$
\psi_{\alpha} ( x ) \overline{\psi}_{\beta} ( y ) = \left\langle \psi_{\alpha} ( x ) \overline{\psi}_{\beta} ( y ) \right\rangle_{o} + N \left( \psi_{\alpha} ( x ) \overline{\psi}_{\beta} ( y ) \right) \tag{502}
$$

And in fact for _any_ two field operators $\mathcal{P} , \mathcal{Q}$ we have

<a id="eq-503"></a>

$$
\mathcal{P} \mathcal{Q} = \langle \mathcal{P} \mathcal{Q} \rangle_{o} + N ( \mathcal{P} \mathcal{Q} ) \tag{503}
$$

<a id="p97"><sup>[p. 97]</sup></a>

provided $\mathcal{P}$ and $\mathcal{Q}$ are both linear in emission and absorption operators. The proof of (503) has in effect been done by proving (501) and (502), because these include all of the possible products of two boson or two fermion operators, and (503) is trivial for the product of one boson and one fermion operator, because they commute. Equations (501) – (503) are operator identities, and hold whether or not the physical problem is directly concerned with the vacuum state of the fields. In fact we could if we wished define the “vacuum expectation values” as the functions appearing in (501) – (503) and so avoid speaking about the vacuum state at all.

Next we shall state the generalization of the rule (503) to any product $\mathcal{O}$ of field operators, for example the $\mathcal{O}$ given by (496). We define a “factor pairing” of $\mathcal{O}$ by picking out from $\mathcal{O}$ a certain even number of factors, either all or none or any intermediate number, and associating them together in pairs. For the product $\mathcal{P} \mathcal{Q}$ there are only two factor-pairings, either we choose the pair $\mathcal{P} \mathcal{Q}$ together or we choose no pairs at all. To each factor-pairing $n$ corresponds to a normal constituent $\mathcal{O}_{n}$ obtained as follows: For each pair of factors $\mathcal{P} \mathcal{Q}$ which is paired in $n , \mathcal{O}_{n}$ contains the numerical factor $\langle \mathcal{P} \mathcal{Q} \rangle_{o}$, the order of $\mathcal{P}$ and $\mathcal{Q}$ being maintained as it was in $\mathcal{O}$. The unpaired factors $\mathcal{R}_{1} \mathcal{R}_{2} \dots \mathcal{R}_{m}$ in $\mathcal{O}$ appear in $\mathcal{O}_{n}$ rearranged in normal form. Thus the complete form of $\mathcal{O}_{n}$ is

<a id="eq-504"></a>

$$
\mathcal{O}_{n} = \pm \langle \mathcal{P} \mathcal{Q} \rangle_{o} \langle \mathcal{P}^{\prime} \mathcal{Q}^{\prime} \rangle_{o} \dots N ( \mathcal{R}_{1} , \mathcal{R}_{2} ,\dots \mathcal{R}_{m} ) \tag{504}
$$

the sign in front being + or $-$ according to the even or odd character of the permutation of the electron-positron operators from the order in which they are written in $\mathcal{O}$ to the order in which they are written in (504). With this definition of (504) of $\mathcal{O} n$, we have the following theorem:

_Every operator product_ $\mathcal{O}$ _is identically equal to the sum of the_ $\mathcal{O} n$ _obtained from all its factor-pairings._

This theorem gives the decomposition of $\mathcal{O}$ into its normal constituents. Equations (501) – (503) are just special cases of it. Clearly non-zero $\mathcal{O}_{n}$ are only obtained when each pair of factors is either a $\overline{\psi}$ and a $\psi$ operator or two $A_{\mu}$ operators. We shall therefore suppose that the factor-pairings are always restricted in this way.

The proof of the theorem is very simple, by induction on $m$, the number of factors in $\mathcal{O}$. The theorem is true when $m$ = 1 or 2, so we need only prove it true for $m$ assuming it true for $m -$ 2. Let then $\mathcal{O}^{\prime}$ be a product of $( m -$ 2) factors. First we show that the theorem is true for

<a id="eq-505"></a>

$$
\mathcal{O} = ( \mathcal{P} \mathcal{Q} \pm \mathcal{Q} \mathcal{P} ) \mathcal{O}^{\prime} \tag{505}
$$

where $\mathcal{P}$ and $\mathcal{Q}$ are field operators and the plus sign appears only if $\mathcal{P}$ and $\mathcal{Q}$ are both electron-positron operators. In fact, the normal constituents of $\mathcal{P} \mathcal{Q} \mathcal{O}^{\prime}$ and of $( \pm \mathcal{Q} \mathcal{P} \mathcal{O}^{\prime} )$ will be identical, so long as $\mathcal{P}$ and $\mathcal{Q}$ are not paired together. Therefore the sum of the normal constituents of $\mathcal{O}$ reduces to

<a id="eq-506"></a>

$$
\sum \mathcal{O} n = \{ \langle \mathcal{P} \mathcal{Q} \rangle_{o} \pm \langle \mathcal{Q} \mathcal{P} \rangle_{o} \} \sum \mathcal{O}_{n}^{\prime} \tag{506}
$$

But $\sum \mathcal{O}_{n}^{\prime} = \mathcal{O}^{\prime}$, and

<a id="eq-507"></a>

$$
\langle \mathcal{P} \mathcal{Q} \rangle_{o} \pm \langle \mathcal{Q} \mathcal{P} \rangle_{o} = ( \mathcal{P} \mathcal{Q} \pm \mathcal{Q} \mathcal{P} ) \tag{507}
$$

this being a number and not an operator. Therefore (506) gives $\sum \mathcal{O} n$ = 0, and the theorem is proved for $\mathcal{O}$ given by (505). Next let $\mathcal{O}$ be any product of $m$ factors. Then by using the commutation relations we can write

<a id="eq-508"></a>

$$
\mathcal{O} = N ( \mathcal{O} ) + \Sigma \tag{508}
$$

where Σ is a sum of terms of the form (505). The theorem is true for each term (505) and so is true for Σ. The theorem is trivially true for $N ( \mathcal{O} )$, because $\langle \mathcal{P} \mathcal{Q} \rangle_{o}$ = 0 for every pair of factors $\mathcal{P} , \mathcal{Q}$ in the order in which they occur in $N ( \mathcal{O} )$, and so all normal constituents (504) of $N ( \mathcal{O} )$ are zero except the constituent $N ( \mathcal{O} )$ itself. Therefore the theorem holds for every $\mathcal{O}$ given by (508), and this completes the proof.

<a id="p98"><sup>[p. 98]</sup></a>

## <a id="s-feynman-graphs"></a>Feynman Graphs

We use a method of Feynman to enumerate the possible factor-pairings of $\mathcal{O}$. Each pairing is pictured in a diagram or graph $G . G$ consists of a certain number of vertices with lines joining them. The vertices represent simply the different field-points at which the factors of $\mathcal{O}$ operate. Thus for $\mathcal{O}$ given by (496) each $G$ has the two vertices $x_{1} , x_{2}$. The lines in $G$ are either dotted, representing photon operators, or undotted, representing electron-positron operators. The rules for drawing these lines are the following:

1) For each factor-pair $\overline{\psi} ( x ) \psi ( y )$, an undotted line is drawn in $G$ running from $x$ to $y$, the direction being marked by an arrow in the line.

2) For each unpaired factor $\overline{\psi} ( x )$, an undotted line is drawn running from $x$ out of the diagram, the other end of the line being free and not being a vertex of $G$.

3) For each unpaired factor $\psi ( y )$, an undotted line is drawn running into $y$, the other end of the line being free.

4) For each factor-pair $A_{\mu} ( x ) A_{\nu} ( y )$, a dotted line joins $x$ and $y$.

5) For unpaired factor $A_{\mu} ( x )$, a dotted line is drawn with one end of $x$ and the other end free.

6) Every undotted line has a definite direction marked by an arrow. A dotted line has no direction and no arrow.

In general, we must allow factor-pairings in which two operators at the same field-point are paired together. This will give a line in $G$ with both ends at the same point. However in the case of operators such as (496) or more generally (488), a pair of factors taken from the same point will always give rise to a factor

<a id="eq-509"></a>

$$
\langle j_{\mu} ( x ) \rangle_{o} = - iec \left\langle \psi ( x ) \gamma_{\mu} \psi ( x ) \right\rangle_{o} \tag{509}
$$

in the corresponding normal constituents (504). We saw in the discussion following (366) that the vacuum expectation value (509) is zero, the operators being IR operators. Therefore factor-pairings in which two factors at the same field-point are paired, in the analysis of quantum-electrodynamical operators such as (488), always give zero contributions. So we may add to our list of rules for the construction of $G$:

7) Lines joining a point to itself are forbidden.

The possible factor pairings of (496) are then represented by the following $G$’s:

![Figure 8 (p. 98)](images/fig-p104-1.png)

_Figure 8 (p. 98)_

<a id="p99"><sup>[p. 99]</sup></a>

![Figure 9 (p. 99)](images/fig-p105-1.png)

_Figure 9 (p. 99)_

Corresponding to these $G$’s there will be just 8 normal constituents of $\mathcal{O}$, which we call $\mathcal{O}_{1} \dots \mathcal{O}_{8}$. These normal constituents are, in their respective order:

$$
\mathcal{O}_{1} = \pm N \left\{ \psi ( x_{1} ) A / ( x_{1} ) \psi ( x_{1} ) \overline{\psi} ( x_{2} ) A / ( x_{2} ) \psi ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{2} = \pm \left\langle \psi ( x_{1} ) \psi ( x_{2} ) \right\rangle_{o} N \left\{ A / ( x_{1} ) \psi ( x_{1} ) \overline{\psi} ( x_{2} ) A / ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{3} = \pm \left\langle \psi ( x_{2} ) \psi ( x_{1} ) \right\rangle_{o} N \left\{ \psi ( x_{1} ) A / ( x_{1} ) A / ( x_{2} ) \psi ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{4} = \pm \left\langle A / ( x_{1} ) A / ( x_{2} ) \right\rangle_{o} N \left\{ \psi ( x_{1} ) \psi ( x_{1} ) \overline{\psi} ( x_{2} ) \psi ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{5} = \pm \left\langle \psi ( x_{1} ) \psi ( x_{2} ) \right\rangle_{o} \left\langle A / ( x_{1} ) A / ( x_{2} ) \right\rangle_{o} N \left\{ \psi ( x_{2} ) \psi ( x_{1} ) \right\}
$$

$$
\mathcal{O}_{6} = \pm \left\langle \psi ( x_{2} ) \psi ( x_{1} ) \right\rangle_{o} \left\langle A / ( x_{1} ) A / ( x_{2} ) \right\rangle_{o} N \left\{ \psi ( x_{1} ) \psi ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{7} = \pm \left\langle \psi ( x_{1} ) \psi ( x_{2} ) \right\rangle_{o} \left\langle \psi ( x_{2} ) \psi ( x_{1} ) \right\rangle_{o} N \left\{ A / ( x_{1} ) A / ( x_{2} ) \right\}
$$

$$
\mathcal{O}_{8} = \pm \left\langle \psi ( x_{1} ) \psi ( x_{2} ) \right\rangle_{o} \left\langle \psi ( x_{2} ) \psi ( x_{1} ) \right\rangle_{o} \left\langle A / ( x_{1} ) A / ( x_{2} ) \right\rangle_{o}
$$

This type of process to which $\mathcal{O}_{i}$ gives matrix elements can be seen at once by looking at the _external_ lines of $G_{i}$, i.e. the lines which have one end free. Thus, for Møller scattering only $G_{4}$ contributes. For Compton scattering only $G_{2}$ and $G_{3}$. And for a transition from a one-electron atom to a one-electron state, which requires an operator of the form $b_{2}^{*} b_{1}$, and $G_{5}$ and $G_{6}$ will contribute.

A $\psi$ operator not only annihilates electrons but also creates positrons. And a $\overline{\psi}$ not only creates electrons but annihilates positrons. Thus an undotted external line with the arrow pointing in represents _either_ an electron in the initial state _or_ a positron in the final state. And an undotted external line with the arrow pointing out represents either an outgoing electron or an incoming positron. Dotted external lines likewise represent a photon either in the initial or the final state, because $A_{\mu}$ can both annihilate and create a photon. Thus $G_{4}$ will give not only electron-electron scattering, but also electron-positron scattering. $G_{2}$ and $G_{3}$ give not only the Compton effect, but also two-quantum annihilation of a positron-electron pair, and also the inverse process of pair-creation by the collision of two photons.

We have introduced the Feynman graphs simply as a convenient pictorial way of visualizing the analysis of an operator into its normal constituents. The graphs are just diagrams drawn on the paper. But according to Feynman<sup><a id="fnref-71" href="#fn-71">71</a></sup> , “Space-time Approach to Quantum Electrodynamics”, _Phys. Rev._ **76** (1949) 769, the graphs are more than this. He regards the graphs as a picture of an actual process which is occurring physically in space-time. Thus $G_{2}$ represents an electron and a photon coming together and interacting at the space-time point $x_{1}$, where the photon is absorbed; then the electron propagates through space-time along the line $x_{1} x_{2}$, until at $x_{2}$ it emits a photon, and the electron and photon then travel out along the outgoing lines at $x_{2}$. According to Feynman an internal undotted line running from $x_{1}$ to $x_{2}$ represents an electron propagating from $x_{1}$ to $x_{2}$ if the time $x_{2}$ is later than the time $x_{1}$, and represents a positron propagating from $x_{2}$ to $x_{1}$ if the time $x_{2}$ is earlier. In this sense a positron can be regarded as precisely equivalent to an electron which happens to propagate itself backwards instead of forwards in time.

The space-time picture of Feynman is a perfectly consistent and reasonable one. It gives a correct account of everything that happens, including pair creation and annihilation and all the other phenomena <a id="p100"><sup>[p. 100]</sup></a> connected with positrons. It is in fact equivalent mathematically to the field-theory treatment we follow in this course.

The disadvantage of the Feynman theory is that it is constructed as a particle theory. The fact that there are many particles, indistinguishable from each other and obeying quantum statistics, has to be put into the theory as a special assumption. And the equations of motion of the particles become quite complicated when interactions between several particles, not to mention vacuum-polarization effects, are included. Thus the logical basis of the Feynman theory is much less simple than that of the field theory, where everything follows from general principles once the form of the Lagrangian is chosen.

In this course we follow the pedestrian route of logical development, starting from the general principles of quantizations applied to covariant field equations, and deriving from these principles first the existence of particles and later the results of the Feynman theory. Feynman by the use of imagination and intuition was able to build a correct theory and get the right answers to problems much quicker than we can. It is safer and better for us to use the Feynman space-time pictures not as the basis for our calculations but only as a help in visualizing the formulae which we derive rigorously from the field-theory. In this way we have the advantages of the Feynman theory, its concreteness and its simplification of calculations, without its logical disadvantages.

## <a id="s-feynman-rules-of-calculation"></a>Feynman Rules of Calculation

The Feynman rules of calculation arise when we analyze into normal constituents a _chronologically ordered_ operator such as (488). In this case the vacuum expectation values in (504) are always taken for pairs of operators which are already chronologically ordered. Hence the numerical factors in (504) are all either

<a id="eq-510"></a>

$$
\langle P ( A_{\lambda} ( x ) ,A_{\mu} ( y ) ) \rangle_{o} = \tfrac{1}{2} \hbar cD_{F} ( x - y ) \delta_{\lambda \mu} \tag{510}
$$

or

<a id="eq-511"></a>

$$
\epsilon ( x - y ) \left\langle P \left( \psi_{\alpha} ( x ) , \overline{\psi}_{\beta} ( y ) \right) \right\rangle_{o} = - \tfrac{1}{2} S_{F\alpha \beta} ( x - y ) \tag{511}
$$

using (427) and (442). The factor $\epsilon$ is put into (511) so that the $\pm$ sign still characterizes the permutation of electron-positron operators in going from the order _as written_ in (504) to the order as written in $\mathcal{O}$. For the same reason, we shall follow Wick and generally use for chronological products the notation

<a id="eq-512"></a>

$$
T ( \mathcal{R}_{1} \mathcal{R}_{2} \dots \mathcal{R}_{n} ) = \pm P ( \mathcal{R}_{1} \mathcal{R}_{2} \dots \mathcal{R}_{n} ) \tag{512}
$$

where the sign is plus or minus according to the even or odd character of the permutation of electron-positron operators involved in going from the written order to the chronological order in (512). Hence in particular we have

<a id="eq-513"></a>

$$
\begin{aligned} T ( A_{\lambda} ( x ) ,A_{\mu} ( y ) \} &= P \{ A_{\lambda} ( x ) ,A_{\mu} ( y ) ) \\ T \left( \psi_{\alpha} ( x ) , \overline{\psi}_{\beta} ( y ) \right) &= \epsilon ( x - y ) P \left( \psi_{\alpha} ( x ) , \overline{\psi}_{\beta} ( y ) \right) \end{aligned} \tag{513}
$$

and for every set of field operators $\mathcal{R}_{1} \mathcal{R}_{2} \dots \mathcal{R}_{n}$ the quantity (512) is a relativistic invariant although the $P$-product by itself is not. In (488) itself the $P$-product may be written as a $T$-product, the sign in (512) in this case always being plus.

The rules for writing down the normal constituents of (488) are therefore extremely simple. We are generally only interested in those normal constituents which give matrix elements for some specified type of scattering process. Then the rules are

1) Draw all the graphs which have the right set of external lines corresponding to the particles absorbed and emitted in the process considered. Each <a id="ix-70">graph</a> $G$ will have the same external lines, but the number of vertices and of internal lines will vary from graph to graph. We shall always calculate only up to some definite order $N$ in the series (488), and so we draw only graphs with not more than $N$ vertices. The total number of such graphs is finite. Each vertex in each graph must have precisely 3 lines ending at it, one <a id="p101"><sup>[p. 101]</sup></a> incoming electron line, one outgoing electron line, and one photon line.

2) To each <a id="ix-71">graph</a> $G$ with $n$ vertices corresponds one normal constituent $S_{G}$ of $S$.

3) Choosing a particular $G$, write down the $n$<sup>th</sup> term $S_{n}$ of the series (488) and <a id="ix-18">pair</a> off the factors of $S_{n}$ as indicated by $G$. Replace each factor-pair $A_{\lambda} ( x ) A_{\mu} ( y )$ by (510), and replace each factor-pair $\psi_{\alpha} ( x ) \overline{\psi}_{\beta} ( y )$ by (511). Apply an $N$-ordering to the remaining unpaired factors of $S_{n}$, and multiply the whole expression by $( \pm$1) following the rule given for equation (504). The result of applying these operations to $S_{n}$ is the normal constituent $S_{G}$.

If we wish to calculate the matrix element for the <a id="ix-63">scattering</a> process, then we have only to add one more rule to the three already given.

4) In each $S_{G}$, substitute for the unpaired operators the wave-functions of the absorbed and emitted particles, for example writing (437) for $\psi ( x )$ when an electron ($p,u )$ is absorbed, and writing (438) for $A_{\mu} ( x )$ when a photon ($k,e )$ is absorbed. These substitutions may sometimes be made in more than one way (for example in the Compton <a id="ix-60">effect</a> when the absorbed and emitted photon may be assigned in two ways to the two unpaired photon operators.) In such cases the substitutions are to be made in all possible ways, and the results added together, taking account of Fermi statistics by putting in a minus sign when two electron or positron wave-functions are interchanged.

The rules (1)–(4) constitute the Feynman rules for calculating the matrix elements of all processes in electrodynamics. According to Feynman they have an intermediate concrete interpretation. Thus (510) is the probability amplitude for a photon emitted at $x$ with polarization $\lambda$ to propagate itself and arrive at $y$ with polarization $\mu$, plus the amplitude for a photon having been emitted at $y$ to arrive at $x$. And (511) is the amplitude for an electron emitted at $y$ to arrive at $x$, plus the amplitude for a positron emitted at $x$ to arrive at $y$, with the assigned spins $\alpha$ and $\beta$. In this way the matrix element is just the probability amplitude for the succession of events, interactions and propagations, that are depicted in the vertices and lines of $G$. The total probability amplitude for a process is just the sum of the amplitudes derived by from the various graphs $G$ which contribute to the process.

The Feynman rules of calculation take their most practical form when we use the momentum representations (430) and (448) for the $D_{F}$ and $S_{F}$ functions, carry out the integrations over the points $x_{1} \dots x_{n}$, and so obtain the matrix elements as integrals of rational functions in momentum-space. In this way for example the simple matrix elements (432) and (449) were obtained.

In the momentum space integral for $S_{G}$, there will appear

<a id="eq-514"></a>

$$
( 1 ) \quad A \text{ factor } \frac{1}{k^{2}} \text{ corresponding to each internal photon line of } G , \tag{514}
$$

<a id="eq-515"></a>

$$
( 2 ) \quad A \text{ factor } \frac{1}{/ k - i\mu} \text{ corresponding to each internal electron line of } G , \tag{515}
$$

<a id="eq-516"></a>

$$
( 3 ) \quad A \text{ factor } ( 2 \pi )^{4} \delta^{4} ( k_{1} + k_{2} + k_{3} ) \tag{516}
$$

corresponding to each internal photon line of $G$ at which the 3 lines associated with momenta $( k_{1} ,k_{2} ,k_{3} )$ meet. This factor arises from the integration over the space-time position of the vertex.

In addition to these factors there will be numerical factors and Dirac matrices $\gamma_{\alpha}$ arising from the particular form of $S_{n}$. In practice it is easiest not to write down the $S_{G}$ directly in momentum space, but to use the rules (1) – (4) to obtain formulae in configuration space with the right numerical constants, and then transform to momentum space by (430) and (448).

We shall now show how these general methods work by calculating in detail _the_ historic problem, the second-order radiative correction to the scattering of an electron by a weak external potential. This <a id="p102"><sup>[p. 102]</sup></a> problem has been first satisfactorily treated by Schwinger,<sup><a id="fnref-72" href="#fn-72">72</a></sup> _Phys. Rev._ **76** (1949) 790. Schwinger’s paper is outstandingly difficult to read, and I hope you will find my treatment at least slightly easier. But the problem is in its nature complicated and cannot be done without some fairly heavy mathematics. Once the calculations are done for this problem of scattering, it turns out that the results can be used without much further trouble for the relativistic calculation of the Lamb shift too. The scattering and Lamb shift problems are very closely related: in both cases one is calculating the second-order radiative corrections to the motion of an electron, only in one case the electron is in a high continuum state so that the external field can be treated as weak, in the other case the electron is in a discrete state and the potential must be treated as strong.

## <a id="s-the-self-energy-of-the-electron"></a>The Self-Energy of the Electron

Before we can study the effect of radiation interaction on an electron scattered by an external potential, we must first consider the effect of the radiation interaction on a single free electron in the absence of external potentials. Let the free electron be given initially in the state $( pu )$. The effect of the radiation interaction acting alone is given by the scattering matrix (488). If the initial state is Φ$A$, then the final state, reached after the radiation interaction has been acting for an infinitely long time, will be $S$Φ$A$. Now $S$ has matrix elements only for transitions which conserve momentum and energy. Starting from a one-electron state, it is impossible to make a transition to a many-particle state, for example by emitting one or more photons, while conserving momentum and energy. Therefore the only non-zero matrix elements of $S$ from the state Φ$A$ will be given by (487), where Φ$B$ is also a one-electron state. In Φ$B$ let the electron have the momentum and spin $( p^{\prime} u^{\prime} )$.

We consider radiative effects only up to the second order. The term of order 1 in (488) gives transitions only with emission an absorption of photons, and hence gives no contribution to the transition Φ${}_{A} \to$ Φ$B$. Therefore we may write simply

<a id="eq-517"></a>

$$
S = 1 + U_{2} \tag{517}
$$

with $U_{2}$ given by (423). We have to calculate the matrix element $M_{2}$ of $U_{2}$ between the states $( pu )$ and

$$
( p^{\prime} u^{\prime} ) .
$$

To write down $M_{3}$ we use the Feynman <a id="ix-74">rules</a>. The factor-pairings of $U_{2}$ are represented in the 8 graphs on pp 98-99. Of these only $G_{5}$ and $G_{6}$ contribute to $M_{2}$, and they contribute equally since the integral (423) is symmetrical in the variables $x_{1}$ and $x_{2}$. The normal constituent of $U_{2}$ arising from $G_{5}$ and $G_{6}$ is, using (510) and (511)<sup><a id="fnref-73" href="#fn-73">73</a></sup>

<a id="eq-518"></a>

$$
\begin{aligned} U_{2 N} &= \sum_{\lambda ,\mu} \frac{e^{2}}{\hbar^{2} c^{2}} \int \int dx_{1} dx_{2} N \left( \overline{\psi} ( x_{1} ) \gamma_{\lambda} \left\langle T \left( \psi ( x_{1} ) , \overline{\psi} ( x_{2} ) \right) \right\rangle_{o} \gamma_{\mu} \psi ( x_{2} ) \right) \langle T ( A_{\lambda} ( x_{1} ) ,A_{\mu} ( x_{2} ) ) \rangle_{o} \\ &= - \frac{e^{2}}{4 \hbar c} \sum_{\lambda} \int \int dx_{1} dx_{2} N \left( \overline{\psi} ( x_{1} ) \gamma_{\lambda} S_{F} ( x_{1} - x_{2} ) \gamma_{\lambda} \psi ( x_{2} ) \right) D_{F} ( x_{2} - x_{1} ) \end{aligned} \tag{518}
$$

To obtain $M_{2}$ from (518) we substitute for $\psi ( x_{2} )$ and $\overline{\psi} ( x_{1} )$ the wave-functions of the initial and final states, and use the momentum integrals (430), (448). Then the integration over $x_{1}$ and $x_{2}$ can be carried out and we find

<a id="eq-519"></a>

$$
\begin{aligned} M_{2} &= \sum_{\lambda} \frac{ie^{2}}{\hbar c} \int_{F} \int_{F} dk_{1} dk_{2} \left( \overline{u}^{\prime} \gamma_{\lambda} \frac{1}{/ k_{1} - i\mu} \gamma_{\lambda} u \right) \frac{1}{k_{2}^{2}} \delta ( k_{1} - k_{2} - p^{\prime} ) \delta ( k_{2} - k_{1} + p ) \\ &= \sum_{\lambda} \frac{ie^{2}}{\hbar c} \delta ( p - p^{\prime} ) \int_{F} dk \left( \overline{u}^{\prime} \gamma_{\lambda} \frac{1}{/ k + p / - i\mu} \gamma_{\lambda} u \right) \frac{1}{k^{2}} \end{aligned} \tag{519}
$$

We consider the Dirac operator

<a id="eq-520"></a>

$$
\Sigma ( p ) = \sum_{\lambda} \int_{F} dk \left( \gamma_{\lambda} \frac{1}{/ k + p / - i\mu} \gamma_{\lambda} \right) \frac{1}{k^{2}} \tag{520}
$$

<a id="p103"><sup>[p. 103]</sup></a>

appearing in (519). Since $( p,u )$ are the momentum and spin of a real electron, we may use the relations

<a id="eq-521"></a>

$$
p^{2} + \mu^{2} = 0 , \quad ( p / - i\mu ) u = 0 \tag{521}
$$

when we evaluate Σ$( p )$ in (519). So using (376), (585) and following the same method that was used in evaluating (377)<sup><a id="fnref-74" href="#fn-74">74</a></sup>

<a id="eq-522"></a>

$$
\begin{aligned} \Sigma ( p ) &= \int_{F} dk \sum \frac{\gamma_{\lambda} ( / k + p / + i\mu ) \gamma_{\lambda}}{k^{2} ( k^{2} + 2 p \cdot k )} = \int_{F} dk \frac{4 i\mu - 2 k / - 2 p /}{k^{2} ( k^{2} + 2 p \cdot k )} \\ &= 2 \int_{0}^{1} dz \int_{F}^{\lambda} dK \frac{i\mu - / k}{[ k^{2} + 2 zp \cdot k ]^{2}} = 2 \int_{0}^{1} \int_{F} dk \frac{( i\mu - / k + z/p )}{[ k^{2} - z^{2} p^{2} ]^{2}} \\ &= 2 \int_{0}^{1} dz \int_{F} dk \frac{i\mu ( 1 + z )}{[ k^{2} + z^{2} \mu^{2} ]^{2}} \end{aligned} \tag{522}
$$

where we changed the origin of the $k$ <a id="ix-127">integration</a> by the replacement $k \to k - zp$ and we eliminated the odd terms. Using (386) and introducing the logarithmic divergence $R$ again according to (387),

<a id="eq-523"></a>

$$
\Sigma ( p ) = 2 \int_{0}^{1} dzi\mu ( 1 + z ) \{ 2 i\pi^{2} ( R - \log z ) \} = - \pi^{2} \mu [ 6 R + 5 ] = - 6 \pi^{2} \mu R^{\prime} \tag{523}
$$

Thus Σ$( p )$ is a logarithmically divergent constant, depending only on the electron mass and independent of the state of the electron. The difference 5$/$6 between $R$ and $R^{\prime}$ is of course not significant. Substituting (523) into (519) gives the value of $M_{2}$

<a id="eq-524"></a>

$$
M_{2} = - 6 \pi^{2} i \frac{e^{2} \mu}{\hbar c} R^{\prime} \delta ( p - p^{\prime} ) ( \overline{u}^{\prime} u ) \tag{524}
$$

Thus $U_{2}$ does not give any transitions between different one-electron states. It has only the diagonal matrix elements given by (524) .

Now (524) has precisely the correct relativistic form to be identified with a pure self-energy <a id="ix-61">effect</a>. Suppose that in consequence of the radiation interaction the mass of a real electron is

<a id="eq-525"></a>

$$
m = m_{o} + \delta m \tag{525}
$$

where $m_{o}$ is the mass of the bare electron without interaction and $\delta m$ is the electromagnetic contribution to the mass. The mass-change $\delta m$ would be represented by a term

<a id="eq-526"></a>

$$
L_{S} = - \delta mc^{2} \overline{\psi} \psi \tag{526}
$$

in the Lagrangian density (410). This would produce an interaction energy

<a id="eq-527"></a>

$$
H_{S} ( t ) = \delta mc^{2} \int \overline{\psi} ( r,t ) \psi ( r,t ) d^{3} \boldsymbol{r} \tag{527}
$$

in the Schrödinger equation (415), and finally a contribution

<a id="eq-528"></a>

$$
U_{S} = - i \frac{\delta mc}{\hbar} \int \overline{\psi} \psi ( x ) dx \tag{528}
$$

in the scattering matrix (421) or (488).

The matrix element of (528) between the states $( pu )$ and ($p^{\prime} u^{\prime} )$ is

<a id="eq-529"></a>

$$
M_{S} = - i \frac{\delta mc}{\hbar} ( 2 \pi )^{4} \delta ( p - p^{\prime} ) ( \overline{u}^{\prime} u ) \tag{529}
$$

<a id="p104"><sup>[p. 104]</sup></a>

This is identical with (524) if we identify the self-mass $\delta m$ by the equation

<a id="eq-530"></a>

$$
\delta m = \frac{3}{8 \pi^{2}} \frac{e^{2} m}{\hbar c} R^{\prime} = \frac{3 \alpha}{2 \pi} R^{\prime} m \tag{530}
$$

For all one-electron matrix elements, $U_{2}$ is identical with $U_{S}$. That is to say, the whole effect of the radiation interaction upon a free electron is to change its mass by the amount (530). This is a most satisfactory conclusion. It means that an electron with its self-field still has the correct relationship between momentum and energy for a relativistic particle, only the value of the rest-mass being changed by the self-field. It was always one of the central difficulties of the classical electron theory, that a classical extended electron did not have the right relativistic behavior.

The self-mass $\delta m$ is an unobservable quantity. The observed mass of an electron is $m$, and neither $m_{o}$ nor $\delta m$ can be measured separately. Thus it is unsatisfactory that $\delta m$ appears in the scattering matrix $S$ which is supposed to represent the results of experiments.

The reason why $\delta m$ still appears explicitly is just that we have _not_ used the observed mass $m$ in defining the initial and final states of the system. We defined these states as states of a free electron with the bare mass $m_{o}$. Wherever we used the letter $m$ for the electron mass in the theory up to this point, in fact it was an inconsistency of notation and we meant by $m$ the mass of a _bare_ electron.

It is much better not to change the notation, but to keep the notation and change the interpretation, so that $m$ everywhere in the theory is now intended to mean the mass of a real electron. In particular, we set up the interaction representation operators with the real electron mass $m$, and the initial and final states of scattering problems are defined as free particles with the correct mass $m$. With this changed interpretation, the whole of the theory up to this point is correct, except that in $L_{D}$ which appears in the Lagrangian (410) of quantum electrodynamics, and in the field equations (411), (412) satisfied by the Heisenberg operators, the bare mass $m_{o}$ must be used instead of $m$. We prefer to keep the observed mass $m$ in $L_{D}$, and correct for it by writing instead of (410)<sup><a id="fnref-75" href="#fn-75">75</a></sup>

<a id="eq-531"></a>

$$
L = L_{D} + L_{M} - ie \overline{\psi} / A\psi - ie \overline{\psi} / A^{e} \psi - L_{S} \tag{531}
$$

with $L_{S}$ given by (526). The radiation interaction becomes then

<a id="eq-532"></a>

$$
H_{R} ( t ) - H_{S} ( t ) = H^{I} ( t ) \tag{532}
$$

with $H_{R}$ given by (416) and $H_{S}$ by (527). After making the changes (537) and (532), the whole theory becomes consistent with the interpretation that $m$ is everywhere the observed electron mass.

In particular, one result of (532) is that for one-electron states the scattering operator $S$ becomes

<a id="eq-533"></a>

$$
S = 1 + U_{2} - U_{S} \tag{533}
$$

instead of (517), keeping only terms of order $e^{2}$. The matrix elements of $( U_{2} - U_{S} )$ for one-electron states are all zero. Thus, if we use the correct mass $m$ in defining the states of an electron, _there are no longer_ _any observable effects of the radiation interaction on the motion of a free electron._ This shows that the mass-renormalization, the procedure of inserting the term $( - L_{S} )$ in (531), is consistent and is likely to give sensible results.

## <a id="s-second-order-radiative-corrections-to-scattering"></a>Second-Order Radiative Corrections to Scattering

Let an electron be scattered from the initial state $( pu )$ to the final state ($p^{\prime} u^{\prime} )$ by the external potential

<a id="eq-534"></a>

$$
A_{\mu}^{e} ( x ) = \frac{1}{( 2 \pi )^{4}} \int e^{iq \cdot x} e_{\mu} ( q ) dq \tag{534}
$$

At the same time the electron is interacting with the quantized Maxwell field with the interaction (532), since we suppose the initial and final states to be defined with the observed mass of a free electron. The <a id="p105"><sup>[p. 105]</sup></a> scattering matrix element $M$ is then given by (419), with $U$ given by (421) after replacing each $H_{R}$ by $H^{I}$ according to (532).

We treat $A_{\mu}^{e}$ in the linear Born approximation. Thus we keep only terms of order 0 and 1 in $A_{\mu}^{e}$. The terms of order 0 give the effects of the radiation interaction alone; as we have seen, these effects are zero for an initial state consisting of a single <a id="ix-209">electron</a>.

The scattering matrix is thus given effectively by the terms of order 1 in $A_{\mu}^{e}$ taken from (421), namely

<a id="eq-535"></a>

$$
U = \sum_{n = 0}^{\infty} \left( \frac{- i}{\hbar} \right)^{n} \frac{1}{n !} \int \dots \int dtdt_{1} \dots dt_{n} P \left\{ H^{e} ( t ) ,H^{I} ( t_{1} ) ,\dots ,H^{I} ( t_{n} ) \right\} \tag{535}
$$

We shall calculate radiative effects only up to the second order in the radiation interaction. Since $\delta m$ is itself of second order, this means that we go to second order in $H_{R}$ and to first order in $H_{S}$. Thus

<a id="eq-536"></a>

$$
U = U_{0} + \int U_{1} + U_{2} + U_{2}^{\prime} \tag{536}
$$

<a id="eq-537"></a>

$$
U_{0} = \frac{e}{\hbar c} \quad dx \overline{\psi} / A^{e} \psi ( x ) \tag{537}
$$

<a id="eq-538"></a>

$$
U_{1} = \frac{e^{2}}{\hbar^{2} c^{2}} \int \int dxdx_{1} P \left\{ \psi / A^{e} \psi ( x ) , \overline{\psi} / A\psi ( x_{1} ) \right\} \tag{538}
$$

<a id="eq-539"></a>

$$
U_{2} = \frac{e^{3}}{2 \hbar^{3} c^{3}} \int \int \int dxdx_{1} dx_{2} P \left\{ \psi / A^{e} \psi ( x ) , \overline{\psi} / A\psi ( x_{1} ) , \overline{\psi} / A\psi ( x_{2} ) \right\} \tag{539}
$$

<a id="eq-540"></a>

$$
U_{2}^{\prime} = \frac{ie\delta m}{\hbar^{2}} \int \int dxdx_{1} P \left\{ \psi / A^{e} \psi ( x ) , \overline{\psi} \psi ( x_{1} ) \right\} \tag{540}
$$

The matrix element we wish to calculate is then correspondingly

<a id="eq-541"></a>

$$
M = M_{0} + M_{1} + M_{2} + M_{2}^{\prime} \tag{541}
$$

The wave-functions of initial and final states are

<a id="eq-542"></a>

$$
ue^{ip \cdot x} \quad u^{\prime} e^{ip^{\prime} \cdot x} \tag{542}
$$

Then by (534) we have

<a id="eq-543"></a>

$$
M_{0} = \frac{e}{\hbar c} ( u^{\prime} /eu ) \tag{543}
$$

where $q$ is the constant vector

<a id="eq-544"></a>

$$
q = p^{\prime} - p \tag{544}
$$

and

<a id="eq-545"></a>

$$
e_{\mu} = e_{\mu} ( q ) \tag{545}
$$

The operator $U_{1}$ gives transitions from a one-electron state only to states consisting of an electron and a photon. This is just the <a id="ix-58">bremsstrahlung</a> process, scattering of the electron with real photon emission, and the matrix element for it is given by (484). In any scattering experiment, this process will of course go on at the same time as the scattering without radiation. Experimentally, the scattering with photon emission will only be separable from the radiationless scattering if the emitted photon has an energy greater than some limit ∆$E$, roughly equal to the energy resolution with which the energy of the electron can be measured. The scattering with emission of soft quanta (low frequency, $k^{\prime}$ small) will always be included in the radiationless scattering cross-section. Therefore we shall be interested in the value of $M_{1}$ for a final state consisting of the electron $( p^{\prime} u^{\prime} )$ and a photon with potentials (440), in the case where $k^{\prime}$ is so small as to be negligible in comparison with $p , p^{\prime}$ and $q$. In this case (484) gives

<a id="eq-546"></a>

$$
M_{1} = \frac{e^{2}}{\hbar^{2} c^{2}} \left[ \frac{p \cdot e^{\prime}}{p \cdot k^{\prime}} - \frac{p^{\prime} \cdot e^{\prime}}{p^{\prime} \cdot k^{\prime}} \right] ( \overline{u}^{\prime} /eu ) = \frac{e}{\hbar c} \left[ \frac{p \cdot e^{\prime}}{p \cdot k^{\prime}} - \frac{p^{\prime} \cdot e^{\prime}}{p^{\prime} \cdot k^{\prime}} \right] M_{0} \tag{546}
$$

<a id="p106"><sup>[p. 106]</sup></a>

where we used (521) and rule (4) on page 87.

We now come to the calculation of $M_{2}$, the matrix element of (539) between the states (542). This is the main part of the problem. To do it we use the Feynman rules. There are just 9 graphs giving contributions to $M_{2}$, namely

![Figure 10 (p. 106)](images/fig-p112-1.png)

_Figure 10 (p. 106)_

and $\{ G_{6} ,G_{7} ,G_{8} ,G_{9} \}$ obtained by interchanging the labels $( x_{1} ,x_{2} )$ in $\{ G_{1} ,G_{2} ,G_{3} ,G_{4} \}$, respectively. We can see this in the following way: The process we are interested in calls for one external photon line and two external electron lines. Hence $A /^{e} ( x )$, one $\overline{\psi}$ and one $\psi$ must be unpaired. Therefore the two $A /$’s are always coupled. The free electron lines can be $\psi_{0} ,\psi_{1} ; \overline{\psi}_{1} ,\psi_{0} ; \overline{\psi}_{1} ,\psi_{1} ; \overline{\psi}_{1} ,\psi_{2}$ and four more cases obtained by the substitution 1 $\leftrightarrow$ 2. In each case, the rest is uniquely determined by the rules. The ninth case is the one with external electron lines $\overline{\psi}_{0} ,\psi_{0}$.

The effect of $\{ G_{6} ,G_{7} ,G_{8} ,G_{9} \}$ is only to double the contribution from $\{ G_{1} ,G_{2} ,G_{3} ,G_{4} \}$, since (539) is symmetrical in the variables $x_{1}$ and $x_{2}$. Also $G_{5}$ will give only a numerical phase factor multiplying $M_{0}$, the phase factor being the same for all final states; it is in fact precisely the phase-shift between the initial and final states due to the self-energy of the vacuum. Similar phase shift factors would also multiply the contributions of $G_{1} , G_{2}$, etc., coming from disconnected graphs in higher order contributions. In this sense therefore, we can consider $G_{5}$ to be really the contribution of $U_{0}$ with one of the many possible disconnected additions. A numerical phase factor of this kind, the same for all final states, is entirely unobservable and without physical meaning, since it can be cancelled by changing the phase of all wave-functions by the same amount. Hence we can always ignore graphs such as $G_{5}$ having a disconnected part without external lines. There remain to be considered only $\{ G_{1} ,G_{2} ,G_{3} ,G_{4} \}$.

Using the Feynman rules, the contribution of $G_{1}$ to $M_{2}$ is (with the factor 2 from $G_{6} )$

<a id="eq-547"></a>

$$
\begin{aligned} M_{2 1} &= - \frac{e^{3}}{\hbar^{3} c^{3}} \int \int \int dxdx_{1} dx_{2} \sum_{\mu} \operatorname{Spur} \left\{ A /^{e} ( x ) \left\langle T \{ \psi ( x ) , \overline{\psi} ( x_{1} ) \} \right\rangle_{o} \gamma_{\mu} \left\langle T \{ \psi ( x_{1} ) , \overline{\psi} ( x ) \} \right\rangle_{o} \right\} \times \\ & \quad \times \overline{\psi} ( x_{2} ) \left\langle T \{ A_{\mu} ( x_{1} ) ,/ A ( x_{2} ) \} \right\rangle_{o} \psi ( x_{2} ) \end{aligned} \tag{547}
$$

where the spur appears because of the contraction according to rule 3, and the minus sign comes from the change in order of $\overline{\psi}$ and $\psi$ factors between (539) and (547). By (510) and (511)

$$
M_{2 1} = - \frac{e^{3}}{8 \hbar^{2} c^{2}} \int \int \int dxdx_{1} dx_{2} \sum_{\mu} \operatorname{Spur} \left\{ A /^{e} ( x ) S_{F} ( x - x_{1} ) \gamma_{\mu} S_{F} ( x_{1} - x ) \right\} D_{F} ( x_{1} - x_{2} ) \overline{\psi} ( x_{2} ) \gamma_{\mu} \psi ( x_{2} )
$$

Hence, using the momentum integrals (430), (448), (534), (542) and carrying out the integration over

<a id="p107"><sup>[p. 107]</sup></a>

<a id="eq-548"></a>

$$
\begin{aligned} & \quad ( x,x_{1} ,x_{2} ) , \\ M_{2 1} &= \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \int \int \int \int dk_{1} dk_{2} dk_{3} dq \sum_{\mu} \operatorname{Spur} \left\{ /e ( q ) \frac{1}{/ k_{1} - i\mu} \gamma_{\mu} \frac{1}{/ k_{2} - i\mu} \right\} \frac{1}{k_{3}^{2}} ( \overline{u}^{\prime} \gamma_{\mu} u ) \times \\ & \quad \times \delta ( q + k_{1} - k_{2} ) \delta ( - k_{1} + k_{2} + k_{3} ) \delta ( - k_{3} - p^{\prime} + p ) \\ &= \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \int_{F} dk \sum_{\mu} \operatorname{Spur} \left\{ /e \frac{1}{/ k - i\mu} \gamma_{\mu} \frac{1}{/ k + /q - i\mu} \right\} \frac{1}{q^{2}} ( \overline{u}^{\prime} \gamma_{\mu} u ) \\ &= \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \sum_{\mu} \frac{1}{q^{2}} ( \overline{u}^{\prime} \gamma_{\mu} u ) J_{\mu} \end{aligned} \tag{548}
$$

with $q$ given by (544) and<sup><a id="fnref-76" href="#fn-76">76</a></sup>

<a id="eq-549"></a>

$$
J_{\mu} = \int_{F} F_{\mu} ( k ) dk \tag{549}
$$

the function $F_{\mu} ( k )$ being identical with (371) for $\delta$ = 0. Note that (549) is a Feynman integral, which is precisely the same thing as the contour integral (374) with the contour drawn in the diagram. The effect of the $\epsilon$ in (431) is just equivalent to the contour $C$. Hence<sup><a id="fnref-77" href="#fn-77">77</a></sup> using (388) for $J_{\mu}$,

<a id="eq-550"></a>

$$
M_{2 1} = - \frac{e^{3}}{2 \pi^{2} \hbar^{2} c^{2}} ( \overline{u}^{\prime} /eu ) \left\{ \tfrac{1}{3} R - \int_{0}^{1} ( z - z^{2} ) \log \left( 1 + ( z - z^{2} ) \frac{q^{2}}{\mu^{2}} \right) dz \right\} \tag{550}
$$

where we have also dropped the term $q_{\mu}$ in(388), since

<a id="eq-551"></a>

$$
( \overline{u}^{\prime} /qu ) = \{ u^{\prime} ( p /^{\prime} - i\mu ) u \}-\{ u^{\prime} ( p / - i\mu ) u \} = 0 \tag{551}
$$

Writing $\alpha = \frac{e^{2}}{4 \pi \hbar c}$, (550) becomes

<a id="eq-552"></a>

$$
M_{2 1} = \alpha M_{0} \left\{ - \frac{2}{3 \pi} R + \frac{2}{\pi} \int_{0}^{1} ( z - z^{2} ) \log \left( 1 + ( z - z^{2} ) \frac{q^{2}}{\mu^{2}} \right) dz \right\} \tag{552}
$$

This is just the scattering which is produced by the charge-current density induced in the vacuum by the potential $A_{\mu}^{e}$ according to (392). As before, the term in $R$ is unobservable since it can never be separated experimentally from the simple scattering $M_{0}$ to which it is proportional. The observed external potential, measured in any way whatever, will not be $A_{\mu}^{e}$ but $A_{\mu}^{e} ( 1 - \tfrac{2 \alpha}{3 \pi} R )$, which we may call the “renormalized external potential”. Hence in terms of the observed $A_{\mu}^{e}$ the total contribution from $G_{1}$ will be

<a id="eq-553"></a>

$$
M_{2 1} = \frac{2 \alpha}{\pi} M_{0} \int_{0}^{1} ( z - z^{2} ) \log \left( 1 + ( z - z^{2} ) \frac{q^{2}}{\mu^{2}} \right) dz \tag{553}
$$

The integral will be in general complex as before. But for small $q$ it will be real, and neglecting terms of order higher than $q^{2}$, we have

<a id="eq-554"></a>

$$
M_{2 1} = \frac{2 \alpha}{\pi} \frac{q^{2}}{\mu^{2}} M_{0} \int_{0}^{1} ( z - z^{2} )^{2} dz = \frac{\alpha}{1 5 \pi} M_{0} \frac{q^{2}}{\mu^{2}} \tag{554}
$$

<a id="p108"><sup>[p. 108]</sup></a>

Next we consider the contribution to $M_{2}$ from $G_{2}$. This is

<a id="eq-555"></a>

$$
M_{2 2} = \frac{e^{3}}{8 \hbar^{2} c^{2}} \int \int \int dxdx_{1} dx_{2} \sum_{\lambda} \psi ( x ) A /^{e} ( x ) S_{F} ( x - x_{2} ) \gamma_{\lambda} S_{F} ( x_{2} - x_{1} ) \gamma_{\lambda} \psi ( x_{1} ) D_{F} ( x_{1} - x_{2} ) \tag{555}
$$

$$
= - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \int \int \int \int dk_{1} dk_{2} dk_{3} dq \sum_{\lambda} \left\{ \overline{u}^{\prime} /e ( q ) \frac{1}{k /_{1} - i\mu} \gamma_{\lambda} \frac{1}{/ k_{2} - i\mu} \gamma_{\lambda} u \right\} \frac{1}{k_{3}^{2}} \times
$$

<a id="eq-556"></a>

$$
\times \delta ( k_{1} + q - p^{\prime} ) \delta ( k_{2} - k_{1} - k_{3} ) \delta ( k_{3} + p - k_{2} ) \tag{556}
$$

<a id="eq-557"></a>

$$
= - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \sum_{\lambda} \int_{F} dk \left( \overline{u}^{\prime} /e \frac{1}{p / - i\mu} \gamma_{\lambda} \frac{1}{/ k + p / - i\mu} \gamma_{\lambda} u \right) \frac{1}{k^{2}} \tag{557}
$$

<a id="eq-558"></a>

$$
= - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \left( \overline{u}^{\prime} /e \frac{1}{p / - i\mu} \Sigma ( p ) u \right) \tag{558}
$$

with Σ$( p )$ given by(520).

Before discussing Σ$( p )$ we must look at the factor $\frac{1}{p / - i\mu}$ which appears in (558). This factor is

<a id="eq-559"></a>

$$
\frac{p / + i\mu}{p^{2} + \mu^{2}} \tag{559}
$$

But since $p$ is the momentum vector of a real electron, $p^{2} + \mu^{2}$ = 0 and the factor (559) is singular. This means that the integrals over $x_{1}$ and $x_{2}$ are really divergent and not merely finitely oscillating at $t = \pm \infty$, and the transformation into momentum integrals is not allowable. Eq. (558) as it stands is strictly meaningless.

This is the place where we have to take explicitly into our calculations the slowly-varying cut-off functions $g ( t_{i} )$ appearing in (495), which are put in for the purpose of defining unambiguously the initial and final states of the problem. So we write instead of (555)

$$
M_{2 2} = \frac{e^{3}}{8 \hbar^{2} c^{2}} \int \int \int dxdx_{1} dx_{2} \sum_{\lambda} \psi ( x ) A /^{e} ( x ) S_{F} ( x - x_{2} ) \gamma_{\lambda} S_{F} ( x_{2} - x_{1} ) \gamma_{\lambda} \psi ( x_{1} ) D_{F} ( x_{1} - x_{2} ) g ( t_{1} ) g ( t_{2} ) ( 5 6 0 )
$$

Here the $g ( t_{1} ) g ( t_{2} )$ factors are attached to the radiation interaction operating at $x_{1}$ and $x_{2}$. It is supposed that the time $T$ over which $g ( t )$ varies appreciably is long compared with the duration of the scattering process. Let the Fourier integral representation of $g ( t )$ be

<a id="eq-561"></a>

$$
\begin{aligned} g ( t ) &= \int_{-\infty}^{\infty} G ( \epsilon_{0} ) e^{- i\epsilon_{0} ct} d\epsilon_{0} \\ &= \int_{-\infty}^{\infty} G ( \epsilon_{0} ) e^{i\epsilon \cdot x} d\epsilon_{0} \end{aligned} \tag{561}
$$

where $\epsilon_{0}$ is a real variable and $\epsilon$ is the vector

<a id="eq-562"></a>

$$
\epsilon = ( 0 , 0 , 0 ,\epsilon_{0} ) \tag{562}
$$

We have the normalization $\infty$

<a id="eq-563"></a>

$$
g ( 0 ) = \int_{-\infty} G ( \epsilon_{0} ) d\epsilon_{0} = 1 \tag{563}
$$

and we suppose that $G ( \epsilon_{0} )$ is “almost” a $\delta$-function, that is to say a function which is large only for values of $\epsilon_{0}$ in a range of about $( cT )^{- 1}$ on either side of zero. Substituting (561) into (560), we obtain instead of (558) the corrected formula

<a id="eq-564"></a>

$$
M_{2 2} = - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \int \int G ( \epsilon_{0} ) G ( \epsilon_{0}^{\prime} ) d\epsilon_{0} d\epsilon_{0}^{\prime} \left\{ \overline{u} /e ( q - \epsilon - \epsilon^{\prime} ) \frac{1}{p / + /\epsilon + /\epsilon^{\prime} - i\mu} \Sigma ( p + \epsilon ) u \right\} \tag{564}
$$

<a id="p109"><sup>[p. 109]</sup></a>

In (564) the inadmissible factor (559) is replaced by something finite and mathematically well-defined. There will be a singularity in the integration of (564) over $\epsilon_{0}$, but this is an ordinary pole and the integration over $\epsilon_{0}$ will give a well-determined result when taken as a Feynman integral. We have as $T \to \infty$ and $\epsilon_{0}$ and

<a id="eq-565"></a>

$$
\begin{aligned} & \quad \epsilon_{0}^{\prime} \to 0_{\prime} \\ \frac{1}{p / + /\epsilon + /\epsilon^{\prime} - i\mu} &= \frac{p / + /\epsilon + /\epsilon + i\mu}{2 p \cdot ( \epsilon + \epsilon^{\prime} ) + ( \epsilon + \epsilon^{\prime} )^{2}} \sim - \frac{p / + i\mu}{2 p_{0} ( \epsilon_{0} + \epsilon_{0}^{\prime} )} \end{aligned} \tag{565}
$$

Hence when evaluating Σ$( p + \epsilon )$ we need retain only terms of order zero and one in $\epsilon_{0}$; the terms of order two and higher are negligible, because even when multiplied by (565) they tend to zero as $T \to \infty$.

Keeping only terms of order zero and one in $\epsilon$, Σ$( p + \epsilon )$ becomes

<a id="eq-566"></a>

$$
\Sigma ( p + \epsilon ) = \Sigma ( p ) - \sum_{\alpha} \epsilon_{\alpha} I_{\alpha} ( p ) \tag{566}
$$

<a id="eq-567"></a>

$$
I_{\alpha} ( p ) = \int_{F} dk \sum_{\lambda} \left( \gamma_{\lambda} \frac{1}{k / + p / - i\mu} \gamma_{\alpha} \frac{1}{/ k + p / - i\mu} \gamma_{\lambda} \right) \frac{1}{k^{2}} \tag{567}
$$

Here we have used the identity

<a id="eq-568"></a>

$$
\frac{1}{A + B} = \frac{1}{A} - \frac{1}{A} B \frac{1}{A} + \frac{1}{A} B \frac{1}{A} B \frac{1}{A} - \dots \tag{568}
$$

which is valid for any two operators $A$ and $B$, commuting or not, if the series on the RHS converges in some sense. This can be seen at once by multiplying through by $A + B$; then the condition becomes $( B/A )^{n} \to$ 0 in some sense.

In (564) we may use the conditions (521), and these give for Σ$( p )$ the constant value (523). The integral $I_{\alpha} ( p )$ is like Σ($p )$ logarithmically divergent for large $k$, and it is also logarithmically divergent for small $k$, which Σ$( p )$ is not. We shall not attempt to evaluate $I_{\alpha} ( p )$ mathematically. From general principles of covariance we can say what is its form as a function of $p$. For general $p$ not satisfying (521), $I_{\alpha} ( p )$ is a Dirac matrix transforming like a vector under Lorentz transformations, and therefore must be of the form

<a id="eq-569"></a>

$$
I_{\alpha} ( p ) = F_{1} ( p^{2} ) \gamma_{\alpha} + F_{2} ( p^{2} ) ( p / - i\mu ) \gamma_{\alpha} + F_{3} ( p^{2} ) \gamma_{\alpha} ( /p - i\mu ) + F_{4} ( p^{2} ) ( p / - i\mu ) \gamma_{\alpha} ( p / - i\mu ) \tag{569}
$$

where $F_{1} ,\dots ,F_{4}$ are functions of the scalar $p^{2}$. Hence using (521) and (523), we see that in (564) we may put

<a id="eq-570"></a>

$$
\Sigma ( p + \epsilon ) = - 6 \pi^{2} \mu R^{\prime} - I_{1} /\epsilon - I_{2} ( p / - i\mu ) /\epsilon \tag{570}
$$

where $I_{1}$ and $I_{2}$ are new absolute constants, and in particular

<a id="eq-571"></a>

$$
I_{1} = F_{1} ( - \mu^{2} ) \tag{571}
$$

But in (564) the term

$$
\left( \frac{1}{p / + /\epsilon + /\epsilon^{\prime} - i\mu} \right) ( p / - i\mu ) /\epsilon = /\epsilon - \frac{1}{( p / + /\epsilon + /\epsilon^{\prime} - i\mu )} ( /\epsilon + /\epsilon^{\prime} ) /\epsilon
$$

is of order $\epsilon$ and tends to zero as $T \to$ 0. This term may be dropped, and then (564) becomes<sup><a id="fnref-78" href="#fn-78">78</a></sup>

<a id="eq-572"></a>

$$
M_{2 2} = - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \int \int G \left\{ \overline{u} /e ( q - \epsilon - \epsilon^{\prime} ) \frac{1}{p / + /\epsilon + /\epsilon^{\prime} - i\mu} ( - 6 \pi^{2} \mu R^{\prime} - I_{1} /\epsilon ) u \right\} ( \epsilon_{0} ) G ( \epsilon_{0}^{\prime} ) d\epsilon_{0} d\epsilon_{0}^{\prime} \times \tag{572}
$$

Note that if $I_{\alpha} ( p )$ given by (567) were to be evaluated assuming that $p^{2} + \mu^{2}$ = 0 and $p / - i\mu$ = 0 operating both to the left and to the right, instead of only to the right as in (521), the result obtained would be just

<a id="eq-573"></a>

$$
I_{\alpha} ( p ) = I_{1} \gamma_{\alpha} \tag{573}
$$

<a id="p110"><sup>[p. 110]</sup></a>

This is a convenient definition of $I_{1}$ for future reference.

Now it is clear that the term $R^{\prime}$ in $M_{2 2}$ represents some kind of effect of the electron self-energy, which ought not to be observable. We may expect that this term will be cancelled by the term $M_{2}^{\prime}$ arising from the self-energy correction $H_{S}$ in (532). This is all the more plausible, because the graph $G_{2}$ on page 106 contains as a part the graph $G_{5}$ on page 99 which represents the self-energy of a free electron. We now turn to the calculation of $M_{2}^{\prime}$.

$M_{2}^{\prime}$ is the sum of two contributions arising from the two graphs shown below.

![Figure 11 (p. 110)](images/fig-p116-1.png)

_Figure 11 (p. 110)_

From $G_{2}^{\prime}$ the contribution is (compare with (528))

<a id="eq-574"></a>

$$
M_{2 2}^{\prime} = - \frac{ie\delta m}{2 \hbar^{2}} \int \int dxdx_{1} \overline{\psi} ( x ) A /^{e} S_{F} ( x - x_{1} ) \psi ( x_{1} ) \tag{574}
$$

Like (555) this <a id="ix-97">integral</a> does not oscillate but diverges at $t_{1} = \pm \infty$. Therefore we must take explicitly into account the cut-off factor multiplying the radiation interaction. At time $t_{1}$ the radiation interaction $H_{R} ( t_{1} )$ will carry the cut-off factor $g ( t_{1} )$. But the self-energy $\delta m$ at time $t_{1}$ is an effect of second order in $H_{R}$, and therefore becomes multiplied by $[ g ( t_{1} ) ]$<sup>2</sup> if $g ( t_{1} )$ varies slowly enough. In the definition of the scattering matrix element (487), the cut-off factors $g ( t )$ were introduced in order to represent the initial and final states by simple bare-particle wave-functions in an unambiguous way. We now require that the bare-particles’ wave-functions should always have the same mass $m$ as a real electron. This is achieved if we add the term $( - H_{S} )$ to the radiation interaction appearing in (495), each $H_{S} ( t_{i} )$ being multiplied by the cut-off factor $[ g ( t_{i} ) ]$<sup>2</sup> so long as we are dealing only with second order terms in $\delta m$. If we were calculating effects to fourth order in $e$, then the fourth order part of $\delta m$ should be multiplied by $[ g ( t_{i} ) ]$<sup>4</sup>, and so on.

The effect of the cut-off factors is therefore to replace (574) by

<a id="eq-575"></a>

$$
M_{2 2}^{\prime} = - \frac{ie\delta m}{2 \hbar^{2}} \int \int dxdx_{1} \overline{\psi} ( x ) A /^{e} S_{F} ( x - x_{1} ) \psi ( x_{1} ) [ g ( t_{1} ) ]^{2} \tag{575}
$$

Using (561) and performing the integrations as before, this becomes

<a id="eq-576"></a>

$$
M_{2 2}^{\prime} = - \frac{ie\delta m}{2 \hbar^{2}} \int \int G ( \epsilon_{0} ) G ( \epsilon_{0}^{\prime} ) d\epsilon_{0} d\epsilon_{0}^{\prime} \left\{ \overline{u}^{\prime} /e ( q - \epsilon - \epsilon^{\prime} ) \left( \frac{1}{p / + /\epsilon + /\epsilon^{\prime} - i\mu} \right) u \right\} \tag{576}
$$

In virtue of (530), this term does precisely cancel the term in $R^{\prime}$ in (572), as was to be expected.

To simplify the term in $I_{1}$ in (572), we may replace $/\epsilon$ by $\tfrac{1}{2} ( /\epsilon + /\epsilon^{\prime} )$ since the integrand is otherwise symmetrical between $\epsilon$ and $\epsilon^{\prime}$. And using (521) we may replace this in turn by $\tfrac{1}{2} ( p / + /\epsilon + /\epsilon^{\prime} - i\mu )$. This cancels the denominator of (572) precisely. After the denominator is cancelled, the expression is non-singular and we may go to the limit $T \to \infty$, using (563) to perform the integration over $\epsilon_{0}$ and $\epsilon_{0}^{\prime}$. Since it is assumed that the external potential is of a limited duration not tending to infinity with $T$, the factor $/e ( q - \epsilon - \epsilon^{\prime} )$ is a continuous function of $\epsilon + \epsilon^{\prime}$ and tends to $/e ( q )$ as $T \to \infty$.<sup><a id="fnref-79" href="#fn-79">79</a></sup> Hence in the limit $T \to \infty$ we have

<a id="eq-577"></a>

$$
M_{2 2} + M_{2 2}^{\prime} = \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} \frac{1}{2} I_{1} ( \overline{u}^{\prime} /\epsilon u ) = \frac{i\alpha}{( 2 \pi )^{3}} I_{1} M_{o} \tag{577}
$$

<a id="p111"><sup>[p. 111]</sup></a>

The graphs $G_{3}$ and $G_{3}^{\prime}$ give an exactly equal contribution. Hence

<a id="eq-578"></a>

$$
M_{2 2} + M_{2 3} + M_{2 2}^{\prime} + M_{2 3}^{\prime} = \frac{i\alpha}{4 \pi^{3}} I_{1} M_{o} \tag{578}
$$

It turns out that $I_{1}$ is pure imaginary, and the factor multiplying $M_{o}$ in (578) is real and negative.

What is the physical interpretation of the divergent term (578)? It is just a divergent constant multiple of $M_{o}$, like the charge-renormalization term in (552). So one is tempted at first to call it an additional charge-renormalization effect. But this cannot be right, because the whole charge-renormalization was calculated in (392) and the result agreed with (552). In fact (578) has a much more elementary interpretation. When the electron arrives at the point $x$ of the external potential where it undergoes scattering, there will be a certain probability $P$ that it will have previously emitted and not yet reabsorbed a photon, as pictured in the Feynman graph $G_{4}$. There will be a probability (1 $- P )$ that it will arrive at $x$ not accompanied by a photon, as pictured in $G_{2}$ or $G_{3}$.

Consider then the contribution $M_{NP}$ to the matrix element $M$, produced by scattering processes in which the electron arrives at $x$ with no photon present. In the zero-order approximation $M_{NP} = M_{o}$ simply. But in the second-order approximation we must take account of the reduced probability that the electron comes to $x$ without a photon present; this is done by multiplying the wave-function of the electron in both initial and final states by the factor

<a id="eq-579"></a>

$$
( 1 - P )^{1 / 2} \tag{579}
$$

Therefore in second-order approximation

<a id="eq-580"></a>

$$
M_{NP} = ( 1 - P ) M_{o} \tag{580}
$$

Since in second order

<a id="eq-581"></a>

$$
M_{NP} = M_{o} + M_{2 2} + M_{2 2}^{\prime} + M_{2 3} + M_{2 3}^{\prime} , \tag{581}
$$

the result (578) agrees with (580) provided that we identify $P$ with

<a id="eq-582"></a>

$$
P = - \frac{i\alpha}{4 \pi^{3}} I_{1} \tag{582}
$$

The factor (579) represents a renormalization of the amplitude of the wave-function, and for this reason (578) is usually called a “wave-function renormalization” effect. But this does not mean that the term (578) is to be removed by a process analogous to mass or charge renormalization. No difficulties arise if we simply retain (578) as it stands. Finally it will cancel out against a term (+$PM_{o} )$ which comes from $G_{4}$.

The contribution from $G_{4}$ to $M$ is, with a factor 2 to allow for $G_{9}$, using the Feynman rules,

<a id="eq-583"></a>

$$
\begin{aligned} M_{2 4} &= \frac{e^{3}}{8 \hbar^{2} c^{2}} \int \int dxdx_{1} dx_{2} \sum_{\lambda} \psi ( x ) \gamma_{\lambda} S_{F} ( x_{1} - x ) A /^{e} ( x ) S_{F} ( x - x_{2} ) \gamma_{\lambda} \psi ( x_{2} ) D_{F} ( x_{2} - x_{1} ) \\ &= - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} ( \overline{u}^{\prime} \Lambda ( p,p^{\prime} ) u ) \end{aligned} \tag{583}
$$

where

<a id="eq-584"></a>

$$
\Lambda ( p,p^{\prime} ) = \int_{F} dk \sum_{\lambda} \left( \gamma_{\lambda} \frac{1}{/ k + p /^{\prime} - i\mu} /e \frac{1}{k / + p / - i\mu} \gamma_{\lambda} \right) \frac{1}{k^{2}} \tag{584}
$$

There is no singular factor in (584) such as we had in (558). To perform the sum over $\lambda$ in (584), we use <a id="p112"><sup>[p. 112]</sup></a> the table

<a id="eq-585"></a>

$$
\begin{aligned} \sum_{\lambda} \gamma_{\lambda} \gamma_{\lambda} &= 4 \\ \sum_{\lambda} \gamma_{\lambda} a\gamma /_{\lambda} &= - 2 a / \\ \sum_{\lambda} \gamma_{\lambda} a/b\gamma /_{\lambda} &= 4 ( a \cdot b ) \\ \sum_{\lambda} \gamma_{\lambda} a/b/c\gamma /_{\lambda} &= - 2 /c/b/ a \end{aligned} \tag{585}
$$

valid for any vectors $a , b , c$. These formulae can be deduced from the following recursive formula:

Denote $/q_{( n )} = q /_{1} /q_{2} \dots /q_{n}$ where $q_{i}$ are arbitrary vectors, and $\chi_{n} = \sum_{\lambda} \gamma_{\lambda} q /_{( n )} \gamma_{\lambda} , \chi_{o}$ = 4. Then we have

$$
\chi_{n + 1} = \sum_{\lambda} \gamma_{\lambda} /q_{( n )} /q_{n + 1} \gamma_{\lambda} = \sum_{\lambda} \sum_{\mu} \gamma_{\lambda} /q_{( n )} \gamma_{\mu} \gamma_{\lambda} ( q_{n + 1} )_{\mu} = \sum_{\lambda} \sum_{\mu} \gamma_{\lambda} /q_{( n )} [ 2 \delta_{\lambda \mu} - \gamma_{\lambda} \gamma_{\mu} ] ( q_{n + 1} )_{\mu}
$$

$$
= 2 /q_{n + 1} /q_{( n )} - \chi_{n} q /_{n + 1}
$$

which then gives (585) for $n$ = 1$, 2 ,$ 3. Thus

<a id="eq-586"></a>

$$
\Lambda ( p,p^{\prime} ) = - 2 \int_{F} dk \frac{( / k + p / ) /e ( k / + p /^{\prime} ) - 2 i\mu ( 2 e \cdot k + e \cdot p + e \cdot p^{\prime} ) - \mu^{2} /e}{k^{2} [ k^{2} + 2 k \cdot p^{\prime} ] [ k^{2} + 2 k \cdot p ]} \tag{586}
$$

In (583) we may use the relations

<a id="eq-587"></a>

$$
p^{2} + \mu^{2} = p^{\prime 2} + \mu^{2} = 0 \quad ( p / - i\mu ) u = 0 \quad \overline{u}^{\prime} ( p /^{\prime} - i\mu ) = 0 \tag{587}
$$

We also assume that the external potential satisfies the Lorentz condition

<a id="eq-588"></a>

$$
\sum_{\lambda} \frac{\partial A_{\mu}^{e}}{\partial x_{\mu}} = 0 \quad \text{ so that } \quad e \cdot q = 0 \tag{588}
$$

To evaluate (586) we use the 3-variable generalization of (376)

<a id="eq-589"></a>

$$
\frac{1}{abc} = 2 \int_{0}^{1} dx \int_{0}^{1} xdy \frac{1}{[ a ( 1 - x ) + bxy + cx ( 1 - y ) ]^{3}} \tag{589}
$$

which one can verify at once by direct integration. We write

<a id="eq-590"></a>

$$
\begin{aligned} p_{y} &= py + p^{\prime} ( 1 - y ) \\ p_{y}^{2} &= [ - ( p^{\prime} - p ) y + p^{\prime} ]^{2} = q^{2} y^{2} - ( 2 p^{\prime 2} - 2 p \cdot p^{\prime} ) y - \mu^{2} = q^{2} y^{2} - ( p^{\prime 2} - 2 p \cdot p^{\prime} + p^{2} ) - \mu^{2} \\ &= - \mu^{2} - ( y - y^{2} ) q^{2} \end{aligned} \tag{590}
$$

Then changing the origin of the $k$-integration by the substitution $k \to k - xp_{y}$ (586) and (589) give

<a id="eq-591"></a>

$$
\Lambda ( p,p^{\prime} ) = - 4 \int \int xdxdy \int_{F} dk \frac{( / k - x/p_{y} + p / ) /e ( k / - x/p_{y} + p /^{\prime} ) - 2 i\mu e \cdot ( 2 k - 2 xp_{y} + p + p^{\prime} ) - \mu^{2} /e}{\left[ k^{2} - x^{2} p_{y}^{2} \right]^{3}} \tag{591}
$$

In (591) we may drop terms which are odd in $k$. Also using (587) and (588) we may put

<a id="eq-592"></a>

$$
e \cdot p = e \cdot p^{\prime} = e \cdot p_{y} = i\mu /e + \tfrac{1}{2} /e/q \tag{592}
$$

<a id="p113"><sup>[p. 113]</sup></a>

<a id="eq-593"></a>

$$
\begin{aligned} ( /p - x/p_{y} ) /e ( p /^{\prime} - x/p_{y} ) &= \left\{ ( 1 - x ) i\mu - ( 1 - xy ) /q \right\} /e \left\{ ( 1 - x ) i\mu + ( 1 - x + xy ) /q \right\} \\ &= - ( 1 - x )^{2} \mu^{2} /e + ( 1 - x ) i\mu /e/q ( 2 - x ) + ( 1 - xy ) ( 1 - x + xy ) q^{2} /e \end{aligned} \tag{593}
$$

Collecting terms from (592), (593), we have

<a id="eq-594"></a>

$$
\Lambda ( p,p^{\prime} ) = - 4 \int \int xdxdy \int_{F} dk \frac{/ k/e/ k + ( 1 - xy ) ( 1 - x + xy ) q^{2} /e - ( x - x^{2} ) i\mu /e/q + ( 2 - 2 x - x^{2} ) \mu^{2} /e}{[ k^{2} + x^{2} ( \mu^{2} + ( y - y^{2} ) q^{2} ) ]^{3}} \tag{594}
$$

Now we saw earlier that the integral (567), evaluated using the conditions $p^{2} + \mu^{2}$ = 0$,/p - i\mu$ = 0, has the value (573). Comparing (567) with (584), this implies that

<a id="eq-595"></a>

$$
\Lambda ( p,p ) = \Lambda ( p^{\prime} ,p^{\prime} ) = I_{1} /e \tag{595}
$$

when the conditions (587) with $p^{\prime} = p$ are assumed to hold. Thus $( I_{1} /e )$ is just the value of (594) for $p^{\prime} = p$. Adding together (583) and (578), this gives

<a id="eq-596"></a>

$$
M_{2 T} = M_{2 4} + M_{2 2} + M_{2 2}^{\prime} + M_{2 3} + M_{2 3}^{\prime} = - \frac{ie^{3}}{( 2 \pi )^{4} \hbar^{2} c^{2}} ( \overline{u}^{\prime} \Lambda_{c} ( p,p^{\prime} ) u ) \tag{596}
$$

$$
\Lambda_{c} ( p,p^{\prime} ) = \Lambda ( p,p^{\prime} ) - \tfrac{1}{2} \{ \Lambda ( p,p ) + \Lambda ( p^{\prime} ,p^{\prime} ) \}
$$

<a id="eq-597"></a>

$$
\begin{aligned} &= - 4 \int \int xdxdy \left\{ \int_{F} dk \left[ / k/e/ k + ( 2 - 2 x - x^{2} ) \mu^{2} /e \right] \left( \frac{1}{[ k^{2} + x^{2} ( \mu^{2} + ( y - y^{2} ) q^{2} ) ]^{3}} - \frac{1}{[ k^{2} + x^{2} \mu^{2} ]^{3}} \right) \right. \\ & \quad \left. + \left[ ( 1 - xy ) ( 1 - x + xy ) q^{2} /e - ( x - x^{2} ) i\mu /e/q \right] \int_{F} dk \frac{1}{[ k^{2} + x^{2} ( \mu^{2} + ( y - y^{2} ) q^{2} ) ]^{3}} \right\} \end{aligned} \tag{597}
$$

The $k$-integrals in (597) are now convergent. Thus the effect of the “<a id="ix-224">wave-function</a> renormalization” term (578) is just to cancel out the part of $M_{2 4}$ which is independent of $q$ and divergent at high frequencies.

To evaluate (597) we use (385) and (386). First, in the term $k/e/ / k$ we may replace $k_{\mu} k_{\nu}$ by $\tfrac{1}{4} \delta_{\mu \nu} k^{2}$ because of the symmetry of the integral in $k$-space. So we may write, using (585)

<a id="eq-598"></a>

$$
/ k/e/ k = \sum_{\alpha} \tfrac{1}{4} k^{2} \gamma_{\alpha} /e\gamma_{\alpha} = - \tfrac{1}{2} k^{2} /e \tag{598}
$$

Then by (385) and (386)

<a id="eq-599"></a>

$$
\begin{aligned} & \quad \int_{F} dkk^{2} \left\{ \frac{1}{[ k^{2} + \Lambda ]^{3}} - \frac{1}{[ k^{2} + \Lambda^{\prime} ]^{3}} \right\} = \\ &= \int_{F} dkk^{2} \left\{ \frac{1}{[ k^{2} + \Lambda ]^{2}} - \frac{1}{[ k^{2} + \Lambda^{\prime} ]^{2}} - \frac{\Lambda}{[ k^{2} + \Lambda ]^{3}} + \frac{\Lambda^{\prime}}{[ k^{2} + \Lambda^{\prime} ]^{3}} \right\} = \pi^{2} i \log \frac{\Lambda^{\prime}}{\Lambda} \end{aligned} \tag{599}
$$

and (597) becomes

$$
\begin{aligned} \Lambda_{c} ( p,p^{\prime} ) &= 2 \pi^{2} i \int \int xdxdy \left\{ - /e \log \left[ 1 + ( y - y^{2} ) \frac{q^{2}}{\mu^{2}} \right] \right. \\ & \quad \left. + \frac{\{ x - 1 + 2 ( y - y^{2} ) ( 1 - x - x^{2} ) \} q^{2} /e + ( x - x^{2} ) i\mu /e/q}{x^{2} ( \mu^{2} + ( y - y^{2} ) q^{2}} \right\} \end{aligned}
$$

Integrating the logarithmic term by parts with respect to $y$,

<a id="eq-600"></a>

$$
\Lambda_{c} ( p,p^{\prime} ) = - 2 \pi^{2} i \int_{0}^{1} \int_{0}^{1} dxdy \frac{1}{x [ \mu^{2} + ( y - y^{2} ) q^{2} ]} \left\{ \left[ ( 1 - x ) ( 1 - 2 y + 2 y^{2} ) + x^{2} y \right] q^{2} /e - ( x - x^{2} ) i\mu /e/q \right\} \tag{600}
$$

<a id="p114"><sup>[p. 114]</sup></a>

When $q^{2} < - 4 \mu^{2}$, the external potential is able to create real pairs, and the denominator in (600) has poles in the range of the $y$-integration. In this case the Feynman rule of adding a term $( - i\epsilon )$ to $\mu^{2}$, where $\epsilon$ is an infinitesimal positive real number, will give an unambiguous determination of the integral. Just as in the case of the vacuum polarization formula (389), the integral will split into a real part and an imaginary part describing the effects of the real pairs created. We shall not discuss these effects of real pairs since they are not practically important. So we assume $q^{2} > - 4 \mu^{2}$.

In (600) there are no longer any divergences arising from large $k$. But (584) has a logarithmic divergence at small $k$ which appears in (600) as a divergence in the $x$-integration arising from the factor (1$/x )$. This last remaining divergence must now be examined in detail. It is the famous “Infra-Red Catastrophe”.

To discover the physical meaning of the $x$-divergence, we consider what would be the effect on our calculations if the Maxwell field were somehow modified so that all field oscillations with wave-numbers satisfying

<a id="eq-601"></a>

$$
| \boldsymbol{k} |\geq r \tag{601}
$$

were present as usual, while all oscillations not satisfying (601) were simply absent or incapable of being excited. We suppose $r$ to be a constant small compared with $m , p , p^{\prime}$ and $q$. Thus photons will exist only if their energies exceed

<a id="eq-602"></a>

$$
\Delta E = \hbar cr \tag{602}
$$

In the modified Maxwell theory the $D_{F}$ function will still be given by the integral (431), the $k_{1} ,k_{2} ,k_{3}$ integrations being limited by (601), and the $k_{0}$ integration being taken as usual along the whole real axis from $-\infty$ to +$\infty$. Let Λ${}^{r} ( p,p^{\prime} )$ and Λ$\frac{r}{c} ( p,p^{\prime} )$ be the integrals which replace Λ($p,p^{\prime} )$ and Λ${}_{c} ( p,p^{\prime} )$ when the Maxwell field is modified. We calculate the differences (Λ $-$ Λ${}^{r} )$ and (Λ${}_{c} -$ Λ$\frac{r}{c} )$, considering these integrals only in the limit of small $r$, neglecting all terms which tend to zero with $r$. This means that we may neglect terms containing either $k$ or $x$ as a factor in the numerator of integrals such as (591) or (594).

In (583) there is just one factor $D_{F}$. Thus Λ${}^{r} ( p,p^{\prime} )$ is obtained from (584) simply by restricting the $k_{1} ,k_{2} ,k_{3}$ integration by (601). We can now follow the reduction of (584) to the form (594), except that we do not shift the origin of the $k$-integration through $( xp_{y} )$ since this would disturb the <a id="ix-99">condition</a> (601). Dropping terms in the numerator having $k$ or $x$ as a factor, this gives the result

<a id="eq-603"></a>

$$
\Lambda ( p,p^{\prime} ) - \Lambda^{r} ( p,p^{\prime} ) = - 4 \int \int xdxdy ( q^{2} + 2 \mu^{2} ) /e \int_{F} \frac{dk}{[ k^{2} + 2 xk \cdot p_{y} ]^{3}} \tag{603}
$$

Hence by (597)

<a id="eq-604"></a>

$$
\begin{aligned} & \quad \Lambda_{c} ( p,p^{\prime} ) - \Lambda_{c}^{r} ( p,p^{\prime} ) = \\ &= - 4 \int \int xdxdy/e \int_{F} dk \left\{ \frac{q^{2} + 2 \mu^{2}}{[ k^{2} + 2 xk \cdot p_{y} ]^{3}} - \frac{\mu^{2}}{[ k^{2} + 2 xk \cdot p ]^{3}} - \frac{\mu^{2}}{[ k^{2} + 2 xk \cdot p^{\prime} ]^{3}} \right\} \end{aligned} \tag{604}
$$

The integral (604), with the integration extended over the whole $k$-space, would give, using (385), (587), and the substitutions $k \to k - xp_{y} ,k \to k - xp,k \to xp^{\prime}$, respectively in the three integrals,

<a id="eq-605"></a>

$$
- 2 \pi i \int \int dxdy \frac{( 1 - 2 y + 2 y^{2} ) q^{2}}{x ( \mu^{2} + ( y - y^{2} ) q^{2} )} /e \tag{605}
$$

which is just the divergent part of (600). But the integration in (604) actually extends over $k$ not satisfying (601). Therefore subtracting (604) from (600) and using (605), we find for Λ$\frac{r}{c} ( p,p^{\prime} )$ the final result

<a id="eq-606"></a>

$$
\begin{aligned} \Lambda_{c}^{r} ( p,p^{\prime} ) &= - 2 \pi^{2} i \int_{0}^{1} dy \frac{1}{\mu^{2} + ( y - y^{2} ) q^{2}} \left\{ ( - 1 + \tfrac{5}{2} y - 2 y^{2} ) q^{2} /e - \tfrac{1}{2} i\mu /e/q \right\} \\ & \quad - 4 \int \int xdxdy/e \int_{F} dk \left\{ \frac{q^{2} + 2 \mu^{2}}{[ k^{2} + 2 xk \cdot p_{y} ]^{3}} - \frac{\mu^{2}}{[ k^{2} + 2 xk \cdot p ]^{3}} - \frac{\mu^{2}}{[ k^{2} + 2 xk \cdot p^{\prime} ]^{3}} \right\} \end{aligned} \tag{606}
$$

<a id="p115"><sup>[p. 115]</sup></a>

This integral is completely convergent for every finite $r$, the $k$-integration being restricted to $k$ satisfying (601). The formula (606) is exact except for terms which tend to zero with $r$.

To evaluate the $k$ integral in (606) for general $p$ and $p^{\prime}$ is possible but tedious. So we shall do it only in the case of non-relativistic velocities, when

<a id="eq-607"></a>

$$
| \boldsymbol{p} |\ll \mu , \quad | \boldsymbol{p}^{\prime} |\ll \mu , \quad | \boldsymbol{q} |\ll \mu \tag{607}
$$

where $| \boldsymbol{p} |$ means $\sqrt{p_{1}^{2} + p_{x}^{2} + p_{3}^{2}}$, the magnitude of the space-like part of the 4-vector $p$. In addition to (607) we still assume $r$ small compared to $q , p , p^{\prime}$.

We consider the integral

<a id="eq-608"></a>

$$
K = \int_{0}^{1} \int_{0}^{1} xdxdy \int_{F} dk \frac{1}{[ k^{2} + 2 xk \cdot p_{y} ]^{3}} \tag{608}
$$

integrated over $k$ satisfying (601), and evaluate it including terms of order $| \boldsymbol{p} |^{2} , q^{2} , | \boldsymbol{p}^{\prime} |^{2}$, but neglecting higher terms. Integrating over $k_{0}$ only, we have for any positive $b$

<a id="eq-609"></a>

$$
\int_{F} dk_{0} \frac{1}{[ k^{2} + 2 ak_{0} + b ]} = i \int_{-\infty}^{\infty} \frac{dk_{0}}{[ | \boldsymbol{k} |^{2} + k_{0}^{2} + 2 iak_{0} + b ]} = i\pi \frac{1}{\sqrt{| \boldsymbol{k} |^{2} + a^{2} + b}} \tag{609}
$$

Differentiating (609) twice with respect to $b$,

<a id="eq-610"></a>

$$
\int_{F} \frac{dk_{0}}{[ k^{2} + 2 ak_{0} + b ]^{3}} = \frac{3 i\pi}{8} \left\{ | \boldsymbol{k} |^{2} + a^{2} + b \right\}^{- 5 / 2} \tag{610}
$$

Hence<sup><a id="fnref-80" href="#fn-80">80</a></sup>

<a id="eq-611"></a>

$$
\begin{aligned} K &= \int \int xdxdy \frac{3 i\pi}{8} \int_{| \boldsymbol{k} | >r} d^{3} \boldsymbol{k} \left\{ | \boldsymbol{k} + x \boldsymbol{p}_{y} |^{2} + x^{2} \left( \mu^{2} + ( y - y^{2} ) q^{2} \right) \right\}^{- 5 / 2} \\ &= \int \int xdxdy \frac{3 i\pi}{8} \int_{| \boldsymbol{k} | >r} d^{3} \boldsymbol{k} \left\{ \left( | \boldsymbol{k} |^{2} + x^{2} \mu^{2} \right)^{- 5 / 2} - \tfrac{5}{2} \left( 2 x \boldsymbol{k} \cdot \boldsymbol{p}_{y} + x^{2} | \boldsymbol{p}_{y} |^{2} + x^{2} ( y - y^{2} ) q^{2} \right) \times \right. \\ & \quad \left. \left( | \boldsymbol{k} |^{2} + x^{2} \mu^{2} \right)^{- 7 / 2} + \tfrac{3 5}{8} 4 x^{2} ( \boldsymbol{k} \cdot \boldsymbol{p}_{y} )^{2} \left( | \boldsymbol{k} |^{2} + x^{2} \mu^{2} \right)^{- 9 / 2} \right\} \\ &= \int \int xdxdy \frac{3 i\pi^{2}}{2} \int_{r}^{\infty} k^{2} dk \left\{ \left( k^{2} + x^{2} \mu^{2} \right)^{- 5 / 2} - \tfrac{5}{2} x^{2} \left( | \boldsymbol{p}_{y} |^{2} + ( y - y^{2} ) q^{2} \right) \left( k^{2} + x^{2} \mu^{2} \right)^{- 7 / 2} \right. \\ & \quad \left. + \tfrac{3 5}{6} x^{2} k^{2} | \boldsymbol{p}_{y} |^{2} \left( k^{2} + x^{2} \mu^{2} \right)^{- 9 / 2} \right\} \end{aligned} \tag{611}
$$

We can now carry out the integrations over $x$ and $y$, using

<a id="eq-612"></a>

$$
\int_{0}^{1} | \boldsymbol{p}_{y} |^{2} dy = \boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + \tfrac{1}{3} q^{2} \tag{612}
$$

This gives

<a id="eq-613"></a>

$$
\begin{aligned} K &= \frac{3 i\pi^{2}}{2} \int_{r}^{\infty} k^{2} dk \left\{ \frac{1}{3 \mu^{2}} \left( \frac{1}{k^{3}} - \frac{1}{( k^{2} + \mu^{2} )^{3 / 2}} \right) - \frac{\boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + \tfrac{1}{2} q^{2}}{3 \mu^{4}} \left( \frac{1}{k^{3}} - \frac{1}{( k^{2} + \mu^{2} )^{3 / 2}} \right. \right. \\ & \quad \left. - \frac{1}{2 \mu^{2} ( k^{2} + \mu^{2} )^{5 / 2}} \right) + \tfrac{1}{6} k^{2} \left[ \boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + \tfrac{1}{3} q^{2} \right] \frac{2}{\mu^{4}} \left( \frac{1}{k^{5}} - \frac{1}{( k^{2} + \mu^{2} )^{5 / 2}} \right. \\ & \quad \left. \left. - \frac{5}{\mu^{2}} \frac{1}{( k^{2} + \mu^{2} )^{7 / 2}} \right) \right\} \end{aligned} \tag{613}
$$

The $k$-integration is now elementary, and after dropping terms which tend to zero with $r$ we have

<a id="eq-614"></a>

$$
\begin{aligned} K &= \frac{3 i\pi^{2}}{2 \mu^{2}} \left\{ \tfrac{1}{3} \left( \log \frac{\mu}{2 r} + 1 \right) - \frac{\boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + \tfrac{1}{2} q^{2}}{\mu^{2}} \left( \tfrac{1}{3} \log \frac{\mu}{2 r} + \tfrac{1}{6} \right) \right. \\ & \quad \left. + \tfrac{1}{6} \frac{\boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + \tfrac{1}{3} q^{2}}{\mu^{2}} \left( 2 \log \frac{\mu}{2 r} + \tfrac{5}{3} \right) \right\} \end{aligned} \tag{614}
$$

<a id="p116"><sup>[p. 116]</sup></a>

Putting $p = p^{\prime} , q$ = 0 in (614), we find

<a id="eq-615"></a>

$$
K_{0} = \int_{0}^{1} \int_{0}^{1} xdxdy \int_{F} dk \frac{1}{[ k^{2} + 2 xk \cdot p ]^{3}} = \frac{3 i\pi^{2}}{2 \mu^{2}} \left\{ \tfrac{1}{3} \left( \log \frac{\mu}{2 r} + 1 \right) + \tfrac{1}{9} \frac{| \boldsymbol{p} |^{2}}{\mu^{2}} \right\} \tag{615}
$$

Substituting (614) and (615) into (606) and dropping terms of higher order than $| \boldsymbol{p} |^{2} , | \boldsymbol{p}^{\prime} |^{2}$ and $q^{2}$,

<a id="eq-616"></a>

$$
\begin{aligned} \Lambda_{c}^{r} ( p,p^{\prime} ) &= - 2 \pi^{2} i \left\{ - \tfrac{5}{1 2} \frac{q^{2}}{\mu^{2}} /e - \tfrac{1}{2} \frac{i}{\mu} /e/q \right\} - 4 /e \frac{3 i\pi^{2}}{2 \mu^{2}} \left\{ \tfrac{1}{3} q^{2} \left( \log \frac{\mu}{2 r} + 1 \right) - \tfrac{1}{1 8} q^{2} \left( 2 \log \frac{\mu}{2 r} + \tfrac{5}{3} \right) \right\} \\ &= - \tfrac{4}{3} \pi^{2} i \frac{q^{2}}{\mu^{2}} /e \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} \right\} - \frac{\pi^{2}}{\mu} /e/q \end{aligned} \tag{616}
$$

Using (596) and adding the contribution (554) from $G_{1}$, we find for the second-order terms in (541) the value

<a id="eq-617"></a>

$$
M_{2} + M_{2}^{\prime} = - \frac{\alpha}{3 \pi} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \frac{q^{2}}{\mu^{2}} M_{0} + \frac{\alpha}{4 \pi} \frac{ie}{mc^{2}} \left( \overline{u}^{\prime} /e/qu \right) \tag{617}
$$

## <a id="s-treatment-of-low-frequency-photons-the-infra-red-catastrophe"></a>The Treatment of Low-Frequency Photons. The Infra-Red Catastrophe.

The second-order correction (617) to the scattering matrix element $M_{0}$ has been made convergent by only taking into account the effects of photons with energy greater than ∆$E$ according to (603). When ∆$E \to$ 0 the correction diverges logarithmically, and this divergence must now be interpreted.

In the approximation where $| \boldsymbol{p} |^{2}$ and $| \boldsymbol{p}^{\prime} |^{2}$ are small compared with $\mu^{2}$, (546) gives

<a id="eq-618"></a>

$$
M_{1} = \frac{e}{\hbar c\mu | \boldsymbol{k}^{\prime} |} ( q \cdot e^{\prime} ) M_{0} \tag{618}
$$

The total probability that an electron is scattered between the initial and final states (542), with the emission of a photon with potentials (440), summed over all photons with frequencies lying in the range<sup><a id="fnref-81" href="#fn-81">81</a></sup>

<a id="eq-619"></a>

$$
r_{1} < | \boldsymbol{k}^{\prime} | <r_{2} \tag{619}
$$

is therefore

<a id="eq-620"></a>

$$
\begin{aligned} W_{R} ( r_{1} ,r_{2} ) &= \int d^{3} \boldsymbol{k}^{\prime} \sum_{e^{\prime}} \frac{1}{( 2 \pi )^{3}} \left( \frac{\hbar c}{2 | \boldsymbol{k}^{\prime} |} \right) | M_{1} |^{2} \\ &= \frac{e^{2}}{1 6 \pi^{3} \hbar c\mu^{2}} | M_{0} |^{2} \int d^{3} \boldsymbol{k}^{\prime} \frac{1}{| \boldsymbol{k}^{\prime} |^{3}} \sum_{e^{\prime}} | q \cdot e^{\prime} |^{2} \\ &= \frac{\alpha}{\pi \mu^{2}} | M_{0} |^{2} \int_{r_{1}}^{r_{2}} \frac{dk^{\prime}}{k^{\prime}} \tfrac{2}{3} q^{2} \\ &= \frac{2 \alpha}{3 \pi} \left( \log \frac{r_{2}}{r_{1}} \right) \frac{q^{2}}{\mu^{2}} | M_{0} |^{2} \end{aligned} \tag{620}
$$

In (620) it is assumed that both $r_{2}$ and $r_{1}$ are frequencies small compared with $| \boldsymbol{q} |$.

On the other hand, the probability that an electron is scattered between the states (542) without emitting a photon is given by

<a id="eq-621"></a>

$$
\begin{aligned} W_{N} &= | M_{0} + M_{2} + M_{2}^{\prime} |^{2} \\ &= | M_{0} |^{2} + M_{0}^{*} ( M_{2} + M_{2}^{\prime} ) + ( M_{2} + M_{2}^{\prime} )^{*} M_{0} \end{aligned} \tag{621}
$$

neglecting terms of fourth order in the radiation interaction. If we consider in (621) the contribution only from virtual photons with frequencies in the range (619), then by (617) we have

<a id="eq-622"></a>

$$
\begin{aligned} M_{2} + M_{2}^{\prime} &= - \frac{\alpha}{3 \pi} \left( \log \frac{r_{2}}{r_{1}} \right) \frac{q^{2}}{\mu^{2}} | M_{0} |^{2} \\ W_{N} ( r_{1} ,r_{2} ) &= | M_{0} |^{2} - \frac{2 \alpha}{3 \pi} \left( \log \frac{r_{2}}{r_{1}} \right) \frac{q^{2}}{\mu^{2}} | M_{0} |^{2} \end{aligned} \tag{622}
$$

<a id="p117"><sup>[p. 117]</sup></a>

So the contributions to (617) from low-frequency virtual photons serve only to compensate exactly the probability for scattering with emission of low-frequency real photons. The non-radiative probability is decreased by the effects of low-frequency virtual photons, so that the total scattering probability, radiative plus non-radiative, is essentially independent of the presence of very low-frequency photons. The total scattering probability is thus a finite quantity free from any infra-red divergence.

To describe correctly the radiative corrections to scattering it is essential to specify the critical energy ∆$E$ below which real photons will not be detected. Ideally we suppose that in each scattering event a photon of energy greater than ∆$E$ is detected with 100% efficiency, a photon of energy less than ∆$E$ with zero efficiency. Then the total observed probability for non-radiative scattering is given by (621), with $M_{2} + M_{2}^{\prime}$ given by (617) and

<a id="eq-623"></a>

$$
\log \frac{\mu}{2 r} = \log \left( \frac{mc^{2}}{2 \Delta E} \right) \tag{623}
$$

This probability (621) will include the scatterings in which a photon is emitted with energy below the limit of detection. The formula (617) is valid so long as

<a id="eq-624"></a>

$$
r \ll | \boldsymbol{p} | , | \boldsymbol{p}^{\prime} | , | \boldsymbol{q} |\ll \mu \tag{624}
$$

The probability for radiative scattering (i.e. scattering with emission of a detectable photon) is given by (546).

It can be proved that this removal of the infra-red divergence by taking into account the existence of unobservable photons is quite general, and works equally well when $q$ is not small. Only then the evaluation of (608) is considerably more unpleasant. Also, the same argument removes all infra-red divergences, also when higher-order radiative corrections are considered, in which case we are concerned with the effects of the emission of two or more soft photons during the scattering process. For a general discussion of this question see Bloch and Nordsieck, _Phys. Rev._ **52** (1937) 54.

---

## Notes

<a id="fn-68"></a>**68.** The first potential lacked a slash; $A /$ replaces $A$. [↩](#fnref-68)

<a id="fn-69"></a>**69.** The phrase “in which all emission operators stand to the left of all absorption operators” was lost in the transition from the first edition to the second. [↩](#fnref-69)

<a id="fn-70"></a>**70.** Nowadays called “normal order”, this ordering arises in connection with _Wick’s Theorem_: [↩](#fnref-70)

(time ordered operators) = (normal ordered operators) + (all contractions)

the contractions being equal to the propagators $S_{F} , D_{F}$, and so on.

<a id="fn-71"></a>**71.** In Schwinger’s QED anthology. [↩](#fnref-71)

<a id="fn-72"></a>**72.** In Schwinger’s anthology. [↩](#fnref-72)

<a id="fn-73"></a>**73.** Both the time ordering brackets lacked a right bracket. These were added. [↩](#fnref-73)

<a id="fn-74"></a>**74.** Unlike Dyson, Moravcsik cited Eq. (585) as well as Eq. (376). In Eq. (585) are Dirac matrix identities which establish the equality between the first two integrals in Eq. (522). Logically these identities should have been introduced before Chapter 6, but nothing prevents a reader making use of a “forward” reference. [↩](#fnref-74)

<a id="fn-75"></a>**75.** The subscript on the second $L$ was originally “$O$”. It has been replaced with a subscript “$D$”. [↩](#fnref-75)

<a id="fn-76"></a>**76.** Again, a subscript $\mu$ has been appended to the function $F ( k )$ to make it a Lorentz vector. See note 31 at Eq. (371). [↩](#fnref-76)

<a id="fn-77"></a>**77.** The last curly bracket was missing; it has been added. [↩](#fnref-77)

<a id="fn-78"></a>**78.** The times symbol $\times$ was inserted. [↩](#fnref-78)

<a id="fn-79"></a>**79.** This sentence read formerly “. . . the factor $\epsilon ( q - \epsilon - \epsilon^{\prime} )$ is a continuous function of ($\epsilon + \epsilon^{\prime} )$ and tends to $/\epsilon ( ( q ) )$ . . . ” [↩](#fnref-79)

<a id="fn-80"></a>**80.** A sentence, “Here, $( k \cdot p_{y} )_{3}$ denotes the scalar product of the space-like parts of the vectors $k$ and $p_{y}$.”, was deleted, because the expression $\boldsymbol{k} \cdot \boldsymbol{p}_{y}$ is self-explanatory. In both the first and second editions, scant attention was paid to three-vectors; sometimes an overhead arrow was used, but these were very few. In this typed version care has been taken to represent three-vectors with bold type, thus: $( A_{x} ,A_{y} ,A_{z} ) = \boldsymbol{A}$. [↩](#fnref-80)

<a id="fn-81"></a>**81.** “in” inserted; the original read “lying the range” [↩](#fnref-81)

---

## References

<a id="ref-19"></a>**[19]** F. J. Dyson, “Heisenberg operators in quantum electrodynamics”, _Phys. Rev._ **82** (1951) 428. Dyson introduces the term “normal product” on pp 429–430. [↩](#refback-19-1)

---

<a id="08-scattering-by-a-static-potential-comparison-with-experimental-results"></a>

# Scattering By A Static Potential. Comparison With Experimental Results.

<a id="p118"><sup>[p. 118]</sup></a>

Consider the scattering of an electron by a time-independent electrostatic potential

<a id="eq-625"></a>

$$
V ( r ) = \frac{1}{( 2 \pi )^{3}} \int d^{3} \boldsymbol{q} V ( \boldsymbol{q} ) e^{i \boldsymbol{q} \cdot \boldsymbol{r}} \tag{625}
$$

Then (543) gives the matrix element for scattering without radiative corrections (compare (625) and (534))

<a id="eq-626"></a>

$$
M_{0} = 2 \pi i \frac{e}{\hbar c} ( u^{\prime *} u ) V ( \boldsymbol{q} ) \delta ( q_{0} ) \tag{626}
$$

Non-radiative scattering occurs only between states for which

<a id="eq-627"></a>

$$
q_{0} = 0 , \quad | \boldsymbol{p} | = | \boldsymbol{p}^{\prime} | \tag{627}
$$

The cross-section for scattering between the states (542), per element of solid angle $d$Ω in the direction of $p^{\prime}$, is without radiative corrections

<a id="eq-628"></a>

$$
\sigma_{0} = \left( \frac{em}{2 \pi \hbar^{2}} \right)^{2} | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2} d \Omega \tag{628}
$$

This follows immediately from (626) using (627), using the prescription derived on page 29, when we treated the Møller scattering the first time. The procedure is as follows:<sup><a id="fnref-82" href="#fn-82">82</a></sup>

$$
w_{S} = \frac{c | M_{0} |^{2}}{2 \pi \delta ( q_{0} )} = 2 \pi \frac{e^{2}}{\hbar^{2} c} | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2} \delta ( q_{0} )
$$

$$
\rho dE = \frac{mc^{2}}{E} \frac{d^{3} \boldsymbol{p}}{( 2 \pi )^{3}} \quad EdE = \hbar^{2} c^{2} pdp \quad d^{3} \boldsymbol{p} = p^{2} dpd \Omega
$$

$$
\rho = \frac{mc^{2}}{E} \frac{p^{2}}{( 2 \pi )^{3}} \frac{dp}{dE} d \Omega = \frac{mp}{\hbar^{2} ( 2 \pi )^{3}} d \Omega \quad \delta ( q_{0} ) = \hbar c\delta ( E )
$$

$$
w = \frac{2 \pi e^{2}}{\hbar^{2} c} \hbar c \frac{mp}{\hbar^{2} ( 2 \pi )^{3}} d \Omega | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2} = \frac{e^{2} mp}{( 2 \pi )^{2} \hbar^{3}} d \Omega | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2}
$$

$$
\sigma = \frac{wV}{v} \quad V = \frac{mc^{2}}{E} \quad v = \frac{c^{2} \hbar p}{E}
$$

$$
\sigma = \frac{e^{2} mp}{( 2 \pi )^{2} \hbar^{3}} \frac{mc^{2}}{E} \frac{E}{c^{2} \hbar p} d \Omega | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2} = \left( \frac{em}{2 \pi \hbar^{2}} \right)^{2} d \Omega | u^{\prime *} u |^{2} | V ( \boldsymbol{q} ) |^{2}
$$

Summing over the final spin states, and averaging over the initial spin states, we have

<a id="eq-629"></a>

$$
\begin{aligned} \tfrac{1}{2} \sum_{u} \sum_{u^{\prime}} | u^{\prime *} u |^{2} &= \tfrac{1}{2} \frac{1}{( 2 i\mu )^{2}} \operatorname{Spur} \left\{ ( p / + i\mu ) \gamma_{4} ( /p^{\prime} + i\mu ) \gamma_{4} \right\} \\ &= \frac{1}{2 \mu^{2}} \left\{ \mu^{2} + p_{0} p_{0}^{\prime} + \boldsymbol{p} \cdot \boldsymbol{p}^{\prime} \right\} = \frac{1}{2 \mu^{2}} \left\{ 2 p_{0}^{2} - \tfrac{1}{2} | \boldsymbol{q} |^{2} \right\} \\ &= \frac{p_{o}^{2}}{\mu^{2}} \left\{ 1 - \tfrac{1}{4} \frac{| \boldsymbol{p} |^{2} - 2 \boldsymbol{p} \cdot \boldsymbol{p}^{\prime} + | \boldsymbol{p}^{\prime} |^{2}}{p_{0}^{2}} \right\} = \frac{p_{o}^{2}}{\mu^{2}} \left( 1 - \beta^{2} \sin^{2} \frac{\theta}{2} \right) \end{aligned} \tag{629}
$$

<a id="p119"><sup>[p. 119]</sup></a>

where $\theta$ is the angle between $p$ and $p^{\prime}$, and

<a id="eq-630"></a>

$$
\beta = \frac{| \boldsymbol{p} |}{p_{0}} = \frac{v}{c} \tag{630}
$$

where $v$ is the velocity of the incident electron. Hence the cross-section for an unpolarized electron beam is

<a id="eq-631"></a>

$$
\overline{\sigma}_{0} = \left( \frac{eE}{2 \pi \hbar^{2} c^{2}} \right)^{2} \left( 1 - \beta^{2} \sin^{2} \frac{\theta}{2} \right) | V ( \boldsymbol{q} ) |^{2} d \Omega \tag{631}
$$

where $E$ is the energy of the incident electron.

The second-order radiative correction to $M_{0}$ is given by (617), which in this case becomes

<a id="eq-632"></a>

$$
M_{2} + M_{2}^{\prime} = - \frac{\alpha}{3 \pi} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \frac{q^{2}}{\mu^{2}} M_{0} - \frac{\alpha}{2} \frac{e}{\hbar c\mu} ( u^{\prime *} /qu ) V ( \boldsymbol{q} ) \delta ( q_{0} ) \tag{632}
$$

This gives a second order correction to the cross-section $\sigma_{0}$ according to (621). The total non-radiative cross-section, for <a id="ix-217">scattering</a> without the emission of a photon of energy greater than ∆$E$, becomes

<a id="eq-633"></a>

$$
\begin{aligned} \sigma_{N} &= \sigma_{0} + \sigma_{2 N} \\ &= \left( \frac{em}{2 \pi \hbar^{2}} \right)^{2} | V ( \boldsymbol{q} ) |^{2} d \Omega \left| \left\{ 1 - \frac{\alpha}{3 \pi} \left( \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right) \frac{q^{2}}{\mu^{2}} \right\} ( u^{\prime *} u ) + \frac{i\alpha}{4 \pi \mu} ( u^{\prime *} /qu ) \right|^{2} \end{aligned} \tag{633}
$$

Summing and averaging over the spin-states

<a id="eq-634"></a>

$$
\begin{aligned} \tfrac{1}{2} \sum_{u} \sum_{u^{\prime}} ( u^{\prime *} /qu ) ( u^{*} u^{\prime} ) &= \frac{1}{2 ( 2 i\mu )^{2}} \operatorname{Spur} \left\{ ( p / + i\mu ) \gamma_{4} ( p /^{\prime} + i\mu ) \gamma_{4} /q \right\} \\ &= - \frac{1}{8 \mu^{2}} \operatorname{Spur} \left\{ i\mu \left( /p\gamma_{4} \gamma_{4} /q + \gamma_{4} p /^{\prime} \gamma_{4} /q \right) \right\} = - \frac{1}{8 \mu^{2}} \operatorname{Spur} \left\{ i\mu ( p/p /^{\prime} - p/p / - p /^{\prime} p /^{\prime} + p /^{\prime} p / ) \right\} \\ &= - \frac{1}{8 \mu^{2}} \operatorname{Spur} \left\{ i\mu ( p /^{\prime} - p / ) ( p /^{\prime} - p / ) \right\} = \frac{i}{2 \mu} q^{2} \end{aligned} \tag{634}
$$

Hence for an unpolarized electron beam the non-radiative cross-section is

<a id="eq-635"></a>

$$
\overline{\sigma}_{N} = \left( 1 - \frac{2 \alpha}{3 \pi} \left( \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right) \frac{q^{2}}{\mu^{2}} \right) \overline{\sigma}_{0} - \left( \frac{em}{2 \pi \hbar^{2}} \right)^{2} | V ( \boldsymbol{q} ) |^{2} d \Omega \frac{\alpha}{4 \pi} \frac{q^{2}}{\mu^{2}} \tag{635}
$$

Since we are working only to order $q^{2}$ in the radiative corrections, the second term in (635) may be replaced by

<a id="eq-636"></a>

$$
- \frac{\alpha}{4 \pi} \frac{q^{2}}{\mu^{2}} \overline{\sigma}_{0} \tag{636}
$$

and then we find

<a id="eq-637"></a>

$$
\sigma_{N} = \left( 1 - \frac{2 \alpha}{3 \pi} \left( \log \frac{mc^{2}}{2 \Delta E} + \tfrac{5}{6} - \tfrac{1}{5} \right) \frac{q^{2}}{\mu^{2}} \right) \overline{\sigma}_{0} \tag{637}
$$

Formulae (628) and (631) are exact for electrons of any energy; while (632) and (637) are valid only for slow electrons, terms of order higher than $\alpha q^{2}$ being neglected.

To remove the dependence of (637) on $r$ we must consider the cross-section for scattering with emission of a photon of frequency greater than $r$. Since the electron is now considered to be slow, the maximum possible energy of the photon is

<a id="eq-638"></a>

$$
\hbar ck_{\max} = E - mc^{2} \approx \frac{\hbar^{2}}{2 m} | \boldsymbol{p} |^{2} \tag{638}
$$

and therefore for all possible photons the momentum $\hbar | \boldsymbol{k}^{\prime} |$ will be very small compared with the electron momentum $\hbar | \boldsymbol{p} |$.<sup><a id="fnref-83" href="#fn-83">83</a></sup> Thus the recoil of the electron, arising from the momentum carried off by the photon, <a id="p120"><sup>[p. 120]</sup></a> can always be neglected. The matrix element and probability for radiative scattering will be given by (618) and (620), even when the photon takes away a large fraction of the kinetic energy of the electron.

We imagine a scattering experiment in which only the direction of the emerging electron is measured and not its energy. Then the radiative cross-section $\overline{\sigma}_{R}$ will measure the total probability of scattering the electron into a <a id="ix-26">solid</a> angle $d$Ω with emission of a photon having any frequency between the lower limit $r$ and the upper limit $k_{\max}$ given by (638). The observed cross-section will be

<a id="eq-639"></a>

$$
\sigma_{T} = \overline{\sigma}_{N} + \overline{\sigma}_{R} \tag{639}
$$

with the same low-frequency cut-off $r$ in both $\overline{\sigma}_{N}$ and $\overline{\sigma}_{R}$. Thus $\sigma_{T}$ gives the cross-section for scattering into a given solid angle $d$Ω with or without photon emission. Being directly observed, $\sigma_{T}$ must be divergence-free and independent of $r$.

In the radiative scattering process we may take the final momentum of the electron to be $\lambda \hbar p^{\prime}$ where 0 $<\lambda <$ 1 and $p^{\prime}$ satisfies (627). Instead of (627) the conservation of energy now gives by (638)

<a id="eq-640"></a>

$$
\hbar | \boldsymbol{p} |^{2} ( 1 - \lambda^{2} ) = 2 mc | \boldsymbol{k}^{\prime} | \tag{640}
$$

According to (620), the probability for scattering the electron into a state $\lambda \hbar p^{\prime}$, with emission of a photon in any direction with frequency in the range $( k^{\prime} ,k^{\prime} + dk^{\prime} )$ is

<a id="eq-641"></a>

$$
w_{R} ( k^{\prime} ) = \frac{2 \alpha}{3 \pi} \frac{dk^{\prime}}{k^{\prime}} \frac{| \boldsymbol{p} - \lambda \boldsymbol{p}^{\prime} |^{2}}{\mu^{2}} | M_{0} |^{2} \tag{641}
$$

This corresponds to a differential cross-section

<a id="eq-642"></a>

$$
\sigma_{R} ( k^{\prime} ) = \frac{2 \alpha}{3 \pi} \frac{dk^{\prime}}{k^{\prime}} \frac{| \boldsymbol{p} - \lambda \boldsymbol{p}^{\prime} |^{2}}{\mu^{2}} \lambda \left( \frac{eE}{2 \pi \hbar^{2} c^{2}} \right)^{2} | V ( p - \lambda p^{\prime} ) |^{2} d \Omega \tag{642}
$$

for scattering into the solid angle $d$Ω, neglecting now the term in $\beta$ which appeared in (631) since (642) is itself of order $\alpha \beta^{2}$ and higher terms are neglected. The factor $\lambda$ comes in from $\frac{p_{\text{ final }}}{p^{\prime}}$ . According to (640)

<a id="eq-643"></a>

$$
\frac{dk^{\prime}}{k^{\prime}} = - \frac{2 \lambda d\lambda}{1 - \lambda^{2}} \tag{643}
$$

Hence the radiative cross-section integrated over the quantum frequency is

<a id="eq-644"></a>

$$
\overline{\sigma}_{R} = \frac{2 \alpha}{3 \pi} \int_{0}^{\lambda_{m}} \frac{2 \lambda^{2} d\lambda}{1 - \lambda^{2}} \frac{| \boldsymbol{p} - \lambda \boldsymbol{p}^{\prime} |^{2}}{\mu^{2}} \left( \frac{eE}{2 \pi \hbar^{2} c^{2}} \right)^{2} | V ( p - \lambda p^{\prime} ) |^{2} d \Omega \tag{644}
$$

where by (640) and (638)

<a id="eq-645"></a>

$$
\lambda_{m} = \sqrt{1 - \frac{r}{k_{\max}}} = \sqrt{1 - \frac{\Delta E}{T}} \tag{645}
$$

$T$ is the initial kinetic energy of the electron given by (638).

Now we can combine (637) and (644) to give by (639) and (629)

<a id="eq-646"></a>

$$
\begin{aligned} \sigma_{T} &= \left( 1 - \frac{2 \alpha}{3 \pi} \left( \log \frac{mc^{2}}{2 T} + \tfrac{5}{6} - \tfrac{1}{5} \right) 4 \beta^{2} \sin^{2} \frac{\theta}{2} \right) \overline{\sigma}_{0} \\ & \quad + \frac{2 \alpha}{3 \pi} \left( \frac{e}{2 \pi \hbar c} \right)^{2} d \Omega \times \int_{0}^{1} \frac{2 \lambda d\lambda}{1 - \lambda^{2}} \left\{ \lambda | \boldsymbol{p} - \lambda \boldsymbol{p}^{\prime} |^{2} | V ( p - \lambda p^{\prime} ) |^{2} -| \boldsymbol{p} - \boldsymbol{p}^{\prime} |^{2} | V ( p - p^{\prime} ) |^{2} \right\} \end{aligned} \tag{646}
$$

Here we used the following trick: the integral over $\lambda$ blows up at $\lambda$ = 1. Therefore we subtract from the numerator its value at $\lambda$ = 1, which makes the integral behave decently and permits us to change the upper limit from $\lambda_{m}$ to 1, for small ∆$E$’s. We also have to add then the integrand with the numerator having $\lambda$ = 1; this gives then a logarithmic term which combines with (637) to give the first part of (646).

<a id="p121"><sup>[p. 121]</sup></a>

Formula (646) gives a result of the form

<a id="eq-647"></a>

$$
\sigma_{T} = \left( 1 - \frac{8 \alpha}{3 \pi} \beta^{2} \sin^{2} \frac{\theta}{2} \left\{ \log \frac{mc^{2}}{2 T} + f ( \theta ) \right\} \right) \overline{\sigma}_{0} \tag{647}
$$

where for low velocities $f ( \theta )$ is independent of $T$ and of order 1 compared to the logarithm. For any special potential $f ( \theta )$ can be calculated.

From (647) we see that the observable radiative correction is not of order $\alpha$ but of order

<a id="eq-648"></a>

$$
\alpha \left( \frac{v}{c} \right)^{2} \log \left( \frac{c}{v} \right) \tag{648}
$$

which is much smaller if $v$ is not relativistic. Thus the correction _cannot be observed at all_ in a non-relativistic scattering experiment. In the relativistic region the effect is actually of order $\alpha$ as indicated by (647), but the correct formula is then enormously more complicated.

The exact formulae in both non-relativistic and relativistic cases have been published by J. Schwinger, _Phys. Rev._ **76** (1949) 790.

An experimental test in the relativistic range is just on the limits of the possible. See Lyman, Hanson and Scott, _Phys. Rev._ **84** (1951) 626. Scattering of 15 MeV electrons by nuclei were measured with very good energy-resolution, ∆$E/E$ being 1 - 3 %. In this case it was _only_ the <a id="ix-37">non-radiative</a> cross-section $\overline{\sigma}_{N}$ that was observed, and so the radiative correction given by the relativistic form of (637) becomes quite large. In fact the radiative correction in $\overline{\sigma}_{N}$ in the relativistic range becomes of the order

<a id="eq-649"></a>

$$
\alpha \left\{ \log \frac{\Delta E}{E} \right\} \left\{ \log \frac{E}{mc^{2}} \right\} \tag{649}
$$

according to Schwinger, _Phys. Rev._ **76**, 813, Eq. (2.105) (with misprint $K$ for $k$, in my notation $\mu$.) In the conditions of the experiment of Lyman-Hanson-Scott (649) becomes of the order 5% and was clearly observed, the experimental errors being $\sim$ 2%. However, (649) arises mainly from the low-energy virtual photons, with energies going down to ∆$E$. What is observed is just the decrease in the non-radiative cross-section due to the competition of radiative scattering with energy-loss in the range [∆$E,E ]$. Thus the measurement of (649) by LHS is actually only a very inaccurate measurement of the cross-section for bremsstrahlung, which could be observed much more accurately by observing the photons which are really emitted.

The theoretically interesting part of the radiative corrections is the part which is not just the effect of real bremsstrahlung. This part is given by the terms in $\overline{\sigma}_{N}$ which are of order 1 compared with the log(∆ ) appearing in (649). For example, we would have to observe with sufficient accuracy to see the terms $( {}^{5} \frac{E/E}{6} - \tfrac{1}{5} )$ in (637) if we wished to verify the theoretical radiative corrections at low velocities. In the relativistic range the “genuine” radiative corrections are of order

<a id="eq-650"></a>

$$
\alpha \log \left( \frac{E}{mc^{2}} \right) \tag{650}
$$

instead of (649), i.e. about 2% in the LHS experiment. To detect such effects is already possible, but to observe them accurately in a scattering experiment seems not very hopeful.

This is all we can say at present about radiative corrections to scattering by an electrostatic potential.

## <a id="s-a-the-magnetic-moment-of-the-electron"></a>A. The Magnetic Moment of the Electron

The scattering by an electrostatic potential, the two terms in (617) were lumped together. Both gave contributions of the same order of magnitude, $\alpha ( q^{2} /\mu^{2} )$ in the cross section. What then is the meaning of the special form of the second term in (617)? This term has no infra-red divergence and therefore should be particularly simple to interpret experimentally.

<a id="p122"><sup>[p. 122]</sup></a>

Consider scattering of a slow electron by a slowly-varying<sup><a id="fnref-84" href="#fn-84">84</a></sup> magnetic field. The potentials (534) can then be taken to be a pure vector potential, so that

<a id="eq-651"></a>

$$
e_{4} ( q ) = 0 \tag{651}
$$

The matrix elements of $\gamma_{1} ,\gamma_{2} ,\gamma_{3}$ between positive energy electron states are of the order $( v/c )$. Hence $M_{0}$ given by (543) is of order $( v/c )$. The first term in (617) is thus of the order $\alpha ( v/c )$<sup>3</sup> while the second is of order $\alpha ( v/c )$. Therefore the second term in (617) is the main term in considering magnetic effects, and the first term can be ignored. The meaning of the second term must be a change in the _magnetic properties_ of a non-relativistic electron.

As we saw in discussing the Dirac equation, (Eq’s (99) and (100)), an electron by virtue of its charge $( - e )$ behaves in non-relativistic approximation as if it had a magnetic moment

<a id="eq-652"></a>

$$
M = - \frac{e \hbar}{2 mc} \tag{652}
$$

This moment has an energy of interaction with an external <a id="ix-195">Maxwell</a> field $( \boldsymbol{E} , \boldsymbol{H} )$ given by

<a id="eq-653"></a>

$$
H_{M} = - M ( \boldsymbol{\sigma} \cdot \boldsymbol{H} - i \boldsymbol{\alpha} \cdot \boldsymbol{E} ) \tag{653}
$$

the term which appears in the non-relativistic Schrödinger equation (100).

Now suppose that the electron possesses an additional magnetic moment $\delta M$ which does not arise from its charge. Such an additional moment is called “anomalous”. To give the electron an anomalous moment, we only need to add arbitrarily a term proportional to (653) to the Hamiltonian. Comparing (654) with (97) and (98), we see that (653) is a relativistic invariant and can be written

<a id="eq-654"></a>

$$
H_{M} = \tfrac{1}{2} iM \sum_{\mu} \sum_{\nu} \sigma_{\mu \nu} F_{\mu \nu} \tag{654}
$$

Hence an anomalous magnetic moment $\delta M$ will be given to the electron if the term

<a id="eq-655"></a>

$$
L_{M} = - \tfrac{1}{2} i\delta M \sum_{\mu} \sum_{\nu} \sigma_{\mu \nu} F_{\mu \nu} \tag{655}
$$

is added to the Lagrangian. This refers still to the one-electron Dirac equation. In the theory of the quantized Dirac field, the corresponding addition to the Lagrangian density (410) is

<a id="eq-656"></a>

$$
L_{M} = - \tfrac{1}{2} i\delta M \overline{\psi} \sum_{\mu} \sum_{\nu} \sigma_{\mu \nu} \psi F_{\mu \nu}^{e} \tag{656}
$$

where it is supposed that the anomalous moment is interacting with the external Maxwell field. The addition (656) to the Lagrangian<sup><a id="fnref-85" href="#fn-85">85</a></sup> gives a relativistically invariant description of an anomalous moment.

Consider the effect of (656) on the scattering of an electron by the potentials (534). Treating the scattering in Born approximation and using (420), the contribution of (656) to the scattering matrix element is

<a id="eq-657"></a>

$$
U_{M} = \sum_{\mu ,\nu} \frac{\delta M}{2 \hbar c} \int \overline{\psi} ( x ) \sigma_{\mu \nu} \psi ( x ) F_{\mu \nu}^{e} ( x ) dx \tag{657}
$$

the integral being over all space-time. Using (542) for the initial and final electron wave-functions, and defining $q , e$ by (544) and (545), this matrix element becomes

<a id="eq-658"></a>

$$
U_{M} = i \frac{\delta M}{2 \hbar c} \sum_{\mu ,\nu} ( \overline{u}^{\prime} \sigma_{\mu \nu} u ) ( q_{\mu} e_{\nu} - q_{\nu} e_{\mu} ) = i \frac{\delta M}{2 \hbar c} \left[ u^{\prime} ( /q/e - /e/q ) u \right] \tag{658}
$$

where we used $\gamma_{k} \gamma_{\ell} = i\sigma_{m} ,k,\ell ,m$ = (1$, 2 ,$3) cyclically permuted. Since we have also assumed (588) we may write simply

<a id="eq-659"></a>

$$
U_{M} = - i \frac{\delta M}{\hbar c} ( \overline{u}^{\prime} /e/qu ) \tag{659}
$$

<a id="p123"><sup>[p. 123]</sup></a>

Now comparing the matrix element (659) with (617), we see that the magnetic effect of the second-order radiative correction to scattering is exactly described by saying that the electron has an <a id="ix-98">anomalous magnetic moment</a> $\delta M$ given by

<a id="eq-660"></a>

$$
\delta M = - \frac{\alpha}{4 \pi} \frac{e \hbar}{mc} = + \frac{\alpha}{2 \pi} M \tag{660}
$$

This is the famous Schwinger correction to the electron magnetic moment, which we have now calculated. Not only for scattering but for all phenomena in the non-relativistic range, the magnetic part of the second-order radiative correction to the motion of an electron is equivalent simply to the anomalous magnetic moment (660).

This anomalous moment has been extremely accurately confirmed experimentally by<sup><a id="fnref-86" href="#fn-86">86</a></sup> Kusch, Prodell and Koenig (_Phys. Rev._ **83** (1951) 687) who find

$$
\frac{\delta M}{M} = 0 . 0 0 1 1 4 5 \pm 0 . 0 0 0 0 1 3
$$

The calculated value including a fourth-order $\alpha^{2}$ correction found by Karplus and Kroll (_Phys. Rev._ **77** (1950) 536) is

$$
\frac{\delta M}{M} = \frac{\alpha}{2 \pi} - 2 . 9 7 3 \left( \frac{\alpha^{2}}{\pi^{2}} \right) = 0 . 0 0 1 1 4 5 4
$$

## <a id="s-b-relativistic-calculation-of-the-lamb-shift"></a>B. Relativistic Calculation of the Lamb Shift

To make a correct relativistic calculation of the Lamb shift we have to repeat the treatment of line-shifts and line-widths which we gave earlier, only now using the relativistic theory for the atom. So we should set up the equation of motion of atom plus radiation field in the _Bound Interaction Representation_. The equation of motion is then given by (245), (247), only with the $j_{\mu}$ operator now describing the system of a relativistic atom. The solution of (245) can then be found as in the non-relativistic case, using the known wave-functions of the stationary states of the atom. In this way the Lamb shift was actually calculated by Lamb and Kroll<sup><a id="fnref-87" href="#fn-87">87</a></sup>, _Phys. Rev._ **75** (1949) 388. However, in their calculations Lamb and Kroll had troubles with the subtraction of the divergent mass-renormalization effect. Because the calculation was all in terms of the atomic wave-functions, it was not possible to keep using relativistically invariant notations all the way through. Hence the mass term could not be clearly separated from the remaining finite terms by its dependence on the particle momentum $p$, as it was separated for example in Eq. (566) during the calculation of radiative corrections to scattering. The final result of Lamb and Kroll was uncertain because of this difficulty in the mass separation. They obtained the correct answer 1052 Mc but only by making use of the experimentally _measured_ value $\alpha / 2 \pi$ of the electron anomalous magnetic moment.<sup><a id="fnref-88" href="#fn-88">88</a></sup>

Learning from the calculation of the radiative corrections to scattering, we see that to make a clear separation of the mass renormalization from observable effects, we must arrange the calculations so that the separation is done for a particle of momentum $p$ in a _variable_ Lorentz system. Then by varying the Lorentz system we can vary $p$ and identify the mass term unambiguously as the expression which has the correct dependence on $p$. To work in a variable Lorentz system, we must work in a representation which is independent of the Lorentz system, so that the calculations are formally invariant. The only convenient invariant representation is the _Free Interaction Representation_.

Hence our program is to set up the equation of motion first in the _Free Interaction Representation_, then carry through a relativistically invariant calculation to identify and cancel the divergent renormalization effects unambiguously. After this we must transform to the _Bound Interaction Representation_ for the final calculation of the line-shift. This two-stage calculation is absolutely necessary in order to get the right answers. It was Schwinger’s invention.

In the FIR the equation of motion is

<a id="eq-661"></a>

$$
i \hbar \frac{\partial \Psi}{\partial t} = \left( H^{e} ( t ) + H^{I} ( t ) \right) \Psi \tag{661}
$$

<a id="p124"><sup>[p. 124]</sup></a>

with $H^{I}$ given by (532) including the mass-renormalization term $H_{S}$. The first stage in the solution of (661) is to write

<a id="eq-662"></a>

$$
\Psi ( t ) = \Omega_{1} \Phi ( t ) \tag{662}
$$

where Ω${}_{1} ( t )$ is defined as in (492), only with $e \overline{\psi} / A\psi$ replaced by $[ e \overline{\psi} / A\psi + i\delta mc^{2} \overline{\psi} \psi ]$, and the function $g_{A} ( t )$ is as before supposed to tend to the limiting form $g_{A} ( t ) \to$ 1 at the end of the calculation. The operator Ω${}_{1} ( t )$ satisfies

<a id="eq-663"></a>

$$
i \hbar \frac{\partial \Omega_{1} ( t )}{\partial t} = H^{I} ( t ) \Omega_{1} ( t ) \tag{663}
$$

for any value of $t$ not in the remote past, so that we may take $g_{A} ( t )$ = 1. Hence the <a id="ix-234">equation</a> of motion for Φ$( t )$ derived from (661) is just

<a id="eq-664"></a>

$$
i \hbar \frac{\partial \Phi}{\partial t} = H_{T} ( t ) \Phi \tag{664}
$$

<a id="eq-665"></a>

$$
H_{T} ( t ) = ( \Omega_{1} ( t ) )^{- 1} H^{e} ( t ) \Omega_{1} ( t ) \tag{665}
$$

The covariant part of the calculation, which is done in the FIR, is just the evaluation of this transformed <a id="ix-104">Hamiltonian</a> $H_{T} ( t )$.

### <a id="s-covariant-part-of-the-calculation"></a>Covariant Part of the Calculation

Let us write

<a id="eq-666"></a>

$$
H_{F} ( t ) = \Omega_{2} ( t ) H^{e} ( t ) \Omega_{1} ( t ) \tag{666}
$$

with Ω$2$ given by (490). Then

<a id="eq-667"></a>

$$
H_{T} ( t ) = ( \Omega_{2} ( t ) \Omega_{1} ( t ) )^{- 1} H_{F} ( t ) = S^{- 1} H_{F} ( t ) \tag{667}
$$

where $S$, given by (495), is the scattering matrix defined in the absence of the external potential $A_{\mu}^{e}$. We shall now restrict our attention to systems in which only one electron is actually present. We saw by Eq. (533) that $S$ applied to a one-electron state is equivalent to the unit operator, i. e.g. $S$ produces no real scattering or shift of phase in one-particle states. Therefore in discussing the hydrogen atom we may simply omit the $S^{- 1}$ in (667) and write

<a id="eq-668"></a>

$$
H_{T} ( t ) = H_{F} ( t ) \tag{668}
$$

Now $H_{F} ( t )$ is just the term involving $H^{e} ( t )$ in the series expansion (421). In fact we have, by direct multiplication of the series for Ω$1$ and Ω$2$,

<a id="eq-669"></a>

$$
H_{F} ( t ) = \sum_{n = 0}^{\infty} \left( \frac{- i}{\hbar} \right)^{n} \frac{1}{n !} \int \dots \int dt_{1} dt_{2} \dots dt_{n} P \left\{ H^{e} ( t ) ,H^{I} ( t_{1} ) ,\dots ,H^{I} ( t_{n} ) \right\} \tag{669}
$$

the damping functions $g ( t_{i} )$ being always understood when they are not written explicitly. Now (667) is directly related to the operator $U$ given by (535), namely

<a id="eq-670"></a>

$$
U = - \frac{i}{\hbar} \int_{-\infty}^{\infty} H_{F} ( t ) dt \tag{670}
$$

We write

<a id="eq-671"></a>

$$
H_{T} ( t ) = H^{e} ( t ) + H_{T 1} ( t ) + H_{T 2} ( t ) \tag{671}
$$

expanding $H_{T}$ in powers of the radiation interaction just as $U$ was expanded in (536)–(540). But the matrix elements of $\left( U_{2} + U_{2}^{1} \right)$ referring to one-electron transition have already been calculated and are given by (617) assuming non-relativistic velocities for the electron. This enables us to write down immediately a <a id="p125"><sup>[p. 125]</sup></a> formula for the operator $H_{T 2}$, valid for one-electron transitions at non-relativistic velocities. In (617) each factor $q_{\lambda}$ may be replaced by $( - i\partial /\partial x_{\lambda} )$ operating on the potentials (534). Then (617) becomes

<a id="eq-672"></a>

$$
\begin{aligned} U_{2} + U_{2}^{\prime} &= \frac{1}{\mu^{2}} \frac{\alpha}{3 \pi} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \left( \frac{e}{\hbar c} \right) \int \overline{\psi} ( \Box^{2} A / ) \psi ( x ) dx \\ & \quad + \frac{\alpha}{4 \pi} \frac{e}{mc^{2}} \int \overline{\psi} \sum_{\lambda} \frac{\partial / A}{\partial x_{\lambda}} \gamma_{\lambda} \psi ( x ) dx \end{aligned} \tag{672}
$$

And using (670) and (668) we deduce

<a id="eq-673"></a>

$$
\begin{aligned} H_{T 2} &= \frac{ie}{\mu^{2}} \frac{\alpha}{3 \pi} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \int \overline{\psi} ( \Box^{2} A / ) \psi ( x ) d^{3} \boldsymbol{x} \\ & \quad + \frac{\alpha}{4 \pi} \frac{ie \hbar}{mc} \int \overline{\psi} \sum_{\lambda} \frac{\partial / A}{\partial x_{\lambda}} \gamma_{\lambda} \psi ( x ) d^{3} \boldsymbol{x} \end{aligned} \tag{673}
$$

Specializing now to the case of a time-independent electrostatic potential given by

<a id="eq-674"></a>

$$
A_{4} = i\phi ( r ) , \quad V = - e\phi , \tag{674}
$$

<a id="eq-675"></a>

$$
H^{e} ( t ) = \int V ( r ) ( \psi^{*} \psi ) d^{3} \boldsymbol{r} , \tag{675}
$$

we find<sup><a id="fnref-89" href="#fn-89">89</a></sup>

<a id="eq-676"></a>

$$
\begin{aligned} H_{T 2} &= \frac{\alpha}{3 \pi \mu^{2}} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \int ( \nabla^{2} V ) ( \psi^{*} \psi ) d^{3} \boldsymbol{r} \\ & \quad - \frac{i\alpha}{4 \pi \mu} \int \psi^{*} \gamma_{4} ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla V ) \psi d^{3} \boldsymbol{r} \end{aligned} \tag{676}
$$

The calculation of $H_{T 2}$, which is the main part of the <a id="ix-146">Lamb</a> shift calculation, can thus be taken over directly from the scattering calculation. In particular, the elimination of divergent renormalization effects does not have to be considered afresh. Once (676) has been derived, everything is finite and we are permitted to carry out the rest of the calculation in a non-covariant way. Note however that the low-energy photon frequency cut-off $r$ still appears in (676). We expect that this dependence on $r$ will finally disappear when the effects of $H_{T 1}$ are considered, the same cut-off being used in both $H_{T 1}$ and $H_{T 2}$.

We next turn to the evaluation of $H_{T 1}$, which is related to $U_{1}$ in the same way as $H_{T 2}$ to $( U_{2} + U_{2}^{1} )$. According to (484) the matrix element of $U_{1}$ for a one-electron transition between the states (542), with emission of the photon (440), is<sup><a id="fnref-90" href="#fn-90">90</a></sup>

<a id="eq-676a"></a>

$$
M_{1} = - \frac{e^{2}}{\hbar^{2} c^{2}} \overline{u}^{\prime} \left\{ /e \frac{1}{p / - / k^{\prime} - i\mu} /e^{\prime} + /e^{\prime} \frac{1}{p /^{\prime} + k /^{\prime} - i\mu} /e \right\} u \tag{676a}
$$

where now

<a id="eq-677"></a>

$$
e_{\mu} = e_{\mu} ( p^{\prime} + k^{\prime} - p ) \tag{677}
$$

is given by the Fourier expansion (534). Note that we cannot yet use the simple form (546) for $M_{1}$, because we do not know that $k^{\prime} \ll p,q$ for the photons which will be important in this problem. Using the Dirac equation satisfied by $u$ and $u^{\prime}$, we may write without approximations using (587) and rule 4 on page 87,

<a id="eq-677a"></a>

$$
M_{1} = - \frac{e^{2}}{2 \hbar^{2} c^{2}} \overline{u}^{\prime} \left\{ \frac{/e/ k^{\prime} /e^{\prime} - 2 ( p \cdot e^{\prime} ) /e}{p \cdot k^{\prime}} + \frac{/e^{\prime} / k^{\prime} /e + 2 ( p^{\prime} \cdot e^{\prime} ) /e}{p^{\prime} \cdot k^{\prime}} \right\} u \tag{677a}
$$

Since $p$ and $p^{\prime}$ are assumed non-relativistic we may write

$$
p \cdot k^{\prime} = p^{\prime} \cdot k^{\prime} = - \mu k_{0}^{\prime}
$$

<a id="p126"><sup>[p. 126]</sup></a>

and then

<a id="eq-678"></a>

$$
M_{1} = \frac{e^{2}}{2 \hbar^{2} c^{2} \mu k_{0}^{\prime}} \overline{u}^{\prime} \left\{ 2 ( ( p^{\prime} - p ) \cdot e^{\prime} ) /e + /e^{\prime} / k^{\prime} /e + /e/ k^{\prime} /e^{\prime} \right\} u \tag{678}
$$

Now because we shall be considering only an electrostatic potential (674), $/e$ is a multiple of $\gamma_{4}$ simply. Then if $k_{3}^{\prime}$ is the space-like part of the vector $k^{\prime}$, we have

$$
/e^{\prime} / k_{3}^{\prime} /e + /e/ k_{3}^{\prime} /e^{\prime} = /e \left\{ /e^{\prime} / k_{3}^{\prime} + k /_{3}^{\prime} /e^{\prime} \right\} = 0
$$

Therefore

<a id="eq-679"></a>

$$
/e^{\prime} / k_{3}^{\prime} /e + /e/ k_{3}^{\prime} /e^{\prime} = ik_{0}^{\prime} /e^{\prime} ( 2 \gamma_{4} /e ) \tag{679}
$$

Now this term (679) is small compared to the other term in (678), because $/e^{\prime}$ involves the matrices $\gamma_{1} ,\gamma_{2} ,\gamma_{3}$ while $/e$ involves $\gamma_{4}$, and the matrix elements of $\gamma_{1} ,\gamma_{2} ,\gamma_{3}$ for non-relativistic transitions are small, of the order of $( v/c )$. The term (679) in fact describes magnetic radiation, whereas the other term in (678) gives electric. The electric term gives an effect of the order of the <a id="ix-124">Lamb shift</a>. Hence in our approximation we may neglect the magnetic term and write

<a id="eq-680"></a>

$$
M_{1} = \frac{e^{2}}{\hbar^{2} c^{2} \mu k_{0}^{\prime}} ( ( p^{\prime} - p ) \cdot e^{\prime} ) ( \overline{u}^{\prime} /eu ) \tag{680}
$$

which is the same result as we should have obtained from (546).

Let $Z_{A} ( x )$ be the Hertzian vector corresponding to the radiation field potentials $A_{\lambda} ( x )$, defined by

<a id="eq-681"></a>

$$
A_{\lambda} ( x ) = \frac{d}{dt} Z_{\lambda} ( x ) \tag{681}
$$

Then the matrix element of $Z_{\lambda} ( x )$ for emitting the photon with potentials (440) is

<a id="eq-682"></a>

$$
Z_{\lambda} ( x ) = \frac{1}{ick_{0}^{\prime}} e_{\lambda}^{\prime} e^{- ik^{\prime} \cdot x} \tag{682}
$$

Compare with (422)<sup><a id="fnref-91" href="#fn-91">91</a></sup>.

Thus the operator $U_{1}$ which has the matrix element (680) may be written<sup><a id="fnref-92" href="#fn-92">92</a></sup>

<a id="eq-683"></a>

$$
U_{1} = \frac{e^{2}}{\hbar^{2} c\mu} \int dx \overline{\psi} ( Z \cdot \partial ) A /^{e} \psi ( x ) \tag{683}
$$

Using (670) and specializing by means of (674), this gives

<a id="eq-684"></a>

$$
H_{T 1} = \frac{e}{\hbar \mu} \int \psi^{*} ( Z \cdot \partial V ) \psi d^{3} \boldsymbol{r} \tag{684}
$$

This completes the evaluation of $H_{T}$.

### <a id="s-discussion-of-the-nature-of-the-representation"></a>Discussion and the Nature of the Φ-Representation

To understand the effect of the <a id="ix-110">transformation</a> (662), we observe that if Ψ$( t )$ is the state of one real electron in the absence of an external field, then Φ$( t )$ will be independent of $t$ and will represent one “bare” electron with the same momentum as the real electron. In an actual hydrogen atom, we may consider the state Ψ$( t )$ to a very good approximation as a superposition of states of a single real free electron; then Φ($t )$ is a superposition of states of a single bare electron with the same distribution of momenta. Thus by the transformation from Ψ to Φ we have eliminated the radiation field surrounding the electron, all remaining effects of this radiation field being contained in the operator $H_{T}$.

It is essential at this point to make sure that in the Φ representation the field-operators are still free-particle operators, with the correct equations of motion for operators in the FIR. Thus the transformation <a id="p127"><sup>[p. 127]</sup></a> (662) is only a transformation from one set of variables to another within the FIR, and does not take us out of the FIR. This point was never explained properly by Schwinger in his papers, although he no doubt understood it himself.

Let then $Q ( x )$ be a field-operator of the Ψ representation. Being a FIR operator, $Q ( x )$ satisfies

<a id="eq-685"></a>

$$
i \hbar \frac{dQ}{dt} = \boldsymbol{[} Q,H_{0} \boldsymbol{]} \tag{685}
$$

where $H_{0}$ is the Hamiltonian of the Dirac and Maxwell fields without interaction. In the Φ representation the corresponding field-operator is

<a id="eq-686"></a>

$$
Q^{\prime} ( x ) = ( \Omega_{1} ( t ) )^{- 1} Q ( x ) \Omega_{1} ( t ) \tag{686}
$$

Now Ω${}_{1} ( t )$ is given by (492) where we now take the limit $g_{A}$ = 1. The operators appearing in (492) are all FIR operators satisfying equations of motion of the form (685). When the integrations in (492) are carried out, the integrated terms will still have the same time variation given by (685), except for those terms which correspond to transitions in which energy is conserved. The energy-conserving matrix elements will have an explicit linear dependence on $t$ after integration, which is not in accordance with (685). Therefore we conclude that the equation of motion

<a id="eq-687"></a>

$$
i \hbar \frac{d \Omega_{1}}{dt} = \boldsymbol{[} \Omega_{1} ,H_{0} \boldsymbol{]} \tag{687}
$$

is valid for all matrix elements of Ω$1$ which are not diagonal in the FIR. The same equation of motion is satisfied by (Ω${}_{1} ( t ) )^{- 1}$ with the same condition. Now we have seen that Ω${}_{1} ( t )$ has no matrix elements diagonal in $H_{0}$ which give transitions either from or into one-particle states. Hence (687) holds for all matrix elements in which either initial or final state is a one-particle state.

Combining (687) and the corresponding equation for (Ω${}_{1} )^{- 1}$ with (685) and (686), we have

<a id="eq-688"></a>

$$
i \hbar \frac{dQ^{\prime}}{dt} = \boldsymbol{[} Q^{\prime} ,H_{0} \boldsymbol{]} \tag{688}
$$

This equation (688) is valid for all matrix elements between one-particle states. Thus we can conclude that so long as we confine attention to a one-electron system, (688) is satisfied by all field operators of the Φ representation, and so the Φ representation is still within the FIR.

When we consider systems containing more than one particle, then Ω${}_{1} ( t )$ will have an explicit dependence on time in addition to (687). Then the Φ representation would no longer belong to the FIR. And this is physically reasonable, because in many-electron systems it is not in general possible to transform away the radiation interaction completely, the radiation interaction by itself giving rise to real effects such as Møller scattering which we should not wish to transform away.

### <a id="s-concluding-non-covariant-part-of-the-calculation"></a>Concluding Non-Covariant Part of the Calculation

Having established that the Φ-representation in which (664) holds is the FIR, we now proceed at once to transform to the BIR in which we shall finish the calculation of the <a id="ix-147">Lamb shift</a>. To transform to the BIR, we write $e$

<a id="eq-689"></a>

$$
\Phi ( t ) = e^{iH_{0} t/ \hbar} e^{- i \{ H_{0} + H \} t/ \hbar} \Phi^{\prime} ( t ) \tag{689}
$$

The new wave-function Φ${}^{\prime} ( t )$ thus satisfies

<a id="eq-690"></a>

$$
i \hbar \frac{\partial \Phi^{\prime}}{\partial t} = \{ H_{T 1} ( t ) + H_{T 2} ( t ) \} \Phi^{\prime} \tag{690}
$$

where $H_{T 1}$ and $H_{T 2}$ are given by (684) and (676), only now the $\psi^{*}$ and $\psi$ operators have the time-variation of the Dirac field in the external potential $V$ .

<a id="p128"><sup>[p. 128]</sup></a>

To solve (690) we may now use precisely the method that we used for solving (245) in the non-relativistic treatment. There are only two differences, (i) we have now the extra term $H_{T 2}$, and (ii) the form of $H_{T 1}$ is different form (247).

Since we are working only to second order in the radiation interaction and $H_{T 2}$ is already of that order, $H_{T 2}$ is to be treated only as a first-order perturbation. Then $H_{T 2}$ will have no effect on the line-width Γ, and will contribute to the line-shift ∆$E$ just the expectation-value of (676) in the state $\psi_{0}$ of the atom, namely

<a id="eq-691"></a>

$$
\Delta E_{2} = \frac{\alpha}{3 \pi \mu^{2}} \left\{ \log \frac{\mu}{2 r} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \int ( \nabla^{2} V ) | \psi_{0} |^{2} d^{3} \boldsymbol{r} - \frac{i\alpha}{4 \pi \mu} \int \psi_{0}^{*} \gamma_{4} ( \boldsymbol{\alpha} \cdot \nabla V ) \psi_{0} d^{3} \boldsymbol{r} \tag{691}
$$

The effect of the change from (247) to (684) is that the matrix element $j_{\mu}^{k} ( nm )$ given by (256) now becomes everywhere replaced by

<a id="eq-692"></a>

$$
J_{\mu}^{k} ( nm ) = \frac{ie}{\hbar \mu | \boldsymbol{k} |} \int \psi_{n}^{*} \frac{\partial V}{\partial x_{\mu}} e^{- i \boldsymbol{k} \cdot \boldsymbol{r}} \psi_{m} d^{3} \boldsymbol{r} \tag{692}
$$

We see this by comparing (247) and (684), noting that

$$
\boldsymbol{j} ( r,t ) \leftrightarrow \frac{ec}{\hbar \mu} \psi^{*} \psi \nabla V \int dt
$$

or

$$
\int \boldsymbol{j}^{S} ( r ) e^{- i \boldsymbol{k} \cdot \boldsymbol{r}} d^{3} \boldsymbol{r} \leftrightarrow \frac{iec}{\hbar \mu | \boldsymbol{k} | c} \int \psi^{*} \psi \nabla Ve^{- i \boldsymbol{k} \cdot \boldsymbol{r}} d^{3} \boldsymbol{r}
$$

In the previous calculation we used a non-relativistic dipole approximation for $j_{\mu}$ which gave according to (272)

<a id="eq-693"></a>

$$
j_{\mu}^{k} ( nm ) = + \frac{ie \hbar}{m} \int \psi_{n}^{*} \frac{\partial \psi}{\partial x_{\mu}} d^{3} \boldsymbol{r} \tag{693}
$$

We shall again use a dipole approximation and drop the exponential factor in (692). Then taking for the atom the non-relativistic Hamiltonian

<a id="eq-694"></a>

$$
H = \frac{p^{2}}{2 m} + V \tag{694}
$$

the difference between (692) and (693) becomes

<a id="eq-695"></a>

$$
J_{\mu}^{k} ( nm ) - j_{\mu}^{k} ( nm ) = \frac{ie}{\hbar \mu | \boldsymbol{k} |} \left\{ \int \psi_{n}^{*} \frac{\partial \psi}{\partial x_{\mu}} d^{3} \boldsymbol{r} \right\} ( E_{m} - E_{n} - hc | \boldsymbol{k} | ) \tag{695}
$$

where we used $\boldsymbol{[} p^{2} ,p_{\mu} \boldsymbol{]}$ = 0 and $\int p_{\mu} \psi_{n}^{*} p^{2} \psi_{m} d^{3} \boldsymbol{r} = - \int \psi_{n}^{*} p_{\mu} p^{2} \psi_{m} d^{3} \boldsymbol{r}$. This difference vanishes for transitions in which energy is conserved. Hence the value of Γ given by (262) is unaffected by the change from $j$ to $J$. The value Γ calculated previously is still valid in the relativistic theory, except for very small effects from magnetic radiation which we have neglected.

Using (695) we have the simple relation between $j_{\mu}$ and $J_{\mu}$,

<a id="eq-696"></a>

$$
J_{\mu}^{k} ( nm ) = j_{\mu}^{k} ( nm ) \frac{E_{m} - E_{n}}{hc | \boldsymbol{k} |} \tag{696}
$$

Using formula (261) with $J$ substituted for $j$, the contribution from $H_{T 1}$ to the line-shift becomes instead of (273)

<a id="eq-697"></a>

$$
\Delta E_{1} = - \frac{e^{2}}{6 \pi^{2} m^{2} \hbar c^{3}} \int_{r}^{\infty} \frac{dk}{k} \sum_{n} \frac{( E_{n} - E_{0} )^{2} | p_{n 0} |^{2}}{E_{n} - E_{0} + hc | \boldsymbol{k} |} \tag{697}
$$

The integral is now convergent at high frequencies and only divergent at low frequencies where the cut-off $r$ is now required to make it finite. The shift (697) would be zero for a free particle, and so there is no question of subtracting away a mass-renormalization term as we did from (273); in the relativistic treatment the mass-subtraction was already done long before this stage of the calculation was reached.

<a id="p129"><sup>[p. 129]</sup></a>

Integrating (697) directly over $k$, and taking $r$ to be small compared with $( E_{n} - E_{0} )$, we find

<a id="eq-698"></a>

$$
\Delta E_{1} = - \frac{e^{2}}{6 \pi^{2} m^{2} \hbar c^{3}} \sum_{n} ( E_{n} - E_{0} )^{2} | p_{n 0} |^{2} \log \frac{| E_{n} - E_{0} |}{hcr} \tag{698}
$$

This is precisely the non-relativistic line-shift (278) with $r$ substituted for $K$. Defining $( E - E_{0} )_{\text{ av }}$ by (279) and using (281), we have

<a id="eq-699"></a>

$$
\Delta E_{1} = \frac{\alpha}{3 \pi \mu^{2}} \left\{ \log \frac{hcr}{( E - E_{0} )_{\text{ av }}} \right\} \int ( \nabla^{2} V ) | \psi_{0} |^{2} d^{3} \boldsymbol{r} \tag{699}
$$

This combines with (691) to give for the total level shift

<a id="eq-700"></a>

$$
\Delta E = \frac{\alpha}{3 \pi \mu^{2}} \left\{ \log \frac{mc^{2}}{2 ( E - E_{0} )_{\text{ av }}} + \tfrac{1 1}{2 4} - \tfrac{1}{5} \right\} \int ( \nabla^{2} V ) | \psi_{0} |^{2} d^{3} \boldsymbol{r} - \frac{i\alpha}{4 \pi \mu} \int \psi_{0}^{*} \gamma_{4} ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla V ) \psi_{0} d^{3} \boldsymbol{r} \tag{700}
$$

a result which is completely divergence-free and independent of $r$.

The second term of (700) represents the effect of the anomalous magnetic moment of the electron on the energy levels. It therefore gives a spin-dependent shift which modifies slightly the fine-structure which arises from the Dirac magnetic moment. To evaluate this term we use the Dirac equations (see (38)).

<a id="eq-701"></a>

$$
mc^{2} \gamma_{4} \psi_{0} = ( E_{0} - V ) \psi_{0} + i \hbar c ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla ) \psi_{0} \tag{701}
$$

<a id="eq-702"></a>

$$
mc^{2} \psi_{0}^{*} \gamma_{4} = \psi_{0}^{*} ( E_{0} - V ) - i \hbar c ( \nabla \psi_{0}^{*} \boldsymbol{\cdot} \boldsymbol{\alpha} ) \tag{702}
$$

Using both (701) and (702) in turn in the second term of (700) and adding the results using $\alpha^{i} \gamma_{4} + \gamma_{4} \alpha^{i}$ = 0 the terms in $( E_{0} - V )$ cancel and we find<sup><a id="fnref-93" href="#fn-93">93</a></sup>

<a id="eq-703"></a>

$$
\begin{aligned} & \quad 2 mc^{2} \int \psi_{0}^{*} \gamma_{4} ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla V ) \psi_{0} d^{3} \boldsymbol{r} \\ &= - i \hbar c \int \{ ( \nabla \psi_{0}^{*} \boldsymbol{\cdot} \boldsymbol{\alpha} ) ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla V ) \psi_{0} + \psi_{0}^{*} ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla V ) ( \boldsymbol{\alpha} \boldsymbol{\cdot} \nabla \psi_{0} ) \} d^{3} \boldsymbol{r} \\ &= - i \hbar c \int \{ ( \nabla \psi_{0}^{*} \boldsymbol{\cdot} \boldsymbol{\sigma} ) ( \boldsymbol{\sigma} \boldsymbol{\cdot} \nabla V ) \psi_{0} + \psi_{0}^{*} ( \boldsymbol{\sigma} \boldsymbol{\cdot} \nabla V ) ( \boldsymbol{\sigma} \boldsymbol{\cdot} \nabla \psi_{0} ) \} d^{3} \boldsymbol{r} \\ &= - i \hbar c \int \{ \psi_{0} [ \nabla \psi_{0}^{*} \boldsymbol{\cdot} \nabla V + i \boldsymbol{\sigma} \boldsymbol{\cdot} ( \nabla \psi_{0}^{*} \times \nabla V ) ] + \psi_{0}^{*} [ \nabla \psi_{0} \boldsymbol{\cdot} \nabla V + i \boldsymbol{\sigma} \boldsymbol{\cdot} ( \nabla \psi_{0} \times \nabla V ) ] \} d^{3} \boldsymbol{r} \\ &= i \hbar c \int \left\{ + ( \nabla^{2} V ) \psi_{0}^{*} \psi_{0} - 2 i\psi^{*} [ \boldsymbol{\sigma} \boldsymbol{\cdot} ( \nabla V \times \nabla ) ] \psi_{0} \right\} d^{3} \boldsymbol{r} \end{aligned} \tag{703}
$$

Here we used $\alpha^{i} = \epsilon \sigma_{i}$ (see p. 20), $\epsilon^{2} = \mathbb{I}$, and the formula

$$
( \boldsymbol{\sigma} \cdot \boldsymbol{B} ) ( \boldsymbol{\sigma} \cdot \boldsymbol{C} ) = ( \boldsymbol{B} \cdot \boldsymbol{C} ) + i ( \boldsymbol{\sigma} \cdot \boldsymbol{B} \times \boldsymbol{C} )
$$

See Dirac, _The Principles of Quantum Mechanics_, third edition, p. 263.

Now suppose $V$ is a central potential, a function of $r$ only. Then

<a id="eq-704"></a>

$$
\nabla V \times \nabla = \frac{1}{r} \frac{dV}{dr} ( \boldsymbol{r} \times \nabla ) = \frac{1}{r} \frac{dV}{dr} \left( \frac{i}{\hbar} \boldsymbol{L} \right) \tag{704}
$$

with $\boldsymbol{L}$, the orbital angular momentum, is given by (39). In this case (700) becomes

<a id="eq-705"></a>

$$
\begin{aligned} \Delta E &= \frac{\alpha}{3 \pi \mu^{2}} \left\{ \log \frac{mc^{2}}{2 ( E - E_{0} )_{\text{ avg }}} + \tfrac{5}{6} - \tfrac{1}{5} \right\} \int ( \nabla^{2} V ) | \psi_{0} |^{2} d^{3} \boldsymbol{r} \\ & \quad + \frac{\alpha}{4 \pi \mu^{2} \hbar} \int \psi_{0}^{*} \left( \frac{1}{r} \frac{dV}{dr} \right) ( \boldsymbol{\sigma} \cdot \boldsymbol{L} ) \psi_{0} d^{3} \boldsymbol{r} \end{aligned} \tag{705}
$$

<a id="p130"><sup>[p. 130]</sup></a>

In the non-relativistic theory of the hydrogen atom, the quantum number $j$ given by (72) is related to the operator $( \boldsymbol{\sigma} \cdot \boldsymbol{L} )$ by

<a id="eq-706"></a>

$$
\frac{1}{\hbar} ( \boldsymbol{\sigma} \cdot \boldsymbol{L} ) = \begin{cases} \ell , & j = \ell + \tfrac{1}{2} \\ - \ell - 1 , & j = \ell - \tfrac{1}{2} \end{cases} \tag{706}
$$

Therefore for the hydrogen atom

<a id="eq-707"></a>

$$
\Delta E = \frac{\alpha e^{2}}{3 \pi \mu^{2}} \left\{ \log \frac{mc^{2}}{2 ( E - E_{0} )_{\text{ avg }}} + \tfrac{5}{6} - \tfrac{1}{5} \right\} | \psi_{0} ( 0 ) |^{2} + \frac{\alpha e^{2}}{1 6 \pi^{2} \mu^{2}} q \int \frac{1}{r^{3}} | \psi_{0} |^{2} d^{3} \boldsymbol{r} \tag{707}
$$

where $q$ is the coefficient (706).

For $s$-states we have $q$ = 0 and so the shift reduces to (see (284))

<a id="eq-708"></a>

$$
\Delta E = \frac{8 \alpha^{3}}{3 \pi} \frac{1}{n^{3}} \text{ Ry } \left\{ \log \frac{mc^{2}}{2 ( E - E_{0} )_{\text{ avg }}} + \tfrac{5}{6} - \tfrac{1}{5} \right\} \tag{708}
$$

for the state with principal quantum number $n$. For all other states the term in $| \psi_{0} ( 0 ) |^{2}$ is zero, and the shift depends only on the integral

<a id="eq-709"></a>

$$
\left( \frac{1}{r^{3}} \right) = \int \frac{1}{r^{3}} | \psi_{0} |^{2} d^{3} \boldsymbol{r} \tag{709}
$$

The value of (709) is given by Bethe, _Handbuch der Physik_, Vol. 24/1, p. 286, Eq. (3.26) <a id="08-scattering-by-a-static-potential-comparison-with-experimental-results--refback-20-1"></a>[[20]](#08-scattering-by-a-static-potential-comparison-with-experimental-results--ref-20). It is

<a id="eq-710"></a>

$$
\left( \frac{1}{r^{3}} \right) = \frac{1}{\ell ( \ell + \tfrac{1}{2} ) ( \ell + 1 ) n^{3} a_{o}^{3}} \tag{710}
$$

where $a_{o}$ is the Bohr radius of the hydrogen atom. Hence the shift for states with $\ell \neq$ 0 becomes

<a id="eq-711"></a>

$$
\Delta E = \frac{\alpha^{3}}{2 \pi} \frac{1}{n^{3}} \text{ Ry } \frac{1}{( \ell + \tfrac{1}{2} ) ( \ell + 1 )} \quad \text{ for } j = \ell + \tfrac{1}{2} \tag{711}
$$

<a id="eq-712"></a>

$$
\Delta E = - \frac{\alpha^{3}}{2 \pi} \frac{1}{n^{3}} \text{ Ry } \frac{1}{\ell ( \ell + \tfrac{1}{2} )} \quad \text{ for } j = \ell - \tfrac{1}{2} \tag{712}
$$

For the relative displacement of the 2$s$ and 2$p_{1 / 2}$ levels, which in the Dirac theory were degenerate, we have finally by subtracting (712) from (708)

<a id="eq-713"></a>

$$
\Delta E = \frac{\alpha^{3}}{3 \pi} \text{ Ry } \left\{ \log \frac{mc^{2}}{2 ( E - E_{0} )_{\text{ avg }}} + \tfrac{5}{6} - \tfrac{1}{5} + \tfrac{1}{8} \right\} = 1 0 5 1 \text{ Megacycles } \tag{713}
$$

### <a id="s-accuracy-of-the-lamb-shift-calculation"></a>Accuracy of the Lamb Shift Calculation

With the relativistic calculation of the Lamb shift which we have done, this course comes to an end. In this calculation we have met and seen how to overcome all the problems of mass and charge renormalization. We can say we now have a workable quantum electrodynamics which will give finite and unambiguous values for all observable quantities.

This calculation of the Lamb shift was of course not exact. The two most important errors were<br>
(i) using non-relativistic wave-functions and the dipole radiation approximation in evaluating the effects of $H_{T 1}$;

(ii) neglecting the finite mass of the proton.

To correct these errors, very long calculations have been done. In connection with (i), Baranger <a id="refback-21-1"></a>[[21]](#ref-21) has calculated the effect of using relativistic theory in the treatment of $H_{T 1}$ and he finds the observed shift increases by 7 megacycles. The effects of (ii) are being looked at by Salpeter <a id="refback-22-1"></a>[[22]](#ref-22) but are not greater than 1 - 2 megacycles at most. In addition we have not considered

<a id="p131"><sup>[p. 131]</sup></a>

(iii) effects of fourth order in the radiation interaction. These are being looked at by Kroll and others

<a id="refback-23-1"></a>[[23]](#ref-23); they are certainly less than 1 megacycle. <a id="refback-24-1"></a>[[24]](#ref-24)

Therefore the theoretical value of the Lamb shift now stands at 1058 $\pm$ 2 megacycles. There is no clear discrepancy between this and the experimental value 1062 $\pm$ 5 though a discrepancy may be found when the experiments and the theory are further cleaned up.

<div align="center">

The End

</div>

---

## Notes

<a id="fn-82"></a>**82.** In the first line, twice in the fourth line, and in the first appearance in the sixth line, the expression $| u^{\prime *} u |$ lacked the exponent 2. These have been supplied. [↩](#fnref-82)

<a id="fn-83"></a>**83.** The modulus bars around $p$ were absent. [↩](#fnref-83)

<a id="fn-84"></a>**84.** “slowly-varying” replaces “slowly-carrying” [↩](#fnref-84)

<a id="fn-85"></a>**85.** For “Lagrangian” read here “Lagrangian density”. Field theorists, by an abuse of language, often say the first and mean the second. [↩](#fnref-85)

<a id="fn-86"></a>**86.** The original citation lacked Prodell’s name. [↩](#fnref-86)

<a id="fn-87"></a>**87.** In Schwinger’s anthology. [↩](#fnref-87)

<a id="fn-88"></a>**88.** The notation “Mc” is outmoded; usually this is written “MHz.” [↩](#fnref-88)

<a id="fn-89"></a>**89.** “$\alpha$” is the fine-structure constant, $\frac{1}{1 3 7 . 0 3 6}$; “$\boldsymbol{\alpha}$” is the Dirac matrix. Originally “$\boldsymbol{\alpha} \boldsymbol{\cdot} \nabla$” was rendered as “$\alpha \cdot$ grad”. [↩](#fnref-89)

<a id="fn-90"></a>**90.** The equation labeling was faulty. In the second edition, what is here labeled (676a) was a _second_ (676), and what is here labeled (677a) had no label at all. It might have been all right to leave Eq. (677a) unlabeled, except that in the first edition, _both_ Eq. (677) _and_ Eq. (677a) are labeled (677)! This is a compromise. [↩](#fnref-90)

<a id="fn-91"></a>**91.** The comparison with Eq. (422) is not obvious. Perhaps Eq. (438) was meant? [↩](#fnref-91)

<a id="fn-92"></a>**92.** The integrand was originally written [↩](#fnref-92)

$$
\overline{\psi} ( Z \boldsymbol{\cdot} \text{ grad } ) A /^{e} \psi ( x )
$$

Since $Z_{\lambda}$ is a Lorentz vector, the gradient must likewise be. So “grad” here must be $\partial_{\lambda}$, not $\nabla$. One goes from (682) to (680) by an integration by parts; since there are in (680) apparently the dot product of two 4-vectors, this supports the identification here of grad = $\partial_{\lambda}$.

<a id="fn-93"></a>**93.** In the second to last line, the term $i \boldsymbol{\sigma} \boldsymbol{\cdot} ( \nabla \psi_{0} \times \nabla V )$ was written with a dot product between the two gradients, rather than a cross product. [↩](#fnref-93)

---

## References

<a id="08-scattering-by-a-static-potential-comparison-with-experimental-results--ref-20"></a>**[20]** Hans A. Bethe & Edwin E. Salpeter, _Quantum Mechanics of One- and Two-Electron Atoms_, Springer- Verlag, Berlin, 1957. Reissued by Plenum Publishing Co., New York, 1977, (paperback edition). This is a revised and updated version of the article Dyson cites, H. A. Bethe, “Quantenmechanik der Einund Zwei-Elektronenprobleme”, _Handbuch der Physik_, Bd. 24/1, Springer, Berlin, 1933. The relevant formula for Eq. (710) is to be found in the new work with exactly the same label, (3.26), on p. 17. Note that Bethe & Salpeter use Hartree’s “atomic units”, so that distances are measured in terms of $a_{o}$. [↩](#08-scattering-by-a-static-potential-comparison-with-experimental-results--refback-20-1)

<a id="ref-21"></a>**[21]** M. Baranger, H. A. Bethe & R. P. Feynman, “Relativistic Corrections to the Lamb Shift”, _Phys. Rev._ **92** (1953) 482. [↩](#refback-21-1)

<a id="ref-22"></a>**[22]** E. E. Salpeter, “Mass Corrections to the Fine Structure of Hydrogen-Like Atoms”, _Phys. Rev._ **87** (1952) 328. [↩](#refback-22-1)

<a id="ref-23"></a>**[23]** M. Baranger, F. J. Dyson & E. E. Salpeter, “Fourth-Order Vacuum Polarization”, _Phys. Rev._ **88** (1952) 680. [↩](#refback-23-1)

<a id="ref-24"></a>**[24]** E. E. Salpeter, “The Lamb Shift for Hydrogen and Deuterium”, _Phys. Rev._ **89** (1953) 93. [↩](#refback-24-1)

---

<a id="09-typist-s-afterword"></a>

# Typist’s Afterword

<a id="p132"><sup>[p. 132]</sup></a>

Both Kaiser’s admirable _Drawing Theories Apart_ <a id="09-typist-s-afterword--refback-8-1"></a>[[8]](#09-typist-s-afterword--ref-8) and Schweber’s masterful _QED and the Men Who_ _Made It_ <a id="09-typist-s-afterword--refback-7-1"></a>[[7]](#09-typist-s-afterword--ref-7) refer frequently to the famous lectures on quantum electrodynamics given by Freeman Dyson at Cornell University in 1951. Two generations ago, graduate students (and their professors) wishing to learn the new techniques of QED passed around copies of Dyson’s Cornell lecture notes, then the best and fullest treatment available. Textbooks appeared a few years later, e. g. by Jauch & Rohrlich <a id="refback-25-1"></a>[[25]](#ref-25) and Schweber

<a id="09-typist-s-afterword--refback-6-1"></a>[[6]](#09-typist-s-afterword--ref-6), but interest in Dyson’s notes has never fallen to zero. Here is what the noted theorist E. T. Jaynes wrote in an unpublished article <a id="refback-26-1"></a>[[26]](#ref-26) on Dyson’s autobiographical _Disturbing the Universe_, 1984:

But Dyson’s 1951 Cornell course notes on Quantum Electrodynamics were the original basis of the teaching I have done since. For a generation of physicists they were the happy medium: clearer and better motivated than Feynman, and getting to the point faster than Schwinger. All the textbooks that have appeared since have not made them obsolete. Of course, this is to be expected since Dyson is probably, to this day, best known among the physicists as the man who first explained the unity of the Schwinger and Feynman approaches.

As a graduate student in Nicholas Kemmer’s department of theoretical physics (Edinburgh, Scotland) I had heard vaguely about Dyson’s lectures (either from Kemmer or from my advisor, Peter Higgs) and had read his classic papers <a id="refback-27-1"></a>[[27]](#ref-27), <a id="refback-28-1"></a>[[28]](#ref-28) in Schwinger’s collection <a id="09-typist-s-afterword--refback-4-1"></a>[[4]](#09-typist-s-afterword--ref-4). It never occurred to me to ask Kemmer for a copy of Dyson’s lectures which he almost certainly had.

My interest in the legendary notes was revived thirty years later by the Kaiser and Schweber books. Within a few minutes Google led to scans of the notes <a id="refback-29-1"></a>[[29]](#ref-29) at the Dibner Archive (History of Recent Science & Technology) at MIT, maintained by Karl Hall, a historian at the Central European University in Budapest, Hungary. He had gotten permission from Dyson to post scanned images of the Cornell notes. Through the efforts of Hall, Schweber and Babak Ashrafi these were uploaded to the Dibner Archive. To obtain a paper copy would require downloading almost two hundred images, expensive in time and storage. Was there a text version? Had anyone had retyped the notes? Hall did not know, nor did further searching turn anything up. I volunteered to do the job. Hall thought this a worthwhile project, as did Dyson, who sent me a copy of the second edition, edited by Michael J. Moravcsik. (This copy had originally belonged to Sam Schweber.) Dyson suggested that the second edition be retyped, not the first. Nearly all of the differences between the two editions are Moravcsik’s glosses on many calculations; there is essentially no difference in text, and (_modulo_ typos) all the labeled equations are identical.

Between this typed version and Moravcsik’s second edition there are few differences; all are described in the added notes. (I have also added references and an index.) About half are corrections of typographical errors. Missing words or sentences have been restored by comparison with the first edition; very infrequently a word or phrase has been deleted. A few changes have been made in notation. Intermediate steps in two calculations have been corrected but change nothing. Some notes point to articles or books. No doubt new errors have been introduced. Corrections will be welcomed! The young physicists will want familiar terms and notation, occasionally changed from 1951; the historians want _no_ alterations. It was not easy to find the middle ground.

I scarcely knew L$A$TEX before beginning this project. My friend (and Princeton ’74 classmate) Robert Jantzen was enormously helpful, very generous with his time and his extensive knowledge of L$A$TEX. Thanks, Bob. Thanks, too, to Richard Koch, Gerben Wierda and their colleagues, who have made L$A$TEX so easy on <a id="p133"><sup>[p. 133]</sup></a> a Macintosh. George Grätzer’s textbook _Math into L_A_TEX_ was never far from the keyboard. No one who types technical material should be ignorant of L$A$TEX.

This project would never have been undertaken without the approval of Prof. <a id="ix-96">Dyson</a> and the efforts of Profs. Hall, Schweber and Ashrafi, who made the notes accessible. I thank Prof. Hall for his steady encouragement through the many hours of typing. I thank Prof. Dyson both for friendly assistance and for allowing his wonderful lectures to become easier to obtain, to be read with pleasure and with profit for many years to come.

David Derbes

Laboratory Schools

University of Chicago loki@uchicago.edu

11 July 2006

---

## References

<a id="09-typist-s-afterword--ref-8"></a>**[8]** David Kaiser, _Drawing Theories Apart: the dispersion of Feynman diagrams in postwar physics_, U of Chicago Press, Chicago, 2005. The sociology of the transmission of Feynman’s graphical techniques. [↩](#09-typist-s-afterword--refback-8-1)

<a id="09-typist-s-afterword--ref-7"></a>**[7]** Silvan S. Schweber, _QED and the Men Who Made It: Dyson, Feynman, Schwinger and Tomonaga_, Princeton U. P., Princeton NJ, 1994. A very readable, technical history of QED. [↩](#09-typist-s-afterword--refback-7-1)

<a id="ref-25"></a>**[25]** J. M. Jauch and F. Rohrlich, _The Theory of Photons and Electrons_, Addison-Wesley Publishing Co., Cambridge, MA, 1955. [↩](#refback-25-1)

<a id="09-typist-s-afterword--ref-6"></a>**[6]** Silvan S. Schweber, _An Introduction to Relativistic Quantum Field Theory_, Row, Peterson & Co., Evanston, IL, 1961. This magisterial textbook has been reissued by Dover Publications (2005) in paperback. Contains a very complete set of references to the QED work done from 1926-1960. [↩](#09-typist-s-afterword--refback-6-1)

<a id="ref-26"></a>**[26]** E. T. Jaynes, “Disturbing the Memory”, http://bayes.wustl.edu/etj/node2.html; link #18, 1984. [↩](#refback-26-1)

<a id="ref-27"></a>**[27]** F. J. Dyson, “The Radiation Theories of Tomonaga, Schwinger and Feynman”, _Phys. Rev._ **75** (1949) 486. [↩](#refback-27-1)

<a id="ref-28"></a>**[28]** F. J. Dyson, “The S-Matrix in Quantum Electrodynamics”, _Phys. Rev._ **75** (1949) 1736. [↩](#refback-28-1)

<a id="09-typist-s-afterword--ref-4"></a>**[4]** J. Schwinger, ed., _Selected Papers on Quantum Electrodynamics_, Dover Publications, New York, 1958. Many of the most important Feynman, Schwinger and Dyson papers, together with those of other authors, are gathered in this anthology edited by Schwinger. [↩](#09-typist-s-afterword--refback-4-1)

<a id="ref-29"></a>**[29]** F. J. Dyson, “Advanced Quantum Mechanics”, http://hrst.mit.edu/hrs/renormalization/dyson51-intro /index.html, 1951. [↩](#refback-29-1)

---

<a id="10-index"></a>

# Index

<a id="p140"><sup>[p. 140]</sup></a>

- $a_{o}$, _see_ Bohr radius
- contraction of field operators, [103](#p103), [104](#p104), [109](#p109)
- Abelian, [48](#p48)
- Coulomb potential, [22](#p22), [25](#ix-0), [59](#p59), [68](#p68), [71](#p71), [80](#p80), [94](#p94)
- action, [32](#ix-1), [33](#ix-2), [36](#ix-3), [37](#ix-4)
- create, [42](#p42), [83](#ix-5), [85](#p85), [94](#p94), [97](#p97), [98](#p98), [101](#p101)
- adiabatic, [97](#p97)
- creation, [1](#ix-6), [2](#ix-7), [12](#ix-8), [26](#ix-9), [27](#ix-10), [71](#p71)
- $\alpha$, _see_ fine structure constant
  - operator, [66](#p66)
- amplitude, [4](#ix-11), [29](#ix-12), [35](#ix-13), [36](#ix-14), [52](#p52), [55–58](#ix-15), [72](#p72), [103](#p103)
  - pair, [25](#ix-16), [26](#ix-17), [31](#p31), [79](#p79), [84](#p84), [94](#p94), [101](#ix-18), [102](#p102), [116](#p116)
  - transition, [35](#ix-19)
- cross-section
- angle, [26](#ix-20), [31](#ix-21), [91](#ix-22), [121](#p121)
  - annihilation, [93](#p93)
  - scattering, [30](#ix-23), [91](#ix-22), [92](#p92)
  - bremsstrahlung, [94](#ix-24), [123](#p123)
  - solid, [26](#ix-25), [50](#p50), [88](#p88), [93](#p93), [120](#ix-26), [122](#p122)
  - differential, [122](#p122)
- angular momentum, [9](#ix-27), [13](#ix-28), [132](#p132)
  - differential, for Møller scattering, [30](#ix-29)
  - conservation of, [10](#ix-30)
  - differential, photon, [88](#p88)
- annihilate, [92](#p92), [97](#p97), [98](#p98), [101](#p101)
  - experimental, [96](#ix-31)
- annihilation, [1](#ix-32), [30](#ix-33), [86](#p86), [92](#p92)
  - from amplitudes, [28–30](#p28)
  - cross-section, [93](#ix-34)
  - Klein-Nishina, [91](#ix-35)
  - life-time, [93](#p93)
  - neutron, [93](#p93)
  - pair, [92](#ix-36), [101](#ix-18), [102](#p102)
  - non-radiative, [108](#p108), [121](#ix-37)
- anticommute, [40](#ix-38), [41](#p41), [61](#p61), [66](#p66), [68](#ix-39), [69](#ix-40), [71](#p71), [84](#ix-41), [87](#ix-42),
  - pair creation, [12](#ix-43) [89](#p89), [98](#p98)
  - radiative, [122–123](#p122)
  - scattering by a static potential, [120](#p120)
- Baranger, Michael, [133](#p133)
  - Thomson scattering, [92](#p92)
- bare mass, [59](#p59), [106](#p106)
  - unpolarized electron beam, [121](#p121)
- Bethe, Hans A., [50](#p50), [59](#p59), [133](#p133)
- Bleuler, K., [46](#p46)
- d’Alembertian operator, [79](#p79)
- Deutsch, Martin, [86](#p86)
- Bloch, Felix, [119](#p119)
- Bohr radius, [52](#p52), [53](#p53), [60](#p60), [93](#p93), [132](#p132)
- De Broglie wavelength, [25](#ix-44)
- Born approximation, [22](#ix-45), [24](#ix-46), [27](#ix-47), [84](#p84), [94](#ix-48), [96](#p96), [107](#p107),
- dielectric constant, [71](#p71), [78](#p78) [124](#p124)
- Dirac
- Bose statistics, [71](#p71)
  - electron magnetic moment, [60](#p60)
- bound interaction representation, [81](#p81), [125](#p125), [126](#p126)
  - electron theory, [4](#p4), [11](#p11), [60](#p60), [61](#p61), [67](#p67)
  - Schwinger invention, [126](#p126)
  - equation, [4–7](#ix-49), [10](#ix-50), [13](#ix-51), [14](#ix-52), [17](#ix-53), [22](#ix-54), [23](#ix-55), [28](#ix-56), [31](#ix-57),
- bremsstrahlung, [94](#ix-24), [96](#p96), [108](#p108), [123](#p123) [105](#ix-58), [124](#p124), [128](#p128)
- Brown, L. M., [59](#p59) conjugate, [9](#p9) covariant notation, [8](#p8)
- Cauchy principal value, [57](#p57), [59](#p59) field quantization, [60](#p60)
- charge renormalization, [78](#p78) Hamiltonian, [9](#p9)
- charge symmetry, [64](#p64) Lorentz invariance, [6](#p6), [7](#p7)
- chronological product, [83](#p83), [84](#p84), [102](#p102) positron, [11](#p11), [13](#p13)
- classical electron radius, [93](#p93) second order, [18](#p18)
- Compton with electromagnetic fields, [12](#p12)
  - effect, [86–91](#p86), [94](#ix-59), [96](#p96), [101](#ix-60), [103](#ix-61)
  - field, [34](#ix-62), [129](#p129)
  - scattering, [101](#ix-63) interacting with Maxwell field, [34](#p34), [80](#p80)
  - wavelength, [53](#p53) in external potential, [130](#p130)
- conservation laws, [9](#ix-64)
  - hole theory, [11](#ix-65), [66](#p66)

<a id="p141"><sup>[p. 141]</sup></a>

  - ket, [35](#ix-66)
  - contour integral, [85](#ix-67), [87](#p87), [109](#p109), [111](#p111)
  - Lagrangian, [40](#ix-68)
  - definition of operators, [38](#p38)
  - matrices, [5](#ix-69), [20](#p20), [62](#p62), [104](#p104), [111](#p111), [127](#p127)
  - graph, [100](#ix-70), [101](#ix-71), [113](#p113) Dyson notation, [20](#p20)
  - integration formula, [75](#p75) Feynman notation, [21](#p21)
  - propagator, _see_ Feynman, $D_{F} ,S_{F} ,$ ∆$F$ in denominator, [87](#p87)
  - quantization, [35](#ix-72), [36](#ix-73), [40](#p40), [98](#p98), [102](#p102) spur theorems, [89](#p89), [114](#p114)
  - rules, [102–104](#ix-74), [108](#p108), [113](#p113)
  - notation, [38](#ix-75)
- Feynman, Richard P., [1](#ix-76), [2](#ix-77), [34](#ix-78), [82](#ix-79), [133](#p133)
  - wave function, [18](#ix-80), [68](#ix-81), [81](#ix-82)
- Fierz, Markus, [85](#p85)
- Dirac, P. A. M., [1](#ix-83), [2](#ix-84), [4](#ix-85), [10](#ix-86), [14](#ix-87), [31](#ix-88), [132](#p132)
- fine structure constant, [15](#ix-89), [17](#p17), [60](#p60), [78](#p78), [109](#p109), [113](#p113),
- distribution, [24](#ix-90), [98](#p98) [123](#p123), [125](#p125)
  - angular, [24](#ix-91), [27](#ix-92)
- Fitzgerald contraction, [6](#ix-93)
  - momentum, [24](#ix-94)
- Fourier
- Dulit, Everett, [86](#p86)
  - components, [44](#ix-95), [45](#p45), [63](#p63), [79](#p79), [94](#p94)
- Dyson, Freeman J., i, [133](#ix-96)
  - expansion, [127](#p127)
  - integral, [110](#ix-97)
- electron
- free interaction representation, [81](#p81), [125](#p125)
  - absorption and emission operators, [64](#p64)
  - anomalous magnetic moment, [60](#p60), [123–125](#ix-98),
- gauge [131](#p131)
  - condition, [26](#p26), [27](#p27), [114](#ix-99) Schwinger correction, [125](#p125)
  - invariance, [46](#p46), [77](#p77)
  - bare, [59](#p59), [106](#p106), [129](#p129)
  - transformation, [26](#ix-100)
  - classical radius, [91](#p91)
- Gauss integral, _see_ Laplace integral
  - Compton wavelength, [53](#p53)
- Green’s theorem, [60](#p60)
  - in electromagnetic field
- Gupta, S. N., [46](#p46), [48](#p48) nonrelativistic treatment, [17](#p17)
- Gupta-Bleuler method, [46](#p46), [48–50](#p48)
  - magnetic moment, [2](#ix-101), [18](#ix-102), [124](#p124)
- Hamiltonian, [33–35](#ix-103), [40](#p40), [50](#p50), [54](#p54) Dirac prediction, [18](#p18)
  - and anomalous magnetic moment, [124](#ix-104)
  - negative energy, [11](#ix-105), [12](#ix-106)
  - Dirac and Maxwell fields, [129](#p129)
  - states, [10](#ix-107), [22](#ix-108)
  - Dirac field, [65](#p65), [66](#p66), [70](#p70) projection operator, [23](#p23) in external field, [70](#p70)
  - wave function, [23](#ix-109)
  - field equations, [34](#ix-62)
- electron-electron scattering, [27](#p27), [30](#p30)
  - non-relativistic atom, [60](#p60), [130](#p130)
- electron-photon scattering, [86](#p86)
  - transformed, [126](#ix-110)
- electron-positron
  - with and without radiation, [81](#ix-111)
  - annihilation, [92–93](#p92)
- Hanson, A. O., [123](#p123)
  - creation, [26](#ix-9)
- Heaviside units, [52](#p52), [60](#p60), [77](#p77)
  - field, [72](#p72)
- Heitler, Walter, [1](#ix-112), [23](#ix-113), [94](#p94), [95](#p95)
  - pair, [79](#p79)
- Hermitian, [4](#ix-114), [9](#ix-115), [22](#ix-116), [44](#p44)
  - scattering, [30](#ix-23), [86](#ix-117), [92](#p92)
  - conjugate, [4](#ix-118), [63](#ix-119), [68](#p68)
  - symmetry, [65](#p65)
- Huyghens principle, [36](#p36)
- electrons and positrons
- hydrogen atom, [2](#p2), [13–17](#ix-120)
  - relativistic field theory, [60](#p60)
  - energy levels, [17](#p17), [70](#p70)
- Fermi
  - Hamiltonian, [14](#ix-121)
  - form of Maxwell Lagrangian, [34](#ix-122)
  - Lamb shift, [52](#ix-123), [126–132](#ix-124)
  - statistics, [71](#p71)
  - numerical calculation of energies, [59](#p59)
- Feynman
  - radial equation, [15–17](#p15)
  - $D_{F}$, [84–85](#p84)
  - radiative corrections to electron motion, [96](#p96)
  - $S_{F}$, [87](#p87)
  - vacuum polarization, [79](#p79)
  - ∆$F$, [87](#p87)
- infra-red divergence, [116](#p116), [118–119](#p118), [124](#p124)
  - $i\epsilon$ prescription, [116](#p116)

<a id="p142"><sup>[p. 142]</sup></a>

- interaction representation, [81–82](#ix-125), [96](#p96), [106](#p106)
  - distribution, [24](#ix-90)
  - integral, [64](#p64), [75](#ix-126), [103](#ix-127)
- Karplus, Robert, [125](#p125)
  - representation, [43](#p43), [45](#p45), [50](#p50), [62](#p62), [73](#ix-128), [74](#p74), [85](#ix-129), [87](#p87)
- Klein-Gordon equation, [3](#ix-130), [4](#p4), [24](#ix-131), [31](#p31), [34](#p34)
  - space, [22](#ix-132)
- Klein-Nishina formula, [91](#p91)
- Moravcsik, Michael J., i
- Koenig, Seymour H., [125](#p125)
- Morette, Cécile, [36](#ix-133)
- Kramers, Hendrik A., [59](#p59)
- Mott, Neville F., [22](#ix-134), [30](#ix-135)
- Kroll, Norman M., [125](#p125), [133](#p133)
- $\mu$, [34](#p34), [60](#p60)
- Kusch, Polykarp, [125](#p125)
- Nordsieck, Arnold E., [119](#p119)
- Lagrangian, free-particle, [12](#ix-136)
- normal form, [98–103](#p98), [105](#p105)
- Lagrangian density, [32](#ix-137)
  - Dirac, [34](#ix-138), [61](#ix-139)
- Pauli
  - Dirac-Maxwell, [34](#p34)
  - exclusion principle, [66](#p66)
  - inclusion of anomalous magnetic moment, [124](#p124)
  - matrices, [5](#ix-69)
  - Klein-Gordon, imaginary, [34](#ix-140)
  - spin & statistics, [2](#ix-141)
  - Klein-Gordon, real, [34](#ix-140)
- Pauli, Wolfgang, [1–3](#ix-142)
  - Maxwell, [34](#ix-143)
- Peierls
  - quantum electrodynamics, [80](#ix-144)
  - formula, [40](#ix-145), [62](#p62), [69](#p69)
- Lamb, Willis E., [125](#ix-146)
  - method, [42](#p42), [61](#p61)
- Lamb shift, [52](#ix-123), [53](#p53), [60](#p60), [72](#p72), [79](#p79), [80](#p80), [104](#p104), [125](#ix-146), [127–](#ix-147)
- Peierls, Rudolf E., [40](#ix-148) [129](#p129), [133](#p133)
- $\pi$ meson, [3](#p3)
  - experiment, [2](#ix-149), [60](#p60)
- Poisson’s equation, [25](#ix-150)
- Laplace integral, [77](#p77)
- positron, [1](#ix-151), [11](#ix-152), [26](#ix-153)
- Lorentz
  - absorption and emission operators, [64](#p64)
  - frame, [81](#p81)
  - and line shift, [58](#ix-154)
  - gauge condition, [114](#p114)
  - failure of commuting fields, [66](#ix-155)
  - invariance, [1](#ix-156), [5](#ix-157), [73](#p73)
  - positive energy, [70](#p70)
  - system, [6](#ix-158), [29](#ix-159), [68](#ix-160), [125](#p125)
  - states, [11–12](#ix-161), [22](#ix-108), [23](#ix-162), [63–65](#ix-163)
  - transformations, [1](#ix-164), [6](#ix-165), [22](#ix-166), [30](#ix-167), [73](#p73), [79](#p79), [111](#p111)
- positron-electron pair, [71](#p71)
- Lyman, E. M., [123](#p123)
- positronium, [86](#p86), [93](#p93)
- probability, [27](#ix-168), [35](#ix-169)
- Massey, H. S. W., [30](#ix-170)
  - amplitude, [38](#p38), [55](#ix-15)
- mass renormalization, [58–59](#ix-171) transition, [35](#p35)
- Maxwell
  - and continuity, [3](#ix-172)
  - electromagnetic theory, [6](#ix-173), [116](#p116)
  - annihilation, [92](#p92)
  - equations, [26](#ix-174), [27](#ix-175), [45](#p45)
  - conserved, [2](#ix-176)
  - field, [31](#ix-177), [48](#p48), [54](#ix-178), [60](#ix-179), [61](#p61), [64](#ix-180), [65](#ix-181), [67](#ix-182), [72](#p72), [81](#ix-183)
  - creation, [27](#ix-10) classical, [71](#p71)
  - density, [4](#ix-184), [6](#ix-185), [12](#ix-186) external classical, [80](#p80), [124](#p124)
  - differential, [26](#ix-187) Hamiltonian, [54](#ix-188)
  - emission, [49](#ix-189), [50](#p50), [57](#p57) Lagrangian, [34](#ix-143)
  - pair creation, [26](#ix-190), [79](#ix-191) modified, [116](#p116)
  - pair emission, [26](#ix-17) quantized, [60](#ix-192), [72](#p72), [107](#p107)
  - radiation relativistic treatment, [60](#ix-192) per unit time, [57](#p57) vacuum, [55](#p55)
  - scattering, [88](#p88)
  - potentials, [80](#ix-193), [84](#ix-194) infra-red, [119](#p119)
  - radiation, [71](#p71) radiative, [122](#ix-195)
- meson, [1](#ix-196), [2](#ix-197) reduced, [113](#p113)
- Møller scattering, [27](#p27), [30](#ix-198), [31](#p31), [84–86](#p84), [101](#p101), [120](#p120), [129](#p129) with no emitted photon, [119](#p119)
- momentum, [4](#ix-199), [9–12](#ix-200), [15](#ix-201), [23](#ix-202) with one emitted photon, [118](#p118)
  - conservation, [9](#ix-64), [29](#ix-203), [88](#p88)
  - scattering amplitude, [27](#ix-204)

<a id="p143"><sup>[p. 143]</sup></a>

  - transition, [22](#ix-205), [24](#ix-206), [29](#ix-207)
- Scott, M. B., [123](#p123) radiative, [57](#p57)
- second quantization, [31](#ix-208)
- problem
- self-energy _1_, [8](#p8)
  - and Lamb shift, [72](#p72) _2_, [18](#p18)
  - electron, [72](#p72), [105](#ix-209), [112](#p112) _3_, [24](#ix-210)
  - photon, [77](#p77) _4_, [24](#p24)
  - vacuum, [108](#p108) _5_, [34](#p34)
- Smith, Lloyd P., [51](#p51)
- Prodell, Albert G., [125](#p125)
- spin, existence of, [9](#ix-211)
- projection operator, [23](#ix-212), [63](#p63), [89](#p89)
- spinors, [8](#ix-213), [93](#p93)
- spur theorems, [89](#p89), [114](#p114)
- quantum electrodynamics, [2](#ix-214), [34](#ix-215), [54](#p54), [60](#p60), [73](#ix-216), [74](#p74),
- Stehn, J. R., [59](#p59) [80](#p80), [81](#p81), [102](#p102), [106](#p106), [133](#p133)
- Stueckelberg, E. C. G., [84](#p84), [85](#p85)
  - Lagrangian, [40](#ix-68)
- Stueckelberg $D^{c}$, [84](#p84)
- $r_{o}$, _see_ classical electron radius
- Thomson scattering, [92](#p92)
- radiative corrections
- trace theorems, _see_ spur theorems
  - electron motion in hydrogen atom, [96](#p96)
  - scattering, [96](#p96), [119–125](#ix-217)
- Uehling, E. A., [80](#p80) electron by a weak potential, [104](#p104)
- vacuum polarization, [72](#p72), [77–80](#ix-218), [82](#p82), [116](#p116)
- renormalization
  - Schwinger calculation, [72](#p72)
  - advantages of covariant calculation, [82](#p82)
  - and external potential, [109](#p109)
- Weisskopf, Victor, [3](#ix-219)
  - and vacuum polarization, [72](#p72)
- Welton, Theodore A., [53](#p53)
  - charge, [78](#ix-220), [113](#p113)
- Wentzel, Gregor, [1](#ix-221), [34](#ix-222), [48](#p48)
  - elimination of divergent effects, [127](#p127)
- Wick, Gian Carlo, [98](#p98), [102](#p102)
  - mass, [60](#p60), [107](#p107), [125](#p125), [131](#p131)
- Wilson, Robert R., [71](#p71)
  - mass and charge, [133](#p133)
- WKB approximation, [36](#ix-223)
  - wave function, [113](#ix-224), [115](#p115)
- Rydberg energy, [53](#p53)
- Yukawa, Hideki, [1](#ix-225)
- _S_-matrix, [83](#p83)
- Salpeter, Edwin E., [133](#p133)
- Sauter, Fritz, [18](#ix-226)
- scattering
  - and Born approximation, [22](#ix-45)
  - Compton, [86–91](#ix-117)
  - light by light, [78](#p78)
  - Møller, [27](#ix-227), [84–86](#ix-228)
  - photon, and vacuum polarization, [71](#p71)
  - Thomson, [92](#p92)
- Schrödinger
  - equation, [18](#ix-229), [24](#ix-230), [38](#ix-231), [41](#ix-232), [54](#p54), [81](#ix-233), [106](#p106), [124](#ix-234)
  - representation, [39](#p39), [54](#ix-235)
  - wave function, [54](#ix-236)
- Schrödinger, Erwin, [3](#ix-237)
- Schwinger
  - action principle, [36](#ix-3), [38](#ix-238), [40](#ix-239) operator form, [39](#p39)
  - covariant electrodynamics, [73](#p73)
  - difficult to read, [104](#p104)
- Schwinger, Julian, [1](#ix-240), [2](#ix-241), [18](#ix-242), [34](#ix-243), [82](#p82), [125](#p125), [129](#p129)

