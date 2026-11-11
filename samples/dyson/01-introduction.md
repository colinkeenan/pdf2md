# Introduction

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p1"><sup>[p. 1]</sup></a>

## <a id="s-books"></a>Books

W. <a id="ix-219">Pauli</a>, “Die Allgemeinen Principien der Wellenmechanik”; _Handbuch der Physik_, 2 ed., Vol. 24,

Part 1; Edwards reprint, Ann Arbor 1947. (In German) <a id="refback-1-1"></a>[[1]](#ref-1)

W. <a id="ix-151">Heitler</a>, _Quantum Theory of Radiation_, 2nd Edition, Oxford. 3rd edition just published. <a id="refback-2-1"></a>[[2]](#ref-2)

G. <a id="ix-281">Wentzel</a>, _Introduction to the Quantum Theory of Wave-Fields_, Interscience, N.Y. 1949 <a id="refback-3-1"></a>[[3]](#ref-3)

I shall not expect you to have read any of these, but I shall refer to them as we go along. The later part of the course will be new stuff, taken from papers of <a id="ix-135">Feynman</a> and <a id="ix-271">Schwinger</a> mainly. <a id="refback-4-1"></a>[[4]](#ref-4), <a id="refback-5-1"></a>[[5]](#ref-5), <a id="refback-6-1"></a>[[6]](#ref-6), <a id="refback-7-1"></a>[[7]](#ref-7), <a id="refback-8-1"></a>[[8]](#ref-8)

## <a id="s-subject-matter"></a>Subject Matter

You have had a complete course in non-relativistic quantum theory. I assume this known. All the general principles of the NR theory are valid and true under all circumstances, in particular also when the system happens to be relativistic. What you have learned is therefore still good.

You have had a course in classical mechanics and electrodynamics including special relativity. You know what is meant by a system being relativistic; the equations of motion are formally invariant under <a id="ix-174">Lorentz</a> <a id="ix-179">transformations</a>. General relativity we shall not touch.

This course will be concerned with the development of a _Lorentz–invariant Quantum theory_. That is not a general dynamical method like the NR quantum theory, applicable to all systems. We cannot yet devise a general method of that kind, and it is probably impossible. Instead we have to find out what are the possible systems, the particular equations of motion, which can be handled by the NR quantum dynamics and which are at the same time Lorentz–invariant.

In the NR theory it was found that almost any classical system could be handled, i.e.quantized. Now on the contrary we find there are very few possibilities for a relativistic quantized system. This is a most important fact. It means that starting only from the principles of relativity and quantization, it is mathematically possible only for very special types of objects to exist. So one can _predict_ mathematically some important things about the real world. The most striking examples of this are:

(i) <a id="ix-93">Dirac</a> from a study of the electron predicted the <a id="ix-224">positron</a>, which was later discovered <a id="refback-9-1"></a>[[9]](#ref-9).

(ii) <a id="ix-284">Yukawa</a> from a study of nuclear forces predicted the <a id="ix-199">meson</a>, which was later discovered <a id="refback-10-1"></a>[[10]](#ref-10).

These two examples are special cases of the general principle, which is the basic success of the relativistic quantum theory, that _A Relativistic Quantum Theory of a Finite Number of Particles is Impossible._ A RQ theory necessarily contains these features: an indefinite number of particles of one or more types, particles of each type being identical and indistinguishable from each other, possibility of <a id="ix-41">creation</a> and <a id="ix-19">annihilation</a> of particles. Thus the two principles of relativity and quantum theory when combined lead to a world built up out of various types of elementary particles, and so make us feel quite confident that we are on the right way to an understanding of the real world. In addition, various detailed properties of the observed particles are necessary consequences of the

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p2"><sup>[p. 2]</sup></a>

general theory. These are for example:

(i) <a id="ix-112">Magnetic moment</a> of Electron (<a id="ix-94">Dirac</a>) <a id="refback-9-2"></a>[[9]](#ref-9).

(ii) Relation between spin and statistics (<a id="ix-218">Pauli</a>) <a id="refback-11-1"></a>[[11]](#ref-11).

## <a id="s-detailed-program"></a>Detailed Program

We shall not develop straightaway a correct theory including many particles. Instead we follow the historical development. We try to make a relativistic quantum theory of _one_ particle, find out how far we can go and where we get into trouble. Then we shall see how to change the theory and get over the trouble by introducing many particles. Incidentally, the one–particle theories are quite useful, being correct to a good approximation in many situations where <a id="ix-42">creation</a> of new particles does not occur, and where something better than a NR approximation is needed. An example is the Dirac theory of the H atom.<sup><a id="fnref-1" href="#fn-1">1</a></sup>

The NR theory gave levels correctly but no fine-structure. (Accuracy of one part in 10,000). The Dirac one-particle theory gives all the main features of the fine-structure correctly, number of components and separations good to 10% but not better. (Accuracy one part in 100,000).

The Dirac many-particle theory gives the fine-structure separations (Lamb <a id="ix-173">experiment</a>) correctly to about one part in 10,000. (Overall accuracy 1 in 10<sup>8</sup>.)

Probably to get accuracy better than 1 in 10<sup>8</sup> even the DMP theory is not enough and one will need to take all kinds of <a id="ix-200">meson</a> effects into account which are not yet treated properly. Experiments are so far only good to about 1 in 10<sup>8</sup>.

In this course I will go through the one-particle theories first in detail. Then I will talk about their breaking down. At that point I will make a fresh start and discuss how one can make a relativistic quantum theory in general, using the new methods of <a id="ix-136">Feynman</a> and <a id="ix-272">Schwinger</a>. From this we shall be led to the many-particle theories. I will talk about the general features of these theories. Then I will take the special example of <a id="ix-251">quantum electrodynamics</a> and get as far as I can with it before the end of the course.

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

To give a physical meaning to $\psi$, we state that $\rho = \psi^{*} \psi$ is the probability of finding the particle at the point $xyz$ at time $t$. And the probability is <a id="ix-235">conserved</a> because<sup><a id="fnref-3" href="#fn-3">3</a></sup>

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

<hr class="page" style="height:2px;border:0;background:currentColor;opacity:0.25">
<a id="p3"><sup>[p. 3]</sup></a>

which gives the wave equation

<a id="eq-6"></a>

$$
\frac{1}{c^{2}} \frac{\partial^{2}}{\partial t^{2}} \psi = \nabla^{2} \psi - \frac{m^{2} c^{2}}{\hbar^{2}} \psi \tag{6}
$$

This is an historic equation, the <a id="ix-163">Klein-Gordon equation</a>. <a id="ix-268">Schrödinger</a> already in 1926 tried to make a RQ theory out of it. But he failed, and many other people too, until Pauli and <a id="ix-280">Weisskopf</a> gave the many-particle theory in 1934 <a id="refback-12-1"></a>[[12]](#ref-12). Why?

Because in order to interpret the wave-function as a probability we must have a <a id="ix-234">continuity</a> equation. This can only be got out of the wave-equation if we take $\hbar$ as before, and

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

<a id="ref-4"></a>**[4]** J. Schwinger, ed., _Selected Papers on Quantum Electrodynamics_, Dover Publications, New York, 1958. Many of the most important Feynman, Schwinger and Dyson papers, together with those of other authors, are gathered in this anthology edited by Schwinger. [↩](#refback-4-1)

<a id="ref-5"></a>**[5]** Arthur I. Miller, _Early Quantum Electrodynamics: a source book_, Cambridge U. P., Cambridge UK, 1994. Miller’s book includes a valuable historical essay and English translations of three articles cited by Dyson: Heisenberg’s on the Dirac theory of the positron (_Zeits. f. Phys._ **90** (1934) 209), Kramers’s suggestion of mass renormalization (_Nuovo Cim._ NS **15** (1938) 108), and the Pauli-Weisskopf discussion of the relativistic many-particle (scalar) theory (_Helv. Phys. Acta_ **7** (1934) 709). [↩](#refback-5-1)

<a id="ref-6"></a>**[6]** Silvan S. Schweber, _An Introduction to Relativistic Quantum Field Theory_, Row, Peterson & Co., Evanston, IL, 1961. This magisterial textbook has been reissued by Dover Publications (2005) in paperback. Contains a very complete set of references to the QED work done from 1926-1960. [↩](#refback-6-1)

<a id="ref-7"></a>**[7]** Silvan S. Schweber, _QED and the Men Who Made It: Dyson, Feynman, Schwinger and Tomonaga_, Princeton U. P., Princeton NJ, 1994. A very readable, technical history of QED. [↩](#refback-7-1)

<a id="ref-8"></a>**[8]** David Kaiser, _Drawing Theories Apart: the dispersion of Feynman diagrams in postwar physics_, U of Chicago Press, Chicago, 2005. The sociology of the transmission of Feynman’s graphical techniques. [↩](#refback-8-1)

<a id="ref-9"></a>**[9]** P. A. M. Dirac, “The quantum theory of the electron”, _Proc. Roy. Soc. A_ **117** (1928) 610. [↩1](#refback-9-1) [↩2](#refback-9-2)

<a id="ref-10"></a>**[10]** H. Yukawa, “On the interaction of elementary particles”, _Prog. Theo. Phys._ **17** (1935) 48. In Henry A. Boorse and Lloyd Motz, _The World of the Atom_, vol. II, Basic Books, Inc., New York, 1966, pp 1419–1422. [↩](#refback-10-1)

<a id="ref-11"></a>**[11]** W. Pauli, “The Connection Between Spin and Statistics”, _Phys. Rev._ **58** (1940) 716. In Schwinger, _Selected Papers in Quantum Electrodynamics_, pp 372–378. [↩](#refback-11-1)

<a id="ref-12"></a>**[12]** W. Pauli and V. Weisskopf, “The quantization of the scalar relativistic wave equation”, _Helv. Phys._ _Acta_ **7** (1934) 709. In Miller, _Early Quantum Electrodynamics_, pp 188–205 (English). [↩](#refback-12-1)

---

[← Table of Contents](00b-table-of-contents.md) · [The Dirac Theory →](02-the-dirac-theory.md)
