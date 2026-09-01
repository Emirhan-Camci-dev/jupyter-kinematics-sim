import numpy as np
from jupyter_kinematics_sim_core import solve_ik

print("=== JupyterKinematics-Sim Community Edition ===")
print("Standard Jacobian Pseudo-Inverse IK Solver\n")

# A dummy 6x6 jacobian for a 6-DOF arm
jacobian = np.random.rand(6, 6).astype(np.float64)
target_twist = np.array([0.1, 0.0, 0.0, 0.0, 0.0, 0.0], dtype=np.float64)

print("Jacobian:")
print(jacobian)
print("\nTarget Cartesian Twist:", target_twist)

# Solve
joint_velocities = solve_ik(jacobian, target_twist)
print("\nComputed Joint Velocities:", joint_velocities)
