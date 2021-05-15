pub fn preproc_python_code(code: &str) -> String{
    let prep_import = "from preparation import *";

    return format!("{}\n\n{}", prep_import, code);
}

pub fn preproc_python_execution(code: &str) -> String{
    let imports = "import os
from preparation import *
from code import *
CURR_PATH = os.path.dirname(os.path.abspath(__file__))";

    let out_function = "def write_output(output):
    with open(f'{CURR_PATH}/output.out', 'w') as out:
        out.write(output)";

    return format!("{}\n\n{}\n\n{}", imports, out_function, code);
}