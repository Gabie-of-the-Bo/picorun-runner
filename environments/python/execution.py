import os
from preparation import *
from code import *
CURR_PATH = os.path.dirname(os.path.abspath(__file__))

def write_output(output):
    with open(f'{CURR_PATH}/output.out', 'w') as out:
        out.write(output)


write_output(str(sol()))
