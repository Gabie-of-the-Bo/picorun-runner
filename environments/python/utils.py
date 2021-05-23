import os


CURR_PATH = os.path.dirname(os.path.abspath(__file__))


def write_output(output):
    with open(f'{CURR_PATH}/output.out', 'w') as out:
        out.write(output)


def wipe_output():
    open(f'{CURR_PATH}/output.out', 'w').close()


wipe_output()