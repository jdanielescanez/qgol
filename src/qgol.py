
from qiskit import QuantumCircuit, QuantumRegister, ClassicalRegister, transpile
from qiskit.quantum_info.operators import Operator
from qiskit.circuit.library import C3XGate, XGate
from qiskit.providers.basic_provider import BasicProvider

from numpy import sqrt
from multiprocessing import Pool, cpu_count
from copy import deepcopy

class QGoL:
    def __init__(self, probabilities_table):
        self.probabilities_table = probabilities_table
        self.turn_tables = [self.probabilities_table]

        self.N = len(self.probabilities_table)
        self.M = len(self.probabilities_table[0])
        self.SHOTS = 1000
        self.backend = BasicProvider().get_backend('basic_simulator')

        self.output = QuantumRegister(1, name='output')
        self.current_cell = QuantumRegister(1, name='current_cell')
        self.counter_alive = QuantumRegister(4, name='counter_alive')
        self.neighbours = QuantumRegister(8, name='neighbour')
        self.next_turn = ClassicalRegister(1, name='next_turn')

        self.c_inc = self.get_c_inc()

    def get_c_inc(self):
        inc = QuantumCircuit(4, name='INC')
        inc.append(C3XGate(), [0, 1, 2, 3])
        inc.ccx(0, 1, 2)
        inc.cx(0, 1)
        inc.x(0)
        return inc.to_gate().control(1)

    def get_op_from_prob(self, prob_alive):
        a1 = sqrt(1 - prob_alive)
        a2 = sqrt(prob_alive)
        return Operator([
            [complex(a1, 0), complex(a2, 0)],
            [complex(a2, 0), - complex(a1, 0)],
        ])
    
    def get_relative_neighbours_probabilities(self, i, j):
        relative_indexes = [
            (i - 1, j - 1),
            (i - 1, j),
            (i - 1, (j + 1) % self.M),
            (i, j - 1),
            (i, (j + 1) % self.M),
            ((i + 1) % self.N, j - 1),
            ((i + 1) % self.N, j),
            ((i + 1) % self.N, (j + 1) % self.M),
        ]
        return [self.probabilities_table[w][v] for (w, v) in relative_indexes]

    def get_circuit(self, current_prob, relative_neighbours_probs):
        qc = QuantumCircuit(self.output, self.current_cell, self.counter_alive, self.neighbours, self.next_turn)

        qc.unitary(self.get_op_from_prob(current_prob), self.current_cell, label='u_*')
        qc.cx(self.current_cell, self.output)

        for k, (w, v) in enumerate(relative_neighbours_probs):
            prob_alive_w_v = self.probabilities_table[w][v]
            qc.unitary(self.get_op_from_prob(prob_alive_w_v), self.neighbours[k], label=f'u_{k}')
            qc.append(self.c_inc, [self.neighbours[k], *self.counter_alive])

        # TODO: Add birth and survival rules
        qc.append(XGate().control(5, ctrl_state='00110'), [self.current_cell, *self.counter_alive, self.output])
        qc.append(XGate().control(4, ctrl_state='0011'), [self.current_cell, *self.counter_alive[1:], self.output])
        qc.cx(self.current_cell, self.output)

        qc.measure(self.output, self.next_turn)
        return qc

    def get_memory(self, prob_alive_i_j, relative_neighbours_probs):
        not_transpiled_qc = self.get_circuit(prob_alive_i_j, relative_neighbours_probs)
        qc = transpile(not_transpiled_qc, self.backend)
        job = self.backend.run(qc, shots=self.SHOTS, memory=True)
        return job.result().get_memory()

    def step(self):
        new_matrix = deepcopy(self.probabilities_table)
        tasks = [(i, j, self.probabilities_table[i][j], self.get_relative_neighbours_probs(i, j)) for i in range(self.N) for j in range(self.M)]
        pool = Pool(processes=(cpu_count() - 1))
        for i, j, current_prob, relative_neighbours_probs in tasks:
            memory = pool.apply_async(self.get_memory, args=(current_prob, relative_neighbours_probs)).get()
            alive_cnt = len(list(filter(lambda x: x == '1', memory)))
            new_matrix[i][j] = alive_cnt / self.SHOTS
        pool.close()
        pool.join()

        self.turn_tables.append(self.probabilities_table)

    def run(self, turns):
        for _ in range(1, turns + 1):
            self.step()
