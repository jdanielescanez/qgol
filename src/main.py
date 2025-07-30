import argparse
import json
from pathlib import Path
from qgol import QGoL

def get_table_from_file(file_in):
    try:
        f = open(file_in, 'r')
        lines = f.readlines()
        f.close()
    except Exception as e:
        print(f"Failed to write input file: {e}")
        exit(1)

    coefficients = []
    for line in lines:
        row = list(map(float, line.split()))
        coefficients.append(row)

    return coefficients

def main():
    parser = argparse.ArgumentParser(description="Quantum Game of Life")
    parser.add_argument("input_filename", type=Path, help="File with the initial generation")
    parser.add_argument("output_filename", type=Path, help="File to save the game's memory")
    parser.add_argument("turns", type=int, help="Number of turns to simulate")
    # TODO: Add birth and survival rules
    # parser.add_argument("--births", "-b", type=str, default="[3]",
    #                     help="List of neighbor counts that allow a cell to be born")
    # parser.add_argument("--survivals", "-s", type=str, default="[2,3]",
    #                     help="List of neighbor counts that allow a cell to survive")

    args = parser.parse_args()

    probabilities_table = get_table_from_file(args.input_filename)
    qgol = QGoL(probabilities_table)

    # TODO: Add birth and survival rules
    # try:
    #     birth_rules = json.loads(args.births)
    #     survival_rules = json.loads(args.survivals)
    # except json.JSONDecodeError:
    #     print("Please provide survivals and births as valid JSON arrays (e.g., \"[2,3]\")")
    #     exit(1)
    # qgol.change_rules(birth_rules, survival_rules)

    for _ in range(args.turns):
        qgol.next()

    try:
        with open(args.output_filename, "w") as f:
            json.dump(qgol.turn_tables, f, indent=4)
    except Exception as e:
        print(f"Failed to write output file: {e}")
        exit(1)

if __name__ == "__main__":
    main()
