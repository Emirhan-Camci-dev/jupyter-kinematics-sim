use nalgebra::{DMatrix, DVector};

/// Standard Jacobian Pseudo-Inverse IK (Community Edition)
/// Solves dx = J * dq -> dq = J^+ * dx
pub fn solve_ik_pseudoinverse(
    jacobian: &DMatrix<f64>,
    target_velocity: &DVector<f64>,
) -> Result<DVector<f64>, &'static str> {
    let j_svd = jacobian.clone().svd(true, true);
    let j_pinv = j_svd.pseudo_inverse(1e-6).map_err(|_| "SVD failed to converge")?;
    
    let dq = j_pinv * target_velocity;
    Ok(dq)
}
