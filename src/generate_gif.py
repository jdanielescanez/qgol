
import sys
from src.plotter import Plotter
import json

def main():
    args = sys.argv[1:]
    if len(args) == 0:
        print('[!] The input file must be specified as first parameter')
        exit(1)
    if len(args) == 1:
        print('[!] The output file must be specified as second parameter')
        exit(1)
    
    input_file, output_file = args[:2]
    with open(input_file, 'r') as archivo:
        matrices = json.load(archivo)
        plotter = Plotter()
        plotter.generate_gif(matrices, output_file)

if __name__ == "__main__":
    main()
