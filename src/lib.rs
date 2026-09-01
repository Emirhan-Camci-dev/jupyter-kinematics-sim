use pyo3::prelude::*;
use pyo3::exceptions::PyRuntimeError;
use numpy::{PyReadonlyArray2, PyReadonlyArray1, PyArray1, PyUntypedArrayMethods};
use nalgebra::{DMatrix, DVector};

mod ik_solver;

/// Solves Inverse Kinematics using Jacobian Pseudo-Inverse.
/// 
/// Args:
///     jacobian (np.ndarray): The 6xN Jacobian matrix.
///     target_vel (np.ndarray): The 6x1 target velocity twist (spatial velocity).
/// Returns:
///     np.ndarray: The computed joint velocities (dq).
#[pyfunction]
fn solve_ik<'py>(
    py: Python<'py>,
    jacobian: PyReadonlyArray2<'py, f64>,
    target_vel: PyReadonlyArray1<'py, f64>,
) -> PyResult<Bound<'py, PyArray1<f64>>> {
    let j_mat = DMatrix::from_row_slice(
        jacobian.shape()[0],
        jacobian.shape()[1],
        jacobian.as_slice().unwrap()
    );
    
    let v_vec = DVector::from_row_slice(target_vel.as_slice().unwrap());

    match ik_solver::solve_ik_pseudoinverse(&j_mat, &v_vec) {
        Ok(dq) => Ok(PyArray1::from_slice_bound(py, dq.as_slice())),
        Err(e) => Err(PyRuntimeError::new_err(e.to_string())),
    }
}

/// A Python module implemented in Rust.
#[pymodule]
fn jupyter_kinematics_sim_core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(solve_ik, m)?)?;
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    Ok(())
}
