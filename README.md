# JupyterKinematics-Sim

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL%20v3-blue.svg)](https://www.gnu.org/licenses/agpl-3.0) 
[![Enterprise License](https://img.shields.io/badge/Enterprise-Proprietary-red.svg)](https://polar.sh/buy)

**JupyterKinematics-Sim** is an enterprise/R&D-grade Deterministic Inverse Kinematics (IK), Singularity-Robust Trajectory Optimization & Interactive HIL Simulation Suite. Designed for robotic manipulator OEMs (6-DOF, 7-DOF redundant arms, Cobots, humanoid limbs) and R&D labs to eliminate kinematic singularity lockups, bridge Sim2Real dynamic divergence, and execute hard real-time trajectory generation.

## Features

- **Singularity-Robust IK**: Deterministic Damped SVD-based singularity avoidance guaranteeing sub-100μs solver convergence near gimbal/wrist/shoulder singularities.
- **Python / Jupyter Native**: Zero-copy Python bindings (PyO3) allowing millisecond-grade IK sweeping, manipulability ellipsoid plotting, and motor thermal/torque profile visualization inside Jupyter Notebooks.
- **Hardware-in-the-Loop (HIL)**: Zero-copy shared memory IPC & UDP streaming node synchronizing target joint setpoints to EtherCAT controllers at >1 kHz loop rates.

## Community Edition vs. Enterprise Pro

This project follows an Open-Core model. 

| Feature | Community Edition (AGPLv3) | Enterprise Pro Tier (Proprietary) |
|---------|----------------------------|-----------------------------------|
| Inverse Kinematics | Standard Jacobian Pseudo-inverse | Dynamic Singularity-Damped SVD |
| Solver Performance | ~500μs | **< 100μs (SIMD Accelerated)** |
| Singularity Handling | Fails/Locks up near singularities | **Zero Singularity Stalls** |
| Collision Avoidance | Not Included | Continuous Collision Detection (<50μs) |
| Sim2Real Dynamics | Not Included | Dynamic parameter identification |
| License Verification | Open Source | Offline Ed25519 Cryptographic JWT |


[**🚀 Buy Enterprise R&D Seat License via Polar.sh**](https://buy.polar.sh/polar_cl_LtJ9ztRiE4UGkHlvIvEdf9BNSMi91FzE6n56V1s6NQ5)

---

## ⚡ 3-Line Quickstart (Python / Jupyter)

Install the Enterprise SDK and start resolving target poses near singularities immediately:

```python
from jupyter_kinematics_sim_core_pro import ProIKSolver
import numpy as np

# 1. Initialize Pro Solver with your offline license token
solver = ProIKSolver(license_token="eyJhbGci..._YOUR_POLAR_SH_TOKEN_...")

# 2. Define your Jacobian (6x7 for a redundant arm) and target twist
jacobian = np.random.rand(6, 7).astype(np.float64)
target_twist = np.array([0.1, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64)

# 3. Solve IK deterministically (Avoids singularities with Damped SVD)
joint_velocities = solver.solve_ik(jacobian, target_twist, max_damping=0.1, singular_region=0.05)

print(f"Computed Joint Velocities: {joint_velocities}")
```

## Dual-Licensing & Architecture Hygiene

The repository is strictly separated to avoid license contamination:
- `/community/`: Contains the AGPLv3 Rust core and Python bindings. Completely free.
- `/enterprise/`: Contains the proprietary modules (Damped SVD, License Verifier, HIL) built as a separate Rust workspace.

## Offline License Verification

For R&D labs working on air-gapped systems, the Enterprise edition features an **Offline Ed25519 Cryptographic Verifier**. When you purchase a seat via Polar.sh, you receive a JWT (base64-encoded payload and signature). The Rust core directly verifies the signature against a hardcoded public key entirely on-device, ensuring security and time-bounded validity without any internet connection.

## Testing & Stability

The Rust core ensures zero memory leaks. You can run the integration tests which benchmark 100,000+ continuous IK solves checking for Python `tracemalloc` variations to guarantee production readiness in 24/7 environments.

```bash
cd enterprise/tests/
python -m unittest test_memory_leak.py
```

---
*© 2026 Emirhan CAMCI. All Rights Reserved.*

## 🏎️ Hardware-in-the-Loop (HIL) Streaming
The Enterprise Edition includes a deterministic UDP/IPC streamer designed for 1kHz+ control loops with TwinCAT or ROS2 controllers.
```python
from jupyter_kinematics_sim_core_pro import PyHilStreamer
import numpy as np

# Bind local socket and set EtherCAT Controller target IP
streamer = PyHilStreamer("0.0.0.0:0", "192.168.1.100:5000")

# Stream setpoints (Positions, Velocities, Torques)
positions = np.array([0.1, 0.2, 0.3, 0.4, 0.5, 0.6], dtype=np.float64)
streamer.send_setpoints(timestamp_us=123456789, positions=positions, velocities=np.zeros(6), torques=np.zeros(6))
```
