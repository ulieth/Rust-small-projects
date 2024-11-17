use ark_ff::PrimeField;
use ark_poly::{
    univariate::DensePolynomial, DenseUVPolynomial, EvaluationDomain, GeneralEvaluationDomain,
};
// DenseUVPolynomial is a trait from the ark_poly library
// that defines the behavior for univariate polynomials stored in dense form.
// Makes it easier to access coefficients by their position.
struct FRI_LDT<F: PrimeField, P: DenseUVPolynomial<F>, H: Hash<F>> {
  _f: PhantomData<F>,     // PhantomData is used because the struct doesn't actually store any data of these types,
  _poly: PhantomData<P>,  // but needs to "remember" the types for its methods.
  _h: PhantomData<H>,     // PhantomData fields don't take any space in memory.
}

// Creating a polynomial
let coeffs = vec![F::one(), F::zero(), F::from(3)];  // 3x² + 1
let poly = P::from_coefficients_vec(coeffs);

// Splitting polynomial into even/odd parts
fn split(p: &P) -> (P, P) {
    let coeffs = p.coeffs();
    let odd: Vec<F> = coeffs.iter().step_by(2).cloned().collect();
    let even: Vec<F> = coeffs.iter().skip(1).step_by(2).cloned().collect();

    return (
        P::from_coefficients_vec(odd),
        P::from_coefficients_vec(even),
    );
}

// Evaluating polynomial
let value = poly.evaluate(&point);
